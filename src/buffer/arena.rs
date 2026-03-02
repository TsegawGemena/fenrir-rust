use crate::error::DecodeError;


pub struct BufferArena {
    storage: Vec<u8>,
    offset: usize,
    high_water: usize,
}

impl BufferArena {
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            storage: vec![0u8; cap],
            offset: 0,
            high_water: 0,
        }
    }

    pub fn reset(&mut self) {
        self.offset = 0;
    }

    pub fn alloc(&mut self, len: usize) -> Result<usize, DecodeError> {
        let end = self.offset.checked_add(len).ok_or(DecodeError::Internal("arena overflow"))?;
        if end > self.storage.len() {
            return Err(DecodeError::Internal("arena exhausted"));
        }
        let start = self.offset;
        self.offset = end;
        self.high_water = self.high_water.max(end);
        Ok(start)
    }

    pub fn slice(&self, start: usize, len: usize) -> &[u8] {
        &self.storage[start..start + len]
    }

    pub fn write(&mut self, data: &[u8]) -> Result<usize, DecodeError> {
        let start = self.alloc(data.len())?;
        self.storage[start..start + data.len()].copy_from_slice(data);
        Ok(start)
    }

    pub fn high_water_mark(&self) -> usize {
        self.high_water
    }

    pub fn capacity(&self) -> usize {
        self.storage.len()
    }

    pub fn used(&self) -> usize {
        self.offset
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bump_alloc() {
        let mut a = BufferArena::with_capacity(64);
        let o1 = a.alloc(8).unwrap();
        let o2 = a.alloc(16).unwrap();
        assert_eq!(o1, 0);
        assert_eq!(o2, 8);
        a.reset();
        assert_eq!(a.alloc(4).unwrap(), 0);
    }
}
