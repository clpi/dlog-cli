
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
#[derive(Deserialize, Serialize, Debug)]
pub struct Link {
    pub name: String,
    pub created_at: DateTime<Local>
}
impl Default for Link {
    fn default() -> Self {
        Self {
            name: String::new(),
            created_at: Local::now(),
        }
    }
}

impl Model for Link {
    type Op = LinkOp;

}

impl super::ModelOp for LinkOp {

}
