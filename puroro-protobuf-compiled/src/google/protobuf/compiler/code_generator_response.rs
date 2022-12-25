mod _puroro_root {
    #[allow(unused)]
    pub(crate) use super::super::_puroro_root::*;
}
mod _puroro {
    #[allow(unused)]
    pub(crate) use ::puroro::*;
}
#[derive(::std::default::Default)]
pub struct File {
    name: self::_puroro::internal::field_type::OptionalStringField::<0usize>,
    insertion_point: self::_puroro::internal::field_type::OptionalStringField::<1usize>,
    content: self::_puroro::internal::field_type::OptionalStringField::<2usize>,
    generated_code_info: self::_puroro::internal::field_type::SingularHeapMessageField::<
        self::_puroro_root::google::protobuf::GeneratedCodeInfo,
    >,
    _bitfield: self::_puroro::bitvec::BitArray<1usize>,
}
impl File {
    pub fn name(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            0usize,
        > as NonRepeatedFieldType>::get_field(
            &self.name,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn name_opt(&self) -> ::std::option::Option::<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.name, &self._bitfield)
    }
    pub fn name_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            0usize,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.name,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_name(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.name, &self._bitfield)
            .is_some()
    }
    pub fn clear_name(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            0usize,
        > as NonRepeatedFieldType>::clear(&mut self.name, &mut self._bitfield)
    }
    pub fn insertion_point(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            1usize,
        > as NonRepeatedFieldType>::get_field(
            &self.insertion_point,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn insertion_point_opt(&self) -> ::std::option::Option::<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.insertion_point, &self._bitfield)
    }
    pub fn insertion_point_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            1usize,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.insertion_point,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_insertion_point(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.insertion_point, &self._bitfield)
            .is_some()
    }
    pub fn clear_insertion_point(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            1usize,
        > as NonRepeatedFieldType>::clear(&mut self.insertion_point, &mut self._bitfield)
    }
    pub fn content(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            2usize,
        > as NonRepeatedFieldType>::get_field(
            &self.content,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn content_opt(&self) -> ::std::option::Option::<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            2usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.content, &self._bitfield)
    }
    pub fn content_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            2usize,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.content,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_content(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            2usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.content, &self._bitfield)
            .is_some()
    }
    pub fn clear_content(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            2usize,
        > as NonRepeatedFieldType>::clear(&mut self.content, &mut self._bitfield)
    }
    pub fn generated_code_info(
        &self,
    ) -> ::std::option::Option::<
        &self::_puroro_root::google::protobuf::GeneratedCodeInfo,
    > {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField::<
            self::_puroro_root::google::protobuf::GeneratedCodeInfo,
        > as NonRepeatedFieldType>::get_field(
            &self.generated_code_info,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn generated_code_info_opt(
        &self,
    ) -> ::std::option::Option::<
        &self::_puroro_root::google::protobuf::GeneratedCodeInfo,
    > {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField::<
            self::_puroro_root::google::protobuf::GeneratedCodeInfo,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.generated_code_info,
            &self._bitfield,
        )
    }
    pub fn generated_code_info_mut(
        &mut self,
    ) -> &mut self::_puroro_root::google::protobuf::GeneratedCodeInfo {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField::<
            self::_puroro_root::google::protobuf::GeneratedCodeInfo,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.generated_code_info,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_generated_code_info(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField::<
            self::_puroro_root::google::protobuf::GeneratedCodeInfo,
        > as NonRepeatedFieldType>::get_field_opt(
                &self.generated_code_info,
                &self._bitfield,
            )
            .is_some()
    }
    pub fn clear_generated_code_info(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField::<
            self::_puroro_root::google::protobuf::GeneratedCodeInfo,
        > as NonRepeatedFieldType>::clear(
            &mut self.generated_code_info,
            &mut self._bitfield,
        )
    }
}
impl self::_puroro::Message for File {
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
                    <self::_puroro::internal::field_type::OptionalStringField::<
                        0usize,
                    > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.name,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                2i32 => {
                    <self::_puroro::internal::field_type::OptionalStringField::<
                        1usize,
                    > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.insertion_point,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                15i32 => {
                    <self::_puroro::internal::field_type::OptionalStringField::<
                        2usize,
                    > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.content,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                16i32 => {
                    <self::_puroro::internal::field_type::SingularHeapMessageField::<
                        self::_puroro_root::google::protobuf::GeneratedCodeInfo,
                    > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.generated_code_info,
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
        <self::_puroro::internal::field_type::OptionalStringField::<
            0usize,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.name,
            &self._bitfield,
            1i32,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalStringField::<
            1usize,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.insertion_point,
            &self._bitfield,
            2i32,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalStringField::<
            2usize,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.content,
            &self._bitfield,
            15i32,
            out,
        )?;
        <self::_puroro::internal::field_type::SingularHeapMessageField::<
            self::_puroro_root::google::protobuf::GeneratedCodeInfo,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.generated_code_info,
            &self._bitfield,
            16i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for File {
    fn clone(&self) -> Self {
        Self {
            name: <self::_puroro::internal::field_type::OptionalStringField::<
                0usize,
            > as ::std::clone::Clone>::clone(&self.name),
            insertion_point: <self::_puroro::internal::field_type::OptionalStringField::<
                1usize,
            > as ::std::clone::Clone>::clone(&self.insertion_point),
            content: <self::_puroro::internal::field_type::OptionalStringField::<
                2usize,
            > as ::std::clone::Clone>::clone(&self.content),
            generated_code_info: <self::_puroro::internal::field_type::SingularHeapMessageField::<
                self::_puroro_root::google::protobuf::GeneratedCodeInfo,
            > as ::std::clone::Clone>::clone(&self.generated_code_info),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
impl ::std::ops::Drop for File {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for File {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(File))
            .field(stringify!(name), &self.name_opt())
            .field(stringify!(insertion_point), &self.insertion_point_opt())
            .field(stringify!(content), &self.content_opt())
            .field(stringify!(generated_code_info), &self.generated_code_info_opt())
            .finish()
    }
}
impl ::std::cmp::PartialEq for File {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion as _;
        true && self.name_opt() == rhs.name_opt()
            && self.insertion_point_opt() == rhs.insertion_point_opt()
            && self.content_opt() == rhs.content_opt()
            && self.generated_code_info_opt() == rhs.generated_code_info_opt()
    }
}
#[derive(
    ::std::clone::Clone,
    ::std::marker::Copy,
    ::std::cmp::PartialEq,
    ::std::cmp::Eq,
    ::std::cmp::PartialOrd,
    ::std::cmp::Ord,
    ::std::hash::Hash,
    ::std::fmt::Debug,
)]
pub enum Feature {
    FeatureNone,
    FeatureProto3Optional,
}
impl ::std::default::Default for Feature {
    fn default() -> Self {
        Self::FeatureNone
    }
}
impl ::std::convert::From::<Feature> for i32 {
    fn from(val: Feature) -> i32 {
        match val {
            Feature::FeatureNone => 0i32,
            Feature::FeatureProto3Optional => 1i32,
        }
    }
}
impl ::std::convert::TryFrom::<i32> for Feature {
    type Error = self::_puroro::PuroroError;
    fn try_from(val: i32) -> ::std::result::Result<Self, Self::Error> {
        match val {
            0i32 => ::std::result::Result::Ok(self::Feature::FeatureNone),
            1i32 => ::std::result::Result::Ok(self::Feature::FeatureProto3Optional),
            _ => {
                ::std::result::Result::Err(
                    self::_puroro::ErrorKind::UnknownEnumVariant(val),
                )?
            }
        }
    }
}
