//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Tests/TestBase.cs#L49>

use std::path::PathBuf;
use yarn_slinger_compiler::prelude::*;
use yarn_slinger_runtime::prelude::*;

pub struct TestBase {
    pub dialogue: Dialogue,
}

impl Default for TestBase {
    fn default() -> Self {
        let variable_storage = MemoryVariableStore::default();
        let dialogue = Dialogue::with_variable_storage(variable_storage);
        Self { dialogue }
    }
}

pub fn create_test_node(source: &str) -> String {
    create_test_node_with_name(source, "Start")
}

pub fn create_test_node_with_name(source: &str, name: &str) -> String {
    format!("title: {name}\n---\n{source}\n===")
}

pub fn project_root_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

pub fn test_data_path() -> PathBuf {
    let project_root_path = project_root_path();
    let project_root = project_root_path.to_str().unwrap();
    [
        project_root,
        "..",
        "..",
        "third-party",
        "YarnSpinner",
        "Tests",
    ]
    .iter()
    .collect()
}

pub fn space_demo_scripts_path() -> PathBuf {
    let test_data_path = test_data_path();
    let test_data = test_data_path.to_str().unwrap();
    [test_data, "Projects", "Space"].iter().collect()
}

pub trait CompileResultExt {
    fn unwrap_pretty(self) -> Compilation;
}

impl CompileResultExt for Result<Compilation, CompilationError> {
    fn unwrap_pretty(self) -> Compilation {
        match self {
            Ok(compilation) => compilation,
            Err(error) => {
                for diagnostic in error.diagnostics {
                    eprintln!("{}", diagnostic);
                }
                panic!("Compilation failed due to Yarn errors")
            }
        }
    }
}

pub trait TestCompilationJob {
    fn from_test_source(source: &str) -> Self;
}

impl TestCompilationJob for CompilationJob {
    fn from_test_source(source: &str) -> Self {
        let file = File {
            file_name: "<input>".to_string(),
            source: create_test_node(source),
        };

        Self::default().with_file(file)
    }
}
