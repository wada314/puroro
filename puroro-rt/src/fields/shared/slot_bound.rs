//! Teardown / clone of an **extracted slot value** through a message binding.
//!
//! Distinct from [`FieldDeallocate`](super::FieldDeallocate) / [`FieldCloneIn`](super::FieldCloneIn),
//! which operate on catalog field wrappers (`&mut SingularField`, …) that still
//! live in the parent struct. These traits consume (or borrow) the payload
//! after [`ValueSlot::take_value`](super::value_slot::ValueSlot::take_value).
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
pub trait DeallocateBound<A: Allocator, Cx: MessageBindingMut<A>> {
    /// Releases heap owned by `self`. `common` must be this value's binding.
    fn deallocate_bound(self, common: &Cx);
}

/// Deep-copy an extracted slot value, using the **source** `common` for bits.
pub trait CloneBound<A: Allocator, Cx: MessageBindingMut<A>> {
    /// Copies `self` into `alloc`. Destination bits are installed later.
    fn clone_bound(&self, common: &Cx, alloc: A) -> Self;
}

/// Copy / ZST slots: bits are unused, teardown is a no-op.
macro_rules! impl_primitive_slot_bounds {
    ($($t:ty),+ $(,)?) => {$(
        impl<A: Allocator, Cx: MessageBindingMut<A>> DeallocateBound<A, Cx> for $t {
            #[inline]
            fn deallocate_bound(self, _common: &Cx) {}
        }

        impl<A: Allocator + Clone, Cx: MessageBindingMut<A>> CloneBound<A, Cx> for $t {
            #[inline]
            fn clone_bound(&self, _common: &Cx, _alloc: A) -> Self {
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
        impl<A: ::allocator_api2::alloc::Allocator, Cx: $crate::MessageBindingMut<A>>
            $crate::DeallocateBound<A, Cx> for $t
        {
            #[inline]
            fn deallocate_bound(self, _common: &Cx) {}
        }

        impl<
            A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone,
            Cx: $crate::MessageBindingMut<A>,
        > $crate::CloneBound<A, Cx> for $t
        {
            #[inline]
            fn clone_bound(&self, _common: &Cx, _alloc: A) -> Self {
                *self
            }
        }
    };
}

/// For generated owned messages that already impl [`DeallocateIn`] / [`CloneIn`].
#[macro_export]
macro_rules! impl_owned_slot_bounds {
    ($t:ident) => {
        impl<A, Cx> $crate::DeallocateBound<A, Cx> for $t<A>
        where
            A: ::allocator_api2::alloc::Allocator,
            Cx: $crate::MessageBindingMut<A>,
        {
            #[inline]
            fn deallocate_bound(self, common: &Cx) {
                unsafe { $crate::DeallocateIn::deallocate_in(self, common.alloc()) }
            }
        }

        impl<A, Cx> $crate::CloneBound<A, Cx> for $t<A>
        where
            A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone,
            Cx: $crate::MessageBindingMut<A>,
        {
            #[inline]
            fn clone_bound(&self, _common: &Cx, alloc: A) -> Self {
                $crate::CloneIn::clone_in(self, alloc)
            }
        }
    };
}

impl<A, Cx> DeallocateBound<A, Cx> for UnmanagedString<A>
where
    A: Allocator,
    Cx: MessageBindingMut<A>,
{
    #[inline]
    fn deallocate_bound(self, common: &Cx) {
        unsafe { DeallocateIn::deallocate_in(self, common.alloc()) }
    }
}

impl<A, Cx> CloneBound<A, Cx> for UnmanagedString<A>
where
    A: Allocator + Clone,
    Cx: MessageBindingMut<A>,
{
    #[inline]
    fn clone_bound(&self, _common: &Cx, alloc: A) -> Self {
        CloneIn::clone_in(self, alloc)
    }
}

impl<T, A, Cx> DeallocateBound<A, Cx> for UnmanagedVec<T, A>
where
    T: DeallocateIn<A>,
    A: Allocator,
    Cx: MessageBindingMut<A>,
{
    #[inline]
    fn deallocate_bound(self, common: &Cx) {
        unsafe { DeallocateIn::deallocate_in(self, common.alloc()) }
    }
}

impl<T, A, Cx> CloneBound<A, Cx> for UnmanagedVec<T, A>
where
    T: CloneIn<A>,
    A: Allocator + Clone,
    Cx: MessageBindingMut<A>,
{
    #[inline]
    fn clone_bound(&self, _common: &Cx, alloc: A) -> Self {
        CloneIn::clone_in(self, alloc)
    }
}

impl<M, A, Cx> DeallocateBound<A, Cx> for UnmanagedBox<M, A>
where
    M: DeallocateIn<A>,
    A: Allocator,
    Cx: MessageBindingMut<A>,
{
    #[inline]
    fn deallocate_bound(self, common: &Cx) {
        unsafe { DeallocateIn::deallocate_in(self, common.alloc()) }
    }
}

impl<M, A, Cx> CloneBound<A, Cx> for UnmanagedBox<M, A>
where
    M: CloneIn<A>,
    A: Allocator + Clone,
    Cx: MessageBindingMut<A>,
{
    #[inline]
    fn clone_bound(&self, _common: &Cx, alloc: A) -> Self {
        CloneIn::clone_in(self, alloc)
    }
}
