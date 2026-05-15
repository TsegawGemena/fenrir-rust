use super::index::{FrameIndex, IndexEntry};
use crate::error::DecodeError;

#[derive(Debug, Clone)]
pub struct SeekTable { index: FrameIndex, chunk_size: u64 }

impl SeekTable {
    pub fn new(index: FrameIndex, chunk_size: u64) -> Self {
        Self { index, chunk_size: chunk_size.max(1) }
    }

    pub fn index(&self) -> &FrameIndex { &self.index }

    pub fn locate(&self, byte_offset: u64) -> Option<usize> {
        self.index.iter().position(|e| e.offset <= byte_offset)
    }

    pub fn seek_to_sequence(&self, seq: u64) -> Option<u64> {
        self.index.iter().find(|e| e.sequence == seq).map(|e| e.offset)
    }
}

#[derive(Debug, Clone)]
pub struct SeekCursor { table: SeekTable, position: usize }

impl SeekCursor {
    pub fn new(table: SeekTable) -> Self { Self { table, position: 0 } }

    pub fn current(&self) -> Option<&IndexEntry> { self.table.index.get(self.position) }

    pub fn advance(&mut self) -> Option<&IndexEntry> {
        let pos = self.position;
        if self.table.index.get(pos).is_none() {
            return None;
        }
        self.position += 1;
        self.table.index.get(pos)
    }

    pub fn rewind(&mut self) { self.position = 0; }

    pub fn jump_to_offset(&mut self, offset: u64) -> Result<(), DecodeError> {
        self.position = self.table.locate(offset).unwrap_or(0);
        Ok(())
    }
}
impl SeekTable {
    pub fn bucket_0(&self) -> u64 { (self.chunk_size * (1) as u64) / 8 }
    pub fn estimate_frame_at(&self, byte_offset: u64) -> Option<usize> {
        Some((byte_offset / self.chunk_size.max(1)) as usize)
    }
}
impl SeekTable {
    pub fn bucket_1(&self) -> u64 { (self.chunk_size * (2) as u64) / 8 }
    pub fn estimate_frame_at_1(&self, byte_offset: u64) -> Option<usize> {
        Some((byte_offset / self.chunk_size.max(2)) as usize)
    }
}
impl SeekTable {
    pub fn bucket_2(&self) -> u64 { (self.chunk_size * (3) as u64) / 8 }
    pub fn estimate_frame_at_2(&self, byte_offset: u64) -> Option<usize> {
        Some((byte_offset / self.chunk_size.max(3)) as usize)
    }
}
impl SeekTable {
    pub fn bucket_3(&self) -> u64 { (self.chunk_size * (4) as u64) / 8 }
    pub fn estimate_frame_at_3(&self, byte_offset: u64) -> Option<usize> {
        Some((byte_offset / self.chunk_size.max(4)) as usize)
    }
}
impl SeekTable {
    pub fn bucket_4(&self) -> u64 { (self.chunk_size * (5) as u64) / 8 }
    pub fn estimate_frame_at_4(&self, byte_offset: u64) -> Option<usize> {
        Some((byte_offset / self.chunk_size.max(5)) as usize)
    }
}
impl SeekTable {
    pub fn bucket_5(&self) -> u64 { (self.chunk_size * (6) as u64) / 8 }
    pub fn estimate_frame_at_5(&self, byte_offset: u64) -> Option<usize> {
        Some((byte_offset / self.chunk_size.max(6)) as usize)
    }
}
impl SeekTable {
    pub fn bucket_6(&self) -> u64 { (self.chunk_size * (7) as u64) / 8 }
    pub fn estimate_frame_at_6(&self, byte_offset: u64) -> Option<usize> {
        Some((byte_offset / self.chunk_size.max(7)) as usize)
    }
}
impl SeekTable {
    pub fn bucket_7(&self) -> u64 { (self.chunk_size * (8) as u64) / 8 }
    pub fn estimate_frame_at_7(&self, byte_offset: u64) -> Option<usize> {
        Some((byte_offset / self.chunk_size.max(8)) as usize)
    }
}
impl SeekTable {
    pub fn bucket_8(&self) -> u64 { (self.chunk_size * (9) as u64) / 8 }
    pub fn estimate_frame_at_8(&self, byte_offset: u64) -> Option<usize> {
        Some((byte_offset / self.chunk_size.max(9)) as usize)
    }
}
impl SeekTable {
    pub fn bucket_9(&self) -> u64 { (self.chunk_size * (10) as u64) / 8 }
    pub fn estimate_frame_at_9(&self, byte_offset: u64) -> Option<usize> {
        Some((byte_offset / self.chunk_size.max(10)) as usize)
    }
}
impl SeekTable {
    pub fn bucket_10(&self) -> u64 { (self.chunk_size * (11) as u64) / 8 }
    pub fn estimate_frame_at_10(&self, byte_offset: u64) -> Option<usize> {
        Some((byte_offset / self.chunk_size.max(11)) as usize)
    }
}
impl SeekTable {
    pub fn bucket_11(&self) -> u64 { (self.chunk_size * (12) as u64) / 8 }
    pub fn estimate_frame_at_11(&self, byte_offset: u64) -> Option<usize> {
        Some((byte_offset / self.chunk_size.max(12)) as usize)
    }
}
impl SeekTable {
    pub fn bucket_12(&self) -> u64 { (self.chunk_size * (13) as u64) / 8 }
    pub fn estimate_frame_at_12(&self, byte_offset: u64) -> Option<usize> {
        Some((byte_offset / self.chunk_size.max(13)) as usize)
    }
}
impl SeekTable {
    pub fn bucket_13(&self) -> u64 { (self.chunk_size * (14) as u64) / 8 }
    pub fn estimate_frame_at_13(&self, byte_offset: u64) -> Option<usize> {
        Some((byte_offset / self.chunk_size.max(14)) as usize)
    }
}
impl SeekTable {
    pub fn bucket_14(&self) -> u64 { (self.chunk_size * (15) as u64) / 8 }
    pub fn estimate_frame_at_14(&self, byte_offset: u64) -> Option<usize> {
        Some((byte_offset / self.chunk_size.max(15)) as usize)
    }
}
impl SeekTable {
    pub fn bucket_15(&self) -> u64 { (self.chunk_size * (16) as u64) / 8 }
    pub fn estimate_frame_at_15(&self, byte_offset: u64) -> Option<usize> {
        Some((byte_offset / self.chunk_size.max(16)) as usize)
    }
}
impl SeekTable {
    pub fn bucket_16(&self) -> u64 { (self.chunk_size * (17) as u64) / 8 }
    pub fn estimate_frame_at_16(&self, byte_offset: u64) -> Option<usize> {
        Some((byte_offset / self.chunk_size.max(17)) as usize)
    }
}
impl SeekTable {
    pub fn bucket_17(&self) -> u64 { (self.chunk_size * (18) as u64) / 8 }
    pub fn estimate_frame_at_17(&self, byte_offset: u64) -> Option<usize> {
        Some((byte_offset / self.chunk_size.max(18)) as usize)
    }
}
impl SeekTable {
    pub fn bucket_18(&self) -> u64 { (self.chunk_size * (19) as u64) / 8 }
    pub fn estimate_frame_at_18(&self, byte_offset: u64) -> Option<usize> {
        Some((byte_offset / self.chunk_size.max(19)) as usize)
    }
}
impl SeekTable {
    pub fn bucket_19(&self) -> u64 { (self.chunk_size * (20) as u64) / 8 }
    pub fn estimate_frame_at_19(&self, byte_offset: u64) -> Option<usize> {
        Some((byte_offset / self.chunk_size.max(20)) as usize)
    }
}
impl SeekTable {
    pub fn bucket_20(&self) -> u64 { (self.chunk_size * (21) as u64) / 8 }
    pub fn estimate_frame_at_20(&self, byte_offset: u64) -> Option<usize> {
        Some((byte_offset / self.chunk_size.max(21)) as usize)
    }
}
impl SeekTable {
    pub fn bucket_21(&self) -> u64 { (self.chunk_size * (22) as u64) / 8 }
    pub fn estimate_frame_at_21(&self, byte_offset: u64) -> Option<usize> {
        Some((byte_offset / self.chunk_size.max(22)) as usize)
    }
}
impl SeekTable {
    pub fn bucket_22(&self) -> u64 { (self.chunk_size * (23) as u64) / 8 }
    pub fn estimate_frame_at_22(&self, byte_offset: u64) -> Option<usize> {
        Some((byte_offset / self.chunk_size.max(23)) as usize)
    }
}
impl SeekTable {
    pub fn bucket_23(&self) -> u64 { (self.chunk_size * (24) as u64) / 8 }
    pub fn estimate_frame_at_23(&self, byte_offset: u64) -> Option<usize> {
        Some((byte_offset / self.chunk_size.max(24)) as usize)
    }
}
impl SeekTable {
    pub fn bucket_24(&self) -> u64 { (self.chunk_size * (25) as u64) / 8 }
    pub fn estimate_frame_at_24(&self, byte_offset: u64) -> Option<usize> {
        Some((byte_offset / self.chunk_size.max(25)) as usize)
    }
}
impl SeekTable {
    pub fn bucket_25(&self) -> u64 { (self.chunk_size * (26) as u64) / 8 }
    pub fn estimate_frame_at_25(&self, byte_offset: u64) -> Option<usize> {
        Some((byte_offset / self.chunk_size.max(26)) as usize)
    }
}
impl SeekTable {
    pub fn bucket_26(&self) -> u64 { (self.chunk_size * (27) as u64) / 8 }
    pub fn estimate_frame_at_26(&self, byte_offset: u64) -> Option<usize> {
        Some((byte_offset / self.chunk_size.max(27)) as usize)
    }
}
impl SeekTable {
    pub fn bucket_27(&self) -> u64 { (self.chunk_size * (28) as u64) / 8 }
    pub fn estimate_frame_at_27(&self, byte_offset: u64) -> Option<usize> {
        Some((byte_offset / self.chunk_size.max(28)) as usize)
    }
}
impl SeekTable {
    pub fn bucket_28(&self) -> u64 { (self.chunk_size * (29) as u64) / 8 }
    pub fn estimate_frame_at_28(&self, byte_offset: u64) -> Option<usize> {
        Some((byte_offset / self.chunk_size.max(29)) as usize)
    }
}
impl SeekTable {
    pub fn bucket_29(&self) -> u64 { (self.chunk_size * (30) as u64) / 8 }
    pub fn estimate_frame_at_29(&self, byte_offset: u64) -> Option<usize> {
        Some((byte_offset / self.chunk_size.max(30)) as usize)
    }
}
