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
#[cfg(feature = "bevy")]
use bevy::prelude::World;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::result::Result;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use yarnspinner::compiler::*;
use yarnspinner::core::*;
use yarnspinner::runtime::*;

mod extensions;
mod logger;
mod paths;
mod step;
mod test_plan;
mod text_provider;
use logger::*;
pub use text_provider::SharedTextProvider;
use yarnspinner::log::{self, LevelFilter, SetLoggerError};

pub mod prelude {
    #[allow(unused_imports)] // False positive
    pub use crate::test_base::{extensions::*, paths::*, step::*, test_plan::*, *};
}

pub fn init_logger(runtime_errors_cause_failure: Arc<AtomicBool>) -> Result<(), SetLoggerError> {
    let logger = TestLogger::new(runtime_errors_cause_failure);
    log::set_boxed_logger(Box::new(logger)).map(|()| log::set_max_level(LevelFilter::Info))
}

#[derive(Debug)]
pub struct TestBase {
    pub dialogue: Dialogue,
    pub test_plan: Option<TestPlan>,
    pub string_table: SharedTextProvider,
    pub variable_storage: MemoryVariableStorage,
    runtime_errors_cause_failure: Arc<AtomicBool>,
}

impl Default for TestBase {
    fn default() -> Self {
        let runtime_errors_cause_failure = Arc::new(AtomicBool::new(true));
        if let Err(_e) = init_logger(runtime_errors_cause_failure.clone()) {
            // We've set the logger twice, that's alright for the tests.
        }
        let variable_storage = MemoryVariableStorage::new();
        let string_table = SharedTextProvider::new(StringTableTextProvider::new());

        let mut dialogue = Dialogue::new(
            Box::new(variable_storage.clone()),
            Box::new(string_table.clone()),
        );
        dialogue
            .library_mut()
            .add_function("assert", |value: YarnValue| {
                let is_truthy: bool = value.try_into().unwrap();
                assert!(is_truthy);
                true
            });

        Self {
            dialogue,
            runtime_errors_cause_failure,
            variable_storage,
            string_table,
            test_plan: Default::default(),
        }
    }
}

impl TestBase {
    #[must_use]
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the current test plan to one loaded from a given path.
    #[must_use]
    pub fn read_test_plan(self, path: impl AsRef<Path>) -> Self {
        self.with_test_plan(TestPlan::read(path))
    }

    #[must_use]
    pub fn with_test_plan(mut self, test_plan: TestPlan) -> Self {
        self.test_plan.replace(test_plan);
        self
    }

    #[must_use]
    pub fn with_runtime_errors_do_not_cause_failure(self) -> Self {
        self.runtime_errors_cause_failure
            .store(false, Ordering::Relaxed);
        self
    }

    #[must_use]
    pub fn with_compilation(self, compilation: Compilation) -> Self {
        let string_table = compilation.string_table;
        self.with_program(compilation.program.unwrap())
            .with_string_table(string_table)
    }

    #[must_use]
    pub fn extend_library(mut self, extend_fn: impl Fn(&mut Library)) -> Self {
        let library = self.dialogue.library_mut();
        extend_fn(library);
        self
    }

    #[must_use]
    pub fn with_program(mut self, program: Program) -> Self {
        self.dialogue.add_program(program);
        self
    }

    #[must_use]
    pub fn with_string_table(mut self, string_table: HashMap<LineId, StringInfo>) -> Self {
        let string_table: HashMap<_, _> = string_table
            .into_iter()
            .map(|(id, info)| (id, info.text))
            .collect();
        let mut string_table_provider = StringTableTextProvider::new();
        string_table_provider.extend_base_language(string_table.clone());
        string_table_provider.extend_translation("en-US", string_table);
        self.string_table.replace(string_table_provider);
        self.dialogue.set_language_code(Language::from("en-US"));
        self
    }

    /// Executes the named node, and checks any assertions made during
    /// execution. Fails the test if an assertion made in Yarn fails.
    pub fn run_standard_testcase(&mut self) -> &mut Self {
        self.dialogue.set_node("Start").unwrap();

        #[cfg(feature = "bevy")]
        let mut world = World::default();

        while self.dialogue.can_continue() {
            #[cfg(feature = "bevy")]
            let events = self
                .dialogue
                .continue_with_world(&mut world)
                .unwrap_or_else(|e| panic!("Encountered error while running dialogue: {e}"));
            #[cfg(not(feature = "bevy"))]
            let events = self
                .dialogue
                .continue_()
                .unwrap_or_else(|e| panic!("Encountered error while running dialogue: {e}"));

            for event in events {
                match event {
                    DialogueEvent::Line(line) => {
                        println!("Line: {}", line.text);
                        let Some(test_plan) = self.test_plan.as_mut() else {
                            continue;
                        };
                        test_plan.next();

                        assert_eq!(
                            ExpectedStepType::Line,
                            test_plan.next_expected_step,
                            "Received line {}, but was expecting a {:?}",
                            line.text,
                            test_plan.next_expected_step
                        );
                        assert_eq!(
                            test_plan.next_step_value,
                            Some(StepValue::String(line.text))
                        );
                    }
                    DialogueEvent::Options(options) => {
                        println!("Options:");

                        let options: Vec<_> = options
                            .into_iter()
                            .map(|option| ProcessedOption {
                                line: option.line.text,
                                enabled: option.is_available,
                            })
                            .collect();
                        for option in &options {
                            println!(" - {} (available: {})", option.line, option.enabled);
                        }
                        let Some(test_plan) = self.test_plan.as_mut() else {
                            continue;
                        };

                        test_plan.next();
                        assert_eq!(
                            ExpectedStepType::Select,
                            test_plan.next_expected_step,
                            "Received {} options, but wasn't expecting them (was expecting {:?})",
                            options.len(),
                            test_plan.next_expected_step
                        );

                        assert_eq!(test_plan.next_expected_options, options);

                        if let Some(StepValue::Number(selection)) = test_plan.next_step_value {
                            let selection = selection - 1; // 1-indexed for test plan, 0-indexed in the code
                            println!("[Selecting option {selection}]");
                            self.dialogue
                                .set_selected_option(OptionId(selection))
                                .unwrap();
                        } else {
                            println!("[Selecting option 0 implicitly]");
                            self.dialogue.set_selected_option(OptionId(0)).unwrap();
                        }
                    }
                    DialogueEvent::Command(command) => {
                        println!("Command: {}", command.raw);
                        let Some(test_plan) = self.test_plan.as_mut() else {
                            continue;
                        };
                        test_plan.next();
                        assert_eq!(
                            ExpectedStepType::Command,
                            test_plan.next_expected_step,
                            "Received command {}, but wasn't expecting to select one (was expecting {:?})",
                            command.raw,
                            test_plan.next_expected_step
                        );

                        // We don't need to get the composed string for a
                        // command because it's been done for us in the
                        // virtual machine. The VM can do this because
                        // commands are not localised, so we don't need to
                        // refer to the string table to get the text.
                        assert_eq!(
                            test_plan.next_step_value,
                            Some(StepValue::String(command.raw))
                        );
                    }
                    DialogueEvent::NodeComplete(_) => {}
                    DialogueEvent::NodeStart(_) => {}
                    DialogueEvent::LineHints(_) => {}
                    DialogueEvent::DialogueComplete => {
                        let Some(test_plan) = self.test_plan.as_mut() else {
                            continue;
                        };
                        test_plan.next();
                        assert_eq!(
                            ExpectedStepType::Stop,
                            test_plan.next_expected_step,
                            "Stopped dialogue, but wasn't expecting to select it (was expecting {:?})",
                            test_plan.next_expected_step
                        );
                    }
                }
            }
        }
        self
    }

    /// Returns the list of .node and.Yarn files in the third-party/YarnSpinner/Tests/<subdir> directory.
    pub fn file_sources(subdir: impl AsRef<Path>) -> impl Iterator<Item = PathBuf> {
        let subdir: PathBuf = PathBuf::from(subdir.as_ref());
        let path = test_data_path().join(&subdir);
        let allowed_extensions = ["node", "yarn"].map(OsStr::new);
        fs::read_dir(&path)
            .unwrap_or_else(|e| panic!("Failed to read directory {}: {e}", path.display()))
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
            .filter(|entry| !entry.path().ends_with(".upgraded.yarn"))
            .map(move |entry| subdir.join(entry.file_name()))
    }
}
