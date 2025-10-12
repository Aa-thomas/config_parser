pub mod file_io;
pub mod not_supported;
pub mod parse;
pub mod path;
pub mod usage;
pub mod value_type;

pub use file_io::FileIoError;
pub use not_supported::NotSupportedError;
pub use parse::ParseError;
pub use path::PathError;
pub use usage::UsageError;
pub use value_type::TypeError;

use std::process::ExitCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error(transparent)]
    Usage(#[from] UsageError),

    #[error(transparent)]
    FileIO(#[from] FileIoError),

    #[error(transparent)]
    Parse(#[from] ParseError),

    #[error(transparent)]
    Path(#[from] PathError),

    #[error(transparent)]
    Type(#[from] TypeError),

    #[error(transparent)]
    NotSupported(#[from] NotSupportedError),
}

impl ConfigError {
    pub fn exit_code(&self) -> ExitCode {
        match self {
            ConfigError::Usage(_) => ExitCode::from(2),
            ConfigError::FileIO(_) => ExitCode::from(3),
            ConfigError::Parse(_) => ExitCode::from(3),
            ConfigError::Path(_) => ExitCode::from(4),
            ConfigError::Type(_) => ExitCode::from(5),
            ConfigError::NotSupported(_) => ExitCode::from(6),
        }
    }
}
