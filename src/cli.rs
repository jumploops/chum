use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "chum")]
#[command(about = "Filesystem-first documentation workflow CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Init(InitArgs),
    Check(CheckArgs),
    Archive(ArchiveArgs),
    Swim(SwimArgs),
}

#[derive(Debug, Args)]
pub struct InitArgs {
    #[arg(long)]
    pub dry_run: bool,
    #[arg(long)]
    pub write: bool,
    #[arg(long)]
    pub agent_doc: Option<PathBuf>,
    #[arg(long)]
    pub no_agent_doc: bool,
    #[arg(long)]
    pub json: bool,
}

#[derive(Debug, Args, Clone)]
pub struct CheckArgs {
    #[arg(long)]
    pub json: bool,
    #[arg(long)]
    pub allow_external_verify: bool,
    #[arg(long)]
    pub include: Vec<String>,
    #[arg(long)]
    pub include_archive: bool,
    #[arg(long)]
    pub allow_stale: bool,
    #[arg(default_value = ".")]
    pub path: PathBuf,
}

#[derive(Debug, Args)]
pub struct ArchiveArgs {
    pub change_id: String,
    #[arg(long)]
    pub dry_run: bool,
    #[arg(long)]
    pub include: Vec<String>,
    #[arg(long)]
    pub source_ref: Option<String>,
    #[arg(long)]
    pub pr: Option<String>,
    #[arg(long)]
    pub json: bool,
    #[arg(default_value = ".")]
    pub path: PathBuf,
}

#[derive(Debug, Args)]
pub struct SwimArgs {
    #[arg(default_value = ".")]
    pub path: PathBuf,
    #[arg(long)]
    pub dry_run: bool,
    #[arg(long)]
    pub write: bool,
    #[arg(long)]
    pub repair: bool,
    #[arg(long)]
    pub json: bool,
    #[arg(long)]
    pub max_passes: Option<usize>,
    #[arg(long, default_value = "openai")]
    pub provider: String,
    #[arg(long)]
    pub stubs: bool,
    #[arg(long)]
    pub allow_external_verify: bool,
}
