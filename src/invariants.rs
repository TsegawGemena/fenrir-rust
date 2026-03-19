//! Debug invariant checks for buffer/view lifetime contracts.

use crate::buffer::{BufferView, SharedBuffer};
use crate::parser::FrameParser;

/// View generation must match its owner at creation and read time.
#[inline]
pub fn check_view_owner(view: &BufferView, owner: &SharedBuffer, context: &str) {
    debug_assert!(
        view.generation() == owner.generation(),
        "invariant violation ({context}): view generation {} != owner {}",
        view.generation(),
        owner.generation()
    );
    debug_assert!(!view.is_stale(), "invariant violation ({context}): stale view");
}

/// Working buffer must not be recycled while trailer_view aliases it.
#[inline]
pub fn check_no_trailer_pin(parser: &FrameParser, context: &str) {
    debug_assert!(
        !parser.state.trailer_view.is_some(),
        "invariant violation ({context}): trailer_view pinned on working buffer"
    );
}

/// Frame must be in Complete stage when finalize begins.
#[inline]
pub fn check_finalize_stage(parser: &FrameParser) {
    debug_assert!(
        parser.is_complete(),
        "invariant violation: finalize called before Complete stage"
    );
}
