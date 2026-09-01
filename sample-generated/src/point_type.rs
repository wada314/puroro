//! Sample of a tiny nested message used to exercise **shared-common** inlined
//! storage: slot is [`PointBody`], getters return [`PointView`] / [`PointMut`].
//!
//! Two IMPLICIT `int32` fields. Not part of the DESIGN.md reference schema.

use allocator_api2::alloc::{Allocator, Global};
use bitvec::array::BitArray;
use bitvec::order::Lsb0;
use bytes::{Buf, BufMut};
use core::fmt;
use core::mem;
use core::ops::{ControlFlow, Deref, DerefMut};

use puroro::{DecodeBuf, DecodeError, Message};
use puroro_rt::decode::{decode_tag, skip_field_and_save};
use puroro_rt::{
    CloneBound, CloneFieldsVisitor, CloneIn, DeallocateBound, DeallocateIn, DebugStructVisitor,
    DefaultIn, EncodeCtx, EncodeRawVisitor, EncodedLenVisitor, FieldCloneIn, FieldDeallocVisitor,
    FieldEqVisitor, FieldPairVisitor, FieldPairVisitorMut, FieldVisitor, FieldVisitorMut, Implicit,
    InlinedMessageParent, MessageBinding, MessageBindingMut, MessageCommon, MessageEncode,
    MessageMerge, NestedMessage, ProtoInt32, SingularField, Window, WindowMut,
};

use crate::point::{BIT_COUNT as POINT_BIT_COUNT, FIELD_X, FIELD_Y};

/// Field wrappers only — the inlined slot type (no [`MessageCommon`]).
pub struct PointBody<A: Allocator = Global> {
    x: SingularField<ProtoInt32, Implicit, { FIELD_X }, A>,
    y: SingularField<ProtoInt32, Implicit, { FIELD_Y }, A>,
}

/// Owned `Point` (own [`MessageCommon`] + [`PointBody`]).
pub struct Point<A: Allocator = Global> {
    _common: MessageCommon<BitArray<[u8; 1], Lsb0>, A>,
    body: PointBody<A>,
}

/// Shared view: parent [`Window`] + `&PointBody`.
pub struct PointView<'a, A: Allocator> {
    window: Window<'a, A>,
    body: &'a PointBody<A>,
}

impl<A: Allocator> Copy for PointView<'_, A> {}

impl<A: Allocator> Clone for PointView<'_, A> {
    fn clone(&self) -> Self {
        *self
    }
}

/// Mutable view: parent [`WindowMut`] + `&mut PointBody`.
pub struct PointMut<'a, A: Allocator> {
    window: WindowMut<'a, A>,
    body: &'a mut PointBody<A>,
}

/// Infallible field getters (owned + views).
pub trait PointMessage {
    fn x(&self) -> i32;
    fn y(&self) -> i32;
}

/// Mutable field accessors.
pub trait PointMessageMut: PointMessage {
    fn x_mut(&mut self) -> impl DerefMut<Target = i32> + '_;
    fn y_mut(&mut self) -> impl DerefMut<Target = i32> + '_;
}

impl<A: Allocator> PointBody<A> {
    fn visit_fields<C, V: FieldVisitor<C>>(&self, v: &mut V) -> ControlFlow<V::Break>
    where
        C: MessageBindingMut<A>,
    {
        v.visit("x", &self.x)?;
        v.visit("y", &self.y)?;
        ControlFlow::Continue(())
    }

    fn visit_field_pairs<C, V: FieldPairVisitor<C>>(
        &self,
        other: &Self,
        v: &mut V,
    ) -> ControlFlow<V::Break>
    where
        C: MessageBindingMut<A>,
    {
        v.visit("x", &self.x, &other.x)?;
        v.visit("y", &self.y, &other.y)?;
        ControlFlow::Continue(())
    }

    fn visit_field_pairs_mut<V>(&self, dst: &mut Self, v: &mut V) -> ControlFlow<V::Break>
    where
        V: FieldPairVisitorMut<MessageCommon<BitArray<[u8; 1], Lsb0>, A>>,
        A: Clone,
    {
        v.visit("x", &self.x, &mut dst.x)?;
        v.visit("y", &self.y, &mut dst.y)?;
        ControlFlow::Continue(())
    }

    fn visit_fields_mut<C, V>(&mut self, v: &mut V) -> ControlFlow<V::Break>
    where
        V: FieldVisitorMut<C>,
        C: MessageBindingMut<A>,
    {
        v.visit("x", &mut self.x)?;
        v.visit("y", &mut self.y)?;
        ControlFlow::Continue(())
    }

    fn merge_into<C, B: DecodeBuf>(
        &mut self,
        common: &mut C,
        buf: &mut B,
        depth: usize,
    ) -> Result<(), DecodeError>
    where
        C: InlinedMessageParent<A>,
        A: Clone,
    {
        if depth >= ::puroro::RECURSION_LIMIT {
            return Err(DecodeError::RecursionLimitExceeded);
        }
        while buf.has_remaining() {
            let (field_number, wire_type) = decode_tag(buf)?;
            match field_number.as_u32() {
                FIELD_X => {
                    self.x.bind_mut(common).merge(wire_type, buf, depth)?;
                }
                FIELD_Y => {
                    self.y.bind_mut(common).merge(wire_type, buf, depth)?;
                }
                _ => {
                    let alloc = common.clone_alloc();
                    skip_field_and_save(
                        field_number,
                        wire_type,
                        buf,
                        common.unknown_fields_mut(),
                        alloc,
                    )?;
                }
            }
        }
        Ok(())
    }
}

impl<A: Allocator> Point<A> {
    pub fn x(&self) -> i32 {
        self.body.x.bind(&self._common).value()
    }

    pub fn y(&self) -> i32 {
        self.body.y.bind(&self._common).value()
    }

    fn visit_fields<V: FieldVisitor<MessageCommon<BitArray<[u8; 1], Lsb0>, A>>>(
        &self,
        v: &mut V,
    ) -> ControlFlow<V::Break> {
        self.body.visit_fields(v)
    }

    fn visit_field_pairs<V: FieldPairVisitor<MessageCommon<BitArray<[u8; 1], Lsb0>, A>>>(
        &self,
        other: &Self,
        v: &mut V,
    ) -> ControlFlow<V::Break> {
        self.body.visit_field_pairs(&other.body, v)
    }

    fn visit_field_pairs_mut<V: FieldPairVisitorMut<MessageCommon<BitArray<[u8; 1], Lsb0>, A>>>(
        &self,
        dst: &mut Self,
        v: &mut V,
    ) -> ControlFlow<V::Break>
    where
        A: Clone,
    {
        self.body.visit_field_pairs_mut(&mut dst.body, v)
    }

    fn visit_fields_mut<V: FieldVisitorMut<MessageCommon<BitArray<[u8; 1], Lsb0>, A>>>(
        &mut self,
        v: &mut V,
    ) -> ControlFlow<V::Break> {
        self.body.visit_fields_mut(v)
    }
}

impl<A: Allocator + Clone> Point<A> {
    pub fn new_in(alloc: A) -> Self {
        Self {
            _common: MessageCommon::new_in(BitArray::ZERO, alloc.clone()),
            body: PointBody {
                x: SingularField::new_in(alloc.clone()),
                y: SingularField::new_in(alloc),
            },
        }
    }

    pub fn x_mut(&mut self) -> impl DerefMut<Target = i32> + '_ {
        self.body.x.bind_mut(&mut self._common).value_mut()
    }

    pub fn clear_x(&mut self) {
        self.body.x.bind_mut(&mut self._common).clear();
    }

    pub fn y_mut(&mut self) -> impl DerefMut<Target = i32> + '_ {
        self.body.y.bind_mut(&mut self._common).value_mut()
    }

    pub fn clear_y(&mut self) {
        self.body.y.bind_mut(&mut self._common).clear();
    }
}

impl Point<Global> {
    pub fn new() -> Self {
        <Self as Default>::default()
    }
}

impl<A: Allocator + Clone + Default> Default for Point<A> {
    fn default() -> Self {
        Self::new_in(A::default())
    }
}

impl<A: Allocator + Clone> CloneIn<A> for Point<A> {
    fn clone_in(&self, alloc: A) -> Self {
        let mut dst = Self::new_in(alloc.clone());
        let mut v = CloneFieldsVisitor::new(&self._common, &dst._common);
        let _ = self.visit_field_pairs_mut(&mut dst, &mut v);
        let mut old = mem::replace(&mut dst._common, self._common.clone_in(alloc));
        old.deallocate();
        dst
    }
}

impl<A: Allocator + Clone> Clone for Point<A> {
    #[inline]
    fn clone(&self) -> Self {
        self.clone_in(self._common.alloc.clone())
    }
}

impl<A: Allocator> PartialEq for Point<A> {
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

impl<A: Allocator> fmt::Debug for Point<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut v = DebugStructVisitor::new(f.debug_struct("Point"), &self._common);
        let _ = self.visit_fields(&mut v);
        v.finish()
    }
}

impl<A: Allocator> Drop for Point<A> {
    fn drop(&mut self) {
        let mut v = FieldDeallocVisitor::new(&self._common);
        let _ = self.visit_fields_mut(&mut v);
        self._common.deallocate();
    }
}

impl<A: Allocator> DeallocateIn<A> for Point<A> {
    #[inline]
    unsafe fn deallocate_in(self, _alloc: &A) {
        drop(self);
    }
}

::puroro_rt::impl_owned_slot_bounds!(Point);

impl<A: Allocator> MessageEncode for Point<A> {
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

impl<A: Allocator + Clone> MessageMerge for Point<A> {
    fn merge_from_with_depth<B: DecodeBuf>(
        &mut self,
        buf: &mut B,
        depth: usize,
    ) -> Result<(), DecodeError> {
        self.body.merge_into(&mut self._common, buf, depth)
    }
}

impl<A: Allocator + Clone> DefaultIn<A> for Point<A> {
    #[inline]
    fn default_in(alloc: A) -> Self {
        Self::new_in(alloc)
    }
}

impl<A: Allocator + Clone> DefaultIn<A> for PointBody<A> {
    #[inline]
    fn default_in(alloc: A) -> Self {
        Self {
            x: SingularField::new_in(alloc.clone()),
            y: SingularField::new_in(alloc),
        }
    }
}

impl<A: Allocator> DeallocateBound<A> for PointBody<A> {
    fn deallocate_bound<Cx: MessageBindingMut<A>>(self, common: &Cx) {
        let mut body = self;
        let mut v = FieldDeallocVisitor::new(common);
        let _ = body.visit_fields_mut(&mut v);
    }
}

impl<A: Allocator + Clone> CloneBound<A> for PointBody<A> {
    fn clone_bound<Cx: MessageBindingMut<A>>(&self, common: &Cx, alloc: A) -> Self {
        PointBody {
            x: self.x.clone_field(common, alloc.clone()),
            y: self.y.clone_field(common, alloc),
        }
    }
}

impl<A: Allocator> Message for Point<A> {
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

impl<A: Allocator> PointMessage for Point<A> {
    fn x(&self) -> i32 {
        Point::x(self)
    }
    fn y(&self) -> i32 {
        Point::y(self)
    }
}

impl<A: Allocator + Clone> PointMessageMut for Point<A> {
    fn x_mut(&mut self) -> impl DerefMut<Target = i32> + '_ {
        Point::x_mut(self)
    }
    fn y_mut(&mut self) -> impl DerefMut<Target = i32> + '_ {
        Point::y_mut(self)
    }
}

impl<A: Allocator> PointView<'_, A> {
    pub fn x(&self) -> i32 {
        self.body.x.bind(&self.window).value()
    }

    pub fn y(&self) -> i32 {
        self.body.y.bind(&self.window).value()
    }
}

impl<A: Allocator> PointMessage for PointView<'_, A> {
    fn x(&self) -> i32 {
        PointView::x(self)
    }
    fn y(&self) -> i32 {
        PointView::y(self)
    }
}

impl<A: Allocator> PartialEq for PointView<'_, A> {
    fn eq(&self, other: &Self) -> bool {
        self.x() == other.x() && self.y() == other.y()
    }
}

impl<A: Allocator> fmt::Debug for PointView<'_, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut v = DebugStructVisitor::new(f.debug_struct("Point"), &self.window);
        let _ = self.body.visit_fields(&mut v);
        v.finish()
    }
}

impl<A: Allocator + Clone> PointMut<'_, A> {
    pub fn x(&self) -> i32 {
        self.body.x.bind(&self.window).value()
    }

    pub fn y(&self) -> i32 {
        self.body.y.bind(&self.window).value()
    }

    pub fn x_mut(&mut self) -> impl DerefMut<Target = i32> + '_ {
        self.body.x.bind_mut(&mut self.window).value_mut()
    }

    pub fn y_mut(&mut self) -> impl DerefMut<Target = i32> + '_ {
        self.body.y.bind_mut(&mut self.window).value_mut()
    }

    /// Copies field values from an owned [`Point`] into this inlined slot.
    pub fn copy_from(&mut self, src: &Point<A>) {
        *self.x_mut() = src.x();
        *self.y_mut() = src.y();
    }

    pub fn merge_from<B: DecodeBuf>(&mut self, buf: &mut B) -> Result<(), DecodeError> {
        self.body.merge_into(&mut self.window, buf, 0)
    }
}

impl<A: Allocator + Clone> PointMessage for PointMut<'_, A> {
    fn x(&self) -> i32 {
        PointMut::x(self)
    }
    fn y(&self) -> i32 {
        PointMut::y(self)
    }
}

impl<A: Allocator + Clone> PointMessageMut for PointMut<'_, A> {
    fn x_mut(&mut self) -> impl DerefMut<Target = i32> + '_ {
        PointMut::x_mut(self)
    }
    fn y_mut(&mut self) -> impl DerefMut<Target = i32> + '_ {
        PointMut::y_mut(self)
    }
}

impl<A: Allocator> Deref for PointMut<'_, A> {
    type Target = PointBody<A>;

    fn deref(&self) -> &PointBody<A> {
        self.body
    }
}

impl<A: Allocator> NestedMessage for Point<A> {
    type Alloc = A;
    type Body = PointBody<A>;
    const BIT_COUNT: usize = POINT_BIT_COUNT;
    type View<'a>
        = PointView<'a, A>
    where
        A: 'a;
    type Mut<'a>
        = PointMut<'a, A>
    where
        A: 'a;

    fn as_view(&self) -> PointView<'_, A> {
        PointView {
            window: Window::for_owned(&self._common),
            body: &self.body,
        }
    }

    fn as_mut(&mut self) -> PointMut<'_, A> {
        let Point { _common, body } = self;
        PointMut {
            window: WindowMut::for_owned(_common),
            body,
        }
    }

    fn bind_view<'a>(body: &'a PointBody<A>, window: Window<'a, A>) -> PointView<'a, A>
    where
        Self: 'a,
    {
        PointView { window, body }
    }

    fn bind_mut<'a>(body: &'a mut PointBody<A>, window: WindowMut<'a, A>) -> PointMut<'a, A>
    where
        Self: 'a,
    {
        PointMut { window, body }
    }

    fn view_len(view: PointView<'_, A>, ctx: &mut EncodeCtx) -> usize {
        let mut v = EncodedLenVisitor::new(&view.window, ctx);
        let _ = view.body.visit_fields(&mut v);
        v.len + view.window.unknown_fields().len()
    }

    fn encode_view<B: BufMut>(view: PointView<'_, A>, ctx: &mut EncodeCtx, buf: &mut B) {
        let _ = view
            .body
            .visit_fields(&mut EncodeRawVisitor::new(&view.window, ctx, buf));
        buf.put_slice(view.window.unknown_fields().self_blob());
    }

    fn merge_inline<Ax, Buf>(
        body: &mut PointBody<A>,
        window: &mut WindowMut<'_, Ax>,
        buf: &mut Buf,
        depth: usize,
    ) -> Result<(), DecodeError>
    where
        Ax: Allocator + Clone,
        Buf: DecodeBuf,
    {
        let body: &mut PointBody<Ax> = unsafe { mem::transmute(body) };
        body.merge_into(window, buf, depth)
    }
}
