//!Generated from package "self_recursive"
pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}
pub mod _puroro {
    pub use ::puroro::*;
}
pub struct Msg {
    recursive_unlabeled: self::_puroro::internal::field_type::SingularHeapMessageField::<
        (),
    >,
    _bitfield: self::_puroro::bitvec::BitArray<0usize>,
}
