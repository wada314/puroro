pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}
pub mod _puroro {
    pub use ::puroro::*;
}
pub struct Submsg {
    i32_unlabeled: self::_puroro::internal::field_type::SingularNumericalField::<
        i32,
        self::_puroro::tags::Int32,
    >,
    i32_optional: self::_puroro::internal::field_type::SingularNumericalField::<
        i32,
        self::_puroro::tags::Int32,
    >,
    i64_unlabeled: self::_puroro::internal::field_type::SingularNumericalField::<
        i64,
        self::_puroro::tags::Int64,
    >,
    _bitfield: self::_puroro::bitvec::BitArray<0usize>,
}
impl Submsg {}
