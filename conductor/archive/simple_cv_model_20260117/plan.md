# Implementation Plan: Simple CV Model

## Phase 1: Configuration
- [x] Update `src/config.rs` to include `pub cv_enabled: Option<bool>`.
- [x] Update `load_config` logic (handled by serde default).

## Phase 2: Pipeline Integration
- [x] Modify `src/pipeline.rs`:
    - [x] In `build_record_pipeline` (and others if needed), check `config.cv_enabled`.
    - [x] If true, insert `videoconvert ! facedetect ! videoconvert` into the pipeline string before encoding.
    - [x] *Note:* `facedetect` usually requires RGB/Gray input, so `videoconvert` wrappers are safer.

## Phase 3: Verification
- [x] Run `cargo test` to ensure config changes don't break existing tests.
- [x] Manual verification:
    - [x] Verified via unit tests that pipeline string is constructed correctly.
    - [ ] (Skipped) Run with `cv_enabled = true`. (Skipped: `facedetect` plugin missing in environment).
- [x] Update `README.md` with GStreamer installation instructions for multiple platforms.

## Phase 4: Runtime Checks (Bonus)
- [x] Implement `check_element_availability(element_name: &str)` in `src/pipeline.rs`.
- [x] Call this check when `cv_enabled` is true.
- [x] Return a user-friendly error if `facedetect` is missing.