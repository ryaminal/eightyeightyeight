# Implementation Plan - Implement Streaming and Receiving Modes

## Phase 1: CLI and Configuration [checkpoint: 44bcd3e]

- [x] Task: Update CLI for Networking 3b5d3be
    - [x] Add `Stream` and `Receive` variants to `Commands` enum in `src/cli.rs`.
    - [x] Add destination/listen IP and port arguments with defaults (8088).
    - [x] Update `src/main.rs` to dispatch these subcommands.
- [x] Task: Conductor - User Manual Verification 'CLI and Configuration' (Protocol in workflow.md) 44bcd3e

## Phase 2: Pipeline Implementation

- [ ] Task: Implement Streaming Pipeline Builder
    - [ ] Add `build_stream_pipeline` to `src/pipeline.rs`.
    - [ ] Use `rtph264pay` and `udpsink`.
    - [ ] Ensure `aesenc` settings match `gst.sh stream` (`per-buffer-padding=true`).
    - [ ] Add unit tests for the pipeline string.
- [ ] Task: Implement Receiving Pipeline Builder
    - [ ] Add `build_receive_pipeline` to `src/pipeline.rs`.
    - [ ] Use `udpsrc` and `rtph264depay`.
    - [ ] Ensure `aesdec` settings match `gst.sh receive`.
    - [ ] Add unit tests for the pipeline string.
- [ ] Task: Implement Network Runners
    - [ ] Add `run_stream_pipeline` and `run_receive_pipeline` to `src/pipeline.rs`.
- [ ] Task: Conductor - User Manual Verification 'Pipeline Implementation' (Protocol in workflow.md)

## Phase 3: Integration & Documentation

- [ ] Task: End-to-End Verification
    - [ ] Run `eightyeightyeight stream` and `eightyeightyeight receive` on the same host (localhost).
    - [ ] Verify low-latency video playback.
- [ ] Task: Update README.md
    - [ ] Add "Streaming" and "Receiving" sections with usage examples.
- [ ] Task: Conductor - User Manual Verification 'Integration & Documentation' (Protocol in workflow.md)
