use std::path::PathBuf;

use directories_next::ProjectDirs;
use eyre::bail;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct PersistenceKey {
    pub project_name: PathBuf,
    pub file_slug: PathBuf,
}
impl PersistenceKey {
    pub fn new(project_name: impl Into<PathBuf>, file_slug: impl Into<PathBuf>) -> Self {
        Self {
            project_name: project_name.into(),
            file_slug: file_slug.into(),
        }
    }

    pub fn file_path(&self) -> eyre::Result<PathBuf> {
        let dirs = ProjectDirs::from_path(self.project_name.clone());
        let Some(dirs) = dirs else {
            bail!(
                "Failed to acquire disk locations for project {} and config file {}",
                self.project_name.display(),
                self.file_slug.display()
            );
        };

        let config_path = dirs.config_dir().join(&self.file_slug);
        Ok(config_path)
    }
    
    pub async fn exists(&self) -> eyre::Result<bool> {
        let path = self.file_path()?;
        Ok(tokio::fs::try_exists(&path).await?)
    }
}
