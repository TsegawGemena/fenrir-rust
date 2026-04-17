//! Protocol version 2: metadata + selectable checksum.

use crate::codec::ChecksumAlgorithm;
use crate::wire::{FrameFlags, FrameLimits, FrameMetadata};

pub const VERSION: u8 = 2;

#[derive(Debug, Clone)]
pub struct ProtocolV2 {
    pub limits: FrameLimits,
    pub default_metadata: FrameMetadata,
}

impl Default for ProtocolV2 {
    fn default() -> Self {
        Self {
            limits: FrameLimits::default(),
            default_metadata: FrameMetadata {
                flags: FrameFlags::NONE,
                checksum_algo: ChecksumAlgorithm::Xor,
                stream_id: 0,
                sequence: 0,
                user_tags: Vec::new(),
            },
        }
    }
}

impl ProtocolV2 {
    pub fn with_checksum(mut self, algo: ChecksumAlgorithm) -> Self {
        self.default_metadata.checksum_algo = algo;
        self
    }

    pub fn with_stream_id(mut self, id: u32) -> Self {
        self.default_metadata.stream_id = id;
        self
    }

    pub fn next_sequence(&mut self) -> u32 {
        let s = self.default_metadata.sequence;
        self.default_metadata.sequence = s.wrapping_add(1);
        s
    }

    pub fn metadata_overhead(&self) -> usize {
        if self.default_metadata.flags.requires_extended_header() {
            self.default_metadata.encoded_len()
        } else {
            0
        }
    }
}
