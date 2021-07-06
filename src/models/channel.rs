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
#[derive(Deserialize, Serialize, Debug, Clap)]
pub struct Channel {
    pub name: String,
    pub created_at: DateTime<Local>
}

impl Model for Channel {

}

