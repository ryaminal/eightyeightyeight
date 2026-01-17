use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Config {
    pub device: String,
    pub width: u32,
    pub height: u32,
    pub framerate: String,
    pub bitrate: u32,
    pub key: String,
    pub output_path: PathBuf,
    #[serde(default)]
    pub cv_enabled: bool,
}

impl Config {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let mut config: Config = toml::from_str(&content)?;

        // Resolve the key immediately
        let resolver = crate::secrets::get_resolver(&config.key);
        config.key = resolver.resolve()?;

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_from_file_literal_resolution() {
        let toml_str = r#"
            device = "/dev/video_test"
            width = 1280
            height = 720
            framerate = "30/1"
            bitrate = 2500
            key = "literal:resolved_secret_key"
            output_path = "test_output.ts.enc"
        "#;

        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", toml_str).unwrap();

        let config = Config::load(file.path().to_str().unwrap()).expect("Failed to load config");
        assert_eq!(config.key, "resolved_secret_key");
        assert_eq!(config.cv_enabled, false);
    }

    #[test]
    fn test_load_from_file_env_resolution() {
        unsafe {
            std::env::set_var("MY_CONFIG_KEY", "env_secret_key");
        }

        let toml_str = r#"
            device = "/dev/video_test"
            width = 1280
            height = 720
            framerate = "30/1"
            bitrate = 2500
            key = "env:MY_CONFIG_KEY"
            output_path = "test_output.ts.enc"
        "#;

        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", toml_str).unwrap();

        let config = Config::load(file.path().to_str().unwrap()).expect("Failed to load config");
        assert_eq!(config.key, "env_secret_key");

        unsafe {
            std::env::remove_var("MY_CONFIG_KEY");
        }
    }
}
