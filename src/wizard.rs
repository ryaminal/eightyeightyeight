use crate::config::Config;
use anyhow::Result;
use inquire::{Confirm, CustomType, Select, Text};
use rand::RngCore;
use std::path::PathBuf;

pub fn run(output_path: String) -> Result<()> {
    println!("Welcome to the 8088 Configuration Wizard!");

    // 1. Device Selection
    let device = select_device()?;

    // 2. Resolution
    let width = CustomType::<u32>::new("Width:")
        .with_default(640)
        .with_error_message("Please enter a valid number")
        .prompt()?;
    let height = CustomType::<u32>::new("Height:")
        .with_default(480)
        .with_error_message("Please enter a valid number")
        .prompt()?;

    // 3. Framerate
    let framerate = Text::new("Framerate (e.g. 30/1):")
        .with_default("30/1")
        .prompt()?;

    // 4. Bitrate
    let bitrate = CustomType::<u32>::new("Bitrate (kbps):")
        .with_default(1000)
        .with_error_message("Please enter a valid number")
        .prompt()?;

    // 5. Key Generation
    let key = if Confirm::new("Generate a new secure key?")
        .with_default(true)
        .prompt()?
    {
        generate_key()
    } else {
        Text::new("Enter existing key (hex):").prompt()?
    };

    // Ensure we wrap it in "literal:" if it's just raw hex, or let user type what they want?
    // The previous implementation requires `literal:` prefix for raw keys.
    let key_val =
        if key.starts_with("literal:") || key.starts_with("env:") || key.starts_with("file:") {
            key
        } else {
            // Assume literal if just hex
            format!("literal:{}", key)
        };

    // 6. Output Path
    let output_file = Text::new("Output file path:")
        .with_default("output.ts.enc")
        .prompt()?;

    // 7. CV Enabled
    let cv_enabled = Confirm::new("Enable Computer Vision (Face Detection)?")
        .with_default(false)
        .prompt()?;

    let config = Config {
        device,
        width,
        height,
        framerate,
        bitrate,
        key: key_val,
        output_path: PathBuf::from(output_file),
        cv_enabled,
        min_disk_space_mb: None, // defaults
        max_files: None,
        max_file_size_mb: None,
    };

    let toml_string = toml::to_string_pretty(&config)?;
    std::fs::write(&output_path, toml_string)?;

    println!("Configuration saved to {}", output_path);
    Ok(())
}

fn select_device() -> Result<String> {
    // List /dev/video*
    let mut devices = Vec::new();
    if let Ok(entries) = std::fs::read_dir("/dev") {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if name_str.starts_with("video") {
                devices.push(format!("/dev/{}", name_str));
            }
        }
    }
    devices.sort();

    if devices.is_empty() {
        println!("No /dev/video* devices found. You may need to enter it manually.");
        return Ok(Text::new("Device path:")
            .with_default("/dev/video0")
            .prompt()?);
    }

    // Add manual option
    devices.push("Manual Entry".to_string());

    let selection = Select::new("Select Video Device:", devices).prompt()?;

    if selection == "Manual Entry" {
        Ok(Text::new("Device path:").prompt()?)
    } else {
        Ok(selection)
    }
}

fn generate_key() -> String {
    let mut key = [0u8; 32];
    rand::rng().fill_bytes(&mut key);
    hex::encode(key)
}
