use crate::config::Config;
use anyhow::{Context, Result};
use gstreamer as gst;
use gstreamer::prelude::*;
use inquire::{Confirm, CustomType, Select, Text};
use rand::RngCore;
use std::collections::HashSet;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct VideoMode {
    width: u32,
    height: u32,
    framerate: String, // Kept as string for config compatibility (e.g. "30/1")
    format: String,
}

impl std::fmt::Display for VideoMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}x{} @ {} fps ({})",
            self.width, self.height, self.framerate, self.format
        )
    }
}

pub fn run(output_path: String) -> Result<()> {
    gst::init().context("Failed to initialize GStreamer")?;

    println!("Welcome to the 8088 Configuration Wizard!");

    // 1. Device Selection & 2. Mode Selection
    let (device, width, height, framerate) = select_device_and_mode()?;

    // 3. Bitrate
    let bitrate = CustomType::<u32>::new("Bitrate (kbps):")
        .with_default(1000)
        .with_error_message("Please enter a valid number")
        .prompt()?;

    // 4. Key Generation
    let key = if Confirm::new("Generate a new secure key?")
        .with_default(true)
        .prompt()?
    {
        generate_key()
    } else {
        Text::new("Enter existing key (hex):").prompt()?
    };

    let key_val =
        if key.starts_with("literal:") || key.starts_with("env:") || key.starts_with("file:") {
            key
        } else {
            format!("literal:{}", key)
        };

    // 5. Output Path
    let output_file = Text::new("Output file path:")
        .with_default("output.ts.enc")
        .prompt()?;

    // 6. CV Enabled
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

fn select_device_and_mode() -> Result<(String, u32, u32, String)> {
    let monitor = gst::DeviceMonitor::new();
    monitor.add_filter(Some("Video/Source"), None);
    let _ = monitor.start();

    // Give it a moment to probe? Usually synchronous enough for local devices if started.
    // However, sometimes it needs a main loop context or time.
    // For a simple CLI wizard, we can check immediately.
    let devices = monitor.devices();
    monitor.stop();

    if devices.is_empty() {
        println!("No GStreamer video sources found.");
        return manual_entry();
    }

    struct DeviceOption {
        name: String,
        device_path: String, // e.g. /dev/video0
        gst_device: gst::Device,
    }

    impl std::fmt::Display for DeviceOption {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} ({})", self.name, self.device_path)
        }
    }

    let mut options = Vec::new();
    for device in devices {
        let name = device.display_name().to_string();
        let props = device.properties();

                let path = if let Some(props) = props {
                    if let Ok(p) = props.get::<&str>("device.path") {
                        p.to_string()
                    } else {
                        "unknown".to_string()
                    }
                } else {
                    "unknown".to_string()
                };
        if path != "unknown" {
            options.push(DeviceOption {
                name,
                device_path: path,
                gst_device: device,
            });
        }
    }

    if options.is_empty() {
        println!("No suitable V4L2 devices found.");
        return manual_entry();
    }

    // Add manual override
    let use_detected = Confirm::new("Select from detected devices?")
        .with_default(true)
        .prompt()?;

    if !use_detected {
        return manual_entry();
    }

    let selected_device = Select::new("Select Video Device:", options).prompt()?;

    // Now probe caps
    let caps = selected_device.gst_device.caps();
    let mut modes = HashSet::new();

    if let Some(caps) = caps {
        for i in 0..caps.size() {
            let structure = caps.structure(i).expect("Valid structure");
            // We want width, height, framerate
            // format is also interesting
            if let (Ok(w), Ok(h), Ok(fmt)) = (
                structure.get::<i32>("width"),
                structure.get::<i32>("height"),
                structure.get::<&str>("format"),
            ) {
                // Framerate is a fraction
                if let Ok(fps_fraction) = structure.get::<gst::Fraction>("framerate") {
                    let fps_str = format!("{}/{}", fps_fraction.numer(), fps_fraction.denom());
                    modes.insert(VideoMode {
                        width: w as u32,
                        height: h as u32,
                        framerate: fps_str,
                        format: fmt.to_string(),
                    });
                }
            }
        }
    }

    if modes.is_empty() {
        println!("Could not detect supported modes for this device.");
        return manual_mode_entry(selected_device.device_path);
    }

    let mut mode_list: Vec<VideoMode> = modes.into_iter().collect();
    // Sort: Width desc, Height desc, FPS desc
    mode_list.sort_by(|a, b| {
        b.width
            .cmp(&a.width)
            .then(b.height.cmp(&a.height))
            .then_with(|| {
                // Try to parse FPS numerator for sorting
                let a_num: i32 = a
                    .framerate
                    .split('/')
                    .next()
                    .unwrap_or("0")
                    .parse()
                    .unwrap_or(0);
                let b_num: i32 = b
                    .framerate
                    .split('/')
                    .next()
                    .unwrap_or("0")
                    .parse()
                    .unwrap_or(0);
                b_num.cmp(&a_num)
            })
    });

    let selected_mode = Select::new("Select Capture Mode:", mode_list).prompt()?;

    Ok((
        selected_device.device_path,
        selected_mode.width,
        selected_mode.height,
        selected_mode.framerate,
    ))
}

fn manual_entry() -> Result<(String, u32, u32, String)> {
    let device = Text::new("Device path:")
        .with_default("/dev/video0")
        .prompt()?;
    manual_mode_entry(device)
}

fn manual_mode_entry(device: String) -> Result<(String, u32, u32, String)> {
    let width = CustomType::<u32>::new("Width:")
        .with_default(640)
        .with_error_message("Please enter a valid number")
        .prompt()?;
    let height = CustomType::<u32>::new("Height:")
        .with_default(480)
        .with_error_message("Please enter a valid number")
        .prompt()?;
    let framerate = Text::new("Framerate (e.g. 30/1):")
        .with_default("30/1")
        .prompt()?;

    Ok((device, width, height, framerate))
}

fn generate_key() -> String {
    let mut key = [0u8; 32];
    rand::rng().fill_bytes(&mut key);
    hex::encode(key)
}
