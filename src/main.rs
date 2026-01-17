use clap::Parser;
use tracing::{error, info};

mod cli;
mod config;
mod metrics;
mod pipeline;

fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Parse CLI args
    let args = cli::Args::parse();

    info!("Starting eightyeightyeight...");

    match args.command {
        cli::Commands::Record { config } => {
            info!("Loading configuration from: {}", config);

            // Load configuration
            let config = match config::Config::load(&config) {
                Ok(c) => c,
                Err(e) => {
                    error!("Failed to load configuration: {}", e);
                    return Err(e);
                }
            };

            info!(
                "Configuration loaded successfully. Device: {}",
                config.device
            );

            // Run the pipeline
            pipeline::run_record_pipeline(&config)?;
        }
        cli::Commands::Play { config, input } => {
            info!("Loading configuration from: {}", config);
            let config = match config::Config::load(&config) {
                Ok(c) => c,
                Err(e) => {
                    error!("Failed to load configuration: {}", e);
                    return Err(e);
                }
            };
            info!("Playing back file: {}", input);
            pipeline::run_play_pipeline(&config, &input)?;
        }
        cli::Commands::Stream { config, dest, port } => {
            info!("Loading configuration from: {}", config);
            let config = match config::Config::load(&config) {
                Ok(c) => c,
                Err(e) => {
                    error!("Failed to load configuration: {}", e);
                    return Err(e);
                }
            };
            info!("Streaming to {}:{}", dest, port);
            pipeline::run_stream_pipeline(&config, &dest, port)?;
        }
        cli::Commands::Receive {
            config,
            listen,
            port,
        } => {
            info!("Loading configuration from: {}", config);
            let config = match config::Config::load(&config) {
                Ok(c) => c,
                Err(e) => {
                    error!("Failed to load configuration: {}", e);
                    return Err(e);
                }
            };
            info!("Receiving on {}:{}", listen, port);
            pipeline::run_receive_pipeline(&config, &listen, port)?;
        }
    }

    Ok(())
}
