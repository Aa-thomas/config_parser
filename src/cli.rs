use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "Config Parser",
    about = "CLI tool to read and edit JSON and TOML config files",
    version
)]
pub struct Cli {
    #[arg(long, value_name = "PATH", help = "Path to config file")]
    pub file: PathBuf,

    #[arg(
        long,
        value_enum,
        value_name = "FORMAT",
        help = "Format of config file: JSON or TOML"
    )]
    pub format: Option<Format>,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum Format {
    Json,
    Toml,
}

#[derive(Subcommand, Debug)]
pub enum Command {
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
