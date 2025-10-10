use std::process::ExitCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error(transparent)]
    Usage(#[from] UsageError),
}

#[derive(Debug, Error)]
pub enum UsageError {
    #[error("UsageError: missing required flag {flag}. {hint}")]
    MissingFlag { flag: String, hint: String },

    #[error(
        "UsageError: invalid value '{provided}' for {flag}. \
        Valid options: {valid:?}. Example: --format json"
    )]
    InvalidChoice {
        provided: String,
        flag: String,
        valid: Vec<String>,
    },

    #[error("UsageError: flags {a} and {b} cannot be used together. {hint}")]
    ConflictingFlags { a: String, b: String, hint: String },

    #[error("UsageError: missing argument {name}. Example: {example}")]
    MissingArgument { name: String, example: String },

    #[error(
        "UsageError: invalid key-path syntax: '{input}'. \
    Example: {example}"
    )]
    InvalidPathSyntax { input: String, example: String },
}

impl ConfigError {
    pub fn exit_code(&self) -> ExitCode {
        match self {
            ConfigError::Usage(_) => ExitCode::from(2),
        }
    }
}
