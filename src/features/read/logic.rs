use std::path::PathBuf;

use crate::shared::core::{detect::detect_format, types::ConfigValue};

pub fn read(file: PathBuf, key_path: String) -> ConfigValue {
    let detected = detect_format(file.to_str()?);
    a
}
