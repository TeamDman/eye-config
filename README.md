# 👁 eye_config

A Rust libaray and CLI tool for managing and tracking configuration files for projects.

From the examples:

```rust
use cloud_terrastodon_user_input::prompt_line;
use eye_config::global_args::GlobalArgs;
use eye_config::init_tracing::init_tracing;
use eye_config::persistable_state::PersistableState;
use eye_config::persistence_key::PersistenceKey;
use eye_config::project::PROJECT;
use serde::Deserialize;
use serde::Serialize;
use tracing::info;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct PreferredModelConfig {
    pub preferred_model: Option<String>,
}

#[async_trait::async_trait]
impl PersistableState for PreferredModelConfig {
    async fn key() -> eyre::Result<PersistenceKey> {
        Ok(PersistenceKey::new(PROJECT, "example-preferred_model"))
    }
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Load the preferred model configuration
    let config = PreferredModelConfig::load().await?;

    // Display the value from the persisted state
    info!("Current Preferred Model: {:?}", config.preferred_model);

    let new_preferred_model =
        prompt_line("Enter your preferred model (or leave empty to keep current): ").await?;
    if !new_preferred_model.is_empty() {
        // Update the preferred model if a new value is provided
        let mut config = config;
        config.preferred_model = Some(new_preferred_model);
        config.save().await?;
        info!("Updated Preferred Model: {:?}", config.preferred_model);
    } else {
        info!("No changes made to the preferred model.");
    }

    Ok(())
}
```