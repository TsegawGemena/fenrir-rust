//! Incremental decode checkpoints for resumable parse sessions.

use crate::error::DecodeError;
use crate::parser::{PartialState, Stage};

#[derive(Debug, Clone)]
pub struct ParseCheckpoint {
    pub stage: Stage,
    pub payload_len: u32,
    pub write_cursor: usize,
    pub checksum: u32,
    pub saved_bytes: Vec<u8>,
}

impl ParseCheckpoint {
    pub fn capture(state: &PartialState, write_cursor: usize, working: &[u8]) -> Self {
        Self {
            stage: state.stage,
            payload_len: state.payload_len,
            write_cursor,
            checksum: state.checksum,
            saved_bytes: working[..write_cursor.min(working.len())].to_vec(),
        }
    }

    pub fn restore_into(&self, state: &mut PartialState, working: &mut [u8]) -> Result<usize, DecodeError> {
        state.stage = self.stage;
        state.payload_len = self.payload_len;
        state.checksum = self.checksum;
        let n = self.saved_bytes.len().min(working.len());
        working[..n].copy_from_slice(&self.saved_bytes[..n]);
        Ok(n)
    }

    pub fn is_at_boundary(&self) -> bool {
        matches!(self.stage, Stage::Header | Stage::Payload | Stage::Trailer)
    }
}

#[derive(Debug, Default)]
pub struct CheckpointStack {
    stack: Vec<ParseCheckpoint>,
    max_depth: usize,
}

impl CheckpointStack {
    pub fn new(max_depth: usize) -> Self {
        Self { stack: Vec::new(), max_depth }
    }

    pub fn push(&mut self, cp: ParseCheckpoint) -> Result<(), DecodeError> {
        if self.stack.len() >= self.max_depth {
            return Err(DecodeError::Internal("checkpoint stack overflow"));
        }
        self.stack.push(cp);
        Ok(())
    }

    pub fn pop(&mut self) -> Option<ParseCheckpoint> {
        self.stack.pop()
    }

    pub fn depth(&self) -> usize {
        self.stack.len()
    }
}
