# Product Guidelines

## Engineering Principles
- **Reliability First:** The system must prioritize uptime and graceful recovery. Feature parity with the existing bash script is secondary to ensuring the Rust implementation is robust against hardware and resource failures.
- **Idiomatic Rust:** Leverage Rust's strong type system and ownership model to eliminate common bugs. Use `Result` and `Option` for explicit error handling and prefer standard library features where appropriate.
- **Best Tool for the Job:** Prioritize using high-quality, community-maintained crates (e.g., `gstreamer-rs`, `serde`, `log`) rather than reinventing foundational components.

## Architecture & Documentation
- **Documentation-Centric:** The `README.md` and `DESIGN.md` are primary artifacts. They must clearly articulate architectural trade-offs, assumptions, and the rationale behind choosing specific GStreamer elements or encryption strategies.
- **Transparency via Observability:** Integrate structured logging and runtime metrics from the start. The system's internal state (buffer levels, frame rates, disk usage) should be visible to aid in field troubleshooting.
- **Modular but Integrated:** While logic should be cleanly separated into modules (capture, crypto, storage), the focus is on a cohesive, single-purpose application that demonstrates clear "taste" in systems design.

## Operational Robustness
- **Self-Healing Pipelines:** The application should detect pipeline failures or device disconnects and attempt automated re-initialization to minimize data loss.
- **GStreamer-Native Management:** Utilize GStreamer's built-in capabilities (like `splitmuxsink` or equivalent) for robust file rotation and disk space management to prevent resource exhaustion.
- **Configuration-Driven:** All operational parameters—including device paths, encryption keys, and rotation policies—must be configurable via external files, allowing for portability across different hardware setups.
