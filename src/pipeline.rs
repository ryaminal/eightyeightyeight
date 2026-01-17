use crate::config::Config;
use gstreamer as gst;
use gstreamer::prelude::*;
use anyhow::{Context, Result};
use tracing::{info, error};

pub fn build_record_pipeline(config: &Config) -> String {
    format!(
        "v4l2src device={} \
        ! video/x-raw,width={},height={},framerate={} \
        ! videoconvert \
        ! video/x-raw,format=I420 \
        ! queue \
        ! x264enc tune=zerolatency speed-preset=ultrafast bitrate={} \
        ! queue \
        ! h264parse \
        ! mpegtsmux \
        ! queue \
        ! rndbuffersize min=4096 max=4096 \
        ! aesenc key={} serialize-iv=true per-buffer-padding=false \
        ! filesink location={}",
        config.device,
        config.width,
        config.height,
        config.framerate,
        config.bitrate,
        config.key,
        config.output_path.display()
    )
}

pub fn run_record_pipeline(config: &Config) -> Result<()> {
    gst::init().context("Failed to initialize GStreamer")?;

    let pipeline_str = build_record_pipeline(config);
    info!("Pipeline: {}", pipeline_str);

    let pipeline = gst::parse::launch(&pipeline_str)
        .context("Failed to parse pipeline")?
        .dynamic_cast::<gst::Pipeline>()
        .map_err(|_| anyhow::anyhow!("Element is not a pipeline"))?;

    pipeline
        .set_state(gst::State::Playing)
        .context("Failed to set pipeline to playing")?;

    let bus = pipeline
        .bus()
        .context("Pipeline has no bus")?;

    // Handle Ctrl+C
    let pipeline_weak = pipeline.downgrade();
    ctrlc::set_handler(move || {
        info!("Ctrl+C detected, sending EOS...");
        if let Some(pipeline) = pipeline_weak.upgrade() {
            pipeline.send_event(gst::event::Eos::new());
        }
    })
    .context("Error setting Ctrl-C handler")?;

    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => {
                info!("End of stream received");
                break;
            }
            MessageView::Error(err) => {
                error!(
                    "Error from {:?}: {} ({:?})",
                    msg.src().map(|s| s.path_string()),
                    err.error(),
                    err.debug()
                );
                return Err(anyhow::anyhow!("GStreamer error: {}", err.error()));
            }
            _ => (),
        }
    }

    info!("Shutting down pipeline...");
    pipeline
        .set_state(gst::State::Null)
        .context("Failed to set pipeline to null")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_pipeline_parse() {
        gst::init().unwrap();
        let pipeline_str = "videotestsrc num-buffers=10 ! fakesink";
        let res = gst::parse::launch(pipeline_str);
        assert!(res.is_ok());
    }

    #[test]
    fn test_build_record_pipeline() {
        // ... (previous test content)
        let config = Config {
            device: "/dev/video4".to_string(),
            width: 640,
            height: 480,
            framerate: "30/1".to_string(),
            bitrate: 1000,
            key: "00112233445566778899aabbccddeeff".to_string(),
            iv: "unused".to_string(),
            output_path: PathBuf::from("live.ts.enc"),
        };

        let expected = "v4l2src device=/dev/video4 \
            ! video/x-raw,width=640,height=480,framerate=30/1 \
            ! videoconvert \
            ! video/x-raw,format=I420 \
            ! queue \
            ! x264enc tune=zerolatency speed-preset=ultrafast bitrate=1000 \
            ! queue \
            ! h264parse \
            ! mpegtsmux \
            ! queue \
            ! rndbuffersize min=4096 max=4096 \
            ! aesenc key=00112233445566778899aabbccddeeff serialize-iv=true per-buffer-padding=false \
            ! filesink location=live.ts.enc";

        let actual = build_record_pipeline(&config);
        assert_eq!(actual, expected);
    }
}
