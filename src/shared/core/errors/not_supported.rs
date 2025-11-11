// src/errors/not_supported.rs

use thiserror::Error;

/// Things the CLI *knows about* but intentionally does not support (yet or ever).
/// Use this when the user's request is understood, but we’ve decided not to implement it
/// (format/command/flag/feature/platform/version constraints, etc.).
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum NotSupportedError {
    /// Operation is understood but not supported for the given *config format*.
    /// Example: "set --append" is not supported for TOML arrays in this version.
    #[error("Not supported for format {format}: {op}. {hint}")]
    Format {
        format: &'static str, // e.g., "toml", "json", "yaml"
        op: &'static str,     // short name of the operation or capability
        hint: String,         // remediation, alternative, or doc pointer
    },

    /// A *command* (subcommand/mode) is recognized but not supported.
    #[error("Command not supported: {cmd}. {hint}")]
    Command { cmd: &'static str, hint: String },

    /// A *flag/option or combination* is recognized but not supported.
    #[error("Option not supported: {option}. {hint}")]
    Option {
        option: &'static str, // e.g., "--watch", "--atomic=off", "--schema"
        hint: String,
    },

    /// A known *feature/capability* is not supported (policy / scope / design).
    #[error("Feature not supported: {feature}. {hint}")]
    Feature {
        feature: &'static str, // e.g., "schema-validation", "json5"
        hint: String,
    },

    /// Platform/OS/FS constraint (e.g., hardlinks on certain filesystems).
    #[error("Platform not supported: {platform} for {what}. {hint}")]
    Platform {
        platform: &'static str, // e.g., "windows", "linux", "macOS"
        what: &'static str,     // the thing we tried (e.g., "atomic replace")
        hint: String,
    },

    /// Version constraint (tool/library/runtime too old/new for requested op).
    #[error("{component} version not supported: found {found}, requires {required}. {hint}")]
    Version {
        component: &'static str, // e.g., "toml_edit", "rustc", "config-cli"
        found: String,           // detected version
        required: &'static str,  // minimal or exact requirement
        hint: String,
    },

    /// Catch-all for a specific unsupported combination the user tried.
    #[error("This combination is not supported: {what}. {hint}")]
    Combination {
        what: String, // brief description of the combo (flags/formats/modes)
        hint: String,
    },
}

impl NotSupportedError {
    // ---- Convenience constructors ----

    pub fn format(format: &'static str, op: &'static str, hint: impl Into<String>) -> Self {
        Self::Format {
            format,
            op,
            hint: hint.into(),
        }
    }

    pub fn command(cmd: &'static str, hint: impl Into<String>) -> Self {
        Self::Command {
            cmd,
            hint: hint.into(),
        }
    }

    pub fn option(option: &'static str, hint: impl Into<String>) -> Self {
        Self::Option {
            option,
            hint: hint.into(),
        }
    }

    pub fn feature(feature: &'static str, hint: impl Into<String>) -> Self {
        Self::Feature {
            feature,
            hint: hint.into(),
        }
    }

    pub fn platform(platform: &'static str, what: &'static str, hint: impl Into<String>) -> Self {
        Self::Platform {
            platform,
            what,
            hint: hint.into(),
        }
    }

    pub fn version(
        component: &'static str,
        found: impl Into<String>,
        required: &'static str,
        hint: impl Into<String>,
    ) -> Self {
        Self::Version {
            component,
            found: found.into(),
            required,
            hint: hint.into(),
        }
    }

    pub fn combination(what: impl Into<String>, hint: impl Into<String>) -> Self {
        Self::Combination {
            what: what.into(),
            hint: hint.into(),
        }
    }

    /// Heuristic: returns `true` when the unsupported state could be resolved
    /// by upgrading/changing environment (e.g., Version) rather than changing
    /// the *request* itself. Helpful if you ever want to decide messaging.
    pub fn is_potentially_transient(&self) -> bool {
        matches!(self, Self::Version { .. } | Self::Platform { .. })
    }
}
