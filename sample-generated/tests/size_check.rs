use ::allocator_api2::alloc::{AllocError, Allocator, Global};
use ::core::alloc::Layout;
use ::core::mem;
use ::core::ptr::NonNull;

use ::puroro_sample_generated::{Point, School, Student, Task};

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
fn allocator_is_stored_per_inlined_message() {
    let global = mem::size_of::<Task<Global>>();
    let padded = mem::size_of::<Task<Padded>>();
    let delta = padded - global;
    // MessageCommon stores one `A` per message. Map fields also own a
    // HashMap-embedded `A`. Inlined `origin` is a full `Point`. The oneof
    // `postal` variant is an inlined `Address`, so the enum slot also embeds
    // `A`. Expect: parent + map + origin + postal ≈ 4× sizeof(Padded).
    assert_eq!(
        delta,
        4 * mem::size_of::<Padded>(),
        "Task<Padded> grew by {delta} bytes over Task<Global>; allocator copies exceeded MessageCommon + map + inlined origin + oneof postal"
    );
    assert!(
        mem::size_of::<Point<Padded>>() > mem::size_of::<Point<Global>>(),
        "owned Point keeps MessageCommon and therefore embeds A"
    );
}

#[test]
fn school_inlined_chain_stores_allocator_per_message() {
    let global = mem::size_of::<School<Global>>();
    let padded = mem::size_of::<School<Padded>>();
    let delta = padded - global;
    // School + inlined Student + inlined Point + inlined Address (home).
    // Each has its own MessageCommon `A`.
    assert_eq!(
        delta,
        4 * mem::size_of::<Padded>(),
        "School<Padded> grew by {delta} bytes; expected one allocator per inlined message on the chain"
    );
    assert!(
        mem::size_of::<Student<Padded>>() > mem::size_of::<Student<Global>>(),
        "owned Student keeps MessageCommon and therefore embeds A"
    );
}
