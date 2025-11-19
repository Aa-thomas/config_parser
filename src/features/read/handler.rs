use crate::{
    features::read::{
        domain::{ReadCliArgs, ReadRequest, ReadResult},
        logic::read_config_value_at_path,
    },
    shared::core::{
        adapters::detect_and_parse,
        errors::FileIoError,
        path::{create_value_path, ValidatedPath},
    },
};

pub fn handle_read_command(cli_args: ReadCliArgs) -> anyhow::Result<ReadResult> {
    let source_document = std::fs::read_to_string(&cli_args.document)
        .map_err(|error| FileIoError::read_failed(&cli_args.document, &error))?;
    let config_document = detect_and_parse(&source_document)?;

    let validated_path = ValidatedPath::new(&cli_args.key_path)?;
    let value_path = create_value_path(&validated_path)?;

    let config_value = read_config_value_at_path(ReadRequest {
        document: &config_document,
        path: value_path,
    })?;

    Ok(ReadResult {
        value: config_value,
    })
}
