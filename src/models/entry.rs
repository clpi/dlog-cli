use clap::Clap;
use super::Model;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Local};

#[derive(Clap, Deserialize, Serialize, Debug)]
pub struct EntryOp {
    #[clap(long, short)]
    pub log: Option<String>,
    #[clap(long, short = 'I')]
    pub inbox: bool,
    pub value: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Entry {
    pub created_at: DateTime<Local>
}

impl Model for Entry {

}
