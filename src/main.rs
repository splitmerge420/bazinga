mod engines;
mod commands;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "bazinga",
    version = "0.2.0",
    about = "BAZINGA — Constitutional universal compute layer",
    long_about = "One binary. One identity. Constitutional middleware for the Noosphere.\nPrinciples: 20 ratified. Canonical efficiency: 11,289x verified."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Sync artifacts from Notion to local index
    SyncNotion,
    /// Sync artifacts from Google Drive to local index
    SyncDrive,
    /// Sync codebase state from GitHub
    SyncGithub,
    /// Build full local semantic index across all sources
    Index,
    /// Run canonical efficiency benchmark (11,289x model)
    Bench {
        /// Run full Colossus simulation mode
        #[arg(long)]
        colossus: bool,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Constitutional boot on every command — non-negotiable
    engines::constitutional_engine::boot().await;

    match cli.command {
        Commands::SyncNotion   => commands::sync_notion::run().await,
        Commands::SyncDrive    => commands::sync_drive::run().await,
        Commands::SyncGithub   => commands::sync_github::run().await,
        Commands::Index        => commands::index::run().await,
        Commands::Bench { colossus } => commands::bench::run(colossus).await,
    }
}
