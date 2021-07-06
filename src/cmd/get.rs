use clap::Clap;

#[derive(Debug,  Clap)]
pub struct GetOp {
    #[clap(subcommand)]
    pub target: Option<super::Target>
}
