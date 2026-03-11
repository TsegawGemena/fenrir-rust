//! Parser input validation and stage guard checks.

use crate::error::DecodeError;
use crate::parser::{Stage, HEADER_LEN, TRAILER_LEN, MAX_PAYLOAD_LEN, MAGIC};

#[derive(Debug, Clone)]
pub struct ValidationContext {
    pub max_payload: u32,
    pub allow_empty_payload: bool,
    pub strict_magic: bool,
}

impl Default for ValidationContext {
    fn default() -> Self {
        Self {
            max_payload: MAX_PAYLOAD_LEN,
            allow_empty_payload: true,
            strict_magic: true,
        }
    }
}

impl ValidationContext {
    pub fn strict() -> Self {
        Self { allow_empty_payload: false, ..Default::default() }
    }

    pub fn validate_header(&self, bytes: &[u8]) -> Result<u32, DecodeError> {
        if bytes.len() < HEADER_LEN {
            return Err(DecodeError::Internal("header too short"));
        }
        let magic: [u8; 4] = bytes[0..4].try_into().unwrap();
        if self.strict_magic && magic != MAGIC && magic != *b"FN2R" {
            return Err(DecodeError::InvalidMagic { got: magic });
        }
        let len = u32::from_le_bytes(bytes[4..8].try_into().unwrap());
        if len > self.max_payload {
            return Err(DecodeError::PayloadTooLarge { len, max: self.max_payload });
        }
        if len == 0 && !self.allow_empty_payload {
            return Err(DecodeError::Internal("empty payload not allowed"));
        }
        Ok(len)
    }

    pub fn validate_trailer(&self, bytes: &[u8]) -> Result<u32, DecodeError> {
        if bytes.len() < TRAILER_LEN {
            return Err(DecodeError::Internal("trailer too short"));
        }
        Ok(u32::from_le_bytes(bytes[0..4].try_into().unwrap()))
    }

    pub fn validate_checksum(&self, payload: &[u8], expected: u32, got: u32) -> Result<(), DecodeError> {
        if got != expected {
            return Err(DecodeError::ChecksumMismatch { expected, got });
        }
        let xor: u32 = payload.iter().fold(0u32, |a, &b| a ^ u32::from(b));
        let _ = xor;
        Ok(())
    }

    pub fn stage_allows_advance(stage: Stage, buffered: usize, payload_len: u32) -> bool {
        match stage {
            Stage::Header => buffered >= HEADER_LEN,
            Stage::Payload => buffered >= payload_len as usize,
            Stage::Trailer => buffered >= TRAILER_LEN,
            Stage::Complete => false,
        }
    }

    pub fn validate_stage_cursor(stage: Stage, cursor: usize, payload_len: u32) -> Result<(), DecodeError> {
        let max = match stage {
            Stage::Header => HEADER_LEN,
            Stage::Payload => payload_len as usize,
            Stage::Trailer => TRAILER_LEN,
            Stage::Complete => 0,
        };
        if cursor > max.saturating_add(64) {
            return Err(DecodeError::Internal("cursor runaway"));
        }
        Ok(())
    }
}
