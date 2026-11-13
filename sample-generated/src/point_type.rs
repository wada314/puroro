//! Sample of a tiny nested message used to exercise **inlined** singular
//! storage (`Explicit` presence bit + [`Inline`] slot = `M`, not `Boxed`).
//!
//! Two IMPLICIT `int32` fields. Unknowns are preserved (default
//! [`MessageCommon`] store). Not part of the DESIGN.md reference schema.
//!
//! The shell matches plugin output: [`PointFields`] holds the slots,
//! [`Point`] / [`PointLazy`] are newtypes over [`puroro_rt::Record`].

use ::allocator_api2::alloc::{Allocator, Global};
use ::core::marker::PhantomData;
use ::core::mem;
use ::core::ops::{ControlFlow, DerefMut};
use ::puroro::{DecodeBuf, DecodeError, WireType};
use ::puroro_rt::decode::{MessageScan, ScannedRecord};
use ::puroro_rt::{
    CloneRecordFields, ConstructRecordFields, Eager, EagerRecordFields, FieldCloneIn,
    FieldDeallocate, FieldPairVisitor, FieldVisitor, FieldVisitorMut, HasField, Implicit, Lazy,
    LazyRecordApply, MessageCommon, ProtoInt32, Record, RecordFields, SingularField, UnknownFields,
};

use crate::point::{self, FIELD_X, FIELD_Y};

pub(crate) struct PointFields<A: Allocator + Clone = Global, L: point::Layout<A> = Eager> {
    x: SingularField<ProtoInt32, Implicit, { FIELD_X }, A>,
    y: SingularField<ProtoInt32, Implicit, { FIELD_Y }, A>,
    _layout: PhantomData<(A, L)>,
}

impl<A: Allocator + Clone, L: point::Layout<A>> HasField<{ FIELD_X }> for PointFields<A, L> {
    type Ref<'a>
        = &'a SingularField<ProtoInt32, Implicit, { FIELD_X }, A>
    where
        Self: 'a;
    type Mut<'a>
        = &'a mut SingularField<ProtoInt32, Implicit, { FIELD_X }, A>
    where
        Self: 'a;

    fn field<'a>(&'a self) -> Self::Ref<'a> {
        &self.x
    }

    fn field_mut<'a>(&'a mut self) -> Self::Mut<'a> {
        &mut self.x
    }
}

impl<A: Allocator + Clone, L: point::Layout<A>> HasField<{ FIELD_Y }> for PointFields<A, L> {
    type Ref<'a>
        = &'a SingularField<ProtoInt32, Implicit, { FIELD_Y }, A>
    where
        Self: 'a;
    type Mut<'a>
        = &'a mut SingularField<ProtoInt32, Implicit, { FIELD_Y }, A>
    where
        Self: 'a;

    fn field<'a>(&'a self) -> Self::Ref<'a> {
        &self.y
    }

    fn field_mut<'a>(&'a mut self) -> Self::Mut<'a> {
        &mut self.y
    }
}

pub struct Point<A: Allocator + Clone = Global> {
    pub(crate) record: Record<PointFields<A, Eager>, A, Eager, UnknownFields<A>>,
}

pub struct PointLazy<A: Allocator + Clone = Global> {
    pub(crate) record: Record<PointFields<A, Lazy<A>>, A, Lazy<A>, UnknownFields<A>>,
}

::puroro_rt::forward_record!(eager Point);
::puroro_rt::forward_record!(lazy PointLazy);

impl<A: Allocator + Clone, L: point::Layout<A> + MessageScan<A>> RecordFields<A, L>
    for PointFields<A, L>
{
    type Bits = <L as point::Layout<A>>::Bits;
    type Unknown = UnknownFields<A>;
    type EagerMessage = Point<A>;

    const EAGER_NAME: &'static str = "Point";
    const LAZY_NAME: &'static str = "PointLazy";

    fn empty_bits() -> Self::Bits {
        <L as point::Layout<A>>::empty_bits()
    }

    fn visit_mut<V: FieldVisitorMut<MessageCommon<Self::Bits, A, Self::Unknown, L>>>(
        &mut self,
        v: &mut V,
    ) -> ControlFlow<V::Break> {
        v.visit("x", &mut self.x)?;
        v.visit("y", &mut self.y)?;
        ControlFlow::Continue(())
    }
}

macro_rules! impl_point_build {
    ($layout:ty) => {
        impl<A: Allocator + Clone> ConstructRecordFields<A, $layout> for PointFields<A, $layout> {
            fn new(alloc: A) -> Self {
                Self {
                    x: SingularField::new_in(alloc.clone()),
                    y: SingularField::new_in(alloc),
                    _layout: PhantomData,
                }
            }
        }

        impl<A: Allocator + Clone> CloneRecordFields<A, $layout> for PointFields<A, $layout> {
            fn clone_fields(
                &self,
                dst: &mut Self,
                src_common: &MessageCommon<Self::Bits, A, Self::Unknown, $layout>,
                dst_common: &MessageCommon<Self::Bits, A, Self::Unknown, $layout>,
                alloc: A,
            ) {
                let new = FieldCloneIn::clone_field(&self.x, src_common, alloc.clone());
                let mut old = mem::replace(&mut dst.x, new);
                FieldDeallocate::deallocate(&mut old, dst_common);
                let new = FieldCloneIn::clone_field(&self.y, src_common, alloc.clone());
                let mut old = mem::replace(&mut dst.y, new);
                FieldDeallocate::deallocate(&mut old, dst_common);
            }
        }
    };
}

impl_point_build!(Eager);
impl_point_build!(Lazy<A>);

impl<A: Allocator + Clone> EagerRecordFields<A> for PointFields<A, Eager> {
    fn visit_fields<V: FieldVisitor<MessageCommon<Self::Bits, A, Self::Unknown, Eager>>>(
        &self,
        v: &mut V,
    ) -> ControlFlow<V::Break> {
        v.visit("x", &self.x)?;
        v.visit("y", &self.y)?;
        ControlFlow::Continue(())
    }

    fn visit_pairs<V: FieldPairVisitor<MessageCommon<Self::Bits, A, Self::Unknown, Eager>>>(
        &self,
        other: &Self,
        v: &mut V,
    ) -> ControlFlow<V::Break> {
        v.visit("x", &self.x, &other.x)?;
        v.visit("y", &self.y, &other.y)?;
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
            FIELD_X => {
                <Self as HasField<{ FIELD_X }>>::field_mut(self)
                    .bind_mut(common)
                    .merge(wire_type, buf, depth)?;
                Ok(true)
            }
            FIELD_Y => {
                <Self as HasField<{ FIELD_Y }>>::field_mut(self)
                    .bind_mut(common)
                    .merge(wire_type, buf, depth)?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

impl<A: Allocator + Clone> LazyRecordApply<A> for PointFields<A, Lazy<A>> {
    fn apply_record(
        &mut self,
        common: &mut MessageCommon<Self::Bits, A, Self::Unknown, Lazy<A>>,
        rec: &ScannedRecord,
        _origin: usize,
    ) -> Result<(), DecodeError> {
        match rec.field_number.as_u32() {
            FIELD_X => <Self as HasField<{ FIELD_X }>>::field_mut(self)
                .bind_mut(common)
                .merge_scanned(&rec.value),
            FIELD_Y => <Self as HasField<{ FIELD_Y }>>::field_mut(self)
                .bind_mut(common)
                .merge_scanned(&rec.value),
            _ => common.on_unrecognized_tag(rec.field_number.as_u32()),
        }
    }
}

impl<A: Allocator + Clone> Point<A> {
    pub fn x(&self) -> i32 {
        self.record.field::<{ FIELD_X }>().value()
    }

    pub fn y(&self) -> i32 {
        self.record.field::<{ FIELD_Y }>().value()
    }

    pub fn x_mut(&mut self) -> impl DerefMut<Target = i32> + '_ {
        self.record.field_mut::<{ FIELD_X }>().value_mut()
    }

    pub fn clear_x(&mut self) {
        self.record.field_mut::<{ FIELD_X }>().clear();
    }

    pub fn y_mut(&mut self) -> impl DerefMut<Target = i32> + '_ {
        self.record.field_mut::<{ FIELD_Y }>().value_mut()
    }

    pub fn clear_y(&mut self) {
        self.record.field_mut::<{ FIELD_Y }>().clear();
    }

    /// Encodes `self` and rebuilds it as the lazy alias.
    pub fn into_lazy(self) -> PointLazy<A> {
        PointLazy {
            record: Record::encode_lazy(self.record),
        }
    }
}
