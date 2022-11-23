pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}
pub mod _puroro {
    pub use ::puroro::*;
}
#[derive(::std::default::Default)]
pub struct Annotation {
    path: self::_puroro::internal::field_type::RepeatedNumericalField::<
        i32,
        self::_puroro::tags::Int32,
    >,
    source_file: self::_puroro::internal::field_type::OptionalStringField::<0usize>,
    begin: self::_puroro::internal::field_type::OptionalNumericalField::<
        i32,
        self::_puroro::tags::Int32,
        1usize,
    >,
    end: self::_puroro::internal::field_type::OptionalNumericalField::<
        i32,
        self::_puroro::tags::Int32,
        2usize,
    >,
    _bitfield: self::_puroro::bitvec::BitArray<1usize>,
}
impl Annotation {
    pub fn path(&self) -> &[i32] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            i32,
            self::_puroro::tags::Int32,
        > as RepeatedFieldType>::get_field(&self.path, &self._bitfield)
    }
    pub fn source_file(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            0usize,
        > as NonRepeatedFieldType>::get_field(
            &self.source_file,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn begin(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            1usize,
        > as NonRepeatedFieldType>::get_field(
            &self.begin,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn end(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            2usize,
        > as NonRepeatedFieldType>::get_field(
            &self.end,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
}
impl self::_puroro::Message for Annotation {
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
impl ::std::clone::Clone for Annotation {
    fn clone(&self) -> Self {
        Self {
            path: <self::_puroro::internal::field_type::RepeatedNumericalField::<
                i32,
                self::_puroro::tags::Int32,
            > as ::std::clone::Clone>::clone(&self.path),
            source_file: <self::_puroro::internal::field_type::OptionalStringField::<
                0usize,
            > as ::std::clone::Clone>::clone(&self.source_file),
            begin: <self::_puroro::internal::field_type::OptionalNumericalField::<
                i32,
                self::_puroro::tags::Int32,
                1usize,
            > as ::std::clone::Clone>::clone(&self.begin),
            end: <self::_puroro::internal::field_type::OptionalNumericalField::<
                i32,
                self::_puroro::tags::Int32,
                2usize,
            > as ::std::clone::Clone>::clone(&self.end),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
