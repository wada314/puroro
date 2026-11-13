//! Sample mid-level nested message: an EXPLICIT scalar plus inlined
//! [`Point`](crate::Point) and [`Address`](crate::Address) (each a full `M`).
//!
//! Used by [`School`](crate::School) as a two-hop inline chain. Not in DESIGN.md.
//! This sample has no lazy alias. Slots live on [`StudentFields`]; [`Student`]
//! is a newtype over [`puroro_rt::Record`].

use ::allocator_api2::alloc::{Allocator, Global};
use ::bitvec::array::BitArray;
use ::bitvec::order::Lsb0;
use ::core::mem;
use ::core::ops::{ControlFlow, DerefMut};
use ::puroro::{DecodeBuf, DecodeError, HasDefault, Optional, WireType};
use ::puroro_rt::{
    CloneRecordFields, ConstructRecordFields, Eager, EagerRecordFields, Explicit, FieldCloneIn,
    FieldDeallocate, FieldPairVisitor, FieldVisitor, FieldVisitorMut, HasField, MessageCommon,
    ProtoInt32, ProtoMessage, Record, RecordFields, SingularField, UnknownFields,
};

use crate::Address;
use crate::Point;
use crate::student::{BIT_HOME, BIT_LOCATION, BIT_YEAR, FIELD_HOME, FIELD_LOCATION, FIELD_YEAR};

type Bits = BitArray<[u8; 1], Lsb0>;

pub(crate) struct StudentFields<A: Allocator + Clone = Global> {
    year: SingularField<ProtoInt32, Explicit<{ BIT_YEAR }>, { FIELD_YEAR }, A>,
    location:
        SingularField<ProtoMessage<Point<A>>, Explicit<{ BIT_LOCATION }>, { FIELD_LOCATION }, A>,
    home: SingularField<ProtoMessage<Address<A>>, Explicit<{ BIT_HOME }>, { FIELD_HOME }, A>,
}

macro_rules! has_student_field {
    ($n:expr, $slot:ident, $ty:ty) => {
        impl<A: Allocator + Clone> HasField<{ $n }> for StudentFields<A> {
            type Ref<'a>
                = &'a $ty
            where
                Self: 'a;
            type Mut<'a>
                = &'a mut $ty
            where
                Self: 'a;

            fn field<'a>(&'a self) -> Self::Ref<'a> {
                &self.$slot
            }

            fn field_mut<'a>(&'a mut self) -> Self::Mut<'a> {
                &mut self.$slot
            }
        }
    };
}

has_student_field!(
    FIELD_YEAR,
    year,
    SingularField<ProtoInt32, Explicit<{ BIT_YEAR }>, { FIELD_YEAR }, A>
);
has_student_field!(
    FIELD_LOCATION,
    location,
    SingularField<ProtoMessage<Point<A>>, Explicit<{ BIT_LOCATION }>, { FIELD_LOCATION }, A>
);
has_student_field!(
    FIELD_HOME,
    home,
    SingularField<ProtoMessage<Address<A>>, Explicit<{ BIT_HOME }>, { FIELD_HOME }, A>
);

pub struct Student<A: Allocator + Clone = Global> {
    pub(crate) record: Record<StudentFields<A>, A, Eager, UnknownFields<A>>,
}

::puroro_rt::forward_record!(eager Student);

impl<A: Allocator + Clone> RecordFields<A, Eager> for StudentFields<A> {
    type Bits = Bits;
    type Unknown = UnknownFields<A>;
    type EagerMessage = Student<A>;

    const EAGER_NAME: &'static str = "Student";
    const LAZY_NAME: &'static str = "Student";

    fn empty_bits() -> Self::Bits {
        BitArray::ZERO
    }

    fn visit_mut<V: FieldVisitorMut<MessageCommon<Self::Bits, A, Self::Unknown, Eager>>>(
        &mut self,
        v: &mut V,
    ) -> ControlFlow<V::Break> {
        v.visit("year", &mut self.year)?;
        v.visit("location", &mut self.location)?;
        v.visit("home", &mut self.home)?;
        ControlFlow::Continue(())
    }
}

impl<A: Allocator + Clone> ConstructRecordFields<A, Eager> for StudentFields<A> {
    fn new(alloc: A) -> Self {
        Self {
            year: SingularField::new_in(alloc.clone()),
            location: SingularField::new_in(alloc.clone()),
            home: SingularField::new_in(alloc),
        }
    }
}

macro_rules! clone_student {
    ($self:ident, $dst:ident, $src:ident, $dst_common:ident, $alloc:ident, $name:ident) => {{
        let new = FieldCloneIn::clone_field(&$self.$name, $src, $alloc.clone());
        let mut old = mem::replace(&mut $dst.$name, new);
        FieldDeallocate::deallocate(&mut old, $dst_common);
    }};
}

impl<A: Allocator + Clone> CloneRecordFields<A, Eager> for StudentFields<A> {
    fn clone_fields(
        &self,
        dst: &mut Self,
        src_common: &MessageCommon<Self::Bits, A, Self::Unknown, Eager>,
        dst_common: &MessageCommon<Self::Bits, A, Self::Unknown, Eager>,
        alloc: A,
    ) {
        clone_student!(self, dst, src_common, dst_common, alloc, year);
        clone_student!(self, dst, src_common, dst_common, alloc, location);
        clone_student!(self, dst, src_common, dst_common, alloc, home);
    }
}

impl<A: Allocator + Clone> EagerRecordFields<A> for StudentFields<A> {
    fn visit_fields<V: FieldVisitor<MessageCommon<Self::Bits, A, Self::Unknown, Eager>>>(
        &self,
        v: &mut V,
    ) -> ControlFlow<V::Break> {
        v.visit("year", &self.year)?;
        v.visit("location", &self.location)?;
        v.visit("home", &self.home)?;
        ControlFlow::Continue(())
    }

    fn visit_pairs<V: FieldPairVisitor<MessageCommon<Self::Bits, A, Self::Unknown, Eager>>>(
        &self,
        other: &Self,
        v: &mut V,
    ) -> ControlFlow<V::Break> {
        v.visit("year", &self.year, &other.year)?;
        v.visit("location", &self.location, &other.location)?;
        v.visit("home", &self.home, &other.home)?;
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
            FIELD_YEAR => {
                <Self as HasField<{ FIELD_YEAR }>>::field_mut(self)
                    .bind_mut(common)
                    .merge(wire_type, buf, depth)?;
                Ok(true)
            }
            FIELD_LOCATION => {
                <Self as HasField<{ FIELD_LOCATION }>>::field_mut(self)
                    .bind_mut(common)
                    .merge(wire_type, buf, depth)?;
                Ok(true)
            }
            FIELD_HOME => {
                <Self as HasField<{ FIELD_HOME }>>::field_mut(self)
                    .bind_mut(common)
                    .merge(wire_type, buf, depth)?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

impl<A: Allocator + Clone> Student<A> {
    pub fn year<'a>(&'a self) -> Optional<i32, impl HasDefault<i32>>
    where
        A: 'a,
    {
        self.record.field::<{ FIELD_YEAR }>().optional()
    }

    pub fn location(&self) -> Option<&Point<A>> {
        self.record.field::<{ FIELD_LOCATION }>().get()
    }

    pub fn home(&self) -> Option<&Address<A>> {
        self.record.field::<{ FIELD_HOME }>().get()
    }

    pub fn year_mut<'s>(&'s mut self) -> impl DerefMut<Target = i32> + 's {
        self.record.field_mut::<{ FIELD_YEAR }>().value_mut()
    }

    pub fn clear_year(&mut self) {
        self.record.field_mut::<{ FIELD_YEAR }>().clear();
    }

    pub fn location_mut(&mut self) -> &mut Point<A> {
        self.record.field_mut::<{ FIELD_LOCATION }>().get_mut()
    }

    pub fn set_location(&mut self, src: Point<A>) {
        *self.location_mut() = src;
    }

    pub fn clear_location(&mut self) {
        self.record.field_mut::<{ FIELD_LOCATION }>().clear();
    }

    pub fn home_mut(&mut self) -> &mut Address<A> {
        self.record.field_mut::<{ FIELD_HOME }>().get_mut()
    }

    pub fn set_home(&mut self, src: Address<A>) {
        *self.home_mut() = src;
    }

    pub fn clear_home(&mut self) {
        self.record.field_mut::<{ FIELD_HOME }>().clear();
    }
}
