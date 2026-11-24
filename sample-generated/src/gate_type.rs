//! Sample of a message that **bakes** [`RejectUnknowns`] into `Gate<A>`.
//!
//! Impls are `impl<A> Gate<A>` only — no `U:` bound on the message type.
//! Unrecognized tags fail decode with [`DecodeError::UnrecognizedField`].
//! Not part of the DESIGN.md reference schema. This sample has no lazy alias.

use ::allocator_api2::alloc::{Allocator, Global};
use ::bitvec::array::BitArray;
use ::bitvec::order::Lsb0;
use ::core::marker::PhantomData;
use ::core::mem;
use ::core::ops::{ControlFlow, DerefMut};
use ::puroro::{DecodeBuf, DecodeError, WireType};
use ::puroro_rt::decode::Eager;
use ::puroro_rt::{
    CloneRecordFields, ConstructRecordFields, EagerRecordFields, FieldCloneIn, FieldDeallocate,
    FieldPairVisitor, FieldVisitor, FieldVisitorMut, HasField, Implicit, MessageCommon, ProtoInt32,
    Record, RecordFields, RejectUnknowns, SingularField,
};

use crate::gate::FIELD_N;

type Bits = BitArray<[u8; 1], Lsb0>;

/// Eager-only sample; `L` matches generated `*Fields<A, L>` shape.
pub(crate) struct GateFields<A: Allocator + Clone = Global, L = Eager> {
    n: SingularField<ProtoInt32, Implicit, { FIELD_N }, A>,
    _layout: PhantomData<L>,
}

impl<A: Allocator + Clone, L> HasField<{ FIELD_N }> for GateFields<A, L> {
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

pub struct Gate<A: Allocator + Clone = Global> {
    pub(crate) record: Record<GateFields<A, Eager>, A, Eager, RejectUnknowns>,
}

::puroro_rt::forward_record!(eager Gate, GateFields);

impl<A: Allocator + Clone> RecordFields<A, Eager> for GateFields<A, Eager> {
    type Bits = Bits;
    type Unknown = RejectUnknowns;
    type EagerMessage = Gate<A>;

    const EAGER_NAME: &'static str = "Gate";
    const LAZY_NAME: &'static str = "Gate";
    const FULL_NAME: &'static str = "example.Gate";

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

impl<A: Allocator + Clone> ConstructRecordFields<A, Eager> for GateFields<A, Eager> {
    fn new(alloc: A) -> Self {
        Self {
            n: SingularField::new_in(alloc),
            _layout: PhantomData,
        }
    }
}

impl<A: Allocator + Clone> CloneRecordFields<A, Eager> for GateFields<A, Eager> {
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

impl<A: Allocator + Clone> EagerRecordFields<A> for GateFields<A, Eager> {
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

impl<A: Allocator + Clone> Gate<A> {
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
