use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeError {
    InvalidMagic { got: [u8; 4] },
    PayloadTooLarge { len: u32, max: u32 },
    ChecksumMismatch { expected: u32, got: u32 },
    BufferAlreadyReleased,
    PoolExhausted,
    Internal(&'static str),
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMagic { got } => write!(f, "invalid magic {got:?}"),
            Self::PayloadTooLarge { len, max } => {
                write!(f, "payload {len} exceeds max {max}")
            }
            Self::ChecksumMismatch { expected, got } => {
                write!(f, "checksum mismatch expected {expected:#x} got {got:#x}")
            }
            Self::BufferAlreadyReleased => write!(f, "buffer already released"),
            Self::PoolExhausted => write!(f, "buffer pool exhausted"),
            Self::Internal(msg) => write!(f, "internal: {msg}"),
        }
    }
}

impl std::error::Error for DecodeError {}
