# Track Specification: Handle 'Window Closed' Error Gracefully

## 1. Overview
When the user closes the playback window or interrupts playback, GStreamer emits an "Output window was closed" error. Currently, the application logs this as a standard ERROR and returns a failure result. This track aims to recognize this specific condition and treat it as a normal shutdown event.

## 2. Goals
- **Suppress Error Log:** Identify the "Output window was closed" error.
- **Graceful Exit:** Log a user-friendly INFO message instead of an ERROR.
- **Return Success:** Return `Ok(())` instead of `Err` in this scenario.

## 3. Key Changes
- **Update `src/pipeline.rs`:** In the bus message loop, inside the `MessageView::Error` match arm, check if the error matches the "Output window was closed" condition (likely a `ResourceError`).

## 4. Success Criteria
- Closing the window during playback results in an "INFO: Playback stopped by user" (or similar) message, not an ERROR log stack trace.
- The application exits with code 0 in this case.
