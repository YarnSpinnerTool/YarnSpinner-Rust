//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Tests/TestBase.cs>
//!
//! ## Implementations notes
//! Methods used for upgrade testing were not ported since we don't offer any upgrade functionality.
//! This includes `DirectorySources`.
//!
//! Methods for tests we didn't port are also naturally not included. This includes `FormatParseTreeAsText`

// Allowed because this is a common file and not all tests use the methods provided.
// Everything is actually used, but the checker doesn't recognize it because all integration test files
// are separately compiled as their own crate.
#![allow(dead_code)]

use crate::prelude::*;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use yarn_slinger::prelude::*;
use yarn_slinger_core::prelude::YarnValue;

mod extensions;
mod paths;
mod step;
mod test_plan;

pub mod prelude {
    pub use crate::test_base::{extensions::*, paths::*, step::*, test_plan::*, *};
}

#[derive(Debug, Clone)]
pub struct TestBase {
    pub dialogue: Dialogue,
    test_plan: Arc<RwLock<Option<TestPlan>>>,
    string_table: Arc<RwLock<HashMap<LineId, StringInfo>>>,
    pub runtime_errors_cause_panic: Arc<AtomicBool>,
}

impl Default for TestBase {
    fn default() -> Self {
        let runtime_errors_cause_panic = Arc::new(AtomicBool::new(true));
        let string_table: Arc<RwLock<HashMap<LineId, StringInfo>>> =
            Arc::new(RwLock::new(HashMap::new()));
        let test_plan: Arc<RwLock<Option<TestPlan>>> = Arc::new(RwLock::new(None));

        let dialogue = Dialogue::default()
            .with_language_code("en")
            .with_log_debug_message(|msg| {
                println!("{}", msg);
            });

        let dialogue = {
            let runtime_errors_cause_panic = runtime_errors_cause_panic.clone();
            let string_table = string_table.clone();
            let test_plan = test_plan.clone();
            let read_only_dialogue = dialogue.get_read_only();

            dialogue
                .with_log_error_message(move |msg| {
                    eprintln!("{}", msg);
                    if runtime_errors_cause_panic.load(Ordering::Relaxed) {
                        assert!(msg.is_empty())
                    }
                })
                .with_line_handler(move |line| {
                    let text = get_composed_text_for_line_with_no_self(
                        &line,
                        &string_table,
                        &read_only_dialogue,
                    );
                    println!("Line: {text}");
                    let mut test_plan = test_plan.write().unwrap();
                    let Some(test_plan) = test_plan.as_mut() else {
                        return;
                    };
                    test_plan.next();

                    assert_eq!(
                        ExpectedStepType::Line,
                        test_plan.next_expected_step,
                        "Received line {text}, but was expecting a {:?}",
                        test_plan.next_expected_step
                    );
                    assert_eq!(test_plan.next_step_value, Some(StepValue::String(text)));
                })
        };
        let dialogue = {
            let test_plan = test_plan.clone();
            let read_only_dialogue = dialogue.get_read_only();
            let string_table = string_table.clone();

            dialogue.with_options_handler(move |options| {
                println!("Options:");

                let options: Vec<_> = options
                    .into_iter()
                    .map(|option| {
                        let text = get_composed_text_for_line_with_no_self(
                            &option.line,
                            &string_table,
                            &read_only_dialogue,
                        );
                        ProcessedOption {
                            line: text,
                            enabled: option.is_available,
                        }
                    })
                    .collect();
                for option in &options {
                    println!(" - {} (available: {})", option.line, option.enabled);
                }
                let mut test_plan = test_plan.write().unwrap();
                let Some(test_plan) = test_plan.as_mut() else {
                    return;
                };

                test_plan.next();
                assert_eq!(
                    ExpectedStepType::Option,
                    test_plan.next_expected_step,
                    "Received {} options, but wasn't expecting them (was expecting {:?})",
                    options.len(),
                    test_plan.next_expected_step
                );

                assert_eq!(test_plan.next_expected_options, options);
            })
        };

        let dialogue = {
            let test_plan = test_plan.clone();

            dialogue
                .with_command_handler(move |command| {
                    println!("Command: {}", command.0);
                    let mut test_plan = test_plan.write().unwrap();
                    let Some(test_plan) = test_plan.as_mut() else {
                    return;
                };
                    test_plan.next();
                    assert_eq!(
                    ExpectedStepType::Command,
                    test_plan.next_expected_step,
                    "Received command {}, but wasn't expecting to select one (was expecting {:?})",
                    command.0,
                    test_plan.next_expected_step
                );

                    // We don't need to get the composed string for a
                    // command because it's been done for us in the
                    // virtual machine. The VM can do this because
                    // commands are not localised, so we don't need to
                    // refer to the string table to get the text.
                    assert_eq!(
                        test_plan.next_step_value,
                        Some(StepValue::String(command.0))
                    );
                })
                .with_node_complete_handler(|_| {})
        };
        let mut dialogue = {
            let test_plan = test_plan.clone();
            dialogue.with_dialogue_complete_handler(move || {
                let mut test_plan = test_plan.write().unwrap();
                let Some(test_plan) = test_plan.as_mut() else {
                    return;
                };
                test_plan.next();
                assert_eq!(
                    ExpectedStepType::Stop,
                    test_plan.next_expected_step,
                    "Stopped dialogue, but wasn't expecting to select it (was expecting {:?})",
                    test_plan.next_expected_step
                );
            })
        };

        dialogue
            .library_mut()
            .register_function("assert", |value: YarnValue| {
                let is_truthy: bool = value.try_into().unwrap();
                assert!(is_truthy);
                true
            });
        Self {
            dialogue,
            test_plan,
            string_table,
            runtime_errors_cause_panic,
        }
    }
}

impl TestBase {
    /// Sets the current test plan to one loaded from a given path.
    pub fn read_test_plan(self, path: &Path) -> Self {
        self.with_test_plan(TestPlan::read(path))
    }

    pub fn with_test_plan(self, test_plan: TestPlan) -> Self {
        self.test_plan.write().unwrap().replace(test_plan);
        self
    }

    pub fn runtime_failure_causes_no_panic(self) -> Self {
        self.runtime_errors_cause_panic
            .store(false, Ordering::Relaxed);
        self
    }

    /// Executes the named node, and checks any assertions made during
    /// execution. Fails the test if an assertion made in Yarn fails.
    pub fn run_standard_testcase(&mut self) {
        self.dialogue.set_start_node();

        self.dialogue.continue_();
        while self.dialogue.is_active() {
            let test_plan = self.test_plan.read().unwrap();
            if let Some(test_plan) = test_plan.as_ref() {
                if test_plan.next_expected_step == ExpectedStepType::Option {
                    if let Some(StepValue::Number(selection)) = test_plan.next_step_value {
                        self.dialogue
                            .set_selected_option(OptionId::construct_for_debugging(selection));
                    } else {
                        self.dialogue
                            .set_selected_option(OptionId::construct_for_debugging(0));
                    }
                }
            }
            self.dialogue.continue_();
        }
    }

    /// Returns the list of .node and.yarn files in the Tests/<directory> directory.
    pub fn file_sources(subdir: impl AsRef<Path>) -> impl Iterator<Item = PathBuf> {
        let subdir: PathBuf = PathBuf::from(subdir.as_ref());
        let path = test_data_path().join(&subdir);
        let allowed_extensions = ["node", "yarn"].map(OsStr::new);
        fs::read_dir(path)
            .unwrap()
            .filter_map(|entry| {
                entry
                    .map_err(|e| {
                        println!("Warning: failed to read a directory entry: {e:?}");
                        e
                    })
                    .ok()
            })
            .filter(move |entry| {
                entry
                    .path()
                    .extension()
                    .map(|ext| allowed_extensions.contains(&ext))
                    .unwrap_or_default()
            })
            // don't include ".upgraded.yarn" (used in upgrader tests)
            .filter(|entry| entry.path().ends_with(".upgraded.yarn"))
            .map(move |entry| subdir.join(entry.file_name()))
    }

    pub fn get_composed_text_for_line(&self, line: &Line) -> String {
        get_composed_text_for_line_with_no_self(line, &self.string_table, &self.dialogue)
    }

    pub fn test_plan(&self) -> impl Deref<Target = Option<TestPlan>> + '_ {
        self.test_plan.read().unwrap()
    }

    pub fn test_plan_mut(&mut self) -> impl DerefMut<Target = Option<TestPlan>> + '_ {
        self.test_plan.write().unwrap()
    }

    pub fn string_table(&self) -> impl Deref<Target = HashMap<LineId, StringInfo>> + '_ {
        self.string_table.read().unwrap()
    }

    pub fn string_table_mut(&mut self) -> impl DerefMut<Target = HashMap<LineId, StringInfo>> + '_ {
        self.string_table.write().unwrap()
    }
}

impl Deref for TestBase {
    type Target = Dialogue;

    fn deref(&self) -> &Self::Target {
        &self.dialogue
    }
}

impl DerefMut for TestBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.dialogue
    }
}

impl AsRef<Dialogue> for TestBase {
    fn as_ref(&self) -> &Dialogue {
        &self.dialogue
    }
}

fn get_composed_text_for_line_with_no_self(
    line: &Line,
    string_table: &RwLock<HashMap<LineId, StringInfo>>,
    dialogue: &ReadOnlyDialogue,
) -> String {
    let string_table = string_table.read().unwrap();
    let string_info = string_table.get(&line.id).unwrap();
    let substitutions = line.substitutions.iter().map(|s| s.as_str());
    let substituted_text = ReadOnlyDialogue::expand_substitutions(&string_info.text, substitutions);
    dialogue.parse_markup(&substituted_text)
}
