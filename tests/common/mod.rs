//! Helpers for building wire-format frames in tests.

use fenirer::parser::{MAGIC, TRAILER_LEN, HEADER_LEN};

pub fn payload_checksum(payload: &[u8]) -> u32 {
    payload.iter().fold(0u32, |acc, &b| acc ^ u32::from(b))
}

pub fn encode_frame(payload: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(HEADER_LEN + payload.len() + TRAILER_LEN);
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    out.extend_from_slice(payload);
    out.extend_from_slice(&payload_checksum(payload).to_le_bytes());
    out
}

pub fn split_at(input: &[u8], points: &[usize]) -> Vec<Vec<u8>> {
    let mut chunks = Vec::new();
    let mut start = 0;
    for &p in points {
        chunks.push(input[start..p].to_vec());
        start = p;
    }
    chunks.push(input[start..].to_vec());
    chunks
}
