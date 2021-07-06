use clap::Clap;
use super::Model;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Local};

#[derive(Deserialize, Serialize, Debug, Clap)]
pub struct BookOp {
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Book {
    pub name: String,
    pub created_at: DateTime<Local>
}
impl Default for Book {
    fn default() -> Self {
        Self {
            name: String::new(),
            created_at: Local::now(),
        }
    }
}

impl Model for Book {
    type Op = BookOp;

}
impl super::ModelOp for BookOp {

}
