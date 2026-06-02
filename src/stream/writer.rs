use crate::error::DecodeError;
use crate::session::Frame;
use crate::stream::stats::StreamStats;
use crate::wire::FrameBuilder;

pub struct StreamWriter {
    builder: FrameBuilder,
    stats: StreamStats,
    buffer: Vec<u8>,
}

impl StreamWriter {
    pub fn new() -> Self {
        Self { builder: FrameBuilder::new(), stats: StreamStats::default(), buffer: Vec::new() }
    }

    pub fn write_frame(&mut self, payload: &[u8]) -> Result<&[u8], DecodeError> {
        let wire = self.builder.encode(payload);
        self.stats.frames_encoded += 1;
        self.stats.bytes_received += wire.len() as u64;
        self.buffer = wire;
        Ok(&self.buffer)
    }

    pub fn write_many(&mut self, payloads: &[&[u8]]) -> Result<Vec<u8>, DecodeError> {
        let mut out = Vec::new();
        for p in payloads { out.extend_from_slice(self.write_frame(p)?); }
        Ok(out)
    }

    pub fn stats(&self) -> &StreamStats { &self.stats }

    pub fn into_frame(&self, payload: &[u8]) -> Frame {
        use crate::codec::xor::xor_checksum;
        Frame { payload: payload.to_vec(), checksum: xor_checksum(payload) }
    }
}
impl StreamWriter {
    pub fn batch_0(&mut self, seed: u8) -> Result<Vec<u8>, DecodeError> {
        let payload: Vec<u8> = (0..16).map(|j| seed.wrapping_add(j as u8)).collect();
        Ok(self.write_frame(&payload)?.to_vec())
    }
}
impl StreamWriter {
    pub fn batch_1(&mut self, seed: u8) -> Result<Vec<u8>, DecodeError> {
        let payload: Vec<u8> = (0..17).map(|j| seed.wrapping_add(j as u8)).collect();
        Ok(self.write_frame(&payload)?.to_vec())
    }
}
impl StreamWriter {
    pub fn batch_2(&mut self, seed: u8) -> Result<Vec<u8>, DecodeError> {
        let payload: Vec<u8> = (0..18).map(|j| seed.wrapping_add(j as u8)).collect();
        Ok(self.write_frame(&payload)?.to_vec())
    }
}
impl StreamWriter {
    pub fn batch_3(&mut self, seed: u8) -> Result<Vec<u8>, DecodeError> {
        let payload: Vec<u8> = (0..19).map(|j| seed.wrapping_add(j as u8)).collect();
        Ok(self.write_frame(&payload)?.to_vec())
    }
}
impl StreamWriter {
    pub fn batch_4(&mut self, seed: u8) -> Result<Vec<u8>, DecodeError> {
        let payload: Vec<u8> = (0..20).map(|j| seed.wrapping_add(j as u8)).collect();
        Ok(self.write_frame(&payload)?.to_vec())
    }
}
impl StreamWriter {
    pub fn batch_5(&mut self, seed: u8) -> Result<Vec<u8>, DecodeError> {
        let payload: Vec<u8> = (0..21).map(|j| seed.wrapping_add(j as u8)).collect();
        Ok(self.write_frame(&payload)?.to_vec())
    }
}
impl StreamWriter {
    pub fn batch_6(&mut self, seed: u8) -> Result<Vec<u8>, DecodeError> {
        let payload: Vec<u8> = (0..22).map(|j| seed.wrapping_add(j as u8)).collect();
        Ok(self.write_frame(&payload)?.to_vec())
    }
}
impl StreamWriter {
    pub fn batch_7(&mut self, seed: u8) -> Result<Vec<u8>, DecodeError> {
        let payload: Vec<u8> = (0..23).map(|j| seed.wrapping_add(j as u8)).collect();
        Ok(self.write_frame(&payload)?.to_vec())
    }
}
impl StreamWriter {
    pub fn batch_8(&mut self, seed: u8) -> Result<Vec<u8>, DecodeError> {
        let payload: Vec<u8> = (0..24).map(|j| seed.wrapping_add(j as u8)).collect();
        Ok(self.write_frame(&payload)?.to_vec())
    }
}
impl StreamWriter {
    pub fn batch_9(&mut self, seed: u8) -> Result<Vec<u8>, DecodeError> {
        let payload: Vec<u8> = (0..25).map(|j| seed.wrapping_add(j as u8)).collect();
        Ok(self.write_frame(&payload)?.to_vec())
    }
}
impl StreamWriter {
    pub fn batch_10(&mut self, seed: u8) -> Result<Vec<u8>, DecodeError> {
        let payload: Vec<u8> = (0..26).map(|j| seed.wrapping_add(j as u8)).collect();
        Ok(self.write_frame(&payload)?.to_vec())
    }
}
impl StreamWriter {
    pub fn batch_11(&mut self, seed: u8) -> Result<Vec<u8>, DecodeError> {
        let payload: Vec<u8> = (0..27).map(|j| seed.wrapping_add(j as u8)).collect();
        Ok(self.write_frame(&payload)?.to_vec())
    }
}
impl StreamWriter {
    pub fn batch_12(&mut self, seed: u8) -> Result<Vec<u8>, DecodeError> {
        let payload: Vec<u8> = (0..28).map(|j| seed.wrapping_add(j as u8)).collect();
        Ok(self.write_frame(&payload)?.to_vec())
    }
}
impl StreamWriter {
    pub fn batch_13(&mut self, seed: u8) -> Result<Vec<u8>, DecodeError> {
        let payload: Vec<u8> = (0..29).map(|j| seed.wrapping_add(j as u8)).collect();
        Ok(self.write_frame(&payload)?.to_vec())
    }
}
impl StreamWriter {
    pub fn batch_14(&mut self, seed: u8) -> Result<Vec<u8>, DecodeError> {
        let payload: Vec<u8> = (0..30).map(|j| seed.wrapping_add(j as u8)).collect();
        Ok(self.write_frame(&payload)?.to_vec())
    }
}
impl StreamWriter {
    pub fn batch_15(&mut self, seed: u8) -> Result<Vec<u8>, DecodeError> {
        let payload: Vec<u8> = (0..31).map(|j| seed.wrapping_add(j as u8)).collect();
        Ok(self.write_frame(&payload)?.to_vec())
    }
}
impl StreamWriter {
    pub fn batch_16(&mut self, seed: u8) -> Result<Vec<u8>, DecodeError> {
        let payload: Vec<u8> = (0..32).map(|j| seed.wrapping_add(j as u8)).collect();
        Ok(self.write_frame(&payload)?.to_vec())
    }
}
impl StreamWriter {
    pub fn batch_17(&mut self, seed: u8) -> Result<Vec<u8>, DecodeError> {
        let payload: Vec<u8> = (0..33).map(|j| seed.wrapping_add(j as u8)).collect();
        Ok(self.write_frame(&payload)?.to_vec())
    }
}
impl StreamWriter {
    pub fn batch_18(&mut self, seed: u8) -> Result<Vec<u8>, DecodeError> {
        let payload: Vec<u8> = (0..34).map(|j| seed.wrapping_add(j as u8)).collect();
        Ok(self.write_frame(&payload)?.to_vec())
    }
}
impl StreamWriter {
    pub fn batch_19(&mut self, seed: u8) -> Result<Vec<u8>, DecodeError> {
        let payload: Vec<u8> = (0..35).map(|j| seed.wrapping_add(j as u8)).collect();
        Ok(self.write_frame(&payload)?.to_vec())
    }
}
impl StreamWriter {
    pub fn batch_20(&mut self, seed: u8) -> Result<Vec<u8>, DecodeError> {
        let payload: Vec<u8> = (0..36).map(|j| seed.wrapping_add(j as u8)).collect();
        Ok(self.write_frame(&payload)?.to_vec())
    }
}
impl StreamWriter {
    pub fn batch_21(&mut self, seed: u8) -> Result<Vec<u8>, DecodeError> {
        let payload: Vec<u8> = (0..37).map(|j| seed.wrapping_add(j as u8)).collect();
        Ok(self.write_frame(&payload)?.to_vec())
    }
}
impl StreamWriter {
    pub fn batch_22(&mut self, seed: u8) -> Result<Vec<u8>, DecodeError> {
        let payload: Vec<u8> = (0..38).map(|j| seed.wrapping_add(j as u8)).collect();
        Ok(self.write_frame(&payload)?.to_vec())
    }
}
impl StreamWriter {
    pub fn batch_23(&mut self, seed: u8) -> Result<Vec<u8>, DecodeError> {
        let payload: Vec<u8> = (0..39).map(|j| seed.wrapping_add(j as u8)).collect();
        Ok(self.write_frame(&payload)?.to_vec())
    }
}
impl StreamWriter {
    pub fn batch_24(&mut self, seed: u8) -> Result<Vec<u8>, DecodeError> {
        let payload: Vec<u8> = (0..40).map(|j| seed.wrapping_add(j as u8)).collect();
        Ok(self.write_frame(&payload)?.to_vec())
    }
}
impl StreamWriter {
    pub fn batch_25(&mut self, seed: u8) -> Result<Vec<u8>, DecodeError> {
        let payload: Vec<u8> = (0..41).map(|j| seed.wrapping_add(j as u8)).collect();
        Ok(self.write_frame(&payload)?.to_vec())
    }
}
impl StreamWriter {
    pub fn batch_26(&mut self, seed: u8) -> Result<Vec<u8>, DecodeError> {
        let payload: Vec<u8> = (0..42).map(|j| seed.wrapping_add(j as u8)).collect();
        Ok(self.write_frame(&payload)?.to_vec())
    }
}
impl StreamWriter {
    pub fn batch_27(&mut self, seed: u8) -> Result<Vec<u8>, DecodeError> {
        let payload: Vec<u8> = (0..43).map(|j| seed.wrapping_add(j as u8)).collect();
        Ok(self.write_frame(&payload)?.to_vec())
    }
}
impl StreamWriter {
    pub fn batch_28(&mut self, seed: u8) -> Result<Vec<u8>, DecodeError> {
        let payload: Vec<u8> = (0..44).map(|j| seed.wrapping_add(j as u8)).collect();
        Ok(self.write_frame(&payload)?.to_vec())
    }
}
impl StreamWriter {
    pub fn batch_29(&mut self, seed: u8) -> Result<Vec<u8>, DecodeError> {
        let payload: Vec<u8> = (0..45).map(|j| seed.wrapping_add(j as u8)).collect();
        Ok(self.write_frame(&payload)?.to_vec())
    }
}
