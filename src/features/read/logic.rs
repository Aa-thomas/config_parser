use std::path::PathBuf;

use crate::shared::core::{
    adapters::{detect_and_parse, get_config_value_at_path},
    errors::{ConfigResult, FileIoError},
    path::{create_value_path, ValidatedPath},
    types::ConfigValue,
};

pub fn read<'a>(
    config_document_path: &'a PathBuf,
    key_path: &'a String,
) -> ConfigResult<ConfigValue> {
    let config_document = std::fs::read_to_string(config_document_path)
        .map_err(|error| FileIoError::read_failed(config_document_path, &error))?;
    let parsed_document = detect_and_parse(&config_document)?;
    let validated_key_path = ValidatedPath::new(key_path)?;
    let value_path = create_value_path(&validated_key_path)?;
    let config_value = get_config_value_at_path(&value_path, &parsed_document)?;
    Ok(config_value)
}
