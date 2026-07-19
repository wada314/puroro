//! Semantic [`PartialEq`] / [`Debug`] for catalog fields, plus message-level
//! field visitors so generated code enumerates fields once.

use ::allocator_api2::alloc::Allocator;
use ::core::fmt::{self, Debug, Formatter, Result as FmtResult};
use ::core::ops::ControlFlow;

use super::{MessageCommon, PresenceBits};

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

/// Shared borrow visitor — generated `visit_fields` calls this once per field.
///
/// `name` is the proto field / oneof group name (`"title"`, `"notification"`, …).
/// Call sites that ignore it (e.g. `Drop`) still pass it; LLVM typically drops
/// the unused `&'static str` after inlining.
pub trait FieldVisitor<Pb: PresenceBits, A: Allocator> {
    /// Stop early when `Break` is returned (unused by most visitors).
    type Break;

    fn visit<F: FieldDebug<Pb, A>>(
        &mut self,
        name: &'static str,
        common: &MessageCommon<Pb, A>,
        field: &F,
    ) -> ControlFlow<Self::Break>;
}

/// Paired-message visitor — generated `visit_fields_with` for [`PartialEq`].
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

/// Mutable visitor — generated `visit_fields_mut` for `Drop` / clear-all.
pub trait FieldVisitorMut<Pb: PresenceBits, A: Allocator> {
    type Break;

    fn visit_mut<F: super::FieldDeallocate<Pb, A>>(
        &mut self,
        name: &'static str,
        common: &MessageCommon<Pb, A>,
        field: &mut F,
    ) -> ControlFlow<Self::Break>;
}

// ---------------------------------------------------------------------------
// Ready-made visitors for generated `PartialEq` / `Debug` / `Drop`
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
    fn visit_mut<F: super::FieldDeallocate<Pb, A>>(
        &mut self,
        _name: &'static str,
        common: &MessageCommon<Pb, A>,
        field: &mut F,
    ) -> ControlFlow<()> {
        field.deallocate(common);
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
    fn visit<F: FieldDebug<Pb, A>>(
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
