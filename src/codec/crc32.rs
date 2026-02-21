//! IEEE CRC-32 (Ethernet polynomial 0xEDB88320).

const TABLE: [u32; 256] = {
    let mut table = [0u32; 256];
    let mut i = 0u32;
    while i < 256 {
        let mut c = i;
        let mut k = 0;
        while k < 8 {
            if c & 1 != 0 {
                c = 0xEDB8_8320 ^ (c >> 1);
            } else {
                c >>= 1;
            }
            k += 1;
        }
        table[i as usize] = c;
        i += 1;
    }
    table
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Crc32 {
    state: u32,
}

impl Default for Crc32 {
    fn default() -> Self {
        Self::new()
    }
}

impl Crc32 {
    pub const INIT: u32 = 0xFFFF_FFFF;
    pub const XOROUT: u32 = 0xFFFF_FFFF;

    pub fn new() -> Self {
        Self { state: Self::INIT }
    }

    pub fn reset(&mut self) {
        self.state = Self::INIT;
    }

    pub fn update(&mut self, data: &[u8]) {
        for &b in data {
            let idx = ((self.state ^ u32::from(b)) & 0xFF) as usize;
            self.state = TABLE[idx] ^ (self.state >> 8);
        }
    }

    pub fn finalize(self) -> u32 {
        self.state ^ Self::XOROUT
    }

    pub fn compute(data: &[u8]) -> u32 {
        let mut c = Self::new();
        c.update(data);
        c.finalize()
    }

    pub fn compute_slice(data: &[u8], offset: usize, len: usize) -> u32 {
        let end = offset.saturating_add(len).min(data.len());
        Self::compute(&data[offset..end])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert_eq!(Crc32::compute(b""), 0);
    }

    #[test]
    fn known_vector() {
        assert_eq!(Crc32::compute(b"123456789"), 0xCBF4_3926);
    }
}
