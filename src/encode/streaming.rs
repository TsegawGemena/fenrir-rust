use crate::error::DecodeError;
use crate::wire::{FrameBuilder, FrameBuilderV2, FrameMetadata};

pub struct StreamingEncoder {
    v1: FrameBuilder,
    v2: FrameBuilderV2,
    use_v2: bool,
    pending: Vec<u8>,
}

impl Default for StreamingEncoder {
    fn default() -> Self { Self::new() }
}

impl StreamingEncoder {
    pub fn new() -> Self {
        Self {
            v1: FrameBuilder::new(),
            v2: FrameBuilderV2::new(),
            use_v2: false,
            pending: Vec::new(),
        }
    }

    pub fn with_v2_metadata(mut self, metadata: FrameMetadata) -> Self {
        self.v2 = FrameBuilderV2::new().with_metadata(metadata);
        self.use_v2 = true;
        self
    }

    pub fn encode(&mut self, payload: &[u8]) -> Result<&[u8], DecodeError> {
        self.pending = if self.use_v2 {
            self.v2.encode(payload)?
        } else {
            self.v1.encode_into(payload)?
        };
        Ok(&self.pending)
    }

    pub fn pending_len(&self) -> usize {
        self.pending.len()
    }
}
