pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}
pub mod _puroro {
    pub use ::puroro::*;
}
pub struct Submsg {
    item_inner: self::_puroro::internal::field_type::SingularNumericalField::<
        i32,
        self::_puroro::tags::Int32,
    >,
    _bitfield: self::_puroro::bitvec::BitArray<0usize>,
}
impl Submsg {
    pub fn item_inner(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            i32,
            self::_puroro::tags::Int32,
        > as NonRepeatedFieldType>::get_field(
            &self.item_inner,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
}
pub enum Enum {
    MyValue,
    _None(i32),
}
