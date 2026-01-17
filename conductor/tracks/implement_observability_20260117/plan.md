# Implementation Plan - Implement System Observability

## Phase 1: Dependencies and Scaffolding

- [x] Task: Add Observability Dependencies b57a6f6
    - [x] Add `opentelemetry`, `opentelemetry_sdk`, `opentelemetry-prometheus`, `prometheus` to `Cargo.toml`.
    - [x] Update `src/main.rs` to initialize the metrics pipeline.
- [~] Task: Conductor - User Manual Verification 'Dependencies and Scaffolding' (Protocol in workflow.md)

## Phase 2: Pipeline Instrumentation

- [ ] Task: Implement Pad Probes
    - [ ] Create `src/metrics.rs` to handle metric registration and updates.
    - [ ] In `src/pipeline.rs`, attach probes to the `v4l2src` (source) and `filesink`/`udpsink` (sink) pads.
    - [ ] Calculate FPS and Bitrate in the probe callback.
- [ ] Task: Integrate Metrics Reporting
    - [ ] Periodically log the collected metrics (or expose via Prometheus endpoint if we go that route).
    - [ ] For simplicity/MVP: Log "Metrics: FPS=30.0, Bitrate=1024kbps" every 5 seconds.
- [ ] Task: Conductor - User Manual Verification 'Pipeline Instrumentation' (Protocol in workflow.md)

## Phase 3: Integration & Verification

- [ ] Task: Verify Metrics Output
    - [ ] Run `record` and check logs for metric updates.
    - [ ] Verify overhead is minimal.
- [ ] Task: Conductor - User Manual Verification 'Integration & Verification' (Protocol in workflow.md)
