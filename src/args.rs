use clap::Parser;

use crate::{command::Command, global_args::GlobalArgs};

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
