//! Multi-stage frame parser: header → payload → trailer → complete.

use crate::buffer::{BufferPool, BufferView, SharedBuffer};
use crate::error::DecodeError;

pub const MAGIC: [u8; 4] = *b"FENR";
pub const HEADER_LEN: usize = 8;
pub const TRAILER_LEN: usize = 4;
pub const MAX_PAYLOAD_LEN: u32 = 1 << 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    Header,
    Payload,
    Trailer,
    Complete,
}

/// Partial parse state carried across incremental `feed` calls.
#[derive(Debug)]
pub struct PartialState {
    pub stage: Stage,
    pub payload_len: u32,
    /// Zero-copy view of the completed payload region.
    pub payload_view: Option<BufferView>,
    /// Zero-copy view of the trailer once available.
    pub trailer_view: Option<BufferView>,
    /// XOR checksum over payload bytes.
    pub checksum: u32,
}

impl PartialState {
    pub fn new() -> Self {
        Self {
            stage: Stage::Header,
            payload_len: 0,
            payload_view: None,
            trailer_view: None,
            checksum: 0,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn bytes_needed(&self, buffered: usize) -> usize {
        match self.stage {
            Stage::Header => HEADER_LEN.saturating_sub(buffered),
            Stage::Payload => (self.payload_len as usize).saturating_sub(buffered),
            Stage::Trailer => TRAILER_LEN.saturating_sub(buffered),
            Stage::Complete => 0,
        }
    }

    pub fn feed_checksum(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.checksum ^= u32::from(b);
        }
    }
}

impl Default for PartialState {
    fn default() -> Self {
        Self::new()
    }
}

/// Owns the working buffer and drives stage transitions.
#[derive(Debug)]
pub struct FrameParser {
    pub state: PartialState,
    working: SharedBuffer,
    write_cursor: usize,
    released: bool,
    pool: *mut BufferPool,
}

impl FrameParser {
    pub fn new(pool: &mut BufferPool) -> Result<Self, DecodeError> {
        Ok(Self {
            state: PartialState::new(),
            working: pool.acquire()?,
            write_cursor: 0,
            released: false,
            pool: pool as *mut BufferPool,
        })
    }

    fn pool(&mut self) -> &mut BufferPool {
        // SAFETY: `DecodeSession` owns the pool and outlives the parser.
        unsafe { &mut *self.pool }
    }

    pub fn working_buffer(&self) -> &SharedBuffer {
        &self.working
    }

    pub fn write_cursor(&self) -> usize {
        self.write_cursor
    }

    pub fn has_live_views(&self) -> bool {
        self.state.payload_view.is_some() || self.state.trailer_view.is_some()
    }

    /// Trailer view aliases `working`; payload view owns a separate buffer after `finish_payload`.
    pub fn trailer_pinned_on_working(&self) -> bool {
        self.state.trailer_view.is_some()
    }

    pub fn append(&mut self, data: &[u8]) -> Result<usize, DecodeError> {
        let room = self.working.capacity().saturating_sub(self.write_cursor);
        let n = data.len().min(room);
        if n > 0 {
            self.working.write_at(self.write_cursor, &data[..n])?;
            self.write_cursor += n;
        }
        Ok(n)
    }

    /// Advance stages while enough bytes are buffered.
    pub fn try_advance_stages(&mut self) -> Result<(), DecodeError> {
        loop {
            let needed = self.state.bytes_needed(self.write_cursor);
            if needed > 0 {
                break;
            }

            match self.state.stage {
                Stage::Header => self.finish_header()?,
                Stage::Payload => self.finish_payload()?,
                Stage::Trailer => {
                    self.finish_trailer()?;
                    break;
                }
                Stage::Complete => break,
            }
        }
        Ok(())
    }

    fn finish_header(&mut self) -> Result<(), DecodeError> {
        let header = &self.working.as_slice()[0..HEADER_LEN];
        let magic: [u8; 4] = header[0..4].try_into().unwrap();
        if magic != MAGIC {
            return Err(DecodeError::InvalidMagic { got: magic });
        }

        let payload_len = u32::from_le_bytes(header[4..8].try_into().unwrap());
        if payload_len > MAX_PAYLOAD_LEN {
            return Err(DecodeError::PayloadTooLarge {
                len: payload_len,
                max: MAX_PAYLOAD_LEN,
            });
        }

        self.state.payload_len = payload_len;
        self.state.stage = Stage::Payload;

        // Compact bytes after the header into the front of the working buffer.
        if self.write_cursor > HEADER_LEN {
            let remaining = self.write_cursor - HEADER_LEN;
            self.working
                .as_mut_slice()
                .copy_within(HEADER_LEN..self.write_cursor, 0);
            self.write_cursor = remaining;
        } else {
            self.write_cursor = 0;
        }
        Ok(())
    }

    fn finish_payload(&mut self) -> Result<(), DecodeError> {
        let len = self.state.payload_len as usize;
        let payload = &self.working.as_slice()[0..len];
        self.state.feed_checksum(payload);

        let trailer_prefix = if self.write_cursor > len {
            self.working.as_slice()[len..self.write_cursor].to_vec()
        } else {
            Vec::new()
        };

        let mut view = BufferView::new(&self.working, 0, len);
        let payload_buf = self.take_working_buffer()?;
        view.bind_owner(payload_buf);
        self.state.payload_view = Some(view);

        self.state.stage = Stage::Trailer;
        let mut fresh = self.pool().acquire()?;
        if !trailer_prefix.is_empty() {
            fresh.write_at(0, &trailer_prefix)?;
            self.write_cursor = trailer_prefix.len();
        } else {
            self.write_cursor = 0;
        }
        self.working = fresh;
        self.released = false;
        Ok(())
    }

    fn finish_trailer(&mut self) -> Result<(), DecodeError> {
        let trailer = BufferView::new(&self.working, 0, TRAILER_LEN);
        self.state.trailer_view = Some(trailer);
        self.state.stage = Stage::Complete;
        Ok(())
    }

    pub fn is_complete(&self) -> bool {
        self.state.stage == Stage::Complete
    }

    pub fn expected_checksum(&self) -> u32 {
        self.state.checksum
    }

    pub fn take_working_buffer(&mut self) -> Result<SharedBuffer, DecodeError> {
        if self.released {
            return Err(DecodeError::BufferAlreadyReleased);
        }
        self.released = true;
        self.write_cursor = 0;
        let buf = std::mem::replace(
            &mut self.working,
            SharedBuffer::with_capacity(0, 0),
        );
        Ok(buf)
    }

    pub fn snapshot_working(&self) -> Vec<u8> {
        if self.write_cursor == 0 {
            Vec::new()
        } else {
            self.working.as_slice()[..self.write_cursor].to_vec()
        }
    }

    pub fn set_working_buffer(&mut self, buf: SharedBuffer) {
        self.working = buf;
        self.released = false;
        self.write_cursor = 0;
    }

    pub fn is_released(&self) -> bool {
        self.released
    }

    pub fn clear_views(&mut self) {
        self.state.payload_view = None;
        self.state.trailer_view = None;
    }

    /// Move buffered bytes into a fresh working buffer (only when no views are pinned).
    pub fn reinstall_working(&mut self, fresh: SharedBuffer, saved: Vec<u8>) -> Result<(), DecodeError> {
        self.working = fresh;
        self.released = false;
        self.write_cursor = 0;
        if !saved.is_empty() {
            self.working.write_at(0, &saved)?;
            self.write_cursor = saved.len();
        }
        Ok(())
    }
}
