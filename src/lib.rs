//! Fenirer streaming frame decoder.
//!
//! Wire layout (little-endian):
//! `[ magic:4 "FENR" ][ payload_len:4 ][ payload ][ checksum:4 ]`
//! Checksum = XOR of payload bytes as u32.

pub mod buffer;
pub mod codec;
pub mod encode;
pub mod error;
pub mod finalize;
pub mod invariants;
pub mod parser;
pub mod protocol;
pub mod recovery;
pub mod session;
pub mod util;
pub mod wire;

pub mod archive;
pub mod analysis;
pub mod catalog;
pub mod config;
pub mod filter;
pub mod fsm;
pub mod metrics;
pub mod pipeline;
pub mod schema;
pub mod service;
pub mod stream;
pub mod transform;

pub use error::DecodeError;
pub use session::{DecodeEvent, DecodePipeline, DecodeSession, Frame, SessionConfig};

pub fn decode_chunk(
    session: &mut DecodeSession,
    chunk: &[u8],
) -> Result<Vec<DecodeEvent>, DecodeError> {
    session.feed(chunk)
}

pub fn decode_all(input: &[u8]) -> Result<Vec<Frame>, DecodeError> {
    let mut session = DecodeSession::new();
    let events = session.feed(input)?;
    Ok(events
        .into_iter()
        .filter_map(|e| match e {
            DecodeEvent::Frame(f) => Some(f),
            DecodeEvent::NeedMoreData => None,
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wire::FrameBuilder;

    #[test]
    #[cfg(not(feature = "buggy"))]
    fn roundtrip() {
        let payload = b"fenirer";
        let wire = FrameBuilder::new().encode(payload);
        let frames = decode_all(&wire).unwrap();
        assert_eq!(frames.len(), 1);
        assert_eq!(frames[0].payload, payload);
    }

    #[test]
    #[cfg(feature = "buggy")]
    fn roundtrip_buggy_detects_stale_view() {
        let payload = b"fenirer";
        let wire = FrameBuilder::new().encode(payload);
        let err = decode_all(&wire).unwrap_err();
        assert!(matches!(err, DecodeError::Internal(_)));
    }
}
