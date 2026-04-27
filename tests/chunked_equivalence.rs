//! Property test: valid frames decode identically single-pass vs chunked.

mod common;

use common::encode_frame;
use fenirer::{decode_all, DecodeSession};
use proptest::prelude::*;

proptest! {
    #[test]
    fn chunked_matches_single_pass(payload in prop::collection::vec(any::<u8>(), 0..128)) {
        let wire = encode_frame(&payload);
        let single = decode_all(&wire).expect("single pass");

        let mut session = DecodeSession::with_pool_capacity(16);
        let mut chunked = Vec::new();
        let mut off = 0usize;
        let step = (payload.len() % 5) + 1;
        while off < wire.len() {
            let end = (off + step).min(wire.len());
            for ev in session.feed(&wire[off..end]).expect("chunk feed") {
                if let fenirer::session::DecodeEvent::Frame(f) = ev {
                    chunked.push(f);
                }
            }
            off = end;
        }

        prop_assert_eq!(single.len(), chunked.len());
        if !single.is_empty() {
            prop_assert_eq!(&single[0].payload, &chunked[0].payload);
            prop_assert_eq!(single[0].checksum, chunked[0].checksum);
        }
    }
}
