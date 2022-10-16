// A generated source code by puroro library
// package .google.protobuf.SourceCodeInfo

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub mod _puroro {
    pub use ::puroro::*;
}

#[derive(Default)]
pub struct Location {
    // Repeated, Variant(Int32)
    path: self::_puroro::internal::field_type::RepeatedNumericalField<
        i32,
        self::_puroro::tags::Int32,
    >,
    // Repeated, Variant(Int32)
    span: self::_puroro::internal::field_type::RepeatedNumericalField<
        i32,
        self::_puroro::tags::Int32,
    >,
    // Optional, LengthDelimited(String)
    leading_comments: self::_puroro::internal::field_type::OptionalStringField<0>,
    // Optional, LengthDelimited(String)
    trailing_comments: self::_puroro::internal::field_type::OptionalStringField<1>,
    // Repeated, LengthDelimited(String)
    leading_detached_comments: self::_puroro::internal::field_type::RepeatedStringField,

    _bitfield: self::_puroro::bitvec::BitArray<1>,
}

impl Location {
    // Repeated, Variant(Int32)
    pub fn path(&self) -> &[i32] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<i32, self::_puroro::tags::Int32> as RepeatedFieldType>::get_field(
            &self.path, &self._bitfield, 
        )
    }
    pub fn path_mut(&mut self) -> &mut ::std::vec::Vec<i32> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<i32, self::_puroro::tags::Int32> as RepeatedFieldType>::mut_field(
            &mut self.path, &mut self._bitfield, 
        )
    }
    pub fn clear_path(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<i32, self::_puroro::tags::Int32> as RepeatedFieldType>::clear(
            &mut self.path, &mut self._bitfield, 
        )
    }
    // Repeated, Variant(Int32)
    pub fn span(&self) -> &[i32] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<i32, self::_puroro::tags::Int32> as RepeatedFieldType>::get_field(
            &self.span, &self._bitfield, 
        )
    }
    pub fn span_mut(&mut self) -> &mut ::std::vec::Vec<i32> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<i32, self::_puroro::tags::Int32> as RepeatedFieldType>::mut_field(
            &mut self.span, &mut self._bitfield, 
        )
    }
    pub fn clear_span(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<i32, self::_puroro::tags::Int32> as RepeatedFieldType>::clear(
            &mut self.span, &mut self._bitfield, 
        )
    }
    // Optional, LengthDelimited(String)
    pub fn leading_comments(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field(
            &self.leading_comments, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn leading_comments_opt(&self) -> ::std::option::Option<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field_opt(
            &self.leading_comments, &self._bitfield,
        )
    }
    pub fn has_leading_comments(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field_opt(
            &self.leading_comments, &self._bitfield,
        ).is_some()
    }
    pub fn leading_comments_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::mut_field(
            &mut self.leading_comments, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_leading_comments(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField<0> as NonRepeatedFieldType>::clear(
            &mut self.leading_comments,
            &mut self._bitfield,
        )
    }
    // Optional, LengthDelimited(String)
    pub fn trailing_comments(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<1> as NonRepeatedFieldType>::get_field(
            &self.trailing_comments, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn trailing_comments_opt(&self) -> ::std::option::Option<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<1> as NonRepeatedFieldType>::get_field_opt(
            &self.trailing_comments, &self._bitfield,
        )
    }
    pub fn has_trailing_comments(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<1> as NonRepeatedFieldType>::get_field_opt(
            &self.trailing_comments, &self._bitfield,
        ).is_some()
    }
    pub fn trailing_comments_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<1> as NonRepeatedFieldType>::mut_field(
            &mut self.trailing_comments, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_trailing_comments(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField<1> as NonRepeatedFieldType>::clear(
            &mut self.trailing_comments,
            &mut self._bitfield,
        )
    }
    // Repeated, LengthDelimited(String)
    pub fn leading_detached_comments(&self) -> &[::std::string::String] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedStringField as RepeatedFieldType>::get_field(
            &self.leading_detached_comments,
            &self._bitfield,
        )
    }
    pub fn leading_detached_comments_mut(&mut self) -> &mut ::std::vec::Vec<::std::string::String> {
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
        #[allow(unused)]
        use ::std::result::Result::Ok;
        let mut msg: Self = ::std::default::Default::default();
        msg.merge_from_bytes_iter(iter)?;
        Ok(msg)
    }

    fn merge_from_bytes_iter<I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>>(
        &mut self,
        mut iter: I,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use self::_puroro::internal::field_type::FieldType;
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::option::Option::Some;
        #[allow(unused)]
        use ::std::result::Result::Ok;
        while let Some((number, field_data)) =
            self::_puroro::internal::ser::FieldData::from_bytes_iter(iter.by_ref())?
        {
            match number {
                1 => <self::_puroro::internal::field_type::RepeatedNumericalField::<i32, self::_puroro::tags::Int32> as FieldType>::deser_from_iter(
                    &mut self.path,
                    &mut self._bitfield,
                    field_data,
                )?,
                2 => <self::_puroro::internal::field_type::RepeatedNumericalField::<i32, self::_puroro::tags::Int32> as FieldType>::deser_from_iter(
                    &mut self.span,
                    &mut self._bitfield,
                    field_data,
                )?,
                3 => <self::_puroro::internal::field_type::OptionalStringField::<0> as FieldType>::deser_from_iter(
                    &mut self.leading_comments,
                    &mut self._bitfield,
                    field_data,
                )?,
                4 => <self::_puroro::internal::field_type::OptionalStringField::<1> as FieldType>::deser_from_iter(
                    &mut self.trailing_comments,
                    &mut self._bitfield,
                    field_data,
                )?,
                6 => <self::_puroro::internal::field_type::RepeatedStringField as FieldType>::deser_from_iter(
                    &mut self.leading_detached_comments,
                    &mut self._bitfield,
                    field_data,
                )?,
                _ => todo!(), // Unknown field...
            }
        }
        Ok(())
    }

    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)] out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use self::_puroro::internal::field_type::FieldType;
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::result::Result::Ok;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            i32,
            self::_puroro::tags::Int32,
        > as FieldType>::ser_to_write(&self.path, &self._bitfield, 1, out)?;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            i32,
            self::_puroro::tags::Int32,
        > as FieldType>::ser_to_write(&self.span, &self._bitfield, 2, out)?;
        <self::_puroro::internal::field_type::OptionalStringField<0> as FieldType>::ser_to_write(
            &self.leading_comments,
            &self._bitfield,
            3,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalStringField<1> as FieldType>::ser_to_write(
            &self.trailing_comments,
            &self._bitfield,
            4,
            out,
        )?;
        <self::_puroro::internal::field_type::RepeatedStringField as FieldType>::ser_to_write(
            &self.leading_detached_comments,
            &self._bitfield,
            6,
            out,
        )?;

        Ok(())
    }
}

impl ::std::clone::Clone for Location {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::clone::Clone;
        Self {
            path: Clone::clone(&self.path),
            span: Clone::clone(&self.span),
            leading_comments: Clone::clone(&self.leading_comments),
            trailing_comments: Clone::clone(&self.trailing_comments),
            leading_detached_comments: Clone::clone(&self.leading_detached_comments),

            _bitfield: Clone::clone(&self._bitfield),
        }
    }
}

impl ::std::fmt::Debug for Location {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct("Location")
            .field("path", &self.path())
            .field("span", &self.span())
            .field("leading_comments", &self.leading_comments())
            .field("trailing_comments", &self.trailing_comments())
            .field(
                "leading_detached_comments",
                &self.leading_detached_comments(),
            )
            .finish()
    }
}

impl ::std::cmp::PartialEq for Location {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;

        true && self.path() == rhs.path()
            && self.span() == rhs.span()
            && self.leading_comments_opt() == rhs.leading_comments_opt()
            && self.trailing_comments_opt() == rhs.trailing_comments_opt()
            && self.leading_detached_comments() == rhs.leading_detached_comments()
    }
}

impl ::std::ops::Drop for Location {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
    }
}
