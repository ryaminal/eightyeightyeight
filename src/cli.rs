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
    /// Stream encrypted video over the network
    Stream {
        /// Path to the configuration file
        #[arg(short, long, default_value = "config.toml")]
        config: String,
        /// Destination IP address
        #[arg(short, long, default_value = "127.0.0.1")]
        dest: String,
        /// Destination port
        #[arg(short, long, default_value = "8088")]
        port: u16,
    },
    /// Receive and decrypt video from the network
    Receive {
        /// Path to the configuration file (for decryption keys)
        #[arg(short, long, default_value = "config.toml")]
        config: String,
        /// Listen IP address
        #[arg(short, long, default_value = "0.0.0.0")]
        listen: String,
        /// Listen port
        #[arg(short, long, default_value = "8088")]
        port: u16,
    },
    /// Initialize a new configuration file via an interactive wizard
    Init {
        /// Output path for the generated configuration
        #[arg(short, long, default_value = "config.toml")]
        output: String,
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
    fn test_init_command() {
        let args = Args::parse_from(&["app", "init", "--output", "new_config.toml"]);
        match args.command {
            Commands::Init { output } => assert_eq!(output, "new_config.toml"),
            _ => panic!("Expected Init command"),
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

    #[test]
    fn test_stream_command() {
        let args = Args::parse_from(&[
            "app",
            "stream",
            "--dest",
            "192.168.1.10",
            "--port",
            "9000",
            "--config",
            "stream_config.toml",
        ]);
        match args.command {
            Commands::Stream { config, dest, port } => {
                assert_eq!(config, "stream_config.toml");
                assert_eq!(dest, "192.168.1.10");
                assert_eq!(port, 9000);
            }
            _ => panic!("Expected Stream command"),
        }
    }

    #[test]
    fn test_receive_command() {
        let args = Args::parse_from(&[
            "app",
            "receive",
            "--listen",
            "127.0.0.1",
            "--port",
            "9001",
            "--config",
            "recv_config.toml",
        ]);
        match args.command {
            Commands::Receive {
                config,
                listen,
                port,
            } => {
                assert_eq!(config, "recv_config.toml");
                assert_eq!(listen, "127.0.0.1");
                assert_eq!(port, 9001);
            }
            _ => panic!("Expected Receive command"),
        }
    }
}
