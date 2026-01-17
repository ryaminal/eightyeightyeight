# Track Specification: Implement System Observability

## 1. Overview
This track focuses on adding structured metrics to the application to provide visibility into its operational health. We will integrate OpenTelemetry to capture and export key performance indicators (KPIs) from the GStreamer pipeline.

## 2. Goals
- **Metrics Collection:** Capture critical pipeline metrics:
    - Frame rate (FPS)
    - Bitrate
    - Pipeline errors (counters)
    - Pipeline state changes
- **Metrics Export:** Expose these metrics via a standard exporter (e.g., Prometheus-compatible endpoint or stdout for now).
- **Integration:** Integrate `tracing-opentelemetry` or `opentelemetry` crates.

## 3. Key Features
- **Pipeline Monitoring:** Attach probes to GStreamer pads (e.g., source, sink) to count buffers and bytes.
- **Metrics Server (Optional/Lite):** A simple way to scrape these metrics (or just log them periodically for the MVP). *Decision: For MVP, we will stick to periodic structured logging of metrics to avoid adding a full HTTP server dependency unless necessary, or use a simple Prometheus exporter.*

## 4. Technical Considerations
- **GStreamer Probes:** Use `gst::Pad::add_probe` to intercept data flow without blocking it.
- **Async Metrics:** Ensure metric collection doesn't block the main loop.

## 5. Success Criteria
- Running `record` or `stream` produces periodic log entries (e.g., every 5s) showing FPS and Bitrate.
- `cargo test` passes.
