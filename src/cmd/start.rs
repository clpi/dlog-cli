use clap::Clap;

use crate::models::AgentOp;

#[derive(Debug,  Clap)]
pub struct StartCmd {
    #[clap(flatten)]
    pub agent: AgentOp
}
