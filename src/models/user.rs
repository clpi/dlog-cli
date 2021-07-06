use serde::{Serialize, Deserialize};
use clap::Clap;
use chrono::{DateTime, Local};

#[derive(Deserialize, Serialize, Debug, Clap)]
pub struct UserOp {
    id: Option<u32>,
    name: Option<String>,
    email: Option<String>,

}
#[derive(Deserialize, Serialize, Debug, Clap)]
pub struct User {
    pub name: String,
    pub created_at: DateTime<Local>,

}

impl super::Model for User {}
