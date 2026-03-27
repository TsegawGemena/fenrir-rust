mod config;
mod metrics;
mod pipeline;
mod replay;
mod state;

use crate::buffer::BufferPool;
use crate::error::DecodeError;
use crate::finalize::{Finalizer, FrameArtifacts};
use crate::invariants;
use crate::parser::FrameParser;

pub use config::SessionConfig;
pub use metrics::SessionMetrics;
pub use pipeline::DecodePipeline;
pub use replay::{ChunkRecord, RecordingSession, ReplayLog};
pub use state::{SessionPhase, SessionState};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frame {
    pub payload: Vec<u8>,
    pub checksum: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeEvent {
    Frame(Frame),
    NeedMoreData,
}

#[derive(Debug)]
pub struct DecodeSession {
    pool: BufferPool,
    parser: Option<FrameParser>,
    finalizer: Finalizer,
}

impl DecodeSession {
    pub fn new() -> Self {
        Self {
            pool: BufferPool::new(),
            parser: None,
            finalizer: Finalizer::new(),
        }
    }

    pub fn with_pool_capacity(capacity: usize) -> Self {
        Self {
            pool: BufferPool::with_capacity(capacity),
            parser: None,
            finalizer: Finalizer::new(),
        }
    }

    pub fn feed(&mut self, chunk: &[u8]) -> Result<Vec<DecodeEvent>, DecodeError> {
        let mut events = Vec::new();
        let mut offset = 0;

        while offset < chunk.len() {
            if self.parser.is_none() {
                self.parser = Some(FrameParser::new(&mut self.pool)?);
            }

            let consumed = self.feed_once(&chunk[offset..])?;
            if consumed == 0 {
                break;
            }
            offset += consumed;

            while let Some(frame) = self.try_complete_frame()? {
                events.push(DecodeEvent::Frame(frame));
            }
        }

        Ok(events)
    }

    fn feed_once(&mut self, chunk: &[u8]) -> Result<usize, DecodeError> {
        let parser = self.parser.as_mut().unwrap();

        #[cfg(feature = "buggy")]
        {
            if parser.write_cursor() + chunk.len() > parser.working_buffer().capacity()
                && !parser.is_complete()
            {
                let old = parser.take_working_buffer()?;
                self.pool.release(old);
                let fresh = self.pool.acquire()?;
                parser.set_working_buffer(fresh);
            }
        }

        #[cfg(not(feature = "buggy"))]
        {
            let needed = parser.write_cursor() + chunk.len();
            if needed > parser.working_buffer().capacity() {
                invariants::check_no_trailer_pin(parser, "feed_once grow");
                if parser.trailer_pinned_on_working() {
                    return Err(DecodeError::Internal(
                        "working buffer too small while trailer view is pinned",
                    ));
                }
                let saved = parser.snapshot_working();
                let old = parser.take_working_buffer()?;
                self.pool.release(old);
                let fresh = self.pool.acquire_sized(needed)?;
                parser.reinstall_working(fresh, saved)?;
            }
        }

        let n = parser.append(chunk)?;
        if n == 0 {
            return Ok(0);
        }

        parser.try_advance_stages()?;
        Ok(n)
    }

    fn try_complete_frame(&mut self) -> Result<Option<Frame>, DecodeError> {
        let parser = match self.parser.as_mut() {
            Some(p) if p.is_complete() => p,
            _ => return Ok(None),
        };

        let payload_view = parser
            .state
            .payload_view
            .take()
            .ok_or(DecodeError::Internal("missing payload view"))?;

        let trailer_view = parser
            .state
            .trailer_view
            .take()
            .ok_or(DecodeError::Internal("missing trailer view"))?;

        if payload_view.is_stale() {
            return Err(DecodeError::Internal("stale buffer view detected"));
        }

        let trailer_owner = parser.working_buffer();
        invariants::check_view_owner(&trailer_view, trailer_owner, "finalize trailer");

        if !trailer_view.same_backing_as(trailer_owner) {
            return Err(DecodeError::Internal("stale buffer view detected"));
        }

        invariants::check_finalize_stage(parser);

        let trailer = trailer_view.as_slice();
        let got = u32::from_le_bytes(trailer.try_into().map_err(|_| {
            DecodeError::Internal("trailer too short")
        })?);

        if got != parser.expected_checksum() {
            return Err(DecodeError::ChecksumMismatch {
                expected: parser.expected_checksum(),
                got,
            });
        }

        let payload = payload_view.as_slice().to_vec();
        let payload_buffer = payload_view.into_owner();
        let trailer_buffer = parser.take_working_buffer()?;

        parser.state.reset();
        parser.clear_views();
        parser.set_working_buffer(self.pool.acquire()?);

        self.finalizer.finalize(
            FrameArtifacts {
                payload,
                checksum: got,
                payload_buffer,
                trailer_buffer,
            },
            &mut self.pool,
        )
        .map(Some)
    }
}

impl Default for DecodeSession {
    fn default() -> Self {
        Self::new()
    }
}
