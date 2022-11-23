pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}
pub mod _puroro {
    pub use ::puroro::*;
}
#[derive(::std::default::Default)]
pub struct Location {
    path: self::_puroro::internal::field_type::RepeatedNumericalField::<
        i32,
        self::_puroro::tags::Int32,
    >,
    span: self::_puroro::internal::field_type::RepeatedNumericalField::<
        i32,
        self::_puroro::tags::Int32,
    >,
    leading_comments: self::_puroro::internal::field_type::OptionalStringField::<0usize>,
    trailing_comments: self::_puroro::internal::field_type::OptionalStringField::<
        1usize,
    >,
    leading_detached_comments: self::_puroro::internal::field_type::RepeatedStringField,
    _bitfield: self::_puroro::bitvec::BitArray<1usize>,
}
impl Location {
    pub fn path(&self) -> &[i32] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            i32,
            self::_puroro::tags::Int32,
        > as RepeatedFieldType>::get_field(&self.path, &self._bitfield)
    }
    pub fn span(&self) -> &[i32] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            i32,
            self::_puroro::tags::Int32,
        > as RepeatedFieldType>::get_field(&self.span, &self._bitfield)
    }
    pub fn leading_comments(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            0usize,
        > as NonRepeatedFieldType>::get_field(
            &self.leading_comments,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn trailing_comments(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            1usize,
        > as NonRepeatedFieldType>::get_field(
            &self.trailing_comments,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn leading_detached_comments(
        &self,
    ) -> &[impl ::std::ops::Deref::<Target = str>] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedStringField as RepeatedFieldType>::get_field(
            &self.leading_detached_comments,
            &self._bitfield,
        )
    }
}
impl self::_puroro::Message for Location {
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
impl ::std::clone::Clone for Location {
    fn clone(&self) -> Self {
        Self {
            path: <self::_puroro::internal::field_type::RepeatedNumericalField::<
                i32,
                self::_puroro::tags::Int32,
            > as ::std::clone::Clone>::clone(&self.path),
            span: <self::_puroro::internal::field_type::RepeatedNumericalField::<
                i32,
                self::_puroro::tags::Int32,
            > as ::std::clone::Clone>::clone(&self.span),
            leading_comments: <self::_puroro::internal::field_type::OptionalStringField::<
                0usize,
            > as ::std::clone::Clone>::clone(&self.leading_comments),
            trailing_comments: <self::_puroro::internal::field_type::OptionalStringField::<
                1usize,
            > as ::std::clone::Clone>::clone(&self.trailing_comments),
            leading_detached_comments: <self::_puroro::internal::field_type::RepeatedStringField as ::std::clone::Clone>::clone(
                &self.leading_detached_comments,
            ),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
