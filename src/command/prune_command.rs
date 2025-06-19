use crate::global_args::GlobalArgs;
use crate::our_config::known_projects::KnownProjects;
use crate::persistable_state::PersistableState;
use clap::Parser;
use tracing::warn;

#[derive(Debug, Parser)]
pub struct PruneCommand {}

impl PruneCommand {
    pub async fn handle(self, global_args: GlobalArgs) -> eyre::Result<()> {
        let _ = global_args;
        let mut known_projects = KnownProjects::load().await?;
        let mut remaining = Vec::new();
        for entry in known_projects.entries {
            if !entry.key.exists().await? {
                warn!(
                    "Removing entry for non-existent project: {}",
                    entry.key.file_path()?.display()
                );
            } else {
                remaining.push(entry);
            }
        }
        known_projects.entries = remaining;
        known_projects.save().await?;

        Ok(())
    }
}
