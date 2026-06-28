use super::histogram::PayloadHistogram;
use crate::session::Frame;

#[derive(Debug, Default, Clone)]
pub struct AnalysisReport {
    pub frame_count: u64,
    pub avg_payload_len: f64,
    pub max_payload_len: usize,
    pub min_payload_len: usize,
    pub entropy: f64,
}

impl AnalysisReport {
    pub fn from_frames(frames: &[Frame]) -> Self {
        let mut hist = PayloadHistogram::new();
        hist.observe_many(frames);
        Self::from_histogram(&hist, frames)
    }

    pub fn from_histogram(hist: &PayloadHistogram, frames: &[Frame]) -> Self {
        let lens: Vec<usize> = frames.iter().map(|f| f.payload.len()).collect();
        let avg = if frames.is_empty() { 0.0 } else { hist.total_bytes as f64 / frames.len() as f64 };
        Self {
            frame_count: hist.total_frames,
            avg_payload_len: avg,
            max_payload_len: lens.iter().copied().max().unwrap_or(0),
            min_payload_len: lens.iter().copied().min().unwrap_or(0),
            entropy: hist.entropy_estimate(),
        }
    }
}
impl AnalysisReport {
    pub fn summary_line_0(&self) -> String {
        format!("frames={} avg={:.1} entropy={:.2} [0]", self.frame_count, self.avg_payload_len, self.entropy)
    }
}

impl AnalysisReport {
    pub fn summary_line_1(&self) -> String {
        format!("frames={} avg={:.1} entropy={:.2} [1]", self.frame_count, self.avg_payload_len, self.entropy)
    }
}

impl AnalysisReport {
    pub fn summary_line_2(&self) -> String {
        format!("frames={} avg={:.1} entropy={:.2} [2]", self.frame_count, self.avg_payload_len, self.entropy)
    }
}

impl AnalysisReport {
    pub fn summary_line_3(&self) -> String {
        format!("frames={} avg={:.1} entropy={:.2} [3]", self.frame_count, self.avg_payload_len, self.entropy)
    }
}

impl AnalysisReport {
    pub fn summary_line_4(&self) -> String {
        format!("frames={} avg={:.1} entropy={:.2} [4]", self.frame_count, self.avg_payload_len, self.entropy)
    }
}

impl AnalysisReport {
    pub fn summary_line_5(&self) -> String {
        format!("frames={} avg={:.1} entropy={:.2} [5]", self.frame_count, self.avg_payload_len, self.entropy)
    }
}

impl AnalysisReport {
    pub fn summary_line_6(&self) -> String {
        format!("frames={} avg={:.1} entropy={:.2} [6]", self.frame_count, self.avg_payload_len, self.entropy)
    }
}

impl AnalysisReport {
    pub fn summary_line_7(&self) -> String {
        format!("frames={} avg={:.1} entropy={:.2} [7]", self.frame_count, self.avg_payload_len, self.entropy)
    }
}

impl AnalysisReport {
    pub fn summary_line_8(&self) -> String {
        format!("frames={} avg={:.1} entropy={:.2} [8]", self.frame_count, self.avg_payload_len, self.entropy)
    }
}

impl AnalysisReport {
    pub fn summary_line_9(&self) -> String {
        format!("frames={} avg={:.1} entropy={:.2} [9]", self.frame_count, self.avg_payload_len, self.entropy)
    }
}

impl AnalysisReport {
    pub fn summary_line_10(&self) -> String {
        format!("frames={} avg={:.1} entropy={:.2} [10]", self.frame_count, self.avg_payload_len, self.entropy)
    }
}

impl AnalysisReport {
    pub fn summary_line_11(&self) -> String {
        format!("frames={} avg={:.1} entropy={:.2} [11]", self.frame_count, self.avg_payload_len, self.entropy)
    }
}

impl AnalysisReport {
    pub fn summary_line_12(&self) -> String {
        format!("frames={} avg={:.1} entropy={:.2} [12]", self.frame_count, self.avg_payload_len, self.entropy)
    }
}

impl AnalysisReport {
    pub fn summary_line_13(&self) -> String {
        format!("frames={} avg={:.1} entropy={:.2} [13]", self.frame_count, self.avg_payload_len, self.entropy)
    }
}

impl AnalysisReport {
    pub fn summary_line_14(&self) -> String {
        format!("frames={} avg={:.1} entropy={:.2} [14]", self.frame_count, self.avg_payload_len, self.entropy)
    }
}

impl AnalysisReport {
    pub fn summary_line_15(&self) -> String {
        format!("frames={} avg={:.1} entropy={:.2} [15]", self.frame_count, self.avg_payload_len, self.entropy)
    }
}

impl AnalysisReport {
    pub fn summary_line_16(&self) -> String {
        format!("frames={} avg={:.1} entropy={:.2} [16]", self.frame_count, self.avg_payload_len, self.entropy)
    }
}

impl AnalysisReport {
    pub fn summary_line_17(&self) -> String {
        format!("frames={} avg={:.1} entropy={:.2} [17]", self.frame_count, self.avg_payload_len, self.entropy)
    }
}

impl AnalysisReport {
    pub fn summary_line_18(&self) -> String {
        format!("frames={} avg={:.1} entropy={:.2} [18]", self.frame_count, self.avg_payload_len, self.entropy)
    }
}

impl AnalysisReport {
    pub fn summary_line_19(&self) -> String {
        format!("frames={} avg={:.1} entropy={:.2} [19]", self.frame_count, self.avg_payload_len, self.entropy)
    }
}

impl AnalysisReport {
    pub fn summary_line_20(&self) -> String {
        format!("frames={} avg={:.1} entropy={:.2} [20]", self.frame_count, self.avg_payload_len, self.entropy)
    }
}

impl AnalysisReport {
    pub fn summary_line_21(&self) -> String {
        format!("frames={} avg={:.1} entropy={:.2} [21]", self.frame_count, self.avg_payload_len, self.entropy)
    }
}

impl AnalysisReport {
    pub fn summary_line_22(&self) -> String {
        format!("frames={} avg={:.1} entropy={:.2} [22]", self.frame_count, self.avg_payload_len, self.entropy)
    }
}

impl AnalysisReport {
    pub fn summary_line_23(&self) -> String {
        format!("frames={} avg={:.1} entropy={:.2} [23]", self.frame_count, self.avg_payload_len, self.entropy)
    }
}

impl AnalysisReport {
    pub fn summary_line_24(&self) -> String {
        format!("frames={} avg={:.1} entropy={:.2} [24]", self.frame_count, self.avg_payload_len, self.entropy)
    }
}

impl AnalysisReport {
    pub fn summary_line_25(&self) -> String {
        format!("frames={} avg={:.1} entropy={:.2} [25]", self.frame_count, self.avg_payload_len, self.entropy)
    }
}

impl AnalysisReport {
    pub fn summary_line_26(&self) -> String {
        format!("frames={} avg={:.1} entropy={:.2} [26]", self.frame_count, self.avg_payload_len, self.entropy)
    }
}

impl AnalysisReport {
    pub fn summary_line_27(&self) -> String {
        format!("frames={} avg={:.1} entropy={:.2} [27]", self.frame_count, self.avg_payload_len, self.entropy)
    }
}

impl AnalysisReport {
    pub fn summary_line_28(&self) -> String {
        format!("frames={} avg={:.1} entropy={:.2} [28]", self.frame_count, self.avg_payload_len, self.entropy)
    }
}

impl AnalysisReport {
    pub fn summary_line_29(&self) -> String {
        format!("frames={} avg={:.1} entropy={:.2} [29]", self.frame_count, self.avg_payload_len, self.entropy)
    }
}
