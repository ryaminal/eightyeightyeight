use clap::Parser;
use tracing::{info, error};

mod config;
mod cli;
mod pipeline;

fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Parse CLI args
    let args = cli::Args::parse();

    info!("Starting eightyeightyeight...");
    info!("Loading configuration from: {}", args.config);

    // Load configuration
    let config = match config::Config::load(&args.config) {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            return Err(e);
        }
    };

    info!("Configuration loaded successfully. Device: {}", config.device);

    Ok(())
}