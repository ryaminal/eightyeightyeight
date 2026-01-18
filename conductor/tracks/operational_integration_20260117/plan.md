# Implementation Plan - Operational Integration

## Phase 1: Systemd Service

- [x] Task: Create `eightyeightyeight.service` file.
    - [x] Define `[Unit]`, `[Service]`, and `[Install]` sections.
    - [x] `ExecStart` should point to the `eightyeightyeight` binary with the `record` subcommand.
    - [x] Configure `Restart=on-failure`.
- [x] Task: Add documentation for `systemd` usage in `README.md`.

## Phase 2: Disk Space Monitoring

- [x] Task: Implement disk space check in `src/config.rs` and `src/main.rs`.
    - [x] Add `min_disk_space_mb` to `Config` struct.
    - [x] In `main.rs`, before starting the record pipeline, check available disk space of the filesystem where `output_path` is located.
- [x] Task: Periodically check disk space during recording.
    - [x] In `run_record_pipeline` in `src/pipeline.rs`, spawn a thread to check disk space every 10 seconds.
    - [x] If disk space is below the threshold, send an EOS event to the pipeline bus to trigger a graceful shutdown.
- [x] Task: Add unit tests for disk space checking logic.

## Phase 2 Checkpoint: [checkpoint: pending]
