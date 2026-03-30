//! Wire format v2: optional metadata block and selectable checksum.

use crate::codec::{ChecksumAlgorithm, IncrementalChecksum};
use crate::error::DecodeError;
use crate::parser::MAGIC;

use super::flags::FrameFlags;
use super::header::{WireHeader, HEADER_LEN};
use super::limits::FrameLimits;
use super::metadata::FrameMetadata;
use super::trailer::{WireTrailer, TRAILER_LEN};

pub const V2_MAGIC: [u8; 4] = *b"FN2R";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WireFrameV2 {
    pub payload: Vec<u8>,
    pub checksum: u32,
    pub metadata: FrameMetadata,
}

pub struct FrameBuilderV2 {
    limits: FrameLimits,
    metadata: FrameMetadata,
}

impl Default for FrameBuilderV2 {
    fn default() -> Self { Self::new() }
}

impl FrameBuilderV2 {
    pub fn new() -> Self {
        Self {
            limits: FrameLimits::default(),
            metadata: FrameMetadata::default(),
        }
    }

    pub fn with_metadata(mut self, metadata: FrameMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn with_limits(mut self, limits: FrameLimits) -> Self {
        self.limits = limits;
        self
    }

    pub fn encode(&self, payload: &[u8]) -> Result<Vec<u8>, DecodeError> {
        if !self.limits.validate_payload(payload.len() as u32) {
            return Err(DecodeError::PayloadTooLarge {
                len: payload.len() as u32,
                max: self.limits.max_payload,
            });
        }
        let mut out = Vec::new();
        let mut header = [0u8; HEADER_LEN];
        WireHeader { magic: V2_MAGIC, payload_len: payload.len() as u32 }.encode(&mut header);
        out.extend_from_slice(&header);
        if self.metadata.flags.requires_extended_header()
            || !self.metadata.user_tags.is_empty()
            || self.metadata.checksum_algo != ChecksumAlgorithm::Xor
            || self.metadata.stream_id != 0
            || self.metadata.sequence != 0
        {
            self.metadata.encode(&mut out)?;
        }
        out.extend_from_slice(payload);
        let mut cs = IncrementalChecksum::new(self.metadata.checksum_algo);
        cs.feed(payload);
        let checksum = cs.finish();
        let mut trailer = [0u8; TRAILER_LEN];
        WireTrailer { checksum }.encode(&mut trailer);
        out.extend_from_slice(&trailer);
        Ok(out)
    }

    pub fn decode(input: &[u8]) -> Result<WireFrameV2, DecodeError> {
        if input.len() < HEADER_LEN + TRAILER_LEN {
            return Err(DecodeError::Internal("v2 frame too short"));
        }
        let magic: [u8; 4] = input[0..4].try_into().unwrap();
        if magic != V2_MAGIC && magic != MAGIC {
            return Err(DecodeError::InvalidMagic { got: magic });
        }
        let payload_len = u32::from_le_bytes(input[4..8].try_into().unwrap());
        let mut offset = HEADER_LEN;
        let mut metadata = FrameMetadata::default();
        if input.len() > offset + 2 && &input[offset..offset + 2] == b"MD" {
            let (md, consumed) = FrameMetadata::parse(&input[offset..])?;
            metadata = md;
            offset += consumed;
        }
        let payload_end = offset + payload_len as usize;
        if input.len() < payload_end + TRAILER_LEN {
            return Err(DecodeError::Internal("v2 payload truncated"));
        }
        let payload = input[offset..payload_end].to_vec();
        let trailer = &input[payload_end..payload_end + TRAILER_LEN];
        let checksum = u32::from_le_bytes(trailer.try_into().map_err(|_| DecodeError::Internal("trailer"))?);
        if !metadata.checksum_algo.verify(&payload, checksum) {
            return Err(DecodeError::ChecksumMismatch {
                expected: metadata.checksum_algo.compute(&payload),
                got: checksum,
            });
        }
        Ok(WireFrameV2 { payload, checksum, metadata })
    }
}

