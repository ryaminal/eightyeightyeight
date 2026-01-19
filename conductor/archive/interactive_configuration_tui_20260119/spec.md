# Track Specification: Interactive Configuration TUI

## 1. Overview
This track implements an interactive command-line wizard to assist users in creating `config.toml` files. It simplifies the setup process by discovering available hardware, providing sensible defaults, and auto-generating secure encryption keys.

## 2. Goals
- **Ease of Use:** Reduce the friction of setting up the application for the first time.
- **Hardware Discovery:** Automatically list available video devices to prevent user error in path entry.
- **Security:** Auto-generate strong, random 32-byte hex keys for encryption.
- **Validation:** Ensure valid inputs for bitrate, framerate, and resolution.

## 3. Key Features
- **New Subcommand:** `eightyeightyeight init` (or `wizard`).
- **Device Selection:** Interactive list of `/dev/video*` devices.
- **Parameter Prompts:**
    - Resolution (Width/Height) with defaults (640x480).
    - Framerate (default 30/1).
    - Bitrate (default 1000 Kbps).
    - Output Path (default `output.ts.enc`).
    - Face Detection toggle.
- **Key Generation:** Option to generate a new random key automatically.
- **Output:** Write the resulting TOML to a file.

## 4. Technical Approach
- **Library:** Use `inquire` for robust, cross-platform interactive prompts (text, select, confirm).
- **Device Scanning:** glob `/dev/video*` to find candidate devices.
- **Serialization:** Reuse existing `Config` struct and `toml` serializer.

## 5. Success Criteria
- Running `eightyeightyeight init` launches the wizard.
- The wizard produces a valid `config.toml` file.
- The generated config can be immediately used by `record` and `play` commands.
