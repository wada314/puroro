pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}
pub mod _puroro {
    pub use ::puroro::*;
}
pub struct Msg {
    i32_unlabeled: self::_puroro::internal::field_type::SingularNumericalField::<
        i32,
        self::_puroro::tags::Int32,
    >,
    i32_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        i32,
        self::_puroro::tags::Int32,
    >,
    float_unlabeled: self::_puroro::internal::field_type::SingularNumericalField::<
        f32,
        self::_puroro::tags::Float,
    >,
    float_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        f32,
        self::_puroro::tags::Float,
    >,
    string_unlabeled: self::_puroro::internal::field_type::SingularStringField,
    string_repeated: self::_puroro::internal::field_type::RepeatedStringField,
    submsg_unlabeled: self::_puroro::internal::field_type::SingularHeapMessageField::<
        (),
    >,
    submsg_repeated: self::_puroro::internal::field_type::RepeatedMessageField::<()>,
    enum_unlabeled: self::_puroro::internal::field_type::SingularNumericalField::<
        self::_puroro_root::ser_tests3::Enum,
        self::_puroro::tags::Enum3::<self::_puroro_root::ser_tests3::Enum>,
    >,
    enum_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        self::_puroro_root::ser_tests3::Enum,
        self::_puroro::tags::Enum3::<self::_puroro_root::ser_tests3::Enum>,
    >,
    very_large_field_number: self::_puroro::internal::field_type::SingularNumericalField::<
        i32,
        self::_puroro::tags::Int32,
    >,
    _bitfield: self::_puroro::bitvec::BitArray<0usize>,
}
impl Msg {}
pub enum Enum {
    Zeroth,
    First,
    Tenth,
    _None(i32),
}
