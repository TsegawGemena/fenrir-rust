//! Replay logged chunk sequences through a decode session.

use crate::error::DecodeError;
use crate::session::{DecodeEvent, DecodeSession, Frame};

#[derive(Debug, Clone)]
pub struct ChunkRecord {
    pub offset: u64,
    pub data: Vec<u8>,
}

#[derive(Debug, Default)]
pub struct ReplayLog {
    pub records: Vec<ChunkRecord>,
    pub pool_capacity: usize,
}

impl ReplayLog {
    pub fn new(pool_capacity: usize) -> Self {
        Self {
            records: Vec::new(),
            pool_capacity,
        }
    }

    pub fn record(&mut self, offset: u64, data: Vec<u8>) {
        self.records.push(ChunkRecord { offset, data });
    }

    pub fn replay(&self) -> Result<Vec<Frame>, DecodeError> {
        let mut session = DecodeSession::with_pool_capacity(self.pool_capacity);
        let mut frames = Vec::new();
        for rec in &self.records {
            for ev in session.feed(&rec.data)? {
                if let DecodeEvent::Frame(f) = ev {
                    frames.push(f);
                }
            }
        }
        Ok(frames)
    }

    pub fn total_bytes(&self) -> usize {
        self.records.iter().map(|r| r.data.len()).sum()
    }
}

/// Capture feed calls and replay later (useful for reproducing fuzz crashes).
pub struct RecordingSession {
    inner: DecodeSession,
    log: ReplayLog,
    byte_offset: u64,
}

impl RecordingSession {
    pub fn with_pool_capacity(cap: usize) -> Self {
        Self {
            inner: DecodeSession::with_pool_capacity(cap),
            log: ReplayLog::new(cap),
            byte_offset: 0,
        }
    }

    pub fn feed(&mut self, chunk: &[u8]) -> Result<Vec<DecodeEvent>, DecodeError> {
        self.log.record(self.byte_offset, chunk.to_vec());
        self.byte_offset += chunk.len() as u64;
        self.inner.feed(chunk)
    }

    pub fn into_log(self) -> ReplayLog {
        self.log
    }

    pub fn inner_mut(&mut self) -> &mut DecodeSession {
        &mut self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wire::FrameBuilder;

    #[test]
    fn replay_matches_live() {
        let payload = b"replay-test";
        let wire = FrameBuilder::new().encode(payload);
        let mut rec = RecordingSession::with_pool_capacity(32);
        rec.feed(&wire[..4]).unwrap();
        rec.feed(&wire[4..]).unwrap();
        let log = rec.into_log();
        let frames = log.replay().unwrap();
        assert_eq!(frames.len(), 1);
        assert_eq!(frames[0].payload, payload);
    }
}
