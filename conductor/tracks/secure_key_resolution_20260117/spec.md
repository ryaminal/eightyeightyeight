# Secure Key Resolution Track

## Goal
Implement a secure, extensible system for resolving sensitive configuration values (like encryption keys) to avoid storing them in plaintext config files.

## Motivation
Hardcoding secrets in `config.toml` is a security risk. Production systems require fetching secrets from secure sources like environment variables, local files (e.g., Kubernetes secrets mounts), or remote secret managers.

## Key Changes
1.  **SecretResolver Trait:** Define an interface for secret resolution strategies.
2.  **Implementations:**
    -   `EnvVarResolver`: `env:VAR_NAME`
    -   `FileResolver`: `file:/path/to/secret`
    -   `LiteralResolver`: `literal:HEX_KEY` (backward compatibility)
3.  **Config Refactor:** Update `Config` struct to hold a resolved key but load from a source string.
4.  **Integration:** Wire up resolution logic during config loading.

## Tasks
- [ ] Define `SecretResolver` trait in `src/config.rs` (or `src/secrets.rs`).
- [ ] Implement `EnvVarResolver`, `FileResolver`, and `LiteralResolver`.
- [ ] Refactor `Config` loading to parse the `key` field and delegate to the appropriate resolver.
- [ ] Update `main.rs` to handle potential resolution errors gracefully.
- [ ] Add unit tests for each resolver and the integration logic.
