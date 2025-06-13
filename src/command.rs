use clap::Parser;

use crate::{clean_command::CleanCommand, global_args::GlobalArgs, list_command::ListCommand, show_command::ShowCommand};

#[derive(Debug, Parser)]
pub enum Command {
    List(ListCommand),
    Show(ShowCommand),
    Clean(CleanCommand),
}
impl Command {
    pub async fn handle(self, global_args: GlobalArgs) -> eyre::Result<()> {
        match self {
            Command::List(cmd) => cmd.handle(global_args).await,
            Command::Show(cmd) => cmd.handle(global_args).await,
            Command::Clean(cmd) => cmd.handle(global_args).await,
        }
    }
}