use clap::Parser;
use tracing::{error, info};

mod cli;
mod config;
mod metrics;
mod pipeline;
mod secrets;

fn load_config(path: &str) -> anyhow::Result<config::Config> {
    info!("Loading configuration from: {}", path);
    match config::Config::load(path) {
        Ok(c) => Ok(c),
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            Err(e)
        }
    }
}

fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Parse CLI args
    let args = cli::Args::parse();

    info!("Starting eightyeightyeight...");

    match args.command {
        cli::Commands::Record { config } => {
            let config = load_config(&config)?;

            info!(
                "Configuration loaded successfully. Device: {}",
                config.device
            );

            // Run the pipeline
            pipeline::run_record_pipeline(&config)?;
        }
        cli::Commands::Play { config, input } => {
            let config = load_config(&config)?;
            info!("Playing back file: {}", input);
            pipeline::run_play_pipeline(&config, &input)?;
        }
        cli::Commands::Stream { config, dest, port } => {
            let config = load_config(&config)?;
            info!("Streaming to {}:{}", dest, port);
            pipeline::run_stream_pipeline(&config, &dest, port)?;
        }
        cli::Commands::Receive {
            config,
            listen,
            port,
        } => {
            let config = load_config(&config)?;
            info!("Receiving on {}:{}", listen, port);
            pipeline::run_receive_pipeline(&config, &listen, port)?;
        }
    }

    Ok(())
}
