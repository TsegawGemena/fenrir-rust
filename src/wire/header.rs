use crate::error::DecodeError;
use crate::parser::MAGIC;

pub const HEADER_LEN: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WireHeader {
    pub magic: [u8; 4],
    pub payload_len: u32,
}

impl WireHeader {
    pub fn parse(bytes: &[u8]) -> Result<Self, DecodeError> {
        if bytes.len() < HEADER_LEN {
            return Err(DecodeError::Internal("short header"));
        }
        let magic: [u8; 4] = bytes[0..4].try_into().unwrap();
        if magic != MAGIC {
            return Err(DecodeError::InvalidMagic { got: magic });
        }
        Ok(Self {
            magic,
            payload_len: u32::from_le_bytes(bytes[4..8].try_into().unwrap()),
        })
    }

    pub fn encode(&self, out: &mut [u8]) {
        out[0..4].copy_from_slice(&self.magic);
        out[4..8].copy_from_slice(&self.payload_len.to_le_bytes());
    }
}
