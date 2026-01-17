# Track Specification: Port `gst.sh` Prototype to Rust

## 1. Overview
This track focuses on establishing the foundational Rust codebase by porting the proven GStreamer pipeline from the `gst.sh` script. The goal is to create a robust, configurable CLI application that can capture video, encrypt it, and save it to disk, meeting the mandatory assignment requirements.

## 2. Goals
- **Scaffold Project:** Initialize a new Rust project with the necessary dependencies (`gstreamer`, `clap`, `serde`, `ring`/`aes`).
- **Port Pipeline:** Reconstruct the `v4l2src` -> `x264enc` -> `aesenc` -> `filesink` pipeline using `gstreamer-rs`.
- **Implement Configuration:** Replace hardcoded script variables with a TOML-based configuration system.
- **Ensure Reliability:** Implement basic error handling for pipeline failures and signals (Ctrl+C) to ensure files are finalized correctly.
- **Observability:** Integrate `tracing` for structured logging of pipeline state and errors.

## 3. Key Features
- **CLI Interface:** Accepts command-line arguments for config file path and basic commands (record, play - optional for this phase).
- **Configurable Pipeline:**
    - Source device (e.g., `/dev/video0`)
    - Resolution and Framerate
    - Encryption Key and IV
    - Output filename/directory
- **Encryption:** AES-256 (or 128 as in script) encryption compatible with the `gst.sh` logic.
- **Graceful Shutdown:** Catches `SIGINT` to send EOS (End of Stream) to the pipeline, ensuring MP4/MKV headers are written correctly.

## 4. Technical Considerations
- **GStreamer Bindings:** Use `gstreamer` crate. Need to ensure the "unsafe" parts of GStreamer are wrapped safely or handled correctly.
- **Encryption Compatibility:** The Rust implementation must produce files compatible with the decryption logic (or the `play` command) to ensure verifiable security.
- **Async Runtime:** Likely use `tokio` for handling signals and potentially future async tasks, though the main GStreamer loop might run on its own thread.

## 5. Success Criteria
- The Rust application compiles and runs without warnings.
- It captures video from the webcam.
- It produces an encrypted file on disk.
- The encrypted file can be decrypted and played back (either by `gst.sh play` or a new Rust equivalent).
- Logs are produced via `tracing`.
