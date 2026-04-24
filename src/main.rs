mod cli;
mod commands;
mod config;
mod discovery;
mod docs;
mod output;
mod provider;
mod spec;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();
    commands::run(cli)
}
