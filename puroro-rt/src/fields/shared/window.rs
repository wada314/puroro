//! Parent-common window for inlined (and later boxed) nested-message views.
//!
//! [`Window`] / [`WindowMut`] erase the parent's bit-array type so
//! [`EncodeType::View`](crate::fields::wire::encode_type::EncodeType::View)
//! stays `View<'a, A>`. Bits are addressed at `bit_base + local`. Unknowns are
//! the resolved child store (or the owned message's own store).

use ::allocator_api2::alloc::{Allocator, Global};
use ::bitvec::{
    order::Lsb0,
    ptr::{BitRef, Mut},
};

use super::{
    MessageBinding, MessageBindingMut, MessageCommon, MessageCommonAlloc, MessageCommonBits,
};
use crate::unknown_fields::UnknownFields;

/// Empty store used when an inlined child has no unknown-field entry yet.
///
/// The wrapper is `Sync` only because this instance is always `None` and is
/// never written.
struct EmptyUnknown(UnknownFields<Global>);

// SAFETY: `EmptyUnknown` is a never-written empty (`None`) store.
unsafe impl Sync for EmptyUnknown {}

static EMPTY_UNKNOWN: EmptyUnknown = EmptyUnknown(UnknownFields::new());

fn empty_unknown_fields<'a, A: Allocator>() -> &'a UnknownFields<A> {
    // SAFETY: empty store is `None`; the allocator parameter is unused.
    unsafe { &*(&EMPTY_UNKNOWN.0 as *const UnknownFields<Global>).cast::<UnknownFields<A>>() }
}

/// Shared view onto a parent [`MessageCommon`] (owned or inlined child path).
pub struct Window<'a, A: Allocator> {
    bits: &'a dyn MessageCommonBits,
    bit_base: usize,
    alloc: &'a A,
    unknowns: Option<&'a UnknownFields<A>>,
}

impl<A: Allocator> Copy for Window<'_, A> {}

impl<A: Allocator> Clone for Window<'_, A> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, A: Allocator> Window<'a, A> {
    /// Window onto `common` itself (`bit_base = 0`, this message's unknowns).
    #[inline]
    pub fn for_owned<B>(common: &'a MessageCommon<B, A>) -> Self
    where
        MessageCommon<B, A>: MessageCommonBits,
    {
        Self {
            bits: common,
            bit_base: 0,
            alloc: &common.alloc,
            unknowns: Some(&common.unknown_fields),
        }
    }

    /// One-level inlined child: bits at `bit_base`, unknowns at `field`.
    #[inline]
    pub fn for_child<B>(common: &'a MessageCommon<B, A>, bit_base: usize, field: u32) -> Self
    where
        MessageCommon<B, A>: MessageCommonBits,
    {
        Self {
            bits: common,
            bit_base,
            alloc: &common.alloc,
            unknowns: common.unknown_fields.child(field),
        }
    }

    /// Window from any [`MessageBinding`] (owned or already-nested).
    #[inline]
    pub fn from_binding<C: MessageBinding<A> + MessageCommonBits>(
        common: &'a C,
        bit_base: usize,
        field: u32,
    ) -> Self {
        Self {
            bits: common,
            bit_base,
            alloc: common.alloc(),
            unknowns: common.unknown_fields().child(field),
        }
    }

    /// Bit index added to every child-local bit.
    #[inline]
    pub fn bit_base(&self) -> usize {
        self.bit_base
    }

    /// One more inlined-child hop (`bit_base` added, unknowns at `field`).
    #[inline]
    pub fn nest(&self, bit_base: usize, field: u32) -> Window<'a, A> {
        Window {
            bits: self.bits,
            bit_base: self.bit_base + bit_base,
            alloc: self.alloc,
            unknowns: self.unknowns.and_then(|fields| fields.child(field)),
        }
    }
}

impl<A: Allocator> MessageCommonAlloc for Window<'_, A> {
    type Alloc = A;

    #[inline]
    fn alloc(&self) -> &A {
        self.alloc
    }
}

impl<A: Allocator> MessageCommonBits for Window<'_, A> {
    #[inline]
    fn is_bit_set(&self, bit: usize) -> bool {
        self.bits.is_bit_set(self.bit_base + bit)
    }

    fn set_bit(&mut self, _bit: usize, _value: bool) {
        unreachable!("shared Window is read-only");
    }

    fn bit_mut(&mut self, _bit: usize) -> BitRef<'_, Mut, u8, Lsb0> {
        unreachable!("shared Window is read-only");
    }
}

impl<A: Allocator> MessageBinding<A> for Window<'_, A> {
    #[inline]
    fn unknown_fields(&self) -> &UnknownFields<A> {
        match self.unknowns {
            Some(fields) => fields,
            None => empty_unknown_fields(),
        }
    }

    /// Compose onto this window (`bit_base` added, unknowns at `field`).
    #[inline]
    fn child_window(&self, bit_base: usize, field: u32) -> Window<'_, A> {
        self.nest(bit_base, field)
    }
}

impl<A: Allocator> MessageBindingMut<A> for Window<'_, A> {
    fn unknown_fields_mut(&mut self) -> &mut UnknownFields<A> {
        unreachable!("shared Window is read-only");
    }
}

impl<A: Allocator> super::InlinedMessageParent<A> for Window<'_, A> {
    fn child_window_mut(&mut self, _bit_base: usize, _field: u32) -> WindowMut<'_, A>
    where
        A: Clone,
    {
        unreachable!("shared Window is read-only");
    }
}

/// Mutable view onto a parent [`MessageCommon`].
pub struct WindowMut<'a, A: Allocator> {
    bits: &'a mut dyn MessageCommonBits,
    bit_base: usize,
    alloc: &'a A,
    unknowns: &'a mut UnknownFields<A>,
}

impl<'a, A: Allocator> WindowMut<'a, A> {
    /// Window onto `common` itself (`bit_base = 0`, this message's unknowns).
    #[inline]
    pub fn for_owned<B>(common: &'a mut MessageCommon<B, A>) -> Self
    where
        B: MessageCommonBits,
    {
        let MessageCommon {
            bits,
            unknown_fields,
            alloc,
        } = common;
        Self {
            bits,
            bit_base: 0,
            alloc,
            unknowns: unknown_fields,
        }
    }

    /// One-level inlined child: bits at `bit_base`, unknowns at `child_mut(field)`.
    #[inline]
    pub fn for_child<B>(common: &'a mut MessageCommon<B, A>, bit_base: usize, field: u32) -> Self
    where
        A: Clone,
        B: MessageCommonBits,
    {
        let alloc_clone = common.alloc.clone();
        let MessageCommon {
            bits,
            unknown_fields,
            alloc,
        } = common;
        Self {
            bits,
            bit_base,
            alloc,
            unknowns: unknown_fields.child_mut(field, alloc_clone),
        }
    }

    /// Bit index added to every child-local bit.
    #[inline]
    pub fn bit_base(&self) -> usize {
        self.bit_base
    }
}

impl<A: Allocator> MessageCommonAlloc for WindowMut<'_, A> {
    type Alloc = A;

    #[inline]
    fn alloc(&self) -> &A {
        self.alloc
    }
}

impl<A: Allocator> MessageCommonBits for WindowMut<'_, A> {
    #[inline]
    fn is_bit_set(&self, bit: usize) -> bool {
        self.bits.is_bit_set(self.bit_base + bit)
    }

    #[inline]
    fn set_bit(&mut self, bit: usize, value: bool) {
        self.bits.set_bit(self.bit_base + bit, value);
    }

    #[inline]
    fn bit_mut(&mut self, bit: usize) -> BitRef<'_, Mut, u8, Lsb0> {
        self.bits.bit_mut(self.bit_base + bit)
    }
}

impl<A: Allocator> MessageBinding<A> for WindowMut<'_, A> {
    #[inline]
    fn unknown_fields(&self) -> &UnknownFields<A> {
        self.unknowns
    }
}

impl<A: Allocator> MessageBindingMut<A> for WindowMut<'_, A> {
    #[inline]
    fn unknown_fields_mut(&mut self) -> &mut UnknownFields<A> {
        self.unknowns
    }
}

impl<A: Allocator> super::InlinedMessageParent<A> for WindowMut<'_, A> {
    #[inline]
    fn child_window_mut(&mut self, bit_base: usize, field: u32) -> WindowMut<'_, A>
    where
        A: Clone,
    {
        let alloc_clone = self.alloc.clone();
        WindowMut {
            bits: self.bits,
            bit_base: self.bit_base + bit_base,
            alloc: self.alloc,
            unknowns: self.unknowns.child_mut(field, alloc_clone),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::bitvec::array::BitArray;
    use ::bitvec::order::Lsb0;

    #[test]
    fn bit_base_offsets_into_parent_array() {
        let mut common =
            MessageCommon::<BitArray<[u8; 1], Lsb0>, Global>::new_in(BitArray::ZERO, Global);
        common.set_bit(3, true);
        common.set_bit(4, false);

        let window = Window::for_child(&common, 3, 1);
        assert_eq!(window.bit_base(), 3);
        assert!(window.is_bit_set(0));
        assert!(!window.is_bit_set(1));

        {
            let mut window = WindowMut::for_child(&mut common, 3, 1);
            window.set_bit(1, true);
        }
        assert!(common.is_bit_set(4));
    }

    #[test]
    fn nest_adds_bit_base() {
        let mut common =
            MessageCommon::<BitArray<[u8; 1], Lsb0>, Global>::new_in(BitArray::ZERO, Global);
        common.set_bit(5, true);
        let child = Window::for_child(&common, 3, 1);
        let nested = child.child_window(2, 2);
        assert_eq!(nested.bit_base(), 5);
        assert!(nested.is_bit_set(0));
    }
}
