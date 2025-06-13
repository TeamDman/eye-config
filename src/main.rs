use clap::Parser;
use eye_config::args::Args;
use eye_config::init_tracing::init_tracing;
use tracing::debug;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let args = Args::try_parse()?;
    init_tracing(&args.global, std::io::stderr)?;
    debug!("Ahoy, world!");
    args.command.handle(args.global).await?;
    debug!("Command executed successfully.");
    Ok(())
}
