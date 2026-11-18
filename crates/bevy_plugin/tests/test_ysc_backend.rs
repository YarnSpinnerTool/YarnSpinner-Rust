#![cfg(not(feature = "compiler"))]

use bevy::prelude::*;
use bevy_yarnspinner::prelude::*;
use std::fs;
use std::path::Path;
use tempfile::tempdir;
use utils::prelude::*;

mod utils;

fn copy_fixture_sources(dest: &Path) {
    let from = project_root_path().join("assets/ysc");
    let to = dest.join("ysc");
    fs::create_dir_all(&to).unwrap();
    for file in ["start.yarn", "shop.yarn", "project.yarnproject"] {
        fs::copy(from.join(file), to.join(file)).unwrap();
    }
}

fn texts(project: &YarnProject) -> Vec<String> {
    let mut texts: Vec<_> = project
        .compiled_file()
        .texts()
        .map(|(_, text)| text.to_owned())
        .collect();
    texts.sort();
    texts
}

#[test]
fn loads_precompiled_project_without_ysc() {
    let dir = tempdir().unwrap();
    copy_fixture_sources(dir.path());
    compile_yarn(dir.path(), &["ysc/project.yarnproject"], "ysc/compiled");

    let mut app = App::new();
    app.setup_default_plugins_for_path(dir.path())
        .insert_resource(YscSettings {
            ysc_path: "/definitely/not/ysc".into(),
            output_dir: "ysc/compiled".into(),
        })
        .add_plugins(YarnSpinnerPlugin::from_yarnproject(
            YarnProjectSource::file("ysc/project.yarnproject"),
        ))
        .add_systems(
            Update,
            check_precompiled_project.run_if(resource_exists::<YarnProject>),
        );
    assert!(app.run().is_success());
}

fn check_precompiled_project(project: Res<YarnProject>, mut commands: Commands) {
    let texts = texts(&project);
    cmd_assert!(
        commands,
        texts
            == [
                "Go to the shop",
                "Leave",
                "Narrator: Welcome.",
                "Shopkeeper: What'll it be?"
            ],
        "Wrong texts: {texts:?}"
    );
    let metadata = project.line_metadata(&LineId("line:shop_1".into()));
    cmd_assert!(
        commands,
        metadata == Some(&["line:shop_1".to_owned(), "mood:grumpy".to_owned()][..]),
        "Wrong metadata: {metadata:?}"
    );
    commands.write_message(AppExit::Success);
}

#[test]
fn compiles_manual_file_list_on_startup_when_watching() {
    let dir = tempdir().unwrap();
    copy_fixture_sources(dir.path());
    let compiled = dir.path().join("ysc/compiled/Output.yarnc");

    let mut app = App::new();
    app.setup_default_plugins_for_path_with_watching(dir.path(), true)
        .insert_resource(YscSettings {
            output_dir: "ysc/compiled".into(),
            ..Default::default()
        })
        .add_plugins(YarnSpinnerPlugin::with_yarn_sources([
            YarnFileSource::file("ysc/start.yarn"),
            YarnFileSource::file("ysc/shop.yarn"),
        ]))
        .add_systems(
            Update,
            (move |project: Res<YarnProject>, mut commands: Commands| {
                cmd_assert!(commands, compiled.is_file(), "ysc wrote no program");
                let count = texts(&project).len();
                cmd_assert!(commands, count == 4, "Expected 4 lines, got {count}");
                commands.write_message(AppExit::Success);
            })
            .run_if(resource_exists::<YarnProject>),
        );
    assert!(app.run().is_success());
}

#[test]
fn recompiles_project_when_a_source_changes() {
    let dir = tempdir().unwrap();
    copy_fixture_sources(dir.path());
    let start = dir.path().join("ysc/start.yarn");

    let mut app = App::new();
    app.setup_default_plugins_for_path_with_watching(dir.path(), true)
        .insert_resource(YscSettings {
            output_dir: "ysc/compiled".into(),
            ..Default::default()
        })
        .add_plugins(YarnSpinnerPlugin::from_yarnproject(
            YarnProjectSource::file("ysc/project.yarnproject"),
        ))
        .add_systems(
            Update,
            (move |project: Res<YarnProject>, mut edited: Local<bool>, mut commands: Commands| {
                let texts = texts(&project);
                if !*edited {
                    cmd_assert!(
                        commands,
                        texts.contains(&"Narrator: Welcome.".to_owned()),
                        "Wrong initial texts: {texts:?}"
                    );
                    let source = fs::read_to_string(&start).unwrap();
                    fs::write(&start, source.replace("Welcome.", "Welcome back.")).unwrap();
                    *edited = true;
                } else if texts.contains(&"Narrator: Welcome back.".to_owned()) {
                    commands.write_message(AppExit::Success);
                }
            })
            .run_if(resource_exists::<YarnProject>),
        );
    assert!(app.run().is_success());
}
