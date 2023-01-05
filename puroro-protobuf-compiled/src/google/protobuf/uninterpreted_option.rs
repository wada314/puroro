mod _root {
    #[allow(unused)]
    pub(crate) use super::super::_root::*;
}
mod _puroro {
    #[allow(unused)]
    pub(crate) use ::puroro::*;
}
mod _pinternal {
    #[allow(unused)]
    pub(crate) use ::puroro::internal::*;
}
#[derive(::std::default::Default)]
pub struct NamePart {
    fields: self::_root::google::protobuf::uninterpreted_option::_fields::NamePartFields<
        self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        >,
        self::_pinternal::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            1usize,
        >,
    >,
    _bitfield: self::_pinternal::BitArray<1usize>,
}
impl NamePart {
    pub fn name_part(&self) -> &str {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.fields.name_part,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn name_part_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.fields.name_part,
            &self._bitfield,
        )
    }
    pub fn name_part_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.fields.name_part,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_name_part(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(
                &self.fields.name_part,
                &self._bitfield,
            )
            .is_some()
    }
    pub fn clear_name_part(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::clear(
            &mut self.fields.name_part,
            &mut self._bitfield,
        )
    }
    pub fn is_extension(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            1usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.fields.is_extension,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn is_extension_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.fields.is_extension,
            &self._bitfield,
        )
    }
    pub fn is_extension_mut(&mut self) -> &mut bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            1usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.fields.is_extension,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_is_extension(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(
                &self.fields.is_extension,
                &self._bitfield,
            )
            .is_some()
    }
    pub fn clear_is_extension(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            1usize,
        > as NonRepeatedFieldType>::clear(
            &mut self.fields.is_extension,
            &mut self._bitfield,
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
        use self::_pinternal::ser::FieldData;
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            match number {
                1i32 => {
                    <self::_pinternal::OptionalUnsizedField::<
                        ::std::string::String,
                        self::_pinternal::tags::String,
                        0usize,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.fields.name_part,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                2i32 => {
                    <self::_pinternal::OptionalNumericalField::<
                        bool,
                        self::_pinternal::tags::Bool,
                        1usize,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.fields.is_extension,
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
        use self::_pinternal::OneofUnion as _;
        <self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.fields.name_part,
            &self._bitfield,
            1i32,
            out,
        )?;
        <self::_pinternal::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            1usize,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.fields.is_extension,
            &self._bitfield,
            2i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for NamePart {
    fn clone(&self) -> Self {
        Self {
            fields: self::_fields::NamePartFields {
                name_part: <self::_pinternal::OptionalUnsizedField::<
                    ::std::string::String,
                    self::_pinternal::tags::String,
                    0usize,
                > as ::std::clone::Clone>::clone(&self.fields.name_part),
                is_extension: <self::_pinternal::OptionalNumericalField::<
                    bool,
                    self::_pinternal::tags::Bool,
                    1usize,
                > as ::std::clone::Clone>::clone(&self.fields.is_extension),
            },
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
impl ::std::ops::Drop for NamePart {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for NamePart {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(NamePart))
            .field(stringify!(name_part), &self.name_part_opt())
            .field(stringify!(is_extension), &self.is_extension_opt())
            .finish()
    }
}
impl ::std::cmp::PartialEq for NamePart {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        true && self.name_part_opt() == rhs.name_part_opt()
            && self.is_extension_opt() == rhs.is_extension_opt()
    }
}
pub mod _fields {
    mod _root {
        #[allow(unused)]
        pub use super::super::_root::*;
    }
    mod _puroro {
        #[allow(unused)]
        pub use ::puroro::*;
    }
    mod _pinternal {
        #[allow(unused)]
        pub use ::puroro::internal::*;
    }
    #[derive(::std::default::Default)]
    pub struct NamePartFields<TNamePart, TIsExtension> {
        pub name_part: TNamePart,
        pub is_extension: TIsExtension,
    }
}
pub use self::_fields::*;
