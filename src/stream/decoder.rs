//! Multi-frame stream decoder: consumes byte chunks and yields completed frames.

use crate::error::DecodeError;
use crate::session::{DecodeEvent, DecodeSession, Frame};
use crate::stream::backpressure::BackpressurePolicy;
use crate::stream::stats::StreamStats;

/// Pull-based decoder for unbounded input streams.
pub struct StreamDecoder {
    session: DecodeSession,
    stats: StreamStats,
    backpressure: BackpressurePolicy,
    pending: Vec<u8>,
    max_pending: usize,
    closed: bool,
}

impl StreamDecoder {
    pub fn new(pool_capacity: usize) -> Self {
        Self {
            session: DecodeSession::with_pool_capacity(pool_capacity),
            stats: StreamStats::default(),
            backpressure: BackpressurePolicy::default(),
            pending: Vec::new(),
            max_pending: pool_capacity * 4,
            closed: false,
        }
    }

    pub fn with_backpressure(mut self, policy: BackpressurePolicy) -> Self {
        self.backpressure = policy;
        self
    }

    pub fn push(&mut self, chunk: &[u8]) -> Result<Vec<Frame>, DecodeError> {
        if self.closed {
            return Err(DecodeError::Internal("stream closed"));
        }
        self.stats.bytes_received += chunk.len() as u64;
        self.pending.extend_from_slice(chunk);
        if self.pending.len() > self.max_pending {
            self.backpressure.apply(self.pending.len())?;
        }
        self.drain_frames()
    }

    pub fn close(&mut self) -> Result<Vec<Frame>, DecodeError> {
        self.closed = true;
        self.drain_frames()
    }

    fn drain_frames(&mut self) -> Result<Vec<Frame>, DecodeError> {
        let mut out = Vec::new();
        if self.pending.is_empty() {
            return Ok(out);
        }
        let events = self.session.feed(&self.pending)?;
        self.pending.clear();
        for ev in events {
            match ev {
                DecodeEvent::Frame(f) => {
                    self.stats.frames_decoded += 1;
                    out.push(f);
                }
                DecodeEvent::NeedMoreData => {
                    self.stats.need_more += 1;
                }
            }
        }
        Ok(out)
    }

    pub fn stats(&self) -> &StreamStats {
        &self.stats
    }

    pub fn session_mut(&mut self) -> &mut DecodeSession {
        &mut self.session
    }
}
