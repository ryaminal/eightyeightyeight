# Design Document: Eightyeightyeight

## 1. System Overview

`eightyeightyeight` is a robust, Rust-based CLI application designed for secure video capture and playback in embedded Linux environments. It leverages the GStreamer framework for media processing, ensuring high performance and reliability.

### Key Components
- **CLI (`src/cli.rs`, `src/main.rs`):** Handles user input, command dispatch (`record`, `play`), and configuration loading.
- **Configuration (`src/config.rs`):** manages system settings via TOML files, including device paths, resolution, and encryption keys.
- **Pipeline Engine (`src/pipeline.rs`):** Construct and executes GStreamer pipelines for recording and playback. It abstracts the complexity of GStreamer behind a clean Rust interface.

## 2. Architecture

### 2.1 Record Pipeline
The recording pipeline is designed for low latency and data security. It captures raw video, encodes it to H.264, encrypts it on the fly using AES, and writes it to an MPEG-TS container.

**Flow:**
`Source (V4L2/Auto)` -> `Video Convert` -> `H.264 Encode` -> `MPEG-TS Mux` -> `AES Encrypt` -> `File Sink`

**Key Elements:**
- `v4l2src` / `autovideosrc`: Video capture source.
- `x264enc`: H.264 encoding tuned for zero latency (`tune=zerolatency`, `speed-preset=ultrafast`).
- `aesenc`: AES encryption with serialized IVs (`serialize-iv=true`) to ensure data is readable even if the stream is interrupted.
- `filesink`: Writes the encrypted stream to disk.

### 2.2 Playback Pipeline
The playback pipeline reverses the recording process. It reads the encrypted file, decrypts it, parses the stream, and renders it to the display.

**Flow:**
`File Source` -> `AES Decrypt` -> `TS Demux` -> `H.264 Parse` -> `Decode` -> `Video Sink`

**Key Elements:**
- `filesrc`: Reads the encrypted file.
- `aesdec`: Decrypts the stream using the configured Key and IV. must match the `aesenc` settings (`serialize-iv=true`).
- `tsdemux`: Demultiplexes the MPEG-TS container.
- `autovideosink`: Automatically selects the best video sink for the platform (e.g., `xvimagesink`, `waylandsink`).

## 3. Configuration
Configuration is managed via a TOML file. This decoupling allows the binary to be portable across different hardware setups without recompilation.

**Example `config.toml`:**
```toml
device = "/dev/video0"      # or "auto"
width = 640
height = 480
framerate = "30/1"
bitrate = 1000              # Bitrate in Kbps
key = "001122...eeff"       # 32-byte hex string (AES-256)
iv = "unused"               # IV is serialized in the stream
output_path = "output.ts.enc"
```

## 4. Security
- **Encryption:** AES (Advanced Encryption Standard) is used to encrypt the video stream.
- **Key Management:** Keys are currently provided via the configuration file. In a production environment, integration with a secure key store (e.g., TPM, HSM) would be recommended.
- **Data Integrity:** The use of `mpegtsmux` and serialized IVs ensures that valid video data can be recovered up to the point of interruption (e.g., power loss).

## 5. Reliability
- **Graceful Shutdown:** The application handles `SIGINT` (Ctrl+C) by sending an End-of-Stream (EOS) event to the pipeline. This ensures that the file containers are properly closed and headers are written before the application exits.
- **Error Handling:** GStreamer bus messages are monitored for errors, which are logged via `tracing`.
