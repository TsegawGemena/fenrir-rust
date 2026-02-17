use crate::codec::xor_checksum;
use crate::error::DecodeError;
use crate::parser::MAGIC;

use super::header::{WireHeader, HEADER_LEN};
use super::limits::FrameLimits;
use super::trailer::{WireTrailer, TRAILER_LEN};

pub struct FrameBuilder {
    limits: FrameLimits,
}

impl Default for FrameBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl FrameBuilder {
    pub fn new() -> Self {
        Self {
            limits: FrameLimits::default(),
        }
    }

    pub fn with_limits(mut self, limits: FrameLimits) -> Self {
        self.limits = limits;
        self
    }

    pub fn encode(&self, payload: &[u8]) -> Vec<u8> {
        self.encode_into(payload).expect("valid payload")
    }

    pub fn encode_into(&self, payload: &[u8]) -> Result<Vec<u8>, DecodeError> {
        if !self.limits.validate_payload(payload.len() as u32) {
            return Err(DecodeError::PayloadTooLarge {
                len: payload.len() as u32,
                max: self.limits.max_payload,
            });
        }

        let mut out = Vec::with_capacity(HEADER_LEN + payload.len() + TRAILER_LEN);
        let mut header = [0u8; HEADER_LEN];
        WireHeader {
            magic: MAGIC,
            payload_len: payload.len() as u32,
        }
        .encode(&mut header);
        out.extend_from_slice(&header);
        out.extend_from_slice(payload);

        let cs = xor_checksum(payload);
        let mut trailer = [0u8; TRAILER_LEN];
        WireTrailer { checksum: cs }.encode(&mut trailer);
        out.extend_from_slice(&trailer);
        Ok(out)
    }
}
