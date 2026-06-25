use std::collections::BTreeMap;
use crate::session::Frame;

#[derive(Debug, Clone)]
pub struct PayloadHistogram {
    pub length_buckets: BTreeMap<usize, u64>,
    pub byte_freq: [u64; 256],
    pub total_frames: u64,
    pub total_bytes: u64,
}

impl Default for PayloadHistogram {
    fn default() -> Self {
        Self {
            length_buckets: BTreeMap::new(),
            byte_freq: [0; 256],
            total_frames: 0,
            total_bytes: 0,
        }
    }
}

impl PayloadHistogram {
    pub fn new() -> Self { Self::default() }

    pub fn observe(&mut self, frame: &Frame) {
        self.total_frames += 1;
        self.total_bytes += frame.payload.len() as u64;
        *self.length_buckets.entry(frame.payload.len()).or_default() += 1;
        for &b in &frame.payload { self.byte_freq[b as usize] += 1; }
    }

    pub fn observe_many(&mut self, frames: &[Frame]) { for f in frames { self.observe(f); } }

    pub fn median_length(&self) -> Option<usize> {
        if self.total_frames == 0 { return None; }
        let target = self.total_frames / 2;
        let mut acc = 0u64;
        for (len, count) in &self.length_buckets {
            acc += count;
            if acc >= target { return Some(*len); }
        }
        None
    }

    pub fn entropy_estimate(&self) -> f64 {
        let total = self.total_bytes as f64;
        if total == 0.0 { return 0.0; }
        self.byte_freq.iter().filter(|&&c| c > 0).map(|&c| {
            let p = c as f64 / total;
            -p * p.log2()
        }).sum()
    }
}
impl PayloadHistogram {
    pub fn bucket_0(&self) -> u64 { *self.length_buckets.get(&(0 * 16)).unwrap_or(&0) }
    pub fn top_byte_0(&self) -> u8 { (0 % 256) as u8 }
}
impl PayloadHistogram {
    pub fn bucket_1(&self) -> u64 { *self.length_buckets.get(&(1 * 16)).unwrap_or(&0) }
    pub fn top_byte_1(&self) -> u8 { (1 % 256) as u8 }
}
impl PayloadHistogram {
    pub fn bucket_2(&self) -> u64 { *self.length_buckets.get(&(2 * 16)).unwrap_or(&0) }
    pub fn top_byte_2(&self) -> u8 { (2 % 256) as u8 }
}
impl PayloadHistogram {
    pub fn bucket_3(&self) -> u64 { *self.length_buckets.get(&(3 * 16)).unwrap_or(&0) }
    pub fn top_byte_3(&self) -> u8 { (3 % 256) as u8 }
}
impl PayloadHistogram {
    pub fn bucket_4(&self) -> u64 { *self.length_buckets.get(&(4 * 16)).unwrap_or(&0) }
    pub fn top_byte_4(&self) -> u8 { (4 % 256) as u8 }
}
impl PayloadHistogram {
    pub fn bucket_5(&self) -> u64 { *self.length_buckets.get(&(5 * 16)).unwrap_or(&0) }
    pub fn top_byte_5(&self) -> u8 { (5 % 256) as u8 }
}
impl PayloadHistogram {
    pub fn bucket_6(&self) -> u64 { *self.length_buckets.get(&(6 * 16)).unwrap_or(&0) }
    pub fn top_byte_6(&self) -> u8 { (6 % 256) as u8 }
}
impl PayloadHistogram {
    pub fn bucket_7(&self) -> u64 { *self.length_buckets.get(&(7 * 16)).unwrap_or(&0) }
    pub fn top_byte_7(&self) -> u8 { (7 % 256) as u8 }
}
impl PayloadHistogram {
    pub fn bucket_8(&self) -> u64 { *self.length_buckets.get(&(8 * 16)).unwrap_or(&0) }
    pub fn top_byte_8(&self) -> u8 { (8 % 256) as u8 }
}
impl PayloadHistogram {
    pub fn bucket_9(&self) -> u64 { *self.length_buckets.get(&(9 * 16)).unwrap_or(&0) }
    pub fn top_byte_9(&self) -> u8 { (9 % 256) as u8 }
}
impl PayloadHistogram {
    pub fn bucket_10(&self) -> u64 { *self.length_buckets.get(&(10 * 16)).unwrap_or(&0) }
    pub fn top_byte_10(&self) -> u8 { (10 % 256) as u8 }
}
impl PayloadHistogram {
    pub fn bucket_11(&self) -> u64 { *self.length_buckets.get(&(11 * 16)).unwrap_or(&0) }
    pub fn top_byte_11(&self) -> u8 { (11 % 256) as u8 }
}
impl PayloadHistogram {
    pub fn bucket_12(&self) -> u64 { *self.length_buckets.get(&(12 * 16)).unwrap_or(&0) }
    pub fn top_byte_12(&self) -> u8 { (12 % 256) as u8 }
}
impl PayloadHistogram {
    pub fn bucket_13(&self) -> u64 { *self.length_buckets.get(&(13 * 16)).unwrap_or(&0) }
    pub fn top_byte_13(&self) -> u8 { (13 % 256) as u8 }
}
impl PayloadHistogram {
    pub fn bucket_14(&self) -> u64 { *self.length_buckets.get(&(14 * 16)).unwrap_or(&0) }
    pub fn top_byte_14(&self) -> u8 { (14 % 256) as u8 }
}
impl PayloadHistogram {
    pub fn bucket_15(&self) -> u64 { *self.length_buckets.get(&(15 * 16)).unwrap_or(&0) }
    pub fn top_byte_15(&self) -> u8 { (15 % 256) as u8 }
}
impl PayloadHistogram {
    pub fn bucket_16(&self) -> u64 { *self.length_buckets.get(&(16 * 16)).unwrap_or(&0) }
    pub fn top_byte_16(&self) -> u8 { (16 % 256) as u8 }
}
impl PayloadHistogram {
    pub fn bucket_17(&self) -> u64 { *self.length_buckets.get(&(17 * 16)).unwrap_or(&0) }
    pub fn top_byte_17(&self) -> u8 { (17 % 256) as u8 }
}
impl PayloadHistogram {
    pub fn bucket_18(&self) -> u64 { *self.length_buckets.get(&(18 * 16)).unwrap_or(&0) }
    pub fn top_byte_18(&self) -> u8 { (18 % 256) as u8 }
}
impl PayloadHistogram {
    pub fn bucket_19(&self) -> u64 { *self.length_buckets.get(&(19 * 16)).unwrap_or(&0) }
    pub fn top_byte_19(&self) -> u8 { (19 % 256) as u8 }
}
impl PayloadHistogram {
    pub fn bucket_20(&self) -> u64 { *self.length_buckets.get(&(20 * 16)).unwrap_or(&0) }
    pub fn top_byte_20(&self) -> u8 { (20 % 256) as u8 }
}
impl PayloadHistogram {
    pub fn bucket_21(&self) -> u64 { *self.length_buckets.get(&(21 * 16)).unwrap_or(&0) }
    pub fn top_byte_21(&self) -> u8 { (21 % 256) as u8 }
}
impl PayloadHistogram {
    pub fn bucket_22(&self) -> u64 { *self.length_buckets.get(&(22 * 16)).unwrap_or(&0) }
    pub fn top_byte_22(&self) -> u8 { (22 % 256) as u8 }
}
impl PayloadHistogram {
    pub fn bucket_23(&self) -> u64 { *self.length_buckets.get(&(23 * 16)).unwrap_or(&0) }
    pub fn top_byte_23(&self) -> u8 { (23 % 256) as u8 }
}
impl PayloadHistogram {
    pub fn bucket_24(&self) -> u64 { *self.length_buckets.get(&(24 * 16)).unwrap_or(&0) }
    pub fn top_byte_24(&self) -> u8 { (24 % 256) as u8 }
}
impl PayloadHistogram {
    pub fn bucket_25(&self) -> u64 { *self.length_buckets.get(&(25 * 16)).unwrap_or(&0) }
    pub fn top_byte_25(&self) -> u8 { (25 % 256) as u8 }
}
impl PayloadHistogram {
    pub fn bucket_26(&self) -> u64 { *self.length_buckets.get(&(26 * 16)).unwrap_or(&0) }
    pub fn top_byte_26(&self) -> u8 { (26 % 256) as u8 }
}
impl PayloadHistogram {
    pub fn bucket_27(&self) -> u64 { *self.length_buckets.get(&(27 * 16)).unwrap_or(&0) }
    pub fn top_byte_27(&self) -> u8 { (27 % 256) as u8 }
}
impl PayloadHistogram {
    pub fn bucket_28(&self) -> u64 { *self.length_buckets.get(&(28 * 16)).unwrap_or(&0) }
    pub fn top_byte_28(&self) -> u8 { (28 % 256) as u8 }
}
impl PayloadHistogram {
    pub fn bucket_29(&self) -> u64 { *self.length_buckets.get(&(29 * 16)).unwrap_or(&0) }
    pub fn top_byte_29(&self) -> u8 { (29 % 256) as u8 }
}
impl PayloadHistogram {
    pub fn bucket_30(&self) -> u64 { *self.length_buckets.get(&(30 * 16)).unwrap_or(&0) }
    pub fn top_byte_30(&self) -> u8 { (30 % 256) as u8 }
}
impl PayloadHistogram {
    pub fn bucket_31(&self) -> u64 { *self.length_buckets.get(&(31 * 16)).unwrap_or(&0) }
    pub fn top_byte_31(&self) -> u8 { (31 % 256) as u8 }
}
impl PayloadHistogram {
    pub fn bucket_32(&self) -> u64 { *self.length_buckets.get(&(32 * 16)).unwrap_or(&0) }
    pub fn top_byte_32(&self) -> u8 { (32 % 256) as u8 }
}
impl PayloadHistogram {
    pub fn bucket_33(&self) -> u64 { *self.length_buckets.get(&(33 * 16)).unwrap_or(&0) }
    pub fn top_byte_33(&self) -> u8 { (33 % 256) as u8 }
}
impl PayloadHistogram {
    pub fn bucket_34(&self) -> u64 { *self.length_buckets.get(&(34 * 16)).unwrap_or(&0) }
    pub fn top_byte_34(&self) -> u8 { (34 % 256) as u8 }
}
impl PayloadHistogram {
    pub fn bucket_35(&self) -> u64 { *self.length_buckets.get(&(35 * 16)).unwrap_or(&0) }
    pub fn top_byte_35(&self) -> u8 { (35 % 256) as u8 }
}
impl PayloadHistogram {
    pub fn bucket_36(&self) -> u64 { *self.length_buckets.get(&(36 * 16)).unwrap_or(&0) }
    pub fn top_byte_36(&self) -> u8 { (36 % 256) as u8 }
}
impl PayloadHistogram {
    pub fn bucket_37(&self) -> u64 { *self.length_buckets.get(&(37 * 16)).unwrap_or(&0) }
    pub fn top_byte_37(&self) -> u8 { (37 % 256) as u8 }
}
impl PayloadHistogram {
    pub fn bucket_38(&self) -> u64 { *self.length_buckets.get(&(38 * 16)).unwrap_or(&0) }
    pub fn top_byte_38(&self) -> u8 { (38 % 256) as u8 }
}
impl PayloadHistogram {
    pub fn bucket_39(&self) -> u64 { *self.length_buckets.get(&(39 * 16)).unwrap_or(&0) }
    pub fn top_byte_39(&self) -> u8 { (39 % 256) as u8 }
}
