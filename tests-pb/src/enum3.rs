//!Generated from package "enum3"
pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}
pub mod _puroro {
    pub use ::puroro::*;
}
pub struct Msg {
    enum_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        (),
        self::_puroro::tags::Enum3<()>,
        0usize,
    >,
    enum_unlabeled: self::_puroro::internal::field_type::SingularNumericalField::<
        (),
        self::_puroro::tags::Enum3<()>,
    >,
    enum_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        (),
        self::_puroro::tags::Enum3<()>,
    >,
    _bitfield: self::_puroro::bitvec::BitArray<1usize>,
}
