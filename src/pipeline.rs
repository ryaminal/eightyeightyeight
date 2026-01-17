use crate::config::Config;

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
