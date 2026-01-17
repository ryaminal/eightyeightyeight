use opentelemetry::{
    global,
    metrics::{Counter, Meter},
};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;
use tracing::info;

pub struct Metrics {
    pub frame_counter: Counter<u64>,
    pub byte_counter: Counter<u64>,
    pub start_time: Instant,
    pub last_report_time: std::sync::Mutex<Instant>,
    pub frame_count_total: AtomicU64,
    pub byte_count_total: AtomicU64,
}

impl Metrics {
    pub fn new() -> Self {
        let meter: Meter = global::meter("eightyeightyeight");

        let frame_counter = meter
            .u64_counter("video_frames_processed")
            .with_description("Number of video frames processed")
            .build();

        let byte_counter = meter
            .u64_counter("video_bytes_processed")
            .with_description("Number of bytes processed")
            .build();

        Metrics {
            frame_counter,
            byte_counter,
            start_time: Instant::now(),
            last_report_time: std::sync::Mutex::new(Instant::now()),
            frame_count_total: AtomicU64::new(0),
            byte_count_total: AtomicU64::new(0),
        }
    }

    pub fn increment_frames(&self, count: u64) {
        self.frame_counter.add(count, &[]);
        self.frame_count_total.fetch_add(count, Ordering::Relaxed);
    }

    pub fn increment_bytes(&self, count: u64) {
        self.byte_counter.add(count, &[]);
        self.byte_count_total.fetch_add(count, Ordering::Relaxed);
    }

    pub fn log_status(&self) {
        let now = Instant::now();
        let mut last_report = self.last_report_time.lock().unwrap();

        if now.duration_since(*last_report).as_secs() >= 5 {
            let total_frames = self.frame_count_total.load(Ordering::Relaxed);
            let total_bytes = self.byte_count_total.load(Ordering::Relaxed);
            let duration = now.duration_since(self.start_time).as_secs_f64();

            let fps = total_frames as f64 / duration;
            let bitrate_kbps = (total_bytes as f64 * 8.0) / duration / 1000.0;

            info!("Metrics: FPS={:.2}, Bitrate={:.2} kbps", fps, bitrate_kbps);
            *last_report = now;
        }
    }
}
