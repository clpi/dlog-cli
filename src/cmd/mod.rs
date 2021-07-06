pub mod new;
pub mod search;
pub mod config;
pub mod delete;
pub mod get;
pub mod set;
pub mod list;

pub use self::{
    new::NewCmd,
    delete::DeleteOp,
    set::SetOp,
    get::GetOp,
    list::ListOp,
    search::SearchOp,
    config::ConfigCmd,
};

use clap::{Clap, AppSettings};
use crate::models::{TopicOp, BookOp, UserOp, ChannelOp, LinkOp, ItemOp, FieldOp, AgentOp, EntryOp};

#[derive(Debug, Clap)]
#[clap(author, about, version, setting = AppSettings::InferSubcommands)]
pub struct Dlog {
    value: Option<String>,
    #[clap(short, long)]
    agent: Option<String>,
    #[clap(short, long)]
    book: Option<String>,
    #[clap(short, long)]
    item: Option<String>,
    #[clap(short, long)]
    field: Option<String>,
    #[clap(short, long)]
    log: Option<String>,
    #[clap(short, long)]
    user: Option<String>,
    #[clap(short, long)]
    topics: Vec<String>,
    #[clap(short, long)]
    records: Vec<String>,
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
    #[clap(subcommand)]
    cmd: Option<Cmd>,
}


#[derive(Debug, Clap)]
pub enum Cmd {
    #[clap(name = "new", aliases = &["init", "ini", "i"])]
    New(NewCmd),
    #[clap(name = "get")]
    Get(GetOp),
    #[clap(name = "list", aliases = &["get-all", "ls"])]
    List(ListOp),
    #[clap(name = "delete", aliases = &["remove", "rm", "r"])]
    Delete(DeleteOp),
    #[clap(name = "update")]
    Update(SetOp),
    #[clap(name = "search", alias = "find")]
    Search(SearchOp),
    #[clap(name = "config")]
    Config(ConfigCmd),
    #[clap(flatten)]
    Target(Op),
}

#[derive(Debug,  Clap)]
pub enum Op {
    #[clap(name = "new", aliases = &["init", "ini", "i"])]
    New,
    #[clap(name = "get")]
    Get,
    #[clap(name = "list", aliases = &["get-all", "ls"])]
    List,
    #[clap(name = "delete", aliases = &["remove", "rm", "r"])]
    Delete,
    #[clap(name = "update")]
    Update,
}
#[derive(Debug, Clap)]
pub enum Target {
    #[clap(name = "book")]
    Book(BookOp),
    #[clap(name = "link")]
    Link(LinkOp),
    #[clap(name = "channel")]
    Channel(ChannelOp),
    #[clap(name = "item")]
    Item(ItemOp),
    #[clap(name = "field")]
    Field(FieldOp),
    #[clap(name = "entry")]
    Entry(EntryOp),
    #[clap(name = "agent")]
    Agent(AgentOp),
    #[clap(name = "user")]
    User(UserOp),
    #[clap(name = "topic")]
    TopicOp(TopicOp),
}
impl Dlog {
    pub fn get() -> Self {
        let cmd = Self::parse();
        cmd
    }
}

