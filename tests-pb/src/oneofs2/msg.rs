pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}
pub mod _puroro {
    pub use ::puroro::*;
}
pub(super) union GroupOne {
    _none: (),
    g1_int32: ::std::mem::ManuallyDrop::<
        self::_puroro::internal::oneof_field_type::NumericalField::<
            i32,
            self::_puroro::tags::Int32,
        >,
    >,
    g1_string: ::std::mem::ManuallyDrop::<
        self::_puroro::internal::oneof_field_type::StringField,
    >,
}
pub(super) union GroupTwo {
    _none: (),
    g2_f32: ::std::mem::ManuallyDrop::<
        self::_puroro::internal::oneof_field_type::NumericalField::<
            f32,
            self::_puroro::tags::Float,
        >,
    >,
    g2_string: ::std::mem::ManuallyDrop::<
        self::_puroro::internal::oneof_field_type::StringField,
    >,
    g2_submsg: ::std::mem::ManuallyDrop::<
        self::_puroro::internal::oneof_field_type::HeapMessageField::<
            self::_puroro_root::oneofs2::Submsg,
        >,
    >,
}
pub(super) union GroupThree {
    _none: (),
    g3_int32: ::std::mem::ManuallyDrop::<
        self::_puroro::internal::oneof_field_type::NumericalField::<
            i32,
            self::_puroro::tags::Int32,
        >,
    >,
}
