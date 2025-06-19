use crate::command::Command;
use crate::global_args::GlobalArgs;
use clap::Parser;

#[derive(Parser, Debug)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Resource))]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(flatten)]
    pub global: GlobalArgs,

    #[command(subcommand)]
    pub command: Command,
}
