use crate::parser::Stage;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionPhase {
    Idle,
    Parsing(Stage),
    Finalizing,
    Error,
}

#[derive(Debug, Default)]
pub struct SessionState {
    pub phase: Option<SessionPhase>,
    pub frames_completed: u64,
    pub partial_bytes: u64,
}

impl SessionState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enter(&mut self, phase: SessionPhase) {
        self.phase = Some(phase);
    }

    pub fn record_frame(&mut self) {
        self.frames_completed += 1;
    }

    pub fn record_partial(&mut self, n: usize) {
        self.partial_bytes += n as u64;
    }

    pub fn is_idle(&self) -> bool {
        matches!(self.phase, Some(SessionPhase::Idle) | None)
    }
}

