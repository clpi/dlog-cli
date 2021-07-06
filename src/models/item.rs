use clap::Clap;
use super::Model;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Local};

#[derive(Deserialize, Serialize, Debug, Clap)]
pub struct ItemOp {
    pub name: String,
    #[clap(long, short)]
    pub log: Vec<String>,
    #[clap(long, short)]
    pub topic: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clap)]
pub struct Item {
    pub name: String,
    pub created_at: DateTime<Local>
}
impl Model for Item {

}
