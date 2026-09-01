//! Sample of a mid-level inlined message: [`StudentBody`] holds an EXPLICIT
//! scalar, an Explicit inlined [`Point`](crate::Point), and an Explicit inlined
//! [`Address`](crate::Address) (SSO / heap bits).
//!
//! Field accessors live on [`StudentBound`] (`C` / `B`). Owned [`Student`]
//! wraps that binding and is the only spelling that [`Drop`]s fields. Views
//! ([`StudentView`] / [`StudentMut`]) are the same bound type with a parent
//! [`Window`] / [`WindowMut`] — they stay [`Copy`] so NLL ends the parent
//! borrow after last use. A single `Student<A, C, B>` cannot both `Drop`
//! (owned) and be `Copy` (views).
//!
//! When inlined into [`School`](crate::School), teardown / clone of this body
//! go through [`DeallocateBound`](::puroro_rt::DeallocateBound) /
//! [`CloneBound`](::puroro_rt::CloneBound) with the parent window.

use allocator_api2::alloc::{Allocator, Global};
use bitvec::array::BitArray;
use bitvec::order::Lsb0;
use bytes::{Buf, BufMut};
use core::borrow::{Borrow, BorrowMut};
use core::fmt;
use core::marker::PhantomData;
use core::mem;
use core::ops::ControlFlow;
use core::ops::{Deref, DerefMut};

use puroro::{DecodeBuf, DecodeError, HasDefault, Message, Optional, StringMut};
use puroro_rt::decode::{decode_tag, skip_field_and_save};
use puroro_rt::{
    CloneBound, CloneFieldsVisitor, CloneIn, DeallocateBound, DeallocateIn, DebugStructVisitor,
    DefaultIn, EncodeCtx, EncodeRawVisitor, EncodedLenVisitor, Explicit, FieldCloneIn,
    FieldDeallocVisitor, FieldEqVisitor, FieldPairVisitor, FieldPairVisitorMut, FieldVisitor,
    FieldVisitorMut, InlinedMessageParent, MessageBinding, MessageBindingMut, MessageCommon,
    MessageEncode, MessageMerge, NestedMessage, ProtoInt32, SharedMessage, SingularField, Window,
    WindowMut,
};

use crate::Address;
use crate::Point;
use crate::address_type::{AddressMut, AddressView};
use crate::point_type::{PointMut, PointView};
use crate::student::{
    BIT_COUNT as STUDENT_BIT_COUNT, BIT_HOME, BIT_HOME_BASE, BIT_LOCATION, BIT_YEAR, FIELD_HOME,
    FIELD_LOCATION, FIELD_YEAR,
};

/// Field wrappers only — the inlined slot type (no [`MessageCommon`]).
pub struct StudentBody<A: Allocator = Global> {
    year: SingularField<ProtoInt32, Explicit<{ BIT_YEAR }>, { FIELD_YEAR }, A>,
    location: SingularField<
        SharedMessage<Point<A>, { FIELD_LOCATION }>,
        Explicit<{ BIT_LOCATION }>,
        { FIELD_LOCATION },
        A,
    >,
    home: SingularField<
        SharedMessage<Address<A>, { FIELD_HOME }, { BIT_HOME_BASE }>,
        Explicit<{ BIT_HOME }>,
        { FIELD_HOME },
        A,
    >,
}

/// Owned common for a standalone [`Student`].
pub type StudentOwnedCommon<A = Global> = MessageCommon<BitArray<[u8; 2], Lsb0>, A>;

/// Common + body binding. One accessor `impl` for owned, shared view, and mut.
///
/// - Owned inner: `C = StudentOwnedCommon<A>`, `B = StudentBody<A>`
/// - [`StudentView`]: `C = Window`, `B = &StudentBody`
/// - [`StudentMut`]: `C = WindowMut`, `B = &mut StudentBody`
pub struct StudentBound<A: Allocator, C, B> {
    common: C,
    body: B,
    _alloc: PhantomData<A>,
}

/// Owned message (`Drop` / `Message` / deep `Clone`). Field accessors via
/// [`Deref`] to [`StudentBound`].
pub struct Student<A: Allocator = Global> {
    inner: StudentBound<A, StudentOwnedCommon<A>, StudentBody<A>>,
}

/// Shared view: parent [`Window`] + `&StudentBody`.
pub type StudentView<'a, A = Global> = StudentBound<A, Window<'a, A>, &'a StudentBody<A>>;

/// Mutable view: parent [`WindowMut`] + `&mut StudentBody`.
pub type StudentMut<'a, A = Global> = StudentBound<A, WindowMut<'a, A>, &'a mut StudentBody<A>>;

impl<A: Allocator> Copy for StudentView<'_, A> {}

impl<A: Allocator> Clone for StudentView<'_, A> {
    fn clone(&self) -> Self {
        *self
    }
}

/// Infallible field getters (owned + views).
pub trait StudentMessage<A: Allocator> {
    fn year(&self) -> Optional<i32, impl HasDefault<i32>>;
    fn location(&self) -> Option<PointView<'_, A>>;
    fn home(&self) -> Option<AddressView<'_, A>>;
}

/// Mutable field accessors.
pub trait StudentMessageMut<A: Allocator>: StudentMessage<A> {
    fn year_mut(&mut self) -> impl DerefMut<Target = i32> + '_;
    fn location_mut(&mut self) -> PointMut<'_, A>;
    fn home_mut(&mut self) -> AddressMut<'_, A>;
}

impl<A: Allocator> StudentBody<A> {
    fn visit_fields<C, V: FieldVisitor<C>>(&self, v: &mut V) -> ControlFlow<V::Break>
    where
        C: MessageBindingMut<A>,
    {
        v.visit("year", &self.year)?;
        v.visit("location", &self.location)?;
        v.visit("home", &self.home)?;
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
        v.visit("year", &self.year, &other.year)?;
        v.visit("location", &self.location, &other.location)?;
        v.visit("home", &self.home, &other.home)?;
        ControlFlow::Continue(())
    }

    fn visit_field_pairs_mut<V>(&self, dst: &mut Self, v: &mut V) -> ControlFlow<V::Break>
    where
        V: FieldPairVisitorMut<MessageCommon<BitArray<[u8; 2], Lsb0>, A>>,
        A: Clone,
    {
        v.visit("year", &self.year, &mut dst.year)?;
        v.visit("location", &self.location, &mut dst.location)?;
        v.visit("home", &self.home, &mut dst.home)?;
        ControlFlow::Continue(())
    }

    fn visit_fields_mut<C, V>(&mut self, v: &mut V) -> ControlFlow<V::Break>
    where
        V: FieldVisitorMut<C>,
        C: MessageBindingMut<A>,
    {
        v.visit("year", &mut self.year)?;
        v.visit("location", &mut self.location)?;
        v.visit("home", &mut self.home)?;
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
                FIELD_YEAR => {
                    self.year.bind_mut(common).merge(wire_type, buf, depth)?;
                }
                FIELD_LOCATION => {
                    self.location
                        .bind_mut(common)
                        .merge(wire_type, buf, depth)?;
                }
                FIELD_HOME => {
                    self.home.bind_mut(common).merge(wire_type, buf, depth)?;
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

impl<A, C, B> StudentBound<A, C, B>
where
    A: Allocator,
    C: MessageBindingMut<A>,
    B: Borrow<StudentBody<A>>,
{
    pub fn year(&self) -> Optional<i32, impl HasDefault<i32>> {
        self.body.borrow().year.bind(&self.common).optional()
    }

    pub fn location(&self) -> Option<PointView<'_, A>> {
        self.body.borrow().location.bind(&self.common).get()
    }

    pub fn home(&self) -> Option<AddressView<'_, A>> {
        self.body.borrow().home.bind(&self.common).get()
    }
}

impl<A, C, B> StudentBound<A, C, B>
where
    A: Allocator + Clone,
    C: InlinedMessageParent<A>,
    B: BorrowMut<StudentBody<A>>,
{
    pub fn year_mut(&mut self) -> impl DerefMut<Target = i32> + '_ {
        self.body
            .borrow_mut()
            .year
            .bind_mut(&mut self.common)
            .value_mut()
    }

    pub fn clear_year(&mut self) {
        self.body
            .borrow_mut()
            .year
            .bind_mut(&mut self.common)
            .clear();
    }

    pub fn location_mut(&mut self) -> PointMut<'_, A> {
        self.body
            .borrow_mut()
            .location
            .bind_mut(&mut self.common)
            .get_mut()
    }

    pub fn set_location(&mut self, src: Point<A>) {
        self.location_mut().copy_from(&src);
    }

    pub fn clear_location(&mut self) {
        self.body
            .borrow_mut()
            .location
            .bind_mut(&mut self.common)
            .clear();
    }

    pub fn home_mut(&mut self) -> AddressMut<'_, A> {
        self.body
            .borrow_mut()
            .home
            .bind_mut(&mut self.common)
            .get_mut()
    }

    pub fn set_home(&mut self, src: Address<A>) {
        self.home_mut().copy_from(&src);
    }

    pub fn clear_home(&mut self) {
        self.body
            .borrow_mut()
            .home
            .bind_mut(&mut self.common)
            .clear();
    }

    /// Copies field values from an owned [`Student`] into this binding.
    pub fn copy_from(&mut self, src: &Student<A>) {
        if src.year().is_set() {
            *self.year_mut() = src.year().get();
        } else {
            self.clear_year();
        }
        if let Some(loc) = src.location() {
            *self.location_mut().x_mut() = loc.x();
            *self.location_mut().y_mut() = loc.y();
        } else {
            self.clear_location();
        }
        if let Some(home) = src.home() {
            if home.street().is_set() {
                self.home_mut().street_mut().set(home.street().get());
            } else {
                self.home_mut().clear_street();
            }
            if home.city().is_set() {
                self.home_mut().city_mut().set(home.city().get());
            } else {
                self.home_mut().clear_city();
            }
            if home.postal_code().is_set() {
                *self.home_mut().postal_code_mut() = home.postal_code().get();
            } else {
                self.home_mut().clear_postal_code();
            }
            if home.latitude().is_set() {
                *self.home_mut().latitude_mut() = home.latitude().get();
            } else {
                self.home_mut().clear_latitude();
            }
        } else {
            self.clear_home();
        }
    }

    pub fn merge_from<Buf: DecodeBuf>(&mut self, buf: &mut Buf) -> Result<(), DecodeError> {
        self.body.borrow_mut().merge_into(&mut self.common, buf, 0)
    }
}

impl<A: Allocator> Student<A> {
    fn visit_fields<V: FieldVisitor<MessageCommon<BitArray<[u8; 2], Lsb0>, A>>>(
        &self,
        v: &mut V,
    ) -> ControlFlow<V::Break> {
        self.inner.body.visit_fields(v)
    }

    fn visit_field_pairs<V: FieldPairVisitor<MessageCommon<BitArray<[u8; 2], Lsb0>, A>>>(
        &self,
        other: &Self,
        v: &mut V,
    ) -> ControlFlow<V::Break> {
        self.inner.body.visit_field_pairs(&other.inner.body, v)
    }

    fn visit_field_pairs_mut<V: FieldPairVisitorMut<MessageCommon<BitArray<[u8; 2], Lsb0>, A>>>(
        &self,
        dst: &mut Self,
        v: &mut V,
    ) -> ControlFlow<V::Break>
    where
        A: Clone,
    {
        self.inner
            .body
            .visit_field_pairs_mut(&mut dst.inner.body, v)
    }

    fn visit_fields_mut<V: FieldVisitorMut<MessageCommon<BitArray<[u8; 2], Lsb0>, A>>>(
        &mut self,
        v: &mut V,
    ) -> ControlFlow<V::Break> {
        self.inner.body.visit_fields_mut(v)
    }
}

impl<A: Allocator + Clone> Student<A> {
    pub fn new_in(alloc: A) -> Self {
        Self {
            inner: StudentBound {
                common: MessageCommon::new_in(BitArray::ZERO, alloc.clone()),
                body: StudentBody {
                    year: SingularField::new_in(alloc.clone()),
                    location: SingularField::new_in(alloc.clone()),
                    home: SingularField::new_in(alloc),
                },
                _alloc: PhantomData,
            },
        }
    }
}

impl Student<Global> {
    pub fn new() -> Self {
        <Self as Default>::default()
    }
}

impl<A: Allocator + Clone + Default> Default for Student<A> {
    fn default() -> Self {
        Self::new_in(A::default())
    }
}

impl<A: Allocator + Clone> CloneIn<A> for Student<A> {
    fn clone_in(&self, alloc: A) -> Self {
        let mut dst = Self::new_in(alloc.clone());
        let mut v = CloneFieldsVisitor::new(&self.inner.common, &dst.inner.common);
        let _ = self.visit_field_pairs_mut(&mut dst, &mut v);
        let mut old = mem::replace(&mut dst.inner.common, self.inner.common.clone_in(alloc));
        old.deallocate();
        dst
    }
}

impl<A: Allocator + Clone> Clone for Student<A> {
    #[inline]
    fn clone(&self) -> Self {
        self.clone_in(self.inner.common.alloc.clone())
    }
}

impl<A: Allocator> PartialEq for Student<A> {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            self.visit_field_pairs(
                other,
                &mut FieldEqVisitor::new(&self.inner.common, &other.inner.common)
            ),
            ControlFlow::Continue(())
        ) && self.inner.common.unknown_fields_eq(&other.inner.common)
    }
}

impl<A: Allocator> fmt::Debug for Student<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut v = DebugStructVisitor::new(f.debug_struct("Student"), &self.inner.common);
        let _ = self.visit_fields(&mut v);
        v.finish()
    }
}

impl<A: Allocator> Drop for Student<A> {
    fn drop(&mut self) {
        let mut v = FieldDeallocVisitor::new(&self.inner.common);
        let _ = self.inner.body.visit_fields_mut(&mut v);
        self.inner.common.deallocate();
    }
}

impl<A: Allocator> Deref for Student<A> {
    type Target = StudentBound<A, StudentOwnedCommon<A>, StudentBody<A>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<A: Allocator> DerefMut for Student<A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<A: Allocator> DeallocateIn<A> for Student<A> {
    #[inline]
    unsafe fn deallocate_in(self, _alloc: &A) {
        drop(self);
    }
}

::puroro_rt::impl_owned_slot_bounds!(Student);

impl<A: Allocator> MessageEncode for Student<A> {
    fn encoded_len(&self, ctx: &mut EncodeCtx) -> usize {
        let mut v = EncodedLenVisitor::new(&self.inner.common, ctx);
        let _ = self.visit_fields(&mut v);
        v.len + self.inner.common.unknown_fields.len()
    }

    fn encode_raw<B: BufMut>(&self, ctx: &mut EncodeCtx, buf: &mut B) {
        let _ = self.visit_fields(&mut EncodeRawVisitor::new(&self.inner.common, ctx, buf));
        let unknown: &[u8] = &self.inner.common.unknown_fields;
        buf.put_slice(unknown);
    }
}

impl<A: Allocator + Clone> MessageMerge for Student<A> {
    fn merge_from_with_depth<B: DecodeBuf>(
        &mut self,
        buf: &mut B,
        depth: usize,
    ) -> Result<(), DecodeError> {
        self.inner
            .body
            .merge_into(&mut self.inner.common, buf, depth)
    }
}

impl<A: Allocator + Clone> DefaultIn<A> for Student<A> {
    #[inline]
    fn default_in(alloc: A) -> Self {
        Self::new_in(alloc)
    }
}

impl<A: Allocator + Clone> DefaultIn<A> for StudentBody<A> {
    #[inline]
    fn default_in(alloc: A) -> Self {
        Self {
            year: SingularField::new_in(alloc.clone()),
            location: SingularField::new_in(alloc.clone()),
            home: SingularField::new_in(alloc),
        }
    }
}

impl<A: Allocator> DeallocateBound<A> for StudentBody<A> {
    fn deallocate_bound<Cx: MessageBindingMut<A>>(self, common: &Cx) {
        let mut body = self;
        let mut v = FieldDeallocVisitor::new(common);
        let _ = body.visit_fields_mut(&mut v);
    }
}

impl<A: Allocator + Clone> CloneBound<A> for StudentBody<A> {
    fn clone_bound<Cx: MessageBindingMut<A>>(&self, common: &Cx, alloc: A) -> Self {
        StudentBody {
            year: self.year.clone_field(common, alloc.clone()),
            location: self.location.clone_field(common, alloc.clone()),
            home: self.home.clone_field(common, alloc),
        }
    }
}

impl<A: Allocator> Message for Student<A> {
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
        self.inner.common.iter_unknown_fields()
    }

    fn validate(&self) -> Result<(), DecodeError> {
        Ok(())
    }
}

impl<A, C, B> StudentMessage<A> for StudentBound<A, C, B>
where
    A: Allocator,
    C: MessageBindingMut<A>,
    B: Borrow<StudentBody<A>>,
{
    fn year(&self) -> Optional<i32, impl HasDefault<i32>> {
        StudentBound::year(self)
    }
    fn location(&self) -> Option<PointView<'_, A>> {
        StudentBound::location(self)
    }
    fn home(&self) -> Option<AddressView<'_, A>> {
        StudentBound::home(self)
    }
}

impl<A: Allocator> StudentMessage<A> for Student<A> {
    fn year(&self) -> Optional<i32, impl HasDefault<i32>> {
        StudentBound::year(&self.inner)
    }
    fn location(&self) -> Option<PointView<'_, A>> {
        StudentBound::location(&self.inner)
    }
    fn home(&self) -> Option<AddressView<'_, A>> {
        StudentBound::home(&self.inner)
    }
}

impl<A, C, B> StudentMessageMut<A> for StudentBound<A, C, B>
where
    A: Allocator + Clone,
    C: InlinedMessageParent<A>,
    B: BorrowMut<StudentBody<A>>,
{
    fn year_mut(&mut self) -> impl DerefMut<Target = i32> + '_ {
        StudentBound::year_mut(self)
    }
    fn location_mut(&mut self) -> PointMut<'_, A> {
        StudentBound::location_mut(self)
    }
    fn home_mut(&mut self) -> AddressMut<'_, A> {
        StudentBound::home_mut(self)
    }
}

impl<A: Allocator + Clone> StudentMessageMut<A> for Student<A> {
    fn year_mut(&mut self) -> impl DerefMut<Target = i32> + '_ {
        StudentBound::year_mut(&mut self.inner)
    }
    fn location_mut(&mut self) -> PointMut<'_, A> {
        StudentBound::location_mut(&mut self.inner)
    }
    fn home_mut(&mut self) -> AddressMut<'_, A> {
        StudentBound::home_mut(&mut self.inner)
    }
}

impl<A: Allocator> PartialEq for StudentView<'_, A> {
    fn eq(&self, other: &Self) -> bool {
        self.year().is_set() == other.year().is_set()
            && self.year().get() == other.year().get()
            && self.location() == other.location()
            && self.home() == other.home()
    }
}

impl<A: Allocator> fmt::Debug for StudentView<'_, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut v = DebugStructVisitor::new(f.debug_struct("Student"), &self.common);
        let _ = self.body.visit_fields(&mut v);
        v.finish()
    }
}

impl<A: Allocator> Deref for StudentMut<'_, A> {
    type Target = StudentBody<A>;

    fn deref(&self) -> &StudentBody<A> {
        self.body
    }
}

impl<A: Allocator> NestedMessage for Student<A> {
    type Alloc = A;
    type Body = StudentBody<A>;
    const BIT_COUNT: usize = STUDENT_BIT_COUNT;
    type View<'a>
        = StudentView<'a, A>
    where
        A: 'a;
    type Mut<'a>
        = StudentMut<'a, A>
    where
        A: 'a;

    fn as_view(&self) -> StudentView<'_, A> {
        StudentBound {
            common: Window::for_owned(&self.inner.common),
            body: &self.inner.body,
            _alloc: PhantomData,
        }
    }

    fn as_mut(&mut self) -> StudentMut<'_, A> {
        let StudentBound {
            common,
            body,
            _alloc,
        } = &mut self.inner;
        StudentBound {
            common: WindowMut::for_owned(common),
            body,
            _alloc: PhantomData,
        }
    }

    fn bind_view<'a>(body: &'a StudentBody<A>, window: Window<'a, A>) -> StudentView<'a, A>
    where
        Self: 'a,
    {
        StudentBound {
            common: window,
            body,
            _alloc: PhantomData,
        }
    }

    fn bind_mut<'a>(body: &'a mut StudentBody<A>, window: WindowMut<'a, A>) -> StudentMut<'a, A>
    where
        Self: 'a,
    {
        StudentBound {
            common: window,
            body,
            _alloc: PhantomData,
        }
    }

    fn view_len(view: StudentView<'_, A>, ctx: &mut EncodeCtx) -> usize {
        let mut v = EncodedLenVisitor::new(&view.common, ctx);
        let _ = view.body.visit_fields(&mut v);
        v.len + view.common.unknown_fields().len()
    }

    fn encode_view<B: BufMut>(view: StudentView<'_, A>, ctx: &mut EncodeCtx, buf: &mut B) {
        let _ = view
            .body
            .visit_fields(&mut EncodeRawVisitor::new(&view.common, ctx, buf));
        buf.put_slice(view.common.unknown_fields().self_blob());
    }

    fn merge_inline<Ax, Buf>(
        body: &mut StudentBody<A>,
        window: &mut WindowMut<'_, Ax>,
        buf: &mut Buf,
        depth: usize,
    ) -> Result<(), DecodeError>
    where
        Ax: Allocator + Clone,
        Buf: DecodeBuf,
    {
        let body: &mut StudentBody<Ax> = unsafe { mem::transmute(body) };
        body.merge_into(window, buf, depth)
    }
}
