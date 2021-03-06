use clap::Clap;

#[derive(Debug,  Clap)]
pub struct ListOp {
    #[clap(short, long, default_value = "15")]
    num: u32,
    #[clap(short, long)]
    all: bool,
    #[clap(short, long)]
    pretty: bool,
    #[clap(subcommand)]
    pub target: Option<super::Target>,
    #[clap(short, long, default_value = "Vec::new()")]
    topics: Vec<String>,
    #[clap(short, long, default_value = "Vec::new()")]
    records: Vec<String>,
}
