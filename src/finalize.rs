//! Frame finalization and buffer lifecycle cleanup.

use crate::buffer::{BufferPool, SharedBuffer};
use crate::error::DecodeError;
use crate::Frame;

/// Resources associated with a completed frame before handoff.
pub struct FrameArtifacts {
    pub payload: Vec<u8>,
    pub checksum: u32,
    pub payload_buffer: SharedBuffer,
    pub trailer_buffer: SharedBuffer,
}

/// Performs validation and returns pooled buffers after a frame is complete.
#[derive(Debug, Default)]
pub struct Finalizer {
    finalized_count: u64,
}

impl Finalizer {
    pub fn new() -> Self {
        Self { finalized_count: 0 }
    }

    pub fn finalized_count(&self) -> u64 {
        self.finalized_count
    }

    /// Complete the frame and recycle its backing buffer exactly once.
    pub fn finalize(
        &mut self,
        artifacts: FrameArtifacts,
        pool: &mut BufferPool,
    ) -> Result<Frame, DecodeError> {
        let frame = Frame {
            payload: artifacts.payload,
            checksum: artifacts.checksum,
        };

        pool.release(artifacts.payload_buffer);
        pool.release(artifacts.trailer_buffer);
        self.finalized_count += 1;
        Ok(frame)
    }
}
