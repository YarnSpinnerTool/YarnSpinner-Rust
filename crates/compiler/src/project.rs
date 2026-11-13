//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/v2.5.0/YarnSpinner.Compiler/Project.cs>

use std::path::Path;
use std::{collections::HashMap, path::PathBuf};

#[cfg(feature = "bevy")]
use bevy::prelude::*;
use globset::{Glob, GlobSet, GlobSetBuilder};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

/// Current version of the .yarnproject
pub const CURRENT_PROJECT_FILE_VERSION: u32 = 2;

/// Yarn Projects represent instructions on where to find Yarn scripts and
/// associated assets, and how they should be compiled.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "bevy", derive(Reflect))]
pub struct Project {
    /// Gets or sets the file version of the project.
    ///
    /// This value is required to be equal to [`CURRENT_PROJECT_FILE_VERSION`]
    pub project_file_version: u32,

    /// Gets the path that the [`Project`]` was loaded from.
    ///
    /// This value is not stored when the file is saved, but is instead
    /// determined when the file is loaded by [`self::load_from_file()`]
    #[cfg_attr(feature = "serde", serde(skip))]
    pub path: Option<PathBuf>,

    /// Gets or sets the collection of file search patterns used to locate
    /// Yarn files that form this project.
    #[cfg_attr(
        feature = "serde",
        serde(rename = "sourceFiles", default = "default_source_files")
    )]
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
    #[cfg_attr(feature = "serde", serde(rename = "excludeFiles", default))]
    pub exclude_file_patterns: Vec<String>,

    /// Gets or sets the collection of [`LocalizationInfo`]
    /// objects that store information about where localized data for this
    /// project is found.
    #[cfg_attr(feature = "serde", serde(default))]
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
    #[cfg_attr(feature = "serde", serde(default))]
    pub definitions: Option<String>,

    /// Gets or sets a dictionary containing instructions that control how
    /// the Yarn Spinner compiler should compile a project.
    ///
    /// Note: Unsure about the type of options. No documentation in v2.5.0.
    #[cfg_attr(feature = "serde", serde(default))]
    pub compiler_options: HashMap<String, String>,
}

fn default_source_files() -> Vec<String> {
    vec!["**/*.yarn".to_owned()]
}

/// Stores the locations of where localized assets and a localized
/// string table for a Yarn Project may be found.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", derive(bevy::prelude::Reflect))]
pub struct LocalizationInfo {
    /// Gets or sets the location at which localized assets may be found.
    #[cfg_attr(feature = "serde", serde(default))]
    pub assets: Option<String>,
    /// Gets or sets the location at which the localized string table may be found.
    #[cfg_attr(feature = "serde", serde(default))]
    pub strings: Option<String>,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            project_file_version: CURRENT_PROJECT_FILE_VERSION,
            path: Default::default(),
            source_file_patterns: default_source_files(),
            exclude_file_patterns: Default::default(),
            localisation: Default::default(),
            base_language: Default::default(),
            definitions: Default::default(),
            compiler_options: Default::default(),
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

    /// Loads and parses a [`Project`] from a file on disk.
    #[cfg(feature = "serde")]
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let text =
            std::fs::read_to_string(&path).map_err(|e| format!("Cannot open project file: {e}"))?;

        let mut project: Project =
            serde_json::from_str(&text).map_err(|e| format!("Invalid JSON: {e}"))?;

        if project.project_file_version != CURRENT_PROJECT_FILE_VERSION {
            return Err(format!(
                "Incorrect project file version (expected {}, got {})",
                CURRENT_PROJECT_FILE_VERSION, project.project_file_version
            ));
        }

        project.path = Some(path.as_ref().to_path_buf());
        Ok(project)
    }

    /// Gets a string containing JSON-formatted text that represents this [`Project`]
    #[cfg(feature = "serde")]
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }

    /// Saves a [`Project`] as JSON-formatted text to a file on disk
    #[cfg(feature = "serde")]
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

        WalkDir::new(&root)
            .into_iter()
            .filter_map(Result::ok)
            .map(|e| e.path().to_path_buf())
            .filter(|path| source_matcher.is_match(path) && !exclude_matcher.is_match(path))
            .collect()
    }

    /// Gets the path to the Definitions file, relative to this project's location.
    pub fn definitions_path(&self) -> Option<PathBuf> {
        let root = self.search_directory()?;
        let defs = self.definitions.as_ref()?;
        Some(root.join(defs))
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

    /// Checks a path relative to the project's root. Does not check that the root itself exists.
    pub fn is_matching_path_relative<P: AsRef<Path>>(&self, path: P) -> bool {
        self.build_source_globset().is_match(&path) && !self.build_exclude_globset().is_match(&path)
    }
}
