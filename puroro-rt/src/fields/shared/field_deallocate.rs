//! Explicit field release through message common context — shared by message
//! `Drop` and oneof variant teardown.

/// Releases a field's owned payloads through the message allocator in `common`.
///
/// Generated message `Drop` imports this trait and calls it on **every direct
/// child** (scalars, bools, LEN, repeated, nested messages, and oneof
/// **groups**) with the same shape: `field.deallocate(&self._common)`. Copy /
/// bit-packed fields are no-ops.
///
/// `common` must be **that field's parent** `MessageCommon` — the same instance
/// whose allocator (and bitfield) was used when the payload was created. The
/// type system does not tie `self` to `common`; passing another message's
/// common is unsound.
///
/// Oneof **variants** are not message children; the group's `deallocate` takes
/// the active variant and frees it via [`OneofDeallocate`](crate::OneofDeallocate),
/// which in turn calls this trait on the variant's field wrapper.
///
/// After `deallocate`, `self` must not be used again (except as part of the
/// enclosing message / enum going out of scope).
///
/// `C` is the message common context (typically [`MessageCommon`](super::MessageCommon)).
/// Impls that need the allocator bound [`MessageCommonAlloc`](super::MessageCommonAlloc);
/// bit-tracked singular teardown also needs [`MessageCommonBits`](super::MessageCommonBits).
pub trait FieldDeallocate<C> {
    /// Frees heap payloads (if any) using `common`'s allocator.
    ///
    /// `common` must be this field's parent message common (see trait docs).
    fn deallocate(&mut self, common: &C);
}
