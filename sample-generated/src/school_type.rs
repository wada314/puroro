//! Sample parent for two-hop inlined storage: [`School::student`] is a full
//! [`Student`](crate::Student) (`ProtoMessage` + `Inline`). Getters return
//! `Option<&Student>` / `&mut Student`.
//!
//! This sample has no lazy alias. Slots live on [`SchoolFields`]; [`School`]
//! is a newtype over [`puroro_rt::Record`].

use ::allocator_api2::alloc::{Allocator, Global};
use ::bitvec::array::BitArray;
use ::bitvec::order::Lsb0;
use ::core::mem;
use ::core::ops::ControlFlow;
use ::puroro::{DecodeBuf, DecodeError, HasDefault, Optional, WireType};
use ::puroro_rt::{
    CloneRecordFields, ConstructRecordFields, Eager, EagerRecordFields, Explicit, FieldCloneIn,
    FieldDeallocate, FieldPairVisitor, FieldVisitor, FieldVisitorMut, HasField, InlineOrHeap,
    MessageCommon, ProtoMessage, ProtoString, Record, RecordFields, SingularField, UnknownFields,
};

use crate::Student;
use crate::school::{BIT_NAME, BIT_NAME_SSO, BIT_STUDENT, FIELD_NAME, FIELD_STUDENT};

type Bits = BitArray<[u8; 1], Lsb0>;

pub(crate) struct SchoolFields<A: Allocator + Clone = Global> {
    name: SingularField<
        ProtoString,
        Explicit<{ BIT_NAME }>,
        { FIELD_NAME },
        A,
        InlineOrHeap<{ BIT_NAME_SSO }>,
    >,
    student:
        SingularField<ProtoMessage<Student<A>>, Explicit<{ BIT_STUDENT }>, { FIELD_STUDENT }, A>,
}

impl<A: Allocator + Clone> HasField<{ FIELD_NAME }> for SchoolFields<A> {
    type Ref<'a>
        = &'a SingularField<
        ProtoString,
        Explicit<{ BIT_NAME }>,
        { FIELD_NAME },
        A,
        InlineOrHeap<{ BIT_NAME_SSO }>,
    >
    where
        Self: 'a;
    type Mut<'a>
        = &'a mut SingularField<
        ProtoString,
        Explicit<{ BIT_NAME }>,
        { FIELD_NAME },
        A,
        InlineOrHeap<{ BIT_NAME_SSO }>,
    >
    where
        Self: 'a;

    fn field<'a>(&'a self) -> Self::Ref<'a> {
        &self.name
    }

    fn field_mut<'a>(&'a mut self) -> Self::Mut<'a> {
        &mut self.name
    }
}

impl<A: Allocator + Clone> HasField<{ FIELD_STUDENT }> for SchoolFields<A> {
    type Ref<'a>
        =
        &'a SingularField<ProtoMessage<Student<A>>, Explicit<{ BIT_STUDENT }>, { FIELD_STUDENT }, A>
    where
        Self: 'a;
    type Mut<'a>
        = &'a mut SingularField<
        ProtoMessage<Student<A>>,
        Explicit<{ BIT_STUDENT }>,
        { FIELD_STUDENT },
        A,
    >
    where
        Self: 'a;

    fn field<'a>(&'a self) -> Self::Ref<'a> {
        &self.student
    }

    fn field_mut<'a>(&'a mut self) -> Self::Mut<'a> {
        &mut self.student
    }
}

pub struct School<A: Allocator + Clone = Global> {
    pub(crate) record: Record<SchoolFields<A>, A, Eager, UnknownFields<A>>,
}

::puroro_rt::forward_record!(eager School);

impl<A: Allocator + Clone> RecordFields<A, Eager> for SchoolFields<A> {
    type Bits = Bits;
    type Unknown = UnknownFields<A>;
    type EagerMessage = School<A>;

    const EAGER_NAME: &'static str = "School";
    const LAZY_NAME: &'static str = "School";

    fn empty_bits() -> Self::Bits {
        BitArray::ZERO
    }

    fn visit_mut<V: FieldVisitorMut<MessageCommon<Self::Bits, A, Self::Unknown, Eager>>>(
        &mut self,
        v: &mut V,
    ) -> ControlFlow<V::Break> {
        v.visit("name", &mut self.name)?;
        v.visit("student", &mut self.student)?;
        ControlFlow::Continue(())
    }
}

impl<A: Allocator + Clone> ConstructRecordFields<A, Eager> for SchoolFields<A> {
    fn new(alloc: A) -> Self {
        Self {
            name: SingularField::new_in(alloc.clone()),
            student: SingularField::new_in(alloc),
        }
    }
}

impl<A: Allocator + Clone> CloneRecordFields<A, Eager> for SchoolFields<A> {
    fn clone_fields(
        &self,
        dst: &mut Self,
        src_common: &MessageCommon<Self::Bits, A, Self::Unknown, Eager>,
        dst_common: &MessageCommon<Self::Bits, A, Self::Unknown, Eager>,
        alloc: A,
    ) {
        let new = FieldCloneIn::clone_field(&self.name, src_common, alloc.clone());
        let mut old = mem::replace(&mut dst.name, new);
        FieldDeallocate::deallocate(&mut old, dst_common);
        let new = FieldCloneIn::clone_field(&self.student, src_common, alloc.clone());
        let mut old = mem::replace(&mut dst.student, new);
        FieldDeallocate::deallocate(&mut old, dst_common);
    }
}

impl<A: Allocator + Clone> EagerRecordFields<A> for SchoolFields<A> {
    fn visit_fields<V: FieldVisitor<MessageCommon<Self::Bits, A, Self::Unknown, Eager>>>(
        &self,
        v: &mut V,
    ) -> ControlFlow<V::Break> {
        v.visit("name", &self.name)?;
        v.visit("student", &self.student)?;
        ControlFlow::Continue(())
    }

    fn visit_pairs<V: FieldPairVisitor<MessageCommon<Self::Bits, A, Self::Unknown, Eager>>>(
        &self,
        other: &Self,
        v: &mut V,
    ) -> ControlFlow<V::Break> {
        v.visit("name", &self.name, &other.name)?;
        v.visit("student", &self.student, &other.student)?;
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
            FIELD_NAME => {
                <Self as HasField<{ FIELD_NAME }>>::field_mut(self)
                    .bind_mut(common)
                    .merge(wire_type, buf, depth)?;
                Ok(true)
            }
            FIELD_STUDENT => {
                <Self as HasField<{ FIELD_STUDENT }>>::field_mut(self)
                    .bind_mut(common)
                    .merge(wire_type, buf, depth)?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

impl<A: Allocator + Clone> School<A> {
    pub fn name<'a>(&'a self) -> Optional<&'a str, impl HasDefault<&'a str>>
    where
        A: 'a,
    {
        self.record.field::<{ FIELD_NAME }>().optional()
    }

    pub fn student(&self) -> Option<&Student<A>> {
        self.record.field::<{ FIELD_STUDENT }>().get()
    }

    pub fn name_mut<'s>(&'s mut self) -> impl ::puroro::StringMut<A> + 's {
        self.record.field_mut::<{ FIELD_NAME }>().value_mut()
    }

    pub fn clear_name(&mut self) {
        self.record.field_mut::<{ FIELD_NAME }>().clear();
    }

    pub fn student_mut(&mut self) -> &mut Student<A> {
        self.record.field_mut::<{ FIELD_STUDENT }>().get_mut()
    }

    pub fn set_student(&mut self, src: Student<A>) {
        *self.student_mut() = src;
    }

    pub fn clear_student(&mut self) {
        self.record.field_mut::<{ FIELD_STUDENT }>().clear();
    }
}
