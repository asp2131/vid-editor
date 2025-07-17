use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "video_ai")]
#[command(about = "VideoAI CLI – intelligent video processing", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Print version information
    Version,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Version => {
            println!("video_ai {}", env!("CARGO_PKG_VERSION"));
        }
    }
    Ok(())
}
