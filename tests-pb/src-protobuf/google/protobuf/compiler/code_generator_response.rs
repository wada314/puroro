// A generated source code by puroro library
// package .google.protobuf.compiler.CodeGeneratorResponse

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub mod _puroro {
    pub use ::puroro::*;
}

#[derive(Default)]
pub struct File {
    // Optional, LengthDelimited(String)
    name: self::_puroro::internal::field_type::OptionalStringField<0>,
    // Optional, LengthDelimited(String)
    insertion_point: self::_puroro::internal::field_type::OptionalStringField<1>,
    // Optional, LengthDelimited(String)
    content: self::_puroro::internal::field_type::OptionalStringField<2>,
    // Optional, LengthDelimited(Message(Fqtn(".google.protobuf.GeneratedCodeInfo")))
    generated_code_info: self::_puroro::internal::field_type::SingularHeapMessageField<
        _puroro_root::google::protobuf::GeneratedCodeInfo,
    >,

    _bitfield: self::_puroro::bitvec::BitArray<1>,
}

impl File {
    // Optional, LengthDelimited(String)
    pub fn name(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field(
            &self.name, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn name_opt(&self) -> ::std::option::Option<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field_opt(
            &self.name, &self._bitfield,
        )
    }
    pub fn has_name(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field_opt(
            &self.name, &self._bitfield,
        ).is_some()
    }
    pub fn name_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::mut_field(
            &mut self.name, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_name(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField<0> as NonRepeatedFieldType>::clear(
            &mut self.name,
            &mut self._bitfield,
        )
    }
    // Optional, LengthDelimited(String)
    pub fn insertion_point(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<1> as NonRepeatedFieldType>::get_field(
            &self.insertion_point, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn insertion_point_opt(&self) -> ::std::option::Option<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<1> as NonRepeatedFieldType>::get_field_opt(
            &self.insertion_point, &self._bitfield,
        )
    }
    pub fn has_insertion_point(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<1> as NonRepeatedFieldType>::get_field_opt(
            &self.insertion_point, &self._bitfield,
        ).is_some()
    }
    pub fn insertion_point_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<1> as NonRepeatedFieldType>::mut_field(
            &mut self.insertion_point, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_insertion_point(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField<1> as NonRepeatedFieldType>::clear(
            &mut self.insertion_point,
            &mut self._bitfield,
        )
    }
    // Optional, LengthDelimited(String)
    pub fn content(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<2> as NonRepeatedFieldType>::get_field(
            &self.content, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn content_opt(&self) -> ::std::option::Option<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<2> as NonRepeatedFieldType>::get_field_opt(
            &self.content, &self._bitfield,
        )
    }
    pub fn has_content(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<2> as NonRepeatedFieldType>::get_field_opt(
            &self.content, &self._bitfield,
        ).is_some()
    }
    pub fn content_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<2> as NonRepeatedFieldType>::mut_field(
            &mut self.content, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_content(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField<2> as NonRepeatedFieldType>::clear(
            &mut self.content,
            &mut self._bitfield,
        )
    }
    // Optional, LengthDelimited(Message(Fqtn(".google.protobuf.GeneratedCodeInfo")))
    pub fn generated_code_info(
        &self,
    ) -> ::std::option::Option<&_puroro_root::google::protobuf::GeneratedCodeInfo> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::GeneratedCodeInfo,
        > as NonRepeatedFieldType>::get_field(
            &self.generated_code_info,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn generated_code_info_opt(
        &self,
    ) -> ::std::option::Option<&_puroro_root::google::protobuf::GeneratedCodeInfo> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::GeneratedCodeInfo,
        > as NonRepeatedFieldType>::get_field_opt(&self.generated_code_info, &self._bitfield)
    }
    pub fn has_generated_code_info(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::GeneratedCodeInfo,
        > as NonRepeatedFieldType>::get_field_opt(&self.generated_code_info, &self._bitfield)
        .is_some()
    }
    pub fn generated_code_info_mut(
        &mut self,
    ) -> &mut _puroro_root::google::protobuf::GeneratedCodeInfo {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::GeneratedCodeInfo,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.generated_code_info,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_generated_code_info(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::GeneratedCodeInfo,
        > as NonRepeatedFieldType>::clear(&mut self.generated_code_info, &mut self._bitfield)
    }
}

impl self::_puroro::Message for File {
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
                    &mut self.name,
                    &mut self._bitfield,
                    field_data,
                )?,
                2 => <self::_puroro::internal::field_type::OptionalStringField::<1> as FieldType>::deser_from_iter(
                    &mut self.insertion_point,
                    &mut self._bitfield,
                    field_data,
                )?,
                15 => <self::_puroro::internal::field_type::OptionalStringField::<2> as FieldType>::deser_from_iter(
                    &mut self.content,
                    &mut self._bitfield,
                    field_data,
                )?,
                16 => <self::_puroro::internal::field_type::SingularHeapMessageField::<_puroro_root::google::protobuf::GeneratedCodeInfo> as FieldType>::deser_from_iter(
                    &mut self.generated_code_info,
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
            &self.name,
            &self._bitfield,
            1,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalStringField<1> as FieldType>::ser_to_write(
            &self.insertion_point,
            &self._bitfield,
            2,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalStringField<2> as FieldType>::ser_to_write(
            &self.content,
            &self._bitfield,
            15,
            out,
        )?;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::GeneratedCodeInfo,
        > as FieldType>::ser_to_write(&self.generated_code_info, &self._bitfield, 16, out)?;

        Ok(())
    }
}

impl ::std::clone::Clone for File {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::clone::Clone;
        Self {
            name: Clone::clone(&self.name),
            insertion_point: Clone::clone(&self.insertion_point),
            content: Clone::clone(&self.content),
            generated_code_info: Clone::clone(&self.generated_code_info),

            _bitfield: Clone::clone(&self._bitfield),
        }
    }
}

impl ::std::fmt::Debug for File {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct("File")
            .field("name", &self.name())
            .field("insertion_point", &self.insertion_point())
            .field("content", &self.content())
            .field("generated_code_info", &self.generated_code_info())
            .finish()
    }
}

impl ::std::cmp::PartialEq for File {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;

        true && self.name_opt() == rhs.name_opt()
            && self.insertion_point_opt() == rhs.insertion_point_opt()
            && self.content_opt() == rhs.content_opt()
            && self.generated_code_info_opt() == rhs.generated_code_info_opt()
    }
}

impl ::std::ops::Drop for File {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Feature {
    FeatureNone,
    FeatureProto3Optional,
}

impl ::std::default::Default for Feature {
    fn default() -> Self {
        Feature::FeatureNone
    }
}

impl ::std::convert::TryFrom<i32> for Feature {
    type Error = self::_puroro::PuroroError;
    fn try_from(x: i32) -> ::std::result::Result<Self, Self::Error> {
        #[allow(unused)]
        use ::std::result::Result::{Err, Ok};
        match x {
            0 => Ok(self::Feature::FeatureNone),
            1 => Ok(self::Feature::FeatureProto3Optional),
            e => Err(self::_puroro::ErrorKind::UnknownEnumVariant(e))?,
        }
    }
}

impl ::std::convert::From<Feature> for i32 {
    fn from(x: Feature) -> i32 {
        match x {
            self::Feature::FeatureNone => 0,
            self::Feature::FeatureProto3Optional => 1,
        }
    }
}
pub mod file;
