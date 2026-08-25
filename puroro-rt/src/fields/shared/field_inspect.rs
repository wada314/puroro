//! Semantic [`PartialEq`] / [`Debug`] / encode / clone for catalog fields, plus
//! message-level field visitors so generated code enumerates fields once.
//!
//! Visitors hold the message common context from construction time; generated
//! `visit_*` methods only pass field names and field slots. Shared walks use
//! ordinary references. Walks that also mutably borrow field slots of the same
//! message store the common as a raw pointer so the borrow checker does not see
//! an overlap — field payloads are disjoint from the common context.
//!
//! `C` is the common context type (typically
//! [`MessageCommon`](super::MessageCommon)). Impls add
//! [`MessageCommonBits`](super::MessageCommonBits) /
//! [`MessageCommonAlloc`](super::MessageCommonAlloc) as needed.

use crate::message_encode::EncodeCtx;
use ::bytes::BufMut;
use ::core::fmt::{self, Debug, Formatter, Result as FmtResult};
use ::core::mem;
use ::core::ops::ControlFlow;

use super::FieldDeallocate;
use super::MessageCommonAlloc;

/// Semantic equality for one catalog field (getter-level, not raw slots).
pub trait FieldPartialEq<C> {
    fn field_eq(&self, common: &C, other: &Self, other_common: &C) -> bool;
}

/// Semantic [`Debug`] view for one catalog field.
pub trait FieldDebug<C> {
    /// Formats like the public getter (presence-aware).
    fn fmt_debug(&self, common: &C, f: &mut Formatter<'_>) -> FmtResult;
}

/// Wire length / encode for one catalog field.
pub trait FieldEncode<C> {
    fn encoded_len(&self, common: &C, ctx: &mut EncodeCtx) -> usize;

    fn encode_raw<B: BufMut>(&self, common: &C, ctx: &mut EncodeCtx, buf: &mut B);
}

/// Deep-clone one catalog field through `common` / allocator.
pub trait FieldCloneIn<C: MessageCommonAlloc>: Sized {
    fn clone_field(&self, common: &C, alloc: C::Alloc) -> Self;
}

/// Capabilities required of every field passed to [`FieldVisitor`].
pub trait CatalogField<C>: FieldDebug<C> + FieldEncode<C> {}

impl<T, C> CatalogField<C> for T where T: FieldDebug<C> + FieldEncode<C> {}

/// Scalar / shared visitor — generated `visit_fields` walks one message immutably.
///
/// `name` is the proto field / oneof group name (`"title"`, `"notification"`, …).
/// Call sites that ignore it (e.g. encode length) still pass it; LLVM typically
/// drops the unused `&'static str` after inlining.
pub trait FieldVisitor<C> {
    /// Stop early when `Break` is returned (unused by most visitors).
    type Break;

    fn visit<F: CatalogField<C>>(
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
pub trait FieldPairVisitor<C> {
    type Break;

    fn visit<F: FieldPartialEq<C>>(
        &mut self,
        name: &'static str,
        field: &F,
        other_field: &F,
    ) -> ControlFlow<Self::Break>;
}

/// Pair / mutable visitor — generated `visit_field_pairs_mut` (e.g. [`CloneIn`](unmanaged::CloneIn)).
///
/// The destination field is `&mut`; source stays shared.
pub trait FieldPairVisitorMut<C> {
    type Break;

    fn visit<F>(
        &mut self,
        name: &'static str,
        src_field: &F,
        dst_field: &mut F,
    ) -> ControlFlow<Self::Break>
    where
        C: MessageCommonAlloc,
        F: FieldCloneIn<C> + FieldDeallocate<C>;
}

/// Scalar / mutable visitor — generated `visit_fields_mut` (e.g. `Drop` / clear-all).
pub trait FieldVisitorMut<C> {
    type Break;

    fn visit<F: FieldDeallocate<C>>(
        &mut self,
        name: &'static str,
        field: &mut F,
    ) -> ControlFlow<Self::Break>;
}

// ---------------------------------------------------------------------------
// Ready-made visitors for generated PartialEq / Debug / Drop / encode / clone
// ---------------------------------------------------------------------------

/// [`FieldPairVisitor`] that breaks on the first unequal field.
pub struct FieldEqVisitor<'a, C> {
    common: &'a C,
    other_common: &'a C,
}

impl<'a, C> FieldEqVisitor<'a, C> {
    #[inline]
    pub fn new(common: &'a C, other_common: &'a C) -> Self {
        Self {
            common,
            other_common,
        }
    }
}

impl<'a, C> FieldPairVisitor<C> for FieldEqVisitor<'a, C> {
    type Break = ();

    #[inline]
    fn visit<F: FieldPartialEq<C>>(
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
/// Stores the common context as a raw pointer so it can be built from
/// `&message._common` before [`visit_fields_mut`] mutably borrows field slots.
pub struct FieldDeallocVisitor<C> {
    common: *const C,
}

impl<C> FieldDeallocVisitor<C> {
    /// Captures `common` for a subsequent mutable field walk.
    ///
    /// The pointed-to value must outlive the walk and must not be mutated for
    /// that duration. Mutating disjoint catalog field slots on the same message
    /// is intended and sound.
    #[inline]
    pub fn new(common: &C) -> Self {
        Self {
            common: common as *const C,
        }
    }

    #[inline]
    fn common(&self) -> &C {
        // SAFETY: `new` requires `common` stay valid and immutable for the walk;
        // only disjoint field slots are mutated meanwhile.
        unsafe { &*self.common }
    }
}

impl<C> FieldVisitorMut<C> for FieldDeallocVisitor<C> {
    type Break = ();

    #[inline]
    fn visit<F: FieldDeallocate<C>>(
        &mut self,
        _name: &'static str,
        field: &mut F,
    ) -> ControlFlow<()> {
        field.deallocate(self.common());
        ControlFlow::Continue(())
    }
}

/// Sums [`FieldEncode::encoded_len`].
pub struct EncodedLenVisitor<'a, C> {
    common: &'a C,
    ctx: &'a mut EncodeCtx,
    pub len: usize,
}

impl<'a, C> EncodedLenVisitor<'a, C> {
    #[inline]
    pub fn new(common: &'a C, ctx: &'a mut EncodeCtx) -> Self {
        Self {
            common,
            ctx,
            len: 0,
        }
    }
}

impl<'a, C> FieldVisitor<C> for EncodedLenVisitor<'a, C> {
    type Break = ();

    #[inline]
    fn visit<F: CatalogField<C>>(&mut self, _name: &'static str, field: &F) -> ControlFlow<()> {
        self.len += field.encoded_len(self.common, self.ctx);
        ControlFlow::Continue(())
    }
}

/// Writes each field via [`FieldEncode::encode_raw`].
pub struct EncodeRawVisitor<'a, B: BufMut, C> {
    common: &'a C,
    ctx: &'a mut EncodeCtx,
    pub buf: &'a mut B,
}

impl<'a, B: BufMut, C> EncodeRawVisitor<'a, B, C> {
    #[inline]
    pub fn new(common: &'a C, ctx: &'a mut EncodeCtx, buf: &'a mut B) -> Self {
        Self { common, ctx, buf }
    }
}

impl<'a, B: BufMut, C> FieldVisitor<C> for EncodeRawVisitor<'a, B, C> {
    type Break = ();

    #[inline]
    fn visit<F: CatalogField<C>>(&mut self, _name: &'static str, field: &F) -> ControlFlow<()> {
        field.encode_raw(self.common, self.ctx, self.buf);
        ControlFlow::Continue(())
    }
}

/// Replaces each destination field with a clone of the source field.
///
/// Intended for `dst = Self::new_in(alloc)` followed by
/// `src.visit_field_pairs_mut(&mut dst, &mut CloneFieldsVisitor::new(...))` —
/// empty placeholders are deallocated as they are replaced.
///
/// Destination common is stored as a raw pointer so the visitor can be built
/// before mutably borrowing `dst`'s field slots.
pub struct CloneFieldsVisitor<C> {
    src_common: *const C,
    dst_common: *const C,
}

impl<C> CloneFieldsVisitor<C> {
    /// Captures source / destination commons for a clone-into walk.
    ///
    /// Both pointees must outlive the walk. Destination common must not be
    /// mutated during the walk (field slots may be). Presence on `dst` should
    /// still be empty when used after [`MessageCommon::new_in`](super::MessageCommon::new_in).
    #[inline]
    pub fn new(src_common: &C, dst_common: &C) -> Self {
        Self {
            src_common: src_common as *const C,
            dst_common: dst_common as *const C,
        }
    }

    #[inline]
    fn src_common(&self) -> &C {
        // SAFETY: see [`Self::new`].
        unsafe { &*self.src_common }
    }

    #[inline]
    fn dst_common(&self) -> &C {
        // SAFETY: see [`Self::new`].
        unsafe { &*self.dst_common }
    }
}

impl<C: MessageCommonAlloc> FieldPairVisitorMut<C> for CloneFieldsVisitor<C>
where
    C::Alloc: Clone,
{
    type Break = ();

    #[inline]
    fn visit<F>(&mut self, _name: &'static str, src_field: &F, dst_field: &mut F) -> ControlFlow<()>
    where
        F: FieldCloneIn<C> + FieldDeallocate<C>,
    {
        let dst_common = self.dst_common();
        let new = src_field.clone_field(self.src_common(), dst_common.clone_alloc());
        let mut old = mem::replace(dst_field, new);
        old.deallocate(dst_common);
        ControlFlow::Continue(())
    }
}

/// Adapts [`fmt::DebugStruct`] as a [`FieldVisitor`].
///
/// Owns the [`DebugStruct`] so generated `Debug` can call [`finish`](Self::finish)
/// without overlapping borrows.
pub struct DebugStructVisitor<'a, 'b, 'c, C> {
    ds: fmt::DebugStruct<'a, 'b>,
    common: &'c C,
}

impl<'a, 'b, 'c, C> DebugStructVisitor<'a, 'b, 'c, C> {
    #[inline]
    pub fn new(ds: fmt::DebugStruct<'a, 'b>, common: &'c C) -> Self {
        Self { ds, common }
    }

    #[inline]
    pub fn finish(mut self) -> FmtResult {
        self.ds.finish()
    }
}

impl<'a, 'b, 'c, C> FieldVisitor<C> for DebugStructVisitor<'a, 'b, 'c, C> {
    type Break = ();

    #[inline]
    fn visit<F: CatalogField<C>>(&mut self, name: &'static str, field: &F) -> ControlFlow<()> {
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

struct FieldDebugAdapter<'a, F, C> {
    field: &'a F,
    common: &'a C,
}

impl<'a, F, C> Debug for FieldDebugAdapter<'a, F, C>
where
    F: FieldDebug<C>,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.field.fmt_debug(self.common, f)
    }
}
