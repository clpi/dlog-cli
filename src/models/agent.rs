use clap::Clap;
use super::Model;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Local};

#[derive(Deserialize, Serialize, Debug, Clap)]
pub struct AgentOp {
    pub name: String,
}
#[derive(Deserialize, Serialize, Debug, Clap)]
pub struct Agent {
    pub name: String,
    pub created_at: DateTime<Local>
}

impl Model for AgentOp {

}
