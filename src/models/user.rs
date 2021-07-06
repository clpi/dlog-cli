use serde::{Serialize, Deserialize};
use clap::Clap;
use chrono::{DateTime, Local};

#[derive(Deserialize, Serialize, Debug, Clap)]
pub struct UserOp {
    id: Option<u32>,
    name: Option<String>,
    email: Option<String>,

}
#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub name: String,
    pub created_at: DateTime<Local>,

}
impl Default for User {
    fn default() -> Self {
        Self {
            name: String::new(),
            created_at: Local::now(),
        }
    }
}

impl super::Model for User {

    type Op = UserOp;
}
impl super::ModelOp for UserOp {

}
