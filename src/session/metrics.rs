#[derive(Debug, Default, Clone)]
pub struct SessionMetrics {
    pub bytes_in: u64,
    pub chunks_in: u64,
    pub frames_ok: u64,
    pub frames_err: u64,
    pub stale_view_hits: u64,
    pub pool_rotations: u64,
    pub resync_events: u64,
}

impl SessionMetrics {
    pub fn record_chunk(&mut self, nbytes: usize) {
        self.chunks_in += 1;
        self.bytes_in += nbytes as u64;
    }

    pub fn record_frame_ok(&mut self) {
        self.frames_ok += 1;
    }

    pub fn record_frame_err(&mut self) {
        self.frames_err += 1;
    }

    pub fn record_stale_view(&mut self) {
        self.stale_view_hits += 1;
    }

    pub fn record_pool_rotation(&mut self) {
        self.pool_rotations += 1;
    }

    pub fn record_resync(&mut self) {
        self.resync_events += 1;
    }

    pub fn error_rate(&self) -> f64 {
        let total = self.frames_ok + self.frames_err;
        if total == 0 { 0.0 } else { self.frames_err as f64 / total as f64 }
    }
}

