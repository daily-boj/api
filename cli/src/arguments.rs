use clap::{crate_version, Clap};

#[derive(Debug, Clap)]
#[clap(name = "daily-boj api cli", version = crate_version!())]
/// Utilize management of daily-boj api
pub struct CliOptions {
    #[clap(subcommand)]
    pub sub_command: SubCommand,
}

#[derive(Debug, Clap)]
pub enum SubCommand {
    /// Build the api response(s) statically
    Build(Build),
    /// Update database schema
    UpdateSchema(UpdateSchema),
}

#[derive(Debug, Clap)]
pub struct Build {}

#[derive(Debug, Clap)]
pub struct UpdateSchema {}
