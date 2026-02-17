use super::context::ServiceContext;
use crate::error::DecodeError;
use crate::session::{DecodeEvent, Frame};

pub struct HandlerResult { pub frames: Vec<Frame>, pub bytes_consumed: usize }

pub struct DecodeHandler;

impl DecodeHandler {
    pub fn handle(ctx: &mut ServiceContext, chunk: &[u8]) -> Result<HandlerResult, DecodeError> {
        let events = ctx.session.feed(chunk)?;
        let mut frames = Vec::new();
        for ev in events {
            if let DecodeEvent::Frame(f) = ev { frames.push(f); }
        }
        let fam = ctx.metrics.family("decode");
        fam.record_feed(chunk.len(), frames.len(), false);
        Ok(HandlerResult { frames, bytes_consumed: chunk.len() })
    }
}
impl DecodeHandler {
    pub fn handle_with_limit_0(ctx: &mut ServiceContext, chunk: &[u8], max: usize) -> Result<HandlerResult, DecodeError> {
        let slice = if chunk.len() > max { &chunk[..max] } else { chunk };
        Self::handle(ctx, slice)
    }
}
impl DecodeHandler {
    pub fn handle_with_limit_1(ctx: &mut ServiceContext, chunk: &[u8], max: usize) -> Result<HandlerResult, DecodeError> {
        let slice = if chunk.len() > max { &chunk[..max] } else { chunk };
        Self::handle(ctx, slice)
    }
}
impl DecodeHandler {
    pub fn handle_with_limit_2(ctx: &mut ServiceContext, chunk: &[u8], max: usize) -> Result<HandlerResult, DecodeError> {
        let slice = if chunk.len() > max { &chunk[..max] } else { chunk };
        Self::handle(ctx, slice)
    }
}
impl DecodeHandler {
    pub fn handle_with_limit_3(ctx: &mut ServiceContext, chunk: &[u8], max: usize) -> Result<HandlerResult, DecodeError> {
        let slice = if chunk.len() > max { &chunk[..max] } else { chunk };
        Self::handle(ctx, slice)
    }
}
impl DecodeHandler {
    pub fn handle_with_limit_4(ctx: &mut ServiceContext, chunk: &[u8], max: usize) -> Result<HandlerResult, DecodeError> {
        let slice = if chunk.len() > max { &chunk[..max] } else { chunk };
        Self::handle(ctx, slice)
    }
}
impl DecodeHandler {
    pub fn handle_with_limit_5(ctx: &mut ServiceContext, chunk: &[u8], max: usize) -> Result<HandlerResult, DecodeError> {
        let slice = if chunk.len() > max { &chunk[..max] } else { chunk };
        Self::handle(ctx, slice)
    }
}
impl DecodeHandler {
    pub fn handle_with_limit_6(ctx: &mut ServiceContext, chunk: &[u8], max: usize) -> Result<HandlerResult, DecodeError> {
        let slice = if chunk.len() > max { &chunk[..max] } else { chunk };
        Self::handle(ctx, slice)
    }
}
impl DecodeHandler {
    pub fn handle_with_limit_7(ctx: &mut ServiceContext, chunk: &[u8], max: usize) -> Result<HandlerResult, DecodeError> {
        let slice = if chunk.len() > max { &chunk[..max] } else { chunk };
        Self::handle(ctx, slice)
    }
}
impl DecodeHandler {
    pub fn handle_with_limit_8(ctx: &mut ServiceContext, chunk: &[u8], max: usize) -> Result<HandlerResult, DecodeError> {
        let slice = if chunk.len() > max { &chunk[..max] } else { chunk };
        Self::handle(ctx, slice)
    }
}
impl DecodeHandler {
    pub fn handle_with_limit_9(ctx: &mut ServiceContext, chunk: &[u8], max: usize) -> Result<HandlerResult, DecodeError> {
        let slice = if chunk.len() > max { &chunk[..max] } else { chunk };
        Self::handle(ctx, slice)
    }
}
impl DecodeHandler {
    pub fn handle_with_limit_10(ctx: &mut ServiceContext, chunk: &[u8], max: usize) -> Result<HandlerResult, DecodeError> {
        let slice = if chunk.len() > max { &chunk[..max] } else { chunk };
        Self::handle(ctx, slice)
    }
}
impl DecodeHandler {
    pub fn handle_with_limit_11(ctx: &mut ServiceContext, chunk: &[u8], max: usize) -> Result<HandlerResult, DecodeError> {
        let slice = if chunk.len() > max { &chunk[..max] } else { chunk };
        Self::handle(ctx, slice)
    }
}
impl DecodeHandler {
    pub fn handle_with_limit_12(ctx: &mut ServiceContext, chunk: &[u8], max: usize) -> Result<HandlerResult, DecodeError> {
        let slice = if chunk.len() > max { &chunk[..max] } else { chunk };
        Self::handle(ctx, slice)
    }
}
impl DecodeHandler {
    pub fn handle_with_limit_13(ctx: &mut ServiceContext, chunk: &[u8], max: usize) -> Result<HandlerResult, DecodeError> {
        let slice = if chunk.len() > max { &chunk[..max] } else { chunk };
        Self::handle(ctx, slice)
    }
}
impl DecodeHandler {
    pub fn handle_with_limit_14(ctx: &mut ServiceContext, chunk: &[u8], max: usize) -> Result<HandlerResult, DecodeError> {
        let slice = if chunk.len() > max { &chunk[..max] } else { chunk };
        Self::handle(ctx, slice)
    }
}
impl DecodeHandler {
    pub fn handle_with_limit_15(ctx: &mut ServiceContext, chunk: &[u8], max: usize) -> Result<HandlerResult, DecodeError> {
        let slice = if chunk.len() > max { &chunk[..max] } else { chunk };
        Self::handle(ctx, slice)
    }
}
impl DecodeHandler {
    pub fn handle_with_limit_16(ctx: &mut ServiceContext, chunk: &[u8], max: usize) -> Result<HandlerResult, DecodeError> {
        let slice = if chunk.len() > max { &chunk[..max] } else { chunk };
        Self::handle(ctx, slice)
    }
}
impl DecodeHandler {
    pub fn handle_with_limit_17(ctx: &mut ServiceContext, chunk: &[u8], max: usize) -> Result<HandlerResult, DecodeError> {
        let slice = if chunk.len() > max { &chunk[..max] } else { chunk };
        Self::handle(ctx, slice)
    }
}
impl DecodeHandler {
    pub fn handle_with_limit_18(ctx: &mut ServiceContext, chunk: &[u8], max: usize) -> Result<HandlerResult, DecodeError> {
        let slice = if chunk.len() > max { &chunk[..max] } else { chunk };
        Self::handle(ctx, slice)
    }
}
impl DecodeHandler {
    pub fn handle_with_limit_19(ctx: &mut ServiceContext, chunk: &[u8], max: usize) -> Result<HandlerResult, DecodeError> {
        let slice = if chunk.len() > max { &chunk[..max] } else { chunk };
        Self::handle(ctx, slice)
    }
}
impl DecodeHandler {
    pub fn handle_with_limit_20(ctx: &mut ServiceContext, chunk: &[u8], max: usize) -> Result<HandlerResult, DecodeError> {
        let slice = if chunk.len() > max { &chunk[..max] } else { chunk };
        Self::handle(ctx, slice)
    }
}
impl DecodeHandler {
    pub fn handle_with_limit_21(ctx: &mut ServiceContext, chunk: &[u8], max: usize) -> Result<HandlerResult, DecodeError> {
        let slice = if chunk.len() > max { &chunk[..max] } else { chunk };
        Self::handle(ctx, slice)
    }
}
impl DecodeHandler {
    pub fn handle_with_limit_22(ctx: &mut ServiceContext, chunk: &[u8], max: usize) -> Result<HandlerResult, DecodeError> {
        let slice = if chunk.len() > max { &chunk[..max] } else { chunk };
        Self::handle(ctx, slice)
    }
}
impl DecodeHandler {
    pub fn handle_with_limit_23(ctx: &mut ServiceContext, chunk: &[u8], max: usize) -> Result<HandlerResult, DecodeError> {
        let slice = if chunk.len() > max { &chunk[..max] } else { chunk };
        Self::handle(ctx, slice)
    }
}
impl DecodeHandler {
    pub fn handle_with_limit_24(ctx: &mut ServiceContext, chunk: &[u8], max: usize) -> Result<HandlerResult, DecodeError> {
        let slice = if chunk.len() > max { &chunk[..max] } else { chunk };
        Self::handle(ctx, slice)
    }
}
impl DecodeHandler {
    pub fn handle_with_limit_25(ctx: &mut ServiceContext, chunk: &[u8], max: usize) -> Result<HandlerResult, DecodeError> {
        let slice = if chunk.len() > max { &chunk[..max] } else { chunk };
        Self::handle(ctx, slice)
    }
}
impl DecodeHandler {
    pub fn handle_with_limit_26(ctx: &mut ServiceContext, chunk: &[u8], max: usize) -> Result<HandlerResult, DecodeError> {
        let slice = if chunk.len() > max { &chunk[..max] } else { chunk };
        Self::handle(ctx, slice)
    }
}
impl DecodeHandler {
    pub fn handle_with_limit_27(ctx: &mut ServiceContext, chunk: &[u8], max: usize) -> Result<HandlerResult, DecodeError> {
        let slice = if chunk.len() > max { &chunk[..max] } else { chunk };
        Self::handle(ctx, slice)
    }
}
impl DecodeHandler {
    pub fn handle_with_limit_28(ctx: &mut ServiceContext, chunk: &[u8], max: usize) -> Result<HandlerResult, DecodeError> {
        let slice = if chunk.len() > max { &chunk[..max] } else { chunk };
        Self::handle(ctx, slice)
    }
}
impl DecodeHandler {
    pub fn handle_with_limit_29(ctx: &mut ServiceContext, chunk: &[u8], max: usize) -> Result<HandlerResult, DecodeError> {
        let slice = if chunk.len() > max { &chunk[..max] } else { chunk };
        Self::handle(ctx, slice)
    }
}
impl DecodeHandler {
    pub fn handle_with_limit_30(ctx: &mut ServiceContext, chunk: &[u8], max: usize) -> Result<HandlerResult, DecodeError> {
        let slice = if chunk.len() > max { &chunk[..max] } else { chunk };
        Self::handle(ctx, slice)
    }
}
impl DecodeHandler {
    pub fn handle_with_limit_31(ctx: &mut ServiceContext, chunk: &[u8], max: usize) -> Result<HandlerResult, DecodeError> {
        let slice = if chunk.len() > max { &chunk[..max] } else { chunk };
        Self::handle(ctx, slice)
    }
}
impl DecodeHandler {
    pub fn handle_with_limit_32(ctx: &mut ServiceContext, chunk: &[u8], max: usize) -> Result<HandlerResult, DecodeError> {
        let slice = if chunk.len() > max { &chunk[..max] } else { chunk };
        Self::handle(ctx, slice)
    }
}
impl DecodeHandler {
    pub fn handle_with_limit_33(ctx: &mut ServiceContext, chunk: &[u8], max: usize) -> Result<HandlerResult, DecodeError> {
        let slice = if chunk.len() > max { &chunk[..max] } else { chunk };
        Self::handle(ctx, slice)
    }
}
impl DecodeHandler {
    pub fn handle_with_limit_34(ctx: &mut ServiceContext, chunk: &[u8], max: usize) -> Result<HandlerResult, DecodeError> {
        let slice = if chunk.len() > max { &chunk[..max] } else { chunk };
        Self::handle(ctx, slice)
    }
}
