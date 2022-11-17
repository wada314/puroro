//!Generated from package "official_samples2"
pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}
pub mod _puroro {
    pub use ::puroro::*;
}
pub struct Test1 {
    a: self::_puroro::internal::field_type::OptionalNumericalField::<
        i32,
        self::_puroro::tags::Int32,
        0usize,
    >,
}
pub struct Test2 {
    b: self::_puroro::internal::field_type::OptionalStringField::<0usize>,
}
pub struct Test3 {
    c: self::_puroro::internal::field_type::SingularHeapMessageField::<()>,
}
pub struct Test4 {
    d: self::_puroro::internal::field_type::RepeatedNumericalField::<
        i32,
        self::_puroro::tags::Int32,
    >,
}
