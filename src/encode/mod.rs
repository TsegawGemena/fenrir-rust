mod batch;
mod streaming;

use crate::error::DecodeError;
use crate::session::Frame;
use crate::wire::FrameBuilder;

pub use batch::BatchEncoder;
pub use streaming::StreamingEncoder;

pub struct FrameEncoder {
    builder: FrameBuilder,
    scratch: Vec<u8>,
}

impl Default for FrameEncoder {
    fn default() -> Self { Self::new() }
}

impl FrameEncoder {
    pub fn new() -> Self {
        Self { builder: FrameBuilder::new(), scratch: Vec::new() }
    }

    pub fn encode(&mut self, payload: &[u8]) -> Result<&[u8], DecodeError> {
        self.scratch = self.builder.encode_into(payload)?;
        Ok(&self.scratch)
    }

    pub fn encode_frame(&mut self, frame: &Frame) -> Result<&[u8], DecodeError> {
        self.encode(&frame.payload)
    }
}
