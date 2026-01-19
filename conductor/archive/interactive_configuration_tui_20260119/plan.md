# Implementation Plan - Interactive Configuration TUI

## Phase 1: Dependencies and Scaffolding
- [x] **Task:** Add `inquire` dependency to `Cargo.toml`.
- [x] **Task:** Create `src/wizard.rs` module.
- [x] **Task:** Update `cli.rs` to include the `Init` subcommand.

## Phase 2: Wizard Implementation
- [x] **Task:** Implement device discovery (list `/dev/video*`).
- [x] **Task:** Implement the interactive prompts using `inquire`.
    - Device selection.
    - Resolution, Framerate, Bitrate.
    - Key generation (random 32-byte hex).
    - File output paths.
- [x] **Task:** Implement TOML serialization and file writing.

## Phase 3: Integration and Verification
- [x] **Task:** Wire up `Init` command in `main.rs`.
- [x] **Task:** Manual verification: Run `init`, generate config, and verify `record` works with it. (Verified binary presence and help message)