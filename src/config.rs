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

        Self::validate_key(&config.key)?;

        Ok(config)
    }

    fn validate_key(key: &str) -> anyhow::Result<()> {
        if key.len() != 64 {
            return Err(anyhow::anyhow!(
                "Invalid key length: {}. Expected 64 characters (32-byte hex).",
                key.len()
            ));
        }
        if !key.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(anyhow::anyhow!("Invalid key: contains non-hex characters"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_from_file_literal_resolution() {
        let valid_key = "00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff";
        let toml_str = format!(r#"
            device = "/dev/video_test"
            width = 1280
            height = 720
            framerate = "30/1"
            bitrate = 2500
            key = "literal:{}"
            output_path = "test_output.ts.enc"
        "#, valid_key);

        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", toml_str).unwrap();

        let config = Config::load(file.path().to_str().unwrap()).expect("Failed to load config");
        assert_eq!(config.key, valid_key);
        assert_eq!(config.cv_enabled, false);
    }

    #[test]
    fn test_invalid_key_length() {
        let toml_str = r#"
            device = "/dev/video_test"
            width = 1280
            height = 720
            framerate = "30/1"
            bitrate = 2500
            key = "short_key"
            output_path = "test_output.ts.enc"
        "#;

        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", toml_str).unwrap();

        let err = Config::load(file.path().to_str().unwrap()).unwrap_err();
        assert!(err.to_string().contains("Invalid key length"));
    }

    #[test]
    fn test_load_from_file_env_resolution() {
        let valid_key = "00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff";
        unsafe {
            std::env::set_var("MY_CONFIG_KEY", valid_key);
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
        assert_eq!(config.key, valid_key);

        unsafe {
            std::env::remove_var("MY_CONFIG_KEY");
        }
    }
}
