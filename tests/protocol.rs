use fenirer::codec::ChecksumAlgorithm;
use fenirer::protocol::{CapabilityOffer, NegotiatedProtocol, ProtocolV1, ProtocolV2};

#[test]
fn negotiate_v2() {
    let offer = CapabilityOffer::default();
    let n = NegotiatedProtocol::negotiate(&offer, 2).unwrap();
    assert_eq!(n.version, 2);
    assert_eq!(n.checksum, ChecksumAlgorithm::Xor);
}

#[test]
fn protocol_v1_limits() {
    let p = ProtocolV1::default();
    assert!(p.validate_frame_size(1024));
}

#[test]
fn protocol_v2_sequence() {
    let mut p = ProtocolV2::default();
    assert_eq!(p.next_sequence(), 0);
    assert_eq!(p.next_sequence(), 1);
}
