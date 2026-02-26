use std::sync::Arc;

use crate::error::DecodeError;

pub(crate) const DEFAULT_CAPACITY: usize = 4096;
pub(crate) const MAX_POOL_SIZE: usize = 8;

#[derive(Debug, Clone)]
pub struct SharedBuffer {
    inner: Arc<Vec<u8>>,
    pub(crate) generation: u64,
}

impl SharedBuffer {
    pub fn with_capacity(capacity: usize, generation: u64) -> Self {
        Self {
            inner: Arc::new(vec![0u8; capacity]),
            generation,
        }
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn capacity(&self) -> usize {
        self.inner.len()
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.inner
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        Arc::get_mut(&mut self.inner).expect("exclusive access required")
    }

    pub fn write_at(&mut self, offset: usize, data: &[u8]) -> Result<(), DecodeError> {
        let end = offset
            .checked_add(data.len())
            .ok_or(DecodeError::Internal("write overflow"))?;
        if end > self.capacity() {
            return Err(DecodeError::Internal("write past capacity"));
        }
        self.as_mut_slice()[offset..end].copy_from_slice(data);
        Ok(())
    }

    pub fn weak_same_backing(&self, other: &SharedBuffer) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }
}
