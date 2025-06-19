use super::clean_command::CleanCommand;
use super::list_command::ListCommand;
use super::prune_command::PruneCommand;
use super::show_command::ShowCommand;
use crate::cli::global_args::GlobalArgs;
use clap::Parser;

#[derive(Debug, Parser)]
pub enum Command {
    List(ListCommand),
    Show(ShowCommand),
    Clean(CleanCommand),
    Prune(PruneCommand),
}
impl Command {
    pub async fn handle(self, global_args: GlobalArgs) -> eyre::Result<()> {
        match self {
            Command::List(cmd) => cmd.handle(global_args).await,
            Command::Show(cmd) => cmd.handle(global_args).await,
            Command::Clean(cmd) => cmd.handle(global_args).await,
            Command::Prune(cmd) => cmd.handle(global_args).await,
        }
    }
}
