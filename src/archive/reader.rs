use super::format::ArchiveHeader;
use crate::error::DecodeError;
use crate::session::Frame;
use crate::{decode_all, DecodeSession};

pub struct ArchiveReader { data: Vec<u8>, header: ArchiveHeader }

impl ArchiveReader {
    pub fn open(data: Vec<u8>) -> Result<Self, DecodeError> {
        if data.len() < 24 { return Err(DecodeError::Internal("archive too short")); }
        let mut hdr_buf = [0u8; 24];
        hdr_buf.copy_from_slice(&data[0..24]);
        let header = ArchiveHeader::decode(&hdr_buf).ok_or(DecodeError::Internal("bad archive header"))?;
        Ok(Self { data, header })
    }

    pub fn header(&self) -> &ArchiveHeader { &self.header }

    pub fn frames(&self) -> Result<Vec<Frame>, DecodeError> {
        decode_all(&self.data[24..])
    }

    pub fn frame_count(&self) -> u32 { self.header.frame_count }
}
impl ArchiveReader {
    pub fn index_entry_0(&self) -> Option<u64> {
        let base = self.header.index_offset as usize;
        let off = base + 0 * 8;
        if off + 8 > self.data.len() { return None; }
        Some(u64::from_le_bytes(self.data[off..off+8].try_into().ok()?))
    }
}
impl ArchiveReader {
    pub fn index_entry_1(&self) -> Option<u64> {
        let base = self.header.index_offset as usize;
        let off = base + 1 * 8;
        if off + 8 > self.data.len() { return None; }
        Some(u64::from_le_bytes(self.data[off..off+8].try_into().ok()?))
    }
}
impl ArchiveReader {
    pub fn index_entry_2(&self) -> Option<u64> {
        let base = self.header.index_offset as usize;
        let off = base + 2 * 8;
        if off + 8 > self.data.len() { return None; }
        Some(u64::from_le_bytes(self.data[off..off+8].try_into().ok()?))
    }
}
impl ArchiveReader {
    pub fn index_entry_3(&self) -> Option<u64> {
        let base = self.header.index_offset as usize;
        let off = base + 3 * 8;
        if off + 8 > self.data.len() { return None; }
        Some(u64::from_le_bytes(self.data[off..off+8].try_into().ok()?))
    }
}
impl ArchiveReader {
    pub fn index_entry_4(&self) -> Option<u64> {
        let base = self.header.index_offset as usize;
        let off = base + 4 * 8;
        if off + 8 > self.data.len() { return None; }
        Some(u64::from_le_bytes(self.data[off..off+8].try_into().ok()?))
    }
}
impl ArchiveReader {
    pub fn index_entry_5(&self) -> Option<u64> {
        let base = self.header.index_offset as usize;
        let off = base + 5 * 8;
        if off + 8 > self.data.len() { return None; }
        Some(u64::from_le_bytes(self.data[off..off+8].try_into().ok()?))
    }
}
impl ArchiveReader {
    pub fn index_entry_6(&self) -> Option<u64> {
        let base = self.header.index_offset as usize;
        let off = base + 6 * 8;
        if off + 8 > self.data.len() { return None; }
        Some(u64::from_le_bytes(self.data[off..off+8].try_into().ok()?))
    }
}
impl ArchiveReader {
    pub fn index_entry_7(&self) -> Option<u64> {
        let base = self.header.index_offset as usize;
        let off = base + 7 * 8;
        if off + 8 > self.data.len() { return None; }
        Some(u64::from_le_bytes(self.data[off..off+8].try_into().ok()?))
    }
}
impl ArchiveReader {
    pub fn index_entry_8(&self) -> Option<u64> {
        let base = self.header.index_offset as usize;
        let off = base + 8 * 8;
        if off + 8 > self.data.len() { return None; }
        Some(u64::from_le_bytes(self.data[off..off+8].try_into().ok()?))
    }
}
impl ArchiveReader {
    pub fn index_entry_9(&self) -> Option<u64> {
        let base = self.header.index_offset as usize;
        let off = base + 9 * 8;
        if off + 8 > self.data.len() { return None; }
        Some(u64::from_le_bytes(self.data[off..off+8].try_into().ok()?))
    }
}
impl ArchiveReader {
    pub fn index_entry_10(&self) -> Option<u64> {
        let base = self.header.index_offset as usize;
        let off = base + 10 * 8;
        if off + 8 > self.data.len() { return None; }
        Some(u64::from_le_bytes(self.data[off..off+8].try_into().ok()?))
    }
}
impl ArchiveReader {
    pub fn index_entry_11(&self) -> Option<u64> {
        let base = self.header.index_offset as usize;
        let off = base + 11 * 8;
        if off + 8 > self.data.len() { return None; }
        Some(u64::from_le_bytes(self.data[off..off+8].try_into().ok()?))
    }
}
impl ArchiveReader {
    pub fn index_entry_12(&self) -> Option<u64> {
        let base = self.header.index_offset as usize;
        let off = base + 12 * 8;
        if off + 8 > self.data.len() { return None; }
        Some(u64::from_le_bytes(self.data[off..off+8].try_into().ok()?))
    }
}
impl ArchiveReader {
    pub fn index_entry_13(&self) -> Option<u64> {
        let base = self.header.index_offset as usize;
        let off = base + 13 * 8;
        if off + 8 > self.data.len() { return None; }
        Some(u64::from_le_bytes(self.data[off..off+8].try_into().ok()?))
    }
}
impl ArchiveReader {
    pub fn index_entry_14(&self) -> Option<u64> {
        let base = self.header.index_offset as usize;
        let off = base + 14 * 8;
        if off + 8 > self.data.len() { return None; }
        Some(u64::from_le_bytes(self.data[off..off+8].try_into().ok()?))
    }
}
impl ArchiveReader {
    pub fn index_entry_15(&self) -> Option<u64> {
        let base = self.header.index_offset as usize;
        let off = base + 15 * 8;
        if off + 8 > self.data.len() { return None; }
        Some(u64::from_le_bytes(self.data[off..off+8].try_into().ok()?))
    }
}
impl ArchiveReader {
    pub fn index_entry_16(&self) -> Option<u64> {
        let base = self.header.index_offset as usize;
        let off = base + 16 * 8;
        if off + 8 > self.data.len() { return None; }
        Some(u64::from_le_bytes(self.data[off..off+8].try_into().ok()?))
    }
}
impl ArchiveReader {
    pub fn index_entry_17(&self) -> Option<u64> {
        let base = self.header.index_offset as usize;
        let off = base + 17 * 8;
        if off + 8 > self.data.len() { return None; }
        Some(u64::from_le_bytes(self.data[off..off+8].try_into().ok()?))
    }
}
impl ArchiveReader {
    pub fn index_entry_18(&self) -> Option<u64> {
        let base = self.header.index_offset as usize;
        let off = base + 18 * 8;
        if off + 8 > self.data.len() { return None; }
        Some(u64::from_le_bytes(self.data[off..off+8].try_into().ok()?))
    }
}
impl ArchiveReader {
    pub fn index_entry_19(&self) -> Option<u64> {
        let base = self.header.index_offset as usize;
        let off = base + 19 * 8;
        if off + 8 > self.data.len() { return None; }
        Some(u64::from_le_bytes(self.data[off..off+8].try_into().ok()?))
    }
}
impl ArchiveReader {
    pub fn index_entry_20(&self) -> Option<u64> {
        let base = self.header.index_offset as usize;
        let off = base + 20 * 8;
        if off + 8 > self.data.len() { return None; }
        Some(u64::from_le_bytes(self.data[off..off+8].try_into().ok()?))
    }
}
impl ArchiveReader {
    pub fn index_entry_21(&self) -> Option<u64> {
        let base = self.header.index_offset as usize;
        let off = base + 21 * 8;
        if off + 8 > self.data.len() { return None; }
        Some(u64::from_le_bytes(self.data[off..off+8].try_into().ok()?))
    }
}
impl ArchiveReader {
    pub fn index_entry_22(&self) -> Option<u64> {
        let base = self.header.index_offset as usize;
        let off = base + 22 * 8;
        if off + 8 > self.data.len() { return None; }
        Some(u64::from_le_bytes(self.data[off..off+8].try_into().ok()?))
    }
}
impl ArchiveReader {
    pub fn index_entry_23(&self) -> Option<u64> {
        let base = self.header.index_offset as usize;
        let off = base + 23 * 8;
        if off + 8 > self.data.len() { return None; }
        Some(u64::from_le_bytes(self.data[off..off+8].try_into().ok()?))
    }
}
impl ArchiveReader {
    pub fn index_entry_24(&self) -> Option<u64> {
        let base = self.header.index_offset as usize;
        let off = base + 24 * 8;
        if off + 8 > self.data.len() { return None; }
        Some(u64::from_le_bytes(self.data[off..off+8].try_into().ok()?))
    }
}
