use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy)]
pub enum ProjectPath {
    ThirdPersonYarnSpinner,
    MainCrate,
    Core,
    Codegen,
    Runtime,
    Compiler,
    BevyPlugin,
}

pub fn path(path: ProjectPath) -> PathBuf {
    let current_dir = Path::new(file!()).parent().unwrap();
    let crates_dir = current_dir.join("../..");
    let fragment = match path {
        ProjectPath::ThirdPersonYarnSpinner => "../third-party/YarnSpinner",
        ProjectPath::MainCrate => "yarn_slinger",
        ProjectPath::Core => "core",
        ProjectPath::Codegen => "codegen",
        ProjectPath::Runtime => "runtime",
        ProjectPath::Compiler => "compiler",
        ProjectPath::BevyPlugin => "bevy_plugin",
    };
    crates_dir.join(fragment)
}
