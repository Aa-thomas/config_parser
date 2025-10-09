// Example commands
// config --file settings.toml read network.timeout
// config --file settings.toml read servers[0].host
// config --file settings.json  --format json read network.timeout
// config --file settings.toml set network.timeout=1500
// config --file settings.toml set servers[0].host=localhost
// config --file settings.toml delete servers[0].host
// config --file settings.toml list servers[0].host

// Globals:
//   - file: path to config file
//   - format: json or toml

// Subcommands:
//   - read: read a value from the config file
//   - set: set a value in the config file
//   - delete: delete a value from the config file
//   - list: list all values in the config file

use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(
    name = "config",
    about = "CLI tool to read and edit JSON and TOML config files",
    version
)]
struct Cli {
    #[arg(
        long,
        value_name = "PATH",
        help = "Path to config file",
        required = true
    )]
    file: PathBuf,

    #[arg(
        long,
        value_enum,
        value_name = "FORMAT",
        help = "Format of config file: JSON or TOML"
    )]
    format: Option<Format>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum Format {
    Json,
    Toml,
}

#[derive(Subcommand, Debug)]
enum Command {
    #[command(about = "Print the value at KEY_PATH", alias = "get")]
    Read {
        #[arg(
            value_name = "KEY_PATH",
            help = "Dot or index path like network.timeout or servers[0].host"
        )]
        key_path: String,
    },

    #[command(about = "Set KEY_PATH to VALUE (types enforced unless --coerce)")]
    Set {
        #[arg(value_name = "KEY_PATH", help = "Path to key in config file")]
        key_path: String,

        #[arg(value_name = "VALUE", help = "Value to set at the given key path")]
        value: String,
    },

    #[command(about = "Remove the value at KEY_PATH")]
    Delete {
        #[arg(value_name = "KEY_PATH", help = "Path to key in config file")]
        key_path: String,
    },

    #[command(about = "List keys (optionally under KEY_PATH)", alias = "ls")]
    List {
        #[arg(
            value_name = "KEY_PATH",
            help = "Optional: restrict listing to subtree",
            required = false
        )]
        key_path: Option<String>,
    },
}
fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Read { key_path } => {
            println!(
                "READ -> file={:?} format={:?} key_path={}",
                cli.file, cli.format, key_path
            );
        }
        Command::Set { key_path, value } => {
            println!(
                "SET  -> file={:?} format={:?} key_path={} value={}",
                cli.file, cli.format, key_path, value
            );
        }
        Command::Delete { key_path } => {
            println!(
                "DEL  -> file={:?} format={:?} key_path={}",
                cli.file, cli.format, key_path
            );
        }
        Command::List { key_path } => {
            println!(
                "LIST -> file={:?} format={:?} key_path={:?}",
                cli.file, cli.format, key_path
            );
        }
    }
}
