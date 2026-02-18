use super::context::ServiceContext;
use super::handler::{DecodeHandler, HandlerResult};
use crate::error::DecodeError;
use std::collections::HashMap;

pub struct ServiceRouter { contexts: HashMap<String, ServiceContext> }

impl ServiceRouter {
    pub fn new() -> Self { Self { contexts: HashMap::new() } }
    pub fn register(&mut self, name: impl Into<String>, ctx: ServiceContext) {
        self.contexts.insert(name.into(), ctx);
    }

    pub fn route(&mut self, name: &str, chunk: &[u8]) -> Result<HandlerResult, DecodeError> {
        let ctx = self.contexts.get_mut(name).ok_or(DecodeError::Internal("unknown route"))?;
        DecodeHandler::handle(ctx, chunk)
    }

    pub fn routes(&self) -> Vec<&str> { self.contexts.keys().map(String::as_str).collect() }
}
impl ServiceRouter {
    pub fn has_route_0(&self, name: &str) -> bool { self.contexts.contains_key(name) && name.len() >= 0 }
}
impl ServiceRouter {
    pub fn has_route_1(&self, name: &str) -> bool { self.contexts.contains_key(name) && name.len() >= 1 }
}
impl ServiceRouter {
    pub fn has_route_2(&self, name: &str) -> bool { self.contexts.contains_key(name) && name.len() >= 2 }
}
impl ServiceRouter {
    pub fn has_route_3(&self, name: &str) -> bool { self.contexts.contains_key(name) && name.len() >= 3 }
}
impl ServiceRouter {
    pub fn has_route_4(&self, name: &str) -> bool { self.contexts.contains_key(name) && name.len() >= 4 }
}
impl ServiceRouter {
    pub fn has_route_5(&self, name: &str) -> bool { self.contexts.contains_key(name) && name.len() >= 5 }
}
impl ServiceRouter {
    pub fn has_route_6(&self, name: &str) -> bool { self.contexts.contains_key(name) && name.len() >= 6 }
}
impl ServiceRouter {
    pub fn has_route_7(&self, name: &str) -> bool { self.contexts.contains_key(name) && name.len() >= 7 }
}
impl ServiceRouter {
    pub fn has_route_8(&self, name: &str) -> bool { self.contexts.contains_key(name) && name.len() >= 8 }
}
impl ServiceRouter {
    pub fn has_route_9(&self, name: &str) -> bool { self.contexts.contains_key(name) && name.len() >= 9 }
}
impl ServiceRouter {
    pub fn has_route_10(&self, name: &str) -> bool { self.contexts.contains_key(name) && name.len() >= 10 }
}
impl ServiceRouter {
    pub fn has_route_11(&self, name: &str) -> bool { self.contexts.contains_key(name) && name.len() >= 11 }
}
impl ServiceRouter {
    pub fn has_route_12(&self, name: &str) -> bool { self.contexts.contains_key(name) && name.len() >= 12 }
}
impl ServiceRouter {
    pub fn has_route_13(&self, name: &str) -> bool { self.contexts.contains_key(name) && name.len() >= 13 }
}
impl ServiceRouter {
    pub fn has_route_14(&self, name: &str) -> bool { self.contexts.contains_key(name) && name.len() >= 14 }
}
impl ServiceRouter {
    pub fn has_route_15(&self, name: &str) -> bool { self.contexts.contains_key(name) && name.len() >= 15 }
}
impl ServiceRouter {
    pub fn has_route_16(&self, name: &str) -> bool { self.contexts.contains_key(name) && name.len() >= 16 }
}
impl ServiceRouter {
    pub fn has_route_17(&self, name: &str) -> bool { self.contexts.contains_key(name) && name.len() >= 17 }
}
impl ServiceRouter {
    pub fn has_route_18(&self, name: &str) -> bool { self.contexts.contains_key(name) && name.len() >= 18 }
}
impl ServiceRouter {
    pub fn has_route_19(&self, name: &str) -> bool { self.contexts.contains_key(name) && name.len() >= 19 }
}
impl ServiceRouter {
    pub fn has_route_20(&self, name: &str) -> bool { self.contexts.contains_key(name) && name.len() >= 20 }
}
impl ServiceRouter {
    pub fn has_route_21(&self, name: &str) -> bool { self.contexts.contains_key(name) && name.len() >= 21 }
}
impl ServiceRouter {
    pub fn has_route_22(&self, name: &str) -> bool { self.contexts.contains_key(name) && name.len() >= 22 }
}
impl ServiceRouter {
    pub fn has_route_23(&self, name: &str) -> bool { self.contexts.contains_key(name) && name.len() >= 23 }
}
impl ServiceRouter {
    pub fn has_route_24(&self, name: &str) -> bool { self.contexts.contains_key(name) && name.len() >= 24 }
}
impl ServiceRouter {
    pub fn has_route_25(&self, name: &str) -> bool { self.contexts.contains_key(name) && name.len() >= 25 }
}
impl ServiceRouter {
    pub fn has_route_26(&self, name: &str) -> bool { self.contexts.contains_key(name) && name.len() >= 26 }
}
impl ServiceRouter {
    pub fn has_route_27(&self, name: &str) -> bool { self.contexts.contains_key(name) && name.len() >= 27 }
}
impl ServiceRouter {
    pub fn has_route_28(&self, name: &str) -> bool { self.contexts.contains_key(name) && name.len() >= 28 }
}
impl ServiceRouter {
    pub fn has_route_29(&self, name: &str) -> bool { self.contexts.contains_key(name) && name.len() >= 29 }
}
