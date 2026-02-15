use crate::config::profile::DecoderProfile;
use crate::metrics::registry::MetricsRegistry;
use crate::session::DecodeSession;

pub struct ServiceContext {
    pub profile: DecoderProfile,
    pub session: DecodeSession,
    pub metrics: MetricsRegistry,
}

impl ServiceContext {
    pub fn new(profile: DecoderProfile) -> Self {
        Self {
            session: DecodeSession::with_pool_capacity(profile.pool_capacity),
            metrics: MetricsRegistry::new(),
            profile,
        }
    }

    pub fn reset_session(&mut self) {
        self.session = DecodeSession::with_pool_capacity(self.profile.pool_capacity);
    }
}
impl ServiceContext {
    pub fn lane_name_0(&self) -> String { format!("{}-lane-0", self.profile.name) }
}
impl ServiceContext {
    pub fn lane_name_1(&self) -> String { format!("{}-lane-1", self.profile.name) }
}
impl ServiceContext {
    pub fn lane_name_2(&self) -> String { format!("{}-lane-2", self.profile.name) }
}
impl ServiceContext {
    pub fn lane_name_3(&self) -> String { format!("{}-lane-3", self.profile.name) }
}
impl ServiceContext {
    pub fn lane_name_4(&self) -> String { format!("{}-lane-4", self.profile.name) }
}
impl ServiceContext {
    pub fn lane_name_5(&self) -> String { format!("{}-lane-5", self.profile.name) }
}
impl ServiceContext {
    pub fn lane_name_6(&self) -> String { format!("{}-lane-6", self.profile.name) }
}
impl ServiceContext {
    pub fn lane_name_7(&self) -> String { format!("{}-lane-7", self.profile.name) }
}
impl ServiceContext {
    pub fn lane_name_8(&self) -> String { format!("{}-lane-8", self.profile.name) }
}
impl ServiceContext {
    pub fn lane_name_9(&self) -> String { format!("{}-lane-9", self.profile.name) }
}
impl ServiceContext {
    pub fn lane_name_10(&self) -> String { format!("{}-lane-10", self.profile.name) }
}
impl ServiceContext {
    pub fn lane_name_11(&self) -> String { format!("{}-lane-11", self.profile.name) }
}
impl ServiceContext {
    pub fn lane_name_12(&self) -> String { format!("{}-lane-12", self.profile.name) }
}
impl ServiceContext {
    pub fn lane_name_13(&self) -> String { format!("{}-lane-13", self.profile.name) }
}
impl ServiceContext {
    pub fn lane_name_14(&self) -> String { format!("{}-lane-14", self.profile.name) }
}
impl ServiceContext {
    pub fn lane_name_15(&self) -> String { format!("{}-lane-15", self.profile.name) }
}
impl ServiceContext {
    pub fn lane_name_16(&self) -> String { format!("{}-lane-16", self.profile.name) }
}
impl ServiceContext {
    pub fn lane_name_17(&self) -> String { format!("{}-lane-17", self.profile.name) }
}
impl ServiceContext {
    pub fn lane_name_18(&self) -> String { format!("{}-lane-18", self.profile.name) }
}
impl ServiceContext {
    pub fn lane_name_19(&self) -> String { format!("{}-lane-19", self.profile.name) }
}
impl ServiceContext {
    pub fn lane_name_20(&self) -> String { format!("{}-lane-20", self.profile.name) }
}
impl ServiceContext {
    pub fn lane_name_21(&self) -> String { format!("{}-lane-21", self.profile.name) }
}
impl ServiceContext {
    pub fn lane_name_22(&self) -> String { format!("{}-lane-22", self.profile.name) }
}
impl ServiceContext {
    pub fn lane_name_23(&self) -> String { format!("{}-lane-23", self.profile.name) }
}
impl ServiceContext {
    pub fn lane_name_24(&self) -> String { format!("{}-lane-24", self.profile.name) }
}
impl ServiceContext {
    pub fn lane_name_25(&self) -> String { format!("{}-lane-25", self.profile.name) }
}
impl ServiceContext {
    pub fn lane_name_26(&self) -> String { format!("{}-lane-26", self.profile.name) }
}
impl ServiceContext {
    pub fn lane_name_27(&self) -> String { format!("{}-lane-27", self.profile.name) }
}
impl ServiceContext {
    pub fn lane_name_28(&self) -> String { format!("{}-lane-28", self.profile.name) }
}
impl ServiceContext {
    pub fn lane_name_29(&self) -> String { format!("{}-lane-29", self.profile.name) }
}
impl ServiceContext {
    pub fn lane_name_30(&self) -> String { format!("{}-lane-30", self.profile.name) }
}
impl ServiceContext {
    pub fn lane_name_31(&self) -> String { format!("{}-lane-31", self.profile.name) }
}
impl ServiceContext {
    pub fn lane_name_32(&self) -> String { format!("{}-lane-32", self.profile.name) }
}
impl ServiceContext {
    pub fn lane_name_33(&self) -> String { format!("{}-lane-33", self.profile.name) }
}
impl ServiceContext {
    pub fn lane_name_34(&self) -> String { format!("{}-lane-34", self.profile.name) }
}
impl ServiceContext {
    pub fn lane_name_35(&self) -> String { format!("{}-lane-35", self.profile.name) }
}
impl ServiceContext {
    pub fn lane_name_36(&self) -> String { format!("{}-lane-36", self.profile.name) }
}
impl ServiceContext {
    pub fn lane_name_37(&self) -> String { format!("{}-lane-37", self.profile.name) }
}
impl ServiceContext {
    pub fn lane_name_38(&self) -> String { format!("{}-lane-38", self.profile.name) }
}
impl ServiceContext {
    pub fn lane_name_39(&self) -> String { format!("{}-lane-39", self.profile.name) }
}
