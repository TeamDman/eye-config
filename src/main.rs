use clap::CommandFactory;
use clap::FromArgMatches;
use eye_config::args::Args;
use eye_config::init_tracing::init_tracing;
use tracing::debug;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let cmd = Args::command();
    let args = Args::from_arg_matches(&cmd.get_matches())?;

    init_tracing(&args.global, std::io::stderr)?;
    debug!("Ahoy, world!");
    args.command.handle(args.global).await?;
    debug!("Command executed successfully.");
    Ok(())
}
