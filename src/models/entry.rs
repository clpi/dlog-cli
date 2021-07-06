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
    pub name: Option<String>,
    pub created_at: DateTime<Local>
}
impl Default for Entry {
    fn default() -> Self {
        Self {
            name: None,
            created_at: Local::now(),
        }
    }
}

impl Model for Entry {
    type Op = EntryOp;

}
impl super::ModelOp for EntryOp {

}
