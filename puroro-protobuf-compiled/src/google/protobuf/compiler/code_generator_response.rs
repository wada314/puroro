pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}
pub mod _puroro {
    pub use ::puroro::*;
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
#[derive(
    ::std::clone::Clone,
    ::std::marker::Copy,
    ::std::cmp::PartialEq,
    ::std::cmp::Eq,
    ::std::cmp::PartialOrd,
    ::std::cmp::Ord,
    ::std::hash::Hash,
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
        use ::std::result::Result::{Ok, Err};
        match val {
            0i32 => Ok(self::Feature::FeatureNone),
            1i32 => Ok(self::Feature::FeatureProto3Optional),
            _ => Err(self::_puroro::ErrorKind::UnknownEnumVariant(val))?,
        }
    }
}
