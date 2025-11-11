use std::{io, path::Path};
use thiserror::Error;

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

pub fn io_reason_and_hint(kind: io::ErrorKind, path: &str, op: &str) -> (String, String) {
    use io::ErrorKind::*;
    match kind {
        NotFound => (
            "No such file or directory".into(),
            format!("Check the path or create it first: {}", path),
        ),
        PermissionDenied => (
            "Permission denied".into(),
            "Check file permissions or run with appropriate rights.".into(),
        ),
        IsADirectory => (
            "Path is a directory, not a file".into(),
            "Use a regular file path for this operation.".into(),
        ),
        WouldBlock => (
            "Resource temporarily unavailable".into(),
            "Try again or ensure no other process is locking the file.".into(),
        ),
        AlreadyExists => (
            "File already exists".into(),
            "Remove existing temp file or choose a different output path.".into(),
        ),
        InvalidInput => (
            "Invalid path or parameters".into(),
            "Verify the path string and encoding.".into(),
        ),
        _ => (
            kind.to_string(),
            "Re-run with --debug for more details.".into(),
        ),
    }
}

impl FileIoError {
    pub fn read_failed(path: impl AsRef<Path>, err: &io::Error) -> Self {
        let path_str = path.as_ref().display().to_string();
        let (reason, hint) = io_reason_and_hint(err.kind(), &path_str, "read");
        Self::ReadFailed {
            path: path_str,
            reason,
            hint,
        }
    }

    pub fn write_failed(path: impl AsRef<Path>, err: &io::Error) -> Self {
        let path_str = path.as_ref().display().to_string();
        let (reason, hint) = io_reason_and_hint(err.kind(), &path_str, "write");
        Self::WriteFailed {
            path: path_str,
            reason,
            hint,
        }
    }

    pub fn temp_create_failed(path: impl AsRef<Path>, err: &io::Error) -> Self {
        let path_str = path.as_ref().display().to_string();
        let (reason, hint) = io_reason_and_hint(err.kind(), &path_str, "temp file create");
        Self::TempCreateFailed {
            path: path_str,
            reason,
            hint,
        }
    }

    pub fn atomic_replace_failed(
        temp_path: impl AsRef<Path>,
        final_path: impl AsRef<Path>,
        err: &io::Error,
    ) -> Self {
        let temp_str = temp_path.as_ref().display().to_string();
        let final_str = final_path.as_ref().display().to_string();
        let (reason, hint) = io_reason_and_hint(err.kind(), &final_str, "atomic replace");
        Self::AtomicReplaceFailed {
            temp_path: temp_str,
            final_path: final_str,
            reason,
            hint,
        }
    }
}
