use std::path::PathBuf;

use crate::shared::core::{
    path::ValuePath,
    types::{ConfigDocument, ConfigValue},
};

#[derive(Debug)]
pub struct ReadCliArgs {
    pub document: PathBuf,
    pub key_path: String,
}

#[derive(Debug)]
pub struct ReadRequest<'a> {
    pub document: &'a ConfigDocument,
    pub path: ValuePath,
}

#[derive(Debug)]
pub struct ReadResult {
    pub value: ConfigValue,
}
