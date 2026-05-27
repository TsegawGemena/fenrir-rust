#[derive(Debug, Default, Clone)]
pub struct StreamStats {
    pub bytes_received: u64,
    pub frames_decoded: u64,
    pub frames_encoded: u64,
    pub need_more: u64,
    pub backpressure_hits: u64,
    pub errors: u64,
}

impl StreamStats {
    pub fn throughput_frames_per_kb(&self) -> f64 {
        if self.bytes_received == 0 { return 0.0; }
        (self.frames_decoded as f64) / (self.bytes_received as f64 / 1024.0)
    }

    pub fn merge(&mut self, other: &StreamStats) {
        self.bytes_received += other.bytes_received;
        self.frames_decoded += other.frames_decoded;
        self.frames_encoded += other.frames_encoded;
        self.need_more += other.need_more;
        self.backpressure_hits += other.backpressure_hits;
        self.errors += other.errors;
    }
}
