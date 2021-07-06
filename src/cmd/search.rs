
use clap::Clap;

#[derive(Debug,  Clap)]
pub struct SearchOp {
    #[clap(short, long, default_value = "15")]
    pub num: u32,
    #[clap(short, long)]
    pub all: bool,
    #[clap(short, long)]
    pub pretty: bool,
    #[clap(short, long, default_value = "Vec::new()")]
    pub topics: Vec<String>,
    #[clap(short, long, default_value = "Vec::new()")]
    pub records: Vec<String>,
    #[clap(subcommand)]
    pub target: Option<super::Target>
}
