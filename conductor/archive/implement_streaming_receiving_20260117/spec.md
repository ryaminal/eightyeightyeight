# Track Specification: Implement Streaming and Receiving Modes

## 1. Overview
This track focuses on porting the `stream` and `receive` modes from the `gst.sh` script to the Rust application. This will enable network streaming of encrypted video over UDP/RTP and real-time reception/decryption.

## 2. Goals
- **CLI Expansion:** Add `stream` and `receive` subcommands.
- **Streaming Pipeline:** Implement a GStreamer pipeline that captures, encodes, encrypts, and sends video over RTP/UDP.
- **Receiving Pipeline:** Implement a GStreamer pipeline that receives RTP/UDP packets, decrypts, and plays the video in real-time.
- **Parity with `gst.sh`:** Ensure network compatibility with the original script for hybrid testing.

## 3. Key Features
- **`stream` Subcommand:**
    - Arguments: `--dest <IP>` (default 127.0.0.1), `--port <PORT>` (default 8088), `--config <FILE>`.
    - Pipeline: `v4l2src ! ... ! rtph264pay ! aesenc (packetized) ! udpsink`.
- **`receive` Subcommand:**
    - Arguments: `--listen <IP>` (default 0.0.0.0), `--port <PORT>` (default 8088), `--config <FILE>`.
    - Pipeline: `udpsrc ! aesdec (packetized) ! rtph264depay ! ... ! autovideosink`.
- **Encryption Settings:** Use `per-buffer-padding=true` and `serialize-iv=true` for streaming mode (as identified in `gst.sh`).

## 4. Technical Considerations
- **Network Compatibility:** RTP payloads must be handled correctly (payload type, clock-rate).
- **Encryption Gotchas:** Packetized encryption requires padding to be handled per-buffer.

## 5. Success Criteria
- `eightyeightyeight stream` sends encrypted video to a destination.
- `eightyeightyeight receive` displays the stream.
- Interoperability with `gst.sh` (e.g., Rust stream -> Bash receive).
- `README.md` updated with examples.
