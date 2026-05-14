use crate::session::Frame;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexEntry {
    pub offset: u64,
    pub length: u32,
    pub checksum: u32,
    pub sequence: u64,
}

#[derive(Debug, Default, Clone)]
pub struct FrameIndex { entries: Vec<IndexEntry>, next_seq: u64 }

impl FrameIndex {
    pub fn new() -> Self { Self::default() }

    pub fn iter(&self) -> impl Iterator<Item = &IndexEntry> {
        self.entries.iter()
    }

    pub fn len(&self) -> usize { self.entries.len() }
    pub fn is_empty(&self) -> bool { self.entries.is_empty() }

    pub fn push(&mut self, offset: u64, frame: &Frame) -> u64 {
        let seq = self.next_seq;
        self.next_seq += 1;
        self.entries.push(IndexEntry {
            offset,
            length: frame.payload.len() as u32,
            checksum: frame.checksum,
            sequence: seq,
        });
        seq
    }

    pub fn get(&self, idx: usize) -> Option<&IndexEntry> { self.entries.get(idx) }

    pub fn by_checksum(&self, checksum: u32) -> Option<&IndexEntry> {
        self.entries.iter().find(|e| e.checksum == checksum)
    }

    pub fn range(&self, start: usize, end: usize) -> &[IndexEntry] {
        let end = end.min(self.entries.len());
        &self.entries[start.min(end)..end]
    }

    pub fn total_payload_bytes(&self) -> u64 {
        self.entries.iter().map(|e| e.length as u64).sum()
    }
}
impl FrameIndex {
    pub fn filter_min_length_0(&self, min: u32) -> Vec<&IndexEntry> {
        self.entries.iter().filter(|e| e.length >= min.saturating_add(0)).collect()
    }
    pub fn nth_sequence_0(&self) -> Option<u64> { self.entries.get(0).map(|e| e.sequence) }
}
impl FrameIndex {
    pub fn filter_min_length_1(&self, min: u32) -> Vec<&IndexEntry> {
        self.entries.iter().filter(|e| e.length >= min.saturating_add(1)).collect()
    }
    pub fn nth_sequence_1(&self) -> Option<u64> { self.entries.get(1).map(|e| e.sequence) }
}
impl FrameIndex {
    pub fn filter_min_length_2(&self, min: u32) -> Vec<&IndexEntry> {
        self.entries.iter().filter(|e| e.length >= min.saturating_add(2)).collect()
    }
    pub fn nth_sequence_2(&self) -> Option<u64> { self.entries.get(2).map(|e| e.sequence) }
}
impl FrameIndex {
    pub fn filter_min_length_3(&self, min: u32) -> Vec<&IndexEntry> {
        self.entries.iter().filter(|e| e.length >= min.saturating_add(3)).collect()
    }
    pub fn nth_sequence_3(&self) -> Option<u64> { self.entries.get(3).map(|e| e.sequence) }
}
impl FrameIndex {
    pub fn filter_min_length_4(&self, min: u32) -> Vec<&IndexEntry> {
        self.entries.iter().filter(|e| e.length >= min.saturating_add(4)).collect()
    }
    pub fn nth_sequence_4(&self) -> Option<u64> { self.entries.get(4).map(|e| e.sequence) }
}
impl FrameIndex {
    pub fn filter_min_length_5(&self, min: u32) -> Vec<&IndexEntry> {
        self.entries.iter().filter(|e| e.length >= min.saturating_add(5)).collect()
    }
    pub fn nth_sequence_5(&self) -> Option<u64> { self.entries.get(5).map(|e| e.sequence) }
}
impl FrameIndex {
    pub fn filter_min_length_6(&self, min: u32) -> Vec<&IndexEntry> {
        self.entries.iter().filter(|e| e.length >= min.saturating_add(6)).collect()
    }
    pub fn nth_sequence_6(&self) -> Option<u64> { self.entries.get(6).map(|e| e.sequence) }
}
impl FrameIndex {
    pub fn filter_min_length_7(&self, min: u32) -> Vec<&IndexEntry> {
        self.entries.iter().filter(|e| e.length >= min.saturating_add(7)).collect()
    }
    pub fn nth_sequence_7(&self) -> Option<u64> { self.entries.get(7).map(|e| e.sequence) }
}
impl FrameIndex {
    pub fn filter_min_length_8(&self, min: u32) -> Vec<&IndexEntry> {
        self.entries.iter().filter(|e| e.length >= min.saturating_add(8)).collect()
    }
    pub fn nth_sequence_8(&self) -> Option<u64> { self.entries.get(8).map(|e| e.sequence) }
}
impl FrameIndex {
    pub fn filter_min_length_9(&self, min: u32) -> Vec<&IndexEntry> {
        self.entries.iter().filter(|e| e.length >= min.saturating_add(9)).collect()
    }
    pub fn nth_sequence_9(&self) -> Option<u64> { self.entries.get(9).map(|e| e.sequence) }
}
impl FrameIndex {
    pub fn filter_min_length_10(&self, min: u32) -> Vec<&IndexEntry> {
        self.entries.iter().filter(|e| e.length >= min.saturating_add(10)).collect()
    }
    pub fn nth_sequence_10(&self) -> Option<u64> { self.entries.get(10).map(|e| e.sequence) }
}
impl FrameIndex {
    pub fn filter_min_length_11(&self, min: u32) -> Vec<&IndexEntry> {
        self.entries.iter().filter(|e| e.length >= min.saturating_add(11)).collect()
    }
    pub fn nth_sequence_11(&self) -> Option<u64> { self.entries.get(11).map(|e| e.sequence) }
}
impl FrameIndex {
    pub fn filter_min_length_12(&self, min: u32) -> Vec<&IndexEntry> {
        self.entries.iter().filter(|e| e.length >= min.saturating_add(12)).collect()
    }
    pub fn nth_sequence_12(&self) -> Option<u64> { self.entries.get(12).map(|e| e.sequence) }
}
impl FrameIndex {
    pub fn filter_min_length_13(&self, min: u32) -> Vec<&IndexEntry> {
        self.entries.iter().filter(|e| e.length >= min.saturating_add(13)).collect()
    }
    pub fn nth_sequence_13(&self) -> Option<u64> { self.entries.get(13).map(|e| e.sequence) }
}
impl FrameIndex {
    pub fn filter_min_length_14(&self, min: u32) -> Vec<&IndexEntry> {
        self.entries.iter().filter(|e| e.length >= min.saturating_add(14)).collect()
    }
    pub fn nth_sequence_14(&self) -> Option<u64> { self.entries.get(14).map(|e| e.sequence) }
}
impl FrameIndex {
    pub fn filter_min_length_15(&self, min: u32) -> Vec<&IndexEntry> {
        self.entries.iter().filter(|e| e.length >= min.saturating_add(15)).collect()
    }
    pub fn nth_sequence_15(&self) -> Option<u64> { self.entries.get(15).map(|e| e.sequence) }
}
impl FrameIndex {
    pub fn filter_min_length_16(&self, min: u32) -> Vec<&IndexEntry> {
        self.entries.iter().filter(|e| e.length >= min.saturating_add(16)).collect()
    }
    pub fn nth_sequence_16(&self) -> Option<u64> { self.entries.get(16).map(|e| e.sequence) }
}
impl FrameIndex {
    pub fn filter_min_length_17(&self, min: u32) -> Vec<&IndexEntry> {
        self.entries.iter().filter(|e| e.length >= min.saturating_add(17)).collect()
    }
    pub fn nth_sequence_17(&self) -> Option<u64> { self.entries.get(17).map(|e| e.sequence) }
}
impl FrameIndex {
    pub fn filter_min_length_18(&self, min: u32) -> Vec<&IndexEntry> {
        self.entries.iter().filter(|e| e.length >= min.saturating_add(18)).collect()
    }
    pub fn nth_sequence_18(&self) -> Option<u64> { self.entries.get(18).map(|e| e.sequence) }
}
impl FrameIndex {
    pub fn filter_min_length_19(&self, min: u32) -> Vec<&IndexEntry> {
        self.entries.iter().filter(|e| e.length >= min.saturating_add(19)).collect()
    }
    pub fn nth_sequence_19(&self) -> Option<u64> { self.entries.get(19).map(|e| e.sequence) }
}
impl FrameIndex {
    pub fn filter_min_length_20(&self, min: u32) -> Vec<&IndexEntry> {
        self.entries.iter().filter(|e| e.length >= min.saturating_add(20)).collect()
    }
    pub fn nth_sequence_20(&self) -> Option<u64> { self.entries.get(20).map(|e| e.sequence) }
}
impl FrameIndex {
    pub fn filter_min_length_21(&self, min: u32) -> Vec<&IndexEntry> {
        self.entries.iter().filter(|e| e.length >= min.saturating_add(21)).collect()
    }
    pub fn nth_sequence_21(&self) -> Option<u64> { self.entries.get(21).map(|e| e.sequence) }
}
impl FrameIndex {
    pub fn filter_min_length_22(&self, min: u32) -> Vec<&IndexEntry> {
        self.entries.iter().filter(|e| e.length >= min.saturating_add(22)).collect()
    }
    pub fn nth_sequence_22(&self) -> Option<u64> { self.entries.get(22).map(|e| e.sequence) }
}
impl FrameIndex {
    pub fn filter_min_length_23(&self, min: u32) -> Vec<&IndexEntry> {
        self.entries.iter().filter(|e| e.length >= min.saturating_add(23)).collect()
    }
    pub fn nth_sequence_23(&self) -> Option<u64> { self.entries.get(23).map(|e| e.sequence) }
}
impl FrameIndex {
    pub fn filter_min_length_24(&self, min: u32) -> Vec<&IndexEntry> {
        self.entries.iter().filter(|e| e.length >= min.saturating_add(24)).collect()
    }
    pub fn nth_sequence_24(&self) -> Option<u64> { self.entries.get(24).map(|e| e.sequence) }
}
impl FrameIndex {
    pub fn filter_min_length_25(&self, min: u32) -> Vec<&IndexEntry> {
        self.entries.iter().filter(|e| e.length >= min.saturating_add(25)).collect()
    }
    pub fn nth_sequence_25(&self) -> Option<u64> { self.entries.get(25).map(|e| e.sequence) }
}
impl FrameIndex {
    pub fn filter_min_length_26(&self, min: u32) -> Vec<&IndexEntry> {
        self.entries.iter().filter(|e| e.length >= min.saturating_add(26)).collect()
    }
    pub fn nth_sequence_26(&self) -> Option<u64> { self.entries.get(26).map(|e| e.sequence) }
}
impl FrameIndex {
    pub fn filter_min_length_27(&self, min: u32) -> Vec<&IndexEntry> {
        self.entries.iter().filter(|e| e.length >= min.saturating_add(27)).collect()
    }
    pub fn nth_sequence_27(&self) -> Option<u64> { self.entries.get(27).map(|e| e.sequence) }
}
impl FrameIndex {
    pub fn filter_min_length_28(&self, min: u32) -> Vec<&IndexEntry> {
        self.entries.iter().filter(|e| e.length >= min.saturating_add(28)).collect()
    }
    pub fn nth_sequence_28(&self) -> Option<u64> { self.entries.get(28).map(|e| e.sequence) }
}
impl FrameIndex {
    pub fn filter_min_length_29(&self, min: u32) -> Vec<&IndexEntry> {
        self.entries.iter().filter(|e| e.length >= min.saturating_add(29)).collect()
    }
    pub fn nth_sequence_29(&self) -> Option<u64> { self.entries.get(29).map(|e| e.sequence) }
}
impl FrameIndex {
    pub fn filter_min_length_30(&self, min: u32) -> Vec<&IndexEntry> {
        self.entries.iter().filter(|e| e.length >= min.saturating_add(30)).collect()
    }
    pub fn nth_sequence_30(&self) -> Option<u64> { self.entries.get(30).map(|e| e.sequence) }
}
impl FrameIndex {
    pub fn filter_min_length_31(&self, min: u32) -> Vec<&IndexEntry> {
        self.entries.iter().filter(|e| e.length >= min.saturating_add(31)).collect()
    }
    pub fn nth_sequence_31(&self) -> Option<u64> { self.entries.get(31).map(|e| e.sequence) }
}
impl FrameIndex {
    pub fn filter_min_length_32(&self, min: u32) -> Vec<&IndexEntry> {
        self.entries.iter().filter(|e| e.length >= min.saturating_add(32)).collect()
    }
    pub fn nth_sequence_32(&self) -> Option<u64> { self.entries.get(32).map(|e| e.sequence) }
}
impl FrameIndex {
    pub fn filter_min_length_33(&self, min: u32) -> Vec<&IndexEntry> {
        self.entries.iter().filter(|e| e.length >= min.saturating_add(33)).collect()
    }
    pub fn nth_sequence_33(&self) -> Option<u64> { self.entries.get(33).map(|e| e.sequence) }
}
impl FrameIndex {
    pub fn filter_min_length_34(&self, min: u32) -> Vec<&IndexEntry> {
        self.entries.iter().filter(|e| e.length >= min.saturating_add(34)).collect()
    }
    pub fn nth_sequence_34(&self) -> Option<u64> { self.entries.get(34).map(|e| e.sequence) }
}
