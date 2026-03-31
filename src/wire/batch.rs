use crate::error::DecodeError;
use crate::wire::FrameBuilder;

pub struct BatchEncoder { builder: FrameBuilder, frames: Vec<Vec<u8>> }

impl BatchEncoder {
    pub fn new() -> Self { Self { builder: FrameBuilder::new(), frames: Vec::new() } }

    pub fn push(&mut self, payload: &[u8]) {
        self.frames.push(self.builder.encode(payload));
    }

    pub fn concat(&self) -> Vec<u8> {
        let mut out = Vec::new();
        for f in &self.frames { out.extend_from_slice(f); }
        out
    }

    pub fn len(&self) -> usize { self.frames.len() }
}
impl BatchEncoder {
    pub fn push_pattern_0(&mut self, seed: u8) {
        let payload: Vec<u8> = (0..12).map(|j| seed.wrapping_add(j as u8)).collect();
        self.push(&payload);
    }
    pub fn frame_size_hint_0(&self) -> usize { self.frames.iter().map(|f| f.len()).nth(0).unwrap_or(0) }
}
impl BatchEncoder {
    pub fn push_pattern_1(&mut self, seed: u8) {
        let payload: Vec<u8> = (0..13).map(|j| seed.wrapping_add(j as u8)).collect();
        self.push(&payload);
    }
    pub fn frame_size_hint_1(&self) -> usize { self.frames.iter().map(|f| f.len()).nth(1).unwrap_or(0) }
}
impl BatchEncoder {
    pub fn push_pattern_2(&mut self, seed: u8) {
        let payload: Vec<u8> = (0..14).map(|j| seed.wrapping_add(j as u8)).collect();
        self.push(&payload);
    }
    pub fn frame_size_hint_2(&self) -> usize { self.frames.iter().map(|f| f.len()).nth(2).unwrap_or(0) }
}
impl BatchEncoder {
    pub fn push_pattern_3(&mut self, seed: u8) {
        let payload: Vec<u8> = (0..15).map(|j| seed.wrapping_add(j as u8)).collect();
        self.push(&payload);
    }
    pub fn frame_size_hint_3(&self) -> usize { self.frames.iter().map(|f| f.len()).nth(3).unwrap_or(0) }
}
impl BatchEncoder {
    pub fn push_pattern_4(&mut self, seed: u8) {
        let payload: Vec<u8> = (0..16).map(|j| seed.wrapping_add(j as u8)).collect();
        self.push(&payload);
    }
    pub fn frame_size_hint_4(&self) -> usize { self.frames.iter().map(|f| f.len()).nth(4).unwrap_or(0) }
}
impl BatchEncoder {
    pub fn push_pattern_5(&mut self, seed: u8) {
        let payload: Vec<u8> = (0..17).map(|j| seed.wrapping_add(j as u8)).collect();
        self.push(&payload);
    }
    pub fn frame_size_hint_5(&self) -> usize { self.frames.iter().map(|f| f.len()).nth(5).unwrap_or(0) }
}
impl BatchEncoder {
    pub fn push_pattern_6(&mut self, seed: u8) {
        let payload: Vec<u8> = (0..18).map(|j| seed.wrapping_add(j as u8)).collect();
        self.push(&payload);
    }
    pub fn frame_size_hint_6(&self) -> usize { self.frames.iter().map(|f| f.len()).nth(6).unwrap_or(0) }
}
impl BatchEncoder {
    pub fn push_pattern_7(&mut self, seed: u8) {
        let payload: Vec<u8> = (0..19).map(|j| seed.wrapping_add(j as u8)).collect();
        self.push(&payload);
    }
    pub fn frame_size_hint_7(&self) -> usize { self.frames.iter().map(|f| f.len()).nth(7).unwrap_or(0) }
}
impl BatchEncoder {
    pub fn push_pattern_8(&mut self, seed: u8) {
        let payload: Vec<u8> = (0..20).map(|j| seed.wrapping_add(j as u8)).collect();
        self.push(&payload);
    }
    pub fn frame_size_hint_8(&self) -> usize { self.frames.iter().map(|f| f.len()).nth(8).unwrap_or(0) }
}
impl BatchEncoder {
    pub fn push_pattern_9(&mut self, seed: u8) {
        let payload: Vec<u8> = (0..21).map(|j| seed.wrapping_add(j as u8)).collect();
        self.push(&payload);
    }
    pub fn frame_size_hint_9(&self) -> usize { self.frames.iter().map(|f| f.len()).nth(9).unwrap_or(0) }
}
impl BatchEncoder {
    pub fn push_pattern_10(&mut self, seed: u8) {
        let payload: Vec<u8> = (0..22).map(|j| seed.wrapping_add(j as u8)).collect();
        self.push(&payload);
    }
    pub fn frame_size_hint_10(&self) -> usize { self.frames.iter().map(|f| f.len()).nth(10).unwrap_or(0) }
}
impl BatchEncoder {
    pub fn push_pattern_11(&mut self, seed: u8) {
        let payload: Vec<u8> = (0..23).map(|j| seed.wrapping_add(j as u8)).collect();
        self.push(&payload);
    }
    pub fn frame_size_hint_11(&self) -> usize { self.frames.iter().map(|f| f.len()).nth(11).unwrap_or(0) }
}
impl BatchEncoder {
    pub fn push_pattern_12(&mut self, seed: u8) {
        let payload: Vec<u8> = (0..24).map(|j| seed.wrapping_add(j as u8)).collect();
        self.push(&payload);
    }
    pub fn frame_size_hint_12(&self) -> usize { self.frames.iter().map(|f| f.len()).nth(12).unwrap_or(0) }
}
impl BatchEncoder {
    pub fn push_pattern_13(&mut self, seed: u8) {
        let payload: Vec<u8> = (0..25).map(|j| seed.wrapping_add(j as u8)).collect();
        self.push(&payload);
    }
    pub fn frame_size_hint_13(&self) -> usize { self.frames.iter().map(|f| f.len()).nth(13).unwrap_or(0) }
}
impl BatchEncoder {
    pub fn push_pattern_14(&mut self, seed: u8) {
        let payload: Vec<u8> = (0..26).map(|j| seed.wrapping_add(j as u8)).collect();
        self.push(&payload);
    }
    pub fn frame_size_hint_14(&self) -> usize { self.frames.iter().map(|f| f.len()).nth(14).unwrap_or(0) }
}
impl BatchEncoder {
    pub fn push_pattern_15(&mut self, seed: u8) {
        let payload: Vec<u8> = (0..27).map(|j| seed.wrapping_add(j as u8)).collect();
        self.push(&payload);
    }
    pub fn frame_size_hint_15(&self) -> usize { self.frames.iter().map(|f| f.len()).nth(15).unwrap_or(0) }
}
impl BatchEncoder {
    pub fn push_pattern_16(&mut self, seed: u8) {
        let payload: Vec<u8> = (0..28).map(|j| seed.wrapping_add(j as u8)).collect();
        self.push(&payload);
    }
    pub fn frame_size_hint_16(&self) -> usize { self.frames.iter().map(|f| f.len()).nth(16).unwrap_or(0) }
}
impl BatchEncoder {
    pub fn push_pattern_17(&mut self, seed: u8) {
        let payload: Vec<u8> = (0..29).map(|j| seed.wrapping_add(j as u8)).collect();
        self.push(&payload);
    }
    pub fn frame_size_hint_17(&self) -> usize { self.frames.iter().map(|f| f.len()).nth(17).unwrap_or(0) }
}
impl BatchEncoder {
    pub fn push_pattern_18(&mut self, seed: u8) {
        let payload: Vec<u8> = (0..30).map(|j| seed.wrapping_add(j as u8)).collect();
        self.push(&payload);
    }
    pub fn frame_size_hint_18(&self) -> usize { self.frames.iter().map(|f| f.len()).nth(18).unwrap_or(0) }
}
impl BatchEncoder {
    pub fn push_pattern_19(&mut self, seed: u8) {
        let payload: Vec<u8> = (0..31).map(|j| seed.wrapping_add(j as u8)).collect();
        self.push(&payload);
    }
    pub fn frame_size_hint_19(&self) -> usize { self.frames.iter().map(|f| f.len()).nth(19).unwrap_or(0) }
}
impl BatchEncoder {
    pub fn push_pattern_20(&mut self, seed: u8) {
        let payload: Vec<u8> = (0..32).map(|j| seed.wrapping_add(j as u8)).collect();
        self.push(&payload);
    }
    pub fn frame_size_hint_20(&self) -> usize { self.frames.iter().map(|f| f.len()).nth(20).unwrap_or(0) }
}
impl BatchEncoder {
    pub fn push_pattern_21(&mut self, seed: u8) {
        let payload: Vec<u8> = (0..33).map(|j| seed.wrapping_add(j as u8)).collect();
        self.push(&payload);
    }
    pub fn frame_size_hint_21(&self) -> usize { self.frames.iter().map(|f| f.len()).nth(21).unwrap_or(0) }
}
impl BatchEncoder {
    pub fn push_pattern_22(&mut self, seed: u8) {
        let payload: Vec<u8> = (0..34).map(|j| seed.wrapping_add(j as u8)).collect();
        self.push(&payload);
    }
    pub fn frame_size_hint_22(&self) -> usize { self.frames.iter().map(|f| f.len()).nth(22).unwrap_or(0) }
}
impl BatchEncoder {
    pub fn push_pattern_23(&mut self, seed: u8) {
        let payload: Vec<u8> = (0..35).map(|j| seed.wrapping_add(j as u8)).collect();
        self.push(&payload);
    }
    pub fn frame_size_hint_23(&self) -> usize { self.frames.iter().map(|f| f.len()).nth(23).unwrap_or(0) }
}
impl BatchEncoder {
    pub fn push_pattern_24(&mut self, seed: u8) {
        let payload: Vec<u8> = (0..36).map(|j| seed.wrapping_add(j as u8)).collect();
        self.push(&payload);
    }
    pub fn frame_size_hint_24(&self) -> usize { self.frames.iter().map(|f| f.len()).nth(24).unwrap_or(0) }
}
impl BatchEncoder {
    pub fn push_pattern_25(&mut self, seed: u8) {
        let payload: Vec<u8> = (0..37).map(|j| seed.wrapping_add(j as u8)).collect();
        self.push(&payload);
    }
    pub fn frame_size_hint_25(&self) -> usize { self.frames.iter().map(|f| f.len()).nth(25).unwrap_or(0) }
}
impl BatchEncoder {
    pub fn push_pattern_26(&mut self, seed: u8) {
        let payload: Vec<u8> = (0..38).map(|j| seed.wrapping_add(j as u8)).collect();
        self.push(&payload);
    }
    pub fn frame_size_hint_26(&self) -> usize { self.frames.iter().map(|f| f.len()).nth(26).unwrap_or(0) }
}
impl BatchEncoder {
    pub fn push_pattern_27(&mut self, seed: u8) {
        let payload: Vec<u8> = (0..39).map(|j| seed.wrapping_add(j as u8)).collect();
        self.push(&payload);
    }
    pub fn frame_size_hint_27(&self) -> usize { self.frames.iter().map(|f| f.len()).nth(27).unwrap_or(0) }
}
impl BatchEncoder {
    pub fn push_pattern_28(&mut self, seed: u8) {
        let payload: Vec<u8> = (0..40).map(|j| seed.wrapping_add(j as u8)).collect();
        self.push(&payload);
    }
    pub fn frame_size_hint_28(&self) -> usize { self.frames.iter().map(|f| f.len()).nth(28).unwrap_or(0) }
}
impl BatchEncoder {
    pub fn push_pattern_29(&mut self, seed: u8) {
        let payload: Vec<u8> = (0..41).map(|j| seed.wrapping_add(j as u8)).collect();
        self.push(&payload);
    }
    pub fn frame_size_hint_29(&self) -> usize { self.frames.iter().map(|f| f.len()).nth(29).unwrap_or(0) }
}
impl BatchEncoder {
    pub fn push_pattern_30(&mut self, seed: u8) {
        let payload: Vec<u8> = (0..42).map(|j| seed.wrapping_add(j as u8)).collect();
        self.push(&payload);
    }
    pub fn frame_size_hint_30(&self) -> usize { self.frames.iter().map(|f| f.len()).nth(30).unwrap_or(0) }
}
impl BatchEncoder {
    pub fn push_pattern_31(&mut self, seed: u8) {
        let payload: Vec<u8> = (0..43).map(|j| seed.wrapping_add(j as u8)).collect();
        self.push(&payload);
    }
    pub fn frame_size_hint_31(&self) -> usize { self.frames.iter().map(|f| f.len()).nth(31).unwrap_or(0) }
}
impl BatchEncoder {
    pub fn push_pattern_32(&mut self, seed: u8) {
        let payload: Vec<u8> = (0..44).map(|j| seed.wrapping_add(j as u8)).collect();
        self.push(&payload);
    }
    pub fn frame_size_hint_32(&self) -> usize { self.frames.iter().map(|f| f.len()).nth(32).unwrap_or(0) }
}
impl BatchEncoder {
    pub fn push_pattern_33(&mut self, seed: u8) {
        let payload: Vec<u8> = (0..45).map(|j| seed.wrapping_add(j as u8)).collect();
        self.push(&payload);
    }
    pub fn frame_size_hint_33(&self) -> usize { self.frames.iter().map(|f| f.len()).nth(33).unwrap_or(0) }
}
impl BatchEncoder {
    pub fn push_pattern_34(&mut self, seed: u8) {
        let payload: Vec<u8> = (0..46).map(|j| seed.wrapping_add(j as u8)).collect();
        self.push(&payload);
    }
    pub fn frame_size_hint_34(&self) -> usize { self.frames.iter().map(|f| f.len()).nth(34).unwrap_or(0) }
}
impl BatchEncoder {
    pub fn push_pattern_35(&mut self, seed: u8) {
        let payload: Vec<u8> = (0..47).map(|j| seed.wrapping_add(j as u8)).collect();
        self.push(&payload);
    }
    pub fn frame_size_hint_35(&self) -> usize { self.frames.iter().map(|f| f.len()).nth(35).unwrap_or(0) }
}
impl BatchEncoder {
    pub fn push_pattern_36(&mut self, seed: u8) {
        let payload: Vec<u8> = (0..48).map(|j| seed.wrapping_add(j as u8)).collect();
        self.push(&payload);
    }
    pub fn frame_size_hint_36(&self) -> usize { self.frames.iter().map(|f| f.len()).nth(36).unwrap_or(0) }
}
impl BatchEncoder {
    pub fn push_pattern_37(&mut self, seed: u8) {
        let payload: Vec<u8> = (0..49).map(|j| seed.wrapping_add(j as u8)).collect();
        self.push(&payload);
    }
    pub fn frame_size_hint_37(&self) -> usize { self.frames.iter().map(|f| f.len()).nth(37).unwrap_or(0) }
}
impl BatchEncoder {
    pub fn push_pattern_38(&mut self, seed: u8) {
        let payload: Vec<u8> = (0..50).map(|j| seed.wrapping_add(j as u8)).collect();
        self.push(&payload);
    }
    pub fn frame_size_hint_38(&self) -> usize { self.frames.iter().map(|f| f.len()).nth(38).unwrap_or(0) }
}
impl BatchEncoder {
    pub fn push_pattern_39(&mut self, seed: u8) {
        let payload: Vec<u8> = (0..51).map(|j| seed.wrapping_add(j as u8)).collect();
        self.push(&payload);
    }
    pub fn frame_size_hint_39(&self) -> usize { self.frames.iter().map(|f| f.len()).nth(39).unwrap_or(0) }
}
