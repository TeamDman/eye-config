use crate::known_projects::KnownProjects;
use crate::persistence_key::PersistenceKey;
use chrono::Utc;
use eyre::Context;
use eyre::Result;
use eyre::eyre;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use serde_json::{self};
use tokio::fs;
use tracing::debug;
use tracing::warn;

#[async_trait::async_trait]
pub trait PersistableState:
    Sized
    + Default
    + std::fmt::Debug
    + Sync
    + for<'de> Deserialize<'de>
    + Serialize
    + Clone
    + Send
    + 'static
    + PartialEq
{
    async fn key() -> eyre::Result<PersistenceKey>;

    /// Asynchronously load the configuration with incremental upgrading.
    async fn load() -> Result<Self> {
        let key = Self::key().await?;
        let exists = key.exists().await?;
        let path = key.file_path()?;
        let instance = if exists {
            debug!("Loading config from {}", path.display());
            let content = fs::read_to_string(&path).await?;
            // Try to parse file into a serde_json::Value
            let user_json: Value = match serde_json::from_str(&content) {
                Ok(val) => val,
                Err(err) => {
                    warn!(
                        "Failed to load config as valid json, will make a backup and will revert to defaults. Error: {}",
                        err
                    );
                    // If we fail, backup the original file and use the default.
                    let now = Utc::now().format("%Y%m%dT%H%M%SZ");
                    let backup_path = path.with_extension(format!("{}.bak", now));
                    fs::copy(&path, &backup_path).await?;
                    // For upgrade purposes, we use the default JSON.
                    serde_json::to_value(&Self::default())?
                }
            };

            // Get the default config as JSON.
            let default_json = serde_json::to_value(&Self::default())?;
            // Merge the user config (if present) into the default config.
            let merged_json = merge_json(default_json, user_json);
            // Deserialize the merged JSON.
            serde_json::from_value(merged_json)
                .map_err(|e| eyre!("Failed to deserialize merged config: {}", e))?
        } else {
            debug!(
                "Config file {} does not exist, using default config",
                path.display()
            );
            Self::default()
        };

        if !Self::is_secret() {
            KnownProjects::track_project_accessed(key).await?;
        }

        Ok(instance)
    }

    /// Asynchronously save the configuration.
    async fn save(&self) -> Result<()> {
        let path = Self::key().await?.file_path()?;
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir).await?;
        }
        let content = serde_json::to_string_pretty(self)
            .wrap_err_with(|| eyre::eyre!("Failed to serialize config {} with value {self:?}", path.display()))?;
        debug!("Writing config to {:?}", path);
        fs::write(&path, content).await?;
        Ok(())
    }

    async fn modify_and_save<F>(&mut self, f: F) -> Result<()>
    where
        F: FnOnce(&mut Self) -> () + Send,
    {
        f(self);
        self.save().await?;
        Ok(())
    }

    /// If a config is secret, it will not be included in the index used by the eye_config cli.
    /// By default, configs are not secret.
    fn is_secret() -> bool {
        false
    }
}

/// A recursive merge function that takes the `default` value and overrides
/// with any keys found in `user` where the key exists in both objects.
/// If both values are objects, the merge is done recursively.
fn merge_json(default: Value, user: Value) -> Value {
    match (default, user) {
        // Both default and user are objects: merge key by key.
        (Value::Object(mut default_map), Value::Object(user_map)) => {
            for (key, user_value) in user_map {
                let entry = default_map.entry(key).or_insert(Value::Null);
                *entry = merge_json(entry.take(), user_value);
            }
            Value::Object(default_map)
        }
        // In all other cases, take the user value.
        (_, user_value) => user_value,
    }
}
