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
    _bitfield: self::_puroro::bitvec::BitArray<1usize>,
}
impl Test1 {
    pub fn a(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field(
            &self.a,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
}
pub struct Test2 {
    b: self::_puroro::internal::field_type::OptionalStringField::<0usize>,
    _bitfield: self::_puroro::bitvec::BitArray<1usize>,
}
impl Test2 {
    pub fn b(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            0usize,
        > as NonRepeatedFieldType>::get_field(
            &self.b,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
}
pub struct Test3 {
    c: self::_puroro::internal::field_type::SingularHeapMessageField::<
        self::_puroro_root::official_samples2::Test1,
    >,
    _bitfield: self::_puroro::bitvec::BitArray<0usize>,
}
impl Test3 {
    pub fn c(
        &self,
    ) -> ::std::option::Option::<&self::_puroro_root::official_samples2::Test1> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField::<
            self::_puroro_root::official_samples2::Test1,
        > as NonRepeatedFieldType>::get_field(
            &self.c,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
}
pub struct Test4 {
    d: self::_puroro::internal::field_type::RepeatedNumericalField::<
        i32,
        self::_puroro::tags::Int32,
    >,
    _bitfield: self::_puroro::bitvec::BitArray<0usize>,
}
impl Test4 {
    pub fn d(&self) -> &[i32] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            i32,
            self::_puroro::tags::Int32,
        > as RepeatedFieldType>::get_field(&self.d, &self._bitfield)
    }
}
