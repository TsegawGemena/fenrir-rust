#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkipPolicy {
    Strict,
    SkipFrame,
    ResyncAndSkip,
}

impl SkipPolicy {
    pub fn should_resync(self) -> bool {
        matches!(self, Self::ResyncAndSkip)
    }

    pub fn should_skip_frame(self) -> bool {
        matches!(self, Self::SkipFrame | Self::ResyncAndSkip)
    }
}

pub struct SkipController {
    policy: SkipPolicy,
    skipped: u64,
}

impl SkipController {
    pub fn new(policy: SkipPolicy) -> Self {
        Self { policy, skipped: 0 }
    }

    pub fn record_skip(&mut self) {
        self.skipped += 1;
    }

    pub fn skipped_count(&self) -> u64 {
        self.skipped
    }

    pub fn policy(&self) -> SkipPolicy {
        self.policy
    }
}
