
use clap::Clap;
use super::Model;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Local};

#[derive(Deserialize, Serialize, Debug, Clap)]
pub struct LinkOp {
    pub name: String,
    #[clap(long, short)]
    pub item: Vec<String>,
    #[clap(long, short)]
    pub topic: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clap)]
pub struct Link {
    pub name: String,
    pub created_at: DateTime<Local>
}

impl Model for Link {

}

