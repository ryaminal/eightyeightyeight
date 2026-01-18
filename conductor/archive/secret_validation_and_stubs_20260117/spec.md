# Secret Validation and Stubs Track

## Goal
Enhance the secure key resolution system by adding validation for key length (AES-256) and stubbing out future integrations for AWS Secrets Manager and HashiCorp Vault.

## Motivation
- **Validation:** Catch configuration errors early. GStreamer's error messages for invalid keys can be cryptic. We should ensure the resolved key is a valid 32-byte (64 hex char) string.
- **Extensibility:** Demonstrate the power of the `SecretResolver` trait by sketching out how enterprise integrations would look.

## Key Changes
1.  **Validation:** In `Config::load`, after resolving the key, verify it is a 64-character hex string.
2.  **Stubs:** Add `AwsSecretManagerResolver` and `VaultResolver` to `src/secrets.rs`.

## Tasks
- [ ] Add `AwsSecretManagerResolver` struct and impl `SecretResolver` (returning error).
- [ ] Add `VaultResolver` struct and impl `SecretResolver` (returning error).
- [ ] Update `get_resolver` factory to recognize `aws:` and `vault:` prefixes.
- [ ] Implement `validate_key_format(key: &str)` in `src/config.rs`.
- [ ] Call validation in `Config::load`.
- [ ] Add tests for validation (valid/invalid keys) and stubs.
