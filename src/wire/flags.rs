bitflags::bitflags! {
    /// Per-frame attribute flags (wire v2 extension header).
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct FrameFlags: u16 {
        const NONE          = 0;
        const COMPRESSED    = 1 << 0;
        const ENCRYPTED     = 1 << 1;
        const HAS_METADATA  = 1 << 2;
        const BATCHED       = 1 << 3;
        const CHECKPOINTED  = 1 << 4;
        const STREAM_END    = 1 << 5;
    }
}

impl Default for FrameFlags {
    fn default() -> Self {
        FrameFlags::NONE
    }
}

impl FrameFlags {
    pub fn from_le_bytes(bytes: [u8; 2]) -> Self {
        FrameFlags::from_bits_truncate(u16::from_le_bytes(bytes))
    }

    pub fn to_le_bytes(self) -> [u8; 2] {
        self.bits().to_le_bytes()
    }

    pub fn requires_extended_header(self) -> bool {
        !self.is_empty()
    }
}
