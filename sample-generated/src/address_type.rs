//! Sample of the code puroro generates for message `example.Address`
//! (from `example.proto`). The public type is re-exported at the crate root so
//! it sits beside companion module [`crate::address`].
//!
//! Field wrappers live in [`AddressBody`]. Owned [`Address`] keeps its own
//! [`MessageCommon`]. [`Task.assignee`](crate::Task) uses
//! [`SharedMessage<Address>`](::puroro_rt::SharedMessage) + [`Boxed`](::puroro_rt::Boxed):
//! the slot is the owned message; getters are [`AddressView`] / [`AddressMut`]
//! (`Window` onto the child’s common, `bit_base = 0`).

use allocator_api2::alloc::{Allocator, Global};
use bitvec::array::BitArray;
use bitvec::order::Lsb0;
use bytes::{Buf, BufMut};
use core::fmt;
use core::mem;
use core::ops::ControlFlow;
use core::ops::{Deref, DerefMut};

use puroro::{DecodeBuf, DecodeError, HasDefault, Message, Optional, StringMut};
use puroro_rt::decode::{decode_tag, skip_field_and_save};
use puroro_rt::{
    CloneBound, CloneFieldsVisitor, CloneIn, DeallocateBound, DebugStructVisitor, EncodeCtx,
    EncodeRawVisitor, EncodedLenVisitor, Explicit, FieldCloneIn, FieldDeallocVisitor,
    FieldEqVisitor, FieldPairVisitor, FieldPairVisitorMut, FieldVisitor, FieldVisitorMut,
    InlineOrHeap, InlinedMessageParent, MessageBinding, MessageBindingMut, MessageCommon,
    MessageEncode, MessageMerge, NestedMessage, ProtoDouble, ProtoFixed32, ProtoString,
    SingularField, Window, WindowMut,
};

use crate::address::{
    BIT_CITY, BIT_CITY_SSO, BIT_COUNT as ADDRESS_BIT_COUNT, BIT_LATITUDE, BIT_POSTAL_CODE,
    BIT_STREET, BIT_STREET_SSO, FIELD_CITY, FIELD_LATITUDE, FIELD_POSTAL_CODE, FIELD_STREET,
};

/// Field wrappers only — no [`MessageCommon`].
pub struct AddressBody<A: Allocator = Global> {
    street: SingularField<
        ProtoString,
        Explicit<{ BIT_STREET }>,
        { FIELD_STREET },
        A,
        InlineOrHeap<{ BIT_STREET_SSO }>,
    >, // proto: string street = 1;
    city: SingularField<
        ProtoString,
        Explicit<{ BIT_CITY }>,
        { FIELD_CITY },
        A,
        InlineOrHeap<{ BIT_CITY_SSO }>,
    >, // proto: string city = 2;
    postal_code:
        SingularField<ProtoFixed32, Explicit<{ BIT_POSTAL_CODE }>, { FIELD_POSTAL_CODE }, A>, // proto: fixed32 postal_code = 3;
    latitude: SingularField<ProtoDouble, Explicit<{ BIT_LATITUDE }>, { FIELD_LATITUDE }, A>, // proto: double latitude = 4;
}

/// Owned `Address` (own [`MessageCommon`] + [`AddressBody`]).
pub struct Address<A: Allocator = Global> {
    _common: MessageCommon<BitArray<[u8; 1], Lsb0>, A>,
    body: AddressBody<A>,
}

/// Shared view: [`Window`] onto this message's common + `&AddressBody`.
pub struct AddressView<'a, A: Allocator> {
    window: Window<'a, A>,
    body: &'a AddressBody<A>,
}

impl<A: Allocator> Copy for AddressView<'_, A> {}

impl<A: Allocator> Clone for AddressView<'_, A> {
    fn clone(&self) -> Self {
        *self
    }
}

/// Mutable view: [`WindowMut`] + `&mut AddressBody`.
pub struct AddressMut<'a, A: Allocator> {
    window: WindowMut<'a, A>,
    body: &'a mut AddressBody<A>,
}

/// Field getters (owned + views).
pub trait AddressMessage {
    fn street(&self) -> Optional<&str, impl HasDefault<&str>>;
    fn city(&self) -> Optional<&str, impl HasDefault<&str>>;
    fn postal_code(&self) -> Optional<u32, impl HasDefault<u32>>;
    fn latitude(&self) -> Optional<f64, impl HasDefault<f64>>;
}

/// Mutable field accessors.
pub trait AddressMessageMut<A: Allocator>: AddressMessage {
    fn street_mut(&mut self) -> impl ::puroro::StringMut<A> + '_;
    fn city_mut(&mut self) -> impl ::puroro::StringMut<A> + '_;
    fn postal_code_mut(&mut self) -> impl DerefMut<Target = u32> + '_;
    fn latitude_mut(&mut self) -> impl DerefMut<Target = f64> + '_;
}

impl<A: Allocator> AddressBody<A> {
    fn visit_fields<C, V: FieldVisitor<C>>(&self, v: &mut V) -> ControlFlow<V::Break>
    where
        C: MessageBindingMut<A>,
    {
        v.visit("street", &self.street)?;
        v.visit("city", &self.city)?;
        v.visit("postal_code", &self.postal_code)?;
        v.visit("latitude", &self.latitude)?;
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
        v.visit("street", &self.street, &other.street)?;
        v.visit("city", &self.city, &other.city)?;
        v.visit("postal_code", &self.postal_code, &other.postal_code)?;
        v.visit("latitude", &self.latitude, &other.latitude)?;
        ControlFlow::Continue(())
    }

    fn visit_field_pairs_mut<V>(&self, dst: &mut Self, v: &mut V) -> ControlFlow<V::Break>
    where
        V: FieldPairVisitorMut<MessageCommon<BitArray<[u8; 1], Lsb0>, A>>,
        A: Clone,
    {
        v.visit("street", &self.street, &mut dst.street)?;
        v.visit("city", &self.city, &mut dst.city)?;
        v.visit("postal_code", &self.postal_code, &mut dst.postal_code)?;
        v.visit("latitude", &self.latitude, &mut dst.latitude)?;
        ControlFlow::Continue(())
    }

    fn visit_fields_mut<C, V>(&mut self, v: &mut V) -> ControlFlow<V::Break>
    where
        V: FieldVisitorMut<C>,
        C: MessageBindingMut<A>,
    {
        v.visit("street", &mut self.street)?;
        v.visit("city", &mut self.city)?;
        v.visit("postal_code", &mut self.postal_code)?;
        v.visit("latitude", &mut self.latitude)?;
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
                FIELD_STREET => {
                    self.street.bind_mut(common).merge(wire_type, buf, depth)?;
                }
                FIELD_CITY => {
                    self.city.bind_mut(common).merge(wire_type, buf, depth)?;
                }
                FIELD_POSTAL_CODE => {
                    self.postal_code
                        .bind_mut(common)
                        .merge(wire_type, buf, depth)?;
                }
                FIELD_LATITUDE => {
                    self.latitude
                        .bind_mut(common)
                        .merge(wire_type, buf, depth)?;
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

impl<A: Allocator> Address<A> {
    pub fn street<'a>(&'a self) -> Optional<&'a str, impl HasDefault<&'a str>>
    where
        A: 'a,
    {
        self.body.street.bind(&self._common).optional()
    }

    pub fn city<'a>(&'a self) -> Optional<&'a str, impl HasDefault<&'a str>>
    where
        A: 'a,
    {
        self.body.city.bind(&self._common).optional()
    }

    pub fn postal_code<'a>(&'a self) -> Optional<u32, impl HasDefault<u32>>
    where
        A: 'a,
    {
        self.body.postal_code.bind(&self._common).optional()
    }

    pub fn latitude<'a>(&'a self) -> Optional<f64, impl HasDefault<f64>>
    where
        A: 'a,
    {
        self.body.latitude.bind(&self._common).optional()
    }

    /// Window onto this owned message's common (`bit_base = 0`).
    pub fn as_view(&self) -> AddressView<'_, A> {
        AddressView {
            window: Window::for_owned(&self._common),
            body: &self.body,
        }
    }

    /// Mutable window onto this owned message's common (`bit_base = 0`).
    pub fn as_mut(&mut self) -> AddressMut<'_, A> {
        let Address { _common, body } = self;
        AddressMut {
            window: WindowMut::for_owned(_common),
            body,
        }
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

impl<A: Allocator + Clone> Address<A> {
    pub fn new_in(alloc: A) -> Self {
        Self {
            _common: MessageCommon::new_in(BitArray::ZERO, alloc.clone()),
            body: AddressBody {
                street: SingularField::new_in(alloc.clone()),
                city: SingularField::new_in(alloc.clone()),
                postal_code: SingularField::new_in(alloc.clone()),
                latitude: SingularField::new_in(alloc),
            },
        }
    }

    pub fn street_mut(&mut self) -> impl ::puroro::StringMut<A> + '_ {
        self.body.street.bind_mut(&mut self._common).value_mut()
    }

    pub fn clear_street(&mut self) {
        self.body.street.bind_mut(&mut self._common).clear();
    }

    pub fn city_mut(&mut self) -> impl ::puroro::StringMut<A> + '_ {
        self.body.city.bind_mut(&mut self._common).value_mut()
    }

    pub fn clear_city(&mut self) {
        self.body.city.bind_mut(&mut self._common).clear();
    }

    pub fn postal_code_mut(&mut self) -> impl DerefMut<Target = u32> + '_ {
        self.body
            .postal_code
            .bind_mut(&mut self._common)
            .value_mut()
    }

    pub fn clear_postal_code(&mut self) {
        self.body.postal_code.bind_mut(&mut self._common).clear();
    }

    pub fn latitude_mut(&mut self) -> impl DerefMut<Target = f64> + '_ {
        self.body.latitude.bind_mut(&mut self._common).value_mut()
    }

    pub fn clear_latitude(&mut self) {
        self.body.latitude.bind_mut(&mut self._common).clear();
    }
}

impl Address<Global> {
    pub fn new() -> Self {
        <Self as Default>::default()
    }
}

impl<A: Allocator + Clone + Default> Default for Address<A> {
    fn default() -> Self {
        Self::new_in(A::default())
    }
}

impl<A: Allocator + Clone> CloneIn<A> for Address<A> {
    fn clone_in(&self, alloc: A) -> Self {
        let mut dst = Self::new_in(alloc.clone());
        let mut v = CloneFieldsVisitor::new(&self._common, &dst._common);
        let _ = self.visit_field_pairs_mut(&mut dst, &mut v);
        let mut old = mem::replace(&mut dst._common, self._common.clone_in(alloc));
        old.deallocate();
        dst
    }
}

impl<A: Allocator + Clone> Clone for Address<A> {
    #[inline]
    fn clone(&self) -> Self {
        self.clone_in(self._common.alloc.clone())
    }
}

impl<A: Allocator> PartialEq for Address<A> {
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

impl<A: Allocator> fmt::Debug for Address<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut v = DebugStructVisitor::new(f.debug_struct("Address"), &self._common);
        let _ = self.visit_fields(&mut v);
        v.finish()
    }
}

impl<A: Allocator> Drop for Address<A> {
    fn drop(&mut self) {
        let mut v = FieldDeallocVisitor::new(&self._common);
        let _ = self.visit_fields_mut(&mut v);
        self._common.deallocate();
    }
}

impl<A: Allocator> ::puroro_rt::DeallocateIn<A> for Address<A> {
    #[inline]
    unsafe fn deallocate_in(self, _alloc: &A) {
        drop(self);
    }
}

::puroro_rt::impl_owned_slot_bounds!(Address);

impl<A: Allocator> MessageEncode for Address<A> {
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

impl<A: Allocator + Clone> MessageMerge for Address<A> {
    fn merge_from_with_depth<B: DecodeBuf>(
        &mut self,
        buf: &mut B,
        depth: usize,
    ) -> Result<(), DecodeError> {
        self.body.merge_into(&mut self._common, buf, depth)
    }
}

impl<A: Allocator + Clone> ::puroro_rt::DefaultIn<A> for Address<A> {
    #[inline]
    fn default_in(alloc: A) -> Self {
        Self::new_in(alloc)
    }
}

impl<A: Allocator + Clone> ::puroro_rt::DefaultIn<A> for AddressBody<A> {
    #[inline]
    fn default_in(alloc: A) -> Self {
        Self {
            street: SingularField::new_in(alloc.clone()),
            city: SingularField::new_in(alloc.clone()),
            postal_code: SingularField::new_in(alloc.clone()),
            latitude: SingularField::new_in(alloc),
        }
    }
}

impl<A: Allocator> DeallocateBound<A> for AddressBody<A> {
    fn deallocate_bound<Cx: MessageBindingMut<A>>(self, common: &Cx) {
        let mut body = self;
        let mut v = FieldDeallocVisitor::new(common);
        let _ = body.visit_fields_mut(&mut v);
    }
}

impl<A: Allocator + Clone> CloneBound<A> for AddressBody<A> {
    fn clone_bound<Cx: MessageBindingMut<A>>(&self, common: &Cx, alloc: A) -> Self {
        AddressBody {
            street: self.street.clone_field(common, alloc.clone()),
            city: self.city.clone_field(common, alloc.clone()),
            postal_code: self.postal_code.clone_field(common, alloc.clone()),
            latitude: self.latitude.clone_field(common, alloc),
        }
    }
}

impl<A: Allocator> Message for Address<A> {
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

impl<A: Allocator> AddressMessage for Address<A> {
    fn street(&self) -> Optional<&str, impl HasDefault<&str>> {
        Address::street(self)
    }
    fn city(&self) -> Optional<&str, impl HasDefault<&str>> {
        Address::city(self)
    }
    fn postal_code(&self) -> Optional<u32, impl HasDefault<u32>> {
        Address::postal_code(self)
    }
    fn latitude(&self) -> Optional<f64, impl HasDefault<f64>> {
        Address::latitude(self)
    }
}

impl<A: Allocator + Clone> AddressMessageMut<A> for Address<A> {
    fn street_mut(&mut self) -> impl ::puroro::StringMut<A> + '_ {
        Address::street_mut(self)
    }
    fn city_mut(&mut self) -> impl ::puroro::StringMut<A> + '_ {
        Address::city_mut(self)
    }
    fn postal_code_mut(&mut self) -> impl DerefMut<Target = u32> + '_ {
        Address::postal_code_mut(self)
    }
    fn latitude_mut(&mut self) -> impl DerefMut<Target = f64> + '_ {
        Address::latitude_mut(self)
    }
}

impl<A: Allocator> AddressView<'_, A> {
    pub fn street(&self) -> Optional<&str, impl HasDefault<&str>> {
        self.body.street.bind(&self.window).optional()
    }

    pub fn city(&self) -> Optional<&str, impl HasDefault<&str>> {
        self.body.city.bind(&self.window).optional()
    }

    pub fn postal_code(&self) -> Optional<u32, impl HasDefault<u32>> {
        self.body.postal_code.bind(&self.window).optional()
    }

    pub fn latitude(&self) -> Optional<f64, impl HasDefault<f64>> {
        self.body.latitude.bind(&self.window).optional()
    }
}

impl<A: Allocator> AddressMessage for AddressView<'_, A> {
    fn street(&self) -> Optional<&str, impl HasDefault<&str>> {
        AddressView::street(self)
    }
    fn city(&self) -> Optional<&str, impl HasDefault<&str>> {
        AddressView::city(self)
    }
    fn postal_code(&self) -> Optional<u32, impl HasDefault<u32>> {
        AddressView::postal_code(self)
    }
    fn latitude(&self) -> Optional<f64, impl HasDefault<f64>> {
        AddressView::latitude(self)
    }
}

impl<A: Allocator> PartialEq for AddressView<'_, A> {
    fn eq(&self, other: &Self) -> bool {
        self.street().get() == other.street().get()
            && self.city().get() == other.city().get()
            && self.postal_code().get() == other.postal_code().get()
            && self.latitude().get() == other.latitude().get()
    }
}

impl<A: Allocator> fmt::Debug for AddressView<'_, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut v = DebugStructVisitor::new(f.debug_struct("Address"), &self.window);
        let _ = self.body.visit_fields(&mut v);
        v.finish()
    }
}

impl<A: Allocator + Clone> AddressMut<'_, A> {
    pub fn street(&self) -> Optional<&str, impl HasDefault<&str>> {
        self.body.street.bind(&self.window).optional()
    }

    pub fn city(&self) -> Optional<&str, impl HasDefault<&str>> {
        self.body.city.bind(&self.window).optional()
    }

    pub fn postal_code(&self) -> Optional<u32, impl HasDefault<u32>> {
        self.body.postal_code.bind(&self.window).optional()
    }

    pub fn latitude(&self) -> Optional<f64, impl HasDefault<f64>> {
        self.body.latitude.bind(&self.window).optional()
    }

    pub fn street_mut(&mut self) -> impl ::puroro::StringMut<A> + '_ {
        self.body.street.bind_mut(&mut self.window).value_mut()
    }

    pub fn clear_street(&mut self) {
        self.body.street.bind_mut(&mut self.window).clear();
    }

    pub fn city_mut(&mut self) -> impl ::puroro::StringMut<A> + '_ {
        self.body.city.bind_mut(&mut self.window).value_mut()
    }

    pub fn clear_city(&mut self) {
        self.body.city.bind_mut(&mut self.window).clear();
    }

    pub fn postal_code_mut(&mut self) -> impl DerefMut<Target = u32> + '_ {
        self.body.postal_code.bind_mut(&mut self.window).value_mut()
    }

    pub fn clear_postal_code(&mut self) {
        self.body.postal_code.bind_mut(&mut self.window).clear();
    }

    pub fn latitude_mut(&mut self) -> impl DerefMut<Target = f64> + '_ {
        self.body.latitude.bind_mut(&mut self.window).value_mut()
    }

    pub fn clear_latitude(&mut self) {
        self.body.latitude.bind_mut(&mut self.window).clear();
    }

    /// Copies field values from an owned [`Address`] into this view.
    pub fn copy_from(&mut self, src: &Address<A>) {
        if src.street().is_set() {
            self.street_mut().set(src.street().get());
        } else {
            self.clear_street();
        }
        if src.city().is_set() {
            self.city_mut().set(src.city().get());
        } else {
            self.clear_city();
        }
        if src.postal_code().is_set() {
            *self.postal_code_mut() = src.postal_code().get();
        } else {
            self.clear_postal_code();
        }
        if src.latitude().is_set() {
            *self.latitude_mut() = src.latitude().get();
        } else {
            self.clear_latitude();
        }
    }

    pub fn merge_from<B: DecodeBuf>(&mut self, buf: &mut B) -> Result<(), DecodeError> {
        self.body.merge_into(&mut self.window, buf, 0)
    }
}

impl<A: Allocator + Clone> AddressMessage for AddressMut<'_, A> {
    fn street(&self) -> Optional<&str, impl HasDefault<&str>> {
        AddressMut::street(self)
    }
    fn city(&self) -> Optional<&str, impl HasDefault<&str>> {
        AddressMut::city(self)
    }
    fn postal_code(&self) -> Optional<u32, impl HasDefault<u32>> {
        AddressMut::postal_code(self)
    }
    fn latitude(&self) -> Optional<f64, impl HasDefault<f64>> {
        AddressMut::latitude(self)
    }
}

impl<A: Allocator + Clone> AddressMessageMut<A> for AddressMut<'_, A> {
    fn street_mut(&mut self) -> impl ::puroro::StringMut<A> + '_ {
        AddressMut::street_mut(self)
    }
    fn city_mut(&mut self) -> impl ::puroro::StringMut<A> + '_ {
        AddressMut::city_mut(self)
    }
    fn postal_code_mut(&mut self) -> impl DerefMut<Target = u32> + '_ {
        AddressMut::postal_code_mut(self)
    }
    fn latitude_mut(&mut self) -> impl DerefMut<Target = f64> + '_ {
        AddressMut::latitude_mut(self)
    }
}

impl<A: Allocator> Deref for AddressMut<'_, A> {
    type Target = AddressBody<A>;

    fn deref(&self) -> &AddressBody<A> {
        self.body
    }
}

impl<A: Allocator> NestedMessage for Address<A> {
    type Alloc = A;
    type Body = AddressBody<A>;
    const BIT_COUNT: usize = ADDRESS_BIT_COUNT;
    type View<'a>
        = AddressView<'a, A>
    where
        A: 'a;
    type Mut<'a>
        = AddressMut<'a, A>
    where
        A: 'a;

    fn as_view(&self) -> AddressView<'_, A> {
        Address::as_view(self)
    }

    fn as_mut(&mut self) -> AddressMut<'_, A> {
        Address::as_mut(self)
    }

    fn bind_view<'a>(body: &'a AddressBody<A>, window: Window<'a, A>) -> AddressView<'a, A>
    where
        Self: 'a,
    {
        AddressView { window, body }
    }

    fn bind_mut<'a>(body: &'a mut AddressBody<A>, window: WindowMut<'a, A>) -> AddressMut<'a, A>
    where
        Self: 'a,
    {
        AddressMut { window, body }
    }

    fn view_len(view: AddressView<'_, A>, ctx: &mut EncodeCtx) -> usize {
        let mut v = EncodedLenVisitor::new(&view.window, ctx);
        let _ = view.body.visit_fields(&mut v);
        v.len + view.window.unknown_fields().len()
    }

    fn encode_view<B: BufMut>(view: AddressView<'_, A>, ctx: &mut EncodeCtx, buf: &mut B) {
        let _ = view
            .body
            .visit_fields(&mut EncodeRawVisitor::new(&view.window, ctx, buf));
        buf.put_slice(view.window.unknown_fields().self_blob());
    }

    fn merge_inline<Ax, Buf>(
        body: &mut AddressBody<A>,
        window: &mut WindowMut<'_, Ax>,
        buf: &mut Buf,
        depth: usize,
    ) -> Result<(), DecodeError>
    where
        Ax: Allocator + Clone,
        Buf: DecodeBuf,
    {
        let body: &mut AddressBody<Ax> = unsafe { mem::transmute(body) };
        body.merge_into(window, buf, depth)
    }
}
