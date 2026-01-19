# Specification: Switch to SplitMuxSink

## 1. Loop Recording
- **Goal:** Enable "Dashcam" style recording where old files are automatically deleted to make space for new ones.
- **Mechanism:** Use GStreamer's `splitmuxsink` element.
- **Configuration:**
  - `max_files`: Maximum number of files to keep (e.g., 5). 0 = unlimited.
  - `max_file_size_mb`: Maximum size of each file in MB. 0 = unlimited.

## 2. Pipeline Changes
- **Current:** `mpegtsmux ! aesenc ! filesink`
- **New:** `splitmuxsink muxer=mpegtsmux sink="aesenc ! filesink" max-size-bytes=... max-files=...`

## 3. Filename Formatting
- **Requirement:** The `output_path` in config must be treated as a pattern or a base name.
- **Implementation:** If `output_path` is `capture.ts`, `splitmuxsink` requires a pattern like `capture_%05d.ts`. We should automatically inject the pattern if missing, or require the user to provide it.
- **Decision:** Automatically append `_%05d.ts` if the extension is `.ts` or `.enc`. Or just require the user to specify a pattern. 
- **Better approach:** Allow the user to specify `output_path` as a pattern. If it doesn't contain `%`, warn or append.

## 4. Playback
- **Requirement:** Playback needs to handle split files.
- **Challenge:** `filesrc` only reads one file.
- **Solution:** For now, playback command will play a *single* file specified by the user. If they want to play the whole sequence, they can use `splitmuxsrc` or just play files sequentially. The `play` command currently takes an `input` argument. This remains valid.
