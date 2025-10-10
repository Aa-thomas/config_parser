use std::process::ExitCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error(transparent)]
    Usage(#[from] UsageError),

    #[error(transparent)]
    FileIO(#[from] FileIoError),
}

#[derive(Debug, Error)]
pub enum UsageError {
    #[error("UsageError: missing required flag {flag}. {hint}")]
    MissingFlag {
        flag: &'static str,
        hint: &'static str,
    },

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

#[derive(Debug, Error)]
pub enum FileIoError {
    #[error("FileIoError: could not read {path}: {reason}. {hint}")]
    ReadFailed {
        path: String,
        reason: String,
        hint: String,
    },

    #[error("FileIoError: could not write {path}: {reason}. {hint}")]
    WriteFailed {
        path: String,
        reason: String,
        hint: String,
    },

    #[error("FileIoError: could not create temp file near {path}: {reason}. {hint}")]
    TempCreateFailed {
        path: String,
        reason: String,
        hint: String,
    },

    #[error("FileIoError: could not atomically replace {final_path} (from {temp_path}): {reason}. {hint}")]
    AtomicReplaceFailed {
        temp_path: String,
        final_path: String,
        reason: String,
        hint: String,
    },
}

impl ConfigError {
    pub fn exit_code(&self) -> ExitCode {
        match self {
            ConfigError::Usage(_) => ExitCode::from(2),
            ConfigError::FileIO(_) => ExitCode::from(3),
        }
    }
}

#[cfg(test)]
fn usage_missing_flag_is_one_line() {
    let err = ConfigError::from(UsageError::MissingFlag {
        flag: "-- format <json|toml>",
        hint: "Use --format when file extension is not json/toml",
    });

    let msg = format!("{}", err);

    assert!(msg.starts_with("UsageError:"), "should prefix category");
    assert!(
        msg.contains("--format <json|toml>"),
        "should mention missing flag"
    );
    assert!(
        msg.contains("Use --format"),
        "should include a helpful hint"
    );
    assert!(!msg.contains('\n'), "should be a single line");
}
