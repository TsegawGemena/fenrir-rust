use super::shared::{SharedBuffer, DEFAULT_CAPACITY, MAX_POOL_SIZE};
use super::view::BufferView;
use crate::error::DecodeError;

#[derive(Debug, Default)]
pub struct BufferPool {
    free: Vec<SharedBuffer>,
    next_generation: u64,
    capacity: usize,
}

impl BufferPool {
    pub fn new() -> Self {
        Self {
            free: Vec::new(),
            next_generation: 1,
            capacity: DEFAULT_CAPACITY,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            free: Vec::new(),
            next_generation: 1,
            capacity,
        }
    }

    pub fn acquire(&mut self) -> Result<SharedBuffer, DecodeError> {
        if let Some(mut buf) = self.free.pop() {
            self.next_generation = self.next_generation.wrapping_add(1);
            buf.generation = self.next_generation;
            Ok(buf)
        } else if self.free.len() + 1 <= MAX_POOL_SIZE {
            let gen = self.next_generation;
            self.next_generation = self.next_generation.wrapping_add(1);
            Ok(SharedBuffer::with_capacity(self.capacity, gen))
        } else {
            Err(DecodeError::PoolExhausted)
        }
    }

    pub fn acquire_sized(&mut self, min_capacity: usize) -> Result<SharedBuffer, DecodeError> {
        if min_capacity <= self.capacity {
            return self.acquire();
        }
        let gen = self.next_generation;
        self.next_generation = self.next_generation.wrapping_add(1);
        Ok(SharedBuffer::with_capacity(min_capacity, gen))
    }

    pub fn release(&mut self, buf: SharedBuffer) {
        if self.free.len() < MAX_POOL_SIZE {
            self.free.push(buf);
        }
    }

    pub fn view_is_valid(&self, view: &BufferView, owner: &SharedBuffer) -> bool {
        if view.is_stale() {
            return false;
        }
        #[cfg(feature = "buggy")]
        {
            view.generation() == owner.generation()
        }
        #[cfg(not(feature = "buggy"))]
        {
            view.same_backing_as(owner)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generation_bumps_on_reuse() {
        let mut pool = BufferPool::new();
        let a = pool.acquire().unwrap();
        let gen = a.generation();
        pool.release(a);
        let b = pool.acquire().unwrap();
        assert_ne!(gen, b.generation());
    }
}
