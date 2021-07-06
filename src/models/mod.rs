pub mod entry;
pub mod link;
pub mod topic;
pub mod agent;
pub mod book;
pub mod field;
pub mod item;
pub mod user;
pub mod channel;

pub use self::{
    link::{LinkOp, Link},
    agent::{AgentOp, Agent},
    book::{BookOp, Book},
    channel::{ChannelOp, Channel},
    entry::{EntryOp, Entry},
    field::{FieldOp, Field},
    item::{ItemOp, Item},
    user::{UserOp, User},
    topic::{TopicOp, Topic}
};

pub trait Model {}

pub trait ModelIn: clap::Clap {

}
