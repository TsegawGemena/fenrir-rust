//! Client/server capability negotiation.

use crate::codec::ChecksumAlgorithm;
use crate::error::DecodeError;

use super::v1::ProtocolV1;
use super::v2::ProtocolV2;

#[derive(Debug, Clone)]
pub struct CapabilityOffer {
    pub max_v: u8,
    pub checksums: Vec<ChecksumAlgorithm>,
    pub max_payload: u32,
}

impl Default for CapabilityOffer {
    fn default() -> Self {
        Self {
            max_v: 2,
            checksums: vec![ChecksumAlgorithm::Xor, ChecksumAlgorithm::Crc32Ieee],
            max_payload: 1 << 20,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NegotiatedProtocol {
    pub version: u8,
    pub checksum: ChecksumAlgorithm,
    pub max_payload: u32,
}

impl NegotiatedProtocol {
    pub fn negotiate(offer: &CapabilityOffer, client_max: u8) -> Result<Self, DecodeError> {
        let version = offer.max_v.min(client_max);
        if version == 0 {
            return Err(DecodeError::Internal("no compatible protocol version"));
        }
        let checksum = offer
            .checksums
            .first()
            .copied()
            .unwrap_or(ChecksumAlgorithm::Xor);
        Ok(Self {
            version,
            checksum,
            max_payload: offer.max_payload,
        })
    }

    pub fn as_v1(&self) -> ProtocolV1 {
        let mut p = ProtocolV1::default();
        p.limits.max_payload = self.max_payload;
        p
    }

    pub fn as_v2(&self) -> ProtocolV2 {
        let mut p = ProtocolV2::default().with_checksum(self.checksum);
        p.limits.max_payload = self.max_payload;
        p
    }
}
