use clap::Parser;
use config_parser::{
    cli::{Cli, Command},
    features::read::{domain::ReadCliArgs, handler::handle_read_command},
};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Read { key_path } => {
            let result = handle_read_command(ReadCliArgs {
                document: cli.config_document,
                key_path,
            })?;

            println!("{:?}", result.value);
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

    Ok(())
}
