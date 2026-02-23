//! LibFuzzer harness for fenirer incremental decode (single-chunk feed).
#![no_main]

use libfuzzer_sys::fuzz_target;

use fenirer::DecodeSession;

fuzz_target!(|data: &[u8]| {
    let mut session = DecodeSession::new();
    let _ = session.feed(data);

    // Also exercise tight pool capacity — stresses buffer reuse paths.
    let mut tight = DecodeSession::with_pool_capacity(16);
    let _ = tight.feed(data);
});
