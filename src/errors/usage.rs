use thiserror::Error;

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

#[cfg(test)]
fn usage_missing_flag_is_one_line() {
    let err = UsageError::MissingFlag {
        flag: "-- format <json|toml>",
        hint: "Use --format when file extension is not json/toml",
    };

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
