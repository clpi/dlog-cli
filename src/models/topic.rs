use serde::{Serialize, Deserialize};
use clap::Clap;
use chrono::{DateTime, Local};
use super::{Channel, Model};

#[derive(Deserialize, Serialize, Debug, Clap)]
pub struct TopicOp {
    pub name: String,
    pub value: Option<String>,
    pub logs: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clap)]
pub struct Topic {
    pub name: String,
    pub created_at: DateTime<Local>

}
impl Model for Topic {

}
