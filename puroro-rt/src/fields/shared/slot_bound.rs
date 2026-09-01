//! Teardown / clone of an **extracted slot value** through a message binding.
//!
//! Distinct from [`FieldDeallocate`](super::FieldDeallocate) / [`FieldCloneIn`](super::FieldCloneIn),
//! which operate on catalog field wrappers (`&mut SingularField`, …) that still
//! live in the parent struct. These traits consume (or borrow) the payload
//! after [`ValueSlot::take_value`](super::value_slot::ValueSlot::take_value).
//!
//! `Cx` is a **method** parameter so [`Inline`](super::value_layout::Inline) can
//! require `Slot: DeallocateBound<A>` on the layout impl, while
//! [`InlineOrHeap`](super::value_layout::InlineOrHeap) does not (SSO teardown
//! is `HEAP_BIT` + [`SsoBuf::deallocate`](crate::fields::wire::sso_buf::SsoBuf::deallocate)).
//!
//! There is **no** blanket over [`DeallocateIn`] / [`CloneIn`]: a generated
//! message Body also impls these traits, and a blanket would be incoherent
//! (downstream could add `DeallocateIn` on that Body). Copy scalars and
//! unmanaged containers are listed explicitly; generated Bodies / owned
//! messages / enums impl the traits next to their type.

use ::allocator_api2::alloc::Allocator;
use ::unmanaged::{CloneIn, DeallocateIn, UnmanagedBox, UnmanagedString, UnmanagedVec};

use super::MessageBindingMut;

/// Consume an extracted slot value, using `common` for bits and the allocator.
///
/// Layouts whose discriminant lives in `common` (SSO `HEAP_BIT`) do **not**
/// implement this; they tear down through their [`ValueLayout`](super::value_layout::ValueLayout).
pub trait DeallocateBound<A: Allocator> {
    /// Releases heap owned by `self`. `common` must be this value's binding.
    fn deallocate_bound<Cx: MessageBindingMut<A>>(self, common: &Cx);
}

/// Deep-copy an extracted slot value, using the **source** `common` for bits.
///
/// Same split as [`DeallocateBound`]: SSO clone is [`InlineOrHeap`](super::value_layout::InlineOrHeap),
/// not this trait.
pub trait CloneBound<A: Allocator + Clone> {
    /// Copies `self` into `alloc`. Destination bits are installed later.
    fn clone_bound<Cx: MessageBindingMut<A>>(&self, common: &Cx, alloc: A) -> Self;
}

/// Copy / ZST slots: bits are unused, teardown is a no-op.
macro_rules! impl_primitive_slot_bounds {
    ($($t:ty),+ $(,)?) => {$(
        impl<A: Allocator> DeallocateBound<A> for $t {
            #[inline]
            fn deallocate_bound<Cx: MessageBindingMut<A>>(self, _common: &Cx) {}
        }

        impl<A: Allocator + Clone> CloneBound<A> for $t {
            #[inline]
            fn clone_bound<Cx: MessageBindingMut<A>>(&self, _common: &Cx, _alloc: A) -> Self {
                *self
            }
        }
    )+};
}

impl_primitive_slot_bounds!(bool, i32, i64, u32, u64, f32, f64, ());

/// For generated `Copy` enum newtypes (`Status`, `Priority`, …).
#[macro_export]
macro_rules! impl_copy_slot_bounds {
    ($t:ty) => {
        impl<A: ::allocator_api2::alloc::Allocator> $crate::DeallocateBound<A> for $t {
            #[inline]
            fn deallocate_bound<Cx: $crate::MessageBindingMut<A>>(self, _common: &Cx) {}
        }

        impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> $crate::CloneBound<A>
            for $t
        {
            #[inline]
            fn clone_bound<Cx: $crate::MessageBindingMut<A>>(
                &self,
                _common: &Cx,
                _alloc: A,
            ) -> Self {
                *self
            }
        }
    };
}

/// For generated owned messages that already impl [`DeallocateIn`] / [`CloneIn`].
#[macro_export]
macro_rules! impl_owned_slot_bounds {
    ($t:ident) => {
        impl<A: ::allocator_api2::alloc::Allocator> $crate::DeallocateBound<A> for $t<A> {
            #[inline]
            fn deallocate_bound<Cx: $crate::MessageBindingMut<A>>(self, common: &Cx) {
                unsafe { $crate::DeallocateIn::deallocate_in(self, common.alloc()) }
            }
        }

        impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> $crate::CloneBound<A>
            for $t<A>
        {
            #[inline]
            fn clone_bound<Cx: $crate::MessageBindingMut<A>>(
                &self,
                _common: &Cx,
                alloc: A,
            ) -> Self {
                $crate::CloneIn::clone_in(self, alloc)
            }
        }
    };
}

impl<A: Allocator> DeallocateBound<A> for UnmanagedString<A> {
    #[inline]
    fn deallocate_bound<Cx: MessageBindingMut<A>>(self, common: &Cx) {
        unsafe { DeallocateIn::deallocate_in(self, common.alloc()) }
    }
}

impl<A: Allocator + Clone> CloneBound<A> for UnmanagedString<A> {
    #[inline]
    fn clone_bound<Cx: MessageBindingMut<A>>(&self, _common: &Cx, alloc: A) -> Self {
        CloneIn::clone_in(self, alloc)
    }
}

impl<T, A> DeallocateBound<A> for UnmanagedVec<T, A>
where
    T: DeallocateIn<A>,
    A: Allocator,
{
    #[inline]
    fn deallocate_bound<Cx: MessageBindingMut<A>>(self, common: &Cx) {
        unsafe { DeallocateIn::deallocate_in(self, common.alloc()) }
    }
}

impl<T, A> CloneBound<A> for UnmanagedVec<T, A>
where
    T: CloneIn<A>,
    A: Allocator + Clone,
{
    #[inline]
    fn clone_bound<Cx: MessageBindingMut<A>>(&self, _common: &Cx, alloc: A) -> Self {
        CloneIn::clone_in(self, alloc)
    }
}

impl<M, A> DeallocateBound<A> for UnmanagedBox<M, A>
where
    M: DeallocateIn<A>,
    A: Allocator,
{
    #[inline]
    fn deallocate_bound<Cx: MessageBindingMut<A>>(self, common: &Cx) {
        unsafe { DeallocateIn::deallocate_in(self, common.alloc()) }
    }
}

impl<M, A> CloneBound<A> for UnmanagedBox<M, A>
where
    M: CloneIn<A>,
    A: Allocator + Clone,
{
    #[inline]
    fn clone_bound<Cx: MessageBindingMut<A>>(&self, _common: &Cx, alloc: A) -> Self {
        CloneIn::clone_in(self, alloc)
    }
}
