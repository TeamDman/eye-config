use super::clean_command::CleanCommand;
use super::list_command::ListCommand;
use super::prune_command::PruneCommand;
use super::show_command::ShowCommand;
use crate::cli::global_args::GlobalArgs;
use clap::Parser;

/// Top-level CLI commands for eye-config.
#[derive(Debug, Parser)]
pub enum Command {
    /// List known configurations
    List(ListCommand),
    /// Shows configuration details interactively or by key
    Show(ShowCommand),
    /// Remove configuration files
    Clean(CleanCommand),
    /// Clean up known configuration entries which are no longer valid
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
