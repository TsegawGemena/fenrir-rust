pub fn hex_encode(data: &[u8]) -> String {
    data.iter().map(|b| format!("{b:02x}")).collect()
}

pub fn hex_decode(s: &str) -> Result<Vec<u8>, &'static str> {
    if s.len() % 2 != 0 {
        return Err("odd hex length");
    }
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|_| "invalid hex"))
        .collect()
}

pub fn split_chunks(data: &[u8], chunk_size: usize) -> impl Iterator<Item = &[u8]> {
    data.chunks(chunk_size.max(1))
}

pub fn xor_bytes(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter()
        .zip(b.iter().cycle())
        .map(|(x, y)| x ^ y)
        .collect()
}
