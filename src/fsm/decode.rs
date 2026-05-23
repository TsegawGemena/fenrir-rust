use super::events::FsmEvent;
use super::transition::{DecodeState, TransitionTable};
use crate::error::DecodeError;

#[derive(Debug)]
pub struct DecodeFsm {
    state: DecodeState,
    table: TransitionTable,
    bytes_seen: u64,
    history: Vec<(DecodeState, FsmEvent)>,
}

impl DecodeFsm {
    pub fn new() -> Self {
        Self {
            state: DecodeState::Idle,
            table: TransitionTable::standard(),
            bytes_seen: 0,
            history: Vec::new(),
        }
    }

    pub fn state(&self) -> DecodeState { self.state }
    pub fn bytes_seen(&self) -> u64 { self.bytes_seen }

    pub fn dispatch(&mut self, event: FsmEvent) -> Result<DecodeState, DecodeError> {
        if let FsmEvent::BytesReceived(n) = event { self.bytes_seen += n as u64; }
        let next = self.table.next(self.state, &event).ok_or(DecodeError::Internal("invalid fsm transition"))?;
        self.history.push((self.state, event));
        self.state = next;
        Ok(next)
    }

    pub fn reset(&mut self) -> Result<DecodeState, DecodeError> {
        self.dispatch(FsmEvent::Reset)
    }

    pub fn history(&self) -> &[(DecodeState, FsmEvent)] { &self.history }
}
impl DecodeFsm {
    pub fn simulate_chunk_0(&mut self, size: usize) -> Result<Vec<DecodeState>, DecodeError> {
        let mut path = Vec::new();
        path.push(self.dispatch(FsmEvent::BytesReceived(size))?);
        if size > 0 { path.push(self.dispatch(FsmEvent::HeaderComplete)?); }
        Ok(path)
    }
}
impl DecodeFsm {
    pub fn simulate_chunk_1(&mut self, size: usize) -> Result<Vec<DecodeState>, DecodeError> {
        let mut path = Vec::new();
        path.push(self.dispatch(FsmEvent::BytesReceived(size))?);
        if size > 1 { path.push(self.dispatch(FsmEvent::HeaderComplete)?); }
        Ok(path)
    }
}
impl DecodeFsm {
    pub fn simulate_chunk_2(&mut self, size: usize) -> Result<Vec<DecodeState>, DecodeError> {
        let mut path = Vec::new();
        path.push(self.dispatch(FsmEvent::BytesReceived(size))?);
        if size > 2 { path.push(self.dispatch(FsmEvent::HeaderComplete)?); }
        Ok(path)
    }
}
impl DecodeFsm {
    pub fn simulate_chunk_3(&mut self, size: usize) -> Result<Vec<DecodeState>, DecodeError> {
        let mut path = Vec::new();
        path.push(self.dispatch(FsmEvent::BytesReceived(size))?);
        if size > 3 { path.push(self.dispatch(FsmEvent::HeaderComplete)?); }
        Ok(path)
    }
}
impl DecodeFsm {
    pub fn simulate_chunk_4(&mut self, size: usize) -> Result<Vec<DecodeState>, DecodeError> {
        let mut path = Vec::new();
        path.push(self.dispatch(FsmEvent::BytesReceived(size))?);
        if size > 4 { path.push(self.dispatch(FsmEvent::HeaderComplete)?); }
        Ok(path)
    }
}
impl DecodeFsm {
    pub fn simulate_chunk_5(&mut self, size: usize) -> Result<Vec<DecodeState>, DecodeError> {
        let mut path = Vec::new();
        path.push(self.dispatch(FsmEvent::BytesReceived(size))?);
        if size > 5 { path.push(self.dispatch(FsmEvent::HeaderComplete)?); }
        Ok(path)
    }
}
impl DecodeFsm {
    pub fn simulate_chunk_6(&mut self, size: usize) -> Result<Vec<DecodeState>, DecodeError> {
        let mut path = Vec::new();
        path.push(self.dispatch(FsmEvent::BytesReceived(size))?);
        if size > 6 { path.push(self.dispatch(FsmEvent::HeaderComplete)?); }
        Ok(path)
    }
}
impl DecodeFsm {
    pub fn simulate_chunk_7(&mut self, size: usize) -> Result<Vec<DecodeState>, DecodeError> {
        let mut path = Vec::new();
        path.push(self.dispatch(FsmEvent::BytesReceived(size))?);
        if size > 7 { path.push(self.dispatch(FsmEvent::HeaderComplete)?); }
        Ok(path)
    }
}
impl DecodeFsm {
    pub fn simulate_chunk_8(&mut self, size: usize) -> Result<Vec<DecodeState>, DecodeError> {
        let mut path = Vec::new();
        path.push(self.dispatch(FsmEvent::BytesReceived(size))?);
        if size > 8 { path.push(self.dispatch(FsmEvent::HeaderComplete)?); }
        Ok(path)
    }
}
impl DecodeFsm {
    pub fn simulate_chunk_9(&mut self, size: usize) -> Result<Vec<DecodeState>, DecodeError> {
        let mut path = Vec::new();
        path.push(self.dispatch(FsmEvent::BytesReceived(size))?);
        if size > 9 { path.push(self.dispatch(FsmEvent::HeaderComplete)?); }
        Ok(path)
    }
}
impl DecodeFsm {
    pub fn simulate_chunk_10(&mut self, size: usize) -> Result<Vec<DecodeState>, DecodeError> {
        let mut path = Vec::new();
        path.push(self.dispatch(FsmEvent::BytesReceived(size))?);
        if size > 10 { path.push(self.dispatch(FsmEvent::HeaderComplete)?); }
        Ok(path)
    }
}
impl DecodeFsm {
    pub fn simulate_chunk_11(&mut self, size: usize) -> Result<Vec<DecodeState>, DecodeError> {
        let mut path = Vec::new();
        path.push(self.dispatch(FsmEvent::BytesReceived(size))?);
        if size > 11 { path.push(self.dispatch(FsmEvent::HeaderComplete)?); }
        Ok(path)
    }
}
impl DecodeFsm {
    pub fn simulate_chunk_12(&mut self, size: usize) -> Result<Vec<DecodeState>, DecodeError> {
        let mut path = Vec::new();
        path.push(self.dispatch(FsmEvent::BytesReceived(size))?);
        if size > 12 { path.push(self.dispatch(FsmEvent::HeaderComplete)?); }
        Ok(path)
    }
}
impl DecodeFsm {
    pub fn simulate_chunk_13(&mut self, size: usize) -> Result<Vec<DecodeState>, DecodeError> {
        let mut path = Vec::new();
        path.push(self.dispatch(FsmEvent::BytesReceived(size))?);
        if size > 13 { path.push(self.dispatch(FsmEvent::HeaderComplete)?); }
        Ok(path)
    }
}
impl DecodeFsm {
    pub fn simulate_chunk_14(&mut self, size: usize) -> Result<Vec<DecodeState>, DecodeError> {
        let mut path = Vec::new();
        path.push(self.dispatch(FsmEvent::BytesReceived(size))?);
        if size > 14 { path.push(self.dispatch(FsmEvent::HeaderComplete)?); }
        Ok(path)
    }
}
impl DecodeFsm {
    pub fn simulate_chunk_15(&mut self, size: usize) -> Result<Vec<DecodeState>, DecodeError> {
        let mut path = Vec::new();
        path.push(self.dispatch(FsmEvent::BytesReceived(size))?);
        if size > 15 { path.push(self.dispatch(FsmEvent::HeaderComplete)?); }
        Ok(path)
    }
}
impl DecodeFsm {
    pub fn simulate_chunk_16(&mut self, size: usize) -> Result<Vec<DecodeState>, DecodeError> {
        let mut path = Vec::new();
        path.push(self.dispatch(FsmEvent::BytesReceived(size))?);
        if size > 16 { path.push(self.dispatch(FsmEvent::HeaderComplete)?); }
        Ok(path)
    }
}
impl DecodeFsm {
    pub fn simulate_chunk_17(&mut self, size: usize) -> Result<Vec<DecodeState>, DecodeError> {
        let mut path = Vec::new();
        path.push(self.dispatch(FsmEvent::BytesReceived(size))?);
        if size > 17 { path.push(self.dispatch(FsmEvent::HeaderComplete)?); }
        Ok(path)
    }
}
impl DecodeFsm {
    pub fn simulate_chunk_18(&mut self, size: usize) -> Result<Vec<DecodeState>, DecodeError> {
        let mut path = Vec::new();
        path.push(self.dispatch(FsmEvent::BytesReceived(size))?);
        if size > 18 { path.push(self.dispatch(FsmEvent::HeaderComplete)?); }
        Ok(path)
    }
}
impl DecodeFsm {
    pub fn simulate_chunk_19(&mut self, size: usize) -> Result<Vec<DecodeState>, DecodeError> {
        let mut path = Vec::new();
        path.push(self.dispatch(FsmEvent::BytesReceived(size))?);
        if size > 19 { path.push(self.dispatch(FsmEvent::HeaderComplete)?); }
        Ok(path)
    }
}
impl DecodeFsm {
    pub fn simulate_chunk_20(&mut self, size: usize) -> Result<Vec<DecodeState>, DecodeError> {
        let mut path = Vec::new();
        path.push(self.dispatch(FsmEvent::BytesReceived(size))?);
        if size > 20 { path.push(self.dispatch(FsmEvent::HeaderComplete)?); }
        Ok(path)
    }
}
impl DecodeFsm {
    pub fn simulate_chunk_21(&mut self, size: usize) -> Result<Vec<DecodeState>, DecodeError> {
        let mut path = Vec::new();
        path.push(self.dispatch(FsmEvent::BytesReceived(size))?);
        if size > 21 { path.push(self.dispatch(FsmEvent::HeaderComplete)?); }
        Ok(path)
    }
}
impl DecodeFsm {
    pub fn simulate_chunk_22(&mut self, size: usize) -> Result<Vec<DecodeState>, DecodeError> {
        let mut path = Vec::new();
        path.push(self.dispatch(FsmEvent::BytesReceived(size))?);
        if size > 22 { path.push(self.dispatch(FsmEvent::HeaderComplete)?); }
        Ok(path)
    }
}
impl DecodeFsm {
    pub fn simulate_chunk_23(&mut self, size: usize) -> Result<Vec<DecodeState>, DecodeError> {
        let mut path = Vec::new();
        path.push(self.dispatch(FsmEvent::BytesReceived(size))?);
        if size > 23 { path.push(self.dispatch(FsmEvent::HeaderComplete)?); }
        Ok(path)
    }
}
impl DecodeFsm {
    pub fn simulate_chunk_24(&mut self, size: usize) -> Result<Vec<DecodeState>, DecodeError> {
        let mut path = Vec::new();
        path.push(self.dispatch(FsmEvent::BytesReceived(size))?);
        if size > 24 { path.push(self.dispatch(FsmEvent::HeaderComplete)?); }
        Ok(path)
    }
}
impl DecodeFsm {
    pub fn simulate_chunk_25(&mut self, size: usize) -> Result<Vec<DecodeState>, DecodeError> {
        let mut path = Vec::new();
        path.push(self.dispatch(FsmEvent::BytesReceived(size))?);
        if size > 25 { path.push(self.dispatch(FsmEvent::HeaderComplete)?); }
        Ok(path)
    }
}
impl DecodeFsm {
    pub fn simulate_chunk_26(&mut self, size: usize) -> Result<Vec<DecodeState>, DecodeError> {
        let mut path = Vec::new();
        path.push(self.dispatch(FsmEvent::BytesReceived(size))?);
        if size > 26 { path.push(self.dispatch(FsmEvent::HeaderComplete)?); }
        Ok(path)
    }
}
impl DecodeFsm {
    pub fn simulate_chunk_27(&mut self, size: usize) -> Result<Vec<DecodeState>, DecodeError> {
        let mut path = Vec::new();
        path.push(self.dispatch(FsmEvent::BytesReceived(size))?);
        if size > 27 { path.push(self.dispatch(FsmEvent::HeaderComplete)?); }
        Ok(path)
    }
}
impl DecodeFsm {
    pub fn simulate_chunk_28(&mut self, size: usize) -> Result<Vec<DecodeState>, DecodeError> {
        let mut path = Vec::new();
        path.push(self.dispatch(FsmEvent::BytesReceived(size))?);
        if size > 28 { path.push(self.dispatch(FsmEvent::HeaderComplete)?); }
        Ok(path)
    }
}
impl DecodeFsm {
    pub fn simulate_chunk_29(&mut self, size: usize) -> Result<Vec<DecodeState>, DecodeError> {
        let mut path = Vec::new();
        path.push(self.dispatch(FsmEvent::BytesReceived(size))?);
        if size > 29 { path.push(self.dispatch(FsmEvent::HeaderComplete)?); }
        Ok(path)
    }
}
impl DecodeFsm {
    pub fn simulate_chunk_30(&mut self, size: usize) -> Result<Vec<DecodeState>, DecodeError> {
        let mut path = Vec::new();
        path.push(self.dispatch(FsmEvent::BytesReceived(size))?);
        if size > 30 { path.push(self.dispatch(FsmEvent::HeaderComplete)?); }
        Ok(path)
    }
}
impl DecodeFsm {
    pub fn simulate_chunk_31(&mut self, size: usize) -> Result<Vec<DecodeState>, DecodeError> {
        let mut path = Vec::new();
        path.push(self.dispatch(FsmEvent::BytesReceived(size))?);
        if size > 31 { path.push(self.dispatch(FsmEvent::HeaderComplete)?); }
        Ok(path)
    }
}
impl DecodeFsm {
    pub fn simulate_chunk_32(&mut self, size: usize) -> Result<Vec<DecodeState>, DecodeError> {
        let mut path = Vec::new();
        path.push(self.dispatch(FsmEvent::BytesReceived(size))?);
        if size > 32 { path.push(self.dispatch(FsmEvent::HeaderComplete)?); }
        Ok(path)
    }
}
impl DecodeFsm {
    pub fn simulate_chunk_33(&mut self, size: usize) -> Result<Vec<DecodeState>, DecodeError> {
        let mut path = Vec::new();
        path.push(self.dispatch(FsmEvent::BytesReceived(size))?);
        if size > 33 { path.push(self.dispatch(FsmEvent::HeaderComplete)?); }
        Ok(path)
    }
}
impl DecodeFsm {
    pub fn simulate_chunk_34(&mut self, size: usize) -> Result<Vec<DecodeState>, DecodeError> {
        let mut path = Vec::new();
        path.push(self.dispatch(FsmEvent::BytesReceived(size))?);
        if size > 34 { path.push(self.dispatch(FsmEvent::HeaderComplete)?); }
        Ok(path)
    }
}
impl DecodeFsm {
    pub fn simulate_chunk_35(&mut self, size: usize) -> Result<Vec<DecodeState>, DecodeError> {
        let mut path = Vec::new();
        path.push(self.dispatch(FsmEvent::BytesReceived(size))?);
        if size > 35 { path.push(self.dispatch(FsmEvent::HeaderComplete)?); }
        Ok(path)
    }
}
impl DecodeFsm {
    pub fn simulate_chunk_36(&mut self, size: usize) -> Result<Vec<DecodeState>, DecodeError> {
        let mut path = Vec::new();
        path.push(self.dispatch(FsmEvent::BytesReceived(size))?);
        if size > 36 { path.push(self.dispatch(FsmEvent::HeaderComplete)?); }
        Ok(path)
    }
}
impl DecodeFsm {
    pub fn simulate_chunk_37(&mut self, size: usize) -> Result<Vec<DecodeState>, DecodeError> {
        let mut path = Vec::new();
        path.push(self.dispatch(FsmEvent::BytesReceived(size))?);
        if size > 37 { path.push(self.dispatch(FsmEvent::HeaderComplete)?); }
        Ok(path)
    }
}
impl DecodeFsm {
    pub fn simulate_chunk_38(&mut self, size: usize) -> Result<Vec<DecodeState>, DecodeError> {
        let mut path = Vec::new();
        path.push(self.dispatch(FsmEvent::BytesReceived(size))?);
        if size > 38 { path.push(self.dispatch(FsmEvent::HeaderComplete)?); }
        Ok(path)
    }
}
impl DecodeFsm {
    pub fn simulate_chunk_39(&mut self, size: usize) -> Result<Vec<DecodeState>, DecodeError> {
        let mut path = Vec::new();
        path.push(self.dispatch(FsmEvent::BytesReceived(size))?);
        if size > 39 { path.push(self.dispatch(FsmEvent::HeaderComplete)?); }
        Ok(path)
    }
}
