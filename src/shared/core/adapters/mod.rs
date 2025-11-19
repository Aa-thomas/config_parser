use crate::shared::core::{
    detect::detect_format,
    errors::ConfigError,
    parse::{parse_json::parse_json, parse_toml::parse_toml},
    types::{ConfigDocument, ConfigFormat},
};

pub fn detect_and_parse(data: &str) -> Result<ConfigDocument, ConfigError> {
    let format = detect_format(data)?;
    match format {
        ConfigFormat::Json => Ok(ConfigDocument::Json(parse_json(data)?)),
        ConfigFormat::Toml => Ok(ConfigDocument::Toml(parse_toml(data)?)),
    }
}
