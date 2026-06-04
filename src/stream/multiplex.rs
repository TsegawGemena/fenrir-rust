use crate::error::DecodeError;
use crate::session::{DecodeEvent, Frame};
use crate::stream::decoder::StreamDecoder;

pub struct MultiplexStream {
    lanes: Vec<StreamDecoder>,
    round_robin: usize,
}

impl MultiplexStream {
    pub fn new(lane_count: usize, pool_capacity: usize) -> Self {
        Self {
            lanes: (0..lane_count.max(1)).map(|_| StreamDecoder::new(pool_capacity)).collect(),
            round_robin: 0,
        }
    }

    pub fn push_lane(&mut self, lane: usize, chunk: &[u8]) -> Result<Vec<Frame>, DecodeError> {
        let idx = lane % self.lanes.len();
        self.lanes[idx].push(chunk)
    }

    pub fn push_round_robin(&mut self, chunk: &[u8]) -> Result<Vec<Frame>, DecodeError> {
        let idx = self.round_robin % self.lanes.len();
        self.round_robin += 1;
        self.lanes[idx].push(chunk)
    }

    pub fn lane_count(&self) -> usize { self.lanes.len() }

    pub fn drain_all(&mut self) -> Result<Vec<Frame>, DecodeError> {
        let mut all = Vec::new();
        for lane in &mut self.lanes { all.extend(lane.close()?); }
        Ok(all)
    }
}
impl MultiplexStream {
    pub fn stats_lane_0(&self) -> Option<u64> {
        self.lanes.get(0).map(|l| l.stats().frames_decoded)
    }
}
impl MultiplexStream {
    pub fn stats_lane_1(&self) -> Option<u64> {
        self.lanes.get(1).map(|l| l.stats().frames_decoded)
    }
}
impl MultiplexStream {
    pub fn stats_lane_2(&self) -> Option<u64> {
        self.lanes.get(2).map(|l| l.stats().frames_decoded)
    }
}
impl MultiplexStream {
    pub fn stats_lane_3(&self) -> Option<u64> {
        self.lanes.get(3).map(|l| l.stats().frames_decoded)
    }
}
impl MultiplexStream {
    pub fn stats_lane_4(&self) -> Option<u64> {
        self.lanes.get(4).map(|l| l.stats().frames_decoded)
    }
}
impl MultiplexStream {
    pub fn stats_lane_5(&self) -> Option<u64> {
        self.lanes.get(5).map(|l| l.stats().frames_decoded)
    }
}
impl MultiplexStream {
    pub fn stats_lane_6(&self) -> Option<u64> {
        self.lanes.get(6).map(|l| l.stats().frames_decoded)
    }
}
impl MultiplexStream {
    pub fn stats_lane_7(&self) -> Option<u64> {
        self.lanes.get(7).map(|l| l.stats().frames_decoded)
    }
}
impl MultiplexStream {
    pub fn stats_lane_8(&self) -> Option<u64> {
        self.lanes.get(8).map(|l| l.stats().frames_decoded)
    }
}
impl MultiplexStream {
    pub fn stats_lane_9(&self) -> Option<u64> {
        self.lanes.get(9).map(|l| l.stats().frames_decoded)
    }
}
impl MultiplexStream {
    pub fn stats_lane_10(&self) -> Option<u64> {
        self.lanes.get(10).map(|l| l.stats().frames_decoded)
    }
}
impl MultiplexStream {
    pub fn stats_lane_11(&self) -> Option<u64> {
        self.lanes.get(11).map(|l| l.stats().frames_decoded)
    }
}
impl MultiplexStream {
    pub fn stats_lane_12(&self) -> Option<u64> {
        self.lanes.get(12).map(|l| l.stats().frames_decoded)
    }
}
impl MultiplexStream {
    pub fn stats_lane_13(&self) -> Option<u64> {
        self.lanes.get(13).map(|l| l.stats().frames_decoded)
    }
}
impl MultiplexStream {
    pub fn stats_lane_14(&self) -> Option<u64> {
        self.lanes.get(14).map(|l| l.stats().frames_decoded)
    }
}
impl MultiplexStream {
    pub fn stats_lane_15(&self) -> Option<u64> {
        self.lanes.get(15).map(|l| l.stats().frames_decoded)
    }
}
impl MultiplexStream {
    pub fn stats_lane_16(&self) -> Option<u64> {
        self.lanes.get(16).map(|l| l.stats().frames_decoded)
    }
}
impl MultiplexStream {
    pub fn stats_lane_17(&self) -> Option<u64> {
        self.lanes.get(17).map(|l| l.stats().frames_decoded)
    }
}
impl MultiplexStream {
    pub fn stats_lane_18(&self) -> Option<u64> {
        self.lanes.get(18).map(|l| l.stats().frames_decoded)
    }
}
impl MultiplexStream {
    pub fn stats_lane_19(&self) -> Option<u64> {
        self.lanes.get(19).map(|l| l.stats().frames_decoded)
    }
}
impl MultiplexStream {
    pub fn stats_lane_20(&self) -> Option<u64> {
        self.lanes.get(20).map(|l| l.stats().frames_decoded)
    }
}
impl MultiplexStream {
    pub fn stats_lane_21(&self) -> Option<u64> {
        self.lanes.get(21).map(|l| l.stats().frames_decoded)
    }
}
impl MultiplexStream {
    pub fn stats_lane_22(&self) -> Option<u64> {
        self.lanes.get(22).map(|l| l.stats().frames_decoded)
    }
}
impl MultiplexStream {
    pub fn stats_lane_23(&self) -> Option<u64> {
        self.lanes.get(23).map(|l| l.stats().frames_decoded)
    }
}
impl MultiplexStream {
    pub fn stats_lane_24(&self) -> Option<u64> {
        self.lanes.get(24).map(|l| l.stats().frames_decoded)
    }
}
