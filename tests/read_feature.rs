use config_parser::{
    features::read::{domain::ReadCliArgs, handler::handle_read_command},
    shared::core::types::ConfigValue,
};
use std::path::PathBuf;

#[test]
fn read_from_json_fixture() -> anyhow::Result<()> {
    let args = ReadCliArgs {
        document: PathBuf::from(".fixtures/jfix.json"),
        key_path: "nested.arr_in_obj[1].y[2]".to_string(),
    };

    let result = handle_read_command(args)?;

    let expected_value = serde_json::json!(30);
    assert_eq!(result.value, ConfigValue::Json(expected_value));

    Ok(())
}

#[test]
fn read_from_toml_fixture() -> anyhow::Result<()> {
    let args = ReadCliArgs {
        document: PathBuf::from(".fixtures/tfix.toml"),
        key_path: "top.str".to_string(),
    };

    let result = handle_read_command(args)?;

    let value = match result.value {
        ConfigValue::Toml(item) => item
            .as_value()
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow::anyhow!("Expected a TOML string value"))?,
        _ => return Err(anyhow::anyhow!("Expected a TOML value")),
    };

    assert_eq!(value, "hi");

    Ok(())
}
