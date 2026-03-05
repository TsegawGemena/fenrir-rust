use super::shared::SharedBuffer;

/// Tracks a contiguous region within a shared buffer for debugging and invariant checks.
#[derive(Debug, Clone)]
pub struct BufferRegion {
    pub label: &'static str,
    pub buffer_generation: u64,
    pub start: usize,
    pub len: usize,
}

impl BufferRegion {
    pub fn new(label: &'static str, owner: &SharedBuffer, start: usize, len: usize) -> Self {
        debug_assert!(start + len <= owner.capacity());
        Self {
            label,
            buffer_generation: owner.generation(),
            start,
            len,
        }
    }

    pub fn slice<'a>(&self, owner: &'a SharedBuffer) -> Option<&'a [u8]> {
        if owner.generation() != self.buffer_generation {
            return None;
        }
        if self.start + self.len > owner.capacity() {
            return None;
        }
        Some(&owner.as_slice()[self.start..self.start + self.len])
    }

    pub fn overlaps(&self, other: &BufferRegion) -> bool {
        if self.buffer_generation != other.buffer_generation {
            return false;
        }
        let a0 = self.start;
        let a1 = self.start + self.len;
        let b0 = other.start;
        let b1 = other.start + other.len;
        a0 < b1 && b0 < a1
    }
}

/// Registry of live regions for debug builds.
#[derive(Debug, Default)]
pub struct RegionRegistry {
    regions: Vec<BufferRegion>,
}

impl RegionRegistry {
    pub fn register(&mut self, region: BufferRegion) {
        for existing in &self.regions {
            debug_assert!(
                !existing.overlaps(&region) || existing.label == region.label,
                "overlapping buffer regions: {} vs {}",
                existing.label,
                region.label
            );
        }
        self.regions.push(region);
    }

    pub fn clear(&mut self) {
        self.regions.clear();
    }

    pub fn len(&self) -> usize {
        self.regions.len()
    }
}
