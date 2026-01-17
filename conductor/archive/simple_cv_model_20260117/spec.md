# Simple CV Model Track

## Goal
Integrate an optional, off-the-shelf Computer Vision model into the video pipeline.

## Motivation
To demonstrate "Edge AI" capabilities as per the product vision, specifically "Integration of lightweight object detection". Using a standard GStreamer element provides a "super simple" implementation path.

## Key Changes
1.  **Config:** Add `cv_enabled` (bool) to `config.toml`.
2.  **Pipeline:** Conditionally inject the `facedetect` element (or `motioncells` as fallback) into the GStreamer pipeline string in `src/pipeline.rs`.
3.  **Error Handling:** Handle cases where the plugin is missing gracefully (warn and continue, or fail depending on strictness).

## Tasks
- [ ] Add `cv_enabled` to `Config` struct in `src/config.rs`.
- [ ] Update `src/pipeline.rs` to insert `facedetect ! videoconvert` if `cv_enabled` is true.
- [ ] Verify functionality (requires `gstreamer1.0-plugins-bad`).
