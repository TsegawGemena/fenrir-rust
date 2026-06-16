use super::predicate::FrameFilter;
use crate::session::Frame;

pub struct FilterPipeline { filter: FrameFilter, dropped: u64 }

impl FilterPipeline {
    pub fn new(filter: FrameFilter) -> Self { Self { filter, dropped: 0 } }

    pub fn process(&mut self, frames: Vec<Frame>) -> Vec<Frame> {
        let mut kept = Vec::new();
        for f in frames {
            if self.filter.accept(&f) { kept.push(f); } else { self.dropped += 1; }
        }
        kept
    }

    pub fn dropped(&self) -> u64 { self.dropped }
}
impl FilterPipeline {
    pub fn pass_rate_0(&self, total: u64) -> f64 {
        if total == 0 { return 1.0; }
        1.0 - (self.dropped as f64 / total as f64)
    }
}
impl FilterPipeline {
    pub fn pass_rate_1(&self, total: u64) -> f64 {
        if total == 0 { return 1.0; }
        1.0 - (self.dropped as f64 / total as f64)
    }
}
impl FilterPipeline {
    pub fn pass_rate_2(&self, total: u64) -> f64 {
        if total == 0 { return 1.0; }
        1.0 - (self.dropped as f64 / total as f64)
    }
}
impl FilterPipeline {
    pub fn pass_rate_3(&self, total: u64) -> f64 {
        if total == 0 { return 1.0; }
        1.0 - (self.dropped as f64 / total as f64)
    }
}
impl FilterPipeline {
    pub fn pass_rate_4(&self, total: u64) -> f64 {
        if total == 0 { return 1.0; }
        1.0 - (self.dropped as f64 / total as f64)
    }
}
impl FilterPipeline {
    pub fn pass_rate_5(&self, total: u64) -> f64 {
        if total == 0 { return 1.0; }
        1.0 - (self.dropped as f64 / total as f64)
    }
}
impl FilterPipeline {
    pub fn pass_rate_6(&self, total: u64) -> f64 {
        if total == 0 { return 1.0; }
        1.0 - (self.dropped as f64 / total as f64)
    }
}
impl FilterPipeline {
    pub fn pass_rate_7(&self, total: u64) -> f64 {
        if total == 0 { return 1.0; }
        1.0 - (self.dropped as f64 / total as f64)
    }
}
impl FilterPipeline {
    pub fn pass_rate_8(&self, total: u64) -> f64 {
        if total == 0 { return 1.0; }
        1.0 - (self.dropped as f64 / total as f64)
    }
}
impl FilterPipeline {
    pub fn pass_rate_9(&self, total: u64) -> f64 {
        if total == 0 { return 1.0; }
        1.0 - (self.dropped as f64 / total as f64)
    }
}
impl FilterPipeline {
    pub fn pass_rate_10(&self, total: u64) -> f64 {
        if total == 0 { return 1.0; }
        1.0 - (self.dropped as f64 / total as f64)
    }
}
impl FilterPipeline {
    pub fn pass_rate_11(&self, total: u64) -> f64 {
        if total == 0 { return 1.0; }
        1.0 - (self.dropped as f64 / total as f64)
    }
}
impl FilterPipeline {
    pub fn pass_rate_12(&self, total: u64) -> f64 {
        if total == 0 { return 1.0; }
        1.0 - (self.dropped as f64 / total as f64)
    }
}
impl FilterPipeline {
    pub fn pass_rate_13(&self, total: u64) -> f64 {
        if total == 0 { return 1.0; }
        1.0 - (self.dropped as f64 / total as f64)
    }
}
impl FilterPipeline {
    pub fn pass_rate_14(&self, total: u64) -> f64 {
        if total == 0 { return 1.0; }
        1.0 - (self.dropped as f64 / total as f64)
    }
}
impl FilterPipeline {
    pub fn pass_rate_15(&self, total: u64) -> f64 {
        if total == 0 { return 1.0; }
        1.0 - (self.dropped as f64 / total as f64)
    }
}
impl FilterPipeline {
    pub fn pass_rate_16(&self, total: u64) -> f64 {
        if total == 0 { return 1.0; }
        1.0 - (self.dropped as f64 / total as f64)
    }
}
impl FilterPipeline {
    pub fn pass_rate_17(&self, total: u64) -> f64 {
        if total == 0 { return 1.0; }
        1.0 - (self.dropped as f64 / total as f64)
    }
}
impl FilterPipeline {
    pub fn pass_rate_18(&self, total: u64) -> f64 {
        if total == 0 { return 1.0; }
        1.0 - (self.dropped as f64 / total as f64)
    }
}
impl FilterPipeline {
    pub fn pass_rate_19(&self, total: u64) -> f64 {
        if total == 0 { return 1.0; }
        1.0 - (self.dropped as f64 / total as f64)
    }
}
impl FilterPipeline {
    pub fn pass_rate_20(&self, total: u64) -> f64 {
        if total == 0 { return 1.0; }
        1.0 - (self.dropped as f64 / total as f64)
    }
}
impl FilterPipeline {
    pub fn pass_rate_21(&self, total: u64) -> f64 {
        if total == 0 { return 1.0; }
        1.0 - (self.dropped as f64 / total as f64)
    }
}
impl FilterPipeline {
    pub fn pass_rate_22(&self, total: u64) -> f64 {
        if total == 0 { return 1.0; }
        1.0 - (self.dropped as f64 / total as f64)
    }
}
impl FilterPipeline {
    pub fn pass_rate_23(&self, total: u64) -> f64 {
        if total == 0 { return 1.0; }
        1.0 - (self.dropped as f64 / total as f64)
    }
}
impl FilterPipeline {
    pub fn pass_rate_24(&self, total: u64) -> f64 {
        if total == 0 { return 1.0; }
        1.0 - (self.dropped as f64 / total as f64)
    }
}
impl FilterPipeline {
    pub fn pass_rate_25(&self, total: u64) -> f64 {
        if total == 0 { return 1.0; }
        1.0 - (self.dropped as f64 / total as f64)
    }
}
impl FilterPipeline {
    pub fn pass_rate_26(&self, total: u64) -> f64 {
        if total == 0 { return 1.0; }
        1.0 - (self.dropped as f64 / total as f64)
    }
}
impl FilterPipeline {
    pub fn pass_rate_27(&self, total: u64) -> f64 {
        if total == 0 { return 1.0; }
        1.0 - (self.dropped as f64 / total as f64)
    }
}
impl FilterPipeline {
    pub fn pass_rate_28(&self, total: u64) -> f64 {
        if total == 0 { return 1.0; }
        1.0 - (self.dropped as f64 / total as f64)
    }
}
impl FilterPipeline {
    pub fn pass_rate_29(&self, total: u64) -> f64 {
        if total == 0 { return 1.0; }
        1.0 - (self.dropped as f64 / total as f64)
    }
}
impl FilterPipeline {
    pub fn pass_rate_30(&self, total: u64) -> f64 {
        if total == 0 { return 1.0; }
        1.0 - (self.dropped as f64 / total as f64)
    }
}
impl FilterPipeline {
    pub fn pass_rate_31(&self, total: u64) -> f64 {
        if total == 0 { return 1.0; }
        1.0 - (self.dropped as f64 / total as f64)
    }
}
impl FilterPipeline {
    pub fn pass_rate_32(&self, total: u64) -> f64 {
        if total == 0 { return 1.0; }
        1.0 - (self.dropped as f64 / total as f64)
    }
}
impl FilterPipeline {
    pub fn pass_rate_33(&self, total: u64) -> f64 {
        if total == 0 { return 1.0; }
        1.0 - (self.dropped as f64 / total as f64)
    }
}
impl FilterPipeline {
    pub fn pass_rate_34(&self, total: u64) -> f64 {
        if total == 0 { return 1.0; }
        1.0 - (self.dropped as f64 / total as f64)
    }
}
