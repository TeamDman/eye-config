use crate::global_args::GlobalArgs;
use crate::our_config::known_projects::KnownProjectEntry;
use crate::our_config::known_projects::KnownProjects;
use crate::persistable_state::PersistableState;
use clap::Parser;
use std::collections::HashMap;
use std::iter::once;

#[derive(Debug, Parser)]
pub struct ListCommand {}

impl ListCommand {
    pub async fn handle(self, global_args: GlobalArgs) -> eyre::Result<()> {
        let _ = global_args;
        let known_projects = KnownProjects::load().await?;
        let display = serde_json::to_string_pretty(
            &known_projects
                .entries
                .into_iter()
                .chain(once(KnownProjectEntry {
                    key: KnownProjects::key().await?,
                    last_accessed: chrono::Local::now(),
                }))
                .map(|entry| Ok((entry.key.file_path()?.display().to_string(), entry)))
                .collect::<eyre::Result<HashMap<_, _>>>()?,
        )?;
        println!("{}", display);
        Ok(())
    }
}
