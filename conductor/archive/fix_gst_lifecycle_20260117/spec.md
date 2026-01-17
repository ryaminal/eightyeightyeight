# Track Specification: Fix GStreamer Pipeline Lifecycle and Error Handling

## 1. Overview
This track addresses critical lifecycle management issues in the GStreamer pipeline implementation. Currently, if an error occurs during playback (e.g., window closed), the function returns early without setting the pipeline state to `NULL`, causing GStreamer critical warnings and potential resource leaks.

## 2. Goals
- **Ensure Cleanup:** Guarantee that the pipeline state is set to `NULL` before the pipeline object is dropped, regardless of success or failure.
- **Improved Error Handling:** Differentiate between fatal errors and user-initiated interruptions (like closing the window).

## 3. Key Changes
- **Refactor `run_pipeline`:** Use a structure or pattern (like `defer` or a manual `finally` block logic) to ensure `pipeline.set_state(gst::State::Null)` is called on all exit paths.
- **Handle Window Close:** Treat "Output window was closed" as a normal shutdown signal rather than a hard error, if possible, or at least clean up gracefully.

## 4. Success Criteria
- The "Trying to dispose element ... but it is in PLAYING" warning **must not** appear when the window is closed or an error occurs.
- The application exits cleanly.
