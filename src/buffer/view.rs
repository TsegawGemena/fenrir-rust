use super::shared::SharedBuffer;

#[derive(Debug, Clone)]
pub struct BufferView {
    owner: SharedBuffer,
    start: usize,
    len: usize,
    generation: u64,
    #[cfg(feature = "buggy")]
    ptr: *const u8,
}

impl BufferView {
    pub fn new(owner: &SharedBuffer, start: usize, len: usize) -> Self {
        debug_assert!(start + len <= owner.capacity());
        Self {
            owner: owner.clone(),
            start,
            len,
            generation: owner.generation(),
            #[cfg(feature = "buggy")]
            ptr: owner.as_slice().as_ptr().wrapping_add(start),
        }
    }

    pub fn bind_owner(&mut self, owner: SharedBuffer) {
        self.generation = owner.generation();
        self.owner = owner;
        #[cfg(feature = "buggy")]
        {
            self.ptr = self.owner.as_slice().as_ptr().wrapping_add(self.start);
        }
    }

    #[cfg(feature = "buggy")]
    pub fn poison_after_release(&mut self) {
        self.owner = SharedBuffer::with_capacity(0, 0);
    }

    pub fn is_stale(&self) -> bool {
        #[cfg(feature = "buggy")]
        {
            self.owner.capacity() == 0
        }
        #[cfg(not(feature = "buggy"))]
        {
            false
        }
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn as_slice(&self) -> &[u8] {
        debug_assert!(!self.is_stale(), "read through stale BufferView");
        #[cfg(not(feature = "buggy"))]
        {
            debug_assert!(
                self.start + self.len <= self.owner.capacity(),
                "view range exceeds owner capacity"
            );
            &self.owner.as_slice()[self.start..self.start + self.len]
        }
        #[cfg(feature = "buggy")]
        {
            unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
        }
    }

    pub fn into_owner(self) -> SharedBuffer {
        self.owner
    }

    pub fn same_backing_as(&self, owner: &SharedBuffer) -> bool {
        self.owner.weak_same_backing(owner)
    }
}
