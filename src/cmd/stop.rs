use clap::Clap;
use crate::models::AgentOp;

#[derive(Debug,  Clap)]
pub struct StopCmd {
    #[clap(flatten)]
    pub agent: AgentOp
}
