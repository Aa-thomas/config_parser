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
) -> PathResult<ConfigValue<'a>> {
    match document {
        ConfigDocument::Json(json_document) => {
            let retreived_value = get_json_at_path(json_document, path)?;
            let converted_value = ConfigValue::Json(retreived_value);
            Ok(converted_value)
        }
        ConfigDocument::Toml(toml_document) => {
            let retreived_value = get_toml_at_path(toml_document.as_item(), path)?;
            let converted_value = ConfigValue::Toml(retreived_value);
            Ok(converted_value)
        }
    }
}
