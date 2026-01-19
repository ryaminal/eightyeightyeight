use clap::Parser;
use tracing::{error, info};

mod cli;
mod config;
mod metrics;
mod pipeline;
mod secrets;
mod wizard;

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

fn handle_record_command(config_path: &str) -> anyhow::Result<()> {
    let config = load_config(config_path)?;

    if let (Some(min_space_mb), Some(output_dir)) =
        (config.min_disk_space_mb, config.output_path.parent())
    {
        let free_space_mb = fs2::available_space(output_dir)? / 1_000_000;
        if free_space_mb < min_space_mb {
            anyhow::bail!(
                "Not enough disk space. Required: {} MB, Available: {} MB",
                min_space_mb,
                free_space_mb
            );
        }
    }

    info!(
        "Configuration loaded successfully. Device: {}",
        config.device
    );

    // Run the pipeline
    pipeline::run_record_pipeline(&config)
}

fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Parse CLI args
    let args = cli::Args::parse();

    info!("Starting eightyeightyeight...");

    match args.command {
        cli::Commands::Record { config } => handle_record_command(&config)?,
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
        cli::Commands::Init { output } => {
            wizard::run(output)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    // This is a mock of the pipeline runner for testing purposes.
    mod pipeline {
        use crate::config;
        pub fn run_record_pipeline(_config: &config::Config) -> anyhow::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn test_disk_space_check_fails() {
        let temp_dir = tempfile::tempdir().unwrap();
        let output_path = temp_dir.path().join("test.enc");

        let free_space = fs2::available_space(temp_dir.path()).unwrap();
        let free_space_mb = free_space / 1_000_000;

        let toml_str = format!(
            r#"
            device = "/dev/video_test"
            width = 1280
            height = 720
            framerate = "30/1"
            bitrate = 2500
            key = "literal:00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff"
            output_path = "{}"
            min_disk_space_mb = {}
        "#,
            output_path.display(),
            free_space_mb + 100 // Require more space than is available
        );

        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", toml_str).unwrap();

        // We need to shadow the pipeline module for this test.
        // The main `handle_record_command` will call our mock `run_record_pipeline`.
        let err = handle_record_command(file.path().to_str().unwrap()).unwrap_err();

        assert!(err.to_string().contains("Not enough disk space"));
    }
}
