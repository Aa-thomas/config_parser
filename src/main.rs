use clap::Parser;
use config_parser::cli::{Cli, Command};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Read {
            config_document,
            key_path,
        } => {
            println!(
                "READ -> file={:?} format={:?} key_path={}",
                cli.config_document, cli.config_format, key_path
            );
        }
        Command::Set { key_path, value } => {
            println!(
                "SET  -> file={:?} format={:?} key_path={} value={}",
                cli.config_document, cli.config_format, key_path, value
            );
        }
        Command::Delete { key_path } => {
            println!(
                "DEL  -> file={:?} format={:?} key_path={}",
                cli.config_document, cli.config_format, key_path
            );
        }
        Command::List { key_path } => {
            println!(
                "LIST -> file={:?} format={:?} key_path={:?}",
                cli.config_document, cli.config_format, key_path
            );
        }
    }
}
