use crate::error::DecodeError;

#[derive(Debug, Clone, Copy)]
pub enum BackpressurePolicy {
    Block { max_pending: usize },
    DropOldest { max_pending: usize },
    Fail,
}

impl Default for BackpressurePolicy {
    fn default() -> Self {
        Self::Block { max_pending: 65536 }
    }
}

impl BackpressurePolicy {
    pub fn apply(&self, pending_len: usize) -> Result<(), DecodeError> {
        match self {
            Self::Block { max_pending } if pending_len > *max_pending => {
                Err(DecodeError::Internal("backpressure: pending limit"))
            }
            Self::Fail if pending_len > 4096 => {
                Err(DecodeError::Internal("backpressure: fail policy"))
            }
            _ => Ok(()),
        }
    }

    pub fn max_pending(&self) -> usize {
        match self {
            Self::Block { max_pending } | Self::DropOldest { max_pending } => *max_pending,
            Self::Fail => 4096,
        }
    }
}
