pub const TRAILER_LEN: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WireTrailer {
    pub checksum: u32,
}

impl WireTrailer {
    pub fn encode(&self, out: &mut [u8]) {
        out[0..4].copy_from_slice(&self.checksum.to_le_bytes());
    }
}
