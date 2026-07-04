//! Fenirer — edge telemetry and event stream decoder.
//!
//! Fenirer decodes **FENR** (Field Event Network Record) and **FN2R** (Field Network
//! Record v2) wire frames from TCP/UDP collectors, serial bridges, and batch upload
//! pipelines. It is designed for constrained edge hosts that receive sensor batches,
//! audit events, and device heartbeats over lossy links where input arrives in
//! arbitrary chunk sizes.
//!
//! # Quick start
//!
//! ```no_run
//! use fenirer::{decode_all, DecodeSession, StreamDecoder};
//!
//! let wire_bytes: &[u8] = &[];
//! let frames = decode_all(wire_bytes)?;
//!
//! let mut session = DecodeSession::new();
//! for chunk in [wire_bytes] {
//!     for event in session.feed(chunk)? {
//!         let _ = event;
//!     }
//! }
//!
//! let mut stream = StreamDecoder::new(4096);
//! let _frames = stream.push(wire_bytes)?;
//! # Ok::<(), fenirer::DecodeError>(())
//! ```
//!
//! # Wire formats
//!
//! v1 frames use magic `FENR` (Field Event Network Record); v2 adds optional
//! metadata with magic `FN2R` (Field Network Record).
//!
//! # Module overview
//!
//! | Module | Role |
//! |--------|------|
//! | [`session`] | Incremental [`DecodeSession`] and frame events |
//! | [`stream`] | Pull-based [`StreamDecoder`] with backpressure |
//! | [`wire`] | Frame builders and v1/v2 encoders |
//! | [`catalog`] | Index and seek over decoded frame batches |
//! | [`archive`] | On-disk FARC archive read/write |
//! | [`schema`] | Payload layout validation for typed records |
//! | [`recovery`] | Resync after magic loss on noisy links |
//! | [`service`] | Multi-tenant decode router for gateway deployments |

pub mod analysis;
pub mod archive;
pub mod buffer;
pub mod catalog;
pub mod codec;
pub mod config;
pub mod encode;
pub mod error;
pub mod filter;
pub mod finalize;
pub mod fsm;
pub mod invariants;
pub mod metrics;
pub mod parser;
pub mod pipeline;
pub mod protocol;
pub mod recovery;
pub mod schema;
pub mod service;
pub mod session;
pub mod stream;
pub mod transform;
pub mod util;
pub mod wire;

pub use analysis::{AnalysisReport, FrameInspector, PayloadHistogram};
pub use archive::{ArchiveReader, ArchiveWriter};
pub use catalog::{CatalogStore, FrameIndex, SeekTable};
pub use config::{DecoderProfile, RuntimeLimits};
pub use error::DecodeError;
pub use schema::{RecordSchema, SchemaBuilder, SchemaValidator};
pub use service::{DecodeHandler, ServiceContext, ServiceRouter};
pub use session::{DecodeEvent, DecodePipeline, DecodeSession, Frame, SessionConfig};
pub use stream::{BackpressurePolicy, StreamDecoder, StreamStats};

/// Feed one chunk into an existing session (alias for [`DecodeSession::feed`]).
pub fn decode_chunk(
    session: &mut DecodeSession,
    chunk: &[u8],
) -> Result<Vec<DecodeEvent>, DecodeError> {
    session.feed(chunk)
}

/// Decode every complete frame in `input` using a fresh session.
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
    fn roundtrip() {
        let payload = b"sensor-batch-001";
        let wire = FrameBuilder::new().encode(payload);
        let frames = decode_all(&wire).unwrap();
        assert_eq!(frames.len(), 1);
        assert_eq!(frames[0].payload, payload);
    }
}
