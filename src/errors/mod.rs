pub mod file_io;
pub mod usage;

pub use file_io::FileIoError;
pub use usage::UsageError;

use std::process::ExitCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error(transparent)]
    Usage(#[from] UsageError),

    #[error(transparent)]
    FileIO(#[from] FileIoError),
}

impl ConfigError {
    pub fn exit_code(&self) -> ExitCode {
        match self {
            ConfigError::Usage(_) => ExitCode::from(2),
            ConfigError::FileIO(_) => ExitCode::from(3),
        }
    }
}
