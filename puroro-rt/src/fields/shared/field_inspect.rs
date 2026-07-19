//! Semantic [`PartialEq`] / [`Debug`] / encode / clone for catalog fields, plus
//! message-level field visitors so generated code enumerates fields once.

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
        common: &MessageCommon<Pb, A>,
        field: &F,
    ) -> ControlFlow<Self::Break>;
}

/// Pair / shared visitor — generated `visit_field_pairs` (e.g. [`PartialEq`]).
pub trait FieldPairVisitor<Pb: PresenceBits, A: Allocator> {
    type Break;

    fn visit<F: FieldPartialEq<Pb, A>>(
        &mut self,
        name: &'static str,
        common: &MessageCommon<Pb, A>,
        field: &F,
        other_common: &MessageCommon<Pb, A>,
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
        src_common: &MessageCommon<Pb, A>,
        src_field: &F,
        dst_common: &MessageCommon<Pb, A>,
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
        common: &MessageCommon<Pb, A>,
        field: &mut F,
    ) -> ControlFlow<Self::Break>;
}

// ---------------------------------------------------------------------------
// Ready-made visitors for generated PartialEq / Debug / Drop / encode / clone
// ---------------------------------------------------------------------------

/// [`FieldPairVisitor`] that breaks on the first unequal field.
#[derive(Clone, Copy, Debug, Default)]
pub struct FieldEqVisitor;

impl<Pb: PresenceBits, A: Allocator> FieldPairVisitor<Pb, A> for FieldEqVisitor {
    type Break = ();

    #[inline]
    fn visit<F: FieldPartialEq<Pb, A>>(
        &mut self,
        _name: &'static str,
        common: &MessageCommon<Pb, A>,
        field: &F,
        other_common: &MessageCommon<Pb, A>,
        other_field: &F,
    ) -> ControlFlow<()> {
        if field.field_eq(common, other_field, other_common) {
            ControlFlow::Continue(())
        } else {
            ControlFlow::Break(())
        }
    }
}

/// [`FieldVisitorMut`] that [`FieldDeallocate::deallocate`]s every field.
#[derive(Clone, Copy, Debug, Default)]
pub struct FieldDeallocVisitor;

impl<Pb: PresenceBits, A: Allocator> FieldVisitorMut<Pb, A> for FieldDeallocVisitor {
    type Break = ();

    #[inline]
    fn visit<F: FieldDeallocate<Pb, A>>(
        &mut self,
        _name: &'static str,
        common: &MessageCommon<Pb, A>,
        field: &mut F,
    ) -> ControlFlow<()> {
        field.deallocate(common);
        ControlFlow::Continue(())
    }
}

/// Sums [`FieldEncode::wire_encoded_len`].
#[derive(Clone, Copy, Debug, Default)]
pub struct EncodedLenVisitor {
    pub len: usize,
}

impl<Pb: PresenceBits, A: Allocator> FieldVisitor<Pb, A> for EncodedLenVisitor {
    type Break = ();

    #[inline]
    fn visit<F: CatalogField<Pb, A>>(
        &mut self,
        _name: &'static str,
        common: &MessageCommon<Pb, A>,
        field: &F,
    ) -> ControlFlow<()> {
        self.len += field.wire_encoded_len(common);
        ControlFlow::Continue(())
    }
}

/// Writes each field via [`FieldEncode::wire_encode_raw`].
pub struct EncodeRawVisitor<'a, B: BufMut> {
    pub buf: &'a mut B,
}

impl<'a, B: BufMut> EncodeRawVisitor<'a, B> {
    #[inline]
    pub fn new(buf: &'a mut B) -> Self {
        Self { buf }
    }
}

impl<'a, B: BufMut, Pb: PresenceBits, A: Allocator> FieldVisitor<Pb, A>
    for EncodeRawVisitor<'a, B>
{
    type Break = ();

    #[inline]
    fn visit<F: CatalogField<Pb, A>>(
        &mut self,
        _name: &'static str,
        common: &MessageCommon<Pb, A>,
        field: &F,
    ) -> ControlFlow<()> {
        field.wire_encode_raw(common, self.buf);
        ControlFlow::Continue(())
    }
}

/// Replaces each destination field with a clone of the source field.
///
/// Intended for `dst = Self::new_in(alloc)` followed by
/// `src.visit_field_pairs_mut(&mut dst, &mut CloneFieldsVisitor)` — empty
/// placeholders are deallocated as they are replaced.
#[derive(Clone, Copy, Debug, Default)]
pub struct CloneFieldsVisitor;

impl<Pb: PresenceBits, A: Allocator + Clone> FieldPairVisitorMut<Pb, A> for CloneFieldsVisitor {
    type Break = ();

    #[inline]
    fn visit<F>(
        &mut self,
        _name: &'static str,
        src_common: &MessageCommon<Pb, A>,
        src_field: &F,
        dst_common: &MessageCommon<Pb, A>,
        dst_field: &mut F,
    ) -> ControlFlow<()>
    where
        F: FieldCloneIn<Pb, A> + FieldDeallocate<Pb, A>,
    {
        let new = src_field.clone_field(src_common, dst_common.alloc.clone());
        let mut old = mem::replace(dst_field, new);
        old.deallocate(dst_common);
        ControlFlow::Continue(())
    }
}

/// Adapts [`fmt::DebugStruct`] as a [`FieldVisitor`].
///
/// Owns the [`DebugStruct`] so generated `Debug` can call [`finish`](Self::finish)
/// without overlapping borrows.
pub struct DebugStructVisitor<'a, 'b> {
    ds: fmt::DebugStruct<'a, 'b>,
}

impl<'a, 'b> DebugStructVisitor<'a, 'b> {
    #[inline]
    pub fn new(ds: fmt::DebugStruct<'a, 'b>) -> Self {
        Self { ds }
    }

    #[inline]
    pub fn finish(mut self) -> FmtResult {
        self.ds.finish()
    }
}

impl<'a, 'b, Pb: PresenceBits, A: Allocator> FieldVisitor<Pb, A> for DebugStructVisitor<'a, 'b> {
    type Break = ();

    #[inline]
    fn visit<F: CatalogField<Pb, A>>(
        &mut self,
        name: &'static str,
        common: &MessageCommon<Pb, A>,
        field: &F,
    ) -> ControlFlow<()> {
        // `field` must format through `FieldDebug` (not `fmt::Debug` on the wrapper).
        self.ds.field(name, &FieldDebugAdapter { field, common });
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
