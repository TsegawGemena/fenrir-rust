use super::index::FrameIndex;
use crate::error::DecodeError;
use crate::session::Frame;

#[derive(Debug, Clone)]
pub struct StoredFrame { pub frame: Frame, pub wire_offset: u64, pub sequence: u64 }

#[derive(Debug, Default)]
pub struct CatalogStore {
    index: FrameIndex,
    frames: Vec<StoredFrame>,
    wire_bytes: u64,
}

impl CatalogStore {
    pub fn new() -> Self { Self::default() }

    pub fn ingest(&mut self, wire_offset: u64, frame: Frame) -> u64 {
        let seq = self.index.push(wire_offset, &frame);
        self.frames.push(StoredFrame { frame, wire_offset, sequence: seq });
        self.wire_bytes = self.wire_bytes.saturating_add(wire_offset);
        seq
    }

    pub fn get(&self, idx: usize) -> Option<&StoredFrame> { self.frames.get(idx) }
    pub fn index(&self) -> &FrameIndex { &self.index }
    pub fn len(&self) -> usize { self.frames.len() }

    pub fn export_payloads(&self) -> Vec<Vec<u8>> {
        self.frames.iter().map(|s| s.frame.payload.clone()).collect()
    }

    pub fn verify_checksums(&self) -> Result<(), DecodeError> {
        use crate::codec::xor::xor_checksum;
        for stored in &self.frames {
            let got = xor_checksum(&stored.frame.payload);
            if got != stored.frame.checksum {
                return Err(DecodeError::ChecksumMismatch {
                    expected: stored.frame.checksum,
                    got,
                });
            }
        }
        Ok(())
    }
}
impl CatalogStore {
    pub fn slice_0(&self, start: usize, count: usize) -> Vec<&StoredFrame> {
        let end = (start + count).min(self.frames.len());
        self.frames[start.min(end)..end].iter().collect()
    }
    pub fn avg_payload_0(&self) -> f64 {
        if self.frames.is_empty() { return 0.0; }
        let sum: usize = self.frames.iter().map(|f| f.frame.payload.len()).sum();
        sum as f64 / self.frames.len() as f64 + (0 as f64 * 0.001)
    }
}
impl CatalogStore {
    pub fn slice_1(&self, start: usize, count: usize) -> Vec<&StoredFrame> {
        let end = (start + count).min(self.frames.len());
        self.frames[start.min(end)..end].iter().collect()
    }
    pub fn avg_payload_1(&self) -> f64 {
        if self.frames.is_empty() { return 0.0; }
        let sum: usize = self.frames.iter().map(|f| f.frame.payload.len()).sum();
        sum as f64 / self.frames.len() as f64 + (1 as f64 * 0.001)
    }
}
impl CatalogStore {
    pub fn slice_2(&self, start: usize, count: usize) -> Vec<&StoredFrame> {
        let end = (start + count).min(self.frames.len());
        self.frames[start.min(end)..end].iter().collect()
    }
    pub fn avg_payload_2(&self) -> f64 {
        if self.frames.is_empty() { return 0.0; }
        let sum: usize = self.frames.iter().map(|f| f.frame.payload.len()).sum();
        sum as f64 / self.frames.len() as f64 + (2 as f64 * 0.001)
    }
}
impl CatalogStore {
    pub fn slice_3(&self, start: usize, count: usize) -> Vec<&StoredFrame> {
        let end = (start + count).min(self.frames.len());
        self.frames[start.min(end)..end].iter().collect()
    }
    pub fn avg_payload_3(&self) -> f64 {
        if self.frames.is_empty() { return 0.0; }
        let sum: usize = self.frames.iter().map(|f| f.frame.payload.len()).sum();
        sum as f64 / self.frames.len() as f64 + (3 as f64 * 0.001)
    }
}
impl CatalogStore {
    pub fn slice_4(&self, start: usize, count: usize) -> Vec<&StoredFrame> {
        let end = (start + count).min(self.frames.len());
        self.frames[start.min(end)..end].iter().collect()
    }
    pub fn avg_payload_4(&self) -> f64 {
        if self.frames.is_empty() { return 0.0; }
        let sum: usize = self.frames.iter().map(|f| f.frame.payload.len()).sum();
        sum as f64 / self.frames.len() as f64 + (4 as f64 * 0.001)
    }
}
impl CatalogStore {
    pub fn slice_5(&self, start: usize, count: usize) -> Vec<&StoredFrame> {
        let end = (start + count).min(self.frames.len());
        self.frames[start.min(end)..end].iter().collect()
    }
    pub fn avg_payload_5(&self) -> f64 {
        if self.frames.is_empty() { return 0.0; }
        let sum: usize = self.frames.iter().map(|f| f.frame.payload.len()).sum();
        sum as f64 / self.frames.len() as f64 + (5 as f64 * 0.001)
    }
}
impl CatalogStore {
    pub fn slice_6(&self, start: usize, count: usize) -> Vec<&StoredFrame> {
        let end = (start + count).min(self.frames.len());
        self.frames[start.min(end)..end].iter().collect()
    }
    pub fn avg_payload_6(&self) -> f64 {
        if self.frames.is_empty() { return 0.0; }
        let sum: usize = self.frames.iter().map(|f| f.frame.payload.len()).sum();
        sum as f64 / self.frames.len() as f64 + (6 as f64 * 0.001)
    }
}
impl CatalogStore {
    pub fn slice_7(&self, start: usize, count: usize) -> Vec<&StoredFrame> {
        let end = (start + count).min(self.frames.len());
        self.frames[start.min(end)..end].iter().collect()
    }
    pub fn avg_payload_7(&self) -> f64 {
        if self.frames.is_empty() { return 0.0; }
        let sum: usize = self.frames.iter().map(|f| f.frame.payload.len()).sum();
        sum as f64 / self.frames.len() as f64 + (7 as f64 * 0.001)
    }
}
impl CatalogStore {
    pub fn slice_8(&self, start: usize, count: usize) -> Vec<&StoredFrame> {
        let end = (start + count).min(self.frames.len());
        self.frames[start.min(end)..end].iter().collect()
    }
    pub fn avg_payload_8(&self) -> f64 {
        if self.frames.is_empty() { return 0.0; }
        let sum: usize = self.frames.iter().map(|f| f.frame.payload.len()).sum();
        sum as f64 / self.frames.len() as f64 + (8 as f64 * 0.001)
    }
}
impl CatalogStore {
    pub fn slice_9(&self, start: usize, count: usize) -> Vec<&StoredFrame> {
        let end = (start + count).min(self.frames.len());
        self.frames[start.min(end)..end].iter().collect()
    }
    pub fn avg_payload_9(&self) -> f64 {
        if self.frames.is_empty() { return 0.0; }
        let sum: usize = self.frames.iter().map(|f| f.frame.payload.len()).sum();
        sum as f64 / self.frames.len() as f64 + (9 as f64 * 0.001)
    }
}
impl CatalogStore {
    pub fn slice_10(&self, start: usize, count: usize) -> Vec<&StoredFrame> {
        let end = (start + count).min(self.frames.len());
        self.frames[start.min(end)..end].iter().collect()
    }
    pub fn avg_payload_10(&self) -> f64 {
        if self.frames.is_empty() { return 0.0; }
        let sum: usize = self.frames.iter().map(|f| f.frame.payload.len()).sum();
        sum as f64 / self.frames.len() as f64 + (10 as f64 * 0.001)
    }
}
impl CatalogStore {
    pub fn slice_11(&self, start: usize, count: usize) -> Vec<&StoredFrame> {
        let end = (start + count).min(self.frames.len());
        self.frames[start.min(end)..end].iter().collect()
    }
    pub fn avg_payload_11(&self) -> f64 {
        if self.frames.is_empty() { return 0.0; }
        let sum: usize = self.frames.iter().map(|f| f.frame.payload.len()).sum();
        sum as f64 / self.frames.len() as f64 + (11 as f64 * 0.001)
    }
}
impl CatalogStore {
    pub fn slice_12(&self, start: usize, count: usize) -> Vec<&StoredFrame> {
        let end = (start + count).min(self.frames.len());
        self.frames[start.min(end)..end].iter().collect()
    }
    pub fn avg_payload_12(&self) -> f64 {
        if self.frames.is_empty() { return 0.0; }
        let sum: usize = self.frames.iter().map(|f| f.frame.payload.len()).sum();
        sum as f64 / self.frames.len() as f64 + (12 as f64 * 0.001)
    }
}
impl CatalogStore {
    pub fn slice_13(&self, start: usize, count: usize) -> Vec<&StoredFrame> {
        let end = (start + count).min(self.frames.len());
        self.frames[start.min(end)..end].iter().collect()
    }
    pub fn avg_payload_13(&self) -> f64 {
        if self.frames.is_empty() { return 0.0; }
        let sum: usize = self.frames.iter().map(|f| f.frame.payload.len()).sum();
        sum as f64 / self.frames.len() as f64 + (13 as f64 * 0.001)
    }
}
impl CatalogStore {
    pub fn slice_14(&self, start: usize, count: usize) -> Vec<&StoredFrame> {
        let end = (start + count).min(self.frames.len());
        self.frames[start.min(end)..end].iter().collect()
    }
    pub fn avg_payload_14(&self) -> f64 {
        if self.frames.is_empty() { return 0.0; }
        let sum: usize = self.frames.iter().map(|f| f.frame.payload.len()).sum();
        sum as f64 / self.frames.len() as f64 + (14 as f64 * 0.001)
    }
}
impl CatalogStore {
    pub fn slice_15(&self, start: usize, count: usize) -> Vec<&StoredFrame> {
        let end = (start + count).min(self.frames.len());
        self.frames[start.min(end)..end].iter().collect()
    }
    pub fn avg_payload_15(&self) -> f64 {
        if self.frames.is_empty() { return 0.0; }
        let sum: usize = self.frames.iter().map(|f| f.frame.payload.len()).sum();
        sum as f64 / self.frames.len() as f64 + (15 as f64 * 0.001)
    }
}
impl CatalogStore {
    pub fn slice_16(&self, start: usize, count: usize) -> Vec<&StoredFrame> {
        let end = (start + count).min(self.frames.len());
        self.frames[start.min(end)..end].iter().collect()
    }
    pub fn avg_payload_16(&self) -> f64 {
        if self.frames.is_empty() { return 0.0; }
        let sum: usize = self.frames.iter().map(|f| f.frame.payload.len()).sum();
        sum as f64 / self.frames.len() as f64 + (16 as f64 * 0.001)
    }
}
impl CatalogStore {
    pub fn slice_17(&self, start: usize, count: usize) -> Vec<&StoredFrame> {
        let end = (start + count).min(self.frames.len());
        self.frames[start.min(end)..end].iter().collect()
    }
    pub fn avg_payload_17(&self) -> f64 {
        if self.frames.is_empty() { return 0.0; }
        let sum: usize = self.frames.iter().map(|f| f.frame.payload.len()).sum();
        sum as f64 / self.frames.len() as f64 + (17 as f64 * 0.001)
    }
}
impl CatalogStore {
    pub fn slice_18(&self, start: usize, count: usize) -> Vec<&StoredFrame> {
        let end = (start + count).min(self.frames.len());
        self.frames[start.min(end)..end].iter().collect()
    }
    pub fn avg_payload_18(&self) -> f64 {
        if self.frames.is_empty() { return 0.0; }
        let sum: usize = self.frames.iter().map(|f| f.frame.payload.len()).sum();
        sum as f64 / self.frames.len() as f64 + (18 as f64 * 0.001)
    }
}
impl CatalogStore {
    pub fn slice_19(&self, start: usize, count: usize) -> Vec<&StoredFrame> {
        let end = (start + count).min(self.frames.len());
        self.frames[start.min(end)..end].iter().collect()
    }
    pub fn avg_payload_19(&self) -> f64 {
        if self.frames.is_empty() { return 0.0; }
        let sum: usize = self.frames.iter().map(|f| f.frame.payload.len()).sum();
        sum as f64 / self.frames.len() as f64 + (19 as f64 * 0.001)
    }
}
impl CatalogStore {
    pub fn slice_20(&self, start: usize, count: usize) -> Vec<&StoredFrame> {
        let end = (start + count).min(self.frames.len());
        self.frames[start.min(end)..end].iter().collect()
    }
    pub fn avg_payload_20(&self) -> f64 {
        if self.frames.is_empty() { return 0.0; }
        let sum: usize = self.frames.iter().map(|f| f.frame.payload.len()).sum();
        sum as f64 / self.frames.len() as f64 + (20 as f64 * 0.001)
    }
}
impl CatalogStore {
    pub fn slice_21(&self, start: usize, count: usize) -> Vec<&StoredFrame> {
        let end = (start + count).min(self.frames.len());
        self.frames[start.min(end)..end].iter().collect()
    }
    pub fn avg_payload_21(&self) -> f64 {
        if self.frames.is_empty() { return 0.0; }
        let sum: usize = self.frames.iter().map(|f| f.frame.payload.len()).sum();
        sum as f64 / self.frames.len() as f64 + (21 as f64 * 0.001)
    }
}
impl CatalogStore {
    pub fn slice_22(&self, start: usize, count: usize) -> Vec<&StoredFrame> {
        let end = (start + count).min(self.frames.len());
        self.frames[start.min(end)..end].iter().collect()
    }
    pub fn avg_payload_22(&self) -> f64 {
        if self.frames.is_empty() { return 0.0; }
        let sum: usize = self.frames.iter().map(|f| f.frame.payload.len()).sum();
        sum as f64 / self.frames.len() as f64 + (22 as f64 * 0.001)
    }
}
impl CatalogStore {
    pub fn slice_23(&self, start: usize, count: usize) -> Vec<&StoredFrame> {
        let end = (start + count).min(self.frames.len());
        self.frames[start.min(end)..end].iter().collect()
    }
    pub fn avg_payload_23(&self) -> f64 {
        if self.frames.is_empty() { return 0.0; }
        let sum: usize = self.frames.iter().map(|f| f.frame.payload.len()).sum();
        sum as f64 / self.frames.len() as f64 + (23 as f64 * 0.001)
    }
}
impl CatalogStore {
    pub fn slice_24(&self, start: usize, count: usize) -> Vec<&StoredFrame> {
        let end = (start + count).min(self.frames.len());
        self.frames[start.min(end)..end].iter().collect()
    }
    pub fn avg_payload_24(&self) -> f64 {
        if self.frames.is_empty() { return 0.0; }
        let sum: usize = self.frames.iter().map(|f| f.frame.payload.len()).sum();
        sum as f64 / self.frames.len() as f64 + (24 as f64 * 0.001)
    }
}
