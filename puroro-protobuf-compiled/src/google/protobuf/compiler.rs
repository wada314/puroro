// A generated source code by puroro library
// package google.protobuf.compiler

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub mod _puroro {
    pub use ::puroro::*;
}

#[derive(Default)]
pub struct Version {
    // Optional, Variant(Int32)
    major: self::_puroro::internal::field_type::OptionalNumericalField<
        i32,
        self::_puroro::tags::Int32,
        0,
    >,
    // Optional, Variant(Int32)
    minor: self::_puroro::internal::field_type::OptionalNumericalField<
        i32,
        self::_puroro::tags::Int32,
        1,
    >,
    // Optional, Variant(Int32)
    patch: self::_puroro::internal::field_type::OptionalNumericalField<
        i32,
        self::_puroro::tags::Int32,
        2,
    >,
    // Optional, LengthDelimited(String)
    suffix: self::_puroro::internal::field_type::OptionalStringField<3>,

    _bitfield: self::_puroro::bitvec::BitArray<1>,
}

impl Version {
    // Optional, Variant(Int32)
    pub fn major(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as NonRepeatedFieldType>::get_field(
            &self.major,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn major_opt(&self) -> ::std::option::Option<i32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as NonRepeatedFieldType>::get_field_opt(&self.major, &self._bitfield)
    }
    pub fn has_major(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as NonRepeatedFieldType>::get_field_opt(&self.major, &self._bitfield)
        .is_some()
    }
    pub fn major_mut(&mut self) -> &mut i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.major,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_major(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as NonRepeatedFieldType>::clear(&mut self.major, &mut self._bitfield)
    }
    // Optional, Variant(Int32)
    pub fn minor(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            1,
        > as NonRepeatedFieldType>::get_field(
            &self.minor,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn minor_opt(&self) -> ::std::option::Option<i32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            1,
        > as NonRepeatedFieldType>::get_field_opt(&self.minor, &self._bitfield)
    }
    pub fn has_minor(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            1,
        > as NonRepeatedFieldType>::get_field_opt(&self.minor, &self._bitfield)
        .is_some()
    }
    pub fn minor_mut(&mut self) -> &mut i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            1,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.minor,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_minor(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            1,
        > as NonRepeatedFieldType>::clear(&mut self.minor, &mut self._bitfield)
    }
    // Optional, Variant(Int32)
    pub fn patch(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            2,
        > as NonRepeatedFieldType>::get_field(
            &self.patch,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn patch_opt(&self) -> ::std::option::Option<i32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            2,
        > as NonRepeatedFieldType>::get_field_opt(&self.patch, &self._bitfield)
    }
    pub fn has_patch(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            2,
        > as NonRepeatedFieldType>::get_field_opt(&self.patch, &self._bitfield)
        .is_some()
    }
    pub fn patch_mut(&mut self) -> &mut i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            2,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.patch,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_patch(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            2,
        > as NonRepeatedFieldType>::clear(&mut self.patch, &mut self._bitfield)
    }
    // Optional, LengthDelimited(String)
    pub fn suffix(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<3> as NonRepeatedFieldType>::get_field(
            &self.suffix, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn suffix_opt(&self) -> ::std::option::Option<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<3> as NonRepeatedFieldType>::get_field_opt(
            &self.suffix, &self._bitfield,
        )
    }
    pub fn has_suffix(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<3> as NonRepeatedFieldType>::get_field_opt(
            &self.suffix, &self._bitfield,
        ).is_some()
    }
    pub fn suffix_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<3> as NonRepeatedFieldType>::mut_field(
            &mut self.suffix, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_suffix(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField<3> as NonRepeatedFieldType>::clear(
            &mut self.suffix,
            &mut self._bitfield,
        )
    }
}

impl self::_puroro::Message for Version {
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
                1 => <self::_puroro::internal::field_type::OptionalNumericalField::<i32, self::_puroro::tags::Int32, 0> as FieldType>::deser_from_iter(
                    &mut self.major,
                    &mut self._bitfield,
                    field_data,
                )?,
                2 => <self::_puroro::internal::field_type::OptionalNumericalField::<i32, self::_puroro::tags::Int32, 1> as FieldType>::deser_from_iter(
                    &mut self.minor,
                    &mut self._bitfield,
                    field_data,
                )?,
                3 => <self::_puroro::internal::field_type::OptionalNumericalField::<i32, self::_puroro::tags::Int32, 2> as FieldType>::deser_from_iter(
                    &mut self.patch,
                    &mut self._bitfield,
                    field_data,
                )?,
                4 => <self::_puroro::internal::field_type::OptionalStringField::<3> as FieldType>::deser_from_iter(
                    &mut self.suffix,
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
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as FieldType>::ser_to_write(&self.major, &self._bitfield, 1, out)?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            1,
        > as FieldType>::ser_to_write(&self.minor, &self._bitfield, 2, out)?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            2,
        > as FieldType>::ser_to_write(&self.patch, &self._bitfield, 3, out)?;
        <self::_puroro::internal::field_type::OptionalStringField<3> as FieldType>::ser_to_write(
            &self.suffix,
            &self._bitfield,
            4,
            out,
        )?;

        Ok(())
    }
}

impl ::std::clone::Clone for Version {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::clone::Clone;
        Self {
            major: Clone::clone(&self.major),
            minor: Clone::clone(&self.minor),
            patch: Clone::clone(&self.patch),
            suffix: Clone::clone(&self.suffix),

            _bitfield: Clone::clone(&self._bitfield),
        }
    }
}

impl ::std::fmt::Debug for Version {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct("Version")
            .field("major", &self.major())
            .field("minor", &self.minor())
            .field("patch", &self.patch())
            .field("suffix", &self.suffix())
            .finish()
    }
}

impl ::std::cmp::PartialEq for Version {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;

        true && self.major_opt() == rhs.major_opt()
            && self.minor_opt() == rhs.minor_opt()
            && self.patch_opt() == rhs.patch_opt()
            && self.suffix_opt() == rhs.suffix_opt()
    }
}

impl ::std::ops::Drop for Version {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
    }
}

#[derive(Default)]
pub struct CodeGeneratorRequest {
    // Repeated, LengthDelimited(String)
    file_to_generate: self::_puroro::internal::field_type::RepeatedStringField,
    // Optional, LengthDelimited(String)
    parameter: self::_puroro::internal::field_type::OptionalStringField<0>,
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.FileDescriptorProto")))
    proto_file: self::_puroro::internal::field_type::RepeatedMessageField<
        _puroro_root::google::protobuf::FileDescriptorProto,
    >,
    // Optional, LengthDelimited(Message(Fqtn(".google.protobuf.compiler.Version")))
    compiler_version: self::_puroro::internal::field_type::SingularHeapMessageField<
        _puroro_root::google::protobuf::compiler::Version,
    >,

    _bitfield: self::_puroro::bitvec::BitArray<1>,
}

impl CodeGeneratorRequest {
    // Repeated, LengthDelimited(String)
    pub fn file_to_generate(&self) -> &[::std::string::String] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedStringField as RepeatedFieldType>::get_field(
            &self.file_to_generate,
            &self._bitfield,
        )
    }
    pub fn file_to_generate_mut(&mut self) -> &mut ::std::vec::Vec<::std::string::String> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedStringField as RepeatedFieldType>::mut_field(
            &mut self.file_to_generate,
            &mut self._bitfield,
        )
    }
    pub fn clear_file_to_generate(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedStringField as RepeatedFieldType>::clear(
            &mut self.file_to_generate,
            &mut self._bitfield,
        )
    }
    // Optional, LengthDelimited(String)
    pub fn parameter(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field(
            &self.parameter, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn parameter_opt(&self) -> ::std::option::Option<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field_opt(
            &self.parameter, &self._bitfield,
        )
    }
    pub fn has_parameter(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field_opt(
            &self.parameter, &self._bitfield,
        ).is_some()
    }
    pub fn parameter_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::mut_field(
            &mut self.parameter, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_parameter(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField<0> as NonRepeatedFieldType>::clear(
            &mut self.parameter,
            &mut self._bitfield,
        )
    }
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.FileDescriptorProto")))
    pub fn proto_file(&self) -> &[_puroro_root::google::protobuf::FileDescriptorProto] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::FileDescriptorProto,
        > as RepeatedFieldType>::get_field(&self.proto_file, &self._bitfield)
    }
    pub fn proto_file_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec<_puroro_root::google::protobuf::FileDescriptorProto> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::FileDescriptorProto,
        > as RepeatedFieldType>::mut_field(&mut self.proto_file, &mut self._bitfield)
    }
    pub fn clear_proto_file(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::FileDescriptorProto,
        > as RepeatedFieldType>::clear(&mut self.proto_file, &mut self._bitfield)
    }
    // Optional, LengthDelimited(Message(Fqtn(".google.protobuf.compiler.Version")))
    pub fn compiler_version(
        &self,
    ) -> ::std::option::Option<&_puroro_root::google::protobuf::compiler::Version> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::compiler::Version,
        > as NonRepeatedFieldType>::get_field(
            &self.compiler_version,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn compiler_version_opt(
        &self,
    ) -> ::std::option::Option<&_puroro_root::google::protobuf::compiler::Version> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::compiler::Version,
        > as NonRepeatedFieldType>::get_field_opt(&self.compiler_version, &self._bitfield)
    }
    pub fn has_compiler_version(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::compiler::Version,
        > as NonRepeatedFieldType>::get_field_opt(&self.compiler_version, &self._bitfield)
        .is_some()
    }
    pub fn compiler_version_mut(
        &mut self,
    ) -> &mut _puroro_root::google::protobuf::compiler::Version {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::compiler::Version,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.compiler_version,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_compiler_version(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::compiler::Version,
        > as NonRepeatedFieldType>::clear(&mut self.compiler_version, &mut self._bitfield)
    }
}

impl self::_puroro::Message for CodeGeneratorRequest {
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
                1 => <self::_puroro::internal::field_type::RepeatedStringField as FieldType>::deser_from_iter(
                    &mut self.file_to_generate,
                    &mut self._bitfield,
                    field_data,
                )?,
                2 => <self::_puroro::internal::field_type::OptionalStringField::<0> as FieldType>::deser_from_iter(
                    &mut self.parameter,
                    &mut self._bitfield,
                    field_data,
                )?,
                15 => <self::_puroro::internal::field_type::RepeatedMessageField::<_puroro_root::google::protobuf::FileDescriptorProto> as FieldType>::deser_from_iter(
                    &mut self.proto_file,
                    &mut self._bitfield,
                    field_data,
                )?,
                3 => <self::_puroro::internal::field_type::SingularHeapMessageField::<_puroro_root::google::protobuf::compiler::Version> as FieldType>::deser_from_iter(
                    &mut self.compiler_version,
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
        <self::_puroro::internal::field_type::RepeatedStringField as FieldType>::ser_to_write(
            &self.file_to_generate,
            &self._bitfield,
            1,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalStringField<0> as FieldType>::ser_to_write(
            &self.parameter,
            &self._bitfield,
            2,
            out,
        )?;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::FileDescriptorProto,
        > as FieldType>::ser_to_write(&self.proto_file, &self._bitfield, 15, out)?;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::compiler::Version,
        > as FieldType>::ser_to_write(&self.compiler_version, &self._bitfield, 3, out)?;

        Ok(())
    }
}

impl ::std::clone::Clone for CodeGeneratorRequest {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::clone::Clone;
        Self {
            file_to_generate: Clone::clone(&self.file_to_generate),
            parameter: Clone::clone(&self.parameter),
            proto_file: Clone::clone(&self.proto_file),
            compiler_version: Clone::clone(&self.compiler_version),

            _bitfield: Clone::clone(&self._bitfield),
        }
    }
}

impl ::std::fmt::Debug for CodeGeneratorRequest {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct("CodeGeneratorRequest")
            .field("file_to_generate", &self.file_to_generate())
            .field("parameter", &self.parameter())
            .field("proto_file", &self.proto_file())
            .field("compiler_version", &self.compiler_version())
            .finish()
    }
}

impl ::std::cmp::PartialEq for CodeGeneratorRequest {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;

        true && self.file_to_generate() == rhs.file_to_generate()
            && self.parameter_opt() == rhs.parameter_opt()
            && self.proto_file() == rhs.proto_file()
            && self.compiler_version_opt() == rhs.compiler_version_opt()
    }
}

impl ::std::ops::Drop for CodeGeneratorRequest {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
    }
}

#[derive(Default)]
pub struct CodeGeneratorResponse {
    // Optional, LengthDelimited(String)
    error: self::_puroro::internal::field_type::OptionalStringField<0>,
    // Optional, Variant(UInt64)
    supported_features: self::_puroro::internal::field_type::OptionalNumericalField<
        u64,
        self::_puroro::tags::UInt64,
        1,
    >,
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.compiler.CodeGeneratorResponse.File")))
    file: self::_puroro::internal::field_type::RepeatedMessageField<
        _puroro_root::google::protobuf::compiler::code_generator_response::File,
    >,

    _bitfield: self::_puroro::bitvec::BitArray<1>,
}

impl CodeGeneratorResponse {
    // Optional, LengthDelimited(String)
    pub fn error(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field(
            &self.error, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn error_opt(&self) -> ::std::option::Option<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field_opt(
            &self.error, &self._bitfield,
        )
    }
    pub fn has_error(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field_opt(
            &self.error, &self._bitfield,
        ).is_some()
    }
    pub fn error_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::mut_field(
            &mut self.error, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_error(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField<0> as NonRepeatedFieldType>::clear(
            &mut self.error,
            &mut self._bitfield,
        )
    }
    // Optional, Variant(UInt64)
    pub fn supported_features(&self) -> u64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u64,
            self::_puroro::tags::UInt64,
            1,
        > as NonRepeatedFieldType>::get_field(
            &self.supported_features,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn supported_features_opt(&self) -> ::std::option::Option<u64> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u64,
            self::_puroro::tags::UInt64,
            1,
        > as NonRepeatedFieldType>::get_field_opt(&self.supported_features, &self._bitfield)
    }
    pub fn has_supported_features(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u64,
            self::_puroro::tags::UInt64,
            1,
        > as NonRepeatedFieldType>::get_field_opt(&self.supported_features, &self._bitfield)
        .is_some()
    }
    pub fn supported_features_mut(&mut self) -> &mut u64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u64,
            self::_puroro::tags::UInt64,
            1,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.supported_features,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_supported_features(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u64,
            self::_puroro::tags::UInt64,
            1,
        > as NonRepeatedFieldType>::clear(&mut self.supported_features, &mut self._bitfield)
    }
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.compiler.CodeGeneratorResponse.File")))
    pub fn file(
        &self,
    ) -> &[_puroro_root::google::protobuf::compiler::code_generator_response::File] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::compiler::code_generator_response::File,
        > as RepeatedFieldType>::get_field(&self.file, &self._bitfield)
    }
    pub fn file_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec<_puroro_root::google::protobuf::compiler::code_generator_response::File>
    {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::compiler::code_generator_response::File,
        > as RepeatedFieldType>::mut_field(&mut self.file, &mut self._bitfield)
    }
    pub fn clear_file(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::compiler::code_generator_response::File,
        > as RepeatedFieldType>::clear(&mut self.file, &mut self._bitfield)
    }
}

impl self::_puroro::Message for CodeGeneratorResponse {
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
                    &mut self.error,
                    &mut self._bitfield,
                    field_data,
                )?,
                2 => <self::_puroro::internal::field_type::OptionalNumericalField::<u64, self::_puroro::tags::UInt64, 1> as FieldType>::deser_from_iter(
                    &mut self.supported_features,
                    &mut self._bitfield,
                    field_data,
                )?,
                15 => <self::_puroro::internal::field_type::RepeatedMessageField::<_puroro_root::google::protobuf::compiler::code_generator_response::File> as FieldType>::deser_from_iter(
                    &mut self.file,
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
            &self.error,
            &self._bitfield,
            1,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u64,
            self::_puroro::tags::UInt64,
            1,
        > as FieldType>::ser_to_write(&self.supported_features, &self._bitfield, 2, out)?;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::compiler::code_generator_response::File,
        > as FieldType>::ser_to_write(&self.file, &self._bitfield, 15, out)?;

        Ok(())
    }
}

impl ::std::clone::Clone for CodeGeneratorResponse {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::clone::Clone;
        Self {
            error: Clone::clone(&self.error),
            supported_features: Clone::clone(&self.supported_features),
            file: Clone::clone(&self.file),

            _bitfield: Clone::clone(&self._bitfield),
        }
    }
}

impl ::std::fmt::Debug for CodeGeneratorResponse {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct("CodeGeneratorResponse")
            .field("error", &self.error())
            .field("supported_features", &self.supported_features())
            .field("file", &self.file())
            .finish()
    }
}

impl ::std::cmp::PartialEq for CodeGeneratorResponse {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;

        true && self.error_opt() == rhs.error_opt()
            && self.supported_features_opt() == rhs.supported_features_opt()
            && self.file() == rhs.file()
    }
}

impl ::std::ops::Drop for CodeGeneratorResponse {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
    }
}
pub mod code_generator_response;
