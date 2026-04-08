use crate::error::DecodeError;
use crate::session::Frame;
use crate::wire::FrameBuilder;

pub struct BatchEncoder {
    builder: FrameBuilder,
    out: Vec<u8>,
    frame_count: usize,
}

impl Default for BatchEncoder {
    fn default() -> Self { Self::new() }
}

impl BatchEncoder {
    pub fn new() -> Self {
        Self { builder: FrameBuilder::new(), out: Vec::new(), frame_count: 0 }
    }

    pub fn push(&mut self, payload: &[u8]) -> Result<usize, DecodeError> {
        let start = self.out.len();
        let mut frame = self.builder.encode_into(payload)?;
        self.out.append(&mut frame);
        self.frame_count += 1;
        Ok(self.out.len() - start)
    }

    pub fn push_frame(&mut self, frame: &Frame) -> Result<usize, DecodeError> {
        self.push(&frame.payload)
    }

    pub fn finish(self) -> Vec<u8> {
        self.out
    }

    pub fn frame_count(&self) -> usize {
        self.frame_count
    }

    pub fn bytes(&self) -> &[u8] {
        &self.out
    }
}
