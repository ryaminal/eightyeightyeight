use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Record video from the configured device
    Record {
        /// Path to the configuration file
        #[arg(short, long, default_value = "config.toml")]
        config: String,
    },
    /// Play back an encrypted video file
    Play {
        /// Path to the configuration file (for decryption keys)
        #[arg(short, long, default_value = "config.toml")]
        config: String,
        /// Path to the encrypted input file
        #[arg(short, long)]
        input: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_command() {
        let args = Args::parse_from(&["app", "record", "--config", "my_config.toml"]);
        match args.command {
            Commands::Record { config } => assert_eq!(config, "my_config.toml"),
            _ => panic!("Expected Record command"),
        }
    }

    #[test]
    fn test_play_command() {
        let args = Args::parse_from(&[
            "app",
            "play",
            "--input",
            "video.enc",
            "--config",
            "my_config.toml",
        ]);
        match args.command {
            Commands::Play { config, input } => {
                assert_eq!(config, "my_config.toml");
                assert_eq!(input, "video.enc");
            }
            _ => panic!("Expected Play command"),
        }
    }
}
