# Technology Stack

## Core Language & Runtime
- **Language:** [Rust](https://www.rust-lang.org/)
- **Reasoning:** Provides the memory safety, performance, and low-level control required for reliable systems programming and real-time video processing.

## Media Processing
- **Framework:** [GStreamer](https://gstreamer.freedesktop.org/) (via [`gstreamer-rs`](https://gitlab.freedesktop.org/gstreamer/gstreamer-rs))
- **Reasoning:** Industry-standard multimedia framework with a robust plugin architecture. The Rust bindings offer a safe interface to GStreamer's powerful pipeline capabilities.

## User Interface
- **TUI Framework:** [Ratatui](https://ratatui.rs/)
- **Reasoning:** Enables a lightweight, responsive, and professional terminal-based interface for monitoring the capture pipeline without the overhead of a full GUI.

## Security & Encryption
- **Library:** GStreamer `aes` elements (current port) with [RustCrypto](https://github.com/RustCrypto) planned for future utilities.
- **Encryption Standard:** AES-256
- **Reasoning:** Leverages proven GStreamer pipeline elements for the initial port to ensure compatibility with existing decryption logic. Future components will prioritize Rust-native crypto libraries.

## Observability
- **Standard:** [OpenTelemetry](https://opentelemetry.io/)
- **Integration:** [`tracing`](https://github.com/tokio-rs/tracing) with OpenTelemetry exporters.
- **Reasoning:** Provides a unified, professional approach to logging and metrics, ensuring the system's internal state is transparent and troubleshootable.

## Configuration & Tooling
- **Serialization:** [Serde](https://serde.rs/) (TOML format)
- **Deployment:** `systemd` (Service units for supervision and auto-restart).
- **Reasoning:** TOML provides a human-readable and standard configuration format. `systemd` ensures operational reliability in embedded Linux environments.
