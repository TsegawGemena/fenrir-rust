fn main() {
    let payload: Vec<u8> = (0..4096).map(|i| (i % 256) as u8).collect();
    let wire = fenirer::wire::FrameBuilder::new().encode(&payload);
    let start = std::time::Instant::now();
    for _ in 0..10_000 {
        fenirer::decode_all(&wire).unwrap();
    }
    eprintln!("single-pass 10k frames: {:?}", start.elapsed());
}
