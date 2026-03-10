use clap::{Parser, Subcommand};

mod engines;
mod commands;

#[derive(Parser)]
#[command(name = "bazinga")]
#[command(about = "Constitutional universal compute layer — BAZINGA v0.2")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Sync artifacts from Notion
    SyncNotion,
    /// Sync artifacts from Google Drive
    SyncDrive,
    /// Sync artifacts from GitHub
    SyncGithub,
    /// Index all sources into Sheldonbrain
    Index,
    /// Run the canonical efficiency benchmark
    Bench {
        #[arg(long)]
        colossus: bool,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    engines::constitutional_engine::boot().await;

    match cli.command {
        Commands::SyncNotion => commands::sync_notion::run().await,
        Commands::SyncDrive => commands::sync_drive::run().await,
        Commands::SyncGithub => commands::sync_github::run().await,
        Commands::Index => commands::index::run().await,
        Commands::Bench { colossus } => commands::bench::run(colossus).await,
    }
}
