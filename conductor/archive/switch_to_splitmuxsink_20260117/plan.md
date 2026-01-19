# Implementation Plan - Switch to SplitMuxSink (Revised: MultiFileSink -> SplitMuxSink)

## Phase 1: Configuration Updates

- [x] Task: Update `Config` struct in `src/config.rs`.
    - [x] Add `max_files: Option<u32>`.
    - [x] Add `max_file_size_mb: Option<u64>`.
- [x] Task: Update `load_config` logic.
    - [x] Ensure defaults are safe (e.g. 0 for unlimited).

## Phase 2: Pipeline Refactoring

- [x] Task: Refactor `build_record_pipeline` in `src/pipeline.rs`.
    - [x] Switch to `splitmuxsink` for rotation.
    - [x] Handle `max-size-bytes` (bytes).
    - [x] Handle `location` pattern logic (ensure it has `%d`).
    - [x] **Pivot:** Switched to `aes-256-cbc` (requires 32-byte key) due to `aes-128-ctr` missing in local GStreamer plugins.
    - [x] **Optimization:** Use `rndbuffersize min=752` to align MPEG-TS packets (4x188) with AES block size (16) to minimize padding issues.
    - [x] **Stability:** Enabled `async-finalize=true` to prevent crashes during file rotation.
- [x] Task: Implement Rotation Logic in `run_pipeline_loop`.
    - [x] `splitmuxsink` handles rotation natively. Removed manual file deletion logic (handled by `max-files`).
- [x] Task: Update unit tests for pipeline string generation.

## Phase 3: Verification

- [x] Task: Verify recording with splitting.
    - [x] Verified `splitmuxsink` creates multiple files.
- [x] Task: Verify rotation.
    - [x] Verified old files are deleted when `max-files` limit is reached.
- [x] Task: Conductor - User Manual Verification 'Loop Recording'.
    - [x] Verified stable recording without crashes.
    - [ ] Note: Playback of rotated segments works but may occasionally show "No program activated" if segment is very short or headers missing (mitigated by splitmuxsink header injection).