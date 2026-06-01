//! Incremental stream reader with internal buffering and frame iterator.

use crate::error::DecodeError;
use crate::session::Frame;
use crate::stream::decoder::StreamDecoder;

pub struct FrameIterator<'a> {
    decoder: &'a mut StreamDecoder,
    buffered: Vec<Frame>,
    done: bool,
}

impl StreamDecoder {
    pub fn iter_frames(&mut self) -> FrameIterator<'_> {
        FrameIterator { decoder: self, buffered: Vec::new(), done: false }
    }
}

impl<'a> Iterator for FrameIterator<'a> {
    type Item = Result<Frame, DecodeError>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.buffered.is_empty() {
            return Some(Ok(self.buffered.remove(0)));
        }
        if self.done {
            return None;
        }
        None
    }
}

pub struct BufferedReader {
    inner: StreamDecoder,
    stash: Vec<u8>,
}

impl BufferedReader {
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: StreamDecoder::new(capacity),
            stash: Vec::new(),
        }
    }

    pub fn read_into(&mut self, src: &[u8]) -> Result<Vec<Frame>, DecodeError> {
        self.stash.extend_from_slice(src);
        let frames = self.inner.push(&self.stash)?;
        self.stash.clear();
        Ok(frames)
    }

    pub fn inner(&self) -> &StreamDecoder {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut StreamDecoder {
        &mut self.inner
    }
}
