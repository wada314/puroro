// A generated source code by puroro library
// package .google.protobuf.UninterpretedOption

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub mod _puroro {
    pub use ::puroro::*;
}

#[derive(Default)]
pub struct NamePart {
    // Optional, LengthDelimited(String)
    name_part: self::_puroro::internal::field_type::OptionalStringField<0>,
    // Optional, Variant(Bool)
    is_extension: self::_puroro::internal::field_type::OptionalNumericalField<
        bool,
        self::_puroro::tags::Bool,
        1,
    >,

    _bitfield: self::_puroro::bitvec::BitArray<1>,
}

impl NamePart {
    // Optional, LengthDelimited(String)
    pub fn name_part(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field(
            &self.name_part, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn name_part_opt(&self) -> ::std::option::Option<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field_opt(
            &self.name_part, &self._bitfield,
        )
    }
    pub fn has_name_part(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field_opt(
            &self.name_part, &self._bitfield,
        ).is_some()
    }
    pub fn name_part_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::mut_field(
            &mut self.name_part, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_name_part(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField<0> as NonRepeatedFieldType>::clear(
            &mut self.name_part,
            &mut self._bitfield,
        )
    }
    // Optional, Variant(Bool)
    pub fn is_extension(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            1,
        > as NonRepeatedFieldType>::get_field(
            &self.is_extension,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn is_extension_opt(&self) -> ::std::option::Option<bool> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            1,
        > as NonRepeatedFieldType>::get_field_opt(&self.is_extension, &self._bitfield)
    }
    pub fn has_is_extension(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            1,
        > as NonRepeatedFieldType>::get_field_opt(&self.is_extension, &self._bitfield)
        .is_some()
    }
    pub fn is_extension_mut(&mut self) -> &mut bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            1,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.is_extension,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_is_extension(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            1,
        > as NonRepeatedFieldType>::clear(&mut self.is_extension, &mut self._bitfield)
    }
}

impl self::_puroro::Message for NamePart {
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
                1 => <self::_puroro::internal::field_type::OptionalStringField::<0> as FieldType>::deser_from_iter(
                    &mut self.name_part,
                    &mut self._bitfield,
                    field_data,
                )?,
                2 => <self::_puroro::internal::field_type::OptionalNumericalField::<bool, self::_puroro::tags::Bool, 1> as FieldType>::deser_from_iter(
                    &mut self.is_extension,
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
        <self::_puroro::internal::field_type::OptionalStringField<0> as FieldType>::ser_to_write(
            &self.name_part,
            &self._bitfield,
            1,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            1,
        > as FieldType>::ser_to_write(&self.is_extension, &self._bitfield, 2, out)?;

        Ok(())
    }
}

impl ::std::clone::Clone for NamePart {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::clone::Clone;
        Self {
            name_part: Clone::clone(&self.name_part),
            is_extension: Clone::clone(&self.is_extension),

            _bitfield: Clone::clone(&self._bitfield),
        }
    }
}

impl ::std::fmt::Debug for NamePart {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct("NamePart")
            .field("name_part", &self.name_part())
            .field("is_extension", &self.is_extension())
            .finish()
    }
}

impl ::std::cmp::PartialEq for NamePart {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;

        true && self.name_part_opt() == rhs.name_part_opt()
            && self.is_extension_opt() == rhs.is_extension_opt()
    }
}

impl ::std::ops::Drop for NamePart {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
    }
}
