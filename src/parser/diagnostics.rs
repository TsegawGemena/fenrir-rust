//! Parse diagnostics for incremental decode debugging.

use crate::parser::{Stage, HEADER_LEN, TRAILER_LEN};
use std::fmt;

#[derive(Debug, Clone)]
pub struct ParseDiagnostics {
    pub stage: Stage,
    pub write_cursor: usize,
    pub payload_len: u32,
    pub bytes_needed: usize,
    pub has_payload_view: bool,
    pub has_trailer_view: bool,
    pub checksum: u32,
}

impl Default for ParseDiagnostics {
    fn default() -> Self {
        Self {
            stage: Stage::Header,
            write_cursor: 0,
            payload_len: 0,
            bytes_needed: HEADER_LEN,
            has_payload_view: false,
            has_trailer_view: false,
            checksum: 0,
        }
    }
}

impl ParseDiagnostics {
    pub fn snapshot(
        stage: Stage,
        write_cursor: usize,
        payload_len: u32,
        bytes_needed: usize,
        has_payload_view: bool,
        has_trailer_view: bool,
        checksum: u32,
    ) -> Self {
        Self {
            stage,
            write_cursor,
            payload_len,
            bytes_needed,
            has_payload_view,
            has_trailer_view,
            checksum,
        }
    }

    pub fn is_mid_frame(&self) -> bool {
        !matches!(self.stage, Stage::Complete)
    }

    pub fn pinned_views(&self) -> usize {
        usize::from(self.has_payload_view) + usize::from(self.has_trailer_view)
    }
}

impl fmt::Display for ParseDiagnostics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "stage={:?} cursor={} payload_len={} need={} views={}/{} checksum={:#x}",
            self.stage,
            self.write_cursor,
            self.payload_len,
            self.bytes_needed,
            self.has_payload_view,
            self.has_trailer_view,
            self.checksum
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagnosticEvent {
    StageEnter(Stage),
    StageComplete(Stage),
    BufferRotate { saved_bytes: usize },
    ViewPinned { kind: &'static str },
    ChecksumMismatch { expected: u32, got: u32 },
    StaleView,
}

pub struct DiagnosticLog {
    events: Vec<DiagnosticEvent>,
    max: usize,
}

impl Default for DiagnosticLog {
    fn default() -> Self {
        Self::new(64)
    }
}

impl DiagnosticLog {
    pub fn new(max: usize) -> Self {
        Self {
            events: Vec::new(),
            max,
        }
    }

    pub fn push(&mut self, ev: DiagnosticEvent) {
        if self.events.len() >= self.max {
            self.events.remove(0);
        }
        self.events.push(ev);
    }

    pub fn events(&self) -> &[DiagnosticEvent] {
        &self.events
    }

    pub fn last_stale(&self) -> bool {
        self.events
            .iter()
            .any(|e| matches!(e, DiagnosticEvent::StaleView))
    }

    pub fn format_trace(&self) -> String {
        self.events
            .iter()
            .map(|e| format!("{e:?}"))
            .collect::<Vec<_>>()
            .join(" -> ")
    }
}

pub fn header_progress(buffered: usize) -> f32 {
    (buffered as f32 / HEADER_LEN as f32).min(1.0)
}

pub fn payload_progress(buffered: usize, payload_len: u32) -> f32 {
    if payload_len == 0 {
        return 1.0;
    }
    (buffered as f32 / payload_len as f32).min(1.0)
}

pub fn trailer_progress(buffered: usize) -> f32 {
    (buffered as f32 / TRAILER_LEN as f32).min(1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_stale_detection() {
        let mut log = DiagnosticLog::new(4);
        log.push(DiagnosticEvent::StaleView);
        assert!(log.last_stale());
    }

    #[test]
    fn progress_monotonic() {
        assert!(header_progress(4) <= header_progress(8));
    }
}
