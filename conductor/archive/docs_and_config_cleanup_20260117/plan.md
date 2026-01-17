# Implementation Plan - Documentation and Configuration Refinement

## Phase 1: Configuration & Legacy Updates

- [x] Task: Update Ports to 8088 5349573
    - [x] Update `gst.sh` DEFAULT_PORT to 8088.
    - [x] Update `conductor/workflow.md` example to use 8088.
    - [x] Verify no other ports need updating (check `config.rs` bitrate is NOT a port).
- [~] Task: Conductor - User Manual Verification 'Configuration & Legacy Updates' (Protocol in workflow.md)

## Phase 2: Documentation Overhaul

- [x] Task: Create DESIGN.md 2c9ab50
    - [x] Document System Overview (CLI -> Rust -> GStreamer).
    - [x] Diagram the Record Pipeline (`v4l2src` -> `aesenc` -> `filesink`).
    - [x] Diagram the Play Pipeline (`filesrc` -> `aesdec` -> `autovideosink`).
    - [x] Explain Configuration (TOML).
- [x] Task: Update README.md 2c9ab50
    - [x] Add Project Description.
    - [x] Add Prerequisites (GStreamer, Rust).
    - [x] Add Installation/Build steps.
    - [x] Add Usage Guide for `record` and `play` commands.
- [~] Task: Conductor - User Manual Verification 'Documentation Overhaul' (Protocol in workflow.md)
