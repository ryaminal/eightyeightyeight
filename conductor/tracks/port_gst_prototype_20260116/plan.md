# Implementation Plan - Port `gst.sh` Prototype

## Phase 1: Project Scaffolding & Configuration [checkpoint: 7891b6c]

- [x] Task: Initialize Rust project and dependencies de0422a
    - [x] Initialize `cargo new eightyeightyeight`
    - [x] Add dependencies: `gstreamer`, `clap`, `serde`, `serde_derive`, `toml`, `tracing`, `tracing-subscriber`, `anyhow`, `tokio` (full features), `ctrlc`.
    - [x] Create `rust-toolchain.toml` if needed.
- [x] Task: Implement Configuration Module de0422a
    - [x] Create `src/config.rs`.
    - [x] Define `Config` struct deriving `Deserialize` to match `gst.sh` parameters (device, resolution, bitrate, key, iv, output_path).
    - [x] Write tests for loading configuration from a sample TOML file.
    - [x] Implement `load_config` function using `config` or standard file I/O.
- [x] Task: Setup CLI and Logging de0422a
    - [x] Update `src/main.rs` to use `clap` for parsing arguments (e.g., `--config <FILE>`).
    - [x] Initialize `tracing_subscriber` for logging to stdout.
    - [x] Verify CLI accepts arguments and logs startup messages.
- [x] Task: Conductor - User Manual Verification 'Project Scaffolding & Configuration' (Protocol in workflow.md) de0422a

## Phase 2: Core GStreamer Pipeline Implementation [checkpoint: e777a2a]

- [x] Task: Construct the Record Pipeline 2be722f
    - [x] Create `src/pipeline.rs`.
    - [x] Implement a builder function that constructs the GStreamer pipeline string dynamically from the `Config` struct (similar to how `gst.sh` does string concatenation).
    - [x] Write a test that asserts the generated pipeline string matches the expected format from `gst.sh`.
- [x] Task: Initialize and Start Pipeline 1dd862b
    - [x] In `src/main.rs` (or `pipeline.rs`), initialize GStreamer (`gst::init`).
    - [x] Parse the pipeline string into a `gst::Pipeline` object.
    - [x] Set the pipeline state to `Playing`.
    - [x] Implement a basic "bus watch" loop to handle messages (Error, EOS).
- [x] Task: Handle Graceful Shutdown 74f993d
    - [x] Integrate `ctrlc` or `tokio::signal` to detect Ctrl+C.
    - [x] On signal, send the EOS event to the pipeline.
    - [x] Wait for the EOS message on the bus before setting state to `Null` (crucial for valid video files).
- [x] Task: Conductor - User Manual Verification 'Core GStreamer Pipeline Implementation' (Protocol in workflow.md) e777a2a

## Phase 3: Encryption Integration & Verification

- [x] Task: Validate Encryption Logic 74f993d
    - [x] Ensure the pipeline uses `aesenc` with the configured Key/IV.
    - [x] Verify that the `key` and `iv` from config are correctly formatted (hex string) and passed to the element.
- [x] Task: End-to-End Test (Manual) 74f993d
    - [x] Run the Rust app to record a 10-second clip.
    - [x] Use `gst.sh play` (from the original script) to try and decrypt/play the Rust-generated file.
    - [x] Verify that the video plays correctly.
- [~] Task: Conductor - User Manual Verification 'Encryption Integration & Verification' (Protocol in workflow.md)
