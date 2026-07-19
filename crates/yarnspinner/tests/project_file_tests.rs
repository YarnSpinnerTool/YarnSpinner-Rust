//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/v2.5.0/YarnSpinner.Tests/ProjectFileTests.cs>
#![cfg(feature = "serde")]

use std::env;

use test_base::prelude::*;
use yarnspinner::compiler::*;

mod test_base;

#[test]
fn test_project_file_can_be_loaded() {
    // Given
    let path = space_demo_scripts_path().join("Space.yarnproject");
    let project = Project::load_from_file(path.clone()).unwrap();

    // Then
    assert_eq!(project.project_file_version, 3);
    assert_eq!(project.path, Some(path));
    assert_eq!(project.source_file_patterns, vec!["**/*.yarn"]);

    assert_eq!(project.base_language, "en");
    assert_eq!(project.definitions, Some("Functions.ysls.json".to_owned()));

    assert!(project.localisation.contains_key("en"));
    assert_eq!(
        *project.localisation.get("en").unwrap(),
        LocalizationInfo {
            assets: Some("../LocalisedAssets/English".to_owned()),
            strings: None
        }
    );

    assert!(project.localisation.contains_key("de"));
    assert_eq!(
        *project.localisation.get("de").unwrap(),
        LocalizationInfo {
            assets: Some("../LocalisedAssets/German".to_owned()),
            strings: Some("../German.csv".to_owned())
        }
    );
}

#[test]
fn test_projects_can_find_files() {
    // Given
    let path = space_demo_scripts_path().join("Space.yarnproject");
    let project = Project::load_from_file(path.clone()).unwrap();

    // When
    let relative_files = project
        .source_files()
        .iter()
        .map(|f| f.file_name().unwrap().to_owned())
        .collect::<Vec<_>>();

    //Then
    assert!(relative_files.contains(&"Sally.yarn".to_string().into()));
    assert!(relative_files.contains(&"Ship.yarn".to_string().into()));
    assert!(!relative_files.contains(&"Space.yarnproject".to_string().into()));
}

#[test]
fn test_projects_can_exclude_files() {
    // Given
    let path = space_demo_scripts_path().join("Space.yarnproject");
    let mut project = Project::load_from_file(path.clone()).unwrap();
    project
        .exclude_file_patterns
        .push("**/Ship.yarn".to_owned());

    // When
    let relative_files = project
        .source_files()
        .iter()
        .map(|f| f.file_name().unwrap().to_owned())
        .collect::<Vec<_>>();

    //Then
    assert!(relative_files.contains(&"Sally.yarn".to_string().into()));
    assert!(!relative_files.contains(&"Ship.yarn".to_string().into()));
    assert!(!relative_files.contains(&"Space.yarnproject".to_string().into()));
}

#[test]
fn test_projects_can_save() {
    // Given
    let project = Project::default();
    let path = env::temp_dir().join("test_projects_can_save.yarnproject");
    println!(
        "Temporary file for test_projects_can_save is {}",
        path.to_str().unwrap()
    );

    // When
    project.save_to_file(&path).unwrap();
    let loaded_project = Project::load_from_file(&path).unwrap();

    // Then
    assert_eq!(
        project.project_file_version,
        loaded_project.project_file_version
    );
    assert_eq!(
        project.source_file_patterns,
        loaded_project.source_file_patterns
    );
    assert_eq!(
        project.exclude_file_patterns,
        loaded_project.exclude_file_patterns
    );
    assert_eq!(project.localisation, loaded_project.localisation);
    assert_eq!(project.base_language, loaded_project.base_language);
    assert_eq!(project.definitions, loaded_project.definitions);
    assert_eq!(project.compiler_options, loaded_project.compiler_options);
}

#[test]
fn test_projects_can_be_modified_and_saved() {
    // Given
    let path = space_demo_scripts_path().join("Space.yarnproject");
    let mut project = Project::load_from_file(path.clone()).unwrap();

    project.localisation.insert(
        "fr".to_owned(),
        LocalizationInfo {
            assets: Some("./French/".to_owned()),
            strings: Some("French.csv".to_owned()),
        },
    );

    // When

    let path = env::temp_dir().join("test_projects_can_be_modified_and_saved.yarnproject");
    println!(
        "Temporary file for test_projects_can_be_modified_and_saved is {}",
        path.to_str().unwrap()
    );
    project.save_to_file(&path).unwrap();
    let loaded_project = Project::load_from_file(&path).unwrap();

    // Then
    assert_eq!(
        project.project_file_version,
        loaded_project.project_file_version
    );
    assert_eq!(
        project.source_file_patterns,
        loaded_project.source_file_patterns
    );
    assert_eq!(
        project.exclude_file_patterns,
        loaded_project.exclude_file_patterns
    );
    assert_eq!(project.localisation, loaded_project.localisation);
    assert_eq!(project.base_language, loaded_project.base_language);
    assert_eq!(project.definitions, loaded_project.definitions);
    assert_eq!(project.compiler_options, loaded_project.compiler_options);
}

#[test]
fn test_project_files_can_all_preview_features() {
    let project_source = "
{
    \"projectFileVersion\": 2,
    \"sourceFiles\": [\"**/*.yarn\"],
    \"baseLanguage\": \"en\",
    \"compilerOptions\": {
        \"allowPreviewFeatures\": true
    }
}
    ";

    let project = Project::load_from_string(project_source.into()).unwrap();

    assert!(project.allow_language_preview_features());
}
