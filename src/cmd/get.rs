use clap::Clap;

#[derive(Debug,  Clap)]
pub struct GetOp {
    #[clap(subcommand)]
    target: Option<super::Target>
}
