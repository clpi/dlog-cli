use clap::Clap;

#[derive(Debug, Clap)]
pub struct NewCmd {
    #[clap(subcommand)]
    target: Option<super::Target>
}
