//! Sample parent for two-hop inlined storage: [`School.student`] is a full
//! [`Student`](crate::Student) (`ProtoMessage` + `Inline`). Getters return
//! `Option<&Student>` / `&mut Student`.

use allocator_api2::alloc::{Allocator, Global};
use bitvec::array::BitArray;
use bitvec::order::Lsb0;
use bytes::{Buf, BufMut};
use core::fmt;
use core::mem;
use core::ops::ControlFlow;

use puroro::{DecodeBuf, DecodeError, HasDefault, Message, Optional};
use puroro_rt::decode::{decode_tag, skip_field_and_save};
use puroro_rt::{
    CloneFieldsVisitor, CloneIn, DebugStructVisitor, EncodeCtx, EncodeRawVisitor,
    EncodedLenVisitor, Explicit, FieldDeallocVisitor, FieldEqVisitor, FieldPairVisitor,
    FieldPairVisitorMut, FieldVisitor, FieldVisitorMut, InlineOrHeap, MessageCommon, MessageEncode,
    MessageMerge, ProtoMessage, ProtoString, SingularField,
};

use crate::Student;
use crate::school::{BIT_NAME, BIT_NAME_SSO, BIT_STUDENT, FIELD_NAME, FIELD_STUDENT};

/// Sample `School` — owned root with one inlined [`Student`].
pub struct School<A: Allocator = Global> {
    _common: MessageCommon<BitArray<[u8; 1], Lsb0>, A>,
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

impl<A: Allocator> School<A> {
    pub fn name<'a>(&'a self) -> Optional<&'a str, impl HasDefault<&'a str>>
    where
        A: 'a,
    {
        self.name.bind(&self._common).optional()
    }

    pub fn student(&self) -> Option<&Student<A>> {
        self.student.bind(&self._common).get()
    }

    fn visit_fields<V: FieldVisitor<MessageCommon<BitArray<[u8; 1], Lsb0>, A>>>(
        &self,
        v: &mut V,
    ) -> ControlFlow<V::Break> {
        v.visit("name", &self.name)?;
        v.visit("student", &self.student)?;
        ControlFlow::Continue(())
    }

    fn visit_field_pairs<V: FieldPairVisitor<MessageCommon<BitArray<[u8; 1], Lsb0>, A>>>(
        &self,
        other: &Self,
        v: &mut V,
    ) -> ControlFlow<V::Break> {
        v.visit("name", &self.name, &other.name)?;
        v.visit("student", &self.student, &other.student)?;
        ControlFlow::Continue(())
    }

    fn visit_field_pairs_mut<V: FieldPairVisitorMut<MessageCommon<BitArray<[u8; 1], Lsb0>, A>>>(
        &self,
        dst: &mut Self,
        v: &mut V,
    ) -> ControlFlow<V::Break>
    where
        A: Clone,
    {
        v.visit("name", &self.name, &mut dst.name)?;
        v.visit("student", &self.student, &mut dst.student)?;
        ControlFlow::Continue(())
    }

    fn visit_fields_mut<V: FieldVisitorMut<MessageCommon<BitArray<[u8; 1], Lsb0>, A>>>(
        &mut self,
        v: &mut V,
    ) -> ControlFlow<V::Break> {
        v.visit("name", &mut self.name)?;
        v.visit("student", &mut self.student)?;
        ControlFlow::Continue(())
    }
}

impl<A: Allocator + Clone> School<A> {
    pub fn new_in(alloc: A) -> Self {
        Self {
            _common: MessageCommon::new_in(BitArray::ZERO, alloc.clone()),
            name: SingularField::new_in(alloc.clone()),
            student: SingularField::new_in(alloc),
        }
    }

    pub fn name_mut(&mut self) -> impl ::puroro::StringMut<A> + '_ {
        self.name.bind_mut(&mut self._common).value_mut()
    }

    pub fn clear_name(&mut self) {
        self.name.bind_mut(&mut self._common).clear();
    }

    pub fn student_mut(&mut self) -> &mut Student<A> {
        self.student.bind_mut(&mut self._common).get_mut()
    }

    pub fn set_student(&mut self, src: Student<A>) {
        *self.student_mut() = src;
    }

    pub fn clear_student(&mut self) {
        self.student.bind_mut(&mut self._common).clear();
    }
}

impl School<Global> {
    pub fn new() -> Self {
        <Self as Default>::default()
    }
}

impl<A: Allocator + Clone + Default> Default for School<A> {
    fn default() -> Self {
        Self::new_in(A::default())
    }
}

impl<A: Allocator + Clone> CloneIn<A> for School<A> {
    fn clone_in(&self, alloc: A) -> Self {
        let mut dst = Self::new_in(alloc.clone());
        let mut v = CloneFieldsVisitor::new(&self._common, &dst._common);
        let _ = self.visit_field_pairs_mut(&mut dst, &mut v);
        let mut old = mem::replace(&mut dst._common, self._common.clone_in(alloc));
        old.deallocate();
        dst
    }
}

impl<A: Allocator + Clone> Clone for School<A> {
    #[inline]
    fn clone(&self) -> Self {
        self.clone_in(self._common.alloc.clone())
    }
}

impl<A: Allocator> PartialEq for School<A> {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            self.visit_field_pairs(
                other,
                &mut FieldEqVisitor::new(&self._common, &other._common)
            ),
            ControlFlow::Continue(())
        ) && self._common.unknown_fields_eq(&other._common)
    }
}

impl<A: Allocator> fmt::Debug for School<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut v = DebugStructVisitor::new(f.debug_struct("School"), &self._common);
        let _ = self.visit_fields(&mut v);
        v.finish()
    }
}

impl<A: Allocator> Drop for School<A> {
    fn drop(&mut self) {
        let mut v = FieldDeallocVisitor::new(&self._common);
        let _ = self.visit_fields_mut(&mut v);
        self._common.deallocate();
    }
}

impl<A: Allocator> ::puroro_rt::DeallocateIn<A> for School<A> {
    #[inline]
    unsafe fn deallocate_in(self, _alloc: &A) {
        drop(self);
    }
}

::puroro_rt::impl_owned_slot_bounds!(School);

impl<A: Allocator> MessageEncode for School<A> {
    fn encoded_len(&self, ctx: &mut EncodeCtx) -> usize {
        let mut v = EncodedLenVisitor::new(&self._common, ctx);
        let _ = self.visit_fields(&mut v);
        v.len + self._common.unknown_fields.len()
    }

    fn encode_raw<B: BufMut>(&self, ctx: &mut EncodeCtx, buf: &mut B) {
        let _ = self.visit_fields(&mut EncodeRawVisitor::new(&self._common, ctx, buf));
        let unknown: &[u8] = &self._common.unknown_fields;
        buf.put_slice(unknown);
    }
}

impl<A: Allocator + Clone> MessageMerge for School<A> {
    fn merge_from_with_depth<B: DecodeBuf>(
        &mut self,
        buf: &mut B,
        depth: usize,
    ) -> Result<(), DecodeError> {
        if depth >= ::puroro::RECURSION_LIMIT {
            return Err(DecodeError::RecursionLimitExceeded);
        }
        while buf.has_remaining() {
            let (field_number, wire_type) = decode_tag(buf)?;
            match field_number.as_u32() {
                FIELD_NAME => {
                    self.name
                        .bind_mut(&mut self._common)
                        .merge(wire_type, buf, depth)?;
                }
                FIELD_STUDENT => {
                    self.student
                        .bind_mut(&mut self._common)
                        .merge(wire_type, buf, depth)?;
                }
                _ => {
                    skip_field_and_save(
                        field_number,
                        wire_type,
                        buf,
                        &mut self._common.unknown_fields,
                        self._common.alloc.clone(),
                    )?;
                }
            }
        }
        Ok(())
    }
}

impl<A: Allocator + Clone> ::puroro_rt::DefaultIn<A> for School<A> {
    #[inline]
    fn default_in(alloc: A) -> Self {
        Self::new_in(alloc)
    }
}

impl<A: Allocator> Message for School<A> {
    type Alloc = A;

    fn new_in(alloc: A) -> Self
    where
        A: Clone,
    {
        Self::new_in(alloc)
    }

    fn encode<B: BufMut>(&self, buf: &mut B) {
        ::puroro_rt::encode_message(self, buf)
    }

    fn encode_to_vec(&self) -> Vec<u8> {
        ::puroro_rt::encode_message_to_vec(self)
    }

    fn merge_from<B: Buf>(&mut self, buf: &mut B) -> Result<(), DecodeError>
    where
        A: Clone,
    {
        ::puroro_rt::merge_message(self, buf)
    }

    fn unknown_fields(&self) -> impl Iterator<Item = ::puroro::UnknownField<'_>> + '_ {
        self._common.iter_unknown_fields()
    }

    fn validate(&self) -> Result<(), DecodeError> {
        Ok(())
    }
}
