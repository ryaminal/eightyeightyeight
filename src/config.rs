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
    pub iv: String,
    pub output_path: PathBuf,
}

impl Config {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_deserialize_config() {
        let toml_str = r#"
            device = "/dev/video0"
            width = 1920
            height = 1080
            framerate = "60/1"
            bitrate = 5000
            key = "00000000000000000000000000000000"
            iv = "11111111111111111111111111111111"
            output_path = "output.ts.enc"
        "#;

        let config: Config = toml::from_str(toml_str).expect("Failed to parse TOML");

        assert_eq!(config.device, "/dev/video0");
        assert_eq!(config.width, 1920);
        assert_eq!(config.height, 1080);
        assert_eq!(config.framerate, "60/1");
        assert_eq!(config.bitrate, 5000);
        assert_eq!(config.key, "00000000000000000000000000000000");
        assert_eq!(config.iv, "11111111111111111111111111111111");
        assert_eq!(config.output_path, PathBuf::from("output.ts.enc"));
    }

    #[test]
    fn test_load_from_file() {
        let toml_str = r#"
            device = "/dev/video_test"
            width = 1280
            height = 720
            framerate = "30/1"
            bitrate = 2500
            key = "12345678901234561234567890123456"
            iv = "12345678901234561234567890123456"
            output_path = "test_output.ts.enc"
        "#;

        let file_path = "test_config.toml";
        let mut file = std::fs::File::create(file_path).unwrap();
        file.write_all(toml_str.as_bytes()).unwrap();

        let config = Config::load(file_path).expect("Failed to load config file");

        assert_eq!(config.device, "/dev/video_test");
        assert_eq!(config.width, 1280);

        // Cleanup
        let _ = std::fs::remove_file(file_path);
    }
}
