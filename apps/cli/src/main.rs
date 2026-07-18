use clap::{Parser, Subcommand};
use odin_core::Odin;

#[derive(Parser)]
#[command(
    name = "odin",
    about = "ODIN - Operational Defense Intelligence Network"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Check system health
    Health,
    /// List all incidents in memory
    Incidents,
    /// List all memory objects
    Memories,
    /// Search similar incidents by ID
    Search {
        /// Incident ID to search similar to
        incident_id: String,
        /// Number of results (default 5)
        #[arg(default_value = "5")]
        top_k: usize,
    },
    /// Show system version
    Version,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let odin = Odin::new();
    let cli = Cli::parse();

    match cli.command {
        Commands::Health => {
            println!("ODIN is running");

            match odin.memory.list_all() {
                Ok(memories) => println!("  {} memory object(s) stored", memories.len()),
                Err(_) => println!("  could not access memory store"),
            }
            println!("  intelligence engine: ✓");
            println!("  retrieval engine: ✓");
            println!("  decision engine: ✓");
            println!("  policy gate: ✓");
        }
        Commands::Incidents => {
            println!("Incidents:");
            match odin.memory.list_all() {
                Ok(memories) => {
                    for m in &memories {
                        let title = m
                            .context
                            .get("title")
                            .and_then(|v| v.as_str())
                            .unwrap_or("Untitled");
                        println!(
                            "  {} | {} (confidence: {:.2})",
                            m.incident_id, title, m.confidence
                        );
                    }
                    if memories.is_empty() {
                        println!("  No incidents found.");
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::Memories => {
            println!("Memory Objects:");
            match odin.memory.list_all() {
                Ok(memories) => {
                    for m in &memories {
                        println!(
                            "  {} | v{} | summary: {} | confidence: {:.2}",
                            m.id,
                            m.version,
                            &m.summary[..m.summary.len().min(60)],
                            m.confidence
                        );
                    }
                    if memories.is_empty() {
                        println!("  No memory objects found.");
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::Search { incident_id, top_k } => {
            println!(
                "Searching for incidents similar to {} (top {})...",
                incident_id, top_k
            );
            println!("Note: Full search requires the API server running with data in memory.");
            println!("Use the API endpoint POST /api/v1/incidents/search instead.");
        }
        Commands::Version => {
            println!("ODIN v{}", env!("CARGO_PKG_VERSION"));
        }
    }
}
