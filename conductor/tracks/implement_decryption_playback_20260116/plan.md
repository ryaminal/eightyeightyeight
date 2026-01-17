# Implementation Plan - Implement Decryption and Playback Mode

## Phase 1: CLI Restructuring [checkpoint: cd274e0]

- [x] Task: Update CLI to Support Subcommands fc2ad67
    - [x] Modify `src/cli.rs` to use `clap::Subcommand`.
    - [x] Define `Record` (with config arg) and `Play` (with config and input file args) variants.
    - [x] Update `src/main.rs` to dispatch execution based on the selected subcommand.
    - [x] Ensure backward compatibility or update documentation if breaking changes are made (likely breaking: `record` becomes explicit).
- [x] Task: Conductor - User Manual Verification 'CLI Restructuring' (Protocol in workflow.md) cd274e0

## Phase 2: Playback Pipeline Implementation [checkpoint: 507ad01]

- [x] Task: Implement Playback Pipeline Builder 2be722f
    - [x] Update `src/pipeline.rs` to include `build_play_pipeline(config: &Config, input_file: &Path)`.
    - [x] Implement the GStreamer string construction for `filesrc ! aesdec ! ... ! autovideosink`.
    - [x] Add unit tests to verify the pipeline string matches `gst.sh play` logic.
- [x] Task: Implement Playback Runner 27074c7
    - [x] Update `src/pipeline.rs` to include `run_play_pipeline`.
    - [x] Reuse or refactor the existing GStreamer bus loop to handle both record and play events (or create a shared helper).
- [x] Task: Conductor - User Manual Verification 'Playback Pipeline Implementation' (Protocol in workflow.md) 507ad01

## Phase 3: Integration & Verification

- [ ] Task: End-to-End Verification
    - [ ] Record a short clip using `eightyeightyeight record`.
    - [ ] Play back the clip using `eightyeightyeight play`.
    - [ ] Verify that the video renders correctly.
- [ ] Task: Conductor - User Manual Verification 'Integration & Verification' (Protocol in workflow.md)
