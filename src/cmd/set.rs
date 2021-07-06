use clap::Clap;

#[derive(Debug,  Clap)]
pub struct SetOp {
    #[clap(subcommand)]
    target: Option<super::Target>
}
