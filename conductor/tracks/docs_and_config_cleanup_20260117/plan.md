# Implementation Plan - Documentation and Configuration Refinement

## Phase 1: Configuration & Legacy Updates

- [x] Task: Update Ports to 8088 5349573
    - [x] Update `gst.sh` DEFAULT_PORT to 8088.
    - [x] Update `conductor/workflow.md` example to use 8088.
    - [x] Verify no other ports need updating (check `config.rs` bitrate is NOT a port).
- [~] Task: Conductor - User Manual Verification 'Configuration & Legacy Updates' (Protocol in workflow.md)

## Phase 2: Documentation Overhaul

- [ ] Task: Create DESIGN.md
    - [ ] Document System Overview (CLI -> Rust -> GStreamer).
    - [ ] Diagram the Record Pipeline (`v4l2src` -> `aesenc` -> `filesink`).
    - [ ] Diagram the Play Pipeline (`filesrc` -> `aesdec` -> `autovideosink`).
    - [ ] Explain Configuration (TOML).
- [ ] Task: Update README.md
    - [ ] Add Project Description.
    - [ ] Add Prerequisites (GStreamer, Rust).
    - [ ] Add Installation/Build steps.
    - [ ] Add Usage Guide for `record` and `play` commands.
- [ ] Task: Conductor - User Manual Verification 'Documentation Overhaul' (Protocol in workflow.md)
