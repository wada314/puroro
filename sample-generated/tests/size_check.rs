use ::allocator_api2::alloc::{AllocError, Allocator, Global};
use ::core::alloc::Layout;
use ::core::ptr::NonNull;

use ::puroro_sample_generated::Task;

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
    let global = ::core::mem::size_of::<Task<Global>>();
    let padded = ::core::mem::size_of::<Task<Padded>>();
    let delta = padded - global;
    // With the single-allocator design the fat allocator is stored exactly once
    // (in `MessageCommon.alloc`). If each heap field embedded its own copy the
    // delta would be a multiple of 64 (there are 6+ heap-backed fields).
    assert!(
        delta <= 64 + 8,
        "Task<Padded> grew by {delta} bytes over Task<Global>; allocator appears duplicated"
    );
}
