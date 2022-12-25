mod _puroro_root {
    #[allow(unused)]
    pub(crate) use super::super::_puroro_root::*;
}
mod _puroro {
    #[allow(unused)]
    pub(crate) use ::puroro::*;
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
    pub fn path_mut(&mut self) -> &mut ::std::vec::Vec::<i32> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            i32,
            self::_puroro::tags::Int32,
        > as RepeatedFieldType>::mut_field(&mut self.path, &mut self._bitfield)
    }
    pub fn clear_path(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            i32,
            self::_puroro::tags::Int32,
        > as RepeatedFieldType>::clear(&mut self.path, &mut self._bitfield)
    }
    pub fn span(&self) -> &[i32] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            i32,
            self::_puroro::tags::Int32,
        > as RepeatedFieldType>::get_field(&self.span, &self._bitfield)
    }
    pub fn span_mut(&mut self) -> &mut ::std::vec::Vec::<i32> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            i32,
            self::_puroro::tags::Int32,
        > as RepeatedFieldType>::mut_field(&mut self.span, &mut self._bitfield)
    }
    pub fn clear_span(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            i32,
            self::_puroro::tags::Int32,
        > as RepeatedFieldType>::clear(&mut self.span, &mut self._bitfield)
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
    pub fn leading_comments_opt(&self) -> ::std::option::Option::<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.leading_comments,
            &self._bitfield,
        )
    }
    pub fn leading_comments_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            0usize,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.leading_comments,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_leading_comments(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(
                &self.leading_comments,
                &self._bitfield,
            )
            .is_some()
    }
    pub fn clear_leading_comments(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            0usize,
        > as NonRepeatedFieldType>::clear(
            &mut self.leading_comments,
            &mut self._bitfield,
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
    pub fn trailing_comments_opt(&self) -> ::std::option::Option::<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.trailing_comments,
            &self._bitfield,
        )
    }
    pub fn trailing_comments_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            1usize,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.trailing_comments,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_trailing_comments(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(
                &self.trailing_comments,
                &self._bitfield,
            )
            .is_some()
    }
    pub fn clear_trailing_comments(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            1usize,
        > as NonRepeatedFieldType>::clear(
            &mut self.trailing_comments,
            &mut self._bitfield,
        )
    }
    pub fn leading_detached_comments(
        &self,
    ) -> &[impl ::std::ops::Deref::<
        Target = str,
    > + ::std::fmt::Debug + ::std::cmp::PartialEq] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedStringField as RepeatedFieldType>::get_field(
            &self.leading_detached_comments,
            &self._bitfield,
        )
    }
    pub fn leading_detached_comments_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<::std::string::String> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedStringField as RepeatedFieldType>::mut_field(
            &mut self.leading_detached_comments,
            &mut self._bitfield,
        )
    }
    pub fn clear_leading_detached_comments(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedStringField as RepeatedFieldType>::clear(
            &mut self.leading_detached_comments,
            &mut self._bitfield,
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
        use self::_puroro::internal::ser::FieldData;
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion as _;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            match number {
                1i32 => {
                    <self::_puroro::internal::field_type::RepeatedNumericalField::<
                        i32,
                        self::_puroro::tags::Int32,
                    > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.path,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                2i32 => {
                    <self::_puroro::internal::field_type::RepeatedNumericalField::<
                        i32,
                        self::_puroro::tags::Int32,
                    > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.span,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                3i32 => {
                    <self::_puroro::internal::field_type::OptionalStringField::<
                        0usize,
                    > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.leading_comments,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                4i32 => {
                    <self::_puroro::internal::field_type::OptionalStringField::<
                        1usize,
                    > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.trailing_comments,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                6i32 => {
                    <self::_puroro::internal::field_type::RepeatedStringField as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.leading_detached_comments,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                _ => todo!(),
            }
        }
        ::std::result::Result::Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion as _;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            i32,
            self::_puroro::tags::Int32,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.path,
            &self._bitfield,
            1i32,
            out,
        )?;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            i32,
            self::_puroro::tags::Int32,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.span,
            &self._bitfield,
            2i32,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalStringField::<
            0usize,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.leading_comments,
            &self._bitfield,
            3i32,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalStringField::<
            1usize,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.trailing_comments,
            &self._bitfield,
            4i32,
            out,
        )?;
        <self::_puroro::internal::field_type::RepeatedStringField as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.leading_detached_comments,
            &self._bitfield,
            6i32,
            out,
        )?;
        ::std::result::Result::Ok(())
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
impl ::std::ops::Drop for Location {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for Location {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(Location))
            .field(stringify!(path), &self.path())
            .field(stringify!(span), &self.span())
            .field(stringify!(leading_comments), &self.leading_comments_opt())
            .field(stringify!(trailing_comments), &self.trailing_comments_opt())
            .field(
                stringify!(leading_detached_comments),
                &self.leading_detached_comments(),
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for Location {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion as _;
        true && self.path() == rhs.path() && self.span() == rhs.span()
            && self.leading_comments_opt() == rhs.leading_comments_opt()
            && self.trailing_comments_opt() == rhs.trailing_comments_opt()
            && self.leading_detached_comments() == rhs.leading_detached_comments()
    }
}
