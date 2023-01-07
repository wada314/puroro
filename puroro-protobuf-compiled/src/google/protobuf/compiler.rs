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
    bitfield: self::_pinternal::BitArray<1usize>,
}
impl Version {
    pub fn major(&self) -> i32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.fields.major,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn major_opt(&self) -> ::std::option::Option::<i32> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.fields.major, &self.bitfield)
    }
    pub fn major_mut(&mut self) -> &mut i32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.fields.major,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_major(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.fields.major, &self.bitfield)
            .is_some()
    }
    pub fn clear_major(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::clear(&mut self.fields.major, &mut self.bitfield)
    }
    pub fn minor(&self) -> i32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            1usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.fields.minor,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn minor_opt(&self) -> ::std::option::Option::<i32> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.fields.minor, &self.bitfield)
    }
    pub fn minor_mut(&mut self) -> &mut i32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            1usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.fields.minor,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_minor(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.fields.minor, &self.bitfield)
            .is_some()
    }
    pub fn clear_minor(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            1usize,
        > as NonRepeatedFieldType>::clear(&mut self.fields.minor, &mut self.bitfield)
    }
    pub fn patch(&self) -> i32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            2usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.fields.patch,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn patch_opt(&self) -> ::std::option::Option::<i32> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            2usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.fields.patch, &self.bitfield)
    }
    pub fn patch_mut(&mut self) -> &mut i32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            2usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.fields.patch,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_patch(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            2usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.fields.patch, &self.bitfield)
            .is_some()
    }
    pub fn clear_patch(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            2usize,
        > as NonRepeatedFieldType>::clear(&mut self.fields.patch, &mut self.bitfield)
    }
    pub fn suffix(&self) -> &str {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            3usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.fields.suffix,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn suffix_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            3usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.fields.suffix, &self.bitfield)
    }
    pub fn suffix_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            3usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.fields.suffix,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_suffix(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            3usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.fields.suffix, &self.bitfield)
            .is_some()
    }
    pub fn clear_suffix(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            3usize,
        > as NonRepeatedFieldType>::clear(&mut self.fields.suffix, &mut self.bitfield)
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
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            match number {
                1i32 => {
                    <self::_pinternal::OptionalNumericalField::<
                        i32,
                        self::_pinternal::tags::Int32,
                        0usize,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.fields.major,
                        &mut self.bitfield,
                        field_data,
                    )?
                }
                2i32 => {
                    <self::_pinternal::OptionalNumericalField::<
                        i32,
                        self::_pinternal::tags::Int32,
                        1usize,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.fields.minor,
                        &mut self.bitfield,
                        field_data,
                    )?
                }
                3i32 => {
                    <self::_pinternal::OptionalNumericalField::<
                        i32,
                        self::_pinternal::tags::Int32,
                        2usize,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.fields.patch,
                        &mut self.bitfield,
                        field_data,
                    )?
                }
                4i32 => {
                    <self::_pinternal::OptionalUnsizedField::<
                        ::std::string::String,
                        self::_pinternal::tags::String,
                        3usize,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.fields.suffix,
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
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.fields.major,
            &self.bitfield,
            1i32,
            out,
        )?;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            1usize,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.fields.minor,
            &self.bitfield,
            2i32,
            out,
        )?;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            2usize,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.fields.patch,
            &self.bitfield,
            3i32,
            out,
        )?;
        <self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            3usize,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.fields.suffix,
            &self.bitfield,
            4i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for Version {
    fn clone(&self) -> Self {
        Self {
            fields: self::_fields::VersionFields {
                major: <self::_pinternal::OptionalNumericalField::<
                    i32,
                    self::_pinternal::tags::Int32,
                    0usize,
                > as ::std::clone::Clone>::clone(&self.fields.major),
                minor: <self::_pinternal::OptionalNumericalField::<
                    i32,
                    self::_pinternal::tags::Int32,
                    1usize,
                > as ::std::clone::Clone>::clone(&self.fields.minor),
                patch: <self::_pinternal::OptionalNumericalField::<
                    i32,
                    self::_pinternal::tags::Int32,
                    2usize,
                > as ::std::clone::Clone>::clone(&self.fields.patch),
                suffix: <self::_pinternal::OptionalUnsizedField::<
                    ::std::string::String,
                    self::_pinternal::tags::String,
                    3usize,
                > as ::std::clone::Clone>::clone(&self.fields.suffix),
            },
            bitfield: ::std::clone::Clone::clone(&self.bitfield),
        }
    }
}
impl ::std::ops::Drop for Version {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for Version {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(Version))
            .field(stringify!(major), &self.major_opt())
            .field(stringify!(minor), &self.minor_opt())
            .field(stringify!(patch), &self.patch_opt())
            .field(stringify!(suffix), &self.suffix_opt())
            .finish()
    }
}
impl ::std::cmp::PartialEq for Version {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        true && self.major_opt() == rhs.major_opt()
            && self.minor_opt() == rhs.minor_opt() && self.patch_opt() == rhs.patch_opt()
            && self.suffix_opt() == rhs.suffix_opt()
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
    bitfield: self::_pinternal::BitArray<1usize>,
}
impl CodeGeneratorRequest {
    pub fn file_to_generate(
        &self,
    ) -> &[impl ::std::ops::Deref::<
        Target = str,
    > + ::std::fmt::Debug + ::std::cmp::PartialEq] {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as RepeatedFieldType>::get_field(&self.fields.file_to_generate, &self.bitfield)
    }
    pub fn file_to_generate_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<::std::string::String> {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as RepeatedFieldType>::get_field_mut(
            &mut self.fields.file_to_generate,
            &mut self.bitfield,
        )
    }
    pub fn clear_file_to_generate(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as RepeatedFieldType>::clear(
            &mut self.fields.file_to_generate,
            &mut self.bitfield,
        )
    }
    pub fn parameter(&self) -> &str {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.fields.parameter,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn parameter_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.fields.parameter, &self.bitfield)
    }
    pub fn parameter_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.fields.parameter,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_parameter(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.fields.parameter, &self.bitfield)
            .is_some()
    }
    pub fn clear_parameter(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::clear(&mut self.fields.parameter, &mut self.bitfield)
    }
    pub fn proto_file(&self) -> &[self::_root::google::protobuf::FileDescriptorProto] {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedMessageField::<
            self::_root::google::protobuf::FileDescriptorProto,
        > as RepeatedFieldType>::get_field(&self.fields.proto_file, &self.bitfield)
    }
    pub fn proto_file_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::FileDescriptorProto> {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedMessageField::<
            self::_root::google::protobuf::FileDescriptorProto,
        > as RepeatedFieldType>::get_field_mut(
            &mut self.fields.proto_file,
            &mut self.bitfield,
        )
    }
    pub fn clear_proto_file(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedMessageField::<
            self::_root::google::protobuf::FileDescriptorProto,
        > as RepeatedFieldType>::clear(&mut self.fields.proto_file, &mut self.bitfield)
    }
    pub fn compiler_version(
        &self,
    ) -> ::std::option::Option::<&self::_root::google::protobuf::compiler::Version> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularHeapMessageField::<
            self::_root::google::protobuf::compiler::Version,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.fields.compiler_version,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn compiler_version_opt(
        &self,
    ) -> ::std::option::Option::<&self::_root::google::protobuf::compiler::Version> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularHeapMessageField::<
            self::_root::google::protobuf::compiler::Version,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.fields.compiler_version,
            &self.bitfield,
        )
    }
    pub fn compiler_version_mut(
        &mut self,
    ) -> &mut self::_root::google::protobuf::compiler::Version {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularHeapMessageField::<
            self::_root::google::protobuf::compiler::Version,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.fields.compiler_version,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_compiler_version(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularHeapMessageField::<
            self::_root::google::protobuf::compiler::Version,
        > as NonRepeatedFieldType>::get_field_opt(
                &self.fields.compiler_version,
                &self.bitfield,
            )
            .is_some()
    }
    pub fn clear_compiler_version(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularHeapMessageField::<
            self::_root::google::protobuf::compiler::Version,
        > as NonRepeatedFieldType>::clear(
            &mut self.fields.compiler_version,
            &mut self.bitfield,
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
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            match number {
                1i32 => {
                    <self::_pinternal::RepeatedUnsizedField::<
                        ::std::string::String,
                        self::_pinternal::tags::String,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.fields.file_to_generate,
                        &mut self.bitfield,
                        field_data,
                    )?
                }
                2i32 => {
                    <self::_pinternal::OptionalUnsizedField::<
                        ::std::string::String,
                        self::_pinternal::tags::String,
                        0usize,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.fields.parameter,
                        &mut self.bitfield,
                        field_data,
                    )?
                }
                15i32 => {
                    <self::_pinternal::RepeatedMessageField::<
                        self::_root::google::protobuf::FileDescriptorProto,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.fields.proto_file,
                        &mut self.bitfield,
                        field_data,
                    )?
                }
                3i32 => {
                    <self::_pinternal::SingularHeapMessageField::<
                        self::_root::google::protobuf::compiler::Version,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.fields.compiler_version,
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
        <self::_pinternal::RepeatedUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.fields.file_to_generate,
            &self.bitfield,
            1i32,
            out,
        )?;
        <self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.fields.parameter,
            &self.bitfield,
            2i32,
            out,
        )?;
        <self::_pinternal::RepeatedMessageField::<
            self::_root::google::protobuf::FileDescriptorProto,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.fields.proto_file,
            &self.bitfield,
            15i32,
            out,
        )?;
        <self::_pinternal::SingularHeapMessageField::<
            self::_root::google::protobuf::compiler::Version,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.fields.compiler_version,
            &self.bitfield,
            3i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for CodeGeneratorRequest {
    fn clone(&self) -> Self {
        Self {
            fields: self::_fields::CodeGeneratorRequestFields {
                file_to_generate: <self::_pinternal::RepeatedUnsizedField::<
                    ::std::string::String,
                    self::_pinternal::tags::String,
                > as ::std::clone::Clone>::clone(&self.fields.file_to_generate),
                parameter: <self::_pinternal::OptionalUnsizedField::<
                    ::std::string::String,
                    self::_pinternal::tags::String,
                    0usize,
                > as ::std::clone::Clone>::clone(&self.fields.parameter),
                proto_file: <self::_pinternal::RepeatedMessageField::<
                    self::_root::google::protobuf::FileDescriptorProto,
                > as ::std::clone::Clone>::clone(&self.fields.proto_file),
                compiler_version: <self::_pinternal::SingularHeapMessageField::<
                    self::_root::google::protobuf::compiler::Version,
                > as ::std::clone::Clone>::clone(&self.fields.compiler_version),
            },
            bitfield: ::std::clone::Clone::clone(&self.bitfield),
        }
    }
}
impl ::std::ops::Drop for CodeGeneratorRequest {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for CodeGeneratorRequest {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(CodeGeneratorRequest))
            .field(stringify!(file_to_generate), &self.file_to_generate())
            .field(stringify!(parameter), &self.parameter_opt())
            .field(stringify!(proto_file), &self.proto_file())
            .field(stringify!(compiler_version), &self.compiler_version_opt())
            .finish()
    }
}
impl ::std::cmp::PartialEq for CodeGeneratorRequest {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        true && self.file_to_generate() == rhs.file_to_generate()
            && self.parameter_opt() == rhs.parameter_opt()
            && self.proto_file() == rhs.proto_file()
            && self.compiler_version_opt() == rhs.compiler_version_opt()
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
    bitfield: self::_pinternal::BitArray<1usize>,
}
impl CodeGeneratorResponse {
    pub fn error(&self) -> &str {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.fields.error,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn error_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.fields.error, &self.bitfield)
    }
    pub fn error_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.fields.error,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_error(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.fields.error, &self.bitfield)
            .is_some()
    }
    pub fn clear_error(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::clear(&mut self.fields.error, &mut self.bitfield)
    }
    pub fn supported_features(&self) -> u64 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            u64,
            self::_pinternal::tags::UInt64,
            1usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.fields.supported_features,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn supported_features_opt(&self) -> ::std::option::Option::<u64> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            u64,
            self::_pinternal::tags::UInt64,
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.fields.supported_features,
            &self.bitfield,
        )
    }
    pub fn supported_features_mut(&mut self) -> &mut u64 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            u64,
            self::_pinternal::tags::UInt64,
            1usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.fields.supported_features,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_supported_features(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            u64,
            self::_pinternal::tags::UInt64,
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(
                &self.fields.supported_features,
                &self.bitfield,
            )
            .is_some()
    }
    pub fn clear_supported_features(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            u64,
            self::_pinternal::tags::UInt64,
            1usize,
        > as NonRepeatedFieldType>::clear(
            &mut self.fields.supported_features,
            &mut self.bitfield,
        )
    }
    pub fn file(
        &self,
    ) -> &[self::_root::google::protobuf::compiler::code_generator_response::File] {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedMessageField::<
            self::_root::google::protobuf::compiler::code_generator_response::File,
        > as RepeatedFieldType>::get_field(&self.fields.file, &self.bitfield)
    }
    pub fn file_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<
        self::_root::google::protobuf::compiler::code_generator_response::File,
    > {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedMessageField::<
            self::_root::google::protobuf::compiler::code_generator_response::File,
        > as RepeatedFieldType>::get_field_mut(&mut self.fields.file, &mut self.bitfield)
    }
    pub fn clear_file(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedMessageField::<
            self::_root::google::protobuf::compiler::code_generator_response::File,
        > as RepeatedFieldType>::clear(&mut self.fields.file, &mut self.bitfield)
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
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            match number {
                1i32 => {
                    <self::_pinternal::OptionalUnsizedField::<
                        ::std::string::String,
                        self::_pinternal::tags::String,
                        0usize,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.fields.error,
                        &mut self.bitfield,
                        field_data,
                    )?
                }
                2i32 => {
                    <self::_pinternal::OptionalNumericalField::<
                        u64,
                        self::_pinternal::tags::UInt64,
                        1usize,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.fields.supported_features,
                        &mut self.bitfield,
                        field_data,
                    )?
                }
                15i32 => {
                    <self::_pinternal::RepeatedMessageField::<
                        self::_root::google::protobuf::compiler::code_generator_response::File,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.fields.file,
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
        <self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.fields.error,
            &self.bitfield,
            1i32,
            out,
        )?;
        <self::_pinternal::OptionalNumericalField::<
            u64,
            self::_pinternal::tags::UInt64,
            1usize,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.fields.supported_features,
            &self.bitfield,
            2i32,
            out,
        )?;
        <self::_pinternal::RepeatedMessageField::<
            self::_root::google::protobuf::compiler::code_generator_response::File,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.fields.file,
            &self.bitfield,
            15i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for CodeGeneratorResponse {
    fn clone(&self) -> Self {
        Self {
            fields: self::_fields::CodeGeneratorResponseFields {
                error: <self::_pinternal::OptionalUnsizedField::<
                    ::std::string::String,
                    self::_pinternal::tags::String,
                    0usize,
                > as ::std::clone::Clone>::clone(&self.fields.error),
                supported_features: <self::_pinternal::OptionalNumericalField::<
                    u64,
                    self::_pinternal::tags::UInt64,
                    1usize,
                > as ::std::clone::Clone>::clone(&self.fields.supported_features),
                file: <self::_pinternal::RepeatedMessageField::<
                    self::_root::google::protobuf::compiler::code_generator_response::File,
                > as ::std::clone::Clone>::clone(&self.fields.file),
            },
            bitfield: ::std::clone::Clone::clone(&self.bitfield),
        }
    }
}
impl ::std::ops::Drop for CodeGeneratorResponse {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for CodeGeneratorResponse {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(CodeGeneratorResponse))
            .field(stringify!(error), &self.error_opt())
            .field(stringify!(supported_features), &self.supported_features_opt())
            .field(stringify!(file), &self.file())
            .finish()
    }
}
impl ::std::cmp::PartialEq for CodeGeneratorResponse {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        true && self.error_opt() == rhs.error_opt()
            && self.supported_features_opt() == rhs.supported_features_opt()
            && self.file() == rhs.file()
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
