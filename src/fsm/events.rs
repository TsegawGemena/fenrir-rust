use crate::error::DecodeError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FsmEvent {
    BytesReceived(usize),
    HeaderComplete,
    PayloadComplete,
    TrailerComplete,
    ChecksumOk,
    ChecksumFail,
    NeedMore,
    Reset,
    Error(DecodeError),
}

impl FsmEvent {
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::ChecksumOk | Self::ChecksumFail | Self::Error(_))
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::BytesReceived(_) => "bytes",
            Self::HeaderComplete => "header",
            Self::PayloadComplete => "payload",
            Self::TrailerComplete => "trailer",
            Self::ChecksumOk => "ok",
            Self::ChecksumFail => "fail",
            Self::NeedMore => "need_more",
            Self::Reset => "reset",
            Self::Error(_) => "error",
        }
    }
}
