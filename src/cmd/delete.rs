use clap::Clap;

#[derive(Debug,  Clap)]
pub struct DeleteOp {
    #[clap(long)]
    pub cascade: bool,
    #[clap(subcommand)]
    pub target: Option<super::Target>
}
