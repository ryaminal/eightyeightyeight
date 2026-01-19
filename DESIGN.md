# Design Document: Eightyeightyeight

## 1. System Overview

`eightyeightyeight` is a Rust-based CLI application designed for secure video capture and playback. It leverages the GStreamer framework for media processing, ensuring high performance and reliability.

GStreamer is an excellent framework, the rust bindings are first-class, and I find the pipeline concept of GStreamer to be intuitive and robust.

Rust was selected because of the GSTreamer bindings as well as a strongly-typed language that GenAI, and humans, can leverage to make safe code that compiles down to a machine binary.

### Key Components

- **CLI (`src/cli.rs`, `src/main.rs`):** Handles user input, command dispatch (`record`, `play`), and configuration loading.
- **Configuration (`src/config.rs`):** manages system settings via TOML files, including device paths, resolution, and encryption keys. Can be created with the convenient wizard which scans for available linux cameras.
- **Pipeline Engine (`src/pipeline.rs`):** Construct and executes GStreamer pipelines for recording and playback. It abstracts the complexity of GStreamer behind a clean Rust interface.

## 2. Architecture

### 2.1 Record Pipeline

The recording pipeline is designed for low latency and data security. It captures raw video, encodes it to H.264, encrypts it on the fly using AES, and writes it to an MPEG-TS container.

We can, of course, change this to encrypt outside of the GSTreamer pipeline, but I chose to go this path because it was challenging and has some perks like the disk never sees unencrypted video.

**Flow:**
`Source (V4L2/Auto)` -> `Video Convert` -> `H.264 Encode` -> `MPEG-TS Mux` -> `AES Encrypt` -> `File Sink`

**Key Elements:**

- `v4l2src` / `autovideosrc`: Video capture source.
- `x264enc`: H.264 encoding tuned for zero latency (`tune=zerolatency`, `speed-preset=ultrafast`).
- `aesenc`: AES encryption with serialized IVs (`serialize-iv=true`) to ensure data is readable even if the stream is interrupted.
- `filesink`: Writes the encrypted stream to disk.
- `splitmuxsink`: Used when file rotation (`max_files`) is configured. It manages segment creation and rotation automatically.

### 2.2 Playback Pipeline

The playback pipeline reverses the recording process. It reads the encrypted file, decrypts it, parses the stream, and renders it to the display.

**Flow:**
`File Source` -> `AES Decrypt` -> `TS Demux` -> `H.264 Parse` -> `Decode` -> `Video Sink`

**Key Elements:**

- `filesrc`: Reads the encrypted file.
- `aesdec`: Decrypts the stream using the configured Key and IV. must match the `aesenc` settings (`serialize-iv=true`).
- `tsdemux`: Demultiplexes the MPEG-TS container.
- `autovideosink`: Automatically selects the best video sink for the platform (e.g., `xvimagesink`, `waylandsink`).

### 2.3 Streaming Pipeline

The streaming pipeline allows real-time secure transmission over UDP using RTP.

**Flow:**
`Source` -> `Encode` -> `RTP Pay` -> `AES Encrypt` -> `UDP Sink`

**Key Elements:**

- `rtph264pay`: Payloads H.264 into RTP packets.
- `udpsink`: Sends packets to a destination IP/Port.

### 2.4 Computer Vision Pipeline

When `cv_enabled` is true, a face detection element is injected into the pipeline before encoding.

**Flow:**
`Source` -> `Convert` -> `FaceDetect` -> `Convert` -> `Encode` ...

**Key Elements:**

- `facedetect`: GStreamer OpenCV element (requires `gst-plugins-bad`) that draws rectangles around detected faces.

## 3. Configuration

Configuration is managed via a TOML file. This decoupling allows the binary to be portable across different hardware setups without recompilation.

**Interactive Wizard:**
An interactive TUI (`eightyeightyeight init`) is provided to simplify configuration generation. It discovers available hardware and auto-generates secure keys.

**Example `config.toml`:**

```toml
device = "/dev/video0"
width = 640
height = 480
framerate = "30/1"
bitrate = 1000
# Key Resolution Strategies:
# - literal:HEX_STRING (default)
# - env:VAR_NAME
# - file:PATH_TO_KEY_FILE
# - vault:VAULT_PATH (stub)
key = "literal:001122...eeff"
output_path = "output_%05d.ts.enc"
max_files = 10          # File rotation
max_file_size_mb = 100
min_disk_space_mb = 500 # Safety stop
cv_enabled = true       # Face detection
```

## 4. Security

- **Encryption:** AES-256-CBC is used to encrypt the video stream.
- **Key Management:** The system supports extensible key resolution strategies (`secrets.rs`):
  - **Literal:** For testing/dev.
  - **Environment Variable:** For containerized deployments.
  - **File:** For mounting secrets (e.g., Kubernetes Secrets).
  - **Pluggable**: Can easily add other secret resolution by implementing a simple trait.
- **Data Integrity:** The use of `mpegtsmux` and serialized IVs ensures that valid video data can be recovered up to the point of interruption (e.g., power loss).

## 5. Observability

- **Metrics:** A Prometheus exporter is available on port 9091 (if configured/enabled in code) exposing pipeline statistics:
  - `frame_count`: Total processed frames.
  - `byte_count`: Total processed bytes.
- **Logs:** Structured logging via `tracing` provides detailed runtime information.
- **OTEL**: leverages the power and ease of OpenTelemetry to easily ship this data, and more, to any supported backend.

## 6. Reliability

- **Graceful Shutdown:** The application handles `SIGINT` (Ctrl+C) by sending an End-of-Stream (EOS) event to the pipeline. This ensures that the file containers are properly closed and headers are written before the application exits.
- **Error Handling:** GStreamer bus messages are monitored for errors, which are logged via `tracing`.
- **Operational Integration:** Support for systemd service supervision and disk space monitoring.

## 7. Architectural Trade-offs

### AES-CBC vs. AES-GCM
**Decision:** Used AES-256-CBC.
**Reasoning:** While GCM provides authenticated encryption (integrity + privacy), it adds significant complexity to the GStreamer pipeline regarding tag handling and container formats. CBC was chosen for broad compatibility with standard MPEG-TS demuxers and lower overhead on embedded CPUs, with the trade-off of lacking intrinsic integrity checks (though the MPEG-TS container structure provides some resilience).

### TOML vs. Environment Variables
**Decision:** Used a dedicated `config.toml` file.
**Reasoning:** While environment variables are standard for cloud-native apps (12-factor), embedded devices often need persistent, complex configurations (like specific resolution/framerate tuples) that are clumsy to encode in strings. TOML provides a balance of structure and readability.

### Hybrid Async/Sync Runtime
**Decision:** Used `tokio` for the CLI and Wizard, but standard GStreamer threading for the pipeline.
**Reasoning:** GStreamer manages its own internal threads. Wrapping the pipeline strictly in `async`/`await` would introduce unnecessary abstraction layers. The hybrid approach allows the control plane (UI/Signals) to be responsive while the data plane runs at native speed.
