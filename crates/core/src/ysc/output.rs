use crate::prelude::*;
use std::path::{Path, PathBuf};
use thiserror::Error;

/// Failed to read the output of `ysc compile`. See [`CompiledFile::read`].
#[derive(Debug, Error)]
pub enum ReadCompiledError {
    /// A file couldn't be read.
    #[error("failed to read {}: {source}", path.display())]
    Io {
        /// The file that couldn't be read.
        path: PathBuf,
        /// The underlying error.
        source: std::io::Error,
    },
    /// The `.yarnc` program is invalid.
    #[error("invalid program in {}: {source}", path.display())]
    Program {
        /// The `.yarnc` file.
        path: PathBuf,
        /// The underlying error.
        source: DecodeError,
    },
    /// A `-Lines.csv` or `-Metadata.csv` file is invalid.
    #[error("invalid CSV in {}: {source}", path.display())]
    Csv {
        /// The CSV file.
        path: PathBuf,
        /// The underlying error.
        source: CsvParseError,
    },
}

impl CompiledFile {
    /// Reads `{dir}/{name}.yarnc`, `{dir}/{name}-Lines.csv` and `{dir}/{name}-Metadata.csv`,
    /// as written by `ysc compile <inputs> -o {dir}`.
    ///
    /// Without `-n`, ysc names its outputs after the input's file stem if there is exactly one
    /// input (e.g. `story.yarnproject` → `story`), and `Output` otherwise.
    pub fn read(dir: impl AsRef<Path>, name: &str) -> Result<Self, ReadCompiledError> {
        let dir = dir.as_ref();
        let read = |suffix: &str| {
            let path = dir.join(format!("{name}{suffix}"));
            std::fs::read(&path)
                .map(|bytes| (path.clone(), bytes))
                .map_err(|source| ReadCompiledError::Io { path, source })
        };
        let (path, bytes) = read(".yarnc")?;
        let program = Program::from_bytes(&bytes)
            .map_err(|source| ReadCompiledError::Program { path, source })?;
        let (path, bytes) = read("-Lines.csv")?;
        let lines = YscLines::from_bytes(&bytes)
            .map_err(|source| ReadCompiledError::Csv { path, source })?;
        let (path, bytes) = read("-Metadata.csv")?;
        let metadata = YscMetadata::from_bytes(&bytes)
            .map_err(|source| ReadCompiledError::Csv { path, source })?;
        Ok(Self {
            program,
            lines,
            metadata,
        })
    }
}
