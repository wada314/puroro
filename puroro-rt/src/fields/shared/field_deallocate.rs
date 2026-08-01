//! Explicit field release through [`MessageCommon`] — shared by message `Drop`
//! and oneof variant teardown.

use ::allocator_api2::alloc::Allocator;

use super::MessageCommon;

/// Releases a field's owned payloads through the message allocator in `common`.
///
/// Generated message `Drop` imports this trait and calls it on **every direct
/// child** (scalars, bools, LEN, repeated, nested messages, and oneof
/// **groups**) with the same shape: `field.deallocate(&self._common)`. Copy /
/// bit-packed fields are no-ops.
///
/// Oneof **variants** are not message children; the group's `deallocate` takes
/// the active variant and frees it via [`OneofDeallocate`](crate::OneofDeallocate),
/// which in turn calls this trait on the variant's field wrapper.
///
/// After `deallocate`, `self` must not be used again (except as part of the
/// enclosing message / enum going out of scope).
///
/// `Pb` is unconstrained here: only impls that read presence / value bits
/// (e.g. explicit singular fields) add a [`PresenceBits`](super::PresenceBits)
/// bound.
pub trait FieldDeallocate<Pb, A: Allocator> {
    /// Frees heap payloads (if any) using `common.alloc`.
    fn deallocate(&mut self, common: &MessageCommon<Pb, A>);
}
