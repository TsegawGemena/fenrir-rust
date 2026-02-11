#[derive(Debug, Clone, Copy)]
pub struct FrameLimits {
    pub max_payload: u32,
}

impl Default for FrameLimits {
    fn default() -> Self {
        Self {
            max_payload: 1 << 20,
        }
    }
}

impl FrameLimits {
    pub fn validate_payload(&self, len: u32) -> bool {
        len <= self.max_payload
    }
}
