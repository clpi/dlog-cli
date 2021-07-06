use clap::Clap;

#[derive(Debug,  Clap)]
pub struct DeleteOp {
    #[clap(long)]
    cascade: bool,
    #[clap(subcommand)]
    target: Option<super::Target>
}
