//! Invariant checks for buffer/view lifetime contracts.

use crate::buffer::{BufferView, SharedBuffer};
use crate::error::DecodeError;
use crate::parser::FrameParser;

/// View generation must match its owner at creation and read time.
#[inline]
pub fn validate_view_owner(
    view: &BufferView,
    owner: &SharedBuffer,
    _context: &str,
) -> Result<(), DecodeError> {
    if view.is_stale() {
        return Err(DecodeError::Internal("stale buffer view detected"));
    }
    if view.generation() != owner.generation() {
        return Err(DecodeError::Internal("stale buffer view detected"));
    }
    if !view.same_backing_as(owner) {
        return Err(DecodeError::Internal("stale buffer view detected"));
    }
    Ok(())
}

/// Debug-only mirror of [`validate_view_owner`].
#[inline]
pub fn check_view_owner(view: &BufferView, owner: &SharedBuffer, context: &str) {
    if let Err(e) = validate_view_owner(view, owner, context) {
        debug_assert!(false, "invariant violation ({context}): {e}");
    }
}

/// Working buffer must not be recycled while trailer_view aliases it.
#[inline]
pub fn validate_no_trailer_pin(parser: &FrameParser) -> Result<(), DecodeError> {
    if parser.trailer_pinned_on_working() {
        return Err(DecodeError::Internal(
            "working buffer too small while trailer view is pinned",
        ));
    }
    Ok(())
}

/// Debug-only mirror of [`validate_no_trailer_pin`].
#[inline]
pub fn check_no_trailer_pin(parser: &FrameParser, context: &str) {
    if let Err(e) = validate_no_trailer_pin(parser) {
        debug_assert!(false, "invariant violation ({context}): {e}");
    }
}

/// Frame must be in Complete stage when finalize begins.
#[inline]
pub fn validate_finalize_stage(parser: &FrameParser) -> Result<(), DecodeError> {
    if !parser.is_complete() {
        return Err(DecodeError::Internal("finalize called before Complete stage"));
    }
    Ok(())
}

/// Debug-only mirror of [`validate_finalize_stage`].
#[inline]
pub fn check_finalize_stage(parser: &FrameParser) {
    if let Err(e) = validate_finalize_stage(parser) {
        debug_assert!(false, "invariant violation: {e}");
    }
}
