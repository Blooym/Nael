#[forbid(unsafe_code)]
mod commands;
mod dalamud_version_manager;
mod directories;
mod logger;
mod repository;

use self::commands::{Install, List, Remove, RunnableCommand, Update, Use};
use crate::logger::error;
use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use commands::Current;

#[derive(Debug, Parser)]
enum NaelCommand {
    Install(Install),
    Remove(Remove),
    Update(Update),
    List(List),
    Use(Use),
    Current(Current),
}

#[async_trait]
impl RunnableCommand for NaelCommand {
    async fn run(&self) -> Result<()> {
        match self {
            NaelCommand::Install(cmd) => cmd.run().await,
            NaelCommand::Remove(cmd) => cmd.run().await,
            NaelCommand::Update(cmd) => cmd.run().await,
            NaelCommand::List(cmd) => cmd.run().await,
            NaelCommand::Use(cmd) => cmd.run().await,
            NaelCommand::Current(cmd) => cmd.run().await,
        }
    }
}

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Opts {
    #[clap(subcommand)]
    cmd: NaelCommand,
}

#[tokio::main]
async fn main() {
    let opts = Opts::parse();

    if let Err(err) = opts.cmd.run().await {
        error!("Something went wrong during command execution: {}", err);
        std::process::exit(1);
    }
}
