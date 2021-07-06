use clap::Clap;
use termcolor::*;
use actix::prelude::*;
use crate::cmd::{DeleteOp, GetOp, ListOp, NewOp, Op, SetOp, StartCmd, Target};

use super::Model;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Local};

#[derive(Deserialize, Serialize, Debug, Clap)]
pub struct AgentOp {
    pub name: String,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct Agent {
    pub name: String,
    pub created_at: DateTime<Local>
}

impl Default for Agent {
    fn default() -> Self {
        Self { name: String::new(), created_at: Local::now() }
    }
}

impl From<Op> for Agent {
    fn from(op: Op) -> Self {
        match op {
            Op::New(NewOp { target: Some(Target::Agent(agent_op)) }) => Self::new(agent_op),
            Op::Get(GetOp { target: Some(Target::Agent(agent_op)) }) => Self::get(agent_op),
            Op::Set(SetOp { target: Some(Target::Agent(agent_op)) }) => Self::set(agent_op),
            Op::Delete(DeleteOp { cascade, target: Some(Target::Agent(agent_op)) }) => Self::delete(agent_op),
            Op::List(ListOp { target: Some(Target::Agent(agent_op)), .. }) => Self::list(agent_op),
            _ => Self::default(),
        }

    }
}
impl Agent {
    pub fn start(start_cmd: StartCmd) {

    }
}
impl Model for Agent {
    type Op = AgentOp;

    fn new(op: Self::Op) -> Self {
        Self {
            name: op.name,
            ..Default::default()
        }
    }

}
impl super::ModelOp for AgentOp {

}

impl Actor for Agent {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("[{}] Started agent {}: ", Local::now(), self.name);
    }
    fn stopped(&mut self, ctx: &mut Self::Context) {
        println!("[{}] Stopped agent {}: ", Local::now(), self.name);
    }
}
