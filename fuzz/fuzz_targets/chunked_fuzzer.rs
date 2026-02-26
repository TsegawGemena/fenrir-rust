//! Fuzz fragmented (chunked) decode — stresses incremental buffer reuse.
#![no_main]

use libfuzzer_sys::fuzz_target;

use fenirer::DecodeSession;

fn feed_chunked(data: &[u8]) {
    if data.is_empty() {
        return;
    }
    let cap = 8 + (data[data.len() - 1] as usize % 32);
    let mut session = DecodeSession::with_pool_capacity(cap);
    let mut step = 1usize;
    let mut offset = 0usize;
    while offset < data.len() {
        let end = (offset + step).min(data.len());
        let _ = session.feed(&data[offset..end]);
        offset = end;
        step = (step + 1).min(9);
    }
}

fuzz_target!(|data: &[u8]| {
    feed_chunked(data);
});
