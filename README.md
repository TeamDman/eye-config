# 👁 eye_config

<img src="logo.png" alt="" width="200"/>

[![Crates.io Version](https://img.shields.io/crates/v/eye_config)](https://crates.io/crates/eye-config)

A Rust libaray and CLI tool for managing and tracking configuration files for
projects.

There is a CLI that interacts with the written configs. Config defintions can
mark themselves as secret to be excused from the CLI tracking.

## Sample library usage

From the examples:

[preferred_model.rs](./examples/preferred_model.rs)

```rust
use cloud_terrastodon_user_input::prompt_line;
use eye_config::cli::global_args::GlobalArgs;
use eye_config::cli::init_tracing::init_tracing;
use eye_config::persistable_state::PersistableState;
use eye_config::persistence_key::PersistenceKey;
use serde::Deserialize;
use serde::Serialize;
use tracing::info;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct PreferredModelConfig {
    pub preferred_model: Option<String>,
}

#[eye_config::async_trait::async_trait]
impl PersistableState for PreferredModelConfig {
    async fn key() -> eyre::Result<PersistenceKey> {
        Ok(PersistenceKey::new(
            "eye_config_examples",
            "example-preferred_model.json",
        ))
    }
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    init_tracing(&GlobalArgs::default(), std::io::stderr)?;

    info!("Run the program multiple times to see the persistent state in action.");

    // Load the preferred model configuration
    let config = PreferredModelConfig::load().await?;
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

## Sample CLI output

```
❯ eye_config help
A configuration persistence library and CLI tool.

Usage: eye_config.exe [OPTIONS] <COMMAND>

Commands:
  list   List known configurations
  show   Shows configuration details interactively or by key
  clean  Remove configuration files
  prune  Clean up known configuration entries which are no longer valid
  help   Print this message or the help of the given subcommand(s)

Options:
      --debug         Enable debug logging
      --interactive   If false, the program will error when interaction is requested
      --auto-approve  If true, any confirmation prompt will be automatically approved
  -h, --help          Print help
  -V, --version       Print version
```
