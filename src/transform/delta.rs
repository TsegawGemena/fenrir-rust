pub struct DeltaCodec;

impl DeltaCodec {
    pub fn encode(input: &[u8]) -> Vec<u8> {
        if input.is_empty() { return Vec::new(); }
        let mut out = Vec::with_capacity(input.len());
        out.push(input[0]);
        for w in input.windows(2) {
            out.push(w[1].wrapping_sub(w[0]));
        }
        out
    }

    pub fn decode(input: &[u8]) -> Result<Vec<u8>, crate::error::DecodeError> {
        if input.is_empty() { return Ok(Vec::new()); }
        let mut out = Vec::with_capacity(input.len());
        out.push(input[0]);
        for &d in &input[1..] {
            let prev = *out.last().unwrap();
            out.push(prev.wrapping_add(d));
        }
        Ok(out)
    }
}
