//! Sample of a message that **bakes** [`DiscardUnknowns`] into `Marker<A>`.
//!
//! Impls are `impl<A> Marker<A>` only — no `U:` bound on the message type.
//! Unknowns are consumed on decode and omitted on encode. Not part of the
//! DESIGN.md reference schema. This sample has no lazy alias.

use ::allocator_api2::alloc::{Allocator, Global};
use ::bitvec::array::BitArray;
use ::bitvec::order::Lsb0;
use ::core::mem;
use ::core::ops::{ControlFlow, DerefMut};
use ::puroro::{DecodeBuf, DecodeError, WireType};
use ::puroro_rt::decode::Eager;
use ::puroro_rt::{
    CloneRecordFields, ConstructRecordFields, DiscardUnknowns, EagerRecordFields, FieldCloneIn,
    FieldDeallocate, FieldPairVisitor, FieldVisitor, FieldVisitorMut, HasField, Implicit,
    MessageCommon, ProtoInt32, Record, RecordFields, SingularField,
};

use crate::marker::FIELD_N;

type Bits = BitArray<[u8; 1], Lsb0>;

pub(crate) struct MarkerFields<A: Allocator + Clone = Global> {
    n: SingularField<ProtoInt32, Implicit, { FIELD_N }, A>,
}

impl<A: Allocator + Clone> HasField<{ FIELD_N }> for MarkerFields<A> {
    type Ref<'a>
        = &'a SingularField<ProtoInt32, Implicit, { FIELD_N }, A>
    where
        Self: 'a;
    type Mut<'a>
        = &'a mut SingularField<ProtoInt32, Implicit, { FIELD_N }, A>
    where
        Self: 'a;

    fn field<'a>(&'a self) -> Self::Ref<'a> {
        &self.n
    }

    fn field_mut<'a>(&'a mut self) -> Self::Mut<'a> {
        &mut self.n
    }
}

pub struct Marker<A: Allocator + Clone = Global> {
    pub(crate) record: Record<MarkerFields<A>, A, Eager, DiscardUnknowns>,
}

::puroro_rt::forward_record!(eager Marker);

impl<A: Allocator + Clone> RecordFields<A, Eager> for MarkerFields<A> {
    type Bits = Bits;
    type Unknown = DiscardUnknowns;
    type EagerMessage = Marker<A>;

    const EAGER_NAME: &'static str = "Marker";
    const LAZY_NAME: &'static str = "Marker";

    fn empty_bits() -> Self::Bits {
        BitArray::ZERO
    }

    fn visit_mut<V: FieldVisitorMut<MessageCommon<Self::Bits, A, Self::Unknown, Eager>>>(
        &mut self,
        v: &mut V,
    ) -> ControlFlow<V::Break> {
        v.visit("n", &mut self.n)?;
        ControlFlow::Continue(())
    }
}

impl<A: Allocator + Clone> ConstructRecordFields<A, Eager> for MarkerFields<A> {
    fn new(alloc: A) -> Self {
        Self {
            n: SingularField::new_in(alloc),
        }
    }
}

impl<A: Allocator + Clone> CloneRecordFields<A, Eager> for MarkerFields<A> {
    fn clone_fields(
        &self,
        dst: &mut Self,
        src_common: &MessageCommon<Self::Bits, A, Self::Unknown, Eager>,
        dst_common: &MessageCommon<Self::Bits, A, Self::Unknown, Eager>,
        alloc: A,
    ) {
        let new = FieldCloneIn::clone_field(&self.n, src_common, alloc);
        let mut old = mem::replace(&mut dst.n, new);
        FieldDeallocate::deallocate(&mut old, dst_common);
    }
}

impl<A: Allocator + Clone> EagerRecordFields<A> for MarkerFields<A> {
    fn visit_fields<V: FieldVisitor<MessageCommon<Self::Bits, A, Self::Unknown, Eager>>>(
        &self,
        v: &mut V,
    ) -> ControlFlow<V::Break> {
        v.visit("n", &self.n)?;
        ControlFlow::Continue(())
    }

    fn visit_pairs<V: FieldPairVisitor<MessageCommon<Self::Bits, A, Self::Unknown, Eager>>>(
        &self,
        other: &Self,
        v: &mut V,
    ) -> ControlFlow<V::Break> {
        v.visit("n", &self.n, &other.n)?;
        ControlFlow::Continue(())
    }

    fn merge_known<B: DecodeBuf>(
        &mut self,
        common: &mut MessageCommon<Self::Bits, A, Self::Unknown, Eager>,
        field_number: u32,
        wire_type: WireType,
        buf: &mut B,
        depth: usize,
    ) -> Result<bool, DecodeError> {
        match field_number {
            FIELD_N => {
                <Self as HasField<{ FIELD_N }>>::field_mut(self)
                    .bind_mut(common)
                    .merge(wire_type, buf, depth)?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

impl<A: Allocator + Clone> Marker<A> {
    pub fn n(&self) -> i32 {
        self.record.field::<{ FIELD_N }>().value()
    }

    pub fn n_mut(&mut self) -> impl DerefMut<Target = i32> + '_ {
        self.record.field_mut::<{ FIELD_N }>().value_mut()
    }

    pub fn clear_n(&mut self) {
        self.record.field_mut::<{ FIELD_N }>().clear();
    }
}
