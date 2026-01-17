# Track Specification: Comprehensive Sanity Check

## 1. Overview
This track involves a manual verification pass of the application's core features to ensure recent changes (port updates, config cleanup, playback implementation) work as intended. This serves as a reset for the strict user verification protocol.

## 2. Goals
- **Verify Recording:** Confirm `record` works with the updated config.
- **Verify Playback:** Confirm `play` works and handles graceful shutdown.
- **Verify Streaming:** Confirm `stream` and `receive` work together.
- **Verify Config:** Confirm the app works without the `iv` field in `config.toml`.

## 3. Success Criteria
- User explicitly confirms success for each verification step.
