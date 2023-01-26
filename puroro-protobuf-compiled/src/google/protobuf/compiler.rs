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
pub mod code_generator_response;
#[derive(::std::default::Default)]
pub struct Version {
    fields: self::_root::google::protobuf::compiler::_fields::VersionFields<
        self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        >,
        self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            1usize,
        >,
        self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            2usize,
        >,
        self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            3usize,
        >,
    >,
    shared: self::_pinternal::SharedItemsImpl<1usize>,
}
impl Version {
    pub fn major(&self) -> i32 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.major,
            self.shared.bitfield(),
            ::std::default::Default::default,
        )
    }
    pub fn major_opt(&self) -> ::std::option::Option::<i32> {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_opt(&self.fields.major, self.shared.bitfield())
    }
    pub fn major_mut(&mut self) -> &mut i32 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.major,
            self.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn has_major(&self) -> bool {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_opt(&self.fields.major, self.shared.bitfield())
            .is_some()
    }
    pub fn clear_major(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(&mut self.fields.major, self.shared.bitfield_mut())
    }
    pub fn minor(&self) -> i32 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.minor,
            self.shared.bitfield(),
            ::std::default::Default::default,
        )
    }
    pub fn minor_opt(&self) -> ::std::option::Option::<i32> {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_opt(&self.fields.minor, self.shared.bitfield())
    }
    pub fn minor_mut(&mut self) -> &mut i32 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.minor,
            self.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn has_minor(&self) -> bool {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_opt(&self.fields.minor, self.shared.bitfield())
            .is_some()
    }
    pub fn clear_minor(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(&mut self.fields.minor, self.shared.bitfield_mut())
    }
    pub fn patch(&self) -> i32 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.patch,
            self.shared.bitfield(),
            ::std::default::Default::default,
        )
    }
    pub fn patch_opt(&self) -> ::std::option::Option::<i32> {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_opt(&self.fields.patch, self.shared.bitfield())
    }
    pub fn patch_mut(&mut self) -> &mut i32 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.patch,
            self.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn has_patch(&self) -> bool {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_opt(&self.fields.patch, self.shared.bitfield())
            .is_some()
    }
    pub fn clear_patch(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(&mut self.fields.patch, self.shared.bitfield_mut())
    }
    pub fn suffix(&self) -> &str {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.suffix,
            self.shared.bitfield(),
            ::std::default::Default::default,
        )
    }
    pub fn suffix_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_opt(&self.fields.suffix, self.shared.bitfield())
    }
    pub fn suffix_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.suffix,
            self.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn has_suffix(&self) -> bool {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_opt(&self.fields.suffix, self.shared.bitfield())
            .is_some()
    }
    pub fn clear_suffix(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(&mut self.fields.suffix, self.shared.bitfield_mut())
    }
}
impl self::_puroro::Message for Version {
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
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        use self::_puroro::PuroroError;
        while let Some((number, mut field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    1i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.major,
                            self.shared.bitfield_mut(),
                            &mut field_data,
                        )?
                    }
                    2i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.minor,
                            self.shared.bitfield_mut(),
                            &mut field_data,
                        )?
                    }
                    3i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.patch,
                            self.shared.bitfield_mut(),
                            &mut field_data,
                        )?
                    }
                    4i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.suffix,
                            self.shared.bitfield_mut(),
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
                    self.shared.unknown_fields_mut().push(number, field_data)?;
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
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.major,
            self.shared.bitfield(),
            1i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.minor,
            self.shared.bitfield(),
            2i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.patch,
            self.shared.bitfield(),
            3i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.suffix,
            self.shared.bitfield(),
            4i32,
            out,
        )?;
        self.shared.unknown_fields().ser_to_write(out)?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for Version {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use self::_pinternal::SharedItems as _;
        Self {
            fields: self::_fields::VersionFields {
                major: ::std::clone::Clone::clone(&self.fields.major),
                minor: ::std::clone::Clone::clone(&self.fields.minor),
                patch: ::std::clone::Clone::clone(&self.fields.patch),
                suffix: ::std::clone::Clone::clone(&self.fields.suffix),
            },
            shared: ::std::clone::Clone::clone(&self.shared),
        }
    }
}
impl ::std::ops::Drop for Version {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::{OneofUnion as _, SharedItems as _};
    }
}
impl ::std::fmt::Debug for Version {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        let mut debug_struct = fmt.debug_struct(stringify!(Version));
        debug_struct
            .field(stringify!(major), &self.major_opt())
            .field(stringify!(minor), &self.minor_opt())
            .field(stringify!(patch), &self.patch_opt())
            .field(stringify!(suffix), &self.suffix_opt());
        self.shared.unknown_fields().debug_struct_fields(&mut debug_struct)?;
        debug_struct.finish()
    }
}
impl ::std::cmp::PartialEq for Version {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::SharedItems as _;
        true && self.major_opt() == rhs.major_opt()
            && self.minor_opt() == rhs.minor_opt() && self.patch_opt() == rhs.patch_opt()
            && self.suffix_opt() == rhs.suffix_opt()
            && self.shared.unknown_fields() == rhs.shared.unknown_fields()
    }
}
#[derive(::std::default::Default)]
pub struct CodeGeneratorRequest {
    fields: self::_root::google::protobuf::compiler::_fields::CodeGeneratorRequestFields<
        self::_pinternal::RepeatedUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        >,
        self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        >,
        self::_pinternal::RepeatedMessageField::<
            self::_root::google::protobuf::FileDescriptorProto,
        >,
        self::_pinternal::SingularHeapMessageField::<
            self::_root::google::protobuf::compiler::Version,
        >,
    >,
    shared: self::_pinternal::SharedItemsImpl<1usize>,
}
impl CodeGeneratorRequest {
    pub fn file_to_generate(
        &self,
    ) -> &[impl ::std::ops::Deref::<
        Target = str,
    > + ::std::fmt::Debug + ::std::cmp::PartialEq] {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field(
            &self.fields.file_to_generate,
            self.shared.bitfield(),
        )
    }
    pub fn file_to_generate_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<::std::string::String> {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.fields.file_to_generate,
            self.shared.bitfield_mut(),
        )
    }
    pub fn clear_file_to_generate(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(
            &mut self.fields.file_to_generate,
            self.shared.bitfield_mut(),
        )
    }
    pub fn parameter(&self) -> &str {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.parameter,
            self.shared.bitfield(),
            ::std::default::Default::default,
        )
    }
    pub fn parameter_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_opt(
            &self.fields.parameter,
            self.shared.bitfield(),
        )
    }
    pub fn parameter_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.parameter,
            self.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn has_parameter(&self) -> bool {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_opt(
                &self.fields.parameter,
                self.shared.bitfield(),
            )
            .is_some()
    }
    pub fn clear_parameter(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(
            &mut self.fields.parameter,
            self.shared.bitfield_mut(),
        )
    }
    pub fn proto_file(&self) -> &[self::_root::google::protobuf::FileDescriptorProto] {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field(&self.fields.proto_file, self.shared.bitfield())
    }
    pub fn proto_file_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::FileDescriptorProto> {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.fields.proto_file,
            self.shared.bitfield_mut(),
        )
    }
    pub fn clear_proto_file(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(&mut self.fields.proto_file, self.shared.bitfield_mut())
    }
    pub fn compiler_version(
        &self,
    ) -> ::std::option::Option::<&self::_root::google::protobuf::compiler::Version> {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.compiler_version,
            self.shared.bitfield(),
            ::std::default::Default::default,
        )
    }
    pub fn compiler_version_opt(
        &self,
    ) -> ::std::option::Option::<&self::_root::google::protobuf::compiler::Version> {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_opt(
            &self.fields.compiler_version,
            self.shared.bitfield(),
        )
    }
    pub fn compiler_version_mut(
        &mut self,
    ) -> &mut self::_root::google::protobuf::compiler::Version {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.compiler_version,
            self.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn has_compiler_version(&self) -> bool {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_opt(
                &self.fields.compiler_version,
                self.shared.bitfield(),
            )
            .is_some()
    }
    pub fn clear_compiler_version(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(
            &mut self.fields.compiler_version,
            self.shared.bitfield_mut(),
        )
    }
}
impl self::_puroro::Message for CodeGeneratorRequest {
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
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        use self::_puroro::PuroroError;
        while let Some((number, mut field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    1i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.file_to_generate,
                            self.shared.bitfield_mut(),
                            &mut field_data,
                        )?
                    }
                    2i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.parameter,
                            self.shared.bitfield_mut(),
                            &mut field_data,
                        )?
                    }
                    15i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.proto_file,
                            self.shared.bitfield_mut(),
                            &mut field_data,
                        )?
                    }
                    3i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.compiler_version,
                            self.shared.bitfield_mut(),
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
                    self.shared.unknown_fields_mut().push(number, field_data)?;
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
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.file_to_generate,
            self.shared.bitfield(),
            1i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.parameter,
            self.shared.bitfield(),
            2i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.proto_file,
            self.shared.bitfield(),
            15i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.compiler_version,
            self.shared.bitfield(),
            3i32,
            out,
        )?;
        self.shared.unknown_fields().ser_to_write(out)?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for CodeGeneratorRequest {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use self::_pinternal::SharedItems as _;
        Self {
            fields: self::_fields::CodeGeneratorRequestFields {
                file_to_generate: ::std::clone::Clone::clone(
                    &self.fields.file_to_generate,
                ),
                parameter: ::std::clone::Clone::clone(&self.fields.parameter),
                proto_file: ::std::clone::Clone::clone(&self.fields.proto_file),
                compiler_version: ::std::clone::Clone::clone(
                    &self.fields.compiler_version,
                ),
            },
            shared: ::std::clone::Clone::clone(&self.shared),
        }
    }
}
impl ::std::ops::Drop for CodeGeneratorRequest {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::{OneofUnion as _, SharedItems as _};
    }
}
impl ::std::fmt::Debug for CodeGeneratorRequest {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        let mut debug_struct = fmt.debug_struct(stringify!(CodeGeneratorRequest));
        debug_struct
            .field(stringify!(file_to_generate), &self.file_to_generate())
            .field(stringify!(parameter), &self.parameter_opt())
            .field(stringify!(proto_file), &self.proto_file())
            .field(stringify!(compiler_version), &self.compiler_version_opt());
        self.shared.unknown_fields().debug_struct_fields(&mut debug_struct)?;
        debug_struct.finish()
    }
}
impl ::std::cmp::PartialEq for CodeGeneratorRequest {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::SharedItems as _;
        true && self.file_to_generate() == rhs.file_to_generate()
            && self.parameter_opt() == rhs.parameter_opt()
            && self.proto_file() == rhs.proto_file()
            && self.compiler_version_opt() == rhs.compiler_version_opt()
            && self.shared.unknown_fields() == rhs.shared.unknown_fields()
    }
}
#[derive(::std::default::Default)]
pub struct CodeGeneratorResponse {
    fields: self::_root::google::protobuf::compiler::_fields::CodeGeneratorResponseFields<
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
        self::_pinternal::RepeatedMessageField::<
            self::_root::google::protobuf::compiler::code_generator_response::File,
        >,
    >,
    shared: self::_pinternal::SharedItemsImpl<1usize>,
}
impl CodeGeneratorResponse {
    pub fn error(&self) -> &str {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.error,
            self.shared.bitfield(),
            ::std::default::Default::default,
        )
    }
    pub fn error_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_opt(&self.fields.error, self.shared.bitfield())
    }
    pub fn error_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.error,
            self.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn has_error(&self) -> bool {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_opt(&self.fields.error, self.shared.bitfield())
            .is_some()
    }
    pub fn clear_error(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(&mut self.fields.error, self.shared.bitfield_mut())
    }
    pub fn supported_features(&self) -> u64 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_or_else(
            &self.fields.supported_features,
            self.shared.bitfield(),
            ::std::default::Default::default,
        )
    }
    pub fn supported_features_opt(&self) -> ::std::option::Option::<u64> {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_opt(
            &self.fields.supported_features,
            self.shared.bitfield(),
        )
    }
    pub fn supported_features_mut(&mut self) -> &mut u64 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.supported_features,
            self.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn has_supported_features(&self) -> bool {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_opt(
                &self.fields.supported_features,
                self.shared.bitfield(),
            )
            .is_some()
    }
    pub fn clear_supported_features(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(
            &mut self.fields.supported_features,
            self.shared.bitfield_mut(),
        )
    }
    pub fn file(
        &self,
    ) -> &[self::_root::google::protobuf::compiler::code_generator_response::File] {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field(&self.fields.file, self.shared.bitfield())
    }
    pub fn file_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<
        self::_root::google::protobuf::compiler::code_generator_response::File,
    > {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.fields.file,
            self.shared.bitfield_mut(),
        )
    }
    pub fn clear_file(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(&mut self.fields.file, self.shared.bitfield_mut())
    }
}
impl self::_puroro::Message for CodeGeneratorResponse {
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
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        use self::_puroro::PuroroError;
        while let Some((number, mut field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    1i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.error,
                            self.shared.bitfield_mut(),
                            &mut field_data,
                        )?
                    }
                    2i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.supported_features,
                            self.shared.bitfield_mut(),
                            &mut field_data,
                        )?
                    }
                    15i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.file,
                            self.shared.bitfield_mut(),
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
                    self.shared.unknown_fields_mut().push(number, field_data)?;
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
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.error,
            self.shared.bitfield(),
            1i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.supported_features,
            self.shared.bitfield(),
            2i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.fields.file,
            self.shared.bitfield(),
            15i32,
            out,
        )?;
        self.shared.unknown_fields().ser_to_write(out)?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for CodeGeneratorResponse {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use self::_pinternal::SharedItems as _;
        Self {
            fields: self::_fields::CodeGeneratorResponseFields {
                error: ::std::clone::Clone::clone(&self.fields.error),
                supported_features: ::std::clone::Clone::clone(
                    &self.fields.supported_features,
                ),
                file: ::std::clone::Clone::clone(&self.fields.file),
            },
            shared: ::std::clone::Clone::clone(&self.shared),
        }
    }
}
impl ::std::ops::Drop for CodeGeneratorResponse {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::{OneofUnion as _, SharedItems as _};
    }
}
impl ::std::fmt::Debug for CodeGeneratorResponse {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        let mut debug_struct = fmt.debug_struct(stringify!(CodeGeneratorResponse));
        debug_struct
            .field(stringify!(error), &self.error_opt())
            .field(stringify!(supported_features), &self.supported_features_opt())
            .field(stringify!(file), &self.file());
        self.shared.unknown_fields().debug_struct_fields(&mut debug_struct)?;
        debug_struct.finish()
    }
}
impl ::std::cmp::PartialEq for CodeGeneratorResponse {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::SharedItems as _;
        true && self.error_opt() == rhs.error_opt()
            && self.supported_features_opt() == rhs.supported_features_opt()
            && self.file() == rhs.file()
            && self.shared.unknown_fields() == rhs.shared.unknown_fields()
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
    pub struct VersionFields<TMajor, TMinor, TPatch, TSuffix> {
        pub major: TMajor,
        pub minor: TMinor,
        pub patch: TPatch,
        pub suffix: TSuffix,
    }
    #[derive(::std::default::Default)]
    pub struct CodeGeneratorRequestFields<
        TFileToGenerate,
        TParameter,
        TProtoFile,
        TCompilerVersion,
    > {
        pub file_to_generate: TFileToGenerate,
        pub parameter: TParameter,
        pub proto_file: TProtoFile,
        pub compiler_version: TCompilerVersion,
    }
    #[derive(::std::default::Default)]
    pub struct CodeGeneratorResponseFields<TError, TSupportedFeatures, TFile> {
        pub error: TError,
        pub supported_features: TSupportedFeatures,
        pub file: TFile,
    }
}
pub use self::_fields::*;
