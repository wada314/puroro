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
pub enum GroupOneCase<G1Int32 = (), G1String = ()> {
    G1Int32(G1Int32),
    G1String(G1String),
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
            self::_puroro_root::oneofs3::Submsg,
        >,
    >,
}
pub enum GroupTwoCase<G2F32 = (), G2String = (), G2Submsg = ()> {
    G2F32(G2F32),
    G2String(G2String),
    G2Submsg(G2Submsg),
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
pub enum GroupThreeCase<G3Int32 = ()> {
    G3Int32(G3Int32),
}
