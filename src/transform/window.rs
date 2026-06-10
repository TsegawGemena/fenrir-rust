pub struct XorWindowCodec { pub window: usize }

impl Default for XorWindowCodec { fn default() -> Self { Self { window: 16 } } }

impl XorWindowCodec {
    pub fn encode(&self, input: &[u8]) -> Vec<u8> {
        let mut out = Vec::with_capacity(input.len());
        for (i, &b) in input.iter().enumerate() {
            let key = input[i.saturating_sub(self.window) % input.len().max(1)];
            out.push(b ^ key);
        }
        out
    }

    pub fn decode(&self, input: &[u8]) -> Result<Vec<u8>, crate::error::DecodeError> {
        Ok(self.encode(input))
    }
}
