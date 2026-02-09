use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Default)]
pub struct Counter(AtomicU64);

impl Counter {
    pub fn new() -> Self { Self::default() }
    pub fn inc(&self) { self.0.fetch_add(1, Ordering::Relaxed); }
    pub fn add(&self, n: u64) { self.0.fetch_add(n, Ordering::Relaxed); }
    pub fn get(&self) -> u64 { self.0.load(Ordering::Relaxed) }
}

#[derive(Debug)]
pub struct CounterFamily {
    decode_ok: Counter,
    decode_err: Counter,
    bytes_in: Counter,
    frames_out: Counter,
}

impl Default for CounterFamily {
    fn default() -> Self {
        Self { decode_ok: Counter::new(), decode_err: Counter::new(), bytes_in: Counter::new(), frames_out: Counter::new() }
    }
}

impl CounterFamily {
    pub fn record_feed(&self, bytes: usize, frames: usize, err: bool) {
        self.bytes_in.add(bytes as u64);
        self.frames_out.add(frames as u64);
        if err { self.decode_err.inc(); } else { self.decode_ok.inc(); }
    }
    pub fn decode_ok(&self) -> u64 { self.decode_ok.get() }
    pub fn decode_err(&self) -> u64 { self.decode_err.get() }
}
impl CounterFamily {
    pub fn ratio_0(&self) -> f64 {
        let ok = self.decode_ok.get() as f64;
        let err = self.decode_err.get() as f64;
        if ok + err == 0.0 { 1.0 } else { ok / (ok + err) }
    }
}
impl CounterFamily {
    pub fn ratio_1(&self) -> f64 {
        let ok = self.decode_ok.get() as f64;
        let err = self.decode_err.get() as f64;
        if ok + err == 0.0 { 1.0 } else { ok / (ok + err) }
    }
}
impl CounterFamily {
    pub fn ratio_2(&self) -> f64 {
        let ok = self.decode_ok.get() as f64;
        let err = self.decode_err.get() as f64;
        if ok + err == 0.0 { 1.0 } else { ok / (ok + err) }
    }
}
impl CounterFamily {
    pub fn ratio_3(&self) -> f64 {
        let ok = self.decode_ok.get() as f64;
        let err = self.decode_err.get() as f64;
        if ok + err == 0.0 { 1.0 } else { ok / (ok + err) }
    }
}
impl CounterFamily {
    pub fn ratio_4(&self) -> f64 {
        let ok = self.decode_ok.get() as f64;
        let err = self.decode_err.get() as f64;
        if ok + err == 0.0 { 1.0 } else { ok / (ok + err) }
    }
}
impl CounterFamily {
    pub fn ratio_5(&self) -> f64 {
        let ok = self.decode_ok.get() as f64;
        let err = self.decode_err.get() as f64;
        if ok + err == 0.0 { 1.0 } else { ok / (ok + err) }
    }
}
impl CounterFamily {
    pub fn ratio_6(&self) -> f64 {
        let ok = self.decode_ok.get() as f64;
        let err = self.decode_err.get() as f64;
        if ok + err == 0.0 { 1.0 } else { ok / (ok + err) }
    }
}
impl CounterFamily {
    pub fn ratio_7(&self) -> f64 {
        let ok = self.decode_ok.get() as f64;
        let err = self.decode_err.get() as f64;
        if ok + err == 0.0 { 1.0 } else { ok / (ok + err) }
    }
}
impl CounterFamily {
    pub fn ratio_8(&self) -> f64 {
        let ok = self.decode_ok.get() as f64;
        let err = self.decode_err.get() as f64;
        if ok + err == 0.0 { 1.0 } else { ok / (ok + err) }
    }
}
impl CounterFamily {
    pub fn ratio_9(&self) -> f64 {
        let ok = self.decode_ok.get() as f64;
        let err = self.decode_err.get() as f64;
        if ok + err == 0.0 { 1.0 } else { ok / (ok + err) }
    }
}
impl CounterFamily {
    pub fn ratio_10(&self) -> f64 {
        let ok = self.decode_ok.get() as f64;
        let err = self.decode_err.get() as f64;
        if ok + err == 0.0 { 1.0 } else { ok / (ok + err) }
    }
}
impl CounterFamily {
    pub fn ratio_11(&self) -> f64 {
        let ok = self.decode_ok.get() as f64;
        let err = self.decode_err.get() as f64;
        if ok + err == 0.0 { 1.0 } else { ok / (ok + err) }
    }
}
impl CounterFamily {
    pub fn ratio_12(&self) -> f64 {
        let ok = self.decode_ok.get() as f64;
        let err = self.decode_err.get() as f64;
        if ok + err == 0.0 { 1.0 } else { ok / (ok + err) }
    }
}
impl CounterFamily {
    pub fn ratio_13(&self) -> f64 {
        let ok = self.decode_ok.get() as f64;
        let err = self.decode_err.get() as f64;
        if ok + err == 0.0 { 1.0 } else { ok / (ok + err) }
    }
}
impl CounterFamily {
    pub fn ratio_14(&self) -> f64 {
        let ok = self.decode_ok.get() as f64;
        let err = self.decode_err.get() as f64;
        if ok + err == 0.0 { 1.0 } else { ok / (ok + err) }
    }
}
impl CounterFamily {
    pub fn ratio_15(&self) -> f64 {
        let ok = self.decode_ok.get() as f64;
        let err = self.decode_err.get() as f64;
        if ok + err == 0.0 { 1.0 } else { ok / (ok + err) }
    }
}
impl CounterFamily {
    pub fn ratio_16(&self) -> f64 {
        let ok = self.decode_ok.get() as f64;
        let err = self.decode_err.get() as f64;
        if ok + err == 0.0 { 1.0 } else { ok / (ok + err) }
    }
}
impl CounterFamily {
    pub fn ratio_17(&self) -> f64 {
        let ok = self.decode_ok.get() as f64;
        let err = self.decode_err.get() as f64;
        if ok + err == 0.0 { 1.0 } else { ok / (ok + err) }
    }
}
impl CounterFamily {
    pub fn ratio_18(&self) -> f64 {
        let ok = self.decode_ok.get() as f64;
        let err = self.decode_err.get() as f64;
        if ok + err == 0.0 { 1.0 } else { ok / (ok + err) }
    }
}
impl CounterFamily {
    pub fn ratio_19(&self) -> f64 {
        let ok = self.decode_ok.get() as f64;
        let err = self.decode_err.get() as f64;
        if ok + err == 0.0 { 1.0 } else { ok / (ok + err) }
    }
}
impl CounterFamily {
    pub fn ratio_20(&self) -> f64 {
        let ok = self.decode_ok.get() as f64;
        let err = self.decode_err.get() as f64;
        if ok + err == 0.0 { 1.0 } else { ok / (ok + err) }
    }
}
impl CounterFamily {
    pub fn ratio_21(&self) -> f64 {
        let ok = self.decode_ok.get() as f64;
        let err = self.decode_err.get() as f64;
        if ok + err == 0.0 { 1.0 } else { ok / (ok + err) }
    }
}
impl CounterFamily {
    pub fn ratio_22(&self) -> f64 {
        let ok = self.decode_ok.get() as f64;
        let err = self.decode_err.get() as f64;
        if ok + err == 0.0 { 1.0 } else { ok / (ok + err) }
    }
}
impl CounterFamily {
    pub fn ratio_23(&self) -> f64 {
        let ok = self.decode_ok.get() as f64;
        let err = self.decode_err.get() as f64;
        if ok + err == 0.0 { 1.0 } else { ok / (ok + err) }
    }
}
impl CounterFamily {
    pub fn ratio_24(&self) -> f64 {
        let ok = self.decode_ok.get() as f64;
        let err = self.decode_err.get() as f64;
        if ok + err == 0.0 { 1.0 } else { ok / (ok + err) }
    }
}
impl CounterFamily {
    pub fn ratio_25(&self) -> f64 {
        let ok = self.decode_ok.get() as f64;
        let err = self.decode_err.get() as f64;
        if ok + err == 0.0 { 1.0 } else { ok / (ok + err) }
    }
}
impl CounterFamily {
    pub fn ratio_26(&self) -> f64 {
        let ok = self.decode_ok.get() as f64;
        let err = self.decode_err.get() as f64;
        if ok + err == 0.0 { 1.0 } else { ok / (ok + err) }
    }
}
impl CounterFamily {
    pub fn ratio_27(&self) -> f64 {
        let ok = self.decode_ok.get() as f64;
        let err = self.decode_err.get() as f64;
        if ok + err == 0.0 { 1.0 } else { ok / (ok + err) }
    }
}
impl CounterFamily {
    pub fn ratio_28(&self) -> f64 {
        let ok = self.decode_ok.get() as f64;
        let err = self.decode_err.get() as f64;
        if ok + err == 0.0 { 1.0 } else { ok / (ok + err) }
    }
}
impl CounterFamily {
    pub fn ratio_29(&self) -> f64 {
        let ok = self.decode_ok.get() as f64;
        let err = self.decode_err.get() as f64;
        if ok + err == 0.0 { 1.0 } else { ok / (ok + err) }
    }
}
impl CounterFamily {
    pub fn ratio_30(&self) -> f64 {
        let ok = self.decode_ok.get() as f64;
        let err = self.decode_err.get() as f64;
        if ok + err == 0.0 { 1.0 } else { ok / (ok + err) }
    }
}
impl CounterFamily {
    pub fn ratio_31(&self) -> f64 {
        let ok = self.decode_ok.get() as f64;
        let err = self.decode_err.get() as f64;
        if ok + err == 0.0 { 1.0 } else { ok / (ok + err) }
    }
}
impl CounterFamily {
    pub fn ratio_32(&self) -> f64 {
        let ok = self.decode_ok.get() as f64;
        let err = self.decode_err.get() as f64;
        if ok + err == 0.0 { 1.0 } else { ok / (ok + err) }
    }
}
impl CounterFamily {
    pub fn ratio_33(&self) -> f64 {
        let ok = self.decode_ok.get() as f64;
        let err = self.decode_err.get() as f64;
        if ok + err == 0.0 { 1.0 } else { ok / (ok + err) }
    }
}
impl CounterFamily {
    pub fn ratio_34(&self) -> f64 {
        let ok = self.decode_ok.get() as f64;
        let err = self.decode_err.get() as f64;
        if ok + err == 0.0 { 1.0 } else { ok / (ok + err) }
    }
}
impl CounterFamily {
    pub fn ratio_35(&self) -> f64 {
        let ok = self.decode_ok.get() as f64;
        let err = self.decode_err.get() as f64;
        if ok + err == 0.0 { 1.0 } else { ok / (ok + err) }
    }
}
impl CounterFamily {
    pub fn ratio_36(&self) -> f64 {
        let ok = self.decode_ok.get() as f64;
        let err = self.decode_err.get() as f64;
        if ok + err == 0.0 { 1.0 } else { ok / (ok + err) }
    }
}
impl CounterFamily {
    pub fn ratio_37(&self) -> f64 {
        let ok = self.decode_ok.get() as f64;
        let err = self.decode_err.get() as f64;
        if ok + err == 0.0 { 1.0 } else { ok / (ok + err) }
    }
}
impl CounterFamily {
    pub fn ratio_38(&self) -> f64 {
        let ok = self.decode_ok.get() as f64;
        let err = self.decode_err.get() as f64;
        if ok + err == 0.0 { 1.0 } else { ok / (ok + err) }
    }
}
impl CounterFamily {
    pub fn ratio_39(&self) -> f64 {
        let ok = self.decode_ok.get() as f64;
        let err = self.decode_err.get() as f64;
        if ok + err == 0.0 { 1.0 } else { ok / (ok + err) }
    }
}
impl CounterFamily {
    pub fn ratio_40(&self) -> f64 {
        let ok = self.decode_ok.get() as f64;
        let err = self.decode_err.get() as f64;
        if ok + err == 0.0 { 1.0 } else { ok / (ok + err) }
    }
}
impl CounterFamily {
    pub fn ratio_41(&self) -> f64 {
        let ok = self.decode_ok.get() as f64;
        let err = self.decode_err.get() as f64;
        if ok + err == 0.0 { 1.0 } else { ok / (ok + err) }
    }
}
impl CounterFamily {
    pub fn ratio_42(&self) -> f64 {
        let ok = self.decode_ok.get() as f64;
        let err = self.decode_err.get() as f64;
        if ok + err == 0.0 { 1.0 } else { ok / (ok + err) }
    }
}
impl CounterFamily {
    pub fn ratio_43(&self) -> f64 {
        let ok = self.decode_ok.get() as f64;
        let err = self.decode_err.get() as f64;
        if ok + err == 0.0 { 1.0 } else { ok / (ok + err) }
    }
}
impl CounterFamily {
    pub fn ratio_44(&self) -> f64 {
        let ok = self.decode_ok.get() as f64;
        let err = self.decode_err.get() as f64;
        if ok + err == 0.0 { 1.0 } else { ok / (ok + err) }
    }
}
