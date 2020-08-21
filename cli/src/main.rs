mod arguments;
mod commands;

use arguments::*;
use clap::Clap;
use commands::*;

fn main() {
    let options = CliOptions::parse();

    match options.sub_command {
        SubCommand::Build(args) => build(args),
    }
}
