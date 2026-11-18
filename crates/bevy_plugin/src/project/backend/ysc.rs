//! Compiles with `ysc` (external).
//!
//! This will hot-reload any watched yarn files as specified by either the
//! `.yarnproject` file or the [`ManualYarnSpinnerPlugin`]. Hot reloading does
//! not work on wasm and android as they cannot access the filesystem and cannot
//! call external programs
//!
//! If hot reloading is disabled, the compilation step is skipped and the precompiled
//! programs are assumed to be on disk. This means the user will need to run `ysc` at least
//! once, whether that's in the game during the development phase or manually.
//!
//! The precompiled program is then loaded from `{output_dir}/{name}.yarnc`,
//! `{name}-Lines.csv` and `{name}-Metadata.csv`, using ysc's default `{name}`;
//! see [`output_name`].

use crate::prelude::*;
#[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
use crate::project::YarnProject;
use crate::project::{
    CompilationStep, DEFAULT_ASSET_DIR, YarnFilesBeingLoaded, YarnProjectConfigToLoad,
    apply_compiled_file,
};
#[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
use crate::utils::log_error;
// use crate::utils::panic_on_err;
use anyhow::bail;
use bevy::asset::{AssetPath, LoadState, UntypedAssetId};
use bevy::prelude::*;
#[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
use bevy::tasks::{AsyncComputeTaskPool, Task, futures::check_ready};
use std::path::{Path, PathBuf};

/// The name ysc gives its outputs when no `-n` is passed: the stem of the input if there is
/// exactly one (e.g. the `.yarnproject`), otherwise `Output`
/// (`YarnSpinner.Console/Commands/CompileCommand.cs:33` at v2.5.0).
fn output_name<'a>(
    inputs: &YscInputs,
    yarn_files: impl ExactSizeIterator<Item = &'a Handle<YarnFile>>,
) -> String {
    let stem = |path: &AssetPath| {
        path.path()
            .file_stem()
            .map(|stem| stem.to_string_lossy().into_owned())
    };
    let single = match &inputs.yarnproject {
        Some(yarnproject) => stem(yarnproject),
        None if yarn_files.len() == 1 => yarn_files
            .into_iter()
            .next()
            .and_then(|handle| handle.path().and_then(stem)),
        None => None,
    };
    single.unwrap_or_else(|| "Output".to_owned())
}

/// Parameters passed to `ysc`.
/// Insert this resource before the project loads to override the defaults.
#[derive(Debug, Clone, PartialEq, Eq, Resource, Reflect)]
#[reflect(Debug, Resource, Default, PartialEq)]
pub struct YscSettings {
    /// The `ysc` executable. Only used when hot reloading is on, and never on Wasm or Android.
    /// If `None`, uses the `YSC` environment variable, falling back to `ysc` on the `PATH`.
    pub ysc_path: PathBuf,
    /// The directory, relative to the asset root, that `ysc` writes to and that the compiled
    /// program is loaded from. If `None`, uses `dialogue/compiled`.
    pub output_dir: PathBuf,
}
impl Default for YscSettings {
    fn default() -> Self {
        let ysc_path = std::env::var_os("YSC")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("ysc"));
        let output_dir = Path::new(DEFAULT_ASSET_DIR).join("compiled");
        Self {
            ysc_path,
            output_dir,
        }
    }
}

pub(crate) fn plugin(app: &mut App) {
    app.register_type::<YscSettings>()
        .init_resource::<YscSettings>()
        .init_resource::<CompiledOutputs>()
        .init_resource::<YscInputs>();

    #[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
    app.init_resource::<YscJob>();

    app.add_systems(
        Update,
        (
            build_yarn_project
                .pipe(panic_on_err)
                .run_if(resource_exists::<YarnProjectConfigToLoad>),
            #[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
            recompile_on_source_change
                .pipe(log_error)
                .run_if(resource_exists::<YarnProject>),
        )
            .chain()
            .in_set(CompilationStep::Compile),
    );
}

/// Handles to the three files `ysc` writes.
#[derive(Debug, Clone)]
struct OutputHandles {
    program: Handle<YscProgram>,
    lines: Handle<YscLines>,
    metadata: Handle<YscMetadata>,
}

impl OutputHandles {
    fn load(asset_server: &AssetServer, output_dir: &Path, name: &str) -> Self {
        // Asset paths always use `/`, regardless of platform.
        let path = |suffix: &str| {
            let path = output_dir.join(format!("{name}{suffix}"));
            path.to_string_lossy().replace('\\', "/")
        };
        Self {
            program: asset_server.load(path(".yarnc")),
            lines: asset_server.load(path("-Lines.csv")),
            metadata: asset_server.load(path("-Metadata.csv")),
        }
    }

    fn ids(&self) -> [UntypedAssetId; 3] {
        [
            self.program.id().untyped(),
            self.lines.id().untyped(),
            self.metadata.id().untyped(),
        ]
    }

    fn get(
        &self,
        programs: &Assets<YscProgram>,
        lines: &Assets<YscLines>,
        metadata: &Assets<YscMetadata>,
    ) -> Option<CompiledFile> {
        Some(CompiledFile {
            program: programs.get(&self.program)?.0.clone(),
            lines: lines.get(&self.lines)?.0.clone(),
            metadata: metadata.get(&self.metadata)?.0.clone(),
        })
    }
}

#[derive(Debug, Default, Resource)]
struct CompiledOutputs(Option<OutputHandles>);

/// What gets passed to `ysc`. Kept around after loading for recompilation.
#[derive(Debug, Default, Resource)]
struct YscInputs {
    yarnproject: Option<AssetPath<'static>>,
    /// see [`output_name`]
    name: String,
}

/// The running `ysc` process, if any. `ysc` runs off the main thread so it doesn't stall
/// the app; [`build_yarn_project`] and [`recompile_on_source_change`] poll it.
#[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
#[derive(Default, Resource)]
struct YscJob {
    task: Option<Task<anyhow::Result<CompiledFile>>>,
    /// A source changed while `ysc` was running; run it again once it finishes.
    rerun: bool,
}

#[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
impl YscJob {
    /// Returns the result of the running `ysc`, if it just finished.
    fn poll(&mut self) -> Option<anyhow::Result<CompiledFile>> {
        let result = check_ready(self.task.as_mut()?)?;
        self.task = None;
        Some(result)
    }
}

/// Compiles (if hot reloading) and loads the outputs, then inserts the [`YarnProject`].
#[expect(clippy::too_many_arguments, reason = "Bevy system")]
fn build_yarn_project(
    mut commands: Commands,
    config: Res<YarnProjectConfigToLoad>,
    mut yarn_files_being_loaded: ResMut<YarnFilesBeingLoaded>,
    #[cfg_attr(
        any(target_arch = "wasm32", target_os = "android"),
        expect(unused_variables, reason = "ysc is never run on these platforms")
    )]
    yarn_files: Res<Assets<YarnFile>>,
    #[cfg_attr(
        any(target_arch = "wasm32", target_os = "android"),
        expect(unused_variables, reason = "ysc is never run on these platforms")
    )]
    asset_root: Res<crate::plugin::AssetRoot>,
    #[cfg(not(any(target_arch = "wasm32", target_os = "android")))] mut job: ResMut<YscJob>,
    settings: Res<YscSettings>,
    asset_server: Res<AssetServer>,
    mut outputs: ResMut<CompiledOutputs>,
    mut inputs: ResMut<YscInputs>,
    programs: Res<Assets<YscProgram>>,
    lines: Res<Assets<YscLines>>,
    metadata: Res<Assets<YscMetadata>>,
) -> SystemResult {
    let Some(handles) = &outputs.0 else {
        inputs.yarnproject = config.yarnproject.clone();
        inputs.name = output_name(&inputs, yarn_files_being_loaded.0.iter());

        // Hot reloading: compile, then use what ysc wrote directly.
        #[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
        if config.watching_for_changes {
            let compilation = match job.poll() {
                // Still compiling.
                None if job.task.is_some() => return Ok(()),
                None => {
                    let sources = &yarn_files_being_loaded.0;
                    if sources.is_empty() || !sources.iter().all(|h| yarn_files.contains(h)) {
                        return Ok(());
                    }
                    job.task = Some(spawn_ysc(&settings, &asset_root, &inputs, sources.iter())?);
                    return Ok(());
                }
                Some(result) => result?,
            };
            let files = std::mem::take(&mut yarn_files_being_loaded.0);
            commands.insert_resource(YarnProject::new(&config, files, compilation, &asset_server));
            info!("Compiled Yarn project with ysc.");
            return Ok(());
        }

        // Otherwise: load the precompiled outputs as assets.
        outputs.0 = Some(OutputHandles::load(
            &asset_server,
            &settings.output_dir,
            &inputs.name,
        ));
        return Ok(());
    };

    for id in handles.ids() {
        if let Some(LoadState::Failed(e)) = asset_server.get_load_state(id) {
            bail!(
                "Failed to load compiled Yarn output: {e}\n\
                Help: without hot reloading, the project must be compiled ahead of time with \
                `ysc compile <inputs> -o {}`.",
                settings.output_dir.display()
            );
        }
    }
    let Some(compilation) = handles.get(&programs, &lines, &metadata) else {
        return Ok(());
    };

    let files = std::mem::take(&mut yarn_files_being_loaded.0);
    commands.insert_resource(YarnProject::new(&config, files, compilation, &asset_server));
    info!(
        "Loaded compiled Yarn project from {}",
        settings.output_dir.display()
    );
    Ok(())
}

/// Reruns `ysc` when a source changes. Swaps in the new program once it succeeds.
///
/// This happens once `ysc` has finished compiling so we don't need to watch assets and debounce.
#[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
fn recompile_on_source_change(
    mut yarn_file_events: MessageReader<AssetEvent<YarnFile>>,
    mut yarnproject_events: MessageReader<AssetEvent<YarnProjectFile>>,
    mut project: ResMut<YarnProject>,
    mut dialogue_runners: Query<&mut DialogueRunner>,
    settings: Res<YscSettings>,
    inputs: Res<YscInputs>,
    asset_root: Res<crate::plugin::AssetRoot>,
    mut job: ResMut<YscJob>,
) -> SystemResult {
    // only checks for `Modified`. `Added` fires for the initial load, which is already compiled.
    // TODO: Split this by yarnproject.
    let yarn_file_changed = yarn_file_events
        .read()
        .any(|m| matches!(m, AssetEvent::Modified { .. }));
    let project_changed = yarnproject_events
        .read()
        .any(|m| matches!(m, AssetEvent::Modified { .. }));
    yarn_file_events.clear();
    yarnproject_events.clear();

    if project.watching_for_changes && (yarn_file_changed || project_changed) {
        job.rerun = true;
    }
    if let Some(result) = job.poll() {
        apply_compiled_file(&mut project, result?, &mut dialogue_runners);
        info!("Reloaded compiled Yarn project.");
    }
    if job.task.is_none() && std::mem::take(&mut job.rerun) {
        job.task = Some(spawn_ysc(
            &settings,
            &asset_root,
            &inputs,
            project.yarn_files.iter(),
        )?);
    }
    Ok(())
}

/// Spawns `ysc compile {inputs} -o {output_dir}` on the compute task pool. The task then reads
/// the outputs back.
#[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
fn spawn_ysc<'a>(
    settings: &YscSettings,
    asset_root: &crate::plugin::AssetRoot,
    inputs: &YscInputs,
    yarn_files: impl Iterator<Item = &'a Handle<YarnFile>>,
) -> anyhow::Result<Task<anyhow::Result<CompiledFile>>> {
    use anyhow::anyhow;
    use bevy::asset::io::AssetSourceId;

    let disk_path = |path: &AssetPath| -> anyhow::Result<PathBuf> {
        if path.source() != &AssetSourceId::Default {
            bail!("The ysc backend only supports the default asset source, got \"{path}\"");
        }
        Ok(asset_root.0.join(path.path()))
    };

    let mut input_paths = match &inputs.yarnproject {
        Some(yarnproject) => vec![disk_path(yarnproject)?],
        None => yarn_files
            .map(|handle| {
                let path = handle.path().ok_or_else(|| {
                    anyhow!(
                        "In-memory Yarn files can't be compiled by ysc; load them from disk instead"
                    )
                })?;
                disk_path(path)
            })
            .collect::<anyhow::Result<Vec<_>>>()?,
    };
    // Input order changes the .yarnc bytes (not the program); keep it stable.
    input_paths.sort();

    let ysc_path = settings.ysc_path.clone();
    let output_dir = asset_root.0.join(&settings.output_dir);
    let name = inputs.name.clone();
    Ok(AsyncComputeTaskPool::get().spawn(async move {
        run_ysc(&ysc_path, &input_paths, &output_dir)?;
        read_outputs(&output_dir, &name)
    }))
}

/// Runs `ysc`, blocking until it finishes. Call through [`spawn_ysc`].
#[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
fn run_ysc(ysc_path: &Path, input_paths: &[PathBuf], output_dir: &Path) -> anyhow::Result<()> {
    use anyhow::anyhow;

    // ysc fails if the output directory doesn't exist.
    std::fs::create_dir_all(output_dir)?;

    let output = std::process::Command::new(ysc_path)
        .arg("compile")
        .args(input_paths)
        .arg("-o")
        .arg(output_dir)
        .output()
        .map_err(|e| {
            anyhow!(
                "Failed to run ysc at \"{}\": {e}\n\
                Help: install ysc, set the YSC environment variable, or set `YscSettings::ysc_path`.",
                ysc_path.display()
            )
        })?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    if !output.status.success() {
        bail!("ysc failed ({}):\n{stdout}{stderr}", output.status);
    }
    debug!("ysc output:\n{stdout}{stderr}");
    info!(
        "Compiled {} Yarn input(s) with ysc into {}",
        input_paths.len(),
        output_dir.display()
    );
    Ok(())
}

/// Reads and parses the three files ysc wrote to `{output_dir}/{name}*`.
#[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
fn read_outputs(output_dir: &Path, name: &str) -> anyhow::Result<CompiledFile> {
    use anyhow::anyhow;
    use yarnspinner::core::{Program, YscLines, YscMetadata};

    let read = |suffix: &str| {
        let path = output_dir.join(format!("{name}{suffix}"));
        std::fs::read(&path).map_err(|e| anyhow!("Failed to read {}: {e}", path.display()))
    };
    Ok(CompiledFile {
        program: Program::from_bytes(&read(".yarnc")?)?,
        lines: YscLines::from_bytes(&read("-Lines.csv")?)?,
        metadata: YscMetadata::from_bytes(&read("-Metadata.csv")?)?,
    })
}
