use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to the configuration file
    #[arg(short, long, default_value = "config.toml")]
    pub config: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_args() {
        let args = Args::parse_from(&["app"]);
        assert_eq!(args.config, "config.toml");
    }

    #[test]
    fn test_custom_config_arg() {
        let args = Args::parse_from(&["app", "--config", "my_config.toml"]);
        assert_eq!(args.config, "my_config.toml");
    }
}
