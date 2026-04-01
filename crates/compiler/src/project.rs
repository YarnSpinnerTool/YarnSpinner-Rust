//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/v2.5.0/YarnSpinner.Compiler/Project.cs>

use std::path::Path;
use std::{collections::HashMap, path::PathBuf};

use globset::{Glob, GlobSet, GlobSetBuilder};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use walkdir::WalkDir;

/// A placeholder string that represents the location of the workspace root in paths.
pub const WORKSPACE_ROOT_PLACEHOLDER: &str = "${workspaceRoot}";

const ALLOW_PREVIEW_FEATURES_KEY: &str = "allowPreviewFeatures";

/// A version number representing Yarn Spinner 2
pub const YARNSPINNER_PROJECT_VERSION_2: u32 = 2;

/// A version number representing Yarn Spinner 3
pub const YARNSPINNER_PROJECT_VERSION_3: u32 = 3;

/// Current version of the .yarnproject
pub const CURRENT_PROJECT_FILE_VERSION: u32 = YARNSPINNER_PROJECT_VERSION_3;

/// Yarn Projects represent instructions on where to find Yarn scripts and
/// associated assets, and how they should be compiled.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    /// Gets or sets the file version of the project.
    ///
    /// This value is required to be equal to [`CURRENT_PROJECT_FILE_VERSION`]
    pub project_file_version: u32,

    /// Gets the path that the [`Project`]` was loaded from.
    ///
    /// This value is not stored when the file is saved, but is instead
    /// determined when the file is loaded by [`self::load_from_file()`]
    #[serde(skip)]
    pub path: Option<PathBuf>,

    /// The location of the root of the workspace in which this project is located.
    pub workspace_root_path: Option<PathBuf>,

    /// Gets or sets the collection of file search patterns used to locate
    /// Yarn files that form this project.
    #[serde(rename = "sourceFiles", default = "default_source_files")]
    pub source_file_patterns: Vec<String>,

    /// Gets or sets the collection of file search patterns that should be
    /// excluded from this project.
    ///
    /// If a file is matched by a pattern in `source_file_patterns`, and is also matched
    /// by a pattern in `exclude_file_patterns`, then it is not included in the value
    /// returned by `source_files()`
    ///
    /// Note: Unlike what is done in the C# version, you need to use the [`globset`] syntax to exclude a file.
    /// `**/Ship.yarn`` to exclude all files named *Ship.yarn*.
    #[serde(rename = "excludeFiles", default)]
    pub exclude_file_patterns: Vec<String>,

    /// Gets or sets the collection of [`LocalizationInfo`]
    /// objects that store information about where localized data for this
    /// project is found.
    #[serde(default)]
    pub localisation: HashMap<String, LocalizationInfo>,

    /// Gets or sets the base language of the project, as an IETF BCP-47
    /// language tag.
    ///
    /// The base language is the language that the Yarn scripts is written in
    pub base_language: String,

    /// Gets or sets the path to a JSON file containing command and function
    /// definitions that this project references.
    ///
    /// Definitions files are used by editing tools to provide type
    /// information and other externally-defined data used by the Yarn scripts.
    #[serde(default)]
    pub definitions: Option<String>,

    /// Gets or sets a dictionary containing instructions that control how
    /// the Yarn Spinner compiler should compile a project.
    ///
    /// Note: Unsure about the type of options. No documentation in v2.5.0.
    #[serde(default)]
    pub compiler_options: HashMap<String, JsonValue>,

    /// Contains any data parsed from the source file that was not matched
    /// to a property on this type.
    #[serde(flatten)]
    pub extension_data: HashMap<String, JsonValue>,
}

fn default_source_files() -> Vec<String> {
    vec!["**/*.yarn".to_owned()]
}

/// Stores the locations of where localized assets and a localized
/// string table for a Yarn Project may be found.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocalizationInfo {
    /// Gets or sets the location at which localized assets may be found.
    #[serde(default)]
    pub assets: Option<String>,
    /// Gets or sets the location at which the localized string table may be found.
    #[serde(default)]
    pub strings: Option<String>,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            project_file_version: CURRENT_PROJECT_FILE_VERSION,
            path: Default::default(),
            workspace_root_path: Default::default(),
            source_file_patterns: default_source_files(),
            exclude_file_patterns: Default::default(),
            localisation: Default::default(),
            base_language: Default::default(),
            definitions: Default::default(),
            compiler_options: Default::default(),
            extension_data: Default::default(),
        }
    }
}

impl Project {
    /// Replaces the path used by the [`Project`].
    pub fn with_path(mut self, path: PathBuf) -> Self {
        self.path = Some(path);
        self
    }

    /// Replaces the source files used by the [`Project`].
    pub fn with_source_files(mut self, source_files: Vec<String>) -> Self {
        self.source_file_patterns = source_files;
        self
    }

    /// Replaces the workspace root path used by the [`Project`]
    pub fn with_workspace_root_path(mut self, workspace_root_path: PathBuf) -> Self {
        self.workspace_root_path = Some(workspace_root_path.into());
        self
    }

    /// Loads and parses a [`Project`] from a file on disk.
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let text =
            std::fs::read_to_string(&path).map_err(|e| format!("Cannot open project file: {e}"))?;

        let mut project = Self::load_from_string(text)?;
        project.path = Some(path.as_ref().to_path_buf());
        Ok(project)
    }

    /// Parse a [`Project`] from a string
    pub fn load_from_string(text: String) -> Result<Self, String> {
        let project: Project =
            serde_json::from_str(&text).map_err(|e| format!("Invalid JSON: {e}"))?;

        if project.project_file_version > CURRENT_PROJECT_FILE_VERSION {
            return Err(format!(
                "Incorrect project file version (expected {}, got {})",
                CURRENT_PROJECT_FILE_VERSION, project.project_file_version
            ));
        }

        Ok(project)
    }

    /// Gets a string containing JSON-formatted text that represents this [`Project`]
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }

    /// Saves a [`Project`] as JSON-formatted text to a file on disk
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
        std::fs::write(path, self.to_json()).map_err(|e| format!("Cannot write project file: {e}"))
    }

    /// Gets the path of the directory from which to start searching for .yarn
    /// files. This value is null if the directory does not exist on disk.
    fn search_directory(&self) -> Option<PathBuf> {
        let path = self.path.as_ref()?;

        if path.is_dir() {
            // This project refers to a directory on disk.
            Some(path.clone())
        } else if path.is_file() {
            // This project refers to a .yarnproject on disk.
            path.parent().map(|p| p.to_path_buf())
        } else {
            // This project does not refer to a file on disk or to a directory.
            None
        }
    }

    /// Build the [`GlobSet`] based on source files.
    fn build_source_globset(&self) -> GlobSet {
        let mut builder = GlobSetBuilder::new();

        for pattern in &self.source_file_patterns {
            builder.add(Glob::new(pattern).unwrap());
        }

        builder.build().unwrap()
    }

    /// Build the [`GlobSet`] based on exclude files.
    fn build_exclude_globset(&self) -> GlobSet {
        let mut builder = GlobSetBuilder::new();

        for pattern in &self.exclude_file_patterns {
            builder.add(Glob::new(pattern).unwrap());
        }

        builder.build().unwrap()
    }

    /// Gets the collection of Yarn files that should be used to compile the project.
    ///
    /// This collection uses a [`GlobSet`] to find all files specified by `source_files`,
    /// excluding those that are specified by `exclude_files`.
    pub fn source_files(&self) -> Vec<PathBuf> {
        let Some(root) = self.search_directory() else {
            return Vec::new();
        };

        let source_matcher = self.build_source_globset();
        let exclude_matcher = self.build_exclude_globset();

        // This is an explicit, absolute path to a Yarn file (which the globbing matcher won't pick up)
        // manually add it to the list of paths that this project references
        let absolute_files = self.source_file_patterns.iter().filter_map(|pattern| {
            PathBuf::try_from(pattern.clone()).ok().and_then(|path| {
                if path.is_absolute() && path.extension().is_some_and(|ext| ext == "yarn") {
                    Some(path)
                } else {
                    None
                }
            })
        });

        WalkDir::new(&root)
            .into_iter()
            .filter_map(Result::ok)
            .map(|e| e.path().to_path_buf())
            .filter(|path| source_matcher.is_match(path) && !exclude_matcher.is_match(path))
            .chain(absolute_files)
            .collect()
    }

    /// Gets the path to the Definitions file, relative to this project's location.
    #[deprecated(since = "0.9.0", note = "please use `definitions_files` instead")]
    pub fn definitions_path(&self) -> Option<PathBuf> {
        let root = self.search_directory()?;
        let defs = self.definitions.as_ref()?;
        Some(root.join(defs))
    }

    /// Gets the absolute paths to the project's Definitions files.
    pub fn definitions_files(&self) -> Vec<PathBuf> {
        let pattern = match self.definitions_files_pattern() {
            Some(p) => p,
            None => return vec![],
        };

        let path = Path::new(&pattern);

        let base_dir = if path.is_absolute() {
            if path.is_file() {
                // The path is to an absolute path that exists on disk; return it as-is
                return vec![path.to_path_buf()];
            }
            // The path is absolute but doesn't exist; it may be a
            // pattern. Split the pattern into an absolute path and
            // the pattern, and attempt to match from there.
            glob_base_dir(path)
        } else {
            // The path is not absolute, so we can use the matcher
            // directly to find paths, starting from our
            match self.search_directory() {
                Some(dir) => dir,
                // We don't know where to start searching from.
                None => return vec![],
            }
        };

        let matcher = match Glob::new(&pattern) {
            Ok(g) => g.compile_matcher(),
            Err(_) => return vec![],
        };

        WalkDir::new(&base_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter(|e| matcher.is_match(e.path()))
            .map(|e| e.into_path())
            .collect()
    }

    fn definitions_files_pattern(&self) -> Option<String> {
        let definitions = self.definitions.as_ref()?;

        if definitions.contains(WORKSPACE_ROOT_PLACEHOLDER) {
            let root = self.workspace_root_path.as_ref()?;
            if root.is_dir() {
                Some(definitions.replace(WORKSPACE_ROOT_PLACEHOLDER, &root.to_string_lossy()))
            } else {
                None
            }
        } else if let Some(search_dir) = &self.search_directory() {
            Some(search_dir.join(definitions).to_string_lossy().into_owned())
        } else {
            None
        }
    }

    /// Gets a value indicating whether the given path is a path
    /// that is included in this project.
    pub fn is_matching_path<P: AsRef<Path>>(&self, path: P) -> bool {
        let Some(root) = self.search_directory() else {
            return false;
        };

        let full_path = root.join(path);

        let source_matcher = self.build_source_globset();
        let exclude_matcher = self.build_exclude_globset();
        source_matcher.is_match(&full_path) && !exclude_matcher.is_match(&full_path)
    }

    /// Gets a value indicating whether this Project is an 'implicit'
    /// project (that is, it does not currently represent a file that exists
    /// on disk.)
    ///
    /// An implicit project is created by tools like the Yarn Spinner
    /// Language Server when opening a folder that contains Yarn files but
    /// no Yarn Project file.
    #[allow(dead_code)] // Seems not used in V3.0, but may be in the future.
    pub(crate) fn is_implicit(&self) -> bool {
        self.path.as_ref().is_some_and(|path| path.exists())
    }

    fn get_compiler_options_flag(&self, key: impl AsRef<str>) -> bool {
        self.compiler_options
            .get(key.as_ref())
            .and_then(|value| match value {
                JsonValue::Bool(b) => Some(*b),
                _ => None,
            })
            .unwrap_or_default()
    }

    fn set_compiler_options_flag(&mut self, key: impl AsRef<str>, value: bool) {
        self.compiler_options
            .insert(key.as_ref().to_owned(), JsonValue::Bool(value));
    }

    /// Gets a value indicating whether compiler features that are
    /// not intended for production use are allowed.
    pub fn allow_language_preview_features(&self) -> bool {
        self.get_compiler_options_flag(ALLOW_PREVIEW_FEATURES_KEY)
    }

    /// Sets a value indicating whether compiler features that are
    /// not intended for production use are allowed.
    pub fn set_allow_language_preview_features(&mut self, value: bool) {
        self.set_compiler_options_flag(ALLOW_PREVIEW_FEATURES_KEY, value);
    }

    /// Gets a value indicating whether the argument is a valid
    /// Yarn Spinner version number.
    pub fn is_valid_version_number(version: u32) -> bool {
        version == YARNSPINNER_PROJECT_VERSION_2 || version == YARNSPINNER_PROJECT_VERSION_3
    }
}

/// Returns the deepest directory path before the first glob wildcard.
/// e.g. `/home/user/projects/**/*.yarn` -> `/home/user/projects`
fn glob_base_dir(pattern: &Path) -> PathBuf {
    let mut base = PathBuf::new();
    for component in pattern.components() {
        let s = component.as_os_str().to_string_lossy();
        if s.contains('*') {
            break;
        }
        base.push(component);
    }
    base
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};

    #[test]
    fn absolute_path_with_double_star() {
        let result = glob_base_dir(Path::new("/home/user/projects/**/*.yarn"));
        assert_eq!(result, PathBuf::from("/home/user/projects"));
    }

    #[test]
    fn absolute_path_with_star_in_filename() {
        let result = glob_base_dir(Path::new("/home/user/*.yarn"));
        assert_eq!(result, PathBuf::from("/home/user"));
    }

    #[test]
    fn absolute_path_no_glob() {
        let result = glob_base_dir(Path::new("/home/user/projects/file.yarn"));
        assert_eq!(result, PathBuf::from("/home/user/projects/file.yarn"));
    }

    #[test]
    fn glob_at_root() {
        let result = glob_base_dir(Path::new("/*.yarn"));
        assert_eq!(result, PathBuf::from("/"));
    }

    #[test]
    fn relative_path_with_glob() {
        let result = glob_base_dir(Path::new("src/**/*.yarn"));
        assert_eq!(result, PathBuf::from("src"));
    }

    #[test]
    fn only_glob() {
        let result = glob_base_dir(Path::new("**/*.yarn"));
        assert_eq!(result, PathBuf::from(""));
    }
}
