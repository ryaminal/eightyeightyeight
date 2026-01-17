# Implementation Plan: Secure Key Resolution

## Phase 1: Core Abstractions
- [x] Create `src/secrets.rs` module.
- [x] Define `SecretResolver` trait with a `resolve` method.
- [x] Implement `LiteralResolver` (handles raw hex or `literal:` prefix).
- [x] Implement `EnvVarResolver` (handles `env:` prefix).
- [x] Implement `FileResolver` (handles `file:` prefix).

## Phase 2: Configuration Integration
- [x] Update `src/config.rs` to use `src/secrets.rs`.
- [x] Modify `Config::load` to parse the `key` field string.
- [x] Instantiate the correct resolver based on the prefix.
- [x] Resolve the key immediately upon load (fail-fast behavior).

## Phase 3: Refactoring Main
- [x] Update `src/main.rs` to use the new `Config::load` which might return specific resolution errors.
- [x] (Optional) Centralize config loading logic if not already done by the refactor.

## Phase 4: Verification
- [x] Add tests in `src/secrets.rs` for each resolver.
- [x] Add integration tests in `src/config.rs` simulating a full load.
- [x] Verify `cargo test` passes.
- [x] Manual verification with a `config.toml` using `env:` and `file:` sources.