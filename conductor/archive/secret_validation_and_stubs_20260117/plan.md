# Implementation Plan: Secret Validation and Stubs

## Phase 1: New Resolvers (Stubs)
- [x] Modify `src/secrets.rs`:
    - [x] Define `AwsSecretManagerResolver` with `region` and `secret_id` fields (parsed from string).
    - [x] Implement `resolve` returning `Err("Not implemented")`.
    - [x] Define `VaultResolver` with `path` and `key`.
    - [x] Implement `resolve` returning `Err("Not implemented")`.
    - [x] Update `get_resolver` to handle `aws:` and `vault:` prefixes.

## Phase 2: Key Validation
- [x] Modify `src/config.rs`:
    - [x] Create private helper `validate_key(key: &str) -> Result<()>`.
        - [x] Check length == 64.
        - [x] Check `key.chars().all(|c| c.is_ascii_hexdigit())`.
    - [x] Call this in `Config::load` immediately after resolution.

## Phase 3: Verification
- [x] Add tests in `src/config.rs` for invalid key lengths/characters.
- [x] Add tests in `src/secrets.rs` ensuring new resolvers are instantiated and return expected errors.