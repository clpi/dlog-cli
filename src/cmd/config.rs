use std::path::PathBuf;
use clap::{Clap, ValueHint};

#[derive(Debug, Clap)]
pub struct ConfigCmd {
    value: Option<String>,
    #[clap(subcommand)]
    op: ConfigCmdOp,
}

#[derive(Debug, Clap)]
pub enum ConfigCmdOp {
    #[clap(name = "get")]
    Get {
        #[clap(short, long)]
        fields: Vec<String>,
        field: String
    },
    #[clap(name = "set")]
    Set {
        #[clap(short, long)]
        fields: Vec<String>,
        #[clap(short, long)]
        value: Option<String>,
    },
    #[clap(name = "reset")]
    Reset {
        field: String,
    },
    #[clap(name = "path", alias = "file")]
    Path(ConfigCmdPathOp)

}
#[derive(Debug, Clap)]
pub enum ConfigCmdPathOp {
    #[clap(name = "set")]
    Set {
        #[clap(parse(from_os_str), value_hint = ValueHint::FilePath)]
        path: PathBuf
    },
    #[clap(name = "get")]
    Get,
    #[clap(name = "reset")]
    Reset,
}
