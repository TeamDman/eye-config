use crate::cli::config::known_projects::KnownProjects;
use crate::cli::global_args::GlobalArgs;
use crate::persistable_state::PersistableState;
use crate::persistence_key::PersistenceKey;
use clap::Parser;
use cloud_terrastodon_user_input::Choice;
use cloud_terrastodon_user_input::FzfArgs;
use cloud_terrastodon_user_input::pick;
use eyre::bail;
use serde_json::json;

/// Command to show details for a specific configuration or item.
#[derive(Debug, Parser)]
pub struct ShowCommand {
    /// Optionally provide a value to parse as JSON for display
    #[clap(long, value_parser = parse_persistence_key)]
    pub key: Option<PersistenceKey>,
}

fn parse_persistence_key(s: &str) -> Result<PersistenceKey, String> {
    serde_json::from_str::<PersistenceKey>(s)
        .map_err(|e| format!("Failed to parse PersistenceKey: {e}"))
}

impl ShowCommand {
    pub async fn handle(self, global_args: GlobalArgs) -> eyre::Result<()> {
        let known_projects = KnownProjects::load().await?;
        if known_projects.entries.is_empty() {
            bail!("No projects found.");
        }
        let key = match (global_args.interactive, self.key) {
            (true, Some(key)) => key,
            (true, None) => {
                // If interactive and no key provided, prompt user to pick a key
                pick(FzfArgs {
                    choices: known_projects
                        .entries
                        .iter()
                        .map(|entry| {
                            eyre::Ok(Choice {
                                key: format!(
                                    "{} ({})",
                                    entry.key.file_path()?.display(),
                                    entry.last_accessed
                                ),
                                value: &entry.key,
                            })
                        })
                        .collect::<eyre::Result<Vec<_>>>()?,
                    header: Some("Select a project to show".to_string()),
                    ..Default::default()
                })?
                .clone()
            }
            (false, Some(key)) => key,
            (false, None) => {
                // If not interactive and no key provided, return an error
                bail!("The `show` command requires either a key or interactivity");
            }
        };
        let last_accessed = known_projects
            .entries
            .iter()
            .find(|entry| entry.key == key)
            .ok_or_else(|| eyre::eyre!("No project found for the provided key"))?;
        let display = serde_json::to_string_pretty(&json!({
            "key": key,
            "last_accessed": last_accessed,
            "file_path": key.file_path()?.display().to_string(),
        }))?;
        println!("{display}");
        Ok(())
    }
}
