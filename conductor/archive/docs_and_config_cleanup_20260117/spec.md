# Track Specification: Documentation and Configuration Refinement

## 1. Overview
This track focuses on cleaning up project documentation, establishing the architectural design document, and ensuring consistency with the "888" theme by updating default ports.

## 2. Goals
- **Theme Consistency:** Update all default and test ports to `8088`.
- **Documentation:**
    - Create `DESIGN.md` to document the system architecture (Rust + GStreamer).
    - Update `README.md` with instructions for the new CLI subcommands (`record`, `play`).
- **Legacy Cleanup:** Update `gst.sh` defaults to match the new port standard.

## 3. Key Actions
- **Update Ports:** Search and replace `5000` or `8080` with `8088` in `gst.sh` and documentation.
- **Create `DESIGN.md`:** Document the high-level design, pipeline architecture, and encryption strategy.
- **Update `README.md`:** Add "Getting Started", "Usage", and "Configuration" sections.

## 4. Success Criteria
- `gst.sh` uses port 8088 by default.
- `DESIGN.md` exists and accurately describes the current system.
- `README.md` provides clear instructions for recording and playback.
- `cargo clippy` and `cargo fmt` pass.
