use crate::wire::FrameLimits;

#[derive(Debug, Clone)]
pub struct SessionConfig {
    pub pool_capacity: usize,
    pub limits: FrameLimits,
    pub max_frames_per_feed: usize,
    pub enable_metrics: bool,
    pub resync_window: usize,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            pool_capacity: 4096,
            limits: FrameLimits::default(),
            max_frames_per_feed: 64,
            enable_metrics: true,
            resync_window: 8192,
        }
    }
}

impl SessionConfig {
    pub fn with_pool_capacity(mut self, cap: usize) -> Self {
        self.pool_capacity = cap;
        self
    }

    pub fn with_limits(mut self, limits: FrameLimits) -> Self {
        self.limits = limits;
        self
    }

    pub fn tight_pool(cap: usize) -> Self {
        Self::default().with_pool_capacity(cap)
    }
}

