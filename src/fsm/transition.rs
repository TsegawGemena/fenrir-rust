use super::events::FsmEvent;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DecodeState {
    Idle, AwaitHeader, AwaitPayload, AwaitTrailer, Verify, Complete, Failed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transition { pub from: DecodeState, pub event: FsmEvent, pub to: DecodeState }

#[derive(Debug, Default, Clone)]
pub struct TransitionTable { rows: Vec<Transition> }

impl TransitionTable {
    pub fn new() -> Self { Self { rows: Vec::new() } }
    pub fn add(mut self, from: DecodeState, event: FsmEvent, to: DecodeState) -> Self {
        self.rows.push(Transition { from, event, to });
        self
    }

    pub fn next(&self, from: DecodeState, event: &FsmEvent) -> Option<DecodeState> {
        self.rows.iter().find(|t| t.from == from && Self::event_matches(&t.event, event)).map(|t| t.to)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Transition> {
        self.rows.iter()
    }

    fn event_matches(expected: &FsmEvent, actual: &FsmEvent) -> bool {
        match (expected, actual) {
            (FsmEvent::BytesReceived(_), FsmEvent::BytesReceived(_)) => true,
            (FsmEvent::Error(_), FsmEvent::Error(_)) => true,
            (a, b) => a == b,
        }
    }

    pub fn standard() -> Self {
        Self::new()
            .add(DecodeState::Idle, FsmEvent::BytesReceived(0), DecodeState::AwaitHeader)
            .add(DecodeState::AwaitHeader, FsmEvent::HeaderComplete, DecodeState::AwaitPayload)
            .add(DecodeState::AwaitPayload, FsmEvent::PayloadComplete, DecodeState::AwaitTrailer)
            .add(DecodeState::AwaitTrailer, FsmEvent::TrailerComplete, DecodeState::Verify)
            .add(DecodeState::Verify, FsmEvent::ChecksumOk, DecodeState::Complete)
            .add(DecodeState::Verify, FsmEvent::ChecksumFail, DecodeState::Failed)
            .add(DecodeState::Complete, FsmEvent::Reset, DecodeState::Idle)
    }
}
impl TransitionTable {
    pub fn variant_0() -> Self {
        let mut t = Self::standard();
        t.rows.push(Transition {
            from: DecodeState::AwaitPayload,
            event: FsmEvent::NeedMore,
            to: DecodeState::AwaitPayload,
        });
        t
    }
}
impl TransitionTable {
    pub fn variant_1() -> Self {
        let mut t = Self::standard();
        t.rows.push(Transition {
            from: DecodeState::AwaitPayload,
            event: FsmEvent::NeedMore,
            to: DecodeState::AwaitPayload,
        });
        t
    }
}
impl TransitionTable {
    pub fn variant_2() -> Self {
        let mut t = Self::standard();
        t.rows.push(Transition {
            from: DecodeState::AwaitPayload,
            event: FsmEvent::NeedMore,
            to: DecodeState::AwaitPayload,
        });
        t
    }
}
impl TransitionTable {
    pub fn variant_3() -> Self {
        let mut t = Self::standard();
        t.rows.push(Transition {
            from: DecodeState::AwaitPayload,
            event: FsmEvent::NeedMore,
            to: DecodeState::AwaitPayload,
        });
        t
    }
}
impl TransitionTable {
    pub fn variant_4() -> Self {
        let mut t = Self::standard();
        t.rows.push(Transition {
            from: DecodeState::AwaitPayload,
            event: FsmEvent::NeedMore,
            to: DecodeState::AwaitPayload,
        });
        t
    }
}
impl TransitionTable {
    pub fn variant_5() -> Self {
        let mut t = Self::standard();
        t.rows.push(Transition {
            from: DecodeState::AwaitPayload,
            event: FsmEvent::NeedMore,
            to: DecodeState::AwaitPayload,
        });
        t
    }
}
impl TransitionTable {
    pub fn variant_6() -> Self {
        let mut t = Self::standard();
        t.rows.push(Transition {
            from: DecodeState::AwaitPayload,
            event: FsmEvent::NeedMore,
            to: DecodeState::AwaitPayload,
        });
        t
    }
}
impl TransitionTable {
    pub fn variant_7() -> Self {
        let mut t = Self::standard();
        t.rows.push(Transition {
            from: DecodeState::AwaitPayload,
            event: FsmEvent::NeedMore,
            to: DecodeState::AwaitPayload,
        });
        t
    }
}
impl TransitionTable {
    pub fn variant_8() -> Self {
        let mut t = Self::standard();
        t.rows.push(Transition {
            from: DecodeState::AwaitPayload,
            event: FsmEvent::NeedMore,
            to: DecodeState::AwaitPayload,
        });
        t
    }
}
impl TransitionTable {
    pub fn variant_9() -> Self {
        let mut t = Self::standard();
        t.rows.push(Transition {
            from: DecodeState::AwaitPayload,
            event: FsmEvent::NeedMore,
            to: DecodeState::AwaitPayload,
        });
        t
    }
}
impl TransitionTable {
    pub fn variant_10() -> Self {
        let mut t = Self::standard();
        t.rows.push(Transition {
            from: DecodeState::AwaitPayload,
            event: FsmEvent::NeedMore,
            to: DecodeState::AwaitPayload,
        });
        t
    }
}
impl TransitionTable {
    pub fn variant_11() -> Self {
        let mut t = Self::standard();
        t.rows.push(Transition {
            from: DecodeState::AwaitPayload,
            event: FsmEvent::NeedMore,
            to: DecodeState::AwaitPayload,
        });
        t
    }
}
impl TransitionTable {
    pub fn variant_12() -> Self {
        let mut t = Self::standard();
        t.rows.push(Transition {
            from: DecodeState::AwaitPayload,
            event: FsmEvent::NeedMore,
            to: DecodeState::AwaitPayload,
        });
        t
    }
}
impl TransitionTable {
    pub fn variant_13() -> Self {
        let mut t = Self::standard();
        t.rows.push(Transition {
            from: DecodeState::AwaitPayload,
            event: FsmEvent::NeedMore,
            to: DecodeState::AwaitPayload,
        });
        t
    }
}
impl TransitionTable {
    pub fn variant_14() -> Self {
        let mut t = Self::standard();
        t.rows.push(Transition {
            from: DecodeState::AwaitPayload,
            event: FsmEvent::NeedMore,
            to: DecodeState::AwaitPayload,
        });
        t
    }
}
impl TransitionTable {
    pub fn variant_15() -> Self {
        let mut t = Self::standard();
        t.rows.push(Transition {
            from: DecodeState::AwaitPayload,
            event: FsmEvent::NeedMore,
            to: DecodeState::AwaitPayload,
        });
        t
    }
}
impl TransitionTable {
    pub fn variant_16() -> Self {
        let mut t = Self::standard();
        t.rows.push(Transition {
            from: DecodeState::AwaitPayload,
            event: FsmEvent::NeedMore,
            to: DecodeState::AwaitPayload,
        });
        t
    }
}
impl TransitionTable {
    pub fn variant_17() -> Self {
        let mut t = Self::standard();
        t.rows.push(Transition {
            from: DecodeState::AwaitPayload,
            event: FsmEvent::NeedMore,
            to: DecodeState::AwaitPayload,
        });
        t
    }
}
impl TransitionTable {
    pub fn variant_18() -> Self {
        let mut t = Self::standard();
        t.rows.push(Transition {
            from: DecodeState::AwaitPayload,
            event: FsmEvent::NeedMore,
            to: DecodeState::AwaitPayload,
        });
        t
    }
}
impl TransitionTable {
    pub fn variant_19() -> Self {
        let mut t = Self::standard();
        t.rows.push(Transition {
            from: DecodeState::AwaitPayload,
            event: FsmEvent::NeedMore,
            to: DecodeState::AwaitPayload,
        });
        t
    }
}
