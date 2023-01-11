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
pub struct File {
    fields: self::_root::google::protobuf::compiler::code_generator_response::_fields::FileFields<
        self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        >,
        self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            1usize,
        >,
        self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            2usize,
        >,
        self::_pinternal::SingularHeapMessageField::<
            self::_root::google::protobuf::GeneratedCodeInfo,
        >,
    >,
    bitfield: self::_pinternal::BitArray<1usize>,
    unknown_fields: self::_pinternal::UnknownFieldsImpl,
}
impl File {
    pub fn name(&self) -> &str {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.name,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn name_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.name, &self.bitfield)
    }
    pub fn name_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.name,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_name(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.name, &self.bitfield).is_some()
    }
    pub fn clear_name(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.name, &mut self.bitfield)
    }
    pub fn insertion_point(&self) -> &str {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.insertion_point,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn insertion_point_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.insertion_point, &self.bitfield)
    }
    pub fn insertion_point_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.insertion_point,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_insertion_point(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.insertion_point, &self.bitfield)
            .is_some()
    }
    pub fn clear_insertion_point(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.insertion_point, &mut self.bitfield)
    }
    pub fn content(&self) -> &str {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.content,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn content_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.content, &self.bitfield)
    }
    pub fn content_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.content,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_content(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.content, &self.bitfield)
            .is_some()
    }
    pub fn clear_content(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.content, &mut self.bitfield)
    }
    pub fn generated_code_info(
        &self,
    ) -> ::std::option::Option::<&self::_root::google::protobuf::GeneratedCodeInfo> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.generated_code_info,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn generated_code_info_opt(
        &self,
    ) -> ::std::option::Option::<&self::_root::google::protobuf::GeneratedCodeInfo> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
            &self.fields.generated_code_info,
            &self.bitfield,
        )
    }
    pub fn generated_code_info_mut(
        &mut self,
    ) -> &mut self::_root::google::protobuf::GeneratedCodeInfo {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.generated_code_info,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_generated_code_info(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
                &self.fields.generated_code_info,
                &self.bitfield,
            )
            .is_some()
    }
    pub fn clear_generated_code_info(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(
            &mut self.fields.generated_code_info,
            &mut self.bitfield,
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
        use self::_pinternal::ser::FieldData;
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            match number {
                1i32 => {
                    self::_pinternal::FieldType::deser_from_iter(
                        &mut self.fields.name,
                        &mut self.bitfield,
                        field_data,
                    )?
                }
                2i32 => {
                    self::_pinternal::FieldType::deser_from_iter(
                        &mut self.fields.insertion_point,
                        &mut self.bitfield,
                        field_data,
                    )?
                }
                15i32 => {
                    self::_pinternal::FieldType::deser_from_iter(
                        &mut self.fields.content,
                        &mut self.bitfield,
                        field_data,
                    )?
                }
                16i32 => {
                    self::_pinternal::FieldType::deser_from_iter(
                        &mut self.fields.generated_code_info,
                        &mut self.bitfield,
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
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.name,
            &self.bitfield,
            1i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.insertion_point,
            &self.bitfield,
            2i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.content,
            &self.bitfield,
            15i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.generated_code_info,
            &self.bitfield,
            16i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for File {
    fn clone(&self) -> Self {
        Self {
            fields: self::_fields::FileFields {
                name: ::std::clone::Clone::clone(&self.fields.name),
                insertion_point: ::std::clone::Clone::clone(
                    &self.fields.insertion_point,
                ),
                content: ::std::clone::Clone::clone(&self.fields.content),
                generated_code_info: ::std::clone::Clone::clone(
                    &self.fields.generated_code_info,
                ),
            },
            bitfield: ::std::clone::Clone::clone(&self.bitfield),
            unknown_fields: ::std::clone::Clone::clone(&self.unknown_fields),
        }
    }
}
impl ::std::ops::Drop for File {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for File {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        use self::_pinternal::UnknownFields as _;
        let mut debug_struct = fmt.debug_struct(stringify!(File));
        debug_struct
            .field(stringify!(name), &self.name_opt())
            .field(stringify!(insertion_point), &self.insertion_point_opt())
            .field(stringify!(content), &self.content_opt())
            .field(stringify!(generated_code_info), &self.generated_code_info_opt());
        self.unknown_fields.debug_struct_fields(&mut debug_struct)?;
        debug_struct.finish()
    }
}
impl ::std::cmp::PartialEq for File {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        true && self.name_opt() == rhs.name_opt()
            && self.insertion_point_opt() == rhs.insertion_point_opt()
            && self.content_opt() == rhs.content_opt()
            && self.generated_code_info_opt() == rhs.generated_code_info_opt()
            && self.unknown_fields == rhs.unknown_fields
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
    pub struct FileFields<TName, TInsertionPoint, TContent, TGeneratedCodeInfo> {
        pub name: TName,
        pub insertion_point: TInsertionPoint,
        pub content: TContent,
        pub generated_code_info: TGeneratedCodeInfo,
    }
}
pub use self::_fields::*;
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
