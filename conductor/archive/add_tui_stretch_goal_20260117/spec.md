# Track Specification: Add TUI Stretch Goal

## 1. Overview
This track updates the Product Definition to include a Terminal User Interface (TUI) as a stretch goal. The TUI will focus on easing configuration management (device selection, resolution settings, keys) and potentially controlling the application (record/play).

## 2. Goals
- **Update Product Definition:** Add "Configuration TUI" to the "Extended Capabilities (Stretch Goals)" section in `conductor/product.md`.

## 3. Key Features (Planned)
- **Device Discovery:** List available V4L2 devices and their capabilities.
- **Interactive Config:** Wizard-style prompts to generate `config.toml`.
- **Context Management:** Manage multiple configuration profiles (contexts).
- **Command Interface:** Trigger `record` and `play` commands from the TUI.

## 4. Success Criteria
- `conductor/product.md` reflects the new stretch goal.
