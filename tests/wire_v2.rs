use fenirer::wire::{FrameBuilderV2, FrameFlags, FrameMetadata};
use fenirer::codec::ChecksumAlgorithm;

#[test]
fn v2_roundtrip_xor() {
    let md = FrameMetadata {
        flags: FrameFlags::HAS_METADATA,
        checksum_algo: ChecksumAlgorithm::Xor,
        stream_id: 1,
        sequence: 42,
        user_tags: vec![0xAA, 0xBB],
    };
    let payload = b"v2-frame-payload";
    let wire = FrameBuilderV2::new()
        .with_metadata(md.clone())
        .encode(payload)
        .unwrap();
    let decoded = FrameBuilderV2::decode(&wire).unwrap();
    assert_eq!(decoded.payload, payload);
    assert_eq!(decoded.metadata.stream_id, 1);
}

#[test]
fn v2_roundtrip_crc32() {
    let md = FrameMetadata {
        flags: FrameFlags::HAS_METADATA,
        checksum_algo: ChecksumAlgorithm::Crc32Ieee,
        ..Default::default()
    };
    let payload: Vec<u8> = (0..64).collect();
    let wire = FrameBuilderV2::new()
        .with_metadata(md)
        .encode(&payload)
        .unwrap();
    let decoded = FrameBuilderV2::decode(&wire).unwrap();
    assert_eq!(decoded.payload, payload);
}
