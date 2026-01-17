# Implementation Plan - Implement Streaming and Receiving Modes

## Phase 1: CLI and Configuration [checkpoint: 44bcd3e]

- [x] Task: Update CLI for Networking 3b5d3be
    - [x] Add `Stream` and `Receive` variants to `Commands` enum in `src/cli.rs`.
    - [x] Add destination/listen IP and port arguments with defaults (8088).
    - [x] Update `src/main.rs` to dispatch these subcommands.
- [x] Task: Conductor - User Manual Verification 'CLI and Configuration' (Protocol in workflow.md) 44bcd3e

## Phase 2: Pipeline Implementation [checkpoint: ef33ea2]

- [x] Task: Implement Streaming Pipeline Builder 893ef43
    - [x] Add `build_stream_pipeline` to `src/pipeline.rs`.
    - [x] Use `rtph264pay` and `udpsink`.
    - [x] Ensure `aesenc` settings match `gst.sh stream` (`per-buffer-padding=true`).
    - [x] Add unit tests for the pipeline string.
- [x] Task: Implement Receiving Pipeline Builder 893ef43
    - [x] Add `build_receive_pipeline` to `src/pipeline.rs`.
    - [x] Use `udpsrc` and `rtph264depay`.
    - [x] Ensure `aesdec` settings match `gst.sh receive`.
    - [x] Add unit tests for the pipeline string.
- [x] Task: Implement Network Runners 893ef43
    - [x] Add `run_stream_pipeline` and `run_receive_pipeline` to `src/pipeline.rs`.
- [x] Task: Conductor - User Manual Verification 'Pipeline Implementation' (Protocol in workflow.md) ef33ea2

## Phase 3: Integration & Documentation [checkpoint: fc5afba]

- [x] Task: End-to-End Verification 893ef43
    - [x] Run `eightyeightyeight stream` and `eightyeightyeight receive` on the same host (localhost).
    - [x] Verify low-latency video playback.
- [x] Task: Update README.md 893ef43
    - [x] Add "Streaming" and "Receiving" sections with usage examples.
- [x] Task: Conductor - User Manual Verification 'Integration & Documentation' (Protocol in workflow.md) fc5afba
