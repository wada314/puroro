pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}
pub mod _puroro {
    pub use ::puroro::*;
}
#[derive(::std::default::Default)]
pub struct NamePart {
    name_part: self::_puroro::internal::field_type::OptionalStringField::<0usize>,
    is_extension: self::_puroro::internal::field_type::OptionalNumericalField::<
        bool,
        self::_puroro::tags::Bool,
        1usize,
    >,
    _bitfield: self::_puroro::bitvec::BitArray<1usize>,
}
impl NamePart {
    pub fn name_part(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            0usize,
        > as NonRepeatedFieldType>::get_field(
            &self.name_part,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn is_extension(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            bool,
            self::_puroro::tags::Bool,
            1usize,
        > as NonRepeatedFieldType>::get_field(
            &self.is_extension,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
}
impl self::_puroro::Message for NamePart {
    fn from_bytes_iter<I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>>(
        iter: I,
    ) -> self::_puroro::Result<Self> {
        let mut msg = <Self as ::std::default::Default>::default();
        msg.merge_from_bytes_iter(iter)?;
        ::std::result::Result::Ok(msg)
    }
    fn merge_from_bytes_iter<I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>>(
        &mut self,
        mut iter: I,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use ::std::result::Result::Ok;
        #[allow(unused)]
        use ::std::option::Option::Some;
        #[allow(unused)]
        use self::_puroro::internal::field_type::FieldType;
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        use self::_puroro::internal::ser::FieldData;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            todo!()
        }
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use ::std::result::Result::Ok;
        Ok(todo!())
    }
}
impl ::std::clone::Clone for NamePart {
    fn clone(&self) -> Self {
        Self {
            name_part: <self::_puroro::internal::field_type::OptionalStringField::<
                0usize,
            > as ::std::clone::Clone>::clone(&self.name_part),
            is_extension: <self::_puroro::internal::field_type::OptionalNumericalField::<
                bool,
                self::_puroro::tags::Bool,
                1usize,
            > as ::std::clone::Clone>::clone(&self.is_extension),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
