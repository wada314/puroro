use ::allocator_api2::alloc::{AllocError, Allocator, Global};
use ::core::alloc::Layout;
use ::core::mem;
use ::core::ptr::NonNull;

use ::puroro_sample_generated::{Point, PointBody, Task};

// A deliberately fat (64-byte), non-ZST allocator that just forwards to Global.
#[derive(Clone)]
struct Padded {
    _pad: [u8; 64],
}

unsafe impl Allocator for Padded {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        Global.allocate(layout)
    }
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        unsafe { Global.deallocate(ptr, layout) }
    }
}

#[test]
fn allocator_is_stored_once() {
    let global = mem::size_of::<Task<Global>>();
    let padded = mem::size_of::<Task<Padded>>();
    let delta = padded - global;
    // MessageCommon stores one `A`. Map fields also own a HashMap-embedded `A`
    // (by design — maps are uncommon). The inlined `origin` slot is `PointBody`
    // (no child `MessageCommon` / extra `A`).
    // Expect: parent common + map ≈ 2× sizeof(Padded).
    assert!(
        delta <= 128 + 16,
        "Task<Padded> grew by {delta} bytes over Task<Global>; allocator appears duplicated beyond MessageCommon + map"
    );
    assert_eq!(
        mem::size_of::<PointBody<Padded>>(),
        mem::size_of::<PointBody<Global>>(),
        "PointBody must not embed an allocator"
    );
    assert!(
        mem::size_of::<PointBody<Padded>>() < mem::size_of::<Point<Padded>>(),
        "owned Point keeps MessageCommon; the inlined slot does not"
    );
}
