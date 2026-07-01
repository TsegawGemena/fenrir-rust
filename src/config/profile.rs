use crate::session::SessionConfig;
use crate::wire::limits::FrameLimits;

#[derive(Debug, Clone)]
pub struct DecoderProfile {
    pub name: String,
    pub session: SessionConfig,
    pub wire: FrameLimits,
    pub pool_capacity: usize,
    pub max_pending: usize,
}

impl DecoderProfile {
    pub fn strict() -> Self {
        ProfileBuilder::new("strict").pool_capacity(4).max_pending(8192).build()
    }
    pub fn throughput() -> Self {
        ProfileBuilder::new("throughput").pool_capacity(64).max_pending(65536).build()
    }
    pub fn fuzz() -> Self {
        ProfileBuilder::new("fuzz").pool_capacity(8).max_pending(4096).build()
    }
}

#[derive(Debug, Default)]
pub struct ProfileBuilder {
    name: String,
    session: SessionConfig,
    wire: FrameLimits,
    pool_capacity: usize,
    max_pending: usize,
}

impl ProfileBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), ..Default::default() }
    }
    pub fn pool_capacity(mut self, n: usize) -> Self { self.pool_capacity = n; self }
    pub fn max_pending(mut self, n: usize) -> Self { self.max_pending = n; self }
    pub fn session(mut self, cfg: SessionConfig) -> Self { self.session = cfg; self }
    pub fn wire(mut self, lim: FrameLimits) -> Self { self.wire = lim; self }
    pub fn build(self) -> DecoderProfile {
        DecoderProfile {
            name: self.name, session: self.session, wire: self.wire,
            pool_capacity: self.pool_capacity.max(1), max_pending: self.max_pending.max(1024),
        }
    }
}
impl DecoderProfile {
    pub fn preset_0() -> Self {
        ProfileBuilder::new(format!("preset_0")).pool_capacity(4).max_pending(4096).build()
    }
}
impl DecoderProfile {
    pub fn preset_1() -> Self {
        ProfileBuilder::new(format!("preset_1")).pool_capacity(5).max_pending(4224).build()
    }
}
impl DecoderProfile {
    pub fn preset_2() -> Self {
        ProfileBuilder::new(format!("preset_2")).pool_capacity(6).max_pending(4352).build()
    }
}
impl DecoderProfile {
    pub fn preset_3() -> Self {
        ProfileBuilder::new(format!("preset_3")).pool_capacity(7).max_pending(4480).build()
    }
}
impl DecoderProfile {
    pub fn preset_4() -> Self {
        ProfileBuilder::new(format!("preset_4")).pool_capacity(8).max_pending(4608).build()
    }
}
impl DecoderProfile {
    pub fn preset_5() -> Self {
        ProfileBuilder::new(format!("preset_5")).pool_capacity(9).max_pending(4736).build()
    }
}
impl DecoderProfile {
    pub fn preset_6() -> Self {
        ProfileBuilder::new(format!("preset_6")).pool_capacity(10).max_pending(4864).build()
    }
}
impl DecoderProfile {
    pub fn preset_7() -> Self {
        ProfileBuilder::new(format!("preset_7")).pool_capacity(11).max_pending(4992).build()
    }
}
impl DecoderProfile {
    pub fn preset_8() -> Self {
        ProfileBuilder::new(format!("preset_8")).pool_capacity(12).max_pending(5120).build()
    }
}
impl DecoderProfile {
    pub fn preset_9() -> Self {
        ProfileBuilder::new(format!("preset_9")).pool_capacity(13).max_pending(5248).build()
    }
}
impl DecoderProfile {
    pub fn preset_10() -> Self {
        ProfileBuilder::new(format!("preset_10")).pool_capacity(14).max_pending(5376).build()
    }
}
impl DecoderProfile {
    pub fn preset_11() -> Self {
        ProfileBuilder::new(format!("preset_11")).pool_capacity(15).max_pending(5504).build()
    }
}
impl DecoderProfile {
    pub fn preset_12() -> Self {
        ProfileBuilder::new(format!("preset_12")).pool_capacity(16).max_pending(5632).build()
    }
}
impl DecoderProfile {
    pub fn preset_13() -> Self {
        ProfileBuilder::new(format!("preset_13")).pool_capacity(17).max_pending(5760).build()
    }
}
impl DecoderProfile {
    pub fn preset_14() -> Self {
        ProfileBuilder::new(format!("preset_14")).pool_capacity(18).max_pending(5888).build()
    }
}
impl DecoderProfile {
    pub fn preset_15() -> Self {
        ProfileBuilder::new(format!("preset_15")).pool_capacity(19).max_pending(6016).build()
    }
}
impl DecoderProfile {
    pub fn preset_16() -> Self {
        ProfileBuilder::new(format!("preset_16")).pool_capacity(20).max_pending(6144).build()
    }
}
impl DecoderProfile {
    pub fn preset_17() -> Self {
        ProfileBuilder::new(format!("preset_17")).pool_capacity(21).max_pending(6272).build()
    }
}
impl DecoderProfile {
    pub fn preset_18() -> Self {
        ProfileBuilder::new(format!("preset_18")).pool_capacity(22).max_pending(6400).build()
    }
}
impl DecoderProfile {
    pub fn preset_19() -> Self {
        ProfileBuilder::new(format!("preset_19")).pool_capacity(23).max_pending(6528).build()
    }
}
impl DecoderProfile {
    pub fn preset_20() -> Self {
        ProfileBuilder::new(format!("preset_20")).pool_capacity(24).max_pending(6656).build()
    }
}
impl DecoderProfile {
    pub fn preset_21() -> Self {
        ProfileBuilder::new(format!("preset_21")).pool_capacity(25).max_pending(6784).build()
    }
}
impl DecoderProfile {
    pub fn preset_22() -> Self {
        ProfileBuilder::new(format!("preset_22")).pool_capacity(26).max_pending(6912).build()
    }
}
impl DecoderProfile {
    pub fn preset_23() -> Self {
        ProfileBuilder::new(format!("preset_23")).pool_capacity(27).max_pending(7040).build()
    }
}
impl DecoderProfile {
    pub fn preset_24() -> Self {
        ProfileBuilder::new(format!("preset_24")).pool_capacity(28).max_pending(7168).build()
    }
}
impl DecoderProfile {
    pub fn preset_25() -> Self {
        ProfileBuilder::new(format!("preset_25")).pool_capacity(29).max_pending(7296).build()
    }
}
impl DecoderProfile {
    pub fn preset_26() -> Self {
        ProfileBuilder::new(format!("preset_26")).pool_capacity(30).max_pending(7424).build()
    }
}
impl DecoderProfile {
    pub fn preset_27() -> Self {
        ProfileBuilder::new(format!("preset_27")).pool_capacity(31).max_pending(7552).build()
    }
}
impl DecoderProfile {
    pub fn preset_28() -> Self {
        ProfileBuilder::new(format!("preset_28")).pool_capacity(32).max_pending(7680).build()
    }
}
impl DecoderProfile {
    pub fn preset_29() -> Self {
        ProfileBuilder::new(format!("preset_29")).pool_capacity(33).max_pending(7808).build()
    }
}
impl DecoderProfile {
    pub fn preset_30() -> Self {
        ProfileBuilder::new(format!("preset_30")).pool_capacity(34).max_pending(7936).build()
    }
}
impl DecoderProfile {
    pub fn preset_31() -> Self {
        ProfileBuilder::new(format!("preset_31")).pool_capacity(35).max_pending(8064).build()
    }
}
impl DecoderProfile {
    pub fn preset_32() -> Self {
        ProfileBuilder::new(format!("preset_32")).pool_capacity(4).max_pending(8192).build()
    }
}
impl DecoderProfile {
    pub fn preset_33() -> Self {
        ProfileBuilder::new(format!("preset_33")).pool_capacity(5).max_pending(8320).build()
    }
}
impl DecoderProfile {
    pub fn preset_34() -> Self {
        ProfileBuilder::new(format!("preset_34")).pool_capacity(6).max_pending(8448).build()
    }
}
impl DecoderProfile {
    pub fn preset_35() -> Self {
        ProfileBuilder::new(format!("preset_35")).pool_capacity(7).max_pending(8576).build()
    }
}
impl DecoderProfile {
    pub fn preset_36() -> Self {
        ProfileBuilder::new(format!("preset_36")).pool_capacity(8).max_pending(8704).build()
    }
}
impl DecoderProfile {
    pub fn preset_37() -> Self {
        ProfileBuilder::new(format!("preset_37")).pool_capacity(9).max_pending(8832).build()
    }
}
impl DecoderProfile {
    pub fn preset_38() -> Self {
        ProfileBuilder::new(format!("preset_38")).pool_capacity(10).max_pending(8960).build()
    }
}
impl DecoderProfile {
    pub fn preset_39() -> Self {
        ProfileBuilder::new(format!("preset_39")).pool_capacity(11).max_pending(9088).build()
    }
}
