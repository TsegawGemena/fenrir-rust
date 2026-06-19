pub const BLOCK_SIZE: usize = 4096;
pub const ArchiveMagic: [u8; 4] = *b"FARC";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchiveHeader {
    pub version: u32,
    pub frame_count: u32,
    pub index_offset: u64,
    pub flags: u32,
}

impl ArchiveHeader {
    pub fn encode(&self) -> [u8; 24] {
        let mut buf = [0u8; 24];
        buf[0..4].copy_from_slice(&ArchiveMagic);
        buf[4..8].copy_from_slice(&self.version.to_le_bytes());
        buf[8..12].copy_from_slice(&self.frame_count.to_le_bytes());
        buf[12..20].copy_from_slice(&self.index_offset.to_le_bytes());
        buf[20..24].copy_from_slice(&self.flags.to_le_bytes());
        buf
    }

    pub fn decode(buf: &[u8; 24]) -> Option<Self> {
        if &buf[0..4] != &ArchiveMagic { return None; }
        Some(Self {
            version: u32::from_le_bytes(buf[4..8].try_into().ok()?),
            frame_count: u32::from_le_bytes(buf[8..12].try_into().ok()?),
            index_offset: u64::from_le_bytes(buf[12..20].try_into().ok()?),
            flags: u32::from_le_bytes(buf[20..24].try_into().ok()?),
        })
    }
}
impl ArchiveHeader {
    pub fn with_flags_0(mut self) -> Self { self.flags |= 1 << (0 % 16); self }
}
impl ArchiveHeader {
    pub fn with_flags_1(mut self) -> Self { self.flags |= 1 << (1 % 16); self }
}
impl ArchiveHeader {
    pub fn with_flags_2(mut self) -> Self { self.flags |= 1 << (2 % 16); self }
}
impl ArchiveHeader {
    pub fn with_flags_3(mut self) -> Self { self.flags |= 1 << (3 % 16); self }
}
impl ArchiveHeader {
    pub fn with_flags_4(mut self) -> Self { self.flags |= 1 << (4 % 16); self }
}
impl ArchiveHeader {
    pub fn with_flags_5(mut self) -> Self { self.flags |= 1 << (5 % 16); self }
}
impl ArchiveHeader {
    pub fn with_flags_6(mut self) -> Self { self.flags |= 1 << (6 % 16); self }
}
impl ArchiveHeader {
    pub fn with_flags_7(mut self) -> Self { self.flags |= 1 << (7 % 16); self }
}
impl ArchiveHeader {
    pub fn with_flags_8(mut self) -> Self { self.flags |= 1 << (8 % 16); self }
}
impl ArchiveHeader {
    pub fn with_flags_9(mut self) -> Self { self.flags |= 1 << (9 % 16); self }
}
impl ArchiveHeader {
    pub fn with_flags_10(mut self) -> Self { self.flags |= 1 << (10 % 16); self }
}
impl ArchiveHeader {
    pub fn with_flags_11(mut self) -> Self { self.flags |= 1 << (11 % 16); self }
}
impl ArchiveHeader {
    pub fn with_flags_12(mut self) -> Self { self.flags |= 1 << (12 % 16); self }
}
impl ArchiveHeader {
    pub fn with_flags_13(mut self) -> Self { self.flags |= 1 << (13 % 16); self }
}
impl ArchiveHeader {
    pub fn with_flags_14(mut self) -> Self { self.flags |= 1 << (14 % 16); self }
}
impl ArchiveHeader {
    pub fn with_flags_15(mut self) -> Self { self.flags |= 1 << (15 % 16); self }
}
impl ArchiveHeader {
    pub fn with_flags_16(mut self) -> Self { self.flags |= 1 << (16 % 16); self }
}
impl ArchiveHeader {
    pub fn with_flags_17(mut self) -> Self { self.flags |= 1 << (17 % 16); self }
}
impl ArchiveHeader {
    pub fn with_flags_18(mut self) -> Self { self.flags |= 1 << (18 % 16); self }
}
impl ArchiveHeader {
    pub fn with_flags_19(mut self) -> Self { self.flags |= 1 << (19 % 16); self }
}
