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
pub mod compiler;
pub mod descriptor_proto;
pub mod enum_descriptor_proto;
pub mod field_descriptor_proto;
pub mod field_options;
pub mod file_options;
pub mod generated_code_info;
pub mod method_options;
pub mod source_code_info;
pub mod uninterpreted_option;
#[derive(::std::default::Default)]
pub struct FileDescriptorSet {
    fields: self::_root::google::protobuf::_fields::FileDescriptorSetFields<
        self::_pinternal::RepeatedMessageField::<
            self::_root::google::protobuf::FileDescriptorProto,
        >,
    >,
    bitfield: self::_pinternal::BitArray<0usize>,
    unknown_fields: self::_pinternal::UnknownFieldsImpl,
}
impl FileDescriptorSet {
    pub fn file(&self) -> &[self::_root::google::protobuf::FileDescriptorProto] {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field(&self.fields.file, &self.bitfield)
    }
    pub fn file_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::FileDescriptorProto> {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field_mut(&mut self.fields.file, &mut self.bitfield)
    }
    pub fn clear_file(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::clear(&mut self.fields.file, &mut self.bitfield)
    }
}
impl self::_puroro::Message for FileDescriptorSet {
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
        use self::_pinternal::UnknownFields as _;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        use self::_puroro::PuroroError;
        while let Some((number, mut field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    1i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.file,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    _ => Err(PuroroError::UnknownFieldNumber)?,
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(
                    PuroroError::UnknownFieldNumber | PuroroError::UnknownEnumVariant(_),
                ) => {
                    self.unknown_fields.push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::UnknownFields as _;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.file,
            &self.bitfield,
            1i32,
            out,
        )?;
        self.unknown_fields.ser_to_write(out)?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for FileDescriptorSet {
    fn clone(&self) -> Self {
        Self {
            fields: self::_fields::FileDescriptorSetFields {
                file: ::std::clone::Clone::clone(&self.fields.file),
            },
            bitfield: ::std::clone::Clone::clone(&self.bitfield),
            unknown_fields: ::std::clone::Clone::clone(&self.unknown_fields),
        }
    }
}
impl ::std::ops::Drop for FileDescriptorSet {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for FileDescriptorSet {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        use self::_pinternal::UnknownFields as _;
        let mut debug_struct = fmt.debug_struct(stringify!(FileDescriptorSet));
        debug_struct.field(stringify!(file), &self.file());
        self.unknown_fields.debug_struct_fields(&mut debug_struct)?;
        debug_struct.finish()
    }
}
impl ::std::cmp::PartialEq for FileDescriptorSet {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        true && self.file() == rhs.file() && self.unknown_fields == rhs.unknown_fields
    }
}
#[derive(::std::default::Default)]
pub struct FileDescriptorProto {
    fields: self::_root::google::protobuf::_fields::FileDescriptorProtoFields<
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
        self::_pinternal::RepeatedUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        >,
        self::_pinternal::RepeatedNumericalField::<i32, self::_pinternal::tags::Int32>,
        self::_pinternal::RepeatedNumericalField::<i32, self::_pinternal::tags::Int32>,
        self::_pinternal::RepeatedMessageField::<
            self::_root::google::protobuf::DescriptorProto,
        >,
        self::_pinternal::RepeatedMessageField::<
            self::_root::google::protobuf::EnumDescriptorProto,
        >,
        self::_pinternal::RepeatedMessageField::<
            self::_root::google::protobuf::ServiceDescriptorProto,
        >,
        self::_pinternal::RepeatedMessageField::<
            self::_root::google::protobuf::FieldDescriptorProto,
        >,
        self::_pinternal::SingularHeapMessageField::<
            self::_root::google::protobuf::FileOptions,
        >,
        self::_pinternal::SingularHeapMessageField::<
            self::_root::google::protobuf::SourceCodeInfo,
        >,
        self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            2usize,
        >,
    >,
    bitfield: self::_pinternal::BitArray<1usize>,
    unknown_fields: self::_pinternal::UnknownFieldsImpl,
}
impl FileDescriptorProto {
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
    pub fn package(&self) -> &str {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.package,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn package_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.package, &self.bitfield)
    }
    pub fn package_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.package,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_package(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.package, &self.bitfield)
            .is_some()
    }
    pub fn clear_package(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.package, &mut self.bitfield)
    }
    pub fn dependency(
        &self,
    ) -> &[impl ::std::ops::Deref::<
        Target = str,
    > + ::std::fmt::Debug + ::std::cmp::PartialEq] {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field(&self.fields.dependency, &self.bitfield)
    }
    pub fn dependency_mut(&mut self) -> &mut ::std::vec::Vec::<::std::string::String> {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field_mut(&mut self.fields.dependency, &mut self.bitfield)
    }
    pub fn clear_dependency(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::clear(&mut self.fields.dependency, &mut self.bitfield)
    }
    pub fn public_dependency(&self) -> &[i32] {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field(&self.fields.public_dependency, &self.bitfield)
    }
    pub fn public_dependency_mut(&mut self) -> &mut ::std::vec::Vec::<i32> {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field_mut(
            &mut self.fields.public_dependency,
            &mut self.bitfield,
        )
    }
    pub fn clear_public_dependency(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::clear(&mut self.fields.public_dependency, &mut self.bitfield)
    }
    pub fn weak_dependency(&self) -> &[i32] {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field(&self.fields.weak_dependency, &self.bitfield)
    }
    pub fn weak_dependency_mut(&mut self) -> &mut ::std::vec::Vec::<i32> {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field_mut(
            &mut self.fields.weak_dependency,
            &mut self.bitfield,
        )
    }
    pub fn clear_weak_dependency(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::clear(&mut self.fields.weak_dependency, &mut self.bitfield)
    }
    pub fn message_type(&self) -> &[self::_root::google::protobuf::DescriptorProto] {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field(&self.fields.message_type, &self.bitfield)
    }
    pub fn message_type_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::DescriptorProto> {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field_mut(
            &mut self.fields.message_type,
            &mut self.bitfield,
        )
    }
    pub fn clear_message_type(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::clear(&mut self.fields.message_type, &mut self.bitfield)
    }
    pub fn enum_type(&self) -> &[self::_root::google::protobuf::EnumDescriptorProto] {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field(&self.fields.enum_type, &self.bitfield)
    }
    pub fn enum_type_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::EnumDescriptorProto> {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field_mut(&mut self.fields.enum_type, &mut self.bitfield)
    }
    pub fn clear_enum_type(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::clear(&mut self.fields.enum_type, &mut self.bitfield)
    }
    pub fn service(&self) -> &[self::_root::google::protobuf::ServiceDescriptorProto] {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field(&self.fields.service, &self.bitfield)
    }
    pub fn service_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::ServiceDescriptorProto> {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field_mut(&mut self.fields.service, &mut self.bitfield)
    }
    pub fn clear_service(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::clear(&mut self.fields.service, &mut self.bitfield)
    }
    pub fn extension(&self) -> &[self::_root::google::protobuf::FieldDescriptorProto] {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field(&self.fields.extension, &self.bitfield)
    }
    pub fn extension_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::FieldDescriptorProto> {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field_mut(&mut self.fields.extension, &mut self.bitfield)
    }
    pub fn clear_extension(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::clear(&mut self.fields.extension, &mut self.bitfield)
    }
    pub fn options(
        &self,
    ) -> ::std::option::Option::<&self::_root::google::protobuf::FileOptions> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.options,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn options_opt(
        &self,
    ) -> ::std::option::Option::<&self::_root::google::protobuf::FileOptions> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.options, &self.bitfield)
    }
    pub fn options_mut(&mut self) -> &mut self::_root::google::protobuf::FileOptions {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.options,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_options(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.options, &self.bitfield)
            .is_some()
    }
    pub fn clear_options(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.options, &mut self.bitfield)
    }
    pub fn source_code_info(
        &self,
    ) -> ::std::option::Option::<&self::_root::google::protobuf::SourceCodeInfo> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.source_code_info,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn source_code_info_opt(
        &self,
    ) -> ::std::option::Option::<&self::_root::google::protobuf::SourceCodeInfo> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
            &self.fields.source_code_info,
            &self.bitfield,
        )
    }
    pub fn source_code_info_mut(
        &mut self,
    ) -> &mut self::_root::google::protobuf::SourceCodeInfo {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.source_code_info,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_source_code_info(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
                &self.fields.source_code_info,
                &self.bitfield,
            )
            .is_some()
    }
    pub fn clear_source_code_info(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(
            &mut self.fields.source_code_info,
            &mut self.bitfield,
        )
    }
    pub fn syntax(&self) -> &str {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.syntax,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn syntax_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.syntax, &self.bitfield)
    }
    pub fn syntax_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.syntax,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_syntax(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.syntax, &self.bitfield)
            .is_some()
    }
    pub fn clear_syntax(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.syntax, &mut self.bitfield)
    }
}
impl self::_puroro::Message for FileDescriptorProto {
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
        use self::_pinternal::UnknownFields as _;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        use self::_puroro::PuroroError;
        while let Some((number, mut field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    1i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.name,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    2i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.package,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    3i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.dependency,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    10i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.public_dependency,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    11i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.weak_dependency,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    4i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.message_type,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    5i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.enum_type,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    6i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.service,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    7i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.extension,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    8i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.options,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    9i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.source_code_info,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    12i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.syntax,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    _ => Err(PuroroError::UnknownFieldNumber)?,
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(
                    PuroroError::UnknownFieldNumber | PuroroError::UnknownEnumVariant(_),
                ) => {
                    self.unknown_fields.push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::UnknownFields as _;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.name,
            &self.bitfield,
            1i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.package,
            &self.bitfield,
            2i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.dependency,
            &self.bitfield,
            3i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.public_dependency,
            &self.bitfield,
            10i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.weak_dependency,
            &self.bitfield,
            11i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.message_type,
            &self.bitfield,
            4i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.enum_type,
            &self.bitfield,
            5i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.service,
            &self.bitfield,
            6i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.extension,
            &self.bitfield,
            7i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.options,
            &self.bitfield,
            8i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.source_code_info,
            &self.bitfield,
            9i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.syntax,
            &self.bitfield,
            12i32,
            out,
        )?;
        self.unknown_fields.ser_to_write(out)?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for FileDescriptorProto {
    fn clone(&self) -> Self {
        Self {
            fields: self::_fields::FileDescriptorProtoFields {
                name: ::std::clone::Clone::clone(&self.fields.name),
                package: ::std::clone::Clone::clone(&self.fields.package),
                dependency: ::std::clone::Clone::clone(&self.fields.dependency),
                public_dependency: ::std::clone::Clone::clone(
                    &self.fields.public_dependency,
                ),
                weak_dependency: ::std::clone::Clone::clone(
                    &self.fields.weak_dependency,
                ),
                message_type: ::std::clone::Clone::clone(&self.fields.message_type),
                enum_type: ::std::clone::Clone::clone(&self.fields.enum_type),
                service: ::std::clone::Clone::clone(&self.fields.service),
                extension: ::std::clone::Clone::clone(&self.fields.extension),
                options: ::std::clone::Clone::clone(&self.fields.options),
                source_code_info: ::std::clone::Clone::clone(
                    &self.fields.source_code_info,
                ),
                syntax: ::std::clone::Clone::clone(&self.fields.syntax),
            },
            bitfield: ::std::clone::Clone::clone(&self.bitfield),
            unknown_fields: ::std::clone::Clone::clone(&self.unknown_fields),
        }
    }
}
impl ::std::ops::Drop for FileDescriptorProto {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for FileDescriptorProto {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        use self::_pinternal::UnknownFields as _;
        let mut debug_struct = fmt.debug_struct(stringify!(FileDescriptorProto));
        debug_struct
            .field(stringify!(name), &self.name_opt())
            .field(stringify!(package), &self.package_opt())
            .field(stringify!(dependency), &self.dependency())
            .field(stringify!(public_dependency), &self.public_dependency())
            .field(stringify!(weak_dependency), &self.weak_dependency())
            .field(stringify!(message_type), &self.message_type())
            .field(stringify!(enum_type), &self.enum_type())
            .field(stringify!(service), &self.service())
            .field(stringify!(extension), &self.extension())
            .field(stringify!(options), &self.options_opt())
            .field(stringify!(source_code_info), &self.source_code_info_opt())
            .field(stringify!(syntax), &self.syntax_opt());
        self.unknown_fields.debug_struct_fields(&mut debug_struct)?;
        debug_struct.finish()
    }
}
impl ::std::cmp::PartialEq for FileDescriptorProto {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        true && self.name_opt() == rhs.name_opt()
            && self.package_opt() == rhs.package_opt()
            && self.dependency() == rhs.dependency()
            && self.public_dependency() == rhs.public_dependency()
            && self.weak_dependency() == rhs.weak_dependency()
            && self.message_type() == rhs.message_type()
            && self.enum_type() == rhs.enum_type() && self.service() == rhs.service()
            && self.extension() == rhs.extension()
            && self.options_opt() == rhs.options_opt()
            && self.source_code_info_opt() == rhs.source_code_info_opt()
            && self.syntax_opt() == rhs.syntax_opt()
            && self.unknown_fields == rhs.unknown_fields
    }
}
#[derive(::std::default::Default)]
pub struct DescriptorProto {
    fields: self::_root::google::protobuf::_fields::DescriptorProtoFields<
        self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        >,
        self::_pinternal::RepeatedMessageField::<
            self::_root::google::protobuf::FieldDescriptorProto,
        >,
        self::_pinternal::RepeatedMessageField::<
            self::_root::google::protobuf::FieldDescriptorProto,
        >,
        self::_pinternal::RepeatedMessageField::<
            self::_root::google::protobuf::DescriptorProto,
        >,
        self::_pinternal::RepeatedMessageField::<
            self::_root::google::protobuf::EnumDescriptorProto,
        >,
        self::_pinternal::RepeatedMessageField::<
            self::_root::google::protobuf::descriptor_proto::ExtensionRange,
        >,
        self::_pinternal::RepeatedMessageField::<
            self::_root::google::protobuf::OneofDescriptorProto,
        >,
        self::_pinternal::SingularHeapMessageField::<
            self::_root::google::protobuf::MessageOptions,
        >,
        self::_pinternal::RepeatedMessageField::<
            self::_root::google::protobuf::descriptor_proto::ReservedRange,
        >,
        self::_pinternal::RepeatedUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        >,
    >,
    bitfield: self::_pinternal::BitArray<1usize>,
    unknown_fields: self::_pinternal::UnknownFieldsImpl,
}
impl DescriptorProto {
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
    pub fn field(&self) -> &[self::_root::google::protobuf::FieldDescriptorProto] {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field(&self.fields.field, &self.bitfield)
    }
    pub fn field_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::FieldDescriptorProto> {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field_mut(&mut self.fields.field, &mut self.bitfield)
    }
    pub fn clear_field(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::clear(&mut self.fields.field, &mut self.bitfield)
    }
    pub fn extension(&self) -> &[self::_root::google::protobuf::FieldDescriptorProto] {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field(&self.fields.extension, &self.bitfield)
    }
    pub fn extension_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::FieldDescriptorProto> {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field_mut(&mut self.fields.extension, &mut self.bitfield)
    }
    pub fn clear_extension(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::clear(&mut self.fields.extension, &mut self.bitfield)
    }
    pub fn nested_type(&self) -> &[self::_root::google::protobuf::DescriptorProto] {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field(&self.fields.nested_type, &self.bitfield)
    }
    pub fn nested_type_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::DescriptorProto> {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field_mut(
            &mut self.fields.nested_type,
            &mut self.bitfield,
        )
    }
    pub fn clear_nested_type(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::clear(&mut self.fields.nested_type, &mut self.bitfield)
    }
    pub fn enum_type(&self) -> &[self::_root::google::protobuf::EnumDescriptorProto] {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field(&self.fields.enum_type, &self.bitfield)
    }
    pub fn enum_type_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::EnumDescriptorProto> {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field_mut(&mut self.fields.enum_type, &mut self.bitfield)
    }
    pub fn clear_enum_type(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::clear(&mut self.fields.enum_type, &mut self.bitfield)
    }
    pub fn extension_range(
        &self,
    ) -> &[self::_root::google::protobuf::descriptor_proto::ExtensionRange] {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field(&self.fields.extension_range, &self.bitfield)
    }
    pub fn extension_range_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<
        self::_root::google::protobuf::descriptor_proto::ExtensionRange,
    > {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field_mut(
            &mut self.fields.extension_range,
            &mut self.bitfield,
        )
    }
    pub fn clear_extension_range(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::clear(&mut self.fields.extension_range, &mut self.bitfield)
    }
    pub fn oneof_decl(&self) -> &[self::_root::google::protobuf::OneofDescriptorProto] {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field(&self.fields.oneof_decl, &self.bitfield)
    }
    pub fn oneof_decl_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::OneofDescriptorProto> {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field_mut(&mut self.fields.oneof_decl, &mut self.bitfield)
    }
    pub fn clear_oneof_decl(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::clear(&mut self.fields.oneof_decl, &mut self.bitfield)
    }
    pub fn options(
        &self,
    ) -> ::std::option::Option::<&self::_root::google::protobuf::MessageOptions> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.options,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn options_opt(
        &self,
    ) -> ::std::option::Option::<&self::_root::google::protobuf::MessageOptions> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.options, &self.bitfield)
    }
    pub fn options_mut(&mut self) -> &mut self::_root::google::protobuf::MessageOptions {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.options,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_options(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.options, &self.bitfield)
            .is_some()
    }
    pub fn clear_options(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.options, &mut self.bitfield)
    }
    pub fn reserved_range(
        &self,
    ) -> &[self::_root::google::protobuf::descriptor_proto::ReservedRange] {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field(&self.fields.reserved_range, &self.bitfield)
    }
    pub fn reserved_range_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<
        self::_root::google::protobuf::descriptor_proto::ReservedRange,
    > {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field_mut(
            &mut self.fields.reserved_range,
            &mut self.bitfield,
        )
    }
    pub fn clear_reserved_range(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::clear(&mut self.fields.reserved_range, &mut self.bitfield)
    }
    pub fn reserved_name(
        &self,
    ) -> &[impl ::std::ops::Deref::<
        Target = str,
    > + ::std::fmt::Debug + ::std::cmp::PartialEq] {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field(&self.fields.reserved_name, &self.bitfield)
    }
    pub fn reserved_name_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<::std::string::String> {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field_mut(
            &mut self.fields.reserved_name,
            &mut self.bitfield,
        )
    }
    pub fn clear_reserved_name(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::clear(&mut self.fields.reserved_name, &mut self.bitfield)
    }
}
impl self::_puroro::Message for DescriptorProto {
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
        use self::_pinternal::UnknownFields as _;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        use self::_puroro::PuroroError;
        while let Some((number, mut field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    1i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.name,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    2i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.field,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    6i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.extension,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    3i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.nested_type,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    4i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.enum_type,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    5i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.extension_range,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    8i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.oneof_decl,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    7i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.options,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    9i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.reserved_range,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    10i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.reserved_name,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    _ => Err(PuroroError::UnknownFieldNumber)?,
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(
                    PuroroError::UnknownFieldNumber | PuroroError::UnknownEnumVariant(_),
                ) => {
                    self.unknown_fields.push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::UnknownFields as _;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.name,
            &self.bitfield,
            1i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.field,
            &self.bitfield,
            2i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.extension,
            &self.bitfield,
            6i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.nested_type,
            &self.bitfield,
            3i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.enum_type,
            &self.bitfield,
            4i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.extension_range,
            &self.bitfield,
            5i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.oneof_decl,
            &self.bitfield,
            8i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.options,
            &self.bitfield,
            7i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.reserved_range,
            &self.bitfield,
            9i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.reserved_name,
            &self.bitfield,
            10i32,
            out,
        )?;
        self.unknown_fields.ser_to_write(out)?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for DescriptorProto {
    fn clone(&self) -> Self {
        Self {
            fields: self::_fields::DescriptorProtoFields {
                name: ::std::clone::Clone::clone(&self.fields.name),
                field: ::std::clone::Clone::clone(&self.fields.field),
                extension: ::std::clone::Clone::clone(&self.fields.extension),
                nested_type: ::std::clone::Clone::clone(&self.fields.nested_type),
                enum_type: ::std::clone::Clone::clone(&self.fields.enum_type),
                extension_range: ::std::clone::Clone::clone(
                    &self.fields.extension_range,
                ),
                oneof_decl: ::std::clone::Clone::clone(&self.fields.oneof_decl),
                options: ::std::clone::Clone::clone(&self.fields.options),
                reserved_range: ::std::clone::Clone::clone(&self.fields.reserved_range),
                reserved_name: ::std::clone::Clone::clone(&self.fields.reserved_name),
            },
            bitfield: ::std::clone::Clone::clone(&self.bitfield),
            unknown_fields: ::std::clone::Clone::clone(&self.unknown_fields),
        }
    }
}
impl ::std::ops::Drop for DescriptorProto {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for DescriptorProto {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        use self::_pinternal::UnknownFields as _;
        let mut debug_struct = fmt.debug_struct(stringify!(DescriptorProto));
        debug_struct
            .field(stringify!(name), &self.name_opt())
            .field(stringify!(field), &self.field())
            .field(stringify!(extension), &self.extension())
            .field(stringify!(nested_type), &self.nested_type())
            .field(stringify!(enum_type), &self.enum_type())
            .field(stringify!(extension_range), &self.extension_range())
            .field(stringify!(oneof_decl), &self.oneof_decl())
            .field(stringify!(options), &self.options_opt())
            .field(stringify!(reserved_range), &self.reserved_range())
            .field(stringify!(reserved_name), &self.reserved_name());
        self.unknown_fields.debug_struct_fields(&mut debug_struct)?;
        debug_struct.finish()
    }
}
impl ::std::cmp::PartialEq for DescriptorProto {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        true && self.name_opt() == rhs.name_opt() && self.field() == rhs.field()
            && self.extension() == rhs.extension()
            && self.nested_type() == rhs.nested_type()
            && self.enum_type() == rhs.enum_type()
            && self.extension_range() == rhs.extension_range()
            && self.oneof_decl() == rhs.oneof_decl()
            && self.options_opt() == rhs.options_opt()
            && self.reserved_range() == rhs.reserved_range()
            && self.reserved_name() == rhs.reserved_name()
            && self.unknown_fields == rhs.unknown_fields
    }
}
#[derive(::std::default::Default)]
pub struct ExtensionRangeOptions {
    fields: self::_root::google::protobuf::_fields::ExtensionRangeOptionsFields<
        self::_pinternal::RepeatedMessageField::<
            self::_root::google::protobuf::UninterpretedOption,
        >,
    >,
    bitfield: self::_pinternal::BitArray<0usize>,
    unknown_fields: self::_pinternal::UnknownFieldsImpl,
}
impl ExtensionRangeOptions {
    pub fn uninterpreted_option(
        &self,
    ) -> &[self::_root::google::protobuf::UninterpretedOption] {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field(&self.fields.uninterpreted_option, &self.bitfield)
    }
    pub fn uninterpreted_option_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::UninterpretedOption> {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field_mut(
            &mut self.fields.uninterpreted_option,
            &mut self.bitfield,
        )
    }
    pub fn clear_uninterpreted_option(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::clear(
            &mut self.fields.uninterpreted_option,
            &mut self.bitfield,
        )
    }
}
impl self::_puroro::Message for ExtensionRangeOptions {
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
        use self::_pinternal::UnknownFields as _;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        use self::_puroro::PuroroError;
        while let Some((number, mut field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    999i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.uninterpreted_option,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    _ => Err(PuroroError::UnknownFieldNumber)?,
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(
                    PuroroError::UnknownFieldNumber | PuroroError::UnknownEnumVariant(_),
                ) => {
                    self.unknown_fields.push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::UnknownFields as _;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.uninterpreted_option,
            &self.bitfield,
            999i32,
            out,
        )?;
        self.unknown_fields.ser_to_write(out)?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for ExtensionRangeOptions {
    fn clone(&self) -> Self {
        Self {
            fields: self::_fields::ExtensionRangeOptionsFields {
                uninterpreted_option: ::std::clone::Clone::clone(
                    &self.fields.uninterpreted_option,
                ),
            },
            bitfield: ::std::clone::Clone::clone(&self.bitfield),
            unknown_fields: ::std::clone::Clone::clone(&self.unknown_fields),
        }
    }
}
impl ::std::ops::Drop for ExtensionRangeOptions {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for ExtensionRangeOptions {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        use self::_pinternal::UnknownFields as _;
        let mut debug_struct = fmt.debug_struct(stringify!(ExtensionRangeOptions));
        debug_struct
            .field(stringify!(uninterpreted_option), &self.uninterpreted_option());
        self.unknown_fields.debug_struct_fields(&mut debug_struct)?;
        debug_struct.finish()
    }
}
impl ::std::cmp::PartialEq for ExtensionRangeOptions {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        true && self.uninterpreted_option() == rhs.uninterpreted_option()
            && self.unknown_fields == rhs.unknown_fields
    }
}
#[derive(::std::default::Default)]
pub struct FieldDescriptorProto {
    fields: self::_root::google::protobuf::_fields::FieldDescriptorProtoFields<
        self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        >,
        self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            1usize,
        >,
        self::_pinternal::OptionalNumericalField::<
            self::_root::google::protobuf::field_descriptor_proto::Label,
            self::_pinternal::tags::Enum2::<
                self::_root::google::protobuf::field_descriptor_proto::Label,
            >,
            2usize,
        >,
        self::_pinternal::OptionalNumericalField::<
            self::_root::google::protobuf::field_descriptor_proto::Type,
            self::_pinternal::tags::Enum2::<
                self::_root::google::protobuf::field_descriptor_proto::Type,
            >,
            3usize,
        >,
        self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            4usize,
        >,
        self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            5usize,
        >,
        self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            6usize,
        >,
        self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            7usize,
        >,
        self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            8usize,
        >,
        self::_pinternal::SingularHeapMessageField::<
            self::_root::google::protobuf::FieldOptions,
        >,
        self::_pinternal::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            9usize,
        >,
    >,
    bitfield: self::_pinternal::BitArray<1usize>,
    unknown_fields: self::_pinternal::UnknownFieldsImpl,
}
impl FieldDescriptorProto {
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
    pub fn number(&self) -> i32 {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.number,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn number_opt(&self) -> ::std::option::Option::<i32> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.number, &self.bitfield)
    }
    pub fn number_mut(&mut self) -> &mut i32 {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.number,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_number(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.number, &self.bitfield)
            .is_some()
    }
    pub fn clear_number(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.number, &mut self.bitfield)
    }
    pub fn label(&self) -> self::_root::google::protobuf::field_descriptor_proto::Label {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.label,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn label_opt(
        &self,
    ) -> ::std::option::Option::<
        self::_root::google::protobuf::field_descriptor_proto::Label,
    > {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.label, &self.bitfield)
    }
    pub fn label_mut(
        &mut self,
    ) -> &mut self::_root::google::protobuf::field_descriptor_proto::Label {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.label,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_label(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.label, &self.bitfield).is_some()
    }
    pub fn clear_label(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.label, &mut self.bitfield)
    }
    pub fn r#type(&self) -> self::_root::google::protobuf::field_descriptor_proto::Type {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.r#type,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn type_opt(
        &self,
    ) -> ::std::option::Option::<
        self::_root::google::protobuf::field_descriptor_proto::Type,
    > {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.r#type, &self.bitfield)
    }
    pub fn type_mut(
        &mut self,
    ) -> &mut self::_root::google::protobuf::field_descriptor_proto::Type {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.r#type,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_type(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.r#type, &self.bitfield)
            .is_some()
    }
    pub fn clear_type(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.r#type, &mut self.bitfield)
    }
    pub fn type_name(&self) -> &str {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.type_name,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn type_name_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.type_name, &self.bitfield)
    }
    pub fn type_name_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.type_name,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_type_name(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.type_name, &self.bitfield)
            .is_some()
    }
    pub fn clear_type_name(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.type_name, &mut self.bitfield)
    }
    pub fn extendee(&self) -> &str {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.extendee,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn extendee_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.extendee, &self.bitfield)
    }
    pub fn extendee_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.extendee,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_extendee(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.extendee, &self.bitfield)
            .is_some()
    }
    pub fn clear_extendee(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.extendee, &mut self.bitfield)
    }
    pub fn default_value(&self) -> &str {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.default_value,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn default_value_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.default_value, &self.bitfield)
    }
    pub fn default_value_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.default_value,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_default_value(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.default_value, &self.bitfield)
            .is_some()
    }
    pub fn clear_default_value(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.default_value, &mut self.bitfield)
    }
    pub fn oneof_index(&self) -> i32 {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.oneof_index,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn oneof_index_opt(&self) -> ::std::option::Option::<i32> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.oneof_index, &self.bitfield)
    }
    pub fn oneof_index_mut(&mut self) -> &mut i32 {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.oneof_index,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_oneof_index(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.oneof_index, &self.bitfield)
            .is_some()
    }
    pub fn clear_oneof_index(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.oneof_index, &mut self.bitfield)
    }
    pub fn json_name(&self) -> &str {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.json_name,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn json_name_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.json_name, &self.bitfield)
    }
    pub fn json_name_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.json_name,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_json_name(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.json_name, &self.bitfield)
            .is_some()
    }
    pub fn clear_json_name(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.json_name, &mut self.bitfield)
    }
    pub fn options(
        &self,
    ) -> ::std::option::Option::<&self::_root::google::protobuf::FieldOptions> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.options,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn options_opt(
        &self,
    ) -> ::std::option::Option::<&self::_root::google::protobuf::FieldOptions> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.options, &self.bitfield)
    }
    pub fn options_mut(&mut self) -> &mut self::_root::google::protobuf::FieldOptions {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.options,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_options(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.options, &self.bitfield)
            .is_some()
    }
    pub fn clear_options(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.options, &mut self.bitfield)
    }
    pub fn proto3_optional(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.proto3_optional,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn proto3_optional_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.proto3_optional, &self.bitfield)
    }
    pub fn proto3_optional_mut(&mut self) -> &mut bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.proto3_optional,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_proto3_optional(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.proto3_optional, &self.bitfield)
            .is_some()
    }
    pub fn clear_proto3_optional(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.proto3_optional, &mut self.bitfield)
    }
}
impl self::_puroro::Message for FieldDescriptorProto {
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
        use self::_pinternal::UnknownFields as _;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        use self::_puroro::PuroroError;
        while let Some((number, mut field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    1i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.name,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    3i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.number,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    4i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.label,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    5i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.r#type,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    6i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.type_name,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    2i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.extendee,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    7i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.default_value,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    9i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.oneof_index,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    10i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.json_name,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    8i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.options,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    17i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.proto3_optional,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    _ => Err(PuroroError::UnknownFieldNumber)?,
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(
                    PuroroError::UnknownFieldNumber | PuroroError::UnknownEnumVariant(_),
                ) => {
                    self.unknown_fields.push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::UnknownFields as _;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.name,
            &self.bitfield,
            1i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.number,
            &self.bitfield,
            3i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.label,
            &self.bitfield,
            4i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.r#type,
            &self.bitfield,
            5i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.type_name,
            &self.bitfield,
            6i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.extendee,
            &self.bitfield,
            2i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.default_value,
            &self.bitfield,
            7i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.oneof_index,
            &self.bitfield,
            9i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.json_name,
            &self.bitfield,
            10i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.options,
            &self.bitfield,
            8i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.proto3_optional,
            &self.bitfield,
            17i32,
            out,
        )?;
        self.unknown_fields.ser_to_write(out)?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for FieldDescriptorProto {
    fn clone(&self) -> Self {
        Self {
            fields: self::_fields::FieldDescriptorProtoFields {
                name: ::std::clone::Clone::clone(&self.fields.name),
                number: ::std::clone::Clone::clone(&self.fields.number),
                label: ::std::clone::Clone::clone(&self.fields.label),
                r#type: ::std::clone::Clone::clone(&self.fields.r#type),
                type_name: ::std::clone::Clone::clone(&self.fields.type_name),
                extendee: ::std::clone::Clone::clone(&self.fields.extendee),
                default_value: ::std::clone::Clone::clone(&self.fields.default_value),
                oneof_index: ::std::clone::Clone::clone(&self.fields.oneof_index),
                json_name: ::std::clone::Clone::clone(&self.fields.json_name),
                options: ::std::clone::Clone::clone(&self.fields.options),
                proto3_optional: ::std::clone::Clone::clone(&self.fields.proto3_optional),
            },
            bitfield: ::std::clone::Clone::clone(&self.bitfield),
            unknown_fields: ::std::clone::Clone::clone(&self.unknown_fields),
        }
    }
}
impl ::std::ops::Drop for FieldDescriptorProto {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for FieldDescriptorProto {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        use self::_pinternal::UnknownFields as _;
        let mut debug_struct = fmt.debug_struct(stringify!(FieldDescriptorProto));
        debug_struct
            .field(stringify!(name), &self.name_opt())
            .field(stringify!(number), &self.number_opt())
            .field(stringify!(label), &self.label_opt())
            .field(stringify!(r#type), &self.type_opt())
            .field(stringify!(type_name), &self.type_name_opt())
            .field(stringify!(extendee), &self.extendee_opt())
            .field(stringify!(default_value), &self.default_value_opt())
            .field(stringify!(oneof_index), &self.oneof_index_opt())
            .field(stringify!(json_name), &self.json_name_opt())
            .field(stringify!(options), &self.options_opt())
            .field(stringify!(proto3_optional), &self.proto3_optional_opt());
        self.unknown_fields.debug_struct_fields(&mut debug_struct)?;
        debug_struct.finish()
    }
}
impl ::std::cmp::PartialEq for FieldDescriptorProto {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        true && self.name_opt() == rhs.name_opt()
            && self.number_opt() == rhs.number_opt()
            && self.label_opt() == rhs.label_opt() && self.type_opt() == rhs.type_opt()
            && self.type_name_opt() == rhs.type_name_opt()
            && self.extendee_opt() == rhs.extendee_opt()
            && self.default_value_opt() == rhs.default_value_opt()
            && self.oneof_index_opt() == rhs.oneof_index_opt()
            && self.json_name_opt() == rhs.json_name_opt()
            && self.options_opt() == rhs.options_opt()
            && self.proto3_optional_opt() == rhs.proto3_optional_opt()
            && self.unknown_fields == rhs.unknown_fields
    }
}
#[derive(::std::default::Default)]
pub struct OneofDescriptorProto {
    fields: self::_root::google::protobuf::_fields::OneofDescriptorProtoFields<
        self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        >,
        self::_pinternal::SingularHeapMessageField::<
            self::_root::google::protobuf::OneofOptions,
        >,
    >,
    bitfield: self::_pinternal::BitArray<1usize>,
    unknown_fields: self::_pinternal::UnknownFieldsImpl,
}
impl OneofDescriptorProto {
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
    pub fn options(
        &self,
    ) -> ::std::option::Option::<&self::_root::google::protobuf::OneofOptions> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.options,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn options_opt(
        &self,
    ) -> ::std::option::Option::<&self::_root::google::protobuf::OneofOptions> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.options, &self.bitfield)
    }
    pub fn options_mut(&mut self) -> &mut self::_root::google::protobuf::OneofOptions {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.options,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_options(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.options, &self.bitfield)
            .is_some()
    }
    pub fn clear_options(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.options, &mut self.bitfield)
    }
}
impl self::_puroro::Message for OneofDescriptorProto {
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
        use self::_pinternal::UnknownFields as _;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        use self::_puroro::PuroroError;
        while let Some((number, mut field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    1i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.name,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    2i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.options,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    _ => Err(PuroroError::UnknownFieldNumber)?,
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(
                    PuroroError::UnknownFieldNumber | PuroroError::UnknownEnumVariant(_),
                ) => {
                    self.unknown_fields.push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::UnknownFields as _;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.name,
            &self.bitfield,
            1i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.options,
            &self.bitfield,
            2i32,
            out,
        )?;
        self.unknown_fields.ser_to_write(out)?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for OneofDescriptorProto {
    fn clone(&self) -> Self {
        Self {
            fields: self::_fields::OneofDescriptorProtoFields {
                name: ::std::clone::Clone::clone(&self.fields.name),
                options: ::std::clone::Clone::clone(&self.fields.options),
            },
            bitfield: ::std::clone::Clone::clone(&self.bitfield),
            unknown_fields: ::std::clone::Clone::clone(&self.unknown_fields),
        }
    }
}
impl ::std::ops::Drop for OneofDescriptorProto {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for OneofDescriptorProto {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        use self::_pinternal::UnknownFields as _;
        let mut debug_struct = fmt.debug_struct(stringify!(OneofDescriptorProto));
        debug_struct
            .field(stringify!(name), &self.name_opt())
            .field(stringify!(options), &self.options_opt());
        self.unknown_fields.debug_struct_fields(&mut debug_struct)?;
        debug_struct.finish()
    }
}
impl ::std::cmp::PartialEq for OneofDescriptorProto {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        true && self.name_opt() == rhs.name_opt()
            && self.options_opt() == rhs.options_opt()
            && self.unknown_fields == rhs.unknown_fields
    }
}
#[derive(::std::default::Default)]
pub struct EnumDescriptorProto {
    fields: self::_root::google::protobuf::_fields::EnumDescriptorProtoFields<
        self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        >,
        self::_pinternal::RepeatedMessageField::<
            self::_root::google::protobuf::EnumValueDescriptorProto,
        >,
        self::_pinternal::SingularHeapMessageField::<
            self::_root::google::protobuf::EnumOptions,
        >,
        self::_pinternal::RepeatedMessageField::<
            self::_root::google::protobuf::enum_descriptor_proto::EnumReservedRange,
        >,
        self::_pinternal::RepeatedUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        >,
    >,
    bitfield: self::_pinternal::BitArray<1usize>,
    unknown_fields: self::_pinternal::UnknownFieldsImpl,
}
impl EnumDescriptorProto {
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
    pub fn value(&self) -> &[self::_root::google::protobuf::EnumValueDescriptorProto] {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field(&self.fields.value, &self.bitfield)
    }
    pub fn value_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<
        self::_root::google::protobuf::EnumValueDescriptorProto,
    > {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field_mut(&mut self.fields.value, &mut self.bitfield)
    }
    pub fn clear_value(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::clear(&mut self.fields.value, &mut self.bitfield)
    }
    pub fn options(
        &self,
    ) -> ::std::option::Option::<&self::_root::google::protobuf::EnumOptions> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.options,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn options_opt(
        &self,
    ) -> ::std::option::Option::<&self::_root::google::protobuf::EnumOptions> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.options, &self.bitfield)
    }
    pub fn options_mut(&mut self) -> &mut self::_root::google::protobuf::EnumOptions {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.options,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_options(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.options, &self.bitfield)
            .is_some()
    }
    pub fn clear_options(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.options, &mut self.bitfield)
    }
    pub fn reserved_range(
        &self,
    ) -> &[self::_root::google::protobuf::enum_descriptor_proto::EnumReservedRange] {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field(&self.fields.reserved_range, &self.bitfield)
    }
    pub fn reserved_range_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<
        self::_root::google::protobuf::enum_descriptor_proto::EnumReservedRange,
    > {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field_mut(
            &mut self.fields.reserved_range,
            &mut self.bitfield,
        )
    }
    pub fn clear_reserved_range(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::clear(&mut self.fields.reserved_range, &mut self.bitfield)
    }
    pub fn reserved_name(
        &self,
    ) -> &[impl ::std::ops::Deref::<
        Target = str,
    > + ::std::fmt::Debug + ::std::cmp::PartialEq] {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field(&self.fields.reserved_name, &self.bitfield)
    }
    pub fn reserved_name_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<::std::string::String> {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field_mut(
            &mut self.fields.reserved_name,
            &mut self.bitfield,
        )
    }
    pub fn clear_reserved_name(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::clear(&mut self.fields.reserved_name, &mut self.bitfield)
    }
}
impl self::_puroro::Message for EnumDescriptorProto {
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
        use self::_pinternal::UnknownFields as _;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        use self::_puroro::PuroroError;
        while let Some((number, mut field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    1i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.name,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    2i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.value,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    3i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.options,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    4i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.reserved_range,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    5i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.reserved_name,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    _ => Err(PuroroError::UnknownFieldNumber)?,
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(
                    PuroroError::UnknownFieldNumber | PuroroError::UnknownEnumVariant(_),
                ) => {
                    self.unknown_fields.push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::UnknownFields as _;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.name,
            &self.bitfield,
            1i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.value,
            &self.bitfield,
            2i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.options,
            &self.bitfield,
            3i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.reserved_range,
            &self.bitfield,
            4i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.reserved_name,
            &self.bitfield,
            5i32,
            out,
        )?;
        self.unknown_fields.ser_to_write(out)?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for EnumDescriptorProto {
    fn clone(&self) -> Self {
        Self {
            fields: self::_fields::EnumDescriptorProtoFields {
                name: ::std::clone::Clone::clone(&self.fields.name),
                value: ::std::clone::Clone::clone(&self.fields.value),
                options: ::std::clone::Clone::clone(&self.fields.options),
                reserved_range: ::std::clone::Clone::clone(&self.fields.reserved_range),
                reserved_name: ::std::clone::Clone::clone(&self.fields.reserved_name),
            },
            bitfield: ::std::clone::Clone::clone(&self.bitfield),
            unknown_fields: ::std::clone::Clone::clone(&self.unknown_fields),
        }
    }
}
impl ::std::ops::Drop for EnumDescriptorProto {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for EnumDescriptorProto {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        use self::_pinternal::UnknownFields as _;
        let mut debug_struct = fmt.debug_struct(stringify!(EnumDescriptorProto));
        debug_struct
            .field(stringify!(name), &self.name_opt())
            .field(stringify!(value), &self.value())
            .field(stringify!(options), &self.options_opt())
            .field(stringify!(reserved_range), &self.reserved_range())
            .field(stringify!(reserved_name), &self.reserved_name());
        self.unknown_fields.debug_struct_fields(&mut debug_struct)?;
        debug_struct.finish()
    }
}
impl ::std::cmp::PartialEq for EnumDescriptorProto {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        true && self.name_opt() == rhs.name_opt() && self.value() == rhs.value()
            && self.options_opt() == rhs.options_opt()
            && self.reserved_range() == rhs.reserved_range()
            && self.reserved_name() == rhs.reserved_name()
            && self.unknown_fields == rhs.unknown_fields
    }
}
#[derive(::std::default::Default)]
pub struct EnumValueDescriptorProto {
    fields: self::_root::google::protobuf::_fields::EnumValueDescriptorProtoFields<
        self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        >,
        self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            1usize,
        >,
        self::_pinternal::SingularHeapMessageField::<
            self::_root::google::protobuf::EnumValueOptions,
        >,
    >,
    bitfield: self::_pinternal::BitArray<1usize>,
    unknown_fields: self::_pinternal::UnknownFieldsImpl,
}
impl EnumValueDescriptorProto {
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
    pub fn number(&self) -> i32 {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.number,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn number_opt(&self) -> ::std::option::Option::<i32> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.number, &self.bitfield)
    }
    pub fn number_mut(&mut self) -> &mut i32 {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.number,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_number(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.number, &self.bitfield)
            .is_some()
    }
    pub fn clear_number(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.number, &mut self.bitfield)
    }
    pub fn options(
        &self,
    ) -> ::std::option::Option::<&self::_root::google::protobuf::EnumValueOptions> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.options,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn options_opt(
        &self,
    ) -> ::std::option::Option::<&self::_root::google::protobuf::EnumValueOptions> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.options, &self.bitfield)
    }
    pub fn options_mut(
        &mut self,
    ) -> &mut self::_root::google::protobuf::EnumValueOptions {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.options,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_options(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.options, &self.bitfield)
            .is_some()
    }
    pub fn clear_options(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.options, &mut self.bitfield)
    }
}
impl self::_puroro::Message for EnumValueDescriptorProto {
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
        use self::_pinternal::UnknownFields as _;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        use self::_puroro::PuroroError;
        while let Some((number, mut field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    1i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.name,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    2i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.number,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    3i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.options,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    _ => Err(PuroroError::UnknownFieldNumber)?,
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(
                    PuroroError::UnknownFieldNumber | PuroroError::UnknownEnumVariant(_),
                ) => {
                    self.unknown_fields.push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::UnknownFields as _;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.name,
            &self.bitfield,
            1i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.number,
            &self.bitfield,
            2i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.options,
            &self.bitfield,
            3i32,
            out,
        )?;
        self.unknown_fields.ser_to_write(out)?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for EnumValueDescriptorProto {
    fn clone(&self) -> Self {
        Self {
            fields: self::_fields::EnumValueDescriptorProtoFields {
                name: ::std::clone::Clone::clone(&self.fields.name),
                number: ::std::clone::Clone::clone(&self.fields.number),
                options: ::std::clone::Clone::clone(&self.fields.options),
            },
            bitfield: ::std::clone::Clone::clone(&self.bitfield),
            unknown_fields: ::std::clone::Clone::clone(&self.unknown_fields),
        }
    }
}
impl ::std::ops::Drop for EnumValueDescriptorProto {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for EnumValueDescriptorProto {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        use self::_pinternal::UnknownFields as _;
        let mut debug_struct = fmt.debug_struct(stringify!(EnumValueDescriptorProto));
        debug_struct
            .field(stringify!(name), &self.name_opt())
            .field(stringify!(number), &self.number_opt())
            .field(stringify!(options), &self.options_opt());
        self.unknown_fields.debug_struct_fields(&mut debug_struct)?;
        debug_struct.finish()
    }
}
impl ::std::cmp::PartialEq for EnumValueDescriptorProto {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        true && self.name_opt() == rhs.name_opt()
            && self.number_opt() == rhs.number_opt()
            && self.options_opt() == rhs.options_opt()
            && self.unknown_fields == rhs.unknown_fields
    }
}
#[derive(::std::default::Default)]
pub struct ServiceDescriptorProto {
    fields: self::_root::google::protobuf::_fields::ServiceDescriptorProtoFields<
        self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        >,
        self::_pinternal::RepeatedMessageField::<
            self::_root::google::protobuf::MethodDescriptorProto,
        >,
        self::_pinternal::SingularHeapMessageField::<
            self::_root::google::protobuf::ServiceOptions,
        >,
    >,
    bitfield: self::_pinternal::BitArray<1usize>,
    unknown_fields: self::_pinternal::UnknownFieldsImpl,
}
impl ServiceDescriptorProto {
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
    pub fn method(&self) -> &[self::_root::google::protobuf::MethodDescriptorProto] {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field(&self.fields.method, &self.bitfield)
    }
    pub fn method_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::MethodDescriptorProto> {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field_mut(&mut self.fields.method, &mut self.bitfield)
    }
    pub fn clear_method(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::clear(&mut self.fields.method, &mut self.bitfield)
    }
    pub fn options(
        &self,
    ) -> ::std::option::Option::<&self::_root::google::protobuf::ServiceOptions> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.options,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn options_opt(
        &self,
    ) -> ::std::option::Option::<&self::_root::google::protobuf::ServiceOptions> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.options, &self.bitfield)
    }
    pub fn options_mut(&mut self) -> &mut self::_root::google::protobuf::ServiceOptions {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.options,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_options(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.options, &self.bitfield)
            .is_some()
    }
    pub fn clear_options(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.options, &mut self.bitfield)
    }
}
impl self::_puroro::Message for ServiceDescriptorProto {
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
        use self::_pinternal::UnknownFields as _;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        use self::_puroro::PuroroError;
        while let Some((number, mut field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    1i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.name,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    2i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.method,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    3i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.options,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    _ => Err(PuroroError::UnknownFieldNumber)?,
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(
                    PuroroError::UnknownFieldNumber | PuroroError::UnknownEnumVariant(_),
                ) => {
                    self.unknown_fields.push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::UnknownFields as _;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.name,
            &self.bitfield,
            1i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.method,
            &self.bitfield,
            2i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.options,
            &self.bitfield,
            3i32,
            out,
        )?;
        self.unknown_fields.ser_to_write(out)?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for ServiceDescriptorProto {
    fn clone(&self) -> Self {
        Self {
            fields: self::_fields::ServiceDescriptorProtoFields {
                name: ::std::clone::Clone::clone(&self.fields.name),
                method: ::std::clone::Clone::clone(&self.fields.method),
                options: ::std::clone::Clone::clone(&self.fields.options),
            },
            bitfield: ::std::clone::Clone::clone(&self.bitfield),
            unknown_fields: ::std::clone::Clone::clone(&self.unknown_fields),
        }
    }
}
impl ::std::ops::Drop for ServiceDescriptorProto {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for ServiceDescriptorProto {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        use self::_pinternal::UnknownFields as _;
        let mut debug_struct = fmt.debug_struct(stringify!(ServiceDescriptorProto));
        debug_struct
            .field(stringify!(name), &self.name_opt())
            .field(stringify!(method), &self.method())
            .field(stringify!(options), &self.options_opt());
        self.unknown_fields.debug_struct_fields(&mut debug_struct)?;
        debug_struct.finish()
    }
}
impl ::std::cmp::PartialEq for ServiceDescriptorProto {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        true && self.name_opt() == rhs.name_opt() && self.method() == rhs.method()
            && self.options_opt() == rhs.options_opt()
            && self.unknown_fields == rhs.unknown_fields
    }
}
#[derive(::std::default::Default)]
pub struct MethodDescriptorProto {
    fields: self::_root::google::protobuf::_fields::MethodDescriptorProtoFields<
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
            self::_root::google::protobuf::MethodOptions,
        >,
        self::_pinternal::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            3usize,
        >,
        self::_pinternal::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            4usize,
        >,
    >,
    bitfield: self::_pinternal::BitArray<1usize>,
    unknown_fields: self::_pinternal::UnknownFieldsImpl,
}
impl MethodDescriptorProto {
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
    pub fn input_type(&self) -> &str {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.input_type,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn input_type_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.input_type, &self.bitfield)
    }
    pub fn input_type_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.input_type,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_input_type(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.input_type, &self.bitfield)
            .is_some()
    }
    pub fn clear_input_type(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.input_type, &mut self.bitfield)
    }
    pub fn output_type(&self) -> &str {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.output_type,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn output_type_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.output_type, &self.bitfield)
    }
    pub fn output_type_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.output_type,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_output_type(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.output_type, &self.bitfield)
            .is_some()
    }
    pub fn clear_output_type(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.output_type, &mut self.bitfield)
    }
    pub fn options(
        &self,
    ) -> ::std::option::Option::<&self::_root::google::protobuf::MethodOptions> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.options,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn options_opt(
        &self,
    ) -> ::std::option::Option::<&self::_root::google::protobuf::MethodOptions> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.options, &self.bitfield)
    }
    pub fn options_mut(&mut self) -> &mut self::_root::google::protobuf::MethodOptions {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.options,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_options(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.options, &self.bitfield)
            .is_some()
    }
    pub fn clear_options(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.options, &mut self.bitfield)
    }
    pub fn client_streaming(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.client_streaming,
            &self.bitfield,
            || false,
        )
    }
    pub fn client_streaming_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
            &self.fields.client_streaming,
            &self.bitfield,
        )
    }
    pub fn client_streaming_mut(&mut self) -> &mut bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.client_streaming,
            &mut self.bitfield,
            || false,
        )
    }
    pub fn has_client_streaming(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
                &self.fields.client_streaming,
                &self.bitfield,
            )
            .is_some()
    }
    pub fn clear_client_streaming(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(
            &mut self.fields.client_streaming,
            &mut self.bitfield,
        )
    }
    pub fn server_streaming(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.server_streaming,
            &self.bitfield,
            || false,
        )
    }
    pub fn server_streaming_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
            &self.fields.server_streaming,
            &self.bitfield,
        )
    }
    pub fn server_streaming_mut(&mut self) -> &mut bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.server_streaming,
            &mut self.bitfield,
            || false,
        )
    }
    pub fn has_server_streaming(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
                &self.fields.server_streaming,
                &self.bitfield,
            )
            .is_some()
    }
    pub fn clear_server_streaming(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(
            &mut self.fields.server_streaming,
            &mut self.bitfield,
        )
    }
}
impl self::_puroro::Message for MethodDescriptorProto {
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
        use self::_pinternal::UnknownFields as _;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        use self::_puroro::PuroroError;
        while let Some((number, mut field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    1i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.name,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    2i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.input_type,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    3i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.output_type,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    4i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.options,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    5i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.client_streaming,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    6i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.server_streaming,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    _ => Err(PuroroError::UnknownFieldNumber)?,
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(
                    PuroroError::UnknownFieldNumber | PuroroError::UnknownEnumVariant(_),
                ) => {
                    self.unknown_fields.push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::UnknownFields as _;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.name,
            &self.bitfield,
            1i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.input_type,
            &self.bitfield,
            2i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.output_type,
            &self.bitfield,
            3i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.options,
            &self.bitfield,
            4i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.client_streaming,
            &self.bitfield,
            5i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.server_streaming,
            &self.bitfield,
            6i32,
            out,
        )?;
        self.unknown_fields.ser_to_write(out)?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for MethodDescriptorProto {
    fn clone(&self) -> Self {
        Self {
            fields: self::_fields::MethodDescriptorProtoFields {
                name: ::std::clone::Clone::clone(&self.fields.name),
                input_type: ::std::clone::Clone::clone(&self.fields.input_type),
                output_type: ::std::clone::Clone::clone(&self.fields.output_type),
                options: ::std::clone::Clone::clone(&self.fields.options),
                client_streaming: ::std::clone::Clone::clone(
                    &self.fields.client_streaming,
                ),
                server_streaming: ::std::clone::Clone::clone(
                    &self.fields.server_streaming,
                ),
            },
            bitfield: ::std::clone::Clone::clone(&self.bitfield),
            unknown_fields: ::std::clone::Clone::clone(&self.unknown_fields),
        }
    }
}
impl ::std::ops::Drop for MethodDescriptorProto {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for MethodDescriptorProto {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        use self::_pinternal::UnknownFields as _;
        let mut debug_struct = fmt.debug_struct(stringify!(MethodDescriptorProto));
        debug_struct
            .field(stringify!(name), &self.name_opt())
            .field(stringify!(input_type), &self.input_type_opt())
            .field(stringify!(output_type), &self.output_type_opt())
            .field(stringify!(options), &self.options_opt())
            .field(stringify!(client_streaming), &self.client_streaming_opt())
            .field(stringify!(server_streaming), &self.server_streaming_opt());
        self.unknown_fields.debug_struct_fields(&mut debug_struct)?;
        debug_struct.finish()
    }
}
impl ::std::cmp::PartialEq for MethodDescriptorProto {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        true && self.name_opt() == rhs.name_opt()
            && self.input_type_opt() == rhs.input_type_opt()
            && self.output_type_opt() == rhs.output_type_opt()
            && self.options_opt() == rhs.options_opt()
            && self.client_streaming_opt() == rhs.client_streaming_opt()
            && self.server_streaming_opt() == rhs.server_streaming_opt()
            && self.unknown_fields == rhs.unknown_fields
    }
}
#[derive(::std::default::Default)]
pub struct FileOptions {
    fields: self::_root::google::protobuf::_fields::FileOptionsFields<
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
        self::_pinternal::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            2usize,
        >,
        self::_pinternal::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            3usize,
        >,
        self::_pinternal::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            4usize,
        >,
        self::_pinternal::OptionalNumericalField::<
            self::_root::google::protobuf::file_options::OptimizeMode,
            self::_pinternal::tags::Enum2::<
                self::_root::google::protobuf::file_options::OptimizeMode,
            >,
            5usize,
        >,
        self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            6usize,
        >,
        self::_pinternal::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            7usize,
        >,
        self::_pinternal::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            8usize,
        >,
        self::_pinternal::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            9usize,
        >,
        self::_pinternal::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            10usize,
        >,
        self::_pinternal::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            11usize,
        >,
        self::_pinternal::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            12usize,
        >,
        self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            13usize,
        >,
        self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            14usize,
        >,
        self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            15usize,
        >,
        self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            16usize,
        >,
        self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            17usize,
        >,
        self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            18usize,
        >,
        self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            19usize,
        >,
        self::_pinternal::RepeatedMessageField::<
            self::_root::google::protobuf::UninterpretedOption,
        >,
    >,
    bitfield: self::_pinternal::BitArray<1usize>,
    unknown_fields: self::_pinternal::UnknownFieldsImpl,
}
impl FileOptions {
    pub fn java_package(&self) -> &str {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.java_package,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn java_package_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.java_package, &self.bitfield)
    }
    pub fn java_package_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.java_package,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_java_package(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.java_package, &self.bitfield)
            .is_some()
    }
    pub fn clear_java_package(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.java_package, &mut self.bitfield)
    }
    pub fn java_outer_classname(&self) -> &str {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.java_outer_classname,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn java_outer_classname_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
            &self.fields.java_outer_classname,
            &self.bitfield,
        )
    }
    pub fn java_outer_classname_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.java_outer_classname,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_java_outer_classname(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
                &self.fields.java_outer_classname,
                &self.bitfield,
            )
            .is_some()
    }
    pub fn clear_java_outer_classname(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(
            &mut self.fields.java_outer_classname,
            &mut self.bitfield,
        )
    }
    pub fn java_multiple_files(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.java_multiple_files,
            &self.bitfield,
            || false,
        )
    }
    pub fn java_multiple_files_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
            &self.fields.java_multiple_files,
            &self.bitfield,
        )
    }
    pub fn java_multiple_files_mut(&mut self) -> &mut bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.java_multiple_files,
            &mut self.bitfield,
            || false,
        )
    }
    pub fn has_java_multiple_files(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
                &self.fields.java_multiple_files,
                &self.bitfield,
            )
            .is_some()
    }
    pub fn clear_java_multiple_files(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(
            &mut self.fields.java_multiple_files,
            &mut self.bitfield,
        )
    }
    pub fn java_generate_equals_and_hash(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.java_generate_equals_and_hash,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn java_generate_equals_and_hash_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
            &self.fields.java_generate_equals_and_hash,
            &self.bitfield,
        )
    }
    pub fn java_generate_equals_and_hash_mut(&mut self) -> &mut bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.java_generate_equals_and_hash,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_java_generate_equals_and_hash(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
                &self.fields.java_generate_equals_and_hash,
                &self.bitfield,
            )
            .is_some()
    }
    pub fn clear_java_generate_equals_and_hash(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(
            &mut self.fields.java_generate_equals_and_hash,
            &mut self.bitfield,
        )
    }
    pub fn java_string_check_utf8(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.java_string_check_utf8,
            &self.bitfield,
            || false,
        )
    }
    pub fn java_string_check_utf8_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
            &self.fields.java_string_check_utf8,
            &self.bitfield,
        )
    }
    pub fn java_string_check_utf8_mut(&mut self) -> &mut bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.java_string_check_utf8,
            &mut self.bitfield,
            || false,
        )
    }
    pub fn has_java_string_check_utf8(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
                &self.fields.java_string_check_utf8,
                &self.bitfield,
            )
            .is_some()
    }
    pub fn clear_java_string_check_utf8(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(
            &mut self.fields.java_string_check_utf8,
            &mut self.bitfield,
        )
    }
    pub fn optimize_for(
        &self,
    ) -> self::_root::google::protobuf::file_options::OptimizeMode {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.optimize_for,
            &self.bitfield,
            || self::_root::google::protobuf::file_options::OptimizeMode::Speed,
        )
    }
    pub fn optimize_for_opt(
        &self,
    ) -> ::std::option::Option::<
        self::_root::google::protobuf::file_options::OptimizeMode,
    > {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.optimize_for, &self.bitfield)
    }
    pub fn optimize_for_mut(
        &mut self,
    ) -> &mut self::_root::google::protobuf::file_options::OptimizeMode {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.optimize_for,
            &mut self.bitfield,
            || self::_root::google::protobuf::file_options::OptimizeMode::Speed,
        )
    }
    pub fn has_optimize_for(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.optimize_for, &self.bitfield)
            .is_some()
    }
    pub fn clear_optimize_for(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.optimize_for, &mut self.bitfield)
    }
    pub fn go_package(&self) -> &str {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.go_package,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn go_package_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.go_package, &self.bitfield)
    }
    pub fn go_package_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.go_package,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_go_package(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.go_package, &self.bitfield)
            .is_some()
    }
    pub fn clear_go_package(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.go_package, &mut self.bitfield)
    }
    pub fn cc_generic_services(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.cc_generic_services,
            &self.bitfield,
            || false,
        )
    }
    pub fn cc_generic_services_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
            &self.fields.cc_generic_services,
            &self.bitfield,
        )
    }
    pub fn cc_generic_services_mut(&mut self) -> &mut bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.cc_generic_services,
            &mut self.bitfield,
            || false,
        )
    }
    pub fn has_cc_generic_services(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
                &self.fields.cc_generic_services,
                &self.bitfield,
            )
            .is_some()
    }
    pub fn clear_cc_generic_services(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(
            &mut self.fields.cc_generic_services,
            &mut self.bitfield,
        )
    }
    pub fn java_generic_services(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.java_generic_services,
            &self.bitfield,
            || false,
        )
    }
    pub fn java_generic_services_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
            &self.fields.java_generic_services,
            &self.bitfield,
        )
    }
    pub fn java_generic_services_mut(&mut self) -> &mut bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.java_generic_services,
            &mut self.bitfield,
            || false,
        )
    }
    pub fn has_java_generic_services(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
                &self.fields.java_generic_services,
                &self.bitfield,
            )
            .is_some()
    }
    pub fn clear_java_generic_services(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(
            &mut self.fields.java_generic_services,
            &mut self.bitfield,
        )
    }
    pub fn py_generic_services(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.py_generic_services,
            &self.bitfield,
            || false,
        )
    }
    pub fn py_generic_services_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
            &self.fields.py_generic_services,
            &self.bitfield,
        )
    }
    pub fn py_generic_services_mut(&mut self) -> &mut bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.py_generic_services,
            &mut self.bitfield,
            || false,
        )
    }
    pub fn has_py_generic_services(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
                &self.fields.py_generic_services,
                &self.bitfield,
            )
            .is_some()
    }
    pub fn clear_py_generic_services(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(
            &mut self.fields.py_generic_services,
            &mut self.bitfield,
        )
    }
    pub fn php_generic_services(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.php_generic_services,
            &self.bitfield,
            || false,
        )
    }
    pub fn php_generic_services_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
            &self.fields.php_generic_services,
            &self.bitfield,
        )
    }
    pub fn php_generic_services_mut(&mut self) -> &mut bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.php_generic_services,
            &mut self.bitfield,
            || false,
        )
    }
    pub fn has_php_generic_services(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
                &self.fields.php_generic_services,
                &self.bitfield,
            )
            .is_some()
    }
    pub fn clear_php_generic_services(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(
            &mut self.fields.php_generic_services,
            &mut self.bitfield,
        )
    }
    pub fn deprecated(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.deprecated,
            &self.bitfield,
            || false,
        )
    }
    pub fn deprecated_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.deprecated, &self.bitfield)
    }
    pub fn deprecated_mut(&mut self) -> &mut bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.deprecated,
            &mut self.bitfield,
            || false,
        )
    }
    pub fn has_deprecated(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.deprecated, &self.bitfield)
            .is_some()
    }
    pub fn clear_deprecated(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.deprecated, &mut self.bitfield)
    }
    pub fn cc_enable_arenas(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.cc_enable_arenas,
            &self.bitfield,
            || true,
        )
    }
    pub fn cc_enable_arenas_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
            &self.fields.cc_enable_arenas,
            &self.bitfield,
        )
    }
    pub fn cc_enable_arenas_mut(&mut self) -> &mut bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.cc_enable_arenas,
            &mut self.bitfield,
            || true,
        )
    }
    pub fn has_cc_enable_arenas(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
                &self.fields.cc_enable_arenas,
                &self.bitfield,
            )
            .is_some()
    }
    pub fn clear_cc_enable_arenas(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(
            &mut self.fields.cc_enable_arenas,
            &mut self.bitfield,
        )
    }
    pub fn objc_class_prefix(&self) -> &str {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.objc_class_prefix,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn objc_class_prefix_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
            &self.fields.objc_class_prefix,
            &self.bitfield,
        )
    }
    pub fn objc_class_prefix_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.objc_class_prefix,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_objc_class_prefix(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
                &self.fields.objc_class_prefix,
                &self.bitfield,
            )
            .is_some()
    }
    pub fn clear_objc_class_prefix(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(
            &mut self.fields.objc_class_prefix,
            &mut self.bitfield,
        )
    }
    pub fn csharp_namespace(&self) -> &str {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.csharp_namespace,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn csharp_namespace_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
            &self.fields.csharp_namespace,
            &self.bitfield,
        )
    }
    pub fn csharp_namespace_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.csharp_namespace,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_csharp_namespace(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
                &self.fields.csharp_namespace,
                &self.bitfield,
            )
            .is_some()
    }
    pub fn clear_csharp_namespace(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(
            &mut self.fields.csharp_namespace,
            &mut self.bitfield,
        )
    }
    pub fn swift_prefix(&self) -> &str {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.swift_prefix,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn swift_prefix_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.swift_prefix, &self.bitfield)
    }
    pub fn swift_prefix_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.swift_prefix,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_swift_prefix(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.swift_prefix, &self.bitfield)
            .is_some()
    }
    pub fn clear_swift_prefix(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.swift_prefix, &mut self.bitfield)
    }
    pub fn php_class_prefix(&self) -> &str {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.php_class_prefix,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn php_class_prefix_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
            &self.fields.php_class_prefix,
            &self.bitfield,
        )
    }
    pub fn php_class_prefix_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.php_class_prefix,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_php_class_prefix(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
                &self.fields.php_class_prefix,
                &self.bitfield,
            )
            .is_some()
    }
    pub fn clear_php_class_prefix(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(
            &mut self.fields.php_class_prefix,
            &mut self.bitfield,
        )
    }
    pub fn php_namespace(&self) -> &str {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.php_namespace,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn php_namespace_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.php_namespace, &self.bitfield)
    }
    pub fn php_namespace_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.php_namespace,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_php_namespace(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.php_namespace, &self.bitfield)
            .is_some()
    }
    pub fn clear_php_namespace(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.php_namespace, &mut self.bitfield)
    }
    pub fn php_metadata_namespace(&self) -> &str {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.php_metadata_namespace,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn php_metadata_namespace_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
            &self.fields.php_metadata_namespace,
            &self.bitfield,
        )
    }
    pub fn php_metadata_namespace_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.php_metadata_namespace,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_php_metadata_namespace(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
                &self.fields.php_metadata_namespace,
                &self.bitfield,
            )
            .is_some()
    }
    pub fn clear_php_metadata_namespace(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(
            &mut self.fields.php_metadata_namespace,
            &mut self.bitfield,
        )
    }
    pub fn ruby_package(&self) -> &str {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.ruby_package,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn ruby_package_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.ruby_package, &self.bitfield)
    }
    pub fn ruby_package_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.ruby_package,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_ruby_package(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.ruby_package, &self.bitfield)
            .is_some()
    }
    pub fn clear_ruby_package(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.ruby_package, &mut self.bitfield)
    }
    pub fn uninterpreted_option(
        &self,
    ) -> &[self::_root::google::protobuf::UninterpretedOption] {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field(&self.fields.uninterpreted_option, &self.bitfield)
    }
    pub fn uninterpreted_option_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::UninterpretedOption> {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field_mut(
            &mut self.fields.uninterpreted_option,
            &mut self.bitfield,
        )
    }
    pub fn clear_uninterpreted_option(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::clear(
            &mut self.fields.uninterpreted_option,
            &mut self.bitfield,
        )
    }
}
impl self::_puroro::Message for FileOptions {
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
        use self::_pinternal::UnknownFields as _;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        use self::_puroro::PuroroError;
        while let Some((number, mut field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    1i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.java_package,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    8i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.java_outer_classname,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    10i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.java_multiple_files,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    20i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.java_generate_equals_and_hash,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    27i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.java_string_check_utf8,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    9i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.optimize_for,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    11i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.go_package,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    16i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.cc_generic_services,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    17i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.java_generic_services,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    18i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.py_generic_services,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    42i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.php_generic_services,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    23i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.deprecated,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    31i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.cc_enable_arenas,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    36i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.objc_class_prefix,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    37i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.csharp_namespace,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    39i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.swift_prefix,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    40i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.php_class_prefix,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    41i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.php_namespace,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    44i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.php_metadata_namespace,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    45i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.ruby_package,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    999i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.uninterpreted_option,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    _ => Err(PuroroError::UnknownFieldNumber)?,
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(
                    PuroroError::UnknownFieldNumber | PuroroError::UnknownEnumVariant(_),
                ) => {
                    self.unknown_fields.push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::UnknownFields as _;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.java_package,
            &self.bitfield,
            1i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.java_outer_classname,
            &self.bitfield,
            8i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.java_multiple_files,
            &self.bitfield,
            10i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.java_generate_equals_and_hash,
            &self.bitfield,
            20i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.java_string_check_utf8,
            &self.bitfield,
            27i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.optimize_for,
            &self.bitfield,
            9i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.go_package,
            &self.bitfield,
            11i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.cc_generic_services,
            &self.bitfield,
            16i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.java_generic_services,
            &self.bitfield,
            17i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.py_generic_services,
            &self.bitfield,
            18i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.php_generic_services,
            &self.bitfield,
            42i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.deprecated,
            &self.bitfield,
            23i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.cc_enable_arenas,
            &self.bitfield,
            31i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.objc_class_prefix,
            &self.bitfield,
            36i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.csharp_namespace,
            &self.bitfield,
            37i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.swift_prefix,
            &self.bitfield,
            39i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.php_class_prefix,
            &self.bitfield,
            40i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.php_namespace,
            &self.bitfield,
            41i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.php_metadata_namespace,
            &self.bitfield,
            44i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.ruby_package,
            &self.bitfield,
            45i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.uninterpreted_option,
            &self.bitfield,
            999i32,
            out,
        )?;
        self.unknown_fields.ser_to_write(out)?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for FileOptions {
    fn clone(&self) -> Self {
        Self {
            fields: self::_fields::FileOptionsFields {
                java_package: ::std::clone::Clone::clone(&self.fields.java_package),
                java_outer_classname: ::std::clone::Clone::clone(
                    &self.fields.java_outer_classname,
                ),
                java_multiple_files: ::std::clone::Clone::clone(
                    &self.fields.java_multiple_files,
                ),
                java_generate_equals_and_hash: ::std::clone::Clone::clone(
                    &self.fields.java_generate_equals_and_hash,
                ),
                java_string_check_utf8: ::std::clone::Clone::clone(
                    &self.fields.java_string_check_utf8,
                ),
                optimize_for: ::std::clone::Clone::clone(&self.fields.optimize_for),
                go_package: ::std::clone::Clone::clone(&self.fields.go_package),
                cc_generic_services: ::std::clone::Clone::clone(
                    &self.fields.cc_generic_services,
                ),
                java_generic_services: ::std::clone::Clone::clone(
                    &self.fields.java_generic_services,
                ),
                py_generic_services: ::std::clone::Clone::clone(
                    &self.fields.py_generic_services,
                ),
                php_generic_services: ::std::clone::Clone::clone(
                    &self.fields.php_generic_services,
                ),
                deprecated: ::std::clone::Clone::clone(&self.fields.deprecated),
                cc_enable_arenas: ::std::clone::Clone::clone(
                    &self.fields.cc_enable_arenas,
                ),
                objc_class_prefix: ::std::clone::Clone::clone(
                    &self.fields.objc_class_prefix,
                ),
                csharp_namespace: ::std::clone::Clone::clone(
                    &self.fields.csharp_namespace,
                ),
                swift_prefix: ::std::clone::Clone::clone(&self.fields.swift_prefix),
                php_class_prefix: ::std::clone::Clone::clone(
                    &self.fields.php_class_prefix,
                ),
                php_namespace: ::std::clone::Clone::clone(&self.fields.php_namespace),
                php_metadata_namespace: ::std::clone::Clone::clone(
                    &self.fields.php_metadata_namespace,
                ),
                ruby_package: ::std::clone::Clone::clone(&self.fields.ruby_package),
                uninterpreted_option: ::std::clone::Clone::clone(
                    &self.fields.uninterpreted_option,
                ),
            },
            bitfield: ::std::clone::Clone::clone(&self.bitfield),
            unknown_fields: ::std::clone::Clone::clone(&self.unknown_fields),
        }
    }
}
impl ::std::ops::Drop for FileOptions {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for FileOptions {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        use self::_pinternal::UnknownFields as _;
        let mut debug_struct = fmt.debug_struct(stringify!(FileOptions));
        debug_struct
            .field(stringify!(java_package), &self.java_package_opt())
            .field(stringify!(java_outer_classname), &self.java_outer_classname_opt())
            .field(stringify!(java_multiple_files), &self.java_multiple_files_opt())
            .field(
                stringify!(java_generate_equals_and_hash),
                &self.java_generate_equals_and_hash_opt(),
            )
            .field(
                stringify!(java_string_check_utf8),
                &self.java_string_check_utf8_opt(),
            )
            .field(stringify!(optimize_for), &self.optimize_for_opt())
            .field(stringify!(go_package), &self.go_package_opt())
            .field(stringify!(cc_generic_services), &self.cc_generic_services_opt())
            .field(stringify!(java_generic_services), &self.java_generic_services_opt())
            .field(stringify!(py_generic_services), &self.py_generic_services_opt())
            .field(stringify!(php_generic_services), &self.php_generic_services_opt())
            .field(stringify!(deprecated), &self.deprecated_opt())
            .field(stringify!(cc_enable_arenas), &self.cc_enable_arenas_opt())
            .field(stringify!(objc_class_prefix), &self.objc_class_prefix_opt())
            .field(stringify!(csharp_namespace), &self.csharp_namespace_opt())
            .field(stringify!(swift_prefix), &self.swift_prefix_opt())
            .field(stringify!(php_class_prefix), &self.php_class_prefix_opt())
            .field(stringify!(php_namespace), &self.php_namespace_opt())
            .field(
                stringify!(php_metadata_namespace),
                &self.php_metadata_namespace_opt(),
            )
            .field(stringify!(ruby_package), &self.ruby_package_opt())
            .field(stringify!(uninterpreted_option), &self.uninterpreted_option());
        self.unknown_fields.debug_struct_fields(&mut debug_struct)?;
        debug_struct.finish()
    }
}
impl ::std::cmp::PartialEq for FileOptions {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        true && self.java_package_opt() == rhs.java_package_opt()
            && self.java_outer_classname_opt() == rhs.java_outer_classname_opt()
            && self.java_multiple_files_opt() == rhs.java_multiple_files_opt()
            && self.java_generate_equals_and_hash_opt()
                == rhs.java_generate_equals_and_hash_opt()
            && self.java_string_check_utf8_opt() == rhs.java_string_check_utf8_opt()
            && self.optimize_for_opt() == rhs.optimize_for_opt()
            && self.go_package_opt() == rhs.go_package_opt()
            && self.cc_generic_services_opt() == rhs.cc_generic_services_opt()
            && self.java_generic_services_opt() == rhs.java_generic_services_opt()
            && self.py_generic_services_opt() == rhs.py_generic_services_opt()
            && self.php_generic_services_opt() == rhs.php_generic_services_opt()
            && self.deprecated_opt() == rhs.deprecated_opt()
            && self.cc_enable_arenas_opt() == rhs.cc_enable_arenas_opt()
            && self.objc_class_prefix_opt() == rhs.objc_class_prefix_opt()
            && self.csharp_namespace_opt() == rhs.csharp_namespace_opt()
            && self.swift_prefix_opt() == rhs.swift_prefix_opt()
            && self.php_class_prefix_opt() == rhs.php_class_prefix_opt()
            && self.php_namespace_opt() == rhs.php_namespace_opt()
            && self.php_metadata_namespace_opt() == rhs.php_metadata_namespace_opt()
            && self.ruby_package_opt() == rhs.ruby_package_opt()
            && self.uninterpreted_option() == rhs.uninterpreted_option()
            && self.unknown_fields == rhs.unknown_fields
    }
}
#[derive(::std::default::Default)]
pub struct MessageOptions {
    fields: self::_root::google::protobuf::_fields::MessageOptionsFields<
        self::_pinternal::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            0usize,
        >,
        self::_pinternal::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            1usize,
        >,
        self::_pinternal::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            2usize,
        >,
        self::_pinternal::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            3usize,
        >,
        self::_pinternal::RepeatedMessageField::<
            self::_root::google::protobuf::UninterpretedOption,
        >,
    >,
    bitfield: self::_pinternal::BitArray<1usize>,
    unknown_fields: self::_pinternal::UnknownFieldsImpl,
}
impl MessageOptions {
    pub fn message_set_wire_format(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.message_set_wire_format,
            &self.bitfield,
            || false,
        )
    }
    pub fn message_set_wire_format_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
            &self.fields.message_set_wire_format,
            &self.bitfield,
        )
    }
    pub fn message_set_wire_format_mut(&mut self) -> &mut bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.message_set_wire_format,
            &mut self.bitfield,
            || false,
        )
    }
    pub fn has_message_set_wire_format(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
                &self.fields.message_set_wire_format,
                &self.bitfield,
            )
            .is_some()
    }
    pub fn clear_message_set_wire_format(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(
            &mut self.fields.message_set_wire_format,
            &mut self.bitfield,
        )
    }
    pub fn no_standard_descriptor_accessor(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.no_standard_descriptor_accessor,
            &self.bitfield,
            || false,
        )
    }
    pub fn no_standard_descriptor_accessor_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
            &self.fields.no_standard_descriptor_accessor,
            &self.bitfield,
        )
    }
    pub fn no_standard_descriptor_accessor_mut(&mut self) -> &mut bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.no_standard_descriptor_accessor,
            &mut self.bitfield,
            || false,
        )
    }
    pub fn has_no_standard_descriptor_accessor(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
                &self.fields.no_standard_descriptor_accessor,
                &self.bitfield,
            )
            .is_some()
    }
    pub fn clear_no_standard_descriptor_accessor(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(
            &mut self.fields.no_standard_descriptor_accessor,
            &mut self.bitfield,
        )
    }
    pub fn deprecated(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.deprecated,
            &self.bitfield,
            || false,
        )
    }
    pub fn deprecated_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.deprecated, &self.bitfield)
    }
    pub fn deprecated_mut(&mut self) -> &mut bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.deprecated,
            &mut self.bitfield,
            || false,
        )
    }
    pub fn has_deprecated(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.deprecated, &self.bitfield)
            .is_some()
    }
    pub fn clear_deprecated(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.deprecated, &mut self.bitfield)
    }
    pub fn map_entry(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.map_entry,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn map_entry_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.map_entry, &self.bitfield)
    }
    pub fn map_entry_mut(&mut self) -> &mut bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.map_entry,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_map_entry(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.map_entry, &self.bitfield)
            .is_some()
    }
    pub fn clear_map_entry(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.map_entry, &mut self.bitfield)
    }
    pub fn uninterpreted_option(
        &self,
    ) -> &[self::_root::google::protobuf::UninterpretedOption] {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field(&self.fields.uninterpreted_option, &self.bitfield)
    }
    pub fn uninterpreted_option_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::UninterpretedOption> {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field_mut(
            &mut self.fields.uninterpreted_option,
            &mut self.bitfield,
        )
    }
    pub fn clear_uninterpreted_option(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::clear(
            &mut self.fields.uninterpreted_option,
            &mut self.bitfield,
        )
    }
}
impl self::_puroro::Message for MessageOptions {
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
        use self::_pinternal::UnknownFields as _;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        use self::_puroro::PuroroError;
        while let Some((number, mut field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    1i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.message_set_wire_format,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    2i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.no_standard_descriptor_accessor,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    3i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.deprecated,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    7i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.map_entry,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    999i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.uninterpreted_option,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    _ => Err(PuroroError::UnknownFieldNumber)?,
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(
                    PuroroError::UnknownFieldNumber | PuroroError::UnknownEnumVariant(_),
                ) => {
                    self.unknown_fields.push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::UnknownFields as _;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.message_set_wire_format,
            &self.bitfield,
            1i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.no_standard_descriptor_accessor,
            &self.bitfield,
            2i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.deprecated,
            &self.bitfield,
            3i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.map_entry,
            &self.bitfield,
            7i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.uninterpreted_option,
            &self.bitfield,
            999i32,
            out,
        )?;
        self.unknown_fields.ser_to_write(out)?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for MessageOptions {
    fn clone(&self) -> Self {
        Self {
            fields: self::_fields::MessageOptionsFields {
                message_set_wire_format: ::std::clone::Clone::clone(
                    &self.fields.message_set_wire_format,
                ),
                no_standard_descriptor_accessor: ::std::clone::Clone::clone(
                    &self.fields.no_standard_descriptor_accessor,
                ),
                deprecated: ::std::clone::Clone::clone(&self.fields.deprecated),
                map_entry: ::std::clone::Clone::clone(&self.fields.map_entry),
                uninterpreted_option: ::std::clone::Clone::clone(
                    &self.fields.uninterpreted_option,
                ),
            },
            bitfield: ::std::clone::Clone::clone(&self.bitfield),
            unknown_fields: ::std::clone::Clone::clone(&self.unknown_fields),
        }
    }
}
impl ::std::ops::Drop for MessageOptions {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for MessageOptions {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        use self::_pinternal::UnknownFields as _;
        let mut debug_struct = fmt.debug_struct(stringify!(MessageOptions));
        debug_struct
            .field(
                stringify!(message_set_wire_format),
                &self.message_set_wire_format_opt(),
            )
            .field(
                stringify!(no_standard_descriptor_accessor),
                &self.no_standard_descriptor_accessor_opt(),
            )
            .field(stringify!(deprecated), &self.deprecated_opt())
            .field(stringify!(map_entry), &self.map_entry_opt())
            .field(stringify!(uninterpreted_option), &self.uninterpreted_option());
        self.unknown_fields.debug_struct_fields(&mut debug_struct)?;
        debug_struct.finish()
    }
}
impl ::std::cmp::PartialEq for MessageOptions {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        true && self.message_set_wire_format_opt() == rhs.message_set_wire_format_opt()
            && self.no_standard_descriptor_accessor_opt()
                == rhs.no_standard_descriptor_accessor_opt()
            && self.deprecated_opt() == rhs.deprecated_opt()
            && self.map_entry_opt() == rhs.map_entry_opt()
            && self.uninterpreted_option() == rhs.uninterpreted_option()
            && self.unknown_fields == rhs.unknown_fields
    }
}
#[derive(::std::default::Default)]
pub struct FieldOptions {
    fields: self::_root::google::protobuf::_fields::FieldOptionsFields<
        self::_pinternal::OptionalNumericalField::<
            self::_root::google::protobuf::field_options::CType,
            self::_pinternal::tags::Enum2::<
                self::_root::google::protobuf::field_options::CType,
            >,
            0usize,
        >,
        self::_pinternal::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            1usize,
        >,
        self::_pinternal::OptionalNumericalField::<
            self::_root::google::protobuf::field_options::JSType,
            self::_pinternal::tags::Enum2::<
                self::_root::google::protobuf::field_options::JSType,
            >,
            2usize,
        >,
        self::_pinternal::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            3usize,
        >,
        self::_pinternal::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            4usize,
        >,
        self::_pinternal::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            5usize,
        >,
        self::_pinternal::RepeatedMessageField::<
            self::_root::google::protobuf::UninterpretedOption,
        >,
    >,
    bitfield: self::_pinternal::BitArray<1usize>,
    unknown_fields: self::_pinternal::UnknownFieldsImpl,
}
impl FieldOptions {
    pub fn ctype(&self) -> self::_root::google::protobuf::field_options::CType {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.ctype,
            &self.bitfield,
            || self::_root::google::protobuf::field_options::CType::String,
        )
    }
    pub fn ctype_opt(
        &self,
    ) -> ::std::option::Option::<self::_root::google::protobuf::field_options::CType> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.ctype, &self.bitfield)
    }
    pub fn ctype_mut(
        &mut self,
    ) -> &mut self::_root::google::protobuf::field_options::CType {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.ctype,
            &mut self.bitfield,
            || self::_root::google::protobuf::field_options::CType::String,
        )
    }
    pub fn has_ctype(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.ctype, &self.bitfield).is_some()
    }
    pub fn clear_ctype(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.ctype, &mut self.bitfield)
    }
    pub fn packed(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.packed,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn packed_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.packed, &self.bitfield)
    }
    pub fn packed_mut(&mut self) -> &mut bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.packed,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_packed(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.packed, &self.bitfield)
            .is_some()
    }
    pub fn clear_packed(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.packed, &mut self.bitfield)
    }
    pub fn jstype(&self) -> self::_root::google::protobuf::field_options::JSType {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.jstype,
            &self.bitfield,
            || self::_root::google::protobuf::field_options::JSType::JsNormal,
        )
    }
    pub fn jstype_opt(
        &self,
    ) -> ::std::option::Option::<self::_root::google::protobuf::field_options::JSType> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.jstype, &self.bitfield)
    }
    pub fn jstype_mut(
        &mut self,
    ) -> &mut self::_root::google::protobuf::field_options::JSType {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.jstype,
            &mut self.bitfield,
            || self::_root::google::protobuf::field_options::JSType::JsNormal,
        )
    }
    pub fn has_jstype(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.jstype, &self.bitfield)
            .is_some()
    }
    pub fn clear_jstype(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.jstype, &mut self.bitfield)
    }
    pub fn lazy(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.lazy,
            &self.bitfield,
            || false,
        )
    }
    pub fn lazy_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.lazy, &self.bitfield)
    }
    pub fn lazy_mut(&mut self) -> &mut bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.lazy,
            &mut self.bitfield,
            || false,
        )
    }
    pub fn has_lazy(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.lazy, &self.bitfield).is_some()
    }
    pub fn clear_lazy(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.lazy, &mut self.bitfield)
    }
    pub fn deprecated(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.deprecated,
            &self.bitfield,
            || false,
        )
    }
    pub fn deprecated_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.deprecated, &self.bitfield)
    }
    pub fn deprecated_mut(&mut self) -> &mut bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.deprecated,
            &mut self.bitfield,
            || false,
        )
    }
    pub fn has_deprecated(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.deprecated, &self.bitfield)
            .is_some()
    }
    pub fn clear_deprecated(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.deprecated, &mut self.bitfield)
    }
    pub fn weak(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.weak,
            &self.bitfield,
            || false,
        )
    }
    pub fn weak_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.weak, &self.bitfield)
    }
    pub fn weak_mut(&mut self) -> &mut bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.weak,
            &mut self.bitfield,
            || false,
        )
    }
    pub fn has_weak(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.weak, &self.bitfield).is_some()
    }
    pub fn clear_weak(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.weak, &mut self.bitfield)
    }
    pub fn uninterpreted_option(
        &self,
    ) -> &[self::_root::google::protobuf::UninterpretedOption] {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field(&self.fields.uninterpreted_option, &self.bitfield)
    }
    pub fn uninterpreted_option_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::UninterpretedOption> {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field_mut(
            &mut self.fields.uninterpreted_option,
            &mut self.bitfield,
        )
    }
    pub fn clear_uninterpreted_option(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::clear(
            &mut self.fields.uninterpreted_option,
            &mut self.bitfield,
        )
    }
}
impl self::_puroro::Message for FieldOptions {
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
        use self::_pinternal::UnknownFields as _;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        use self::_puroro::PuroroError;
        while let Some((number, mut field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    1i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.ctype,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    2i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.packed,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    6i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.jstype,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    5i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.lazy,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    3i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.deprecated,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    10i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.weak,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    999i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.uninterpreted_option,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    _ => Err(PuroroError::UnknownFieldNumber)?,
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(
                    PuroroError::UnknownFieldNumber | PuroroError::UnknownEnumVariant(_),
                ) => {
                    self.unknown_fields.push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::UnknownFields as _;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.ctype,
            &self.bitfield,
            1i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.packed,
            &self.bitfield,
            2i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.jstype,
            &self.bitfield,
            6i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.lazy,
            &self.bitfield,
            5i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.deprecated,
            &self.bitfield,
            3i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.weak,
            &self.bitfield,
            10i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.uninterpreted_option,
            &self.bitfield,
            999i32,
            out,
        )?;
        self.unknown_fields.ser_to_write(out)?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for FieldOptions {
    fn clone(&self) -> Self {
        Self {
            fields: self::_fields::FieldOptionsFields {
                ctype: ::std::clone::Clone::clone(&self.fields.ctype),
                packed: ::std::clone::Clone::clone(&self.fields.packed),
                jstype: ::std::clone::Clone::clone(&self.fields.jstype),
                lazy: ::std::clone::Clone::clone(&self.fields.lazy),
                deprecated: ::std::clone::Clone::clone(&self.fields.deprecated),
                weak: ::std::clone::Clone::clone(&self.fields.weak),
                uninterpreted_option: ::std::clone::Clone::clone(
                    &self.fields.uninterpreted_option,
                ),
            },
            bitfield: ::std::clone::Clone::clone(&self.bitfield),
            unknown_fields: ::std::clone::Clone::clone(&self.unknown_fields),
        }
    }
}
impl ::std::ops::Drop for FieldOptions {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for FieldOptions {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        use self::_pinternal::UnknownFields as _;
        let mut debug_struct = fmt.debug_struct(stringify!(FieldOptions));
        debug_struct
            .field(stringify!(ctype), &self.ctype_opt())
            .field(stringify!(packed), &self.packed_opt())
            .field(stringify!(jstype), &self.jstype_opt())
            .field(stringify!(lazy), &self.lazy_opt())
            .field(stringify!(deprecated), &self.deprecated_opt())
            .field(stringify!(weak), &self.weak_opt())
            .field(stringify!(uninterpreted_option), &self.uninterpreted_option());
        self.unknown_fields.debug_struct_fields(&mut debug_struct)?;
        debug_struct.finish()
    }
}
impl ::std::cmp::PartialEq for FieldOptions {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        true && self.ctype_opt() == rhs.ctype_opt()
            && self.packed_opt() == rhs.packed_opt()
            && self.jstype_opt() == rhs.jstype_opt() && self.lazy_opt() == rhs.lazy_opt()
            && self.deprecated_opt() == rhs.deprecated_opt()
            && self.weak_opt() == rhs.weak_opt()
            && self.uninterpreted_option() == rhs.uninterpreted_option()
            && self.unknown_fields == rhs.unknown_fields
    }
}
#[derive(::std::default::Default)]
pub struct OneofOptions {
    fields: self::_root::google::protobuf::_fields::OneofOptionsFields<
        self::_pinternal::RepeatedMessageField::<
            self::_root::google::protobuf::UninterpretedOption,
        >,
    >,
    bitfield: self::_pinternal::BitArray<0usize>,
    unknown_fields: self::_pinternal::UnknownFieldsImpl,
}
impl OneofOptions {
    pub fn uninterpreted_option(
        &self,
    ) -> &[self::_root::google::protobuf::UninterpretedOption] {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field(&self.fields.uninterpreted_option, &self.bitfield)
    }
    pub fn uninterpreted_option_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::UninterpretedOption> {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field_mut(
            &mut self.fields.uninterpreted_option,
            &mut self.bitfield,
        )
    }
    pub fn clear_uninterpreted_option(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::clear(
            &mut self.fields.uninterpreted_option,
            &mut self.bitfield,
        )
    }
}
impl self::_puroro::Message for OneofOptions {
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
        use self::_pinternal::UnknownFields as _;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        use self::_puroro::PuroroError;
        while let Some((number, mut field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    999i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.uninterpreted_option,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    _ => Err(PuroroError::UnknownFieldNumber)?,
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(
                    PuroroError::UnknownFieldNumber | PuroroError::UnknownEnumVariant(_),
                ) => {
                    self.unknown_fields.push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::UnknownFields as _;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.uninterpreted_option,
            &self.bitfield,
            999i32,
            out,
        )?;
        self.unknown_fields.ser_to_write(out)?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for OneofOptions {
    fn clone(&self) -> Self {
        Self {
            fields: self::_fields::OneofOptionsFields {
                uninterpreted_option: ::std::clone::Clone::clone(
                    &self.fields.uninterpreted_option,
                ),
            },
            bitfield: ::std::clone::Clone::clone(&self.bitfield),
            unknown_fields: ::std::clone::Clone::clone(&self.unknown_fields),
        }
    }
}
impl ::std::ops::Drop for OneofOptions {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for OneofOptions {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        use self::_pinternal::UnknownFields as _;
        let mut debug_struct = fmt.debug_struct(stringify!(OneofOptions));
        debug_struct
            .field(stringify!(uninterpreted_option), &self.uninterpreted_option());
        self.unknown_fields.debug_struct_fields(&mut debug_struct)?;
        debug_struct.finish()
    }
}
impl ::std::cmp::PartialEq for OneofOptions {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        true && self.uninterpreted_option() == rhs.uninterpreted_option()
            && self.unknown_fields == rhs.unknown_fields
    }
}
#[derive(::std::default::Default)]
pub struct EnumOptions {
    fields: self::_root::google::protobuf::_fields::EnumOptionsFields<
        self::_pinternal::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            0usize,
        >,
        self::_pinternal::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            1usize,
        >,
        self::_pinternal::RepeatedMessageField::<
            self::_root::google::protobuf::UninterpretedOption,
        >,
    >,
    bitfield: self::_pinternal::BitArray<1usize>,
    unknown_fields: self::_pinternal::UnknownFieldsImpl,
}
impl EnumOptions {
    pub fn allow_alias(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.allow_alias,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn allow_alias_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.allow_alias, &self.bitfield)
    }
    pub fn allow_alias_mut(&mut self) -> &mut bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.allow_alias,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_allow_alias(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.allow_alias, &self.bitfield)
            .is_some()
    }
    pub fn clear_allow_alias(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.allow_alias, &mut self.bitfield)
    }
    pub fn deprecated(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.deprecated,
            &self.bitfield,
            || false,
        )
    }
    pub fn deprecated_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.deprecated, &self.bitfield)
    }
    pub fn deprecated_mut(&mut self) -> &mut bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.deprecated,
            &mut self.bitfield,
            || false,
        )
    }
    pub fn has_deprecated(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.deprecated, &self.bitfield)
            .is_some()
    }
    pub fn clear_deprecated(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.deprecated, &mut self.bitfield)
    }
    pub fn uninterpreted_option(
        &self,
    ) -> &[self::_root::google::protobuf::UninterpretedOption] {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field(&self.fields.uninterpreted_option, &self.bitfield)
    }
    pub fn uninterpreted_option_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::UninterpretedOption> {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field_mut(
            &mut self.fields.uninterpreted_option,
            &mut self.bitfield,
        )
    }
    pub fn clear_uninterpreted_option(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::clear(
            &mut self.fields.uninterpreted_option,
            &mut self.bitfield,
        )
    }
}
impl self::_puroro::Message for EnumOptions {
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
        use self::_pinternal::UnknownFields as _;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        use self::_puroro::PuroroError;
        while let Some((number, mut field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    2i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.allow_alias,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    3i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.deprecated,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    999i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.uninterpreted_option,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    _ => Err(PuroroError::UnknownFieldNumber)?,
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(
                    PuroroError::UnknownFieldNumber | PuroroError::UnknownEnumVariant(_),
                ) => {
                    self.unknown_fields.push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::UnknownFields as _;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.allow_alias,
            &self.bitfield,
            2i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.deprecated,
            &self.bitfield,
            3i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.uninterpreted_option,
            &self.bitfield,
            999i32,
            out,
        )?;
        self.unknown_fields.ser_to_write(out)?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for EnumOptions {
    fn clone(&self) -> Self {
        Self {
            fields: self::_fields::EnumOptionsFields {
                allow_alias: ::std::clone::Clone::clone(&self.fields.allow_alias),
                deprecated: ::std::clone::Clone::clone(&self.fields.deprecated),
                uninterpreted_option: ::std::clone::Clone::clone(
                    &self.fields.uninterpreted_option,
                ),
            },
            bitfield: ::std::clone::Clone::clone(&self.bitfield),
            unknown_fields: ::std::clone::Clone::clone(&self.unknown_fields),
        }
    }
}
impl ::std::ops::Drop for EnumOptions {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for EnumOptions {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        use self::_pinternal::UnknownFields as _;
        let mut debug_struct = fmt.debug_struct(stringify!(EnumOptions));
        debug_struct
            .field(stringify!(allow_alias), &self.allow_alias_opt())
            .field(stringify!(deprecated), &self.deprecated_opt())
            .field(stringify!(uninterpreted_option), &self.uninterpreted_option());
        self.unknown_fields.debug_struct_fields(&mut debug_struct)?;
        debug_struct.finish()
    }
}
impl ::std::cmp::PartialEq for EnumOptions {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        true && self.allow_alias_opt() == rhs.allow_alias_opt()
            && self.deprecated_opt() == rhs.deprecated_opt()
            && self.uninterpreted_option() == rhs.uninterpreted_option()
            && self.unknown_fields == rhs.unknown_fields
    }
}
#[derive(::std::default::Default)]
pub struct EnumValueOptions {
    fields: self::_root::google::protobuf::_fields::EnumValueOptionsFields<
        self::_pinternal::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            0usize,
        >,
        self::_pinternal::RepeatedMessageField::<
            self::_root::google::protobuf::UninterpretedOption,
        >,
    >,
    bitfield: self::_pinternal::BitArray<1usize>,
    unknown_fields: self::_pinternal::UnknownFieldsImpl,
}
impl EnumValueOptions {
    pub fn deprecated(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.deprecated,
            &self.bitfield,
            || false,
        )
    }
    pub fn deprecated_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.deprecated, &self.bitfield)
    }
    pub fn deprecated_mut(&mut self) -> &mut bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.deprecated,
            &mut self.bitfield,
            || false,
        )
    }
    pub fn has_deprecated(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.deprecated, &self.bitfield)
            .is_some()
    }
    pub fn clear_deprecated(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.deprecated, &mut self.bitfield)
    }
    pub fn uninterpreted_option(
        &self,
    ) -> &[self::_root::google::protobuf::UninterpretedOption] {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field(&self.fields.uninterpreted_option, &self.bitfield)
    }
    pub fn uninterpreted_option_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::UninterpretedOption> {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field_mut(
            &mut self.fields.uninterpreted_option,
            &mut self.bitfield,
        )
    }
    pub fn clear_uninterpreted_option(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::clear(
            &mut self.fields.uninterpreted_option,
            &mut self.bitfield,
        )
    }
}
impl self::_puroro::Message for EnumValueOptions {
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
        use self::_pinternal::UnknownFields as _;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        use self::_puroro::PuroroError;
        while let Some((number, mut field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    1i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.deprecated,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    999i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.uninterpreted_option,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    _ => Err(PuroroError::UnknownFieldNumber)?,
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(
                    PuroroError::UnknownFieldNumber | PuroroError::UnknownEnumVariant(_),
                ) => {
                    self.unknown_fields.push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::UnknownFields as _;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.deprecated,
            &self.bitfield,
            1i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.uninterpreted_option,
            &self.bitfield,
            999i32,
            out,
        )?;
        self.unknown_fields.ser_to_write(out)?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for EnumValueOptions {
    fn clone(&self) -> Self {
        Self {
            fields: self::_fields::EnumValueOptionsFields {
                deprecated: ::std::clone::Clone::clone(&self.fields.deprecated),
                uninterpreted_option: ::std::clone::Clone::clone(
                    &self.fields.uninterpreted_option,
                ),
            },
            bitfield: ::std::clone::Clone::clone(&self.bitfield),
            unknown_fields: ::std::clone::Clone::clone(&self.unknown_fields),
        }
    }
}
impl ::std::ops::Drop for EnumValueOptions {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for EnumValueOptions {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        use self::_pinternal::UnknownFields as _;
        let mut debug_struct = fmt.debug_struct(stringify!(EnumValueOptions));
        debug_struct
            .field(stringify!(deprecated), &self.deprecated_opt())
            .field(stringify!(uninterpreted_option), &self.uninterpreted_option());
        self.unknown_fields.debug_struct_fields(&mut debug_struct)?;
        debug_struct.finish()
    }
}
impl ::std::cmp::PartialEq for EnumValueOptions {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        true && self.deprecated_opt() == rhs.deprecated_opt()
            && self.uninterpreted_option() == rhs.uninterpreted_option()
            && self.unknown_fields == rhs.unknown_fields
    }
}
#[derive(::std::default::Default)]
pub struct ServiceOptions {
    fields: self::_root::google::protobuf::_fields::ServiceOptionsFields<
        self::_pinternal::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            0usize,
        >,
        self::_pinternal::RepeatedMessageField::<
            self::_root::google::protobuf::UninterpretedOption,
        >,
    >,
    bitfield: self::_pinternal::BitArray<1usize>,
    unknown_fields: self::_pinternal::UnknownFieldsImpl,
}
impl ServiceOptions {
    pub fn deprecated(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.deprecated,
            &self.bitfield,
            || false,
        )
    }
    pub fn deprecated_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.deprecated, &self.bitfield)
    }
    pub fn deprecated_mut(&mut self) -> &mut bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.deprecated,
            &mut self.bitfield,
            || false,
        )
    }
    pub fn has_deprecated(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.deprecated, &self.bitfield)
            .is_some()
    }
    pub fn clear_deprecated(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.deprecated, &mut self.bitfield)
    }
    pub fn uninterpreted_option(
        &self,
    ) -> &[self::_root::google::protobuf::UninterpretedOption] {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field(&self.fields.uninterpreted_option, &self.bitfield)
    }
    pub fn uninterpreted_option_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::UninterpretedOption> {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field_mut(
            &mut self.fields.uninterpreted_option,
            &mut self.bitfield,
        )
    }
    pub fn clear_uninterpreted_option(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::clear(
            &mut self.fields.uninterpreted_option,
            &mut self.bitfield,
        )
    }
}
impl self::_puroro::Message for ServiceOptions {
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
        use self::_pinternal::UnknownFields as _;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        use self::_puroro::PuroroError;
        while let Some((number, mut field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    33i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.deprecated,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    999i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.uninterpreted_option,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    _ => Err(PuroroError::UnknownFieldNumber)?,
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(
                    PuroroError::UnknownFieldNumber | PuroroError::UnknownEnumVariant(_),
                ) => {
                    self.unknown_fields.push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::UnknownFields as _;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.deprecated,
            &self.bitfield,
            33i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.uninterpreted_option,
            &self.bitfield,
            999i32,
            out,
        )?;
        self.unknown_fields.ser_to_write(out)?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for ServiceOptions {
    fn clone(&self) -> Self {
        Self {
            fields: self::_fields::ServiceOptionsFields {
                deprecated: ::std::clone::Clone::clone(&self.fields.deprecated),
                uninterpreted_option: ::std::clone::Clone::clone(
                    &self.fields.uninterpreted_option,
                ),
            },
            bitfield: ::std::clone::Clone::clone(&self.bitfield),
            unknown_fields: ::std::clone::Clone::clone(&self.unknown_fields),
        }
    }
}
impl ::std::ops::Drop for ServiceOptions {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for ServiceOptions {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        use self::_pinternal::UnknownFields as _;
        let mut debug_struct = fmt.debug_struct(stringify!(ServiceOptions));
        debug_struct
            .field(stringify!(deprecated), &self.deprecated_opt())
            .field(stringify!(uninterpreted_option), &self.uninterpreted_option());
        self.unknown_fields.debug_struct_fields(&mut debug_struct)?;
        debug_struct.finish()
    }
}
impl ::std::cmp::PartialEq for ServiceOptions {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        true && self.deprecated_opt() == rhs.deprecated_opt()
            && self.uninterpreted_option() == rhs.uninterpreted_option()
            && self.unknown_fields == rhs.unknown_fields
    }
}
#[derive(::std::default::Default)]
pub struct MethodOptions {
    fields: self::_root::google::protobuf::_fields::MethodOptionsFields<
        self::_pinternal::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            0usize,
        >,
        self::_pinternal::OptionalNumericalField::<
            self::_root::google::protobuf::method_options::IdempotencyLevel,
            self::_pinternal::tags::Enum2::<
                self::_root::google::protobuf::method_options::IdempotencyLevel,
            >,
            1usize,
        >,
        self::_pinternal::RepeatedMessageField::<
            self::_root::google::protobuf::UninterpretedOption,
        >,
    >,
    bitfield: self::_pinternal::BitArray<1usize>,
    unknown_fields: self::_pinternal::UnknownFieldsImpl,
}
impl MethodOptions {
    pub fn deprecated(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.deprecated,
            &self.bitfield,
            || false,
        )
    }
    pub fn deprecated_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.deprecated, &self.bitfield)
    }
    pub fn deprecated_mut(&mut self) -> &mut bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.deprecated,
            &mut self.bitfield,
            || false,
        )
    }
    pub fn has_deprecated(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.deprecated, &self.bitfield)
            .is_some()
    }
    pub fn clear_deprecated(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.deprecated, &mut self.bitfield)
    }
    pub fn idempotency_level(
        &self,
    ) -> self::_root::google::protobuf::method_options::IdempotencyLevel {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.idempotency_level,
            &self.bitfield,
            || {
                self::_root::google::protobuf::method_options::IdempotencyLevel::IdempotencyUnknown
            },
        )
    }
    pub fn idempotency_level_opt(
        &self,
    ) -> ::std::option::Option::<
        self::_root::google::protobuf::method_options::IdempotencyLevel,
    > {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
            &self.fields.idempotency_level,
            &self.bitfield,
        )
    }
    pub fn idempotency_level_mut(
        &mut self,
    ) -> &mut self::_root::google::protobuf::method_options::IdempotencyLevel {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.idempotency_level,
            &mut self.bitfield,
            || {
                self::_root::google::protobuf::method_options::IdempotencyLevel::IdempotencyUnknown
            },
        )
    }
    pub fn has_idempotency_level(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
                &self.fields.idempotency_level,
                &self.bitfield,
            )
            .is_some()
    }
    pub fn clear_idempotency_level(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(
            &mut self.fields.idempotency_level,
            &mut self.bitfield,
        )
    }
    pub fn uninterpreted_option(
        &self,
    ) -> &[self::_root::google::protobuf::UninterpretedOption] {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field(&self.fields.uninterpreted_option, &self.bitfield)
    }
    pub fn uninterpreted_option_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::UninterpretedOption> {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field_mut(
            &mut self.fields.uninterpreted_option,
            &mut self.bitfield,
        )
    }
    pub fn clear_uninterpreted_option(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::clear(
            &mut self.fields.uninterpreted_option,
            &mut self.bitfield,
        )
    }
}
impl self::_puroro::Message for MethodOptions {
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
        use self::_pinternal::UnknownFields as _;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        use self::_puroro::PuroroError;
        while let Some((number, mut field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    33i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.deprecated,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    34i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.idempotency_level,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    999i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.uninterpreted_option,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    _ => Err(PuroroError::UnknownFieldNumber)?,
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(
                    PuroroError::UnknownFieldNumber | PuroroError::UnknownEnumVariant(_),
                ) => {
                    self.unknown_fields.push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::UnknownFields as _;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.deprecated,
            &self.bitfield,
            33i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.idempotency_level,
            &self.bitfield,
            34i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.uninterpreted_option,
            &self.bitfield,
            999i32,
            out,
        )?;
        self.unknown_fields.ser_to_write(out)?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for MethodOptions {
    fn clone(&self) -> Self {
        Self {
            fields: self::_fields::MethodOptionsFields {
                deprecated: ::std::clone::Clone::clone(&self.fields.deprecated),
                idempotency_level: ::std::clone::Clone::clone(
                    &self.fields.idempotency_level,
                ),
                uninterpreted_option: ::std::clone::Clone::clone(
                    &self.fields.uninterpreted_option,
                ),
            },
            bitfield: ::std::clone::Clone::clone(&self.bitfield),
            unknown_fields: ::std::clone::Clone::clone(&self.unknown_fields),
        }
    }
}
impl ::std::ops::Drop for MethodOptions {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for MethodOptions {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        use self::_pinternal::UnknownFields as _;
        let mut debug_struct = fmt.debug_struct(stringify!(MethodOptions));
        debug_struct
            .field(stringify!(deprecated), &self.deprecated_opt())
            .field(stringify!(idempotency_level), &self.idempotency_level_opt())
            .field(stringify!(uninterpreted_option), &self.uninterpreted_option());
        self.unknown_fields.debug_struct_fields(&mut debug_struct)?;
        debug_struct.finish()
    }
}
impl ::std::cmp::PartialEq for MethodOptions {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        true && self.deprecated_opt() == rhs.deprecated_opt()
            && self.idempotency_level_opt() == rhs.idempotency_level_opt()
            && self.uninterpreted_option() == rhs.uninterpreted_option()
            && self.unknown_fields == rhs.unknown_fields
    }
}
#[derive(::std::default::Default)]
pub struct UninterpretedOption {
    fields: self::_root::google::protobuf::_fields::UninterpretedOptionFields<
        self::_pinternal::RepeatedMessageField::<
            self::_root::google::protobuf::uninterpreted_option::NamePart,
        >,
        self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        >,
        self::_pinternal::OptionalNumericalField::<
            u64,
            self::_pinternal::tags::UInt64,
            1usize,
        >,
        self::_pinternal::OptionalNumericalField::<
            i64,
            self::_pinternal::tags::Int64,
            2usize,
        >,
        self::_pinternal::OptionalNumericalField::<
            f64,
            self::_pinternal::tags::Double,
            3usize,
        >,
        self::_pinternal::OptionalUnsizedField::<
            ::std::vec::Vec<u8>,
            self::_pinternal::tags::Bytes,
            4usize,
        >,
        self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            5usize,
        >,
    >,
    bitfield: self::_pinternal::BitArray<1usize>,
    unknown_fields: self::_pinternal::UnknownFieldsImpl,
}
impl UninterpretedOption {
    pub fn name(
        &self,
    ) -> &[self::_root::google::protobuf::uninterpreted_option::NamePart] {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field(&self.fields.name, &self.bitfield)
    }
    pub fn name_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<
        self::_root::google::protobuf::uninterpreted_option::NamePart,
    > {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field_mut(&mut self.fields.name, &mut self.bitfield)
    }
    pub fn clear_name(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::clear(&mut self.fields.name, &mut self.bitfield)
    }
    pub fn identifier_value(&self) -> &str {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.identifier_value,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn identifier_value_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
            &self.fields.identifier_value,
            &self.bitfield,
        )
    }
    pub fn identifier_value_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.identifier_value,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_identifier_value(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
                &self.fields.identifier_value,
                &self.bitfield,
            )
            .is_some()
    }
    pub fn clear_identifier_value(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(
            &mut self.fields.identifier_value,
            &mut self.bitfield,
        )
    }
    pub fn positive_int_value(&self) -> u64 {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.positive_int_value,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn positive_int_value_opt(&self) -> ::std::option::Option::<u64> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
            &self.fields.positive_int_value,
            &self.bitfield,
        )
    }
    pub fn positive_int_value_mut(&mut self) -> &mut u64 {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.positive_int_value,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_positive_int_value(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
                &self.fields.positive_int_value,
                &self.bitfield,
            )
            .is_some()
    }
    pub fn clear_positive_int_value(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(
            &mut self.fields.positive_int_value,
            &mut self.bitfield,
        )
    }
    pub fn negative_int_value(&self) -> i64 {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.negative_int_value,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn negative_int_value_opt(&self) -> ::std::option::Option::<i64> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
            &self.fields.negative_int_value,
            &self.bitfield,
        )
    }
    pub fn negative_int_value_mut(&mut self) -> &mut i64 {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.negative_int_value,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_negative_int_value(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(
                &self.fields.negative_int_value,
                &self.bitfield,
            )
            .is_some()
    }
    pub fn clear_negative_int_value(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(
            &mut self.fields.negative_int_value,
            &mut self.bitfield,
        )
    }
    pub fn double_value(&self) -> f64 {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.double_value,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn double_value_opt(&self) -> ::std::option::Option::<f64> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.double_value, &self.bitfield)
    }
    pub fn double_value_mut(&mut self) -> &mut f64 {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.double_value,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_double_value(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.double_value, &self.bitfield)
            .is_some()
    }
    pub fn clear_double_value(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.double_value, &mut self.bitfield)
    }
    pub fn string_value(&self) -> &[u8] {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.string_value,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn string_value_opt(&self) -> ::std::option::Option::<&[u8]> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.string_value, &self.bitfield)
    }
    pub fn string_value_mut(&mut self) -> &mut ::std::vec::Vec::<u8> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.string_value,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_string_value(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.string_value, &self.bitfield)
            .is_some()
    }
    pub fn clear_string_value(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.string_value, &mut self.bitfield)
    }
    pub fn aggregate_value(&self) -> &str {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.aggregate_value,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn aggregate_value_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.aggregate_value, &self.bitfield)
    }
    pub fn aggregate_value_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.aggregate_value,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_aggregate_value(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::get_field_opt(&self.fields.aggregate_value, &self.bitfield)
            .is_some()
    }
    pub fn clear_aggregate_value(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        NonRepeatedFieldType::clear(&mut self.fields.aggregate_value, &mut self.bitfield)
    }
}
impl self::_puroro::Message for UninterpretedOption {
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
        use self::_pinternal::UnknownFields as _;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        use self::_puroro::PuroroError;
        while let Some((number, mut field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    2i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.name,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    3i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.identifier_value,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    4i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.positive_int_value,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    5i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.negative_int_value,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    6i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.double_value,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    7i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.string_value,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    8i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.aggregate_value,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    _ => Err(PuroroError::UnknownFieldNumber)?,
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(
                    PuroroError::UnknownFieldNumber | PuroroError::UnknownEnumVariant(_),
                ) => {
                    self.unknown_fields.push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::UnknownFields as _;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.name,
            &self.bitfield,
            2i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.identifier_value,
            &self.bitfield,
            3i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.positive_int_value,
            &self.bitfield,
            4i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.negative_int_value,
            &self.bitfield,
            5i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.double_value,
            &self.bitfield,
            6i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.string_value,
            &self.bitfield,
            7i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.aggregate_value,
            &self.bitfield,
            8i32,
            out,
        )?;
        self.unknown_fields.ser_to_write(out)?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for UninterpretedOption {
    fn clone(&self) -> Self {
        Self {
            fields: self::_fields::UninterpretedOptionFields {
                name: ::std::clone::Clone::clone(&self.fields.name),
                identifier_value: ::std::clone::Clone::clone(
                    &self.fields.identifier_value,
                ),
                positive_int_value: ::std::clone::Clone::clone(
                    &self.fields.positive_int_value,
                ),
                negative_int_value: ::std::clone::Clone::clone(
                    &self.fields.negative_int_value,
                ),
                double_value: ::std::clone::Clone::clone(&self.fields.double_value),
                string_value: ::std::clone::Clone::clone(&self.fields.string_value),
                aggregate_value: ::std::clone::Clone::clone(&self.fields.aggregate_value),
            },
            bitfield: ::std::clone::Clone::clone(&self.bitfield),
            unknown_fields: ::std::clone::Clone::clone(&self.unknown_fields),
        }
    }
}
impl ::std::ops::Drop for UninterpretedOption {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for UninterpretedOption {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        use self::_pinternal::UnknownFields as _;
        let mut debug_struct = fmt.debug_struct(stringify!(UninterpretedOption));
        debug_struct
            .field(stringify!(name), &self.name())
            .field(stringify!(identifier_value), &self.identifier_value_opt())
            .field(stringify!(positive_int_value), &self.positive_int_value_opt())
            .field(stringify!(negative_int_value), &self.negative_int_value_opt())
            .field(stringify!(double_value), &self.double_value_opt())
            .field(stringify!(string_value), &self.string_value_opt())
            .field(stringify!(aggregate_value), &self.aggregate_value_opt());
        self.unknown_fields.debug_struct_fields(&mut debug_struct)?;
        debug_struct.finish()
    }
}
impl ::std::cmp::PartialEq for UninterpretedOption {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        true && self.name() == rhs.name()
            && self.identifier_value_opt() == rhs.identifier_value_opt()
            && self.positive_int_value_opt() == rhs.positive_int_value_opt()
            && self.negative_int_value_opt() == rhs.negative_int_value_opt()
            && self.double_value_opt() == rhs.double_value_opt()
            && self.string_value_opt() == rhs.string_value_opt()
            && self.aggregate_value_opt() == rhs.aggregate_value_opt()
            && self.unknown_fields == rhs.unknown_fields
    }
}
#[derive(::std::default::Default)]
pub struct SourceCodeInfo {
    fields: self::_root::google::protobuf::_fields::SourceCodeInfoFields<
        self::_pinternal::RepeatedMessageField::<
            self::_root::google::protobuf::source_code_info::Location,
        >,
    >,
    bitfield: self::_pinternal::BitArray<0usize>,
    unknown_fields: self::_pinternal::UnknownFieldsImpl,
}
impl SourceCodeInfo {
    pub fn location(
        &self,
    ) -> &[self::_root::google::protobuf::source_code_info::Location] {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field(&self.fields.location, &self.bitfield)
    }
    pub fn location_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<
        self::_root::google::protobuf::source_code_info::Location,
    > {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field_mut(&mut self.fields.location, &mut self.bitfield)
    }
    pub fn clear_location(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::clear(&mut self.fields.location, &mut self.bitfield)
    }
}
impl self::_puroro::Message for SourceCodeInfo {
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
        use self::_pinternal::UnknownFields as _;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        use self::_puroro::PuroroError;
        while let Some((number, mut field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    1i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.location,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    _ => Err(PuroroError::UnknownFieldNumber)?,
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(
                    PuroroError::UnknownFieldNumber | PuroroError::UnknownEnumVariant(_),
                ) => {
                    self.unknown_fields.push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::UnknownFields as _;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.location,
            &self.bitfield,
            1i32,
            out,
        )?;
        self.unknown_fields.ser_to_write(out)?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for SourceCodeInfo {
    fn clone(&self) -> Self {
        Self {
            fields: self::_fields::SourceCodeInfoFields {
                location: ::std::clone::Clone::clone(&self.fields.location),
            },
            bitfield: ::std::clone::Clone::clone(&self.bitfield),
            unknown_fields: ::std::clone::Clone::clone(&self.unknown_fields),
        }
    }
}
impl ::std::ops::Drop for SourceCodeInfo {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for SourceCodeInfo {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        use self::_pinternal::UnknownFields as _;
        let mut debug_struct = fmt.debug_struct(stringify!(SourceCodeInfo));
        debug_struct.field(stringify!(location), &self.location());
        self.unknown_fields.debug_struct_fields(&mut debug_struct)?;
        debug_struct.finish()
    }
}
impl ::std::cmp::PartialEq for SourceCodeInfo {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        true && self.location() == rhs.location()
            && self.unknown_fields == rhs.unknown_fields
    }
}
#[derive(::std::default::Default)]
pub struct GeneratedCodeInfo {
    fields: self::_root::google::protobuf::_fields::GeneratedCodeInfoFields<
        self::_pinternal::RepeatedMessageField::<
            self::_root::google::protobuf::generated_code_info::Annotation,
        >,
    >,
    bitfield: self::_pinternal::BitArray<0usize>,
    unknown_fields: self::_pinternal::UnknownFieldsImpl,
}
impl GeneratedCodeInfo {
    pub fn annotation(
        &self,
    ) -> &[self::_root::google::protobuf::generated_code_info::Annotation] {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field(&self.fields.annotation, &self.bitfield)
    }
    pub fn annotation_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<
        self::_root::google::protobuf::generated_code_info::Annotation,
    > {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::get_field_mut(&mut self.fields.annotation, &mut self.bitfield)
    }
    pub fn clear_annotation(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        RepeatedFieldType::clear(&mut self.fields.annotation, &mut self.bitfield)
    }
}
impl self::_puroro::Message for GeneratedCodeInfo {
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
        use self::_pinternal::UnknownFields as _;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        use self::_puroro::PuroroError;
        while let Some((number, mut field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    1i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.annotation,
                            &mut self.bitfield,
                            &mut field_data,
                        )?
                    }
                    _ => Err(PuroroError::UnknownFieldNumber)?,
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(
                    PuroroError::UnknownFieldNumber | PuroroError::UnknownEnumVariant(_),
                ) => {
                    self.unknown_fields.push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::UnknownFields as _;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.annotation,
            &self.bitfield,
            1i32,
            out,
        )?;
        self.unknown_fields.ser_to_write(out)?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for GeneratedCodeInfo {
    fn clone(&self) -> Self {
        Self {
            fields: self::_fields::GeneratedCodeInfoFields {
                annotation: ::std::clone::Clone::clone(&self.fields.annotation),
            },
            bitfield: ::std::clone::Clone::clone(&self.bitfield),
            unknown_fields: ::std::clone::Clone::clone(&self.unknown_fields),
        }
    }
}
impl ::std::ops::Drop for GeneratedCodeInfo {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for GeneratedCodeInfo {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        use self::_pinternal::UnknownFields as _;
        let mut debug_struct = fmt.debug_struct(stringify!(GeneratedCodeInfo));
        debug_struct.field(stringify!(annotation), &self.annotation());
        self.unknown_fields.debug_struct_fields(&mut debug_struct)?;
        debug_struct.finish()
    }
}
impl ::std::cmp::PartialEq for GeneratedCodeInfo {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        true && self.annotation() == rhs.annotation()
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
    pub struct FileDescriptorSetFields<TFile> {
        pub file: TFile,
    }
    #[derive(::std::default::Default)]
    pub struct FileDescriptorProtoFields<
        TName,
        TPackage,
        TDependency,
        TPublicDependency,
        TWeakDependency,
        TMessageType,
        TEnumType,
        TService,
        TExtension,
        TOptions,
        TSourceCodeInfo,
        TSyntax,
    > {
        pub name: TName,
        pub package: TPackage,
        pub dependency: TDependency,
        pub public_dependency: TPublicDependency,
        pub weak_dependency: TWeakDependency,
        pub message_type: TMessageType,
        pub enum_type: TEnumType,
        pub service: TService,
        pub extension: TExtension,
        pub options: TOptions,
        pub source_code_info: TSourceCodeInfo,
        pub syntax: TSyntax,
    }
    #[derive(::std::default::Default)]
    pub struct DescriptorProtoFields<
        TName,
        TField,
        TExtension,
        TNestedType,
        TEnumType,
        TExtensionRange,
        TOneofDecl,
        TOptions,
        TReservedRange,
        TReservedName,
    > {
        pub name: TName,
        pub field: TField,
        pub extension: TExtension,
        pub nested_type: TNestedType,
        pub enum_type: TEnumType,
        pub extension_range: TExtensionRange,
        pub oneof_decl: TOneofDecl,
        pub options: TOptions,
        pub reserved_range: TReservedRange,
        pub reserved_name: TReservedName,
    }
    #[derive(::std::default::Default)]
    pub struct ExtensionRangeOptionsFields<TUninterpretedOption> {
        pub uninterpreted_option: TUninterpretedOption,
    }
    #[derive(::std::default::Default)]
    pub struct FieldDescriptorProtoFields<
        TName,
        TNumber,
        TLabel,
        TType,
        TTypeName,
        TExtendee,
        TDefaultValue,
        TOneofIndex,
        TJsonName,
        TOptions,
        TProto3Optional,
    > {
        pub name: TName,
        pub number: TNumber,
        pub label: TLabel,
        pub r#type: TType,
        pub type_name: TTypeName,
        pub extendee: TExtendee,
        pub default_value: TDefaultValue,
        pub oneof_index: TOneofIndex,
        pub json_name: TJsonName,
        pub options: TOptions,
        pub proto3_optional: TProto3Optional,
    }
    #[derive(::std::default::Default)]
    pub struct OneofDescriptorProtoFields<TName, TOptions> {
        pub name: TName,
        pub options: TOptions,
    }
    #[derive(::std::default::Default)]
    pub struct EnumDescriptorProtoFields<
        TName,
        TValue,
        TOptions,
        TReservedRange,
        TReservedName,
    > {
        pub name: TName,
        pub value: TValue,
        pub options: TOptions,
        pub reserved_range: TReservedRange,
        pub reserved_name: TReservedName,
    }
    #[derive(::std::default::Default)]
    pub struct EnumValueDescriptorProtoFields<TName, TNumber, TOptions> {
        pub name: TName,
        pub number: TNumber,
        pub options: TOptions,
    }
    #[derive(::std::default::Default)]
    pub struct ServiceDescriptorProtoFields<TName, TMethod, TOptions> {
        pub name: TName,
        pub method: TMethod,
        pub options: TOptions,
    }
    #[derive(::std::default::Default)]
    pub struct MethodDescriptorProtoFields<
        TName,
        TInputType,
        TOutputType,
        TOptions,
        TClientStreaming,
        TServerStreaming,
    > {
        pub name: TName,
        pub input_type: TInputType,
        pub output_type: TOutputType,
        pub options: TOptions,
        pub client_streaming: TClientStreaming,
        pub server_streaming: TServerStreaming,
    }
    #[derive(::std::default::Default)]
    pub struct FileOptionsFields<
        TJavaPackage,
        TJavaOuterClassname,
        TJavaMultipleFiles,
        TJavaGenerateEqualsAndHash,
        TJavaStringCheckUtf8,
        TOptimizeFor,
        TGoPackage,
        TCcGenericServices,
        TJavaGenericServices,
        TPyGenericServices,
        TPhpGenericServices,
        TDeprecated,
        TCcEnableArenas,
        TObjcClassPrefix,
        TCsharpNamespace,
        TSwiftPrefix,
        TPhpClassPrefix,
        TPhpNamespace,
        TPhpMetadataNamespace,
        TRubyPackage,
        TUninterpretedOption,
    > {
        pub java_package: TJavaPackage,
        pub java_outer_classname: TJavaOuterClassname,
        pub java_multiple_files: TJavaMultipleFiles,
        pub java_generate_equals_and_hash: TJavaGenerateEqualsAndHash,
        pub java_string_check_utf8: TJavaStringCheckUtf8,
        pub optimize_for: TOptimizeFor,
        pub go_package: TGoPackage,
        pub cc_generic_services: TCcGenericServices,
        pub java_generic_services: TJavaGenericServices,
        pub py_generic_services: TPyGenericServices,
        pub php_generic_services: TPhpGenericServices,
        pub deprecated: TDeprecated,
        pub cc_enable_arenas: TCcEnableArenas,
        pub objc_class_prefix: TObjcClassPrefix,
        pub csharp_namespace: TCsharpNamespace,
        pub swift_prefix: TSwiftPrefix,
        pub php_class_prefix: TPhpClassPrefix,
        pub php_namespace: TPhpNamespace,
        pub php_metadata_namespace: TPhpMetadataNamespace,
        pub ruby_package: TRubyPackage,
        pub uninterpreted_option: TUninterpretedOption,
    }
    #[derive(::std::default::Default)]
    pub struct MessageOptionsFields<
        TMessageSetWireFormat,
        TNoStandardDescriptorAccessor,
        TDeprecated,
        TMapEntry,
        TUninterpretedOption,
    > {
        pub message_set_wire_format: TMessageSetWireFormat,
        pub no_standard_descriptor_accessor: TNoStandardDescriptorAccessor,
        pub deprecated: TDeprecated,
        pub map_entry: TMapEntry,
        pub uninterpreted_option: TUninterpretedOption,
    }
    #[derive(::std::default::Default)]
    pub struct FieldOptionsFields<
        TCtype,
        TPacked,
        TJstype,
        TLazy,
        TDeprecated,
        TWeak,
        TUninterpretedOption,
    > {
        pub ctype: TCtype,
        pub packed: TPacked,
        pub jstype: TJstype,
        pub lazy: TLazy,
        pub deprecated: TDeprecated,
        pub weak: TWeak,
        pub uninterpreted_option: TUninterpretedOption,
    }
    #[derive(::std::default::Default)]
    pub struct OneofOptionsFields<TUninterpretedOption> {
        pub uninterpreted_option: TUninterpretedOption,
    }
    #[derive(::std::default::Default)]
    pub struct EnumOptionsFields<TAllowAlias, TDeprecated, TUninterpretedOption> {
        pub allow_alias: TAllowAlias,
        pub deprecated: TDeprecated,
        pub uninterpreted_option: TUninterpretedOption,
    }
    #[derive(::std::default::Default)]
    pub struct EnumValueOptionsFields<TDeprecated, TUninterpretedOption> {
        pub deprecated: TDeprecated,
        pub uninterpreted_option: TUninterpretedOption,
    }
    #[derive(::std::default::Default)]
    pub struct ServiceOptionsFields<TDeprecated, TUninterpretedOption> {
        pub deprecated: TDeprecated,
        pub uninterpreted_option: TUninterpretedOption,
    }
    #[derive(::std::default::Default)]
    pub struct MethodOptionsFields<
        TDeprecated,
        TIdempotencyLevel,
        TUninterpretedOption,
    > {
        pub deprecated: TDeprecated,
        pub idempotency_level: TIdempotencyLevel,
        pub uninterpreted_option: TUninterpretedOption,
    }
    #[derive(::std::default::Default)]
    pub struct UninterpretedOptionFields<
        TName,
        TIdentifierValue,
        TPositiveIntValue,
        TNegativeIntValue,
        TDoubleValue,
        TStringValue,
        TAggregateValue,
    > {
        pub name: TName,
        pub identifier_value: TIdentifierValue,
        pub positive_int_value: TPositiveIntValue,
        pub negative_int_value: TNegativeIntValue,
        pub double_value: TDoubleValue,
        pub string_value: TStringValue,
        pub aggregate_value: TAggregateValue,
    }
    #[derive(::std::default::Default)]
    pub struct SourceCodeInfoFields<TLocation> {
        pub location: TLocation,
    }
    #[derive(::std::default::Default)]
    pub struct GeneratedCodeInfoFields<TAnnotation> {
        pub annotation: TAnnotation,
    }
}
pub use self::_fields::*;
