//! Protocol version 1: baseline FENR frames.

use crate::wire::FrameLimits;

pub const VERSION: u8 = 1;

#[derive(Debug, Clone)]
pub struct ProtocolV1 {
    pub limits: FrameLimits,
}

impl Default for ProtocolV1 {
    fn default() -> Self {
        Self {
            limits: FrameLimits::default(),
        }
    }
}

impl ProtocolV1 {
    pub fn max_frame_bytes(&self) -> usize {
        self.limits.max_payload as usize + 16
    }

    pub fn supports_streaming(&self) -> bool {
        true
    }

    pub fn validate_frame_size(&self, payload_len: u32) -> bool {
        self.limits.validate_payload(payload_len)
    }
}
