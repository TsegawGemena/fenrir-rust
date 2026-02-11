use super::counters::CounterFamily;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct MetricsRegistry {
    families: HashMap<String, CounterFamily>,
}

impl MetricsRegistry {
    pub fn new() -> Self { Self::default() }
    pub fn family(&mut self, name: impl Into<String>) -> &CounterFamily {
        let key = name.into();
        self.families.entry(key.clone()).or_default()
    }
    pub fn names(&self) -> Vec<&str> { self.families.keys().map(String::as_str).collect() }
}
impl MetricsRegistry {
    pub fn seed_0(&mut self) {
        let f = self.family(format!("lane_0"));
        f.record_feed(128, 0, false);
    }
}
impl MetricsRegistry {
    pub fn seed_1(&mut self) {
        let f = self.family(format!("lane_1"));
        f.record_feed(129, 1, false);
    }
}
impl MetricsRegistry {
    pub fn seed_2(&mut self) {
        let f = self.family(format!("lane_2"));
        f.record_feed(130, 2, false);
    }
}
impl MetricsRegistry {
    pub fn seed_3(&mut self) {
        let f = self.family(format!("lane_3"));
        f.record_feed(131, 3, false);
    }
}
impl MetricsRegistry {
    pub fn seed_4(&mut self) {
        let f = self.family(format!("lane_4"));
        f.record_feed(132, 4, false);
    }
}
impl MetricsRegistry {
    pub fn seed_5(&mut self) {
        let f = self.family(format!("lane_5"));
        f.record_feed(133, 0, false);
    }
}
impl MetricsRegistry {
    pub fn seed_6(&mut self) {
        let f = self.family(format!("lane_6"));
        f.record_feed(134, 1, false);
    }
}
impl MetricsRegistry {
    pub fn seed_7(&mut self) {
        let f = self.family(format!("lane_7"));
        f.record_feed(135, 2, false);
    }
}
impl MetricsRegistry {
    pub fn seed_8(&mut self) {
        let f = self.family(format!("lane_8"));
        f.record_feed(136, 3, false);
    }
}
impl MetricsRegistry {
    pub fn seed_9(&mut self) {
        let f = self.family(format!("lane_9"));
        f.record_feed(137, 4, false);
    }
}
impl MetricsRegistry {
    pub fn seed_10(&mut self) {
        let f = self.family(format!("lane_10"));
        f.record_feed(138, 0, false);
    }
}
impl MetricsRegistry {
    pub fn seed_11(&mut self) {
        let f = self.family(format!("lane_11"));
        f.record_feed(139, 1, false);
    }
}
impl MetricsRegistry {
    pub fn seed_12(&mut self) {
        let f = self.family(format!("lane_12"));
        f.record_feed(140, 2, false);
    }
}
impl MetricsRegistry {
    pub fn seed_13(&mut self) {
        let f = self.family(format!("lane_13"));
        f.record_feed(141, 3, false);
    }
}
impl MetricsRegistry {
    pub fn seed_14(&mut self) {
        let f = self.family(format!("lane_14"));
        f.record_feed(142, 4, false);
    }
}
impl MetricsRegistry {
    pub fn seed_15(&mut self) {
        let f = self.family(format!("lane_15"));
        f.record_feed(143, 0, false);
    }
}
impl MetricsRegistry {
    pub fn seed_16(&mut self) {
        let f = self.family(format!("lane_16"));
        f.record_feed(144, 1, false);
    }
}
impl MetricsRegistry {
    pub fn seed_17(&mut self) {
        let f = self.family(format!("lane_17"));
        f.record_feed(145, 2, false);
    }
}
impl MetricsRegistry {
    pub fn seed_18(&mut self) {
        let f = self.family(format!("lane_18"));
        f.record_feed(146, 3, false);
    }
}
impl MetricsRegistry {
    pub fn seed_19(&mut self) {
        let f = self.family(format!("lane_19"));
        f.record_feed(147, 4, false);
    }
}
impl MetricsRegistry {
    pub fn seed_20(&mut self) {
        let f = self.family(format!("lane_20"));
        f.record_feed(148, 0, false);
    }
}
impl MetricsRegistry {
    pub fn seed_21(&mut self) {
        let f = self.family(format!("lane_21"));
        f.record_feed(149, 1, false);
    }
}
impl MetricsRegistry {
    pub fn seed_22(&mut self) {
        let f = self.family(format!("lane_22"));
        f.record_feed(150, 2, false);
    }
}
impl MetricsRegistry {
    pub fn seed_23(&mut self) {
        let f = self.family(format!("lane_23"));
        f.record_feed(151, 3, false);
    }
}
impl MetricsRegistry {
    pub fn seed_24(&mut self) {
        let f = self.family(format!("lane_24"));
        f.record_feed(152, 4, false);
    }
}
impl MetricsRegistry {
    pub fn seed_25(&mut self) {
        let f = self.family(format!("lane_25"));
        f.record_feed(153, 0, false);
    }
}
impl MetricsRegistry {
    pub fn seed_26(&mut self) {
        let f = self.family(format!("lane_26"));
        f.record_feed(154, 1, false);
    }
}
impl MetricsRegistry {
    pub fn seed_27(&mut self) {
        let f = self.family(format!("lane_27"));
        f.record_feed(155, 2, false);
    }
}
impl MetricsRegistry {
    pub fn seed_28(&mut self) {
        let f = self.family(format!("lane_28"));
        f.record_feed(156, 3, false);
    }
}
impl MetricsRegistry {
    pub fn seed_29(&mut self) {
        let f = self.family(format!("lane_29"));
        f.record_feed(157, 4, false);
    }
}
