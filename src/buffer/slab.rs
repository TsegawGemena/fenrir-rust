use crate::error::DecodeError;

/// Fixed-size slab for small header/trailer staging buffers.
pub struct Slab<const N: usize> {
    slots: Vec<[u8; N]>,
    free: Vec<usize>,
}

impl<const N: usize> Slab<N> {
    pub fn with_slots(count: usize) -> Self {
        Self {
            slots: (0..count).map(|_| [0u8; N]).collect(),
            free: (0..count).rev().collect(),
        }
    }

    pub fn acquire(&mut self) -> Result<usize, DecodeError> {
        self.free.pop().ok_or(DecodeError::PoolExhausted)
    }

    pub fn release(&mut self, idx: usize) {
        self.free.push(idx);
    }

    pub fn get(&self, idx: usize) -> &[u8; N] {
        &self.slots[idx]
    }

    pub fn get_mut(&mut self, idx: usize) -> &mut [u8; N] {
        &mut self.slots[idx]
    }

    pub fn available(&self) -> usize {
        self.free.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slab_roundtrip() {
        let mut s = Slab::<32>::with_slots(4);
        let i = s.acquire().unwrap();
        s.get_mut(i)[0] = 0xAB;
        s.release(i);
        assert_eq!(s.available(), 4);
    }
}
