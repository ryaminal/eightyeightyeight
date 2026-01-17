# Implementation Plan - Fix GStreamer Pipeline Lifecycle and Error Handling

## Phase 1: Robust Lifecycle Management

- [x] Task: Implement Cleanup Guard 1192a2c
    - [x] Create a `PipelineGuard` struct in `src/pipeline.rs` that wraps the `gst::Pipeline`.
    - [x] Implement `Drop` for `PipelineGuard` to automatically set the state to `NULL`.
    - [x] Update `run_pipeline` to use this guard.
    - [x] Verify fix by running `play` and closing the window manually.
- [~] Task: Conductor - User Manual Verification 'Robust Lifecycle Management' (Protocol in workflow.md)
