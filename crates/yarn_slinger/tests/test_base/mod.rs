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
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use yarn_slinger::prelude::*;

mod extensions;
mod paths;
mod step;
mod test_plan;

pub mod prelude {
    pub use crate::test_base::{extensions::*, paths::*, step::*, test_plan::*, *};
}

#[derive(Debug, Clone)]
pub struct TestBase {
    pub storage: Arc<RwLock<dyn VariableStorage + Send + Sync>>,
    pub dialogue: Dialogue,
    pub test_plan: TestPlan,
    pub string_table: Arc<RwLock<HashMap<LineId, StringInfo>>>,
    pub runtime_errors_cause_panic: Arc<AtomicBool>,
}

impl Default for TestBase {
    fn default() -> Self {
        let runtime_errors_cause_panic = Arc::new(AtomicBool::new(true));
        let string_table: Arc<RwLock<HashMap<LineId, StringInfo>>> =
            Arc::new(RwLock::new(HashMap::new()));
        let dialogue = {
            let runtime_errors_cause_panic = runtime_errors_cause_panic.clone();
            let string_table = string_table.clone();
            Dialogue::default()
                .with_language_code("en")
                .with_log_debug_message(|msg| {
                    println!("{}", msg);
                })
                .with_log_error_message(move |msg| {
                    eprintln!("{}", msg);
                    if runtime_errors_cause_panic.load(Ordering::Relaxed) {
                        assert!(msg.is_empty())
                    }
                })
                .with_line_handler(move |line| {
                    let id = line.id;
                    let string_table = string_table.read().unwrap();
                    let string_info = string_table.get(&id).unwrap();
                    let line_number = string_info.line_number;
                    // Can't go on because a handler cannot access the dialogue.
                })
        };
        let storage = dialogue.variable_storage.clone();
        Self {
            dialogue,
            storage,
            test_plan: TestPlan::default(),
            string_table,
            runtime_errors_cause_panic,
        }
    }
}

impl TestBase {
    /// Sets the current test plan to one loaded from a given path.
    pub fn read_test_plan(mut self, path: &Path) -> Self {
        self.test_plan = TestPlan::read(path);
        self
    }

    /// Executes the named node, and checks any assertions made during
    /// execution. Fails the test if an assertion made in Yarn fails.
    pub fn run_standard_testcase(&mut self) {
        self.dialogue.set_start_node();

        self.dialogue.continue_();
        while self.dialogue.is_active() {
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

    pub fn get_composed_text_for_line(&self, line: Line) -> String {
        let string_info = self.string_table.get(&line.id).unwrap();
        let substitutions = line.substitutions.iter().map(|s| s.as_str());
        let substituted_text = Dialogue::expand_substitutions(&string_info.text, substitutions);
        self.dialogue.parse_markup(&substituted_text)
    }
}
