//! Untagged 3-word lazy LEN slot: Wire | Inline | Heap | Failed.
//!
//! The live arm is stored as two bits in [`MessageCommon`](crate::MessageCommon),
//! not as a Rust enum tag on the slot. Promotion from Wire to Inline / Heap
//! (or Failed) happens on `&self`.

use ::allocator_api2::alloc::Allocator;
use ::core::cell::UnsafeCell;
use ::core::mem::{self, ManuallyDrop};
use ::core::str;
use ::puroro::DecodeError;
use ::unmanaged::{DefaultIn, String as AllocString, UnmanagedString, UnmanagedVec};

use super::len::{BytesLikeLenCodec, LenScalar, ProtoString};
use super::sso_buf::{INLINE_CAP, InlineBuf, SsoHeap};
use crate::decode::WireSpan;
use crate::fields::shared::slot_init::SlotInitMut;
use crate::fields::shared::value_layout::{ValueLayout, ValueLayoutClone};
use crate::fields::shared::value_slot::{ValueSlot, ValueSlotMutAccess};
use crate::fields::shared::{MessageBindingMut, MessageCommonBits, MessageCommonSharedBits};

/// Two-bit kind stored at [`WireOrSso`]’s `KIND` bit and the next bit (lazy messages only).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WireOrSsoKind {
    Wire = 0,
    Inline = 1,
    Heap = 2,
    Failed = 3,
}

impl WireOrSsoKind {
    pub fn from_bits(arm0: bool, arm1: bool) -> Self {
        match (arm1 as u8) << 1 | (arm0 as u8) {
            0 => Self::Wire,
            1 => Self::Inline,
            2 => Self::Heap,
            _ => Self::Failed,
        }
    }

    pub fn bits(self) -> (bool, bool) {
        let v = self as u8;
        (v & 1 != 0, v & 2 != 0)
    }
}

#[repr(C)]
union WireOrSsoUnion<H> {
    wire: WireSpan,
    inline: InlineBuf,
    heap: ManuallyDrop<H>,
}

const _: () = assert!(mem::size_of::<WireOrSsoUnion<()>>() == mem::size_of::<usize>() * 3);

/// Interior-mutable lazy LEN slot (same width as [`SsoBuf`](super::sso_buf::SsoBuf)).
pub struct WireOrSsoSlot<H> {
    inner: UnsafeCell<WireOrSsoUnion<H>>,
}

impl<H> WireOrSsoSlot<H> {
    pub fn empty() -> Self {
        Self {
            inner: UnsafeCell::new(WireOrSsoUnion {
                wire: WireSpan { offset: 0, len: 0 },
            }),
        }
    }

    fn store_wire_inner<A: Allocator>(&mut self, span: WireSpan, old_arm: WireOrSsoKind, alloc: &A)
    where
        H: SsoHeap<A>,
    {
        self.deallocate_inner(old_arm, alloc);
        unsafe {
            *self.inner.get() = WireOrSsoUnion { wire: span };
        }
    }

    fn deallocate_inner<A: Allocator>(&mut self, arm: WireOrSsoKind, alloc: &A)
    where
        H: SsoHeap<A>,
    {
        if arm != WireOrSsoKind::Heap {
            return;
        }
        unsafe {
            let heap = ManuallyDrop::take(&mut (*self.inner.get()).heap);
            heap.deallocate_heap(alloc);
            *self.inner.get() = WireOrSsoUnion {
                wire: WireSpan { offset: 0, len: 0 },
            };
        }
    }

    pub(crate) fn is_empty<A: Allocator>(&self, arm: WireOrSsoKind) -> bool
    where
        H: SsoHeap<A>,
    {
        match arm {
            WireOrSsoKind::Wire | WireOrSsoKind::Failed => unsafe {
                (*self.inner.get()).wire.len == 0
            },
            WireOrSsoKind::Inline => unsafe { (*self.inner.get()).inline.as_bytes().is_empty() },
            WireOrSsoKind::Heap => unsafe { (*self.inner.get()).heap.as_slice().is_empty() },
        }
    }

    fn clone_in_inner<A: Allocator + Clone>(&self, arm: WireOrSsoKind, alloc: A) -> Self
    where
        H: SsoHeap<A>,
    {
        match arm {
            WireOrSsoKind::Wire | WireOrSsoKind::Failed => {
                let wire = unsafe { (*self.inner.get()).wire };
                Self {
                    inner: UnsafeCell::new(WireOrSsoUnion { wire }),
                }
            }
            WireOrSsoKind::Inline => {
                let inline = unsafe { (*self.inner.get()).inline };
                Self {
                    inner: UnsafeCell::new(WireOrSsoUnion { inline }),
                }
            }
            WireOrSsoKind::Heap => {
                let heap = unsafe { (*self.inner.get()).heap.clone_heap(alloc) };
                Self {
                    inner: UnsafeCell::new(WireOrSsoUnion {
                        heap: ManuallyDrop::new(heap),
                    }),
                }
            }
        }
    }
}

impl<A: Allocator> WireOrSsoSlot<UnmanagedString<A>> {
    pub fn store_wire(&mut self, span: WireSpan, old_arm: WireOrSsoKind, alloc: &A) {
        self.store_wire_inner(span, old_arm, alloc);
    }

    pub fn deallocate(&mut self, arm: WireOrSsoKind, alloc: &A) {
        self.deallocate_inner(arm, alloc);
    }

    pub fn clone_in(&self, arm: WireOrSsoKind, alloc: A) -> Self
    where
        A: Clone,
    {
        self.clone_in_inner(arm, alloc)
    }

    /// Decode / promote a UTF-8 string. Failed is sticky.
    pub fn get_str<'a>(
        &'a self,
        arm: WireOrSsoKind,
        wire: &'a [u8],
        alloc: A,
        set_arm: impl FnOnce(WireOrSsoKind),
    ) -> Result<&'a str, DecodeError>
    where
        A: Clone,
    {
        match arm {
            WireOrSsoKind::Failed => Err(DecodeError::InvalidUtf8),
            WireOrSsoKind::Inline => Ok(unsafe { inline_str(&*self.inner.get()) }),
            WireOrSsoKind::Heap => Ok(unsafe { heap_str(&*self.inner.get()) }),
            WireOrSsoKind::Wire => {
                let span = unsafe { (*self.inner.get()).wire };
                let bytes = span.slice(wire)?;
                let Ok(s) = str::from_utf8(bytes) else {
                    set_arm(WireOrSsoKind::Failed);
                    return Err(DecodeError::InvalidUtf8);
                };
                if s.len() <= INLINE_CAP {
                    unsafe {
                        *self.inner.get() = WireOrSsoUnion {
                            inline: InlineBuf::from_bytes(s.as_bytes()),
                        };
                    }
                    set_arm(WireOrSsoKind::Inline);
                    Ok(unsafe { inline_str(&*self.inner.get()) })
                } else {
                    let heap = UnmanagedString::from_string(AllocString::from_str_in(s, alloc));
                    unsafe {
                        *self.inner.get() = WireOrSsoUnion {
                            heap: ManuallyDrop::new(heap),
                        };
                    }
                    set_arm(WireOrSsoKind::Heap);
                    Ok(unsafe { heap_str(&*self.inner.get()) })
                }
            }
        }
    }
}

impl<A: Allocator> WireOrSsoSlot<UnmanagedVec<u8, A>> {
    pub fn store_wire(&mut self, span: WireSpan, old_arm: WireOrSsoKind, alloc: &A) {
        self.store_wire_inner(span, old_arm, alloc);
    }

    pub fn deallocate(&mut self, arm: WireOrSsoKind, alloc: &A) {
        self.deallocate_inner(arm, alloc);
    }

    pub fn clone_in(&self, arm: WireOrSsoKind, alloc: A) -> Self
    where
        A: Clone,
    {
        self.clone_in_inner(arm, alloc)
    }

    /// Copy a bytes payload into Inline / Heap. Always succeeds.
    pub fn get_bytes<'a>(
        &'a self,
        arm: WireOrSsoKind,
        wire: &'a [u8],
        alloc: A,
        set_arm: impl FnOnce(WireOrSsoKind),
    ) -> Result<&'a [u8], DecodeError>
    where
        A: Clone,
    {
        match arm {
            WireOrSsoKind::Failed => Err(DecodeError::InvalidUtf8),
            WireOrSsoKind::Inline => Ok(unsafe { (*self.inner.get()).inline.as_bytes() }),
            WireOrSsoKind::Heap => Ok(unsafe { (*(*self.inner.get()).heap).as_slice() }),
            WireOrSsoKind::Wire => {
                let span = unsafe { (*self.inner.get()).wire };
                let bytes = span.slice(wire)?;
                if bytes.len() <= INLINE_CAP {
                    unsafe {
                        *self.inner.get() = WireOrSsoUnion {
                            inline: InlineBuf::from_bytes(bytes),
                        };
                    }
                    set_arm(WireOrSsoKind::Inline);
                    Ok(unsafe { (*self.inner.get()).inline.as_bytes() })
                } else {
                    let heap = <UnmanagedVec<u8, A> as SsoHeap<A>>::from_slice(bytes, alloc);
                    unsafe {
                        *self.inner.get() = WireOrSsoUnion {
                            heap: ManuallyDrop::new(heap),
                        };
                    }
                    set_arm(WireOrSsoKind::Heap);
                    Ok(unsafe { (*self.inner.get()).heap.as_slice() })
                }
            }
        }
    }
}

impl<A: Allocator> DefaultIn<A> for WireOrSsoSlot<UnmanagedString<A>> {
    fn default_in(_alloc: A) -> Self {
        Self::empty()
    }
}

impl<A: Allocator> DefaultIn<A> for WireOrSsoSlot<UnmanagedVec<u8, A>> {
    fn default_in(_alloc: A) -> Self {
        Self::empty()
    }
}

/// [`ValueLayout::Slot`] store used by [`WireOrSso`].
pub trait WireOrSsoStore<A: Allocator> {
    fn store_wire(&mut self, span: WireSpan, old_arm: WireOrSsoKind, alloc: &A);
}

impl<A: Allocator> WireOrSsoStore<A> for WireOrSsoSlot<UnmanagedString<A>> {
    fn store_wire(&mut self, span: WireSpan, old_arm: WireOrSsoKind, alloc: &A) {
        self.store_wire_inner(span, old_arm, alloc);
    }
}

impl<A: Allocator> WireOrSsoStore<A> for WireOrSsoSlot<UnmanagedVec<u8, A>> {
    fn store_wire(&mut self, span: WireSpan, old_arm: WireOrSsoKind, alloc: &A) {
        self.store_wire_inner(span, old_arm, alloc);
    }
}

/// Singular `string` / `bytes` layout: [`WireOrSsoSlot`] + a 2-bit kind at `KIND`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct WireOrSso<const KIND: usize>;

impl<const KIND: usize> WireOrSso<KIND> {
    pub fn kind<Cx: MessageCommonBits>(common: &Cx) -> WireOrSsoKind {
        WireOrSsoKind::from_bits(common.is_bit_set(KIND), common.is_bit_set(KIND + 1))
    }

    pub fn set_kind<Cx: MessageCommonBits>(common: &mut Cx, kind: WireOrSsoKind) {
        let (b0, b1) = kind.bits();
        common.set_bit(KIND, b0);
        common.set_bit(KIND + 1, b1);
    }

    pub fn set_kind_shared<Cx: MessageCommonSharedBits>(common: &Cx, kind: WireOrSsoKind) {
        let (b0, b1) = kind.bits();
        common.set_bit_shared(KIND, b0);
        common.set_bit_shared(KIND + 1, b1);
    }
}

impl<A: Allocator, const KIND: usize> ValueLayout<ProtoString, A> for WireOrSso<KIND> {
    type Slot = WireOrSsoSlot<UnmanagedString<A>>;

    fn is_proto_empty<Cx>(slot: &Self::Slot, common: &Cx) -> bool
    where
        Cx: MessageBindingMut<A>,
    {
        slot.is_empty::<A>(Self::kind(common))
    }

    fn get<'a, Cx>(slot: &'a Self::Slot, common: &'a Cx) -> &'a str
    where
        Cx: MessageBindingMut<A>,
    {
        match Self::kind(common) {
            WireOrSsoKind::Inline => unsafe { inline_str(&*slot.inner.get()) },
            WireOrSsoKind::Heap => unsafe { heap_str(&*slot.inner.get()) },
            WireOrSsoKind::Wire | WireOrSsoKind::Failed => "",
        }
    }

    fn clear<VS, I, Cx>(slot: &mut VS, init: I, common: &mut Cx)
    where
        VS: ValueSlot<Self::Slot, A>,
        I: SlotInitMut,
        Cx: MessageBindingMut<A>,
        A: Clone,
        Self::Slot: DefaultIn<A>,
    {
        if !init.is_initialized(|b| common.is_bit_set(b)) {
            Self::set_kind(common, WireOrSsoKind::Wire);
            return;
        }
        let old = Self::kind(common);
        let alloc = common.clone_alloc();
        {
            let s = ValueSlot::with_mut(slot, init, common).get_mut();
            s.deallocate(old, &alloc);
        }
        Self::set_kind(common, WireOrSsoKind::Wire);
        init.set_initialized(|b, v| common.set_bit(b, v), false);
    }

    fn deallocate_slot<VS, Cx>(slot: VS, initialized: bool, common: &Cx)
    where
        VS: ValueSlot<Self::Slot, A>,
        Cx: MessageBindingMut<A>,
    {
        if let Some(mut s) = slot.take_value(initialized) {
            s.deallocate(Self::kind(common), common.alloc());
        }
    }
}

impl<A: Allocator + Clone, const KIND: usize> ValueLayoutClone<ProtoString, A> for WireOrSso<KIND> {
    fn clone_slot<VS, Cx>(slot: &VS, initialized: bool, common: &Cx, alloc: A) -> VS
    where
        VS: ValueSlot<Self::Slot, A>,
        Cx: MessageBindingMut<A>,
    {
        let arm = Self::kind(common);
        VS::from_optional(slot.get_value(initialized).map(|s| s.clone_in(arm, alloc)))
    }
}

impl<A: Allocator, const KIND: usize, C: BytesLikeLenCodec> ValueLayout<LenScalar<C>, A>
    for WireOrSso<KIND>
{
    type Slot = WireOrSsoSlot<UnmanagedVec<u8, A>>;

    fn is_proto_empty<Cx>(slot: &Self::Slot, common: &Cx) -> bool
    where
        Cx: MessageBindingMut<A>,
    {
        slot.is_empty::<A>(Self::kind(common))
    }

    fn get<'a, Cx>(slot: &'a Self::Slot, common: &'a Cx) -> &'a [u8]
    where
        Cx: MessageBindingMut<A>,
    {
        match Self::kind(common) {
            WireOrSsoKind::Inline => unsafe { (*slot.inner.get()).inline.as_bytes() },
            WireOrSsoKind::Heap => unsafe { (*(*slot.inner.get()).heap).as_slice() },
            WireOrSsoKind::Wire | WireOrSsoKind::Failed => &[],
        }
    }

    fn clear<VS, I, Cx>(slot: &mut VS, init: I, common: &mut Cx)
    where
        VS: ValueSlot<Self::Slot, A>,
        I: SlotInitMut,
        Cx: MessageBindingMut<A>,
        A: Clone,
        Self::Slot: DefaultIn<A>,
    {
        if !init.is_initialized(|b| common.is_bit_set(b)) {
            Self::set_kind(common, WireOrSsoKind::Wire);
            return;
        }
        let old = Self::kind(common);
        let alloc = common.clone_alloc();
        {
            let s = ValueSlot::with_mut(slot, init, common).get_mut();
            s.deallocate(old, &alloc);
        }
        Self::set_kind(common, WireOrSsoKind::Wire);
        init.set_initialized(|b, v| common.set_bit(b, v), false);
    }

    fn deallocate_slot<VS, Cx>(slot: VS, initialized: bool, common: &Cx)
    where
        VS: ValueSlot<Self::Slot, A>,
        Cx: MessageBindingMut<A>,
    {
        if let Some(mut s) = slot.take_value(initialized) {
            s.deallocate(Self::kind(common), common.alloc());
        }
    }
}

impl<A: Allocator + Clone, const KIND: usize, C: BytesLikeLenCodec>
    ValueLayoutClone<LenScalar<C>, A> for WireOrSso<KIND>
{
    fn clone_slot<VS, Cx>(slot: &VS, initialized: bool, common: &Cx, alloc: A) -> VS
    where
        VS: ValueSlot<Self::Slot, A>,
        Cx: MessageBindingMut<A>,
    {
        let arm = Self::kind(common);
        VS::from_optional(slot.get_value(initialized).map(|s| s.clone_in(arm, alloc)))
    }
}

unsafe fn inline_str<H>(slot: &WireOrSsoUnion<H>) -> &str {
    unsafe { str::from_utf8_unchecked(slot.inline.as_bytes()) }
}

unsafe fn heap_str<A: Allocator>(slot: &WireOrSsoUnion<UnmanagedString<A>>) -> &str {
    let heap: &UnmanagedString<A> = unsafe { &slot.heap };
    heap
}

/// `WireOrSsoSlot` for singular `string`.
pub type LazyStringSlot<A> = WireOrSsoSlot<UnmanagedString<A>>;
/// `WireOrSsoSlot` for singular `bytes`.
pub type LazyBytesSlot<A> = WireOrSsoSlot<UnmanagedVec<u8, A>>;

#[cfg(test)]
mod tests {
    use super::*;
    use ::allocator_api2::alloc::Global;
    use ::core::cell::Cell;

    fn set_arm_cell(cell: &Cell<WireOrSsoKind>) -> impl FnOnce(WireOrSsoKind) + '_ {
        |arm| cell.set(arm)
    }

    #[test]
    fn promote_inline_ignores_later_wire_mutation() {
        let wire = b"hello".to_vec();
        let arm = Cell::new(WireOrSsoKind::Wire);
        let mut slot = LazyStringSlot::<Global>::empty();
        slot.store_wire(
            WireSpan {
                offset: 0,
                len: wire.len(),
            },
            WireOrSsoKind::Wire,
            &Global,
        );
        let first = slot
            .get_str(arm.get(), &wire, Global, set_arm_cell(&arm))
            .unwrap();
        assert_eq!(first, "hello");
        assert_eq!(arm.get(), WireOrSsoKind::Inline);

        let mut smashed = wire;
        smashed.fill(0xff);
        let second = slot
            .get_str(arm.get(), &smashed, Global, set_arm_cell(&arm))
            .unwrap();
        assert_eq!(second, "hello");
    }

    #[test]
    fn promote_heap_and_clone() {
        let text = "x".repeat(INLINE_CAP + 4);
        let wire = text.as_bytes().to_vec();
        let arm = Cell::new(WireOrSsoKind::Wire);
        let mut slot = LazyStringSlot::<Global>::empty();
        slot.store_wire(
            WireSpan {
                offset: 0,
                len: wire.len(),
            },
            WireOrSsoKind::Wire,
            &Global,
        );
        assert_eq!(
            slot.get_str(arm.get(), &wire, Global, set_arm_cell(&arm))
                .unwrap(),
            text
        );
        assert_eq!(arm.get(), WireOrSsoKind::Heap);

        let mut cloned = slot.clone_in(arm.get(), Global);
        assert_eq!(
            cloned
                .get_str(arm.get(), b"", Global, set_arm_cell(&arm))
                .unwrap(),
            text
        );
        slot.deallocate(arm.get(), &Global);
        cloned.deallocate(WireOrSsoKind::Heap, &Global);
    }

    #[test]
    fn failed_is_sticky() {
        let wire = [0xff, 0xfe];
        let arm = Cell::new(WireOrSsoKind::Wire);
        let mut slot = LazyStringSlot::<Global>::empty();
        slot.store_wire(WireSpan { offset: 0, len: 2 }, WireOrSsoKind::Wire, &Global);
        assert_eq!(
            slot.get_str(arm.get(), &wire, Global, set_arm_cell(&arm))
                .err(),
            Some(DecodeError::InvalidUtf8)
        );
        assert_eq!(arm.get(), WireOrSsoKind::Failed);
        assert_eq!(
            slot.get_str(arm.get(), b"ok", Global, set_arm_cell(&arm))
                .err(),
            Some(DecodeError::InvalidUtf8)
        );
    }

    #[test]
    fn store_wire_after_heap_does_not_panic() {
        let text = "y".repeat(INLINE_CAP + 1);
        let wire = text.as_bytes().to_vec();
        let arm = Cell::new(WireOrSsoKind::Wire);
        let mut slot = LazyStringSlot::<Global>::empty();
        slot.store_wire(
            WireSpan {
                offset: 0,
                len: wire.len(),
            },
            WireOrSsoKind::Wire,
            &Global,
        );
        slot.get_str(arm.get(), &wire, Global, set_arm_cell(&arm))
            .unwrap();
        slot.store_wire(WireSpan { offset: 0, len: 2 }, arm.get(), &Global);
        arm.set(WireOrSsoKind::Wire);
        assert_eq!(
            slot.get_str(arm.get(), b"hi", Global, set_arm_cell(&arm))
                .unwrap(),
            "hi"
        );
        slot.deallocate(arm.get(), &Global);
    }

    #[test]
    fn bytes_promote_inline() {
        let wire = [1u8, 2, 3];
        let arm = Cell::new(WireOrSsoKind::Wire);
        let mut slot = LazyBytesSlot::<Global>::empty();
        slot.store_wire(WireSpan { offset: 0, len: 3 }, WireOrSsoKind::Wire, &Global);
        assert_eq!(
            slot.get_bytes(arm.get(), &wire, Global, set_arm_cell(&arm))
                .unwrap(),
            &[1, 2, 3]
        );
        assert_eq!(arm.get(), WireOrSsoKind::Inline);
        slot.deallocate(arm.get(), &Global);
    }
}
