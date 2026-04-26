mod common;

use fenirer::{decode_all, DecodeSession};
use common::{encode_frame, split_at};

#[test]
#[cfg(not(feature = "buggy"))]
fn single_pass_matches_incremental() {
    let payload = b"hello-fenirer".to_vec();
    let wire = encode_frame(&payload);

    let frames = decode_all(&wire).unwrap();
    assert_eq!(frames.len(), 1);
    assert_eq!(frames[0].payload, payload);

    let mut session = DecodeSession::new();
    let events = session.feed(&wire).unwrap();
    assert_eq!(events.len(), 1);
}

#[test]
#[cfg(not(feature = "buggy"))]
fn incremental_byte_chunks() {
    let payload: Vec<u8> = (0..48).collect();
    let wire = encode_frame(&payload);

    let mut session = DecodeSession::new();
    let mut frames = Vec::new();
    for byte in &wire {
        for ev in session.feed(std::slice::from_ref(byte)).unwrap() {
            if let fenirer::session::DecodeEvent::Frame(f) = ev {
                frames.push(f);
            }
        }
    }
    assert_eq!(frames.len(), 1);
    assert_eq!(frames[0].payload, payload);
}

#[test]
#[cfg(not(feature = "buggy"))]
fn tight_pool_with_adversarial_splits() {
    let payload: Vec<u8> = (0..12).collect();
    let wire = encode_frame(&payload);
    let chunks = split_at(&wire, &[5, 13, 18]);

    let mut session = DecodeSession::with_pool_capacity(14);
    let mut frames = Vec::new();
    for chunk in chunks {
        for ev in session.feed(&chunk).unwrap() {
            if let fenirer::session::DecodeEvent::Frame(f) = ev {
                frames.push(f);
            }
        }
    }
    assert_eq!(frames.len(), 1);
    assert_eq!(frames[0].payload, payload);
}

#[test]
#[cfg(feature = "buggy")]
fn buggy_chunked_splits_detect_stale_view() {
    let payload: Vec<u8> = (0..12).collect();
    let wire = encode_frame(&payload);

    let frames = decode_all(&wire);
    assert!(frames.is_err(), "single pass must fail once payload view is poisoned");

    let chunks = split_at(&wire, &[13, 20]);
    let mut session = DecodeSession::with_pool_capacity(14);
    let mut stale = false;
    for chunk in chunks {
        match session.feed(&chunk) {
            Ok(_) => {}
            Err(fenirer::DecodeError::Internal(msg)) if msg.contains("stale") => stale = true,
            Err(_) => {}
        }
    }
    assert!(stale, "buggy build must detect stale view on chunked input");
}

#[test]
#[cfg(feature = "buggy")]
fn buggy_payload_early_pool_release() {
    let payload: Vec<u8> = (0..8).collect();
    let wire = encode_frame(&payload);

    let mut session = DecodeSession::with_pool_capacity(12);
    let mid = wire.len() / 2;
    let _ = session.feed(&wire[..mid]);
    let result = session.feed(&wire[mid..]);
    assert!(result.is_err(), "buggy path must not complete frame after early pool release");
}
