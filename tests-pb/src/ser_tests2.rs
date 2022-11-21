pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}
pub mod _puroro {
    pub use ::puroro::*;
}
pub struct Msg {
    i32_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        i32,
        self::_puroro::tags::Int32,
        0usize,
    >,
    i32_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        i32,
        self::_puroro::tags::Int32,
    >,
    float_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        f32,
        self::_puroro::tags::Float,
        1usize,
    >,
    float_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        f32,
        self::_puroro::tags::Float,
    >,
    string_optional: self::_puroro::internal::field_type::OptionalStringField::<2usize>,
    string_repeated: self::_puroro::internal::field_type::RepeatedStringField,
    submsg_optional: self::_puroro::internal::field_type::SingularHeapMessageField::<()>,
    submsg_repeated: self::_puroro::internal::field_type::RepeatedMessageField::<()>,
    enum_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        self::_puroro_root::ser_tests2::Enum,
        self::_puroro::tags::Enum2<()>,
        3usize,
    >,
    enum_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        self::_puroro_root::ser_tests2::Enum,
        self::_puroro::tags::Enum2<()>,
    >,
    very_large_field_number: self::_puroro::internal::field_type::OptionalNumericalField::<
        i32,
        self::_puroro::tags::Int32,
        4usize,
    >,
    _bitfield: self::_puroro::bitvec::BitArray<1usize>,
}
