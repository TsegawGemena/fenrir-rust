mod common;

use fenirer::{decode_all, DecodeSession};
use common::{encode_frame, split_at};

#[test]
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
