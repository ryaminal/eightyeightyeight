use crate::config::Config;
use anyhow::{Context, Result};
use gstreamer as gst;
use gstreamer::prelude::*;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tracing::{error, info, warn};

pub fn build_record_pipeline(config: &Config) -> Result<String> {
    let mut parts = Vec::new();

    let source = if config.device == "auto" {
        "autovideosrc".to_string()
    } else if config.device == "videotestsrc" {
        "videotestsrc is-live=true".to_string()
    } else {
        format!("v4l2src device={}", config.device)
    };
    parts.push(source);

    parts.push(format!(
        "video/x-raw,width={},height={},framerate={}",
        config.width, config.height, config.framerate
    ));

    parts.push("videoconvert".to_string());

    if config.cv_enabled {
        check_element_exists("facedetect")?;
        parts.push("videoconvert".to_string());
        parts.push("facedetect".to_string());
        parts.push("videoconvert".to_string());
    }

    parts.push("video/x-raw,format=I420".to_string());
    parts.push("queue".to_string());
    parts.push(format!(
        "x264enc tune=zerolatency speed-preset=ultrafast bitrate={}",
        config.bitrate
    ));
    parts.push("queue".to_string());
    parts.push("h264parse".to_string());

    if config.max_files.is_some() || config.max_file_size_mb.is_some() {
        // Use splitmuxsink for rotation
        let mut location = config.output_path.to_string_lossy().to_string();
        if !location.contains('%')
            && let (Some(stem), Some(ext)) = (
                config.output_path.file_stem(),
                config.output_path.extension(),
            )
        {
            let parent = config
                .output_path
                .parent()
                .unwrap_or_else(|| std::path::Path::new("."));
            location = parent
                .join(format!(
                    "{}_%05d.{}",
                    stem.to_string_lossy(),
                    ext.to_string_lossy()
                ))
                .to_string_lossy()
                .to_string();
        }

        let max_size_bytes = config.max_file_size_mb.unwrap_or(0) * 1_000_000;
        let max_files = config.max_files.unwrap_or(0);

        // We use splitmuxsink with a custom sink bin that includes encryption.
        // splitmuxsink manages the muxer (mpegtsmux) and resets it for each file.
        // The sink property defines where the muxed stream goes.
        // We use rndbuffersize to aggregate 4 TS packets (4 * 188 = 752 bytes).
        // 752 is divisible by 16 (AES block size), ensuring alignment for most buffers.
        // per-buffer-padding=false means only the final buffer (EOS) is padded if needed.
        let sink_str = format!(
            "rndbuffersize min=752 max=752 ! aesenc cipher=aes-256-cbc key={} serialize-iv=true per-buffer-padding=false ! filesink",
            config.key
        );

        parts.push(format!(
            "splitmuxsink location={} muxer=mpegtsmux sink=\"{}\" max-size-bytes={} max-files={} async-finalize=true",
            location,
            sink_str,
            max_size_bytes,
            max_files
        ));
    } else {
        // Standard single file recording
        parts.push("mpegtsmux".to_string());
        parts.push("queue".to_string());
        parts.push("rndbuffersize min=752 max=752".to_string());
        parts.push(format!(
            "aesenc cipher=aes-256-cbc key={} serialize-iv=true per-buffer-padding=false",
            config.key
        ));
        parts.push(format!(
            "filesink location={}",
            config.output_path.to_string_lossy()
        ));
    }

    Ok(parts.join(" ! "))
}

pub fn build_play_pipeline(config: &Config, input_file: &str) -> String {
    let parts = [
        format!("filesrc location={}", input_file),
        format!(
            "aesdec cipher=aes-256-cbc key={} serialize-iv=true per-buffer-padding=false",
            config.key
        ),
        "tsdemux".to_string(),
        "h264parse".to_string(),
        "decodebin".to_string(),
        "autovideosink".to_string(),
    ];
    parts.join(" ! ")
}

pub fn build_stream_pipeline(config: &Config, dest: &str, port: u16) -> Result<String> {
    let mut parts = Vec::new();

    let source = if config.device == "auto" {
        "autovideosrc".to_string()
    } else if config.device == "videotestsrc" {
        "videotestsrc is-live=true".to_string()
    } else {
        format!("v4l2src device={}", config.device)
    };
    parts.push(source);

    parts.push(format!(
        "video/x-raw,width={},height={},framerate={}",
        config.width, config.height, config.framerate
    ));

    parts.push("videoconvert".to_string());

    if config.cv_enabled {
        check_element_exists("facedetect")?;
        parts.push("videoconvert".to_string());
        parts.push("facedetect".to_string());
        parts.push("videoconvert".to_string());
    }

    parts.push("video/x-raw,format=I420".to_string());
    parts.push("queue".to_string());
    parts.push(format!(
        "x264enc tune=zerolatency speed-preset=ultrafast bitrate={}",
        config.bitrate
    ));
    parts.push("rtph264pay config-interval=1 mtu=1400".to_string());
    parts.push("queue".to_string());
    parts.push(format!(
        "aesenc cipher=aes-256-cbc key={} iv={} per-buffer-padding=true serialize-iv=true",
        config.key, config.key
    ));
    parts.push(format!("udpsink host={} port={}", dest, port));

    Ok(parts.join(" ! "))
}

pub fn build_receive_pipeline(config: &Config, listen: &str, port: u16) -> String {
    let parts = [
        format!("udpsrc address={} port={}", listen, port),
        "application/x-rtp,media=(string)video,clock-rate=(int)90000,encoding-name=(string)H264"
            .to_string(),
        format!(
            "aesdec cipher=aes-256-cbc key={} iv={} per-buffer-padding=true serialize-iv=true",
            config.key, config.key
        ),
        "rtph264depay".to_string(),
        "decodebin".to_string(),
        "queue".to_string(),
        "autovideosink sync=false".to_string(),
    ];
    parts.join(" ! ")
}

pub fn run_record_pipeline(config: &Config) -> Result<()> {
    gst::init().context("Failed to initialize GStreamer")?;
    let pipeline_str = build_record_pipeline(config)?;
    info!("Pipeline: {}", pipeline_str);
    let pipeline = gst::parse::launch(&pipeline_str)
        .context("Failed to parse pipeline")?
        .dynamic_cast::<gst::Pipeline>()
        .map_err(|_| anyhow::anyhow!("Element is not a pipeline"))?;

    // Disk space monitor thread
    if let Some(min_space_mb) = config.min_disk_space_mb {
        let pipeline_weak = pipeline.downgrade();
        let output_path = config.output_path.clone();
        thread::spawn(move || {
            while let Some(pipeline) = pipeline_weak.upgrade() {
                if let (Some(_), Ok(free_space)) = (
                    output_path.parent(),
                    fs2::available_space(
                        output_path
                            .parent()
                            .unwrap_or_else(|| std::path::Path::new(".")),
                    ),
                ) {
                    let free_space_mb = free_space / 1_000_000;
                    if free_space_mb < min_space_mb {
                        warn!(
                            "Disk space is low ({} MB remaining), sending EOS to stop pipeline.",
                            free_space_mb
                        );
                        pipeline.send_event(gst::event::Eos::new());
                        break;
                    }
                }
                thread::sleep(Duration::from_secs(10));
            }
        });
    }

    run_pipeline_loop(&pipeline)
}

pub fn run_play_pipeline(config: &Config, input_file: &str) -> Result<()> {
    gst::init().context("Failed to initialize GStreamer")?;
    let pipeline_str = build_play_pipeline(config, input_file);
    info!("Pipeline: {}", pipeline_str);
    let pipeline = gst::parse::launch(&pipeline_str)
        .context("Failed to parse pipeline")?
        .dynamic_cast::<gst::Pipeline>()
        .map_err(|_| anyhow::anyhow!("Element is not a pipeline"))?;
    run_pipeline_loop(&pipeline)
}

pub fn run_stream_pipeline(config: &Config, dest: &str, port: u16) -> Result<()> {
    gst::init().context("Failed to initialize GStreamer")?;
    let pipeline_str = build_stream_pipeline(config, dest, port)?;
    info!("Pipeline: {}", pipeline_str);
    let pipeline = gst::parse::launch(&pipeline_str)
        .context("Failed to parse pipeline")?
        .dynamic_cast::<gst::Pipeline>()
        .map_err(|_| anyhow::anyhow!("Element is not a pipeline"))?;
    run_pipeline_loop(&pipeline)
}

pub fn run_receive_pipeline(config: &Config, listen: &str, port: u16) -> Result<()> {
    gst::init().context("Failed to initialize GStreamer")?;
    let pipeline_str = build_receive_pipeline(config, listen, port);
    info!("Pipeline: {}", pipeline_str);
    let pipeline = gst::parse::launch(&pipeline_str)
        .context("Failed to parse pipeline")?
        .dynamic_cast::<gst::Pipeline>()
        .map_err(|_| anyhow::anyhow!("Element is not a pipeline"))?;
    run_pipeline_loop(&pipeline)
}

// RAII Guard for the pipeline
struct PipelineGuard(gst::Pipeline);

impl Drop for PipelineGuard {
    fn drop(&mut self) {
        info!("Setting pipeline state to NULL");
        let _ = self.0.set_state(gst::State::Null);
    }
}

use crate::metrics::Metrics;

fn check_element_exists(element: &str) -> Result<()> {
    if gst::ElementFactory::find(element).is_none() {
        return Err(anyhow::anyhow!(
            "Missing GStreamer element '{}'. Please install gst-plugins-bad (e.g. sudo apt install gstreamer1.0-plugins-bad).",
            element
        ));
    }
    Ok(())
}

fn run_pipeline_loop(pipeline: &gst::Pipeline) -> Result<()> {
    // Setup Metrics
    let metrics = Arc::new(Metrics::new());
    let mut iter = pipeline.iterate_sinks();
    while let Ok(Some(elem)) = iter.next() {
        if let Some(pad) = elem.static_pad("sink") {
            let metrics = metrics.clone();
            pad.add_probe(gst::PadProbeType::BUFFER, move |_pad, info| {
                if let Some(gst::PadProbeData::Buffer(buffer)) = &info.data {
                    metrics.increment_frames(1);
                    metrics.increment_bytes(buffer.size() as u64);
                    metrics.log_status();
                }
                gst::PadProbeReturn::Ok
            });
        }
    }

    pipeline
        .set_state(gst::State::Playing)
        .context("Failed to set pipeline to playing")?;
    let _guard = PipelineGuard(pipeline.clone());

    let bus = pipeline.bus().context("Pipeline has no bus")?;

    // Handle Ctrl+C
    let pipeline_weak = pipeline.downgrade();
    ctrlc::set_handler(move || {
        info!("Ctrl+C detected, sending EOS...");
        if let Some(pipeline) = pipeline_weak.upgrade() {
            pipeline.send_event(gst::event::Eos::new());
        }
    })
    .context("Error setting Ctrl-C handler")?;

    // Removed manual rotation logic as splitmuxsink handles creation/rotation
    // Note: splitmuxsink handles file creation internally and supports max-files
    // to automatically delete old files. We don't need to manually track/delete.

    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => {
                info!("End of stream received");
                break;
            }
            MessageView::Error(err) => {
                let error_msg = err.error().message().to_string();
                if error_msg.contains("Output window was closed") {
                    info!("Playback stopped by user (Window closed)");
                    break;
                }

                if error_msg.contains("Stream doesn't contain enough data") {
                    info!("Stream stopped with insufficient data (likely interrupted)");
                    break;
                }

                error!(
                    "Error from {:?}: {} ({:?})",
                    msg.src().map(|s| s.path_string()),
                    err.error(),
                    err.debug()
                );
                return Err(anyhow::anyhow!("GStreamer error: {}", err.error()));
            }
            // We can optionally listen for splitmuxsink messages here if needed (e.g. file-opened)
            // splitmuxsink emits element messages like 'splitmuxsink-fragment-opened'
            MessageView::Element(elem_msg) => {
                if let Some(structure) = elem_msg.structure()
                    && structure.name() == "splitmuxsink-fragment-opened"
                    && let Ok(location) = structure.get::<&str>("location")
                {
                    info!("New file created: {}", location);
                }
            }
            _ => (),
        }
    }

    info!("Shutting down pipeline...");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_build_record_pipeline() {
        let config = Config {
            device: "/dev/video4".to_string(),
            width: 640,
            height: 480,
            framerate: "30/1".to_string(),
            bitrate: 1000,
            key: "00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff".to_string(),
            output_path: PathBuf::from("live.ts.enc"),
            cv_enabled: false,
            min_disk_space_mb: None,
            max_files: None,
            max_file_size_mb: None,
        };

        let expected = "v4l2src device=/dev/video4 ! video/x-raw,width=640,height=480,framerate=30/1 ! videoconvert ! video/x-raw,format=I420 ! queue ! x264enc tune=zerolatency speed-preset=ultrafast bitrate=1000 ! queue ! h264parse ! mpegtsmux ! queue ! rndbuffersize min=752 max=752 ! aesenc cipher=aes-256-cbc key=00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff serialize-iv=true per-buffer-padding=false ! filesink location=live.ts.enc";

        let actual = build_record_pipeline(&config).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_build_record_pipeline_with_cv() {
        let config = Config {
            device: "/dev/video4".to_string(),
            width: 640,
            height: 480,
            framerate: "30/1".to_string(),
            bitrate: 1000,
            key: "00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff".to_string(),
            output_path: PathBuf::from("live.ts.enc"),
            cv_enabled: true,
            min_disk_space_mb: None,
            max_files: None,
            max_file_size_mb: None,
        };

        let expected = "v4l2src device=/dev/video4 ! video/x-raw,width=640,height=480,framerate=30/1 ! videoconvert ! videoconvert ! facedetect ! videoconvert ! video/x-raw,format=I420 ! queue ! x264enc tune=zerolatency speed-preset=ultrafast bitrate=1000 ! queue ! h264parse ! mpegtsmux ! queue ! rndbuffersize min=752 max=752 ! aesenc cipher=aes-256-cbc key=00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff serialize-iv=true per-buffer-padding=false ! filesink location=live.ts.enc";

        gst::init().unwrap();

        match build_record_pipeline(&config) {
            Ok(actual) => assert_eq!(actual, expected),
            Err(_) => println!("Skipping test_build_record_pipeline_with_cv: facedetect missing"),
        }
    }

    #[test]
    fn test_build_record_pipeline_with_rotation() {
        let config = Config {
            device: "/dev/video4".to_string(),
            width: 640,
            height: 480,
            framerate: "30/1".to_string(),
            bitrate: 1000,
            key: "00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff".to_string(),
            output_path: PathBuf::from("live.ts.enc"),
            cv_enabled: false,
            min_disk_space_mb: None,
            max_files: Some(10),
            max_file_size_mb: Some(100),
        };

        // This expected string needs to match the splitmuxsink format we constructed above
        let expected = "v4l2src device=/dev/video4 ! video/x-raw,width=640,height=480,framerate=30/1 ! videoconvert ! video/x-raw,format=I420 ! queue ! x264enc tune=zerolatency speed-preset=ultrafast bitrate=1000 ! queue ! h264parse ! splitmuxsink location=live.ts_%05d.enc muxer=mpegtsmux sink=\"rndbuffersize min=752 max=752 ! aesenc cipher=aes-256-cbc key=00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff serialize-iv=true per-buffer-padding=false ! filesink\" max-size-bytes=100000000 max-files=10 async-finalize=true";

        let actual = build_record_pipeline(&config).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_build_play_pipeline() {
        let config = Config {
            device: "/dev/video0".to_string(),
            width: 640,
            height: 480,
            framerate: "30/1".to_string(),
            bitrate: 1000,
            key: "00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff".to_string(),
            output_path: PathBuf::from("unused.enc"),
            cv_enabled: false,
            min_disk_space_mb: None,
            max_files: None,
            max_file_size_mb: None,
        };

        let input_file = "test_video.enc";
        let expected = "filesrc location=test_video.enc ! aesdec cipher=aes-256-cbc key=00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff serialize-iv=true per-buffer-padding=false ! tsdemux ! h264parse ! decodebin ! autovideosink";

        let actual = build_play_pipeline(&config, input_file);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_build_record_pipeline_auto() {
        let config = Config {
            device: "auto".to_string(),
            width: 640,
            height: 480,
            framerate: "30/1".to_string(),
            bitrate: 1000,
            key: "00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff".to_string(),
            output_path: PathBuf::from("live.ts.enc"),
            cv_enabled: false,
            min_disk_space_mb: None,
            max_files: None,
            max_file_size_mb: None,
        };

        let expected = "autovideosrc ! video/x-raw,width=640,height=480,framerate=30/1 ! videoconvert ! video/x-raw,format=I420 ! queue ! x264enc tune=zerolatency speed-preset=ultrafast bitrate=1000 ! queue ! h264parse ! mpegtsmux ! queue ! rndbuffersize min=752 max=752 ! aesenc cipher=aes-256-cbc key=00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff serialize-iv=true per-buffer-padding=false ! filesink location=live.ts.enc";

        let actual = build_record_pipeline(&config).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_build_stream_pipeline() {
        let config = Config {
            device: "/dev/video4".to_string(),
            width: 640,
            height: 480,
            framerate: "30/1".to_string(),
            bitrate: 1000,
            key: "00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff".to_string(),
            output_path: PathBuf::from("unused.enc"),
            cv_enabled: false,
            min_disk_space_mb: None,
            max_files: None,
            max_file_size_mb: None,
        };

        let dest = "127.0.0.1";
        let port = 8088;
        let expected = "v4l2src device=/dev/video4 ! video/x-raw,width=640,height=480,framerate=30/1 ! videoconvert ! video/x-raw,format=I420 ! queue ! x264enc tune=zerolatency speed-preset=ultrafast bitrate=1000 ! rtph264pay config-interval=1 mtu=1400 ! queue ! aesenc cipher=aes-256-cbc key=00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff iv=00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff per-buffer-padding=true serialize-iv=true ! udpsink host=127.0.0.1 port=8088";

        let actual = build_stream_pipeline(&config, dest, port).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_build_receive_pipeline() {
        let config = Config {
            device: "auto".to_string(),
            width: 640,
            height: 480,
            framerate: "30/1".to_string(),
            bitrate: 1000,
            key: "00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff".to_string(),
            output_path: PathBuf::from("unused.enc"),
            cv_enabled: false,
            min_disk_space_mb: None,
            max_files: None,
            max_file_size_mb: None,
        };

        let listen = "0.0.0.0";
        let port = 8088;
        let expected = "udpsrc address=0.0.0.0 port=8088 ! application/x-rtp,media=(string)video,clock-rate=(int)90000,encoding-name=(string)H264 ! aesdec cipher=aes-256-cbc key=00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff iv=00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff per-buffer-padding=true serialize-iv=true ! rtph264depay ! decodebin ! queue ! autovideosink sync=false";

        let actual = build_receive_pipeline(&config, listen, port);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_pipeline_parse() {
        gst::init().unwrap();
        let pipeline_str = "videotestsrc num-buffers=10 ! fakesink";
        let res = gst::parse::launch(pipeline_str);
        assert!(res.is_ok());
    }
}
