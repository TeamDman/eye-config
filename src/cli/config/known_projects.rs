use super::project::PROJECT;
use crate::persistable_state::PersistableState;
use crate::persistence_key::PersistenceKey;
use chrono::DateTime;
use chrono::Local;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct KnownProjects {
    pub entries: Vec<KnownProjectEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct KnownProjectEntry {
    pub key: PersistenceKey,
    pub last_accessed: DateTime<Local>,
}

#[async_trait::async_trait]
impl PersistableState for KnownProjects {
    async fn key() -> eyre::Result<PersistenceKey> {
        Ok(PersistenceKey::new(PROJECT, "known-projects.json"))
    }
    fn is_secret() -> bool {
        true
    }
}

impl KnownProjects {
    #[async_recursion::async_recursion]
    pub async fn track_project_accessed(key: PersistenceKey) -> eyre::Result<()> {
        let now = Local::now();
        let mut known_projects = KnownProjects::load().await?;
        let entry = known_projects
            .entries
            .iter_mut()
            .find(|entry| entry.key == key);
        if let Some(existing_entry) = entry {
            existing_entry.last_accessed = now;
        } else {
            known_projects.entries.push(KnownProjectEntry {
                key,
                last_accessed: now,
            });
        }
        known_projects.save().await?;
        Ok(())
    }
}
