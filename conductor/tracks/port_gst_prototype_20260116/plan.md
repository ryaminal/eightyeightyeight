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

## Phase 2: Core GStreamer Pipeline Implementation

- [ ] Task: Construct the Record Pipeline
    - [ ] Create `src/pipeline.rs`.
    - [ ] Implement a builder function that constructs the GStreamer pipeline string dynamically from the `Config` struct (similar to how `gst.sh` does string concatenation).
    - [ ] Write a test that asserts the generated pipeline string matches the expected format from `gst.sh`.
- [ ] Task: Initialize and Start Pipeline
    - [ ] In `src/main.rs` (or `pipeline.rs`), initialize GStreamer (`gst::init`).
    - [ ] Parse the pipeline string into a `gst::Pipeline` object.
    - [ ] Set the pipeline state to `Playing`.
    - [ ] Implement a basic "bus watch" loop to handle messages (Error, EOS).
- [ ] Task: Handle Graceful Shutdown
    - [ ] Integrate `ctrlc` or `tokio::signal` to detect Ctrl+C.
    - [ ] On signal, send the EOS event to the pipeline.
    - [ ] Wait for the EOS message on the bus before setting state to `Null` (crucial for valid video files).
- [ ] Task: Conductor - User Manual Verification 'Core GStreamer Pipeline Implementation' (Protocol in workflow.md)

## Phase 3: Encryption Integration & Verification

- [ ] Task: Validate Encryption Logic
    - [ ] Ensure the pipeline uses `aesenc` with the configured Key/IV.
    - [ ] Verify that the `key` and `iv` from config are correctly formatted (hex string) and passed to the element.
- [ ] Task: End-to-End Test (Manual)
    - [ ] Run the Rust app to record a 10-second clip.
    - [ ] Use `gst.sh play` (from the original script) to try and decrypt/play the Rust-generated file.
    - [ ] Verify that the video plays correctly.
- [ ] Task: Conductor - User Manual Verification 'Encryption Integration & Verification' (Protocol in workflow.md)
