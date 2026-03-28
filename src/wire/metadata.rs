//! Optional metadata block appended after the standard header in wire v2.

use crate::codec::ChecksumAlgorithm;
use crate::error::DecodeError;

use super::flags::FrameFlags;

pub const METADATA_MAGIC: [u8; 2] = *b"MD";
pub const METADATA_HEADER_LEN: usize = 14;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrameMetadata {
    pub flags: FrameFlags,
    pub checksum_algo: ChecksumAlgorithm,
    pub stream_id: u32,
    pub sequence: u32,
    pub user_tags: Vec<u8>,
}

impl Default for FrameMetadata {
    fn default() -> Self {
        Self {
            flags: FrameFlags::NONE,
            checksum_algo: ChecksumAlgorithm::Xor,
            stream_id: 0,
            sequence: 0,
            user_tags: Vec::new(),
        }
    }
}

impl FrameMetadata {
    pub fn encoded_len(&self) -> usize {
        METADATA_HEADER_LEN + self.user_tags.len()
    }

    pub fn encode(&self, out: &mut Vec<u8>) -> Result<(), DecodeError> {
        out.reserve(self.encoded_len());
        out.extend_from_slice(&METADATA_MAGIC);
        out.extend_from_slice(&self.flags.to_le_bytes());
        out.push(self.checksum_algo.wire_id());
        out.push(self.user_tags.len().min(255) as u8);
        out.extend_from_slice(&self.stream_id.to_le_bytes());
        out.extend_from_slice(&self.sequence.to_le_bytes());
        let tag_len = self.user_tags.len().min(255);
        out.extend_from_slice(&self.user_tags[..tag_len]);
        Ok(())
    }

    pub fn parse(bytes: &[u8]) -> Result<(Self, usize), DecodeError> {
        if bytes.len() < METADATA_HEADER_LEN {
            return Err(DecodeError::Internal("metadata header too short"));
        }
        if bytes[0..2] != METADATA_MAGIC {
            return Err(DecodeError::Internal("metadata magic mismatch"));
        }
        let flags = FrameFlags::from_le_bytes([bytes[2], bytes[3]]);
        let checksum_algo = ChecksumAlgorithm::from_wire_id(bytes[4])
            .ok_or(DecodeError::Internal("unknown checksum algo"))?;
        let tag_len = bytes[5] as usize;
        let stream_id = u32::from_le_bytes(bytes[6..10].try_into().unwrap());
        let sequence = u32::from_le_bytes(bytes[10..14].try_into().unwrap());
        let total = METADATA_HEADER_LEN + tag_len;
        if bytes.len() < total {
            return Err(DecodeError::Internal("metadata tags truncated"));
        }
        Ok((
            Self {
                flags,
                checksum_algo,
                stream_id,
                sequence,
                user_tags: bytes[METADATA_HEADER_LEN..total].to_vec(),
            },
            total,
        ))
    }
}
