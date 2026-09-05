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
use ::unmanaged::{String as AllocString, UnmanagedString, UnmanagedVec};

use super::sso_buf::{INLINE_CAP, InlineBuf, SsoHeap};
use crate::decode::WireSpan;

/// Two-bit arm discriminant for [`WireOrSsoSlot`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WireOrSsoArm {
    Wire = 0,
    Inline = 1,
    Heap = 2,
    Failed = 3,
}

impl WireOrSsoArm {
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
union WireOrSso<H> {
    wire: WireSpan,
    inline: InlineBuf,
    heap: ManuallyDrop<H>,
}

const _: () = assert!(mem::size_of::<WireOrSso<()>>() == mem::size_of::<usize>() * 3);

/// Interior-mutable lazy LEN slot (same width as [`SsoBuf`](super::sso_buf::SsoBuf)).
pub struct WireOrSsoSlot<H> {
    inner: UnsafeCell<WireOrSso<H>>,
}

impl<H> WireOrSsoSlot<H> {
    pub fn empty() -> Self {
        Self {
            inner: UnsafeCell::new(WireOrSso {
                wire: WireSpan { offset: 0, len: 0 },
            }),
        }
    }

    fn store_wire_inner<A: Allocator>(&mut self, span: WireSpan, old_arm: WireOrSsoArm, alloc: &A)
    where
        H: SsoHeap<A>,
    {
        self.deallocate_inner(old_arm, alloc);
        unsafe {
            *self.inner.get() = WireOrSso { wire: span };
        }
    }

    fn deallocate_inner<A: Allocator>(&mut self, arm: WireOrSsoArm, alloc: &A)
    where
        H: SsoHeap<A>,
    {
        if arm != WireOrSsoArm::Heap {
            return;
        }
        unsafe {
            let heap = ManuallyDrop::take(&mut (*self.inner.get()).heap);
            heap.deallocate_heap(alloc);
            *self.inner.get() = WireOrSso {
                wire: WireSpan { offset: 0, len: 0 },
            };
        }
    }

    fn clone_in_inner<A: Allocator + Clone>(&self, arm: WireOrSsoArm, alloc: A) -> Self
    where
        H: SsoHeap<A>,
    {
        match arm {
            WireOrSsoArm::Wire | WireOrSsoArm::Failed => {
                let wire = unsafe { (*self.inner.get()).wire };
                Self {
                    inner: UnsafeCell::new(WireOrSso { wire }),
                }
            }
            WireOrSsoArm::Inline => {
                let inline = unsafe { (*self.inner.get()).inline };
                Self {
                    inner: UnsafeCell::new(WireOrSso { inline }),
                }
            }
            WireOrSsoArm::Heap => {
                let heap = unsafe { (*self.inner.get()).heap.clone_heap(alloc) };
                Self {
                    inner: UnsafeCell::new(WireOrSso {
                        heap: ManuallyDrop::new(heap),
                    }),
                }
            }
        }
    }
}

impl<A: Allocator> WireOrSsoSlot<UnmanagedString<A>> {
    pub fn store_wire(&mut self, span: WireSpan, old_arm: WireOrSsoArm, alloc: &A) {
        self.store_wire_inner(span, old_arm, alloc);
    }

    pub fn deallocate(&mut self, arm: WireOrSsoArm, alloc: &A) {
        self.deallocate_inner(arm, alloc);
    }

    pub fn clone_in(&self, arm: WireOrSsoArm, alloc: A) -> Self
    where
        A: Clone,
    {
        self.clone_in_inner(arm, alloc)
    }

    /// Decode / promote a UTF-8 string. Failed is sticky.
    pub fn get_str<'a>(
        &'a self,
        arm: WireOrSsoArm,
        wire: &'a [u8],
        alloc: A,
        set_arm: impl FnOnce(WireOrSsoArm),
    ) -> Result<&'a str, DecodeError>
    where
        A: Clone,
    {
        match arm {
            WireOrSsoArm::Failed => Err(DecodeError::InvalidUtf8),
            WireOrSsoArm::Inline => Ok(unsafe { inline_str(&*self.inner.get()) }),
            WireOrSsoArm::Heap => Ok(unsafe { heap_str(&*self.inner.get()) }),
            WireOrSsoArm::Wire => {
                let span = unsafe { (*self.inner.get()).wire };
                let bytes = span.slice(wire)?;
                let Ok(s) = str::from_utf8(bytes) else {
                    set_arm(WireOrSsoArm::Failed);
                    return Err(DecodeError::InvalidUtf8);
                };
                if s.len() <= INLINE_CAP {
                    unsafe {
                        *self.inner.get() = WireOrSso {
                            inline: InlineBuf::from_bytes(s.as_bytes()),
                        };
                    }
                    set_arm(WireOrSsoArm::Inline);
                    Ok(unsafe { inline_str(&*self.inner.get()) })
                } else {
                    let heap = UnmanagedString::from_string(AllocString::from_str_in(s, alloc));
                    unsafe {
                        *self.inner.get() = WireOrSso {
                            heap: ManuallyDrop::new(heap),
                        };
                    }
                    set_arm(WireOrSsoArm::Heap);
                    Ok(unsafe { heap_str(&*self.inner.get()) })
                }
            }
        }
    }
}

impl<A: Allocator> WireOrSsoSlot<UnmanagedVec<u8, A>> {
    pub fn store_wire(&mut self, span: WireSpan, old_arm: WireOrSsoArm, alloc: &A) {
        self.store_wire_inner(span, old_arm, alloc);
    }

    pub fn deallocate(&mut self, arm: WireOrSsoArm, alloc: &A) {
        self.deallocate_inner(arm, alloc);
    }

    pub fn clone_in(&self, arm: WireOrSsoArm, alloc: A) -> Self
    where
        A: Clone,
    {
        self.clone_in_inner(arm, alloc)
    }

    /// Copy a bytes payload into Inline / Heap. Always succeeds.
    pub fn get_bytes<'a>(
        &'a self,
        arm: WireOrSsoArm,
        wire: &'a [u8],
        alloc: A,
        set_arm: impl FnOnce(WireOrSsoArm),
    ) -> Result<&'a [u8], DecodeError>
    where
        A: Clone,
    {
        match arm {
            WireOrSsoArm::Failed => Err(DecodeError::InvalidUtf8),
            WireOrSsoArm::Inline => Ok(unsafe { (*self.inner.get()).inline.as_bytes() }),
            WireOrSsoArm::Heap => Ok(unsafe { (*(*self.inner.get()).heap).as_slice() }),
            WireOrSsoArm::Wire => {
                let span = unsafe { (*self.inner.get()).wire };
                let bytes = span.slice(wire)?;
                if bytes.len() <= INLINE_CAP {
                    unsafe {
                        *self.inner.get() = WireOrSso {
                            inline: InlineBuf::from_bytes(bytes),
                        };
                    }
                    set_arm(WireOrSsoArm::Inline);
                    Ok(unsafe { (*self.inner.get()).inline.as_bytes() })
                } else {
                    let heap = <UnmanagedVec<u8, A> as SsoHeap<A>>::from_slice(bytes, alloc);
                    unsafe {
                        *self.inner.get() = WireOrSso {
                            heap: ManuallyDrop::new(heap),
                        };
                    }
                    set_arm(WireOrSsoArm::Heap);
                    Ok(unsafe { (*self.inner.get()).heap.as_slice() })
                }
            }
        }
    }
}

unsafe fn inline_str<H>(slot: &WireOrSso<H>) -> &str {
    unsafe { str::from_utf8_unchecked(slot.inline.as_bytes()) }
}

unsafe fn heap_str<A: Allocator>(slot: &WireOrSso<UnmanagedString<A>>) -> &str {
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

    fn set_arm_cell(cell: &Cell<WireOrSsoArm>) -> impl FnOnce(WireOrSsoArm) + '_ {
        |arm| cell.set(arm)
    }

    #[test]
    fn promote_inline_ignores_later_wire_mutation() {
        let wire = b"hello".to_vec();
        let arm = Cell::new(WireOrSsoArm::Wire);
        let mut slot = LazyStringSlot::<Global>::empty();
        slot.store_wire(
            WireSpan {
                offset: 0,
                len: wire.len(),
            },
            WireOrSsoArm::Wire,
            &Global,
        );
        let first = slot
            .get_str(arm.get(), &wire, Global, set_arm_cell(&arm))
            .unwrap();
        assert_eq!(first, "hello");
        assert_eq!(arm.get(), WireOrSsoArm::Inline);

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
        let arm = Cell::new(WireOrSsoArm::Wire);
        let mut slot = LazyStringSlot::<Global>::empty();
        slot.store_wire(
            WireSpan {
                offset: 0,
                len: wire.len(),
            },
            WireOrSsoArm::Wire,
            &Global,
        );
        assert_eq!(
            slot.get_str(arm.get(), &wire, Global, set_arm_cell(&arm))
                .unwrap(),
            text
        );
        assert_eq!(arm.get(), WireOrSsoArm::Heap);

        let mut cloned = slot.clone_in(arm.get(), Global);
        assert_eq!(
            cloned
                .get_str(arm.get(), b"", Global, set_arm_cell(&arm))
                .unwrap(),
            text
        );
        slot.deallocate(arm.get(), &Global);
        cloned.deallocate(WireOrSsoArm::Heap, &Global);
    }

    #[test]
    fn failed_is_sticky() {
        let wire = [0xff, 0xfe];
        let arm = Cell::new(WireOrSsoArm::Wire);
        let mut slot = LazyStringSlot::<Global>::empty();
        slot.store_wire(WireSpan { offset: 0, len: 2 }, WireOrSsoArm::Wire, &Global);
        assert_eq!(
            slot.get_str(arm.get(), &wire, Global, set_arm_cell(&arm))
                .err(),
            Some(DecodeError::InvalidUtf8)
        );
        assert_eq!(arm.get(), WireOrSsoArm::Failed);
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
        let arm = Cell::new(WireOrSsoArm::Wire);
        let mut slot = LazyStringSlot::<Global>::empty();
        slot.store_wire(
            WireSpan {
                offset: 0,
                len: wire.len(),
            },
            WireOrSsoArm::Wire,
            &Global,
        );
        slot.get_str(arm.get(), &wire, Global, set_arm_cell(&arm))
            .unwrap();
        slot.store_wire(WireSpan { offset: 0, len: 2 }, arm.get(), &Global);
        arm.set(WireOrSsoArm::Wire);
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
        let arm = Cell::new(WireOrSsoArm::Wire);
        let mut slot = LazyBytesSlot::<Global>::empty();
        slot.store_wire(WireSpan { offset: 0, len: 3 }, WireOrSsoArm::Wire, &Global);
        assert_eq!(
            slot.get_bytes(arm.get(), &wire, Global, set_arm_cell(&arm))
                .unwrap(),
            &[1, 2, 3]
        );
        assert_eq!(arm.get(), WireOrSsoArm::Inline);
        slot.deallocate(arm.get(), &Global);
    }
}
