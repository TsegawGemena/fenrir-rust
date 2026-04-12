use crate::parser::MAGIC;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResyncState {
    Scanning,
    FoundMagic,
}

pub struct ResyncScanner {
    window: Vec<u8>,
    max_window: usize,
    state: ResyncState,
}

impl ResyncScanner {
    pub fn new(max_window: usize) -> Self {
        Self {
            window: Vec::with_capacity(max_window.min(4096)),
            max_window,
            state: ResyncState::Scanning,
        }
    }

    pub fn reset(&mut self) {
        self.window.clear();
        self.state = ResyncState::Scanning;
    }

    pub fn feed(&mut self, chunk: &[u8]) -> Option<usize> {
        self.window.extend_from_slice(chunk);
        if self.window.len() > self.max_window {
            let drop = self.window.len() - self.max_window;
            self.window.drain(0..drop);
        }
        if let Some(pos) = self.window.windows(4).position(|w| w == MAGIC) {
            self.state = ResyncState::FoundMagic;
            return Some(pos);
        }
        None
    }

    pub fn state(&self) -> ResyncState {
        self.state
    }

    pub fn window_len(&self) -> usize {
        self.window.len()
    }

    pub fn window_slice(&self) -> &[u8] {
        &self.window
    }
}
