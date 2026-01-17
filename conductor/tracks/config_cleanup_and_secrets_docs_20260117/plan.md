# Implementation Plan - Configuration Cleanup and Secret Management Docs

## Phase 1: Remove Unused IV [checkpoint: cddbaf2]

- [x] Task: Remove IV from Codebase 0865168
    - [x] Remove `iv` field from `Config` struct in `src/config.rs`.
    - [x] Remove `iv` parameter from `aesenc`/`aesdec` pipeline strings in `src/pipeline.rs` (relying on `serialize-iv=true`).
    - [x] Update `config.toml` example.
    - [x] Fix all compilation errors and update unit tests.
- [x] Task: Conductor - User Manual Verification 'Remove Unused IV' (Protocol in workflow.md) cddbaf2

## Phase 2: Documentation Updates

- [ ] Task: Update Design and Readme
    - [ ] Add "Secret Management" section to `DESIGN.md` discussing K8s/Cloud secrets.
    - [ ] Add "Future Improvements" or similar note to `README.md` about external secret fetching.
- [ ] Task: Conductor - User Manual Verification 'Documentation Updates' (Protocol in workflow.md)
