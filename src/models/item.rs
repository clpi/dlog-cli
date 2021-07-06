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

#[derive(Deserialize, Serialize, Debug)]
pub struct Item {
    pub name: String,
    pub created_at: DateTime<Local>
}
impl Default for Item {
    fn default() -> Self {
        Self {
            name: String::new(),
            created_at: Local::now(),
        }
    }
}
impl Model for Item {
    type Op = ItemOp;

}
impl super::ModelOp for ItemOp {

}
