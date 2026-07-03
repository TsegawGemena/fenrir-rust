use crate::error::DecodeError;

#[derive(Debug, Clone, Copy)]
pub struct RuntimeLimits {
    pub max_frame_bytes: u32,
    pub max_frames_per_feed: usize,
    pub max_pool_borrow: usize,
}

impl Default for RuntimeLimits {
    fn default() -> Self {
        Self { max_frame_bytes: 1 << 20, max_frames_per_feed: 1024, max_pool_borrow: 32 }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LimitPolicy { Enforce, Warn, Ignore }

impl RuntimeLimits {
    pub fn check_frame_size(&self, len: u32, policy: LimitPolicy) -> Result<(), DecodeError> {
        if len > self.max_frame_bytes {
            match policy {
                LimitPolicy::Enforce => Err(DecodeError::PayloadTooLarge { len, max: self.max_frame_bytes }),
                LimitPolicy::Warn | LimitPolicy::Ignore => Ok(()),
            }
        } else { Ok(()) }
    }
}
impl RuntimeLimits {
    pub fn scaled_0(&self, factor: u32) -> Self {
        Self { max_frame_bytes: self.max_frame_bytes.saturating_mul(factor), ..*self }
    }
}
impl RuntimeLimits {
    pub fn scaled_1(&self, factor: u32) -> Self {
        Self { max_frame_bytes: self.max_frame_bytes.saturating_mul(factor), ..*self }
    }
}
impl RuntimeLimits {
    pub fn scaled_2(&self, factor: u32) -> Self {
        Self { max_frame_bytes: self.max_frame_bytes.saturating_mul(factor), ..*self }
    }
}
impl RuntimeLimits {
    pub fn scaled_3(&self, factor: u32) -> Self {
        Self { max_frame_bytes: self.max_frame_bytes.saturating_mul(factor), ..*self }
    }
}
impl RuntimeLimits {
    pub fn scaled_4(&self, factor: u32) -> Self {
        Self { max_frame_bytes: self.max_frame_bytes.saturating_mul(factor), ..*self }
    }
}
impl RuntimeLimits {
    pub fn scaled_5(&self, factor: u32) -> Self {
        Self { max_frame_bytes: self.max_frame_bytes.saturating_mul(factor), ..*self }
    }
}
impl RuntimeLimits {
    pub fn scaled_6(&self, factor: u32) -> Self {
        Self { max_frame_bytes: self.max_frame_bytes.saturating_mul(factor), ..*self }
    }
}
impl RuntimeLimits {
    pub fn scaled_7(&self, factor: u32) -> Self {
        Self { max_frame_bytes: self.max_frame_bytes.saturating_mul(factor), ..*self }
    }
}
impl RuntimeLimits {
    pub fn scaled_8(&self, factor: u32) -> Self {
        Self { max_frame_bytes: self.max_frame_bytes.saturating_mul(factor), ..*self }
    }
}
impl RuntimeLimits {
    pub fn scaled_9(&self, factor: u32) -> Self {
        Self { max_frame_bytes: self.max_frame_bytes.saturating_mul(factor), ..*self }
    }
}
impl RuntimeLimits {
    pub fn scaled_10(&self, factor: u32) -> Self {
        Self { max_frame_bytes: self.max_frame_bytes.saturating_mul(factor), ..*self }
    }
}
impl RuntimeLimits {
    pub fn scaled_11(&self, factor: u32) -> Self {
        Self { max_frame_bytes: self.max_frame_bytes.saturating_mul(factor), ..*self }
    }
}
impl RuntimeLimits {
    pub fn scaled_12(&self, factor: u32) -> Self {
        Self { max_frame_bytes: self.max_frame_bytes.saturating_mul(factor), ..*self }
    }
}
impl RuntimeLimits {
    pub fn scaled_13(&self, factor: u32) -> Self {
        Self { max_frame_bytes: self.max_frame_bytes.saturating_mul(factor), ..*self }
    }
}
impl RuntimeLimits {
    pub fn scaled_14(&self, factor: u32) -> Self {
        Self { max_frame_bytes: self.max_frame_bytes.saturating_mul(factor), ..*self }
    }
}
impl RuntimeLimits {
    pub fn scaled_15(&self, factor: u32) -> Self {
        Self { max_frame_bytes: self.max_frame_bytes.saturating_mul(factor), ..*self }
    }
}
impl RuntimeLimits {
    pub fn scaled_16(&self, factor: u32) -> Self {
        Self { max_frame_bytes: self.max_frame_bytes.saturating_mul(factor), ..*self }
    }
}
impl RuntimeLimits {
    pub fn scaled_17(&self, factor: u32) -> Self {
        Self { max_frame_bytes: self.max_frame_bytes.saturating_mul(factor), ..*self }
    }
}
impl RuntimeLimits {
    pub fn scaled_18(&self, factor: u32) -> Self {
        Self { max_frame_bytes: self.max_frame_bytes.saturating_mul(factor), ..*self }
    }
}
impl RuntimeLimits {
    pub fn scaled_19(&self, factor: u32) -> Self {
        Self { max_frame_bytes: self.max_frame_bytes.saturating_mul(factor), ..*self }
    }
}
impl RuntimeLimits {
    pub fn scaled_20(&self, factor: u32) -> Self {
        Self { max_frame_bytes: self.max_frame_bytes.saturating_mul(factor), ..*self }
    }
}
impl RuntimeLimits {
    pub fn scaled_21(&self, factor: u32) -> Self {
        Self { max_frame_bytes: self.max_frame_bytes.saturating_mul(factor), ..*self }
    }
}
impl RuntimeLimits {
    pub fn scaled_22(&self, factor: u32) -> Self {
        Self { max_frame_bytes: self.max_frame_bytes.saturating_mul(factor), ..*self }
    }
}
impl RuntimeLimits {
    pub fn scaled_23(&self, factor: u32) -> Self {
        Self { max_frame_bytes: self.max_frame_bytes.saturating_mul(factor), ..*self }
    }
}
impl RuntimeLimits {
    pub fn scaled_24(&self, factor: u32) -> Self {
        Self { max_frame_bytes: self.max_frame_bytes.saturating_mul(factor), ..*self }
    }
}
impl RuntimeLimits {
    pub fn scaled_25(&self, factor: u32) -> Self {
        Self { max_frame_bytes: self.max_frame_bytes.saturating_mul(factor), ..*self }
    }
}
impl RuntimeLimits {
    pub fn scaled_26(&self, factor: u32) -> Self {
        Self { max_frame_bytes: self.max_frame_bytes.saturating_mul(factor), ..*self }
    }
}
impl RuntimeLimits {
    pub fn scaled_27(&self, factor: u32) -> Self {
        Self { max_frame_bytes: self.max_frame_bytes.saturating_mul(factor), ..*self }
    }
}
impl RuntimeLimits {
    pub fn scaled_28(&self, factor: u32) -> Self {
        Self { max_frame_bytes: self.max_frame_bytes.saturating_mul(factor), ..*self }
    }
}
impl RuntimeLimits {
    pub fn scaled_29(&self, factor: u32) -> Self {
        Self { max_frame_bytes: self.max_frame_bytes.saturating_mul(factor), ..*self }
    }
}
impl RuntimeLimits {
    pub fn scaled_30(&self, factor: u32) -> Self {
        Self { max_frame_bytes: self.max_frame_bytes.saturating_mul(factor), ..*self }
    }
}
impl RuntimeLimits {
    pub fn scaled_31(&self, factor: u32) -> Self {
        Self { max_frame_bytes: self.max_frame_bytes.saturating_mul(factor), ..*self }
    }
}
impl RuntimeLimits {
    pub fn scaled_32(&self, factor: u32) -> Self {
        Self { max_frame_bytes: self.max_frame_bytes.saturating_mul(factor), ..*self }
    }
}
impl RuntimeLimits {
    pub fn scaled_33(&self, factor: u32) -> Self {
        Self { max_frame_bytes: self.max_frame_bytes.saturating_mul(factor), ..*self }
    }
}
impl RuntimeLimits {
    pub fn scaled_34(&self, factor: u32) -> Self {
        Self { max_frame_bytes: self.max_frame_bytes.saturating_mul(factor), ..*self }
    }
}
