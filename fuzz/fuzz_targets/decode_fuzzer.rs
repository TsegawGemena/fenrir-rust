//! Fuzz single-pass and incremental decode with arbitrary input.
#![no_main]

use libfuzzer_sys::fuzz_target;

use fenirer::{decode_all, DecodeSession};

fuzz_target!(|data: &[u8]| {
    let _ = decode_all(data);

    let mut session = DecodeSession::with_pool_capacity(32);
    let _ = session.feed(data);
});
