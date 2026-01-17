# Track Specification: Configuration Cleanup and Secret Management Docs

## 1. Overview
This track focuses on cleaning up the configuration by removing the unused `iv` field and improving documentation regarding secret management best practices.

## 2. Goals
- **Refine Configuration:** Remove the `iv` field from `Config` struct and `config.toml` if verified as unnecessary when `serialize-iv=true` is used.
- **Improve Documentation:** Document that keys should ideally be fetched from secure storage (K8s Secrets, AWS Secrets Manager, etc.) in a production environment.

## 3. Key Actions
- **Code Change:** Remove `iv` from `src/config.rs` and `src/pipeline.rs`.
- **Test Update:** Update tests to reflect the schema change.
- **Documentation:**
    - Update `DESIGN.md` with a "Secret Management" section.
    - Update `README.md` config example.

## 4. Success Criteria
- `cargo test` passes without the `iv` field.
- Pipelines (record/play/stream/receive) still function correctly (verified via tests or logic check).
- Documentation clearly reflects the "Production Readiness" aspect of secret management.
