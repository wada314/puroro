//! Semantic [`PartialEq`] / [`Debug`] / encode / clone for catalog fields, plus
//! message-level field visitors so generated code enumerates fields once.
//!
//! Visitors hold [`MessageCommon`] from construction time; generated
//! `visit_*` methods only pass field names and field slots. Shared walks use
//! ordinary references. Walks that also mutably borrow field slots of the same
//! message store `MessageCommon` as a raw pointer so the borrow checker does
//! not see an overlap — field payloads are disjoint from `MessageCommon`.

use ::allocator_api2::alloc::Allocator;
use ::bytes::BufMut;
use ::core::fmt::{self, Debug, Formatter, Result as FmtResult};
use ::core::mem;
use ::core::ops::ControlFlow;

use super::{FieldDeallocate, MessageCommon, PresenceBits};

/// Semantic equality for one catalog field (getter-level, not raw slots).
pub trait FieldPartialEq<Pb: PresenceBits, A: Allocator> {
    fn field_eq(
        &self,
        common: &MessageCommon<Pb, A>,
        other: &Self,
        other_common: &MessageCommon<Pb, A>,
    ) -> bool;
}

/// Semantic [`Debug`] view for one catalog field.
pub trait FieldDebug<Pb: PresenceBits, A: Allocator> {
    /// Formats like the public getter (presence-aware).
    fn fmt_debug(&self, common: &MessageCommon<Pb, A>, f: &mut Formatter<'_>) -> FmtResult;
}

/// Wire length / encode for one catalog field.
pub trait FieldEncode<Pb: PresenceBits, A: Allocator> {
    fn wire_encoded_len(&self, common: &MessageCommon<Pb, A>) -> usize;

    fn wire_encode_raw<B: BufMut>(&self, common: &MessageCommon<Pb, A>, buf: &mut B);
}

/// Deep-clone one catalog field through `common` / `alloc`.
pub trait FieldCloneIn<Pb: PresenceBits, A: Allocator>: Sized {
    fn clone_field(&self, common: &MessageCommon<Pb, A>, alloc: A) -> Self;
}

/// Capabilities required of every field passed to [`FieldVisitor`].
pub trait CatalogField<Pb: PresenceBits, A: Allocator>:
    FieldDebug<Pb, A> + FieldEncode<Pb, A>
{
}

impl<T, Pb, A> CatalogField<Pb, A> for T
where
    T: FieldDebug<Pb, A> + FieldEncode<Pb, A>,
    Pb: PresenceBits,
    A: Allocator,
{
}

/// Scalar / shared visitor — generated `visit_fields` walks one message immutably.
///
/// `name` is the proto field / oneof group name (`"title"`, `"notification"`, …).
/// Call sites that ignore it (e.g. encode length) still pass it; LLVM typically
/// drops the unused `&'static str` after inlining.
pub trait FieldVisitor<Pb: PresenceBits, A: Allocator> {
    /// Stop early when `Break` is returned (unused by most visitors).
    type Break;

    fn visit<F: CatalogField<Pb, A>>(
        &mut self,
        name: &'static str,
        field: &F,
    ) -> ControlFlow<Self::Break>;
}

/// Pair / shared visitor — generated `visit_field_pairs` (e.g. [`PartialEq`]).
///
/// Not part of the public message API. The [`FieldPartialEq`] bound on `F` is
/// intentional: catalog field types are heterogeneous, so a generic walk needs
/// the capability on the field argument (same pattern as
/// [`FieldPairVisitorMut`] / [`FieldVisitor`]).
pub trait FieldPairVisitor<Pb: PresenceBits, A: Allocator> {
    type Break;

    fn visit<F: FieldPartialEq<Pb, A>>(
        &mut self,
        name: &'static str,
        field: &F,
        other_field: &F,
    ) -> ControlFlow<Self::Break>;
}

/// Pair / mutable visitor — generated `visit_field_pairs_mut` (e.g. [`CloneIn`](unmanaged::CloneIn)).
///
/// The destination field is `&mut`; source stays shared.
pub trait FieldPairVisitorMut<Pb: PresenceBits, A: Allocator> {
    type Break;

    fn visit<F>(
        &mut self,
        name: &'static str,
        src_field: &F,
        dst_field: &mut F,
    ) -> ControlFlow<Self::Break>
    where
        F: FieldCloneIn<Pb, A> + FieldDeallocate<Pb, A>;
}

/// Scalar / mutable visitor — generated `visit_fields_mut` (e.g. `Drop` / clear-all).
pub trait FieldVisitorMut<Pb: PresenceBits, A: Allocator> {
    type Break;

    fn visit<F: FieldDeallocate<Pb, A>>(
        &mut self,
        name: &'static str,
        field: &mut F,
    ) -> ControlFlow<Self::Break>;
}

// ---------------------------------------------------------------------------
// Ready-made visitors for generated PartialEq / Debug / Drop / encode / clone
// ---------------------------------------------------------------------------

/// [`FieldPairVisitor`] that breaks on the first unequal field.
pub struct FieldEqVisitor<'a, Pb: PresenceBits, A: Allocator> {
    common: &'a MessageCommon<Pb, A>,
    other_common: &'a MessageCommon<Pb, A>,
}

impl<'a, Pb: PresenceBits, A: Allocator> FieldEqVisitor<'a, Pb, A> {
    #[inline]
    pub fn new(common: &'a MessageCommon<Pb, A>, other_common: &'a MessageCommon<Pb, A>) -> Self {
        Self {
            common,
            other_common,
        }
    }
}

impl<'a, Pb: PresenceBits, A: Allocator> FieldPairVisitor<Pb, A> for FieldEqVisitor<'a, Pb, A> {
    type Break = ();

    #[inline]
    fn visit<F: FieldPartialEq<Pb, A>>(
        &mut self,
        _name: &'static str,
        field: &F,
        other_field: &F,
    ) -> ControlFlow<()> {
        if field.field_eq(self.common, other_field, self.other_common) {
            ControlFlow::Continue(())
        } else {
            ControlFlow::Break(())
        }
    }
}

/// [`FieldVisitorMut`] that [`FieldDeallocate::deallocate`]s every field.
///
/// Stores [`MessageCommon`] as a raw pointer so it can be built from
/// `&message._common` before [`visit_fields_mut`] mutably borrows field slots.
pub struct FieldDeallocVisitor<Pb: PresenceBits, A: Allocator> {
    common: *const MessageCommon<Pb, A>,
}

impl<Pb: PresenceBits, A: Allocator> FieldDeallocVisitor<Pb, A> {
    /// Captures `common` for a subsequent mutable field walk.
    ///
    /// The pointed-to value must outlive the walk and must not be mutated for
    /// that duration. Mutating disjoint catalog field slots on the same message
    /// is intended and sound.
    #[inline]
    pub fn new(common: &MessageCommon<Pb, A>) -> Self {
        Self {
            common: common as *const MessageCommon<Pb, A>,
        }
    }

    #[inline]
    fn common(&self) -> &MessageCommon<Pb, A> {
        // SAFETY: `new` requires `common` stay valid and immutable for the walk;
        // only disjoint field slots are mutated meanwhile.
        unsafe { &*self.common }
    }
}

impl<Pb: PresenceBits, A: Allocator> FieldVisitorMut<Pb, A> for FieldDeallocVisitor<Pb, A> {
    type Break = ();

    #[inline]
    fn visit<F: FieldDeallocate<Pb, A>>(
        &mut self,
        _name: &'static str,
        field: &mut F,
    ) -> ControlFlow<()> {
        field.deallocate(self.common());
        ControlFlow::Continue(())
    }
}

/// Sums [`FieldEncode::wire_encoded_len`].
pub struct EncodedLenVisitor<'a, Pb: PresenceBits, A: Allocator> {
    common: &'a MessageCommon<Pb, A>,
    pub len: usize,
}

impl<'a, Pb: PresenceBits, A: Allocator> EncodedLenVisitor<'a, Pb, A> {
    #[inline]
    pub fn new(common: &'a MessageCommon<Pb, A>) -> Self {
        Self { common, len: 0 }
    }
}

impl<'a, Pb: PresenceBits, A: Allocator> FieldVisitor<Pb, A> for EncodedLenVisitor<'a, Pb, A> {
    type Break = ();

    #[inline]
    fn visit<F: CatalogField<Pb, A>>(&mut self, _name: &'static str, field: &F) -> ControlFlow<()> {
        self.len += field.wire_encoded_len(self.common);
        ControlFlow::Continue(())
    }
}

/// Writes each field via [`FieldEncode::wire_encode_raw`].
pub struct EncodeRawVisitor<'a, B: BufMut, Pb: PresenceBits, A: Allocator> {
    common: &'a MessageCommon<Pb, A>,
    pub buf: &'a mut B,
}

impl<'a, B: BufMut, Pb: PresenceBits, A: Allocator> EncodeRawVisitor<'a, B, Pb, A> {
    #[inline]
    pub fn new(common: &'a MessageCommon<Pb, A>, buf: &'a mut B) -> Self {
        Self { common, buf }
    }
}

impl<'a, B: BufMut, Pb: PresenceBits, A: Allocator> FieldVisitor<Pb, A>
    for EncodeRawVisitor<'a, B, Pb, A>
{
    type Break = ();

    #[inline]
    fn visit<F: CatalogField<Pb, A>>(&mut self, _name: &'static str, field: &F) -> ControlFlow<()> {
        field.wire_encode_raw(self.common, self.buf);
        ControlFlow::Continue(())
    }
}

/// Replaces each destination field with a clone of the source field.
///
/// Intended for `dst = Self::new_in(alloc)` followed by
/// `src.visit_field_pairs_mut(&mut dst, &mut CloneFieldsVisitor::new(...))` —
/// empty placeholders are deallocated as they are replaced.
///
/// Destination `MessageCommon` is stored as a raw pointer so the visitor can be
/// built before mutably borrowing `dst`'s field slots.
pub struct CloneFieldsVisitor<Pb: PresenceBits, A: Allocator> {
    src_common: *const MessageCommon<Pb, A>,
    dst_common: *const MessageCommon<Pb, A>,
}

impl<Pb: PresenceBits, A: Allocator> CloneFieldsVisitor<Pb, A> {
    /// Captures source / destination commons for a clone-into walk.
    ///
    /// Both pointees must outlive the walk. Destination `MessageCommon` must
    /// not be mutated during the walk (field slots may be). Presence on `dst`
    /// should still be empty when used after [`Self::new_in`].
    #[inline]
    pub fn new(src_common: &MessageCommon<Pb, A>, dst_common: &MessageCommon<Pb, A>) -> Self {
        Self {
            src_common: src_common as *const MessageCommon<Pb, A>,
            dst_common: dst_common as *const MessageCommon<Pb, A>,
        }
    }

    #[inline]
    fn src_common(&self) -> &MessageCommon<Pb, A> {
        // SAFETY: see [`Self::new`].
        unsafe { &*self.src_common }
    }

    #[inline]
    fn dst_common(&self) -> &MessageCommon<Pb, A> {
        // SAFETY: see [`Self::new`].
        unsafe { &*self.dst_common }
    }
}

impl<Pb: PresenceBits, A: Allocator + Clone> FieldPairVisitorMut<Pb, A>
    for CloneFieldsVisitor<Pb, A>
{
    type Break = ();

    #[inline]
    fn visit<F>(&mut self, _name: &'static str, src_field: &F, dst_field: &mut F) -> ControlFlow<()>
    where
        F: FieldCloneIn<Pb, A> + FieldDeallocate<Pb, A>,
    {
        let dst_common = self.dst_common();
        let new = src_field.clone_field(self.src_common(), dst_common.alloc.clone());
        let mut old = mem::replace(dst_field, new);
        old.deallocate(dst_common);
        ControlFlow::Continue(())
    }
}

/// Adapts [`fmt::DebugStruct`] as a [`FieldVisitor`].
///
/// Owns the [`DebugStruct`] so generated `Debug` can call [`finish`](Self::finish)
/// without overlapping borrows.
pub struct DebugStructVisitor<'a, 'b, 'c, Pb: PresenceBits, A: Allocator> {
    ds: fmt::DebugStruct<'a, 'b>,
    common: &'c MessageCommon<Pb, A>,
}

impl<'a, 'b, 'c, Pb: PresenceBits, A: Allocator> DebugStructVisitor<'a, 'b, 'c, Pb, A> {
    #[inline]
    pub fn new(ds: fmt::DebugStruct<'a, 'b>, common: &'c MessageCommon<Pb, A>) -> Self {
        Self { ds, common }
    }

    #[inline]
    pub fn finish(mut self) -> FmtResult {
        self.ds.finish()
    }
}

impl<'a, 'b, 'c, Pb: PresenceBits, A: Allocator> FieldVisitor<Pb, A>
    for DebugStructVisitor<'a, 'b, 'c, Pb, A>
{
    type Break = ();

    #[inline]
    fn visit<F: CatalogField<Pb, A>>(&mut self, name: &'static str, field: &F) -> ControlFlow<()> {
        // `field` must format through `FieldDebug` (not `fmt::Debug` on the wrapper).
        self.ds.field(
            name,
            &FieldDebugAdapter {
                field,
                common: self.common,
            },
        );
        ControlFlow::Continue(())
    }
}

struct FieldDebugAdapter<'a, F, Pb: PresenceBits, A: Allocator> {
    field: &'a F,
    common: &'a MessageCommon<Pb, A>,
}

impl<'a, F, Pb, A> Debug for FieldDebugAdapter<'a, F, Pb, A>
where
    F: FieldDebug<Pb, A>,
    Pb: PresenceBits,
    A: Allocator,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.field.fmt_debug(self.common, f)
    }
}
