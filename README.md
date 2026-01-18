# Eightyeightyeight

A robust, secure video capture and playback application for embedded Linux, written in Rust.

## Description

`eightyeightyeight` demonstrates a production-grade GStreamer implementation in Rust. It provides a CLI for recording encrypted video from a webcam and playing it back, prioritizing reliability and correctness over raw feature count.

## Features

- **Secure Recording:** Captures video and encrypts it on-the-fly using AES-256.
- **Playback:** Decrypts and plays back the secure footage.
- **Computer Vision:** Optional face detection overlay using GStreamer OpenCV plugins.
- **Robustness:** Handles graceful shutdowns (Ctrl+C) to ensure data integrity.
- **Configurable:** Fully driven by a TOML configuration file for hardware adaptability.

## Prerequisites

- **Rust:** Latest stable toolchain.
- **GStreamer:** Development libraries and plugins.
  - **Ubuntu / Debian:**
    ```bash
    sudo apt install libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev gstreamer1.0-plugins-base gstreamer1.0-plugins-good gstreamer1.0-plugins-bad gstreamer1.0-plugins-ugly gstreamer1.0-libav gstreamer1.0-opencv
    ```
  - **Fedora:**
    ```bash
    sudo dnf install gstreamer1-devel gstreamer1-plugins-base-devel gstreamer1-plugins-base gstreamer1-plugins-good gstreamer1-plugins-bad-free gstreamer1-plugins-bad-free-opencv gstreamer1-plugins-ugly-free gstreamer1-libav
    ```
  - **Arch Linux:**
    ```bash
    sudo pacman -S gstreamer gst-plugins-base gst-plugins-good gst-plugins-bad gst-plugins-ugly gst-libav
    ```
  - **macOS (Homebrew):**
    ```bash
    brew install gstreamer gst-plugins-base gst-plugins-good gst-plugins-bad gst-plugins-ugly gst-libav
    ```

*Note: The `gst-plugins-bad` package is required for the optional `cv_enabled` (face detection) feature.*

## Installation

```bash
git clone <repository-url>
cd eightyeightyeight
cargo build --release
```

## Usage

The application uses a `config.toml` file for settings.

### 1. Configuration

Create a `config.toml` file:

```toml
device = "/dev/video0"      # Use "auto" for automatic device selection
width = 640
height = 480
framerate = "30/1"
bitrate = 1000
key = "00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff" # 32-byte hex key (64 chars)
output_path = "output.ts.enc"
cv_enabled = false # Optional: Set to true to enable face detection (requires plugins-bad)
```

### 2. Recording

To start recording:

```bash
./target/release/eightyeightyeight record --config config.toml
```

Press `Ctrl+C` to stop recording. The application will finalize the file and exit.

### 3. Playback

To play back a recorded file:

```bash
./target/release/eightyeightyeight play --config config.toml --input output.ts.enc
```

### 4. Network Streaming

You can stream encrypted video over the network (UDP/RTP).

**On the Receiver (start first):**

```bash
./target/release/eightyeightyeight receive --port 8088 --config config.toml
```

**On the Streamer:**

```bash
./target/release/eightyeightyeight stream --dest <RECEIVER_IP> --port 8088 --config config.toml
```

## Architecture

See [DESIGN.md](./DESIGN.md) for details on the system architecture and pipeline design.

## Future Improvements

- **Secure Key Management:** Integration with Kubernetes Secrets or Cloud KMS to avoid storing keys in plain text `config.toml`.
- **Hardware Acceleration:** Support for hardware-specific encoding/decoding elements (e.g., `vaapih264enc`, `omxh264enc`).