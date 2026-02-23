//! Checksum dispatch for wire-format validation.

use super::crc32::Crc32;
use super::xor::xor_checksum;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ChecksumAlgorithm {
    #[default]
    Xor,
    Crc32Ieee,
}

impl ChecksumAlgorithm {
    pub fn compute(self, payload: &[u8]) -> u32 {
        match self {
            Self::Xor => xor_checksum(payload),
            Self::Crc32Ieee => Crc32::compute(payload),
        }
    }

    pub fn verify(self, payload: &[u8], expected: u32) -> bool {
        self.compute(payload) == expected
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Xor => "xor",
            Self::Crc32Ieee => "crc32-ieee",
        }
    }

    pub fn from_wire_id(id: u8) -> Option<Self> {
        match id {
            0 => Some(Self::Xor),
            1 => Some(Self::Crc32Ieee),
            _ => None,
        }
    }

    pub fn wire_id(self) -> u8 {
        match self {
            Self::Xor => 0,
            Self::Crc32Ieee => 1,
        }
    }
}

/// Incremental checksum for streaming payload assembly.
#[derive(Debug, Clone, Copy)]
pub struct IncrementalChecksum {
    algo: ChecksumAlgorithm,
    xor_acc: u32,
    crc: Crc32,
}

impl IncrementalChecksum {
    pub fn new(algo: ChecksumAlgorithm) -> Self {
        Self {
            algo,
            xor_acc: 0,
            crc: Crc32::new(),
        }
    }

    pub fn feed(&mut self, bytes: &[u8]) {
        match self.algo {
            ChecksumAlgorithm::Xor => {
                for &b in bytes {
                    self.xor_acc ^= u32::from(b);
                }
            }
            ChecksumAlgorithm::Crc32Ieee => self.crc.update(bytes),
        }
    }

    pub fn finish(self) -> u32 {
        match self.algo {
            ChecksumAlgorithm::Xor => self.xor_acc,
            ChecksumAlgorithm::Crc32Ieee => self.crc.finalize(),
        }
    }
}
