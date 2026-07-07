use clap::{Parser, Subcommand};
use odin_core::Odin;

#[derive(Parser)]
#[command(name = "odin", about = "ODIN - Operational Defense Intelligence Network")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Check system health
    Health,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let _odin = Odin::new();
    let cli = Cli::parse();

    match cli.command {
        Commands::Health => {
            println!("ODIN is running");
        }
    }
}
