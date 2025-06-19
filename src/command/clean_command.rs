use crate::global_args::GlobalArgs;
use crate::our_config::known_projects::KnownProjects;
use crate::persistable_state::PersistableState;
use crate::persistence_key::PersistenceKey;
use clap::Parser;
use cloud_terrastodon_user_input::Choice;
use cloud_terrastodon_user_input::FzfArgs;
use cloud_terrastodon_user_input::are_you_sure;
use cloud_terrastodon_user_input::pick_many;
use eyre::bail;

#[derive(Debug, Parser)]
pub struct CleanCommand {
    /// Optionally provide a value to parse as JSON for display
    #[clap(long, value_parser = parse_persistence_key)]
    pub key: Option<PersistenceKey>,
}

fn parse_persistence_key(s: &str) -> Result<PersistenceKey, String> {
    serde_json::from_str::<PersistenceKey>(s)
        .map_err(|e| format!("Failed to parse PersistenceKey: {e}"))
}
impl CleanCommand {
    pub async fn handle(self, global_args: GlobalArgs) -> eyre::Result<()> {
        let mut known_projects = KnownProjects::load().await?;
        let keys = match (global_args.interactive, self.key) {
            (true, Some(key)) => vec![key],
            (true, None) => {
                // If interactive and no key provided, prompt user to pick a key
                pick_many(FzfArgs {
                    choices: known_projects
                        .entries
                        .iter()
                        .map(|entry| {
                            eyre::Result::<Choice<&PersistenceKey>>::Ok(Choice {
                                key: format!(
                                    "{} ({})",
                                    entry.key.file_path()?.display(),
                                    entry.last_accessed
                                ),
                                value: &entry.key,
                            })
                        })
                        .collect::<eyre::Result<Vec<_>>>()?,
                    header: Some("Select projects to remove".to_string()),
                    ..Default::default()
                })?
                .into_iter()
                .map(|x| x.value.clone())
                .collect()
            }
            (false, Some(key)) => vec![key],
            (false, None) => {
                // If not interactive and no key provided, return an error
                bail!("The `clean` command requires either a key or interactivity");
            }
        };
        for key in keys {
            let path_to_remove = key.file_path()?;
            if global_args.interactive
                && !are_you_sure(format!(
                    "Are you sure you want to remove the file at {}?",
                    path_to_remove.display()
                ))? {
                    bail!("Operation cancelled by user");
                }
            tokio::fs::remove_file(path_to_remove).await?;
            known_projects.entries.retain(|entry| entry.key != key);
            known_projects.save().await?;
        }
        Ok(())
    }
}
