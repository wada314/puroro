use ::allocator_api2::alloc::{AllocError, Allocator, Global};
use ::core::alloc::Layout;
use ::core::mem;
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
    let global = mem::size_of::<Task<Global>>();
    let padded = mem::size_of::<Task<Padded>>();
    let delta = padded - global;
    // MessageCommon stores one `A`. Map fields also own a HashMap-embedded `A`
    // (by design — maps are uncommon). An **inlined** nested message embeds its
    // own `MessageCommon` (and therefore another `A`) — not a field-wrapper leak.
    // Expect: parent common + map + inlined `Point` ≈ 3× sizeof(Padded).
    assert!(
        delta <= 192 + 16,
        "Task<Padded> grew by {delta} bytes over Task<Global>; allocator appears duplicated beyond MessageCommon + map + inlined origin"
    );
}
