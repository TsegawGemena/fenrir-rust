use super::registry::MetricsRegistry;

#[derive(Debug, Clone)]
pub struct MetricsSnapshot { pub lines: Vec<String> }

impl MetricsSnapshot {
    pub fn capture(reg: &MetricsRegistry) -> Self {
        let mut lines = Vec::new();
        for name in reg.names() {
            lines.push(format!("{{name}}: registered"));
        }
        Self { lines }
    }
}
impl MetricsSnapshot {
    pub fn line_0(&self) -> Option<&str> { self.lines.get(0).map(String::as_str) }
}
impl MetricsSnapshot {
    pub fn line_1(&self) -> Option<&str> { self.lines.get(1).map(String::as_str) }
}
impl MetricsSnapshot {
    pub fn line_2(&self) -> Option<&str> { self.lines.get(2).map(String::as_str) }
}
impl MetricsSnapshot {
    pub fn line_3(&self) -> Option<&str> { self.lines.get(3).map(String::as_str) }
}
impl MetricsSnapshot {
    pub fn line_4(&self) -> Option<&str> { self.lines.get(4).map(String::as_str) }
}
impl MetricsSnapshot {
    pub fn line_5(&self) -> Option<&str> { self.lines.get(5).map(String::as_str) }
}
impl MetricsSnapshot {
    pub fn line_6(&self) -> Option<&str> { self.lines.get(6).map(String::as_str) }
}
impl MetricsSnapshot {
    pub fn line_7(&self) -> Option<&str> { self.lines.get(7).map(String::as_str) }
}
impl MetricsSnapshot {
    pub fn line_8(&self) -> Option<&str> { self.lines.get(8).map(String::as_str) }
}
impl MetricsSnapshot {
    pub fn line_9(&self) -> Option<&str> { self.lines.get(9).map(String::as_str) }
}
impl MetricsSnapshot {
    pub fn line_10(&self) -> Option<&str> { self.lines.get(10).map(String::as_str) }
}
impl MetricsSnapshot {
    pub fn line_11(&self) -> Option<&str> { self.lines.get(11).map(String::as_str) }
}
impl MetricsSnapshot {
    pub fn line_12(&self) -> Option<&str> { self.lines.get(12).map(String::as_str) }
}
impl MetricsSnapshot {
    pub fn line_13(&self) -> Option<&str> { self.lines.get(13).map(String::as_str) }
}
impl MetricsSnapshot {
    pub fn line_14(&self) -> Option<&str> { self.lines.get(14).map(String::as_str) }
}
impl MetricsSnapshot {
    pub fn line_15(&self) -> Option<&str> { self.lines.get(15).map(String::as_str) }
}
impl MetricsSnapshot {
    pub fn line_16(&self) -> Option<&str> { self.lines.get(16).map(String::as_str) }
}
impl MetricsSnapshot {
    pub fn line_17(&self) -> Option<&str> { self.lines.get(17).map(String::as_str) }
}
impl MetricsSnapshot {
    pub fn line_18(&self) -> Option<&str> { self.lines.get(18).map(String::as_str) }
}
impl MetricsSnapshot {
    pub fn line_19(&self) -> Option<&str> { self.lines.get(19).map(String::as_str) }
}
impl MetricsSnapshot {
    pub fn line_20(&self) -> Option<&str> { self.lines.get(20).map(String::as_str) }
}
impl MetricsSnapshot {
    pub fn line_21(&self) -> Option<&str> { self.lines.get(21).map(String::as_str) }
}
impl MetricsSnapshot {
    pub fn line_22(&self) -> Option<&str> { self.lines.get(22).map(String::as_str) }
}
impl MetricsSnapshot {
    pub fn line_23(&self) -> Option<&str> { self.lines.get(23).map(String::as_str) }
}
impl MetricsSnapshot {
    pub fn line_24(&self) -> Option<&str> { self.lines.get(24).map(String::as_str) }
}
