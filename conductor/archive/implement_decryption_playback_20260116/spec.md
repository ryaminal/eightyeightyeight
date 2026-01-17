# Track Specification: Implement Decryption and Playback Mode

## 1. Overview
This track focuses on implementing the decryption and playback functionality for the `eightyeightyeight` application. This will allow the application to play back the encrypted video files it produces, removing the dependency on the external `gst.sh` script for verification and viewing.

## 2. Goals
- **CLI Expansion:** Update the CLI to support distinct modes of operation: `record` (default/existing behavior) and `play`.
- **Decryption Pipeline:** Implement a GStreamer pipeline that reads an encrypted file, decrypts it using the configured key/IV, and renders it to the screen.
- **Parity with `gst.sh`:** The playback logic must match the `gst.sh play` command to ensure compatibility with existing files.

## 3. Key Features
- **Subcommands:**
    - `eightyeightyeight record` (or just `eightyeightyeight` for backward compatibility if desired, but explicit is better).
    - `eightyeightyeight play --input <FILE>`.
- **Playback Pipeline:**
    - Source: `filesrc`
    - Decryption: `aesdec` (using key from config)
    - Demux/Parse: `tsdemux` -> `h264parse`
    - Decode/Display: `decodebin` -> `autovideosink`
- **Config Usage:** The `play` command should still load the configuration file to retrieve the correct encryption key and IV.

## 4. Technical Considerations
- **GStreamer Elements:** Use `aesdec` with `serialize-iv=true` and `per-buffer-padding=false` to match the recording format.
- **Error Handling:** Gracefully handle "file not found" or "decryption failed" (though GStreamer might just error out on the stream, we should catch that bus message).

## 5. Success Criteria
- The application compiles and passes `cargo clippy`.
- `eightyeightyeight play --input output.ts.enc` successfully opens a window and plays the recorded video.
- `eightyeightyeight record` continues to function as expected.
