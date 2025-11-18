pub mod json;
pub mod toml;

use crate::shared::core::{
    adapters::{json::get_json_at_path, toml::get_toml_at_path},
    detect::detect_format,
    errors::{ConfigError, ConfigResult},
    parse::{parse_json::parse_json, parse_toml::parse_toml},
    path::{PathResult, ValuePath},
    types::{ConfigDocument, ConfigFormat, ConfigValue},
};

pub fn detect_and_parse(data: &str) -> Result<ConfigDocument, ConfigError> {
    let format = detect_format(data)?;
    match format {
        ConfigFormat::Json => Ok(ConfigDocument::Json(parse_json(data)?)),
        ConfigFormat::Toml => Ok(ConfigDocument::Toml(parse_toml(data)?)),
    }
}

pub fn get_config_value_at_path<'a>(
    path: &'a ValuePath,
    document: &'a ConfigDocument,
) -> PathResult<ConfigValue> {
    match document {
        ConfigDocument::Json(json_document) => {
            let json_value = get_json_at_path(json_document, path)?;
            Ok(json_value)
        }
        ConfigDocument::Toml(toml_document) => {
            let toml_item = get_toml_at_path(toml_document, path)?;
            Ok(toml_item)
        }
    }
}
