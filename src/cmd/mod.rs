pub mod new;
pub mod search;
pub mod start;
pub mod stop;
pub mod config;
pub mod delete;
pub mod get;
pub mod set;
pub mod list;

pub use self::{
    new::NewOp,
    delete::DeleteOp,
    set::SetOp,
    get::GetOp,
    list::ListOp,
    search::SearchOp,
    config::ConfigCmd,
    start::StartCmd,
    stop::StopCmd,
};

use clap::{Clap, AppSettings};
use crate::models::{
    ModelOp,
    TopicOp,
    BookOp, UserOp, ChannelOp, LinkOp, ItemOp, FieldOp, AgentOp, EntryOp
};

#[derive(Debug, Clap)]
#[clap(author, about, version)]
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
    #[clap(name = "start", aliases = &["run", "r", "execute", "exec"])]
    Start(StartCmd),
    #[clap(name = "stop", aliases = &["end", "term", "terminate"])]
    Stop(StopCmd),
    #[clap(flatten)]
    Op(Op),
    #[clap(name = "search", aliases = &["find", "s", "f"])]
    Search(SearchOp),
    #[clap(name = "config", aliases = &["conf", "c"])]
    Config(ConfigCmd),
    #[clap(flatten)]
    Target(Target),
}
#[derive(Debug, Clap)]
pub enum Target {
    #[clap(name = "book", aliases = &["record", "rec"])]
    Book(BookOp),
    #[clap(name = "link", alias = "li")]
    Link(LinkOp),
    #[clap(name = "channel", alias = "chan")]
    Channel(ChannelOp),
    #[clap(name = "item")]
    Item(ItemOp),
    #[clap(name = "field")]
    Field(FieldOp),
    #[clap(name = "entry", aliases = &["post", "value", "en"])]
    Entry(EntryOp),
    #[clap(name = "agent", alias = "bot")]
    Agent(AgentOp),
    #[clap(name = "user", alias = "u")]
    User(UserOp),
    #[clap(name = "topic", aliases = &["t", "top"])]
    TopicOp(TopicOp),
}
#[derive(Debug,  Clap)]
pub enum Op {
    #[clap(name = "new", aliases = &["init", "create", "add", "ini", "i"])]
    New(NewOp),
    #[clap(name = "get", alias = "g")]
    Get(GetOp),
    #[clap(name = "list", aliases = &["get-all", "ls", "l"])]
    List(ListOp),
    #[clap(name = "delete", aliases = &["d", "del", "remove", "rm"])]
    Delete(DeleteOp),
    #[clap(name = "set", aliases = &["u", "set", "update", "edit", "e"])]
    Set(SetOp),
}

impl Dlog {
    pub fn get() -> Self {
        let cmd = Self::parse();
        let parsed = dlog_parse::parse::DlBaseParser::parse_input();
        cmd
    }
}

