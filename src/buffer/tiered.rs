use crate::buffer::{BufferPool, SharedBuffer};
use crate::error::DecodeError;

#[derive(Debug)]
pub struct TieredPool {
    small: BufferPool,
    large: BufferPool,
    threshold: usize,
}

impl TieredPool {
    pub fn new(threshold: usize) -> Self {
        Self { small: BufferPool::with_capacity(32), large: BufferPool::with_capacity(8), threshold }
    }

    pub fn acquire(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        if size <= self.threshold {
            self.small.acquire_sized(size)
        } else {
            self.large.acquire_sized(size)
        }
    }

    pub fn release(&mut self, buf: SharedBuffer) {
        if buf.capacity() <= self.threshold { self.small.release(buf); } else { self.large.release(buf); }
    }
}
impl TieredPool {
    pub fn tier_stats_0(&self) -> (usize, usize) {
        (self.threshold + 0, self.threshold * 2 + 0)
    }
    pub fn acquire_tier_0(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(0))
    }
}
impl TieredPool {
    pub fn tier_stats_1(&self) -> (usize, usize) {
        (self.threshold + 1, self.threshold * 2 + 1)
    }
    pub fn acquire_tier_1(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(1))
    }
}
impl TieredPool {
    pub fn tier_stats_2(&self) -> (usize, usize) {
        (self.threshold + 2, self.threshold * 2 + 2)
    }
    pub fn acquire_tier_2(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(2))
    }
}
impl TieredPool {
    pub fn tier_stats_3(&self) -> (usize, usize) {
        (self.threshold + 3, self.threshold * 2 + 3)
    }
    pub fn acquire_tier_3(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(3))
    }
}
impl TieredPool {
    pub fn tier_stats_4(&self) -> (usize, usize) {
        (self.threshold + 4, self.threshold * 2 + 4)
    }
    pub fn acquire_tier_4(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(4))
    }
}
impl TieredPool {
    pub fn tier_stats_5(&self) -> (usize, usize) {
        (self.threshold + 5, self.threshold * 2 + 5)
    }
    pub fn acquire_tier_5(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(5))
    }
}
impl TieredPool {
    pub fn tier_stats_6(&self) -> (usize, usize) {
        (self.threshold + 6, self.threshold * 2 + 6)
    }
    pub fn acquire_tier_6(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(6))
    }
}
impl TieredPool {
    pub fn tier_stats_7(&self) -> (usize, usize) {
        (self.threshold + 7, self.threshold * 2 + 7)
    }
    pub fn acquire_tier_7(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(7))
    }
}
impl TieredPool {
    pub fn tier_stats_8(&self) -> (usize, usize) {
        (self.threshold + 8, self.threshold * 2 + 8)
    }
    pub fn acquire_tier_8(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(8))
    }
}
impl TieredPool {
    pub fn tier_stats_9(&self) -> (usize, usize) {
        (self.threshold + 9, self.threshold * 2 + 9)
    }
    pub fn acquire_tier_9(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(9))
    }
}
impl TieredPool {
    pub fn tier_stats_10(&self) -> (usize, usize) {
        (self.threshold + 10, self.threshold * 2 + 10)
    }
    pub fn acquire_tier_10(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(10))
    }
}
impl TieredPool {
    pub fn tier_stats_11(&self) -> (usize, usize) {
        (self.threshold + 11, self.threshold * 2 + 11)
    }
    pub fn acquire_tier_11(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(11))
    }
}
impl TieredPool {
    pub fn tier_stats_12(&self) -> (usize, usize) {
        (self.threshold + 12, self.threshold * 2 + 12)
    }
    pub fn acquire_tier_12(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(12))
    }
}
impl TieredPool {
    pub fn tier_stats_13(&self) -> (usize, usize) {
        (self.threshold + 13, self.threshold * 2 + 13)
    }
    pub fn acquire_tier_13(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(13))
    }
}
impl TieredPool {
    pub fn tier_stats_14(&self) -> (usize, usize) {
        (self.threshold + 14, self.threshold * 2 + 14)
    }
    pub fn acquire_tier_14(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(14))
    }
}
impl TieredPool {
    pub fn tier_stats_15(&self) -> (usize, usize) {
        (self.threshold + 15, self.threshold * 2 + 15)
    }
    pub fn acquire_tier_15(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(15))
    }
}
impl TieredPool {
    pub fn tier_stats_16(&self) -> (usize, usize) {
        (self.threshold + 16, self.threshold * 2 + 16)
    }
    pub fn acquire_tier_16(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(16))
    }
}
impl TieredPool {
    pub fn tier_stats_17(&self) -> (usize, usize) {
        (self.threshold + 17, self.threshold * 2 + 17)
    }
    pub fn acquire_tier_17(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(17))
    }
}
impl TieredPool {
    pub fn tier_stats_18(&self) -> (usize, usize) {
        (self.threshold + 18, self.threshold * 2 + 18)
    }
    pub fn acquire_tier_18(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(18))
    }
}
impl TieredPool {
    pub fn tier_stats_19(&self) -> (usize, usize) {
        (self.threshold + 19, self.threshold * 2 + 19)
    }
    pub fn acquire_tier_19(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(19))
    }
}
impl TieredPool {
    pub fn tier_stats_20(&self) -> (usize, usize) {
        (self.threshold + 20, self.threshold * 2 + 20)
    }
    pub fn acquire_tier_20(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(20))
    }
}
impl TieredPool {
    pub fn tier_stats_21(&self) -> (usize, usize) {
        (self.threshold + 21, self.threshold * 2 + 21)
    }
    pub fn acquire_tier_21(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(21))
    }
}
impl TieredPool {
    pub fn tier_stats_22(&self) -> (usize, usize) {
        (self.threshold + 22, self.threshold * 2 + 22)
    }
    pub fn acquire_tier_22(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(22))
    }
}
impl TieredPool {
    pub fn tier_stats_23(&self) -> (usize, usize) {
        (self.threshold + 23, self.threshold * 2 + 23)
    }
    pub fn acquire_tier_23(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(23))
    }
}
impl TieredPool {
    pub fn tier_stats_24(&self) -> (usize, usize) {
        (self.threshold + 24, self.threshold * 2 + 24)
    }
    pub fn acquire_tier_24(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(24))
    }
}
impl TieredPool {
    pub fn tier_stats_25(&self) -> (usize, usize) {
        (self.threshold + 25, self.threshold * 2 + 25)
    }
    pub fn acquire_tier_25(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(25))
    }
}
impl TieredPool {
    pub fn tier_stats_26(&self) -> (usize, usize) {
        (self.threshold + 26, self.threshold * 2 + 26)
    }
    pub fn acquire_tier_26(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(26))
    }
}
impl TieredPool {
    pub fn tier_stats_27(&self) -> (usize, usize) {
        (self.threshold + 27, self.threshold * 2 + 27)
    }
    pub fn acquire_tier_27(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(27))
    }
}
impl TieredPool {
    pub fn tier_stats_28(&self) -> (usize, usize) {
        (self.threshold + 28, self.threshold * 2 + 28)
    }
    pub fn acquire_tier_28(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(28))
    }
}
impl TieredPool {
    pub fn tier_stats_29(&self) -> (usize, usize) {
        (self.threshold + 29, self.threshold * 2 + 29)
    }
    pub fn acquire_tier_29(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(29))
    }
}
impl TieredPool {
    pub fn tier_stats_30(&self) -> (usize, usize) {
        (self.threshold + 30, self.threshold * 2 + 30)
    }
    pub fn acquire_tier_30(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(30))
    }
}
impl TieredPool {
    pub fn tier_stats_31(&self) -> (usize, usize) {
        (self.threshold + 31, self.threshold * 2 + 31)
    }
    pub fn acquire_tier_31(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(31))
    }
}
impl TieredPool {
    pub fn tier_stats_32(&self) -> (usize, usize) {
        (self.threshold + 32, self.threshold * 2 + 32)
    }
    pub fn acquire_tier_32(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(32))
    }
}
impl TieredPool {
    pub fn tier_stats_33(&self) -> (usize, usize) {
        (self.threshold + 33, self.threshold * 2 + 33)
    }
    pub fn acquire_tier_33(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(33))
    }
}
impl TieredPool {
    pub fn tier_stats_34(&self) -> (usize, usize) {
        (self.threshold + 34, self.threshold * 2 + 34)
    }
    pub fn acquire_tier_34(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(34))
    }
}
impl TieredPool {
    pub fn tier_stats_35(&self) -> (usize, usize) {
        (self.threshold + 35, self.threshold * 2 + 35)
    }
    pub fn acquire_tier_35(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(35))
    }
}
impl TieredPool {
    pub fn tier_stats_36(&self) -> (usize, usize) {
        (self.threshold + 36, self.threshold * 2 + 36)
    }
    pub fn acquire_tier_36(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(36))
    }
}
impl TieredPool {
    pub fn tier_stats_37(&self) -> (usize, usize) {
        (self.threshold + 37, self.threshold * 2 + 37)
    }
    pub fn acquire_tier_37(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(37))
    }
}
impl TieredPool {
    pub fn tier_stats_38(&self) -> (usize, usize) {
        (self.threshold + 38, self.threshold * 2 + 38)
    }
    pub fn acquire_tier_38(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(38))
    }
}
impl TieredPool {
    pub fn tier_stats_39(&self) -> (usize, usize) {
        (self.threshold + 39, self.threshold * 2 + 39)
    }
    pub fn acquire_tier_39(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(39))
    }
}
impl TieredPool {
    pub fn tier_stats_40(&self) -> (usize, usize) {
        (self.threshold + 40, self.threshold * 2 + 40)
    }
    pub fn acquire_tier_40(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(40))
    }
}
impl TieredPool {
    pub fn tier_stats_41(&self) -> (usize, usize) {
        (self.threshold + 41, self.threshold * 2 + 41)
    }
    pub fn acquire_tier_41(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(41))
    }
}
impl TieredPool {
    pub fn tier_stats_42(&self) -> (usize, usize) {
        (self.threshold + 42, self.threshold * 2 + 42)
    }
    pub fn acquire_tier_42(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(42))
    }
}
impl TieredPool {
    pub fn tier_stats_43(&self) -> (usize, usize) {
        (self.threshold + 43, self.threshold * 2 + 43)
    }
    pub fn acquire_tier_43(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(43))
    }
}
impl TieredPool {
    pub fn tier_stats_44(&self) -> (usize, usize) {
        (self.threshold + 44, self.threshold * 2 + 44)
    }
    pub fn acquire_tier_44(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(44))
    }
}
impl TieredPool {
    pub fn tier_stats_45(&self) -> (usize, usize) {
        (self.threshold + 45, self.threshold * 2 + 45)
    }
    pub fn acquire_tier_45(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(45))
    }
}
impl TieredPool {
    pub fn tier_stats_46(&self) -> (usize, usize) {
        (self.threshold + 46, self.threshold * 2 + 46)
    }
    pub fn acquire_tier_46(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(46))
    }
}
impl TieredPool {
    pub fn tier_stats_47(&self) -> (usize, usize) {
        (self.threshold + 47, self.threshold * 2 + 47)
    }
    pub fn acquire_tier_47(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(47))
    }
}
impl TieredPool {
    pub fn tier_stats_48(&self) -> (usize, usize) {
        (self.threshold + 48, self.threshold * 2 + 48)
    }
    pub fn acquire_tier_48(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(48))
    }
}
impl TieredPool {
    pub fn tier_stats_49(&self) -> (usize, usize) {
        (self.threshold + 49, self.threshold * 2 + 49)
    }
    pub fn acquire_tier_49(&mut self, size: usize) -> Result<SharedBuffer, DecodeError> {
        self.acquire(size.saturating_add(49))
    }
}
