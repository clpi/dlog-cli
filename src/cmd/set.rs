use clap::Clap;
use crate::models::ModelOp;

#[derive(Debug, Clap)]
pub struct SetOp {
    #[clap(subcommand)]
    pub target: Option<super::Target>
}
