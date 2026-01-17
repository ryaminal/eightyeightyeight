# Eightyeightyeight

A robust, secure video capture and playback application for embedded Linux, written in Rust.

## Description

`eightyeightyeight` demonstrates a production-grade GStreamer implementation in Rust. It provides a CLI for recording encrypted video from a webcam and playing it back, prioritizing reliability and correctness over raw feature count.

## Features

- **Secure Recording:** Captures video and encrypts it on-the-fly using AES-256.
- **Playback:** Decrypts and plays back the secure footage.
- **Robustness:** Handles graceful shutdowns (Ctrl+C) to ensure data integrity.
- **Configurable:** Fully driven by a TOML configuration file for hardware adaptability.

## Prerequisites

- **Rust:** Latest stable toolchain.
- **GStreamer:** Development libraries.
  - Ubuntu/Debian: `sudo apt install libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev gstreamer1.0-plugins-base gstreamer1.0-plugins-good gstreamer1.0-plugins-bad gstreamer1.0-plugins-ugly gstreamer1.0-libav`

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
key = "00112233445566778899aabbccddeeff" # 32-byte hex key
iv = "unused"
output_path = "output.ts.enc"
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

## Architecture

See [DESIGN.md](./DESIGN.md) for details on the system architecture and pipeline design.