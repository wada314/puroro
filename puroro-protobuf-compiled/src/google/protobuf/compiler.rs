pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}
pub mod _puroro {
    pub use ::puroro::*;
}
pub mod code_generator_response;
#[derive(::std::default::Default)]
pub struct Version {
    major: self::_puroro::internal::field_type::OptionalNumericalField::<
        i32,
        self::_puroro::tags::Int32,
        0usize,
    >,
    minor: self::_puroro::internal::field_type::OptionalNumericalField::<
        i32,
        self::_puroro::tags::Int32,
        1usize,
    >,
    patch: self::_puroro::internal::field_type::OptionalNumericalField::<
        i32,
        self::_puroro::tags::Int32,
        2usize,
    >,
    suffix: self::_puroro::internal::field_type::OptionalStringField::<3usize>,
    _bitfield: self::_puroro::bitvec::BitArray<1usize>,
}
impl Version {
    pub fn major(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field(
            &self.major,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn major_opt(&self) -> ::std::option::Option::<i32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.major, &self._bitfield)
    }
    pub fn major_mut(&mut self) -> &mut i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.major,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_major(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.major, &self._bitfield)
            .is_some()
    }
    pub fn minor(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            1usize,
        > as NonRepeatedFieldType>::get_field(
            &self.minor,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn minor_opt(&self) -> ::std::option::Option::<i32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.minor, &self._bitfield)
    }
    pub fn minor_mut(&mut self) -> &mut i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            1usize,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.minor,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_minor(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.minor, &self._bitfield)
            .is_some()
    }
    pub fn patch(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            2usize,
        > as NonRepeatedFieldType>::get_field(
            &self.patch,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn patch_opt(&self) -> ::std::option::Option::<i32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            2usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.patch, &self._bitfield)
    }
    pub fn patch_mut(&mut self) -> &mut i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            2usize,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.patch,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_patch(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            2usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.patch, &self._bitfield)
            .is_some()
    }
    pub fn suffix(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            3usize,
        > as NonRepeatedFieldType>::get_field(
            &self.suffix,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn suffix_opt(&self) -> ::std::option::Option::<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            3usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.suffix, &self._bitfield)
    }
    pub fn suffix_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            3usize,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.suffix,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_suffix(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            3usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.suffix, &self._bitfield)
            .is_some()
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
        use self::_puroro::internal::ser::FieldData;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            match number {
                1i32 => {
                    <self::_puroro::internal::field_type::OptionalNumericalField::<
                        i32,
                        self::_puroro::tags::Int32,
                        0usize,
                    > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.major,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                2i32 => {
                    <self::_puroro::internal::field_type::OptionalNumericalField::<
                        i32,
                        self::_puroro::tags::Int32,
                        1usize,
                    > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.minor,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                3i32 => {
                    <self::_puroro::internal::field_type::OptionalNumericalField::<
                        i32,
                        self::_puroro::tags::Int32,
                        2usize,
                    > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.patch,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                4i32 => {
                    <self::_puroro::internal::field_type::OptionalStringField::<
                        3usize,
                    > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.suffix,
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
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            0usize,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.major,
            &self._bitfield,
            1i32,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            1usize,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.minor,
            &self._bitfield,
            2i32,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            2usize,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.patch,
            &self._bitfield,
            3i32,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalStringField::<
            3usize,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.suffix,
            &self._bitfield,
            4i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for Version {
    fn clone(&self) -> Self {
        Self {
            major: <self::_puroro::internal::field_type::OptionalNumericalField::<
                i32,
                self::_puroro::tags::Int32,
                0usize,
            > as ::std::clone::Clone>::clone(&self.major),
            minor: <self::_puroro::internal::field_type::OptionalNumericalField::<
                i32,
                self::_puroro::tags::Int32,
                1usize,
            > as ::std::clone::Clone>::clone(&self.minor),
            patch: <self::_puroro::internal::field_type::OptionalNumericalField::<
                i32,
                self::_puroro::tags::Int32,
                2usize,
            > as ::std::clone::Clone>::clone(&self.patch),
            suffix: <self::_puroro::internal::field_type::OptionalStringField::<
                3usize,
            > as ::std::clone::Clone>::clone(&self.suffix),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
#[derive(::std::default::Default)]
pub struct CodeGeneratorRequest {
    file_to_generate: self::_puroro::internal::field_type::RepeatedStringField,
    parameter: self::_puroro::internal::field_type::OptionalStringField::<0usize>,
    proto_file: self::_puroro::internal::field_type::RepeatedMessageField::<
        self::_puroro_root::google::protobuf::FileDescriptorProto,
    >,
    compiler_version: self::_puroro::internal::field_type::SingularHeapMessageField::<
        self::_puroro_root::google::protobuf::compiler::Version,
    >,
    _bitfield: self::_puroro::bitvec::BitArray<1usize>,
}
impl CodeGeneratorRequest {
    pub fn file_to_generate(&self) -> &[impl ::std::ops::Deref::<Target = str>] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedStringField as RepeatedFieldType>::get_field(
            &self.file_to_generate,
            &self._bitfield,
        )
    }
    pub fn parameter(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            0usize,
        > as NonRepeatedFieldType>::get_field(
            &self.parameter,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn parameter_opt(&self) -> ::std::option::Option::<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.parameter, &self._bitfield)
    }
    pub fn parameter_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            0usize,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.parameter,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_parameter(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.parameter, &self._bitfield)
            .is_some()
    }
    pub fn proto_file(
        &self,
    ) -> &[self::_puroro_root::google::protobuf::FileDescriptorProto] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField::<
            self::_puroro_root::google::protobuf::FileDescriptorProto,
        > as RepeatedFieldType>::get_field(&self.proto_file, &self._bitfield)
    }
    pub fn compiler_version(
        &self,
    ) -> ::std::option::Option::<
        &self::_puroro_root::google::protobuf::compiler::Version,
    > {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField::<
            self::_puroro_root::google::protobuf::compiler::Version,
        > as NonRepeatedFieldType>::get_field(
            &self.compiler_version,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn compiler_version_opt(
        &self,
    ) -> ::std::option::Option::<
        &self::_puroro_root::google::protobuf::compiler::Version,
    > {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField::<
            self::_puroro_root::google::protobuf::compiler::Version,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.compiler_version,
            &self._bitfield,
        )
    }
    pub fn compiler_version_mut(
        &mut self,
    ) -> &mut self::_puroro_root::google::protobuf::compiler::Version {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField::<
            self::_puroro_root::google::protobuf::compiler::Version,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.compiler_version,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_compiler_version(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField::<
            self::_puroro_root::google::protobuf::compiler::Version,
        > as NonRepeatedFieldType>::get_field_opt(
                &self.compiler_version,
                &self._bitfield,
            )
            .is_some()
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
        use self::_puroro::internal::ser::FieldData;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            match number {
                1i32 => {
                    <self::_puroro::internal::field_type::RepeatedStringField as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.file_to_generate,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                2i32 => {
                    <self::_puroro::internal::field_type::OptionalStringField::<
                        0usize,
                    > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.parameter,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                15i32 => {
                    <self::_puroro::internal::field_type::RepeatedMessageField::<
                        self::_puroro_root::google::protobuf::FileDescriptorProto,
                    > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.proto_file,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                3i32 => {
                    <self::_puroro::internal::field_type::SingularHeapMessageField::<
                        self::_puroro_root::google::protobuf::compiler::Version,
                    > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.compiler_version,
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
        <self::_puroro::internal::field_type::RepeatedStringField as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.file_to_generate,
            &self._bitfield,
            1i32,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalStringField::<
            0usize,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.parameter,
            &self._bitfield,
            2i32,
            out,
        )?;
        <self::_puroro::internal::field_type::RepeatedMessageField::<
            self::_puroro_root::google::protobuf::FileDescriptorProto,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.proto_file,
            &self._bitfield,
            15i32,
            out,
        )?;
        <self::_puroro::internal::field_type::SingularHeapMessageField::<
            self::_puroro_root::google::protobuf::compiler::Version,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.compiler_version,
            &self._bitfield,
            3i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for CodeGeneratorRequest {
    fn clone(&self) -> Self {
        Self {
            file_to_generate: <self::_puroro::internal::field_type::RepeatedStringField as ::std::clone::Clone>::clone(
                &self.file_to_generate,
            ),
            parameter: <self::_puroro::internal::field_type::OptionalStringField::<
                0usize,
            > as ::std::clone::Clone>::clone(&self.parameter),
            proto_file: <self::_puroro::internal::field_type::RepeatedMessageField::<
                self::_puroro_root::google::protobuf::FileDescriptorProto,
            > as ::std::clone::Clone>::clone(&self.proto_file),
            compiler_version: <self::_puroro::internal::field_type::SingularHeapMessageField::<
                self::_puroro_root::google::protobuf::compiler::Version,
            > as ::std::clone::Clone>::clone(&self.compiler_version),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
#[derive(::std::default::Default)]
pub struct CodeGeneratorResponse {
    error: self::_puroro::internal::field_type::OptionalStringField::<0usize>,
    supported_features: self::_puroro::internal::field_type::OptionalNumericalField::<
        u64,
        self::_puroro::tags::UInt64,
        1usize,
    >,
    file: self::_puroro::internal::field_type::RepeatedMessageField::<
        self::_puroro_root::google::protobuf::compiler::code_generator_response::File,
    >,
    _bitfield: self::_puroro::bitvec::BitArray<1usize>,
}
impl CodeGeneratorResponse {
    pub fn error(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            0usize,
        > as NonRepeatedFieldType>::get_field(
            &self.error,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn error_opt(&self) -> ::std::option::Option::<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.error, &self._bitfield)
    }
    pub fn error_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            0usize,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.error,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_error(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.error, &self._bitfield)
            .is_some()
    }
    pub fn supported_features(&self) -> u64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            u64,
            self::_puroro::tags::UInt64,
            1usize,
        > as NonRepeatedFieldType>::get_field(
            &self.supported_features,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn supported_features_opt(&self) -> ::std::option::Option::<u64> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            u64,
            self::_puroro::tags::UInt64,
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.supported_features,
            &self._bitfield,
        )
    }
    pub fn supported_features_mut(&mut self) -> &mut u64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            u64,
            self::_puroro::tags::UInt64,
            1usize,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.supported_features,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_supported_features(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            u64,
            self::_puroro::tags::UInt64,
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(
                &self.supported_features,
                &self._bitfield,
            )
            .is_some()
    }
    pub fn file(
        &self,
    ) -> &[self::_puroro_root::google::protobuf::compiler::code_generator_response::File] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField::<
            self::_puroro_root::google::protobuf::compiler::code_generator_response::File,
        > as RepeatedFieldType>::get_field(&self.file, &self._bitfield)
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
        use self::_puroro::internal::ser::FieldData;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            match number {
                1i32 => {
                    <self::_puroro::internal::field_type::OptionalStringField::<
                        0usize,
                    > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.error,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                2i32 => {
                    <self::_puroro::internal::field_type::OptionalNumericalField::<
                        u64,
                        self::_puroro::tags::UInt64,
                        1usize,
                    > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.supported_features,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                15i32 => {
                    <self::_puroro::internal::field_type::RepeatedMessageField::<
                        self::_puroro_root::google::protobuf::compiler::code_generator_response::File,
                    > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.file,
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
        <self::_puroro::internal::field_type::OptionalStringField::<
            0usize,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.error,
            &self._bitfield,
            1i32,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            u64,
            self::_puroro::tags::UInt64,
            1usize,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.supported_features,
            &self._bitfield,
            2i32,
            out,
        )?;
        <self::_puroro::internal::field_type::RepeatedMessageField::<
            self::_puroro_root::google::protobuf::compiler::code_generator_response::File,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.file,
            &self._bitfield,
            15i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for CodeGeneratorResponse {
    fn clone(&self) -> Self {
        Self {
            error: <self::_puroro::internal::field_type::OptionalStringField::<
                0usize,
            > as ::std::clone::Clone>::clone(&self.error),
            supported_features: <self::_puroro::internal::field_type::OptionalNumericalField::<
                u64,
                self::_puroro::tags::UInt64,
                1usize,
            > as ::std::clone::Clone>::clone(&self.supported_features),
            file: <self::_puroro::internal::field_type::RepeatedMessageField::<
                self::_puroro_root::google::protobuf::compiler::code_generator_response::File,
            > as ::std::clone::Clone>::clone(&self.file),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
