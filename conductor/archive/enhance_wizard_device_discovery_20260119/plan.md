# Implementation Plan - Enhance Wizard Device Discovery

## Phase 1: Preparation
- [x] **Task:** Create `src/gst_utils.rs` (or similar, maybe just keep in wizard for now) to handle GStreamer probing logic.
- [x] **Task:** Ensure `gstreamer` features in `Cargo.toml` support `DeviceMonitor` (it's core, so should be fine).

## Phase 2: Implementation
- [x] **Task:** Modify `src/wizard.rs`:
    - [x] Initialize GStreamer at the start of `run()`.
    - [x] Replace `fs::read_dir` logic with `gst::DeviceMonitor` logic.
    - [x] Implement a helper struct to hold Device info (Display Name, Caps).
- [x] **Task:** Implement Capability Parsing:
    - [x] Extract Width, Height, Framerate from Caps.
    - [x] Deduplicate and sort modes (e.g., sort by Resolution desc, then FPS desc).
- [x] **Task:** Update Prompts:
    - [x] "Select Video Device" (shows Name + Path).
    - [x] "Select Capture Mode" (shows Width x Height @ FPS).

## Phase 3: Verification
- [x] **Task:** Run `cargo run -- init` and verify it detects the local camera (if available) or dummy devices if `videotestsrc` acts like one (it usually doesn't show up in DeviceMonitor, but v4l2loopback might).
- [x] **Task:** Verify `config.toml` output is correct.