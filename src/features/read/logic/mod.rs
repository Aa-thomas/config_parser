pub mod json;
pub mod toml;

use crate::{
    features::read::{
        domain::ReadRequest,
        logic::{json::get_json_at_path, toml::get_toml_at_path},
    },
    shared::core::{
        path::PathResult,
        types::{ConfigDocument, ConfigValue},
    },
};

pub fn read_config_value_at_path<'a>(request: ReadRequest<'a>) -> PathResult<ConfigValue> {
    match request.document {
        ConfigDocument::Json(json_document) => {
            let json_value = get_json_at_path(json_document, &request.path)?;
            Ok(json_value)
        }
        ConfigDocument::Toml(toml_document) => {
            let toml_item = get_toml_at_path(toml_document, &request.path)?;
            Ok(toml_item)
        }
    }
}
