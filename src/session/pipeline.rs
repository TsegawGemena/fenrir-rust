use crate::error::DecodeError;
use crate::recovery::ResyncScanner;
use crate::session::{DecodeEvent, DecodeSession, Frame};

use super::config::SessionConfig;
use super::metrics::SessionMetrics;
use super::state::{SessionPhase, SessionState};

/// High-level decode pipeline with resync, metrics, and explicit phase tracking.
pub struct DecodePipeline {
    session: DecodeSession,
    config: SessionConfig,
    state: SessionState,
    metrics: SessionMetrics,
    resync: ResyncScanner,
}

impl DecodePipeline {
    pub fn new(config: SessionConfig) -> Self {
        let window = config.resync_window;
        Self {
            session: DecodeSession::with_pool_capacity(config.pool_capacity),
            resync: ResyncScanner::new(window),
            config,
            state: SessionState::new(),
            metrics: SessionMetrics::default(),
        }
    }

    pub fn with_defaults() -> Self {
        Self::new(SessionConfig::default())
    }

    pub fn feed(&mut self, chunk: &[u8]) -> Result<Vec<DecodeEvent>, DecodeError> {
        self.metrics.record_chunk(chunk.len());
        self.state.record_partial(chunk.len());
        self.state.enter(SessionPhase::Parsing(crate::parser::Stage::Header));

        match self.session.feed(chunk) {
            Ok(events) => {
                let frames = events.iter().filter(|e| matches!(e, DecodeEvent::Frame(_))).count();
                for _ in 0..frames {
                    self.metrics.record_frame_ok();
                    self.state.record_frame();
                }
                self.state.enter(SessionPhase::Idle);
                Ok(events)
            }
            Err(e) => {
                self.metrics.record_frame_err();
                if let DecodeError::Internal(msg) = &e {
                    if msg.contains("stale") {
                        self.metrics.record_stale_view();
                    }
                }
                if self.resync.feed(chunk).is_some() {
                    self.metrics.record_resync();
                }
                self.state.enter(SessionPhase::Error);
                Err(e)
            }
        }
    }

    pub fn decode_all_chunks(&mut self, chunks: &[&[u8]]) -> Result<Vec<Frame>, DecodeError> {
        let mut out = Vec::new();
        for chunk in chunks {
            for ev in self.feed(chunk)? {
                if let DecodeEvent::Frame(f) = ev {
                    out.push(f);
                }
            }
        }
        Ok(out)
    }

    pub fn metrics(&self) -> &SessionMetrics {
        &self.metrics
    }

    pub fn state(&self) -> &SessionState {
        &self.state
    }

    pub fn session_mut(&mut self) -> &mut DecodeSession {
        &mut self.session
    }
}
