# Implementation Plan: Secure Key Resolution

## Phase 1: Core Abstractions
- [ ] Create `src/secrets.rs` module.
- [ ] Define `SecretResolver` trait with a `resolve` method.
- [ ] Implement `LiteralResolver` (handles raw hex or `literal:` prefix).
- [ ] Implement `EnvVarResolver` (handles `env:` prefix).
- [ ] Implement `FileResolver` (handles `file:` prefix).

## Phase 2: Configuration Integration
- [ ] Update `src/config.rs` to use `src/secrets.rs`.
- [ ] Modify `Config::load` to parse the `key` field string.
- [ ] Instantiate the correct resolver based on the prefix.
- [ ] Resolve the key immediately upon load (fail-fast behavior).

## Phase 3: Refactoring Main
- [ ] Update `src/main.rs` to use the new `Config::load` which might return specific resolution errors.
- [ ] (Optional) Centralize config loading logic if not already done by the refactor.

## Phase 4: Verification
- [ ] Add tests in `src/secrets.rs` for each resolver.
- [ ] Add integration tests in `src/config.rs` simulating a full load.
- [ ] Verify `cargo test` passes.
- [ ] Manual verification with a `config.toml` using `env:` and `file:` sources.
