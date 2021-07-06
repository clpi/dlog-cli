use clap::Clap;
use super::Model;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Local};

#[derive(Deserialize, Serialize, Debug, Clap)]
pub struct ChannelOp {
    pub name: String,
    #[clap(long, short)]
    pub item: Vec<String>,
    #[clap(long, short)]
    pub topic: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct Channel {
    pub name: String,
    pub created_at: DateTime<Local>
}
impl Default for Channel {
    fn default() -> Self {
        Self {
            name: String::new(),
            created_at: Local::now(),
        }
    }
}

impl Model for Channel {
    type Op = ChannelOp;

}
impl super::ModelOp for ChannelOp {

}
