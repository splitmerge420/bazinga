use clap::{Parser, Subcommand};

mod engines;
mod commands;

#[derive(Parser)]
#[command(name = "bazinga")]
#[command(about = "Constitutional universal compute layer — BAZINGA v0.2\nKintsuji-first: all commands pass the golden repair gate before execution.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
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

    // KINTSUJI GATE — mandatory first gate, all commands
    // Dave's golden repair protocol: https://github.com/splitmerge420/Kintsuji-code-fixer-
    let command_str = format!("{:?}", cli.command);
    match engines::kintsuji_apl::KintsujiAPL::gate(&command_str).await {
        Ok(report) => {
            println!("   Kintsuji stamp: {} @ {}\n", report.provenance_hash, report.timestamp);
        }
        Err(e) => {
            eprintln!("KINTSUJI BLOCKED: {}", e);
            eprintln!("Command halted. Fix the violation before running.");
            std::process::exit(1);
        }
    }

    engines::constitutional_engine::boot().await;

    match cli.command {
        Commands::SyncNotion => commands::sync_notion::run().await,
        Commands::SyncDrive => commands::sync_drive::run().await,
        Commands::SyncGithub => commands::sync_github::run().await,
        Commands::Index => commands::index::run().await,
        Commands::Bench { colossus } => commands::bench::run(colossus).await,
    }
}
