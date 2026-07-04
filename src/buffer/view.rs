use super::shared::SharedBuffer;

/// Zero-copy slice into a pooled [`SharedBuffer`].
///
/// Views track the buffer generation so stale reads are rejected after the pool
/// reuses backing memory during incremental decode.
#[derive(Debug, Clone)]
pub struct BufferView {
    owner: SharedBuffer,
    start: usize,
    len: usize,
    generation: u64,
}

impl BufferView {
    pub fn new(owner: &SharedBuffer, start: usize, len: usize) -> Self {
        debug_assert!(start + len <= owner.capacity());
        Self {
            owner: owner.clone(),
            start,
            len,
            generation: owner.generation(),
        }
    }

    pub fn bind_owner(&mut self, owner: SharedBuffer) {
        self.generation = owner.generation();
        self.owner = owner;
    }

    pub fn is_stale(&self) -> bool {
        self.generation != self.owner.generation()
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn as_slice(&self) -> &[u8] {
        debug_assert!(!self.is_stale(), "read through stale BufferView");
        debug_assert!(
            self.start + self.len <= self.owner.capacity(),
            "view range exceeds owner capacity"
        );
        &self.owner.as_slice()[self.start..self.start + self.len]
    }

    pub fn into_owner(self) -> SharedBuffer {
        self.owner
    }

    pub fn same_backing_as(&self, owner: &SharedBuffer) -> bool {
        self.owner.weak_same_backing(owner)
    }
}
