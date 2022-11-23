pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}
pub mod _puroro {
    pub use ::puroro::*;
}
pub struct Msg {
    recursive_unlabeled: self::_puroro::internal::field_type::SingularHeapMessageField::<
        self::_puroro_root::self_recursive::Msg,
    >,
    _bitfield: self::_puroro::bitvec::BitArray<0usize>,
}
impl Msg {
    pub fn recursive_unlabeled(
        &self,
    ) -> ::std::option::Option::<&self::_puroro_root::self_recursive::Msg> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField::<
            self::_puroro_root::self_recursive::Msg,
        > as NonRepeatedFieldType>::get_field(
            &self.recursive_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
}
