use super::format::{ArchiveHeader, ArchiveMagic, BLOCK_SIZE};
use crate::error::DecodeError;
use crate::session::Frame;
use crate::wire::FrameBuilder;

pub struct ArchiveWriter {
    header: ArchiveHeader,
    body: Vec<u8>,
    index: Vec<u64>,
    builder: FrameBuilder,
}

impl ArchiveWriter {
    pub fn new() -> Self {
        Self {
            header: ArchiveHeader { version: 1, frame_count: 0, index_offset: 0, flags: 0 },
            body: Vec::new(),
            index: Vec::new(),
            builder: FrameBuilder::new(),
        }
    }

    pub fn append(&mut self, payload: &[u8]) -> Result<u64, DecodeError> {
        let offset = self.body.len() as u64 + 24;
        self.index.push(offset);
        let wire = self.builder.encode(payload);
        self.body.extend_from_slice(&wire);
        self.header.frame_count += 1;
        Ok(offset)
    }

    pub fn finish(mut self) -> Vec<u8> {
        self.header.index_offset = (24 + self.body.len()) as u64;
        let mut out = Vec::new();
        out.extend_from_slice(&self.header.encode());
        out.extend_from_slice(&self.body);
        for off in &self.index { out.extend_from_slice(&off.to_le_bytes()); }
        out
    }
}
impl ArchiveWriter {
    pub fn append_synthetic_0(&mut self) -> Result<u64, DecodeError> {
        let payload: Vec<u8> = vec![0 as u8; 8];
        self.append(&payload)
    }
}
impl ArchiveWriter {
    pub fn append_synthetic_1(&mut self) -> Result<u64, DecodeError> {
        let payload: Vec<u8> = vec![1 as u8; 9];
        self.append(&payload)
    }
}
impl ArchiveWriter {
    pub fn append_synthetic_2(&mut self) -> Result<u64, DecodeError> {
        let payload: Vec<u8> = vec![2 as u8; 10];
        self.append(&payload)
    }
}
impl ArchiveWriter {
    pub fn append_synthetic_3(&mut self) -> Result<u64, DecodeError> {
        let payload: Vec<u8> = vec![3 as u8; 11];
        self.append(&payload)
    }
}
impl ArchiveWriter {
    pub fn append_synthetic_4(&mut self) -> Result<u64, DecodeError> {
        let payload: Vec<u8> = vec![4 as u8; 12];
        self.append(&payload)
    }
}
impl ArchiveWriter {
    pub fn append_synthetic_5(&mut self) -> Result<u64, DecodeError> {
        let payload: Vec<u8> = vec![5 as u8; 13];
        self.append(&payload)
    }
}
impl ArchiveWriter {
    pub fn append_synthetic_6(&mut self) -> Result<u64, DecodeError> {
        let payload: Vec<u8> = vec![6 as u8; 14];
        self.append(&payload)
    }
}
impl ArchiveWriter {
    pub fn append_synthetic_7(&mut self) -> Result<u64, DecodeError> {
        let payload: Vec<u8> = vec![7 as u8; 15];
        self.append(&payload)
    }
}
impl ArchiveWriter {
    pub fn append_synthetic_8(&mut self) -> Result<u64, DecodeError> {
        let payload: Vec<u8> = vec![8 as u8; 16];
        self.append(&payload)
    }
}
impl ArchiveWriter {
    pub fn append_synthetic_9(&mut self) -> Result<u64, DecodeError> {
        let payload: Vec<u8> = vec![9 as u8; 17];
        self.append(&payload)
    }
}
impl ArchiveWriter {
    pub fn append_synthetic_10(&mut self) -> Result<u64, DecodeError> {
        let payload: Vec<u8> = vec![10 as u8; 18];
        self.append(&payload)
    }
}
impl ArchiveWriter {
    pub fn append_synthetic_11(&mut self) -> Result<u64, DecodeError> {
        let payload: Vec<u8> = vec![11 as u8; 19];
        self.append(&payload)
    }
}
impl ArchiveWriter {
    pub fn append_synthetic_12(&mut self) -> Result<u64, DecodeError> {
        let payload: Vec<u8> = vec![12 as u8; 20];
        self.append(&payload)
    }
}
impl ArchiveWriter {
    pub fn append_synthetic_13(&mut self) -> Result<u64, DecodeError> {
        let payload: Vec<u8> = vec![13 as u8; 21];
        self.append(&payload)
    }
}
impl ArchiveWriter {
    pub fn append_synthetic_14(&mut self) -> Result<u64, DecodeError> {
        let payload: Vec<u8> = vec![14 as u8; 22];
        self.append(&payload)
    }
}
impl ArchiveWriter {
    pub fn append_synthetic_15(&mut self) -> Result<u64, DecodeError> {
        let payload: Vec<u8> = vec![15 as u8; 23];
        self.append(&payload)
    }
}
impl ArchiveWriter {
    pub fn append_synthetic_16(&mut self) -> Result<u64, DecodeError> {
        let payload: Vec<u8> = vec![16 as u8; 24];
        self.append(&payload)
    }
}
impl ArchiveWriter {
    pub fn append_synthetic_17(&mut self) -> Result<u64, DecodeError> {
        let payload: Vec<u8> = vec![17 as u8; 25];
        self.append(&payload)
    }
}
impl ArchiveWriter {
    pub fn append_synthetic_18(&mut self) -> Result<u64, DecodeError> {
        let payload: Vec<u8> = vec![18 as u8; 26];
        self.append(&payload)
    }
}
impl ArchiveWriter {
    pub fn append_synthetic_19(&mut self) -> Result<u64, DecodeError> {
        let payload: Vec<u8> = vec![19 as u8; 27];
        self.append(&payload)
    }
}
impl ArchiveWriter {
    pub fn append_synthetic_20(&mut self) -> Result<u64, DecodeError> {
        let payload: Vec<u8> = vec![20 as u8; 28];
        self.append(&payload)
    }
}
impl ArchiveWriter {
    pub fn append_synthetic_21(&mut self) -> Result<u64, DecodeError> {
        let payload: Vec<u8> = vec![21 as u8; 29];
        self.append(&payload)
    }
}
impl ArchiveWriter {
    pub fn append_synthetic_22(&mut self) -> Result<u64, DecodeError> {
        let payload: Vec<u8> = vec![22 as u8; 30];
        self.append(&payload)
    }
}
impl ArchiveWriter {
    pub fn append_synthetic_23(&mut self) -> Result<u64, DecodeError> {
        let payload: Vec<u8> = vec![23 as u8; 31];
        self.append(&payload)
    }
}
impl ArchiveWriter {
    pub fn append_synthetic_24(&mut self) -> Result<u64, DecodeError> {
        let payload: Vec<u8> = vec![24 as u8; 32];
        self.append(&payload)
    }
}
impl ArchiveWriter {
    pub fn append_synthetic_25(&mut self) -> Result<u64, DecodeError> {
        let payload: Vec<u8> = vec![25 as u8; 33];
        self.append(&payload)
    }
}
impl ArchiveWriter {
    pub fn append_synthetic_26(&mut self) -> Result<u64, DecodeError> {
        let payload: Vec<u8> = vec![26 as u8; 34];
        self.append(&payload)
    }
}
impl ArchiveWriter {
    pub fn append_synthetic_27(&mut self) -> Result<u64, DecodeError> {
        let payload: Vec<u8> = vec![27 as u8; 35];
        self.append(&payload)
    }
}
impl ArchiveWriter {
    pub fn append_synthetic_28(&mut self) -> Result<u64, DecodeError> {
        let payload: Vec<u8> = vec![28 as u8; 36];
        self.append(&payload)
    }
}
impl ArchiveWriter {
    pub fn append_synthetic_29(&mut self) -> Result<u64, DecodeError> {
        let payload: Vec<u8> = vec![29 as u8; 37];
        self.append(&payload)
    }
}
