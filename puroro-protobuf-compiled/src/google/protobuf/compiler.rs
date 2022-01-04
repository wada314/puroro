// A generated source code by puroro library
// package google.protobuf.compiler

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub use _puroro_simple_impl::CodeGeneratorRequest;
pub use _puroro_simple_impl::CodeGeneratorResponse;
pub use _puroro_simple_impl::Version;
pub mod _puroro_simple_impl {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
    pub struct Version {
        _bitfield:
            ::puroro::bitvec::array::BitArray<::puroro::bitvec::order::Lsb0, [u32; (4 + 31) / 32]>,
        major: ::puroro::internal::Bare<i32>,
        minor: ::puroro::internal::Bare<i32>,
        patch: ::puroro::internal::Bare<i32>,
        suffix: ::puroro::internal::Bare<::std::string::String>,
    }
    impl ::puroro::Message<Version> for Version {}

    impl Version {
        pub fn new() -> Self {
            Self {
                _bitfield: ::std::default::Default::default(),
                major: ::std::default::Default::default(),
                minor: ::std::default::Default::default(),
                patch: ::std::default::Default::default(),
                suffix: ::std::default::Default::default(),
            }
        }
        pub fn major_opt(&self) -> ::std::option::Option<i32> {
            if self._bitfield.get(0).map_or(false, |v| *v) {
                ::std::option::Option::Some(self.major.clone().inner())
            } else {
                ::std::option::Option::None
            }
        }

        pub fn has_major(&self) -> bool {
            Self::major_opt(self).is_some()
        }

        pub fn major(&self) -> i32 {
            self.major_opt()
                .unwrap_or(::std::default::Default::default())
        }
        pub fn minor_opt(&self) -> ::std::option::Option<i32> {
            if self._bitfield.get(1).map_or(false, |v| *v) {
                ::std::option::Option::Some(self.minor.clone().inner())
            } else {
                ::std::option::Option::None
            }
        }

        pub fn has_minor(&self) -> bool {
            Self::minor_opt(self).is_some()
        }

        pub fn minor(&self) -> i32 {
            self.minor_opt()
                .unwrap_or(::std::default::Default::default())
        }
        pub fn patch_opt(&self) -> ::std::option::Option<i32> {
            if self._bitfield.get(2).map_or(false, |v| *v) {
                ::std::option::Option::Some(self.patch.clone().inner())
            } else {
                ::std::option::Option::None
            }
        }

        pub fn has_patch(&self) -> bool {
            Self::patch_opt(self).is_some()
        }

        pub fn patch(&self) -> i32 {
            self.patch_opt()
                .unwrap_or(::std::default::Default::default())
        }
        pub fn suffix_opt(&self) -> ::std::option::Option<&'_ str> {
            if self._bitfield.get(3).map_or(false, |v| *v) {
                ::std::option::Option::Some(&self.suffix)
            } else {
                ::std::option::Option::None
            }
        }

        pub fn has_suffix(&self) -> bool {
            Self::suffix_opt(self).is_some()
        }

        pub fn suffix(&self) -> &'_ str {
            self.suffix_opt()
                .unwrap_or(::std::default::Default::default())
        }
        pub fn clear_major(&mut self) {
            self._bitfield.set(0, false);
        }
        pub fn major_mut(&mut self) -> &'_ mut i32 {
            if !self.has_major() {
                self.major = ::std::default::Default::default();
                self._bitfield.set(0, true);
            }
            &mut self.major
        }
        pub fn clear_minor(&mut self) {
            self._bitfield.set(1, false);
        }
        pub fn minor_mut(&mut self) -> &'_ mut i32 {
            if !self.has_minor() {
                self.minor = ::std::default::Default::default();
                self._bitfield.set(1, true);
            }
            &mut self.minor
        }
        pub fn clear_patch(&mut self) {
            self._bitfield.set(2, false);
        }
        pub fn patch_mut(&mut self) -> &'_ mut i32 {
            if !self.has_patch() {
                self.patch = ::std::default::Default::default();
                self._bitfield.set(2, true);
            }
            &mut self.patch
        }
        pub fn clear_suffix(&mut self) {
            self._bitfield.set(3, false);
        }
        pub fn suffix_mut(&mut self) -> &'_ mut ::std::string::String {
            if !self.has_suffix() {
                self.suffix = ::std::default::Default::default();
                self._bitfield.set(3, true);
            }
            &mut self.suffix
        }
    }

    impl super::_puroro_traits::VersionTrait for Version {
        fn major_opt<'this>(&'this self) -> Option<i32> {
            <self::Version>::major_opt(self)
        }
        fn minor_opt<'this>(&'this self) -> Option<i32> {
            <self::Version>::minor_opt(self)
        }
        fn patch_opt<'this>(&'this self) -> Option<i32> {
            <self::Version>::patch_opt(self)
        }
        fn suffix_opt<'this>(&'this self) -> Option<&'this str> {
            <self::Version>::suffix_opt(self)
        }
    }

    impl ::puroro::MessageRepresentativeImpl for Version {}

    impl ::puroro::internal::de::DeserMessageFromBytesIter for Version {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::internal::types::FieldData<
                &mut ::puroro::internal::de::from_iter::ScopedIter<I>,
            >,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro::internal::impls::simple::de::DeserFieldFromBytesIter;
            match field_number {
                1 => {
                    self._bitfield.set(0, true);
                    DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Int32
                >::deser_field(&mut self.major, data)
                }
                2 => {
                    self._bitfield.set(1, true);
                    DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Int32
                >::deser_field(&mut self.minor, data)
                }
                3 => {
                    self._bitfield.set(2, true);
                    DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Int32
                >::deser_field(&mut self.patch, data)
                }
                4 => {
                    self._bitfield.set(3, true);
                    DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::String
                >::deser_field(&mut self.suffix, data)
                }

                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl ::puroro::internal::se::SerMessageToIoWrite for Version
    where
        Self: super::_puroro_traits::VersionTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::VersionTrait>::major_opt(self),
                1,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::VersionTrait>::minor_opt(self),
                2,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::VersionTrait>::patch_opt(self),
                3,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::VersionTrait>::suffix_opt(self),
                4,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::default::Default for Version {
        fn default() -> Self {
            Self::new()
        }
    }

    impl ::std::fmt::Debug for Version
    where
        Self: super::_puroro_traits::VersionTrait,
    {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.debug_struct("Version")
                .field("major", &self.major_opt())
                .field("minor", &self.minor_opt())
                .field("patch", &self.patch_opt())
                .field("suffix", &self.suffix_opt())
                .finish()
        }
    }

    impl ::std::clone::Clone for Version {
        fn clone(&self) -> Self {
            Self {
                _bitfield: ::std::clone::Clone::clone(&self._bitfield),
                major: ::std::clone::Clone::clone(&self.major),
                minor: ::std::clone::Clone::clone(&self.minor),
                patch: ::std::clone::Clone::clone(&self.patch),
                suffix: ::std::clone::Clone::clone(&self.suffix),
            }
        }
    }

    impl ::std::cmp::PartialEq for Version {
        fn eq(&self, rhs: &Self) -> bool {
            self._bitfield == rhs._bitfield
                && (self._bitfield.get(0).as_deref() != Some(&true) || self.major == rhs.major)
                && (self._bitfield.get(1).as_deref() != Some(&true) || self.minor == rhs.minor)
                && (self._bitfield.get(2).as_deref() != Some(&true) || self.patch == rhs.patch)
                && (self._bitfield.get(3).as_deref() != Some(&true) || self.suffix == rhs.suffix)
                && true
        }
    }
    pub struct CodeGeneratorRequest {
        _bitfield:
            ::puroro::bitvec::array::BitArray<::puroro::bitvec::order::Lsb0, [u32; (1 + 31) / 32]>,
        file_to_generate: ::std::vec::Vec<::std::string::String>,
        parameter: ::puroro::internal::Bare<::std::string::String>,
        proto_file: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::FileDescriptorProto,
        >,
        compiler_version: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::google::protobuf::compiler::_puroro_simple_impl::Version,
            >,
        >,
    }
    impl ::puroro::Message<CodeGeneratorRequest> for CodeGeneratorRequest {}

    impl CodeGeneratorRequest {
        pub fn new() -> Self {
            Self {
                _bitfield: ::std::default::Default::default(),
                file_to_generate: ::std::default::Default::default(),
                parameter: ::std::default::Default::default(),
                proto_file: ::std::default::Default::default(),
                compiler_version: ::std::default::Default::default(),
            }
        }
        pub fn file_to_generate(
            &self,
        ) -> &'_ [impl ::std::ops::Deref<Target = str> + ::std::fmt::Debug] {
            &self.file_to_generate
        }
        pub fn parameter_opt(&self) -> ::std::option::Option<&'_ str> {
            if self._bitfield.get(0).map_or(false, |v| *v) {
                ::std::option::Option::Some(&self.parameter)
            } else {
                ::std::option::Option::None
            }
        }

        pub fn has_parameter(&self) -> bool {
            Self::parameter_opt(self).is_some()
        }

        pub fn parameter(&self) -> &'_ str {
            self.parameter_opt()
                .unwrap_or(::std::default::Default::default())
        }
        pub fn proto_file(
            &self,
        ) -> &'_ [self::_puroro_root::google::protobuf::_puroro_simple_impl::FileDescriptorProto]
        {
            &self.proto_file
        }
        pub fn compiler_version_opt(
            &self,
        ) -> ::std::option::Option<
            &'_ self::_puroro_root::google::protobuf::compiler::_puroro_simple_impl::Version,
        > {
            self.compiler_version.as_deref()
        }

        pub fn has_compiler_version(&self) -> bool {
            Self::compiler_version_opt(self).is_some()
        }

        pub fn compiler_version(
            &self,
        ) -> ::std::option::Option<
            &'_ self::_puroro_root::google::protobuf::compiler::_puroro_simple_impl::Version,
        > {
            self.compiler_version_opt()
        }
        pub fn file_to_generate_mut(&mut self) -> &'_ mut ::std::vec::Vec<::std::string::String> {
            &mut self.file_to_generate
        }
        pub fn clear_parameter(&mut self) {
            self._bitfield.set(0, false);
        }
        pub fn parameter_mut(&mut self) -> &'_ mut ::std::string::String {
            if !self.has_parameter() {
                self.parameter = ::std::default::Default::default();
                self._bitfield.set(0, true);
            }
            &mut self.parameter
        }
        pub fn proto_file_mut(
            &mut self,
        ) -> &'_ mut ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::FileDescriptorProto,
        > {
            &mut self.proto_file
        }
        pub fn clear_compiler_version(&mut self) {
            self.compiler_version = ::std::default::Default::default();
        }
        pub fn compiler_version_mut(
            &mut self,
        ) -> &'_ mut self::_puroro_root::google::protobuf::compiler::_puroro_simple_impl::Version
        {
            if !self.has_compiler_version() {
                self.compiler_version = ::std::default::Default::default();
            }
            self.compiler_version
                .get_or_insert_with(::std::default::Default::default)
        }
    }

    impl super::_puroro_traits::CodeGeneratorRequestTrait for CodeGeneratorRequest {
        type FileToGenerateRepeatedType<'this> = ::puroro::AsRefRepeatedField<
            'this,
            ::std::vec::Vec<::std::string::String>,
            ::std::string::String,
            str,
        >;

        fn file_to_generate<'this>(&'this self) -> Self::FileToGenerateRepeatedType<'this> {
            ::puroro::AsRefRepeatedField::new(&self.file_to_generate)
        }
        fn parameter_opt<'this>(&'this self) -> Option<&'this str> {
            <self::CodeGeneratorRequest>::parameter_opt(self)
        }
        type ProtoFileMessageType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::FileDescriptorProto;
        type ProtoFileRepeatedType<'this> =
    &'this [self::_puroro_root::google::protobuf::_puroro_simple_impl::FileDescriptorProto];

        fn proto_file<'this>(&'this self) -> Self::ProtoFileRepeatedType<'this> {
            &self.proto_file
        }
        type CompilerVersionMessageType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::google::protobuf::compiler::_puroro_simple_impl::Version;
        fn compiler_version_opt<'this>(
            &'this self,
        ) -> Option<Self::CompilerVersionMessageType<'this>> {
            <self::CodeGeneratorRequest>::compiler_version_opt(self)
        }
    }

    impl ::puroro::MessageRepresentativeImpl for CodeGeneratorRequest {}

    impl ::puroro::internal::de::DeserMessageFromBytesIter for CodeGeneratorRequest {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::internal::types::FieldData<
                &mut ::puroro::internal::de::from_iter::ScopedIter<I>,
            >,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro::internal::impls::simple::de::DeserFieldFromBytesIter;
            match field_number {
            1 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::String
                >::deser_field(&mut self.file_to_generate, data)
            }
            2 => {
                self._bitfield.set(0, true);
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::String
                >::deser_field(&mut self.parameter, data)
            }
            15 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_simple_impl::FileDescriptorProto>
                >::deser_field(&mut self.proto_file, data)
            }
            3 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Message<::std::boxed::Box<self::_puroro_root::google::protobuf::compiler::_puroro_simple_impl::Version>>
                >::deser_field(&mut self.compiler_version, data)
            }

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::internal::se::SerMessageToIoWrite for CodeGeneratorRequest
    where
        Self: super::_puroro_traits::CodeGeneratorRequestTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::CodeGeneratorRequestTrait>::file_to_generate(self),
                1,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::CodeGeneratorRequestTrait>::parameter_opt(self),
                2,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Message<<Self as super::_puroro_traits::CodeGeneratorRequestTrait>::ProtoFileMessageType<'_>>
        >::ser_field(
            <Self as super::_puroro_traits::CodeGeneratorRequestTrait>::proto_file(self),
            15,
            out
        )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
            ::puroro::tags::Optional, ::puroro::tags::Message<<Self as super::_puroro_traits::CodeGeneratorRequestTrait>::CompilerVersionMessageType<'_>>
        >::ser_field(
            <Self as super::_puroro_traits::CodeGeneratorRequestTrait>::compiler_version_opt(self),
            3,
            out
        )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::default::Default for CodeGeneratorRequest {
        fn default() -> Self {
            Self::new()
        }
    }

    impl ::std::fmt::Debug for CodeGeneratorRequest
    where
        Self: super::_puroro_traits::CodeGeneratorRequestTrait,
    {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.debug_struct("CodeGeneratorRequest")
                .field("file_to_generate", &self.file_to_generate())
                .field("parameter", &self.parameter_opt())
                .field("proto_file", &self.proto_file())
                .field("compiler_version", &self.compiler_version())
                .finish()
        }
    }

    impl ::std::clone::Clone for CodeGeneratorRequest {
        fn clone(&self) -> Self {
            Self {
                _bitfield: ::std::clone::Clone::clone(&self._bitfield),
                file_to_generate: ::std::clone::Clone::clone(&self.file_to_generate),
                parameter: ::std::clone::Clone::clone(&self.parameter),
                proto_file: ::std::clone::Clone::clone(&self.proto_file),
                compiler_version: ::std::clone::Clone::clone(&self.compiler_version),
            }
        }
    }

    impl ::std::cmp::PartialEq for CodeGeneratorRequest {
        fn eq(&self, rhs: &Self) -> bool {
            self._bitfield == rhs._bitfield
                && self.file_to_generate == rhs.file_to_generate
                && (self._bitfield.get(0).as_deref() != Some(&true)
                    || self.parameter == rhs.parameter)
                && self.proto_file == rhs.proto_file
                && self.compiler_version == rhs.compiler_version
                && true
        }
    }
    pub struct CodeGeneratorResponse {
    _bitfield: ::puroro::bitvec::array::BitArray<
        ::puroro::bitvec::order::Lsb0,
        [u32; (2 + 31) / 32],
    >,
    error: ::puroro::internal::Bare<::std::string::String>,
    supported_features: ::puroro::internal::Bare<u64>,
    file: ::std::vec::Vec<self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_simple_impl::File>,
}
    impl ::puroro::Message<CodeGeneratorResponse> for CodeGeneratorResponse {}

    impl CodeGeneratorResponse {
        pub fn new() -> Self {
            Self {
                _bitfield: ::std::default::Default::default(),
                error: ::std::default::Default::default(),
                supported_features: ::std::default::Default::default(),
                file: ::std::default::Default::default(),
            }
        }
        pub fn error_opt(&self) -> ::std::option::Option<&'_ str> {
            if self._bitfield.get(0).map_or(false, |v| *v) {
                ::std::option::Option::Some(&self.error)
            } else {
                ::std::option::Option::None
            }
        }

        pub fn has_error(&self) -> bool {
            Self::error_opt(self).is_some()
        }

        pub fn error(&self) -> &'_ str {
            self.error_opt()
                .unwrap_or(::std::default::Default::default())
        }
        pub fn supported_features_opt(&self) -> ::std::option::Option<u64> {
            if self._bitfield.get(1).map_or(false, |v| *v) {
                ::std::option::Option::Some(self.supported_features.clone().inner())
            } else {
                ::std::option::Option::None
            }
        }

        pub fn has_supported_features(&self) -> bool {
            Self::supported_features_opt(self).is_some()
        }

        pub fn supported_features(&self) -> u64 {
            self.supported_features_opt()
                .unwrap_or(::std::default::Default::default())
        }
        pub fn file(&self) -> &'_[self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_simple_impl::File]{
            &self.file
        }
        pub fn clear_error(&mut self) {
            self._bitfield.set(0, false);
        }
        pub fn error_mut(&mut self) -> &'_ mut ::std::string::String {
            if !self.has_error() {
                self.error = ::std::default::Default::default();
                self._bitfield.set(0, true);
            }
            &mut self.error
        }
        pub fn clear_supported_features(&mut self) {
            self._bitfield.set(1, false);
        }
        pub fn supported_features_mut(&mut self) -> &'_ mut u64 {
            if !self.has_supported_features() {
                self.supported_features = ::std::default::Default::default();
                self._bitfield.set(1, true);
            }
            &mut self.supported_features
        }
        pub fn file_mut(&mut self) -> &'_ mut ::std::vec::Vec<self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_simple_impl::File>{
            &mut self.file
        }
    }

    impl super::_puroro_traits::CodeGeneratorResponseTrait for CodeGeneratorResponse {
        fn error_opt<'this>(&'this self) -> Option<&'this str> {
            <self::CodeGeneratorResponse>::error_opt(self)
        }
        fn supported_features_opt<'this>(&'this self) -> Option<u64> {
            <self::CodeGeneratorResponse>::supported_features_opt(self)
        }
        type FileMessageType<'this> where Self: 'this = &'this self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_simple_impl::File;
        type FileRepeatedType<'this> =
    &'this [self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_simple_impl::File];

        fn file<'this>(&'this self) -> Self::FileRepeatedType<'this> {
            &self.file
        }
    }

    impl ::puroro::MessageRepresentativeImpl for CodeGeneratorResponse {}

    impl ::puroro::internal::de::DeserMessageFromBytesIter for CodeGeneratorResponse {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::internal::types::FieldData<
                &mut ::puroro::internal::de::from_iter::ScopedIter<I>,
            >,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro::internal::impls::simple::de::DeserFieldFromBytesIter;
            match field_number {
            1 => {
                self._bitfield.set(0, true);
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::String
                >::deser_field(&mut self.error, data)
            }
            2 => {
                self._bitfield.set(1, true);
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::UInt64
                >::deser_field(&mut self.supported_features, data)
            }
            15 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_simple_impl::File>
                >::deser_field(&mut self.file, data)
            }

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::internal::se::SerMessageToIoWrite for CodeGeneratorResponse
    where
        Self: super::_puroro_traits::CodeGeneratorResponseTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::CodeGeneratorResponseTrait>::error_opt(self),
                1,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::UInt64,
            >::ser_field(
                <Self as super::_puroro_traits::CodeGeneratorResponseTrait>::supported_features_opt(
                    self,
                ),
                2,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    <Self as super::_puroro_traits::CodeGeneratorResponseTrait>::FileMessageType<
                        '_,
                    >,
                >,
            >::ser_field(
                <Self as super::_puroro_traits::CodeGeneratorResponseTrait>::file(self),
                15,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::default::Default for CodeGeneratorResponse {
        fn default() -> Self {
            Self::new()
        }
    }

    impl ::std::fmt::Debug for CodeGeneratorResponse
    where
        Self: super::_puroro_traits::CodeGeneratorResponseTrait,
    {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.debug_struct("CodeGeneratorResponse")
                .field("error", &self.error_opt())
                .field("supported_features", &self.supported_features_opt())
                .field("file", &self.file())
                .finish()
        }
    }

    impl ::std::clone::Clone for CodeGeneratorResponse {
        fn clone(&self) -> Self {
            Self {
                _bitfield: ::std::clone::Clone::clone(&self._bitfield),
                error: ::std::clone::Clone::clone(&self.error),
                supported_features: ::std::clone::Clone::clone(&self.supported_features),
                file: ::std::clone::Clone::clone(&self.file),
            }
        }
    }

    impl ::std::cmp::PartialEq for CodeGeneratorResponse {
        fn eq(&self, rhs: &Self) -> bool {
            self._bitfield == rhs._bitfield
                && (self._bitfield.get(0).as_deref() != Some(&true) || self.error == rhs.error)
                && (self._bitfield.get(1).as_deref() != Some(&true)
                    || self.supported_features == rhs.supported_features)
                && self.file == rhs.file
                && true
        }
    }
}

pub use _puroro_impls::*;
pub mod _puroro_impls {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
    use super::_puroro_traits::*;

    pub struct VersionSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
    {
        pub major: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Version> for VersionSingleField1<ScalarType> where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone
    {
    }

    impl<ScalarType> super::_puroro_traits::VersionTrait for VersionSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
    {
        fn major_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.major,
            )))
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for VersionSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        Self: super::_puroro_traits::VersionTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::VersionTrait>::major_opt(self),
                1,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for VersionSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
    {
        fn from(value: ScalarType) -> Self {
            Self { major: value }
        }
    }

    impl<ScalarType> ::std::clone::Clone for VersionSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                major: ::std::clone::Clone::clone(&self.major),
            }
        }
    }

    pub struct VersionSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
    {
        pub minor: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Version> for VersionSingleField2<ScalarType> where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone
    {
    }

    impl<ScalarType> super::_puroro_traits::VersionTrait for VersionSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
    {
        fn minor_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.minor,
            )))
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for VersionSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        Self: super::_puroro_traits::VersionTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::VersionTrait>::minor_opt(self),
                2,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for VersionSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
    {
        fn from(value: ScalarType) -> Self {
            Self { minor: value }
        }
    }

    impl<ScalarType> ::std::clone::Clone for VersionSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                minor: ::std::clone::Clone::clone(&self.minor),
            }
        }
    }

    pub struct VersionSingleField3<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
    {
        pub patch: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Version> for VersionSingleField3<ScalarType> where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone
    {
    }

    impl<ScalarType> super::_puroro_traits::VersionTrait for VersionSingleField3<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
    {
        fn patch_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.patch,
            )))
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for VersionSingleField3<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        Self: super::_puroro_traits::VersionTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::VersionTrait>::patch_opt(self),
                3,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for VersionSingleField3<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
    {
        fn from(value: ScalarType) -> Self {
            Self { patch: value }
        }
    }

    impl<ScalarType> ::std::clone::Clone for VersionSingleField3<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                patch: ::std::clone::Clone::clone(&self.patch),
            }
        }
    }

    pub struct VersionSingleField4<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>,
    {
        pub suffix: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Version> for VersionSingleField4<ScalarType> where
        ScalarType: ::std::convert::AsRef<str>
    {
    }

    impl<ScalarType> super::_puroro_traits::VersionTrait for VersionSingleField4<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>,
    {
        fn suffix_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::Some(self.suffix.as_ref())
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for VersionSingleField4<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>,
        Self: super::_puroro_traits::VersionTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::VersionTrait>::suffix_opt(self),
                4,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for VersionSingleField4<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>,
    {
        fn from(value: ScalarType) -> Self {
            Self { suffix: value }
        }
    }

    impl<ScalarType> ::std::clone::Clone for VersionSingleField4<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                suffix: ::std::clone::Clone::clone(&self.suffix),
            }
        }
    }
    pub struct VersionBumpalo<'bump> {
        _bump: &'bump ::puroro::bumpalo::Bump,
        _bitfield:
            ::puroro::bitvec::array::BitArray<::puroro::bitvec::order::Lsb0, [u32; (4 + 31) / 32]>,
        major: ::puroro::internal::Bare<i32>,
        minor: ::puroro::internal::Bare<i32>,
        patch: ::puroro::internal::Bare<i32>,
        suffix: ::puroro::internal::Bare<::puroro::internal::NoAllocBumpString>,
    }

    pub type VersionBumpaloOwned = ::puroro::BumpaloOwned<VersionBumpalo<'static>>;
    impl<'bump> VersionBumpalo<'bump> {
        pub fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
            #[allow(unused)]
            let bump_ref: &::puroro::bumpalo::Bump =
                unsafe { ::std::mem::transmute(::std::ops::Deref::deref(&bump)) };

            Self {
                _bump: bump,
                _bitfield: ::std::default::Default::default(),
                major: ::std::default::Default::default(),
                minor: ::std::default::Default::default(),
                patch: ::std::default::Default::default(),
                suffix: ::std::default::Default::default(),
            }
        }
        pub fn major_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            if self._bitfield.get(0).map_or(false, |v| *v) {
                ::std::option::Option::Some(self.major.inner())
            } else {
                ::std::option::Option::None
            }
        }
        pub fn major<'this>(&'this self) -> i32 {
            match self.major_opt() {
                ::std::option::Option::Some(x) => x,
                _ => ::std::default::Default::default(),
            }
        }

        pub fn has_major(&self) -> bool {
            self.major_opt().is_some()
        }
        pub fn minor_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            if self._bitfield.get(1).map_or(false, |v| *v) {
                ::std::option::Option::Some(self.minor.inner())
            } else {
                ::std::option::Option::None
            }
        }
        pub fn minor<'this>(&'this self) -> i32 {
            match self.minor_opt() {
                ::std::option::Option::Some(x) => x,
                _ => ::std::default::Default::default(),
            }
        }

        pub fn has_minor(&self) -> bool {
            self.minor_opt().is_some()
        }
        pub fn patch_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            if self._bitfield.get(2).map_or(false, |v| *v) {
                ::std::option::Option::Some(self.patch.inner())
            } else {
                ::std::option::Option::None
            }
        }
        pub fn patch<'this>(&'this self) -> i32 {
            match self.patch_opt() {
                ::std::option::Option::Some(x) => x,
                _ => ::std::default::Default::default(),
            }
        }

        pub fn has_patch(&self) -> bool {
            self.patch_opt().is_some()
        }
        pub fn suffix_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            if self._bitfield.get(3).map_or(false, |v| *v) {
                ::std::option::Option::Some(&self.suffix)
            } else {
                ::std::option::Option::None
            }
        }
        pub fn suffix<'this>(&'this self) -> &'this str {
            match self.suffix_opt() {
                ::std::option::Option::Some(x) => x,
                _ => ::std::default::Default::default(),
            }
        }

        pub fn has_suffix(&self) -> bool {
            self.suffix_opt().is_some()
        }
        pub fn clear_major(&mut self) {
            self._bitfield.set(0, false);
        }
        pub fn major_mut<'this>(&'this mut self) -> &'this mut i32 {
            if !self.has_major() {
                self.major = ::std::default::Default::default();
                self._bitfield.set(0, true);
            }
            &mut self.major
        }
        pub fn clear_minor(&mut self) {
            self._bitfield.set(1, false);
        }
        pub fn minor_mut<'this>(&'this mut self) -> &'this mut i32 {
            if !self.has_minor() {
                self.minor = ::std::default::Default::default();
                self._bitfield.set(1, true);
            }
            &mut self.minor
        }
        pub fn clear_patch(&mut self) {
            self._bitfield.set(2, false);
        }
        pub fn patch_mut<'this>(&'this mut self) -> &'this mut i32 {
            if !self.has_patch() {
                self.patch = ::std::default::Default::default();
                self._bitfield.set(2, true);
            }
            &mut self.patch
        }
        pub fn clear_suffix(&mut self) {
            self._bitfield.set(3, false);
        }
        pub fn suffix_mut<'this>(
            &'this mut self,
        ) -> impl 'this + ::std::ops::DerefMut<Target = ::puroro::bumpalo::collections::String<'bump>>
        {
            if !self.has_suffix() {
                self.suffix = ::std::default::Default::default();
                self._bitfield.set(3, true);
            }
            unsafe { self.suffix.as_mut_string_in(self._bump) }
        }
    }
    impl<'bump> ::puroro::Message<super::_puroro_simple_impl::Version> for VersionBumpalo<'bump> {}

    impl<'bump> ::puroro::BumpaloMessage<'bump> for VersionBumpalo<'bump> {
        fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
            Self::new_in(bump)
        }
    }

    impl<'bump> ::puroro::internal::BumpDefault<'bump> for VersionBumpalo<'bump> {
        fn default_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
            Self::new_in(bump)
        }
    }

    impl<'bump> super::_puroro_traits::VersionTrait for VersionBumpalo<'bump> {
        fn major_opt<'this>(&'this self) -> Option<i32> {
            <Self>::major_opt(self)
        }
        fn minor_opt<'this>(&'this self) -> Option<i32> {
            <Self>::minor_opt(self)
        }
        fn patch_opt<'this>(&'this self) -> Option<i32> {
            <Self>::patch_opt(self)
        }
        fn suffix_opt<'this>(&'this self) -> Option<&'this str> {
            <Self>::suffix_opt(self)
        }
    }

    impl<'bump> ::puroro::internal::de::DeserMessageFromBytesIter for VersionBumpalo<'bump> {
        fn deser_field<'this, I>(
            &'this mut self,
            field_number: i32,
            data: ::puroro::internal::types::FieldData<
                &mut ::puroro::internal::de::from_iter::ScopedIter<I>,
            >,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro::internal::impls::bumpalo::de::DeserFieldFromBytesIter;
            match field_number {
                1 => {
                    self._bitfield.set(0, true);
                    DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Int32
                >::deser_field(&mut self.major, data, self._bump)
                }
                2 => {
                    self._bitfield.set(1, true);
                    DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Int32
                >::deser_field(&mut self.minor, data, self._bump)
                }
                3 => {
                    self._bitfield.set(2, true);
                    DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Int32
                >::deser_field(&mut self.patch, data, self._bump)
                }
                4 => {
                    self._bitfield.set(3, true);
                    DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::String
                >::deser_field(&mut self.suffix, data, self._bump)
                }

                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl<'bump> ::puroro::internal::se::SerMessageToIoWrite for VersionBumpalo<'bump>
    where
        Self: super::_puroro_traits::VersionTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::VersionTrait>::major_opt(self),
                1,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::VersionTrait>::minor_opt(self),
                2,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::VersionTrait>::patch_opt(self),
                3,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::VersionTrait>::suffix_opt(self),
                4,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }
    pub struct VersionBuilder<T>(T);

    impl<T> VersionBuilder<T>
    where
        T: VersionTrait,
    {
        pub fn append_major<ScalarType>(
            self,
            value: ScalarType,
        ) -> VersionBuilder<(T, VersionSingleField1<ScalarType>)>
        where
            ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        {
            VersionBuilder((self.0, VersionSingleField1 { major: value }))
        }

        pub fn append_minor<ScalarType>(
            self,
            value: ScalarType,
        ) -> VersionBuilder<(T, VersionSingleField2<ScalarType>)>
        where
            ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        {
            VersionBuilder((self.0, VersionSingleField2 { minor: value }))
        }

        pub fn append_patch<ScalarType>(
            self,
            value: ScalarType,
        ) -> VersionBuilder<(T, VersionSingleField3<ScalarType>)>
        where
            ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        {
            VersionBuilder((self.0, VersionSingleField3 { patch: value }))
        }

        pub fn append_suffix<ScalarType>(
            self,
            value: ScalarType,
        ) -> VersionBuilder<(T, VersionSingleField4<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<str>,
        {
            VersionBuilder((self.0, VersionSingleField4 { suffix: value }))
        }

        pub fn build(self) -> T {
            self.0
        }
    }

    impl VersionBuilder<()> {
        pub fn new() -> Self {
            Self(())
        }
    }

    pub struct CodeGeneratorRequestSingleField1<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::AsRef<str>,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub file_to_generate: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::CodeGeneratorRequest>
        for CodeGeneratorRequestSingleField1<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::AsRef<str>,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::CodeGeneratorRequestTrait
        for CodeGeneratorRequestSingleField1<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::AsRef<str>,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type FileToGenerateRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::AsRefRepeatedField<'this, RepeatedType, ScalarType, str>;

        fn file_to_generate<'this>(&'this self) -> Self::FileToGenerateRepeatedType<'this> {
            ::puroro::AsRefRepeatedField::new(&self.file_to_generate)
        }
        type ProtoFileMessageType<'this>
        where
            Self: 'this,
        = ();
        type ProtoFileRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::ProtoFileMessageType<'this>>;
        fn proto_file<'this>(&'this self) -> Self::ProtoFileRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type CompilerVersionMessageType<'this>
        where
            Self: 'this,
        = ();
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::se::SerMessageToIoWrite
        for CodeGeneratorRequestSingleField1<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::AsRef<str>,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        Self: super::_puroro_traits::CodeGeneratorRequestTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::CodeGeneratorRequestTrait>::file_to_generate(self),
                1,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for CodeGeneratorRequestSingleField1<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::AsRef<str>,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                file_to_generate: value,
            }
        }
    }

    impl<ScalarType, RepeatedType> ::std::clone::Clone
        for CodeGeneratorRequestSingleField1<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::AsRef<str>,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        RepeatedType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                file_to_generate: ::std::clone::Clone::clone(&self.file_to_generate),
            }
        }
    }

    pub struct CodeGeneratorRequestSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>,
    {
        pub parameter: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::CodeGeneratorRequest>
        for CodeGeneratorRequestSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>,
    {
    }

    impl<ScalarType> super::_puroro_traits::CodeGeneratorRequestTrait
        for CodeGeneratorRequestSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>,
    {
        type FileToGenerateRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn file_to_generate<'this>(&'this self) -> Self::FileToGenerateRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn parameter_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::Some(self.parameter.as_ref())
        }
        type ProtoFileMessageType<'this>
        where
            Self: 'this,
        = ();
        type ProtoFileRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::ProtoFileMessageType<'this>>;
        fn proto_file<'this>(&'this self) -> Self::ProtoFileRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type CompilerVersionMessageType<'this>
        where
            Self: 'this,
        = ();
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite
        for CodeGeneratorRequestSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>,
        Self: super::_puroro_traits::CodeGeneratorRequestTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::CodeGeneratorRequestTrait>::parameter_opt(self),
                2,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for CodeGeneratorRequestSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>,
    {
        fn from(value: ScalarType) -> Self {
            Self { parameter: value }
        }
    }

    impl<ScalarType> ::std::clone::Clone for CodeGeneratorRequestSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                parameter: ::std::clone::Clone::clone(&self.parameter),
            }
        }
    }

    pub struct CodeGeneratorRequestSingleField15<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::FileDescriptorProtoTrait,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub proto_file: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::CodeGeneratorRequest>
        for CodeGeneratorRequestSingleField15<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::FileDescriptorProtoTrait,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::CodeGeneratorRequestTrait
        for CodeGeneratorRequestSingleField15<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::FileDescriptorProtoTrait,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type FileToGenerateRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn file_to_generate<'this>(&'this self) -> Self::FileToGenerateRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type ProtoFileMessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;

        type ProtoFileRepeatedType<'this>
        where
            Self: 'this,
        = &'this RepeatedType;

        fn proto_file<'this>(&'this self) -> Self::ProtoFileRepeatedType<'this> {
            &self.proto_file
        }
        type CompilerVersionMessageType<'this>
        where
            Self: 'this,
        = ();
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::se::SerMessageToIoWrite
        for CodeGeneratorRequestSingleField15<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::FileDescriptorProtoTrait,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        Self: super::_puroro_traits::CodeGeneratorRequestTrait,
        for<'a> <Self as super::_puroro_traits::CodeGeneratorRequestTrait>::ProtoFileMessageType<'a>:
            ::puroro::internal::se::SerMessageToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Message<<Self as super::_puroro_traits::CodeGeneratorRequestTrait>::ProtoFileMessageType<'_>>
        >::ser_field(
            <Self as super::_puroro_traits::CodeGeneratorRequestTrait>::proto_file(self),
            15,
            out
        )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for CodeGeneratorRequestSingleField15<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::FileDescriptorProtoTrait,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self { proto_file: value }
        }
    }

    impl<ScalarType, RepeatedType> ::std::clone::Clone
        for CodeGeneratorRequestSingleField15<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::FileDescriptorProtoTrait,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        RepeatedType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                proto_file: ::std::clone::Clone::clone(&self.proto_file),
            }
        }
    }

    pub struct CodeGeneratorRequestSingleField3<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::compiler::_puroro_traits::VersionTrait,
    {
        pub compiler_version: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::CodeGeneratorRequest>
        for CodeGeneratorRequestSingleField3<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::compiler::_puroro_traits::VersionTrait,
    {
    }

    impl<ScalarType> super::_puroro_traits::CodeGeneratorRequestTrait
        for CodeGeneratorRequestSingleField3<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::compiler::_puroro_traits::VersionTrait,
    {
        type FileToGenerateRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn file_to_generate<'this>(&'this self) -> Self::FileToGenerateRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type ProtoFileMessageType<'this>
        where
            Self: 'this,
        = ();
        type ProtoFileRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::ProtoFileMessageType<'this>>;
        fn proto_file<'this>(&'this self) -> Self::ProtoFileRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type CompilerVersionMessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;

        fn compiler_version_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::CompilerVersionMessageType<'this>> {
            ::std::option::Option::Some(&self.compiler_version)
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite
        for CodeGeneratorRequestSingleField3<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::compiler::_puroro_traits::VersionTrait,
        Self: super::_puroro_traits::CodeGeneratorRequestTrait,
        for<'a> <Self as super::_puroro_traits::CodeGeneratorRequestTrait>::CompilerVersionMessageType<'a>:
            ::puroro::internal::se::SerMessageToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
            ::puroro::tags::Optional, ::puroro::tags::Message<<Self as super::_puroro_traits::CodeGeneratorRequestTrait>::CompilerVersionMessageType<'_>>
        >::ser_field(
            <Self as super::_puroro_traits::CodeGeneratorRequestTrait>::compiler_version_opt(self),
            3,
            out
        )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for CodeGeneratorRequestSingleField3<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::compiler::_puroro_traits::VersionTrait,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                compiler_version: value,
            }
        }
    }

    impl<ScalarType> ::std::clone::Clone for CodeGeneratorRequestSingleField3<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::compiler::_puroro_traits::VersionTrait,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                compiler_version: ::std::clone::Clone::clone(&self.compiler_version),
            }
        }
    }
    pub struct CodeGeneratorRequestBumpalo<'bump> {
        _bump: &'bump ::puroro::bumpalo::Bump,
        _bitfield:
            ::puroro::bitvec::array::BitArray<::puroro::bitvec::order::Lsb0, [u32; (1 + 31) / 32]>,
        file_to_generate: ::puroro::internal::NoAllocBumpVec<::puroro::internal::NoAllocBumpString>,
        parameter: ::puroro::internal::Bare<::puroro::internal::NoAllocBumpString>,
        proto_file: ::puroro::internal::NoAllocBumpVec<
            self::_puroro_root::google::protobuf::_puroro_impls::FileDescriptorProtoBumpalo<'bump>,
        >,
        compiler_version: ::std::option::Option<
            ::puroro::internal::NoAllocBumpBox<
                self::_puroro_root::google::protobuf::compiler::_puroro_impls::VersionBumpalo<
                    'bump,
                >,
            >,
        >,
    }

    pub type CodeGeneratorRequestBumpaloOwned =
        ::puroro::BumpaloOwned<CodeGeneratorRequestBumpalo<'static>>;
    impl<'bump> CodeGeneratorRequestBumpalo<'bump> {
        pub fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
            #[allow(unused)]
            let bump_ref: &::puroro::bumpalo::Bump =
                unsafe { ::std::mem::transmute(::std::ops::Deref::deref(&bump)) };

            Self {
                _bump: bump,
                _bitfield: ::std::default::Default::default(),
                file_to_generate: ::std::default::Default::default(),
                parameter: ::std::default::Default::default(),
                proto_file: ::std::default::Default::default(),
                compiler_version: ::std::default::Default::default(),
            }
        }
        pub fn file_to_generate<'this>(
            &'this self,
        ) -> &'this [impl ::std::ops::Deref<Target = str>] {
            &self.file_to_generate
        }
        pub fn parameter_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            if self._bitfield.get(0).map_or(false, |v| *v) {
                ::std::option::Option::Some(&self.parameter)
            } else {
                ::std::option::Option::None
            }
        }
        pub fn parameter<'this>(&'this self) -> &'this str {
            match self.parameter_opt() {
                ::std::option::Option::Some(x) => x,
                _ => ::std::default::Default::default(),
            }
        }

        pub fn has_parameter(&self) -> bool {
            self.parameter_opt().is_some()
        }
        pub fn proto_file<'this>(&'this self) -> &'this[self::_puroro_root::google::protobuf::_puroro_impls::FileDescriptorProtoBumpalo<'this>]{
            unsafe { self.proto_file.cast_item_unchecked() }
        }
        pub fn compiler_version_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<
            &'this self::_puroro_root::google::protobuf::compiler::_puroro_impls::VersionBumpalo<
                'this,
            >,
        > {
            self.compiler_version
                .as_ref()
                .map(|x| unsafe { ::std::mem::transmute(::std::ops::Deref::deref(x)) })
        }
        pub fn compiler_version<'this>(
            &'this self,
        ) -> ::std::option::Option<
            &'this self::_puroro_root::google::protobuf::compiler::_puroro_impls::VersionBumpalo<
                'this,
            >,
        > {
            self.compiler_version_opt()
        }

        pub fn has_compiler_version(&self) -> bool {
            self.compiler_version_opt().is_some()
        }
        pub fn file_to_generate_mut<'this>(
            &'this mut self,
        ) -> ::puroro::internal::AddBumpVecView<'bump, 'this, ::puroro::internal::NoAllocBumpString>
        {
            unsafe { self.file_to_generate.as_add_bump_vec_view_in(self._bump) }
        }
        pub fn clear_parameter(&mut self) {
            self._bitfield.set(0, false);
        }
        pub fn parameter_mut<'this>(
            &'this mut self,
        ) -> impl 'this + ::std::ops::DerefMut<Target = ::puroro::bumpalo::collections::String<'bump>>
        {
            if !self.has_parameter() {
                self.parameter = ::std::default::Default::default();
                self._bitfield.set(0, true);
            }
            unsafe { self.parameter.as_mut_string_in(self._bump) }
        }
        pub fn proto_file_mut<'this>(
            &'this mut self,
        ) -> ::puroro::internal::RefMutBumpVec<
            'bump,
            'this,
            self::_puroro_root::google::protobuf::_puroro_impls::FileDescriptorProtoBumpalo<'bump>,
        > {
            unsafe { self.proto_file.as_mut_vec_in(self._bump) }
        }
        pub fn clear_compiler_version(&mut self) {
            self.compiler_version = ::std::default::Default::default();
        }
        pub fn compiler_version_mut<'this>(&'this mut self) -> &'this mut self::_puroro_root::google::protobuf::compiler::_puroro_impls::VersionBumpalo<'bump>{
            if !self.has_compiler_version() {
                self.compiler_version = ::std::default::Default::default();
            }
            let bump = self._bump;
            self.compiler_version.get_or_insert_with(|| {
                ::puroro::internal::NoAllocBumpBox::new_in(
                    ::puroro::internal::BumpDefault::default_in(bump),
                    bump,
                )
            })
        }
    }
    impl<'bump> ::puroro::Message<super::_puroro_simple_impl::CodeGeneratorRequest>
        for CodeGeneratorRequestBumpalo<'bump>
    {
    }

    impl<'bump> ::puroro::BumpaloMessage<'bump> for CodeGeneratorRequestBumpalo<'bump> {
        fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
            Self::new_in(bump)
        }
    }

    impl<'bump> ::puroro::internal::BumpDefault<'bump> for CodeGeneratorRequestBumpalo<'bump> {
        fn default_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
            Self::new_in(bump)
        }
    }

    impl<'bump> super::_puroro_traits::CodeGeneratorRequestTrait
        for CodeGeneratorRequestBumpalo<'bump>
    {
        type FileToGenerateRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::AsRefRepeatedField<
            'this,
            ::puroro::internal::NoAllocBumpVec<::puroro::internal::NoAllocBumpString>,
            ::puroro::internal::NoAllocBumpString,
            str,
        >;

        fn file_to_generate<'this>(&'this self) -> Self::FileToGenerateRepeatedType<'this> {
            ::puroro::AsRefRepeatedField::new(&self.file_to_generate)
        }
        fn parameter_opt<'this>(&'this self) -> Option<&'this str> {
            <Self>::parameter_opt(self)
        }
        type ProtoFileMessageType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::google::protobuf::_puroro_impls::FileDescriptorProtoBumpalo<
            'this,
        >;
        type ProtoFileRepeatedType<'this> where Self: 'this =
    &'this [self::_puroro_root::google::protobuf::_puroro_impls::FileDescriptorProtoBumpalo<'this>];

        fn proto_file<'this>(&'this self) -> Self::ProtoFileRepeatedType<'this> {
            unsafe { self.proto_file.cast_item_unchecked() }
        }
        type CompilerVersionMessageType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::google::protobuf::compiler::_puroro_impls::VersionBumpalo<
            'this,
        >;
        fn compiler_version_opt<'this>(
            &'this self,
        ) -> Option<Self::CompilerVersionMessageType<'this>> {
            <Self>::compiler_version_opt(self)
        }
    }

    impl<'bump> ::puroro::internal::de::DeserMessageFromBytesIter
        for CodeGeneratorRequestBumpalo<'bump>
    {
        fn deser_field<'this, I>(
            &'this mut self,
            field_number: i32,
            data: ::puroro::internal::types::FieldData<
                &mut ::puroro::internal::de::from_iter::ScopedIter<I>,
            >,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro::internal::impls::bumpalo::de::DeserFieldFromBytesIter;
            match field_number {
            1 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::String
                >::deser_field(&mut self.file_to_generate, data, self._bump)
            }
            2 => {
                self._bitfield.set(0, true);
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::String
                >::deser_field(&mut self.parameter, data, self._bump)
            }
            15 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_impls::FileDescriptorProtoBumpalo<'bump>>
                >::deser_field(&mut self.proto_file, data, self._bump)
            }
            3 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Message<::puroro::internal::NoAllocBumpBox<self::_puroro_root::google::protobuf::compiler::_puroro_impls::VersionBumpalo<'bump>>>
                >::deser_field(&mut self.compiler_version, data, self._bump)
            }

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl<'bump> ::puroro::internal::se::SerMessageToIoWrite for CodeGeneratorRequestBumpalo<'bump>
    where
        Self: super::_puroro_traits::CodeGeneratorRequestTrait,
        for<'a> <Self as super::_puroro_traits::CodeGeneratorRequestTrait>::ProtoFileMessageType<'a>:
            ::puroro::internal::se::SerMessageToIoWrite,
        for<'a> <Self as super::_puroro_traits::CodeGeneratorRequestTrait>::CompilerVersionMessageType<'a>:
            ::puroro::internal::se::SerMessageToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::CodeGeneratorRequestTrait>::file_to_generate(self),
                1,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::CodeGeneratorRequestTrait>::parameter_opt(self),
                2,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Message<<Self as super::_puroro_traits::CodeGeneratorRequestTrait>::ProtoFileMessageType<'_>>
        >::ser_field(
            <Self as super::_puroro_traits::CodeGeneratorRequestTrait>::proto_file(self),
            15,
            out
        )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
            ::puroro::tags::Optional, ::puroro::tags::Message<<Self as super::_puroro_traits::CodeGeneratorRequestTrait>::CompilerVersionMessageType<'_>>
        >::ser_field(
            <Self as super::_puroro_traits::CodeGeneratorRequestTrait>::compiler_version_opt(self),
            3,
            out
        )?;
            ::std::result::Result::Ok(())
        }
    }
    pub struct CodeGeneratorRequestBuilder<T>(T);

    impl<T> CodeGeneratorRequestBuilder<T>
    where
        T: CodeGeneratorRequestTrait,
    {
        pub fn append_file_to_generate<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> CodeGeneratorRequestBuilder<(
            T,
            CodeGeneratorRequestSingleField1<ScalarType, RepeatedType>,
        )>
        where
            ScalarType: ::std::convert::AsRef<str>,
            for<'a> &'a RepeatedType:
                ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            CodeGeneratorRequestBuilder((
                self.0,
                CodeGeneratorRequestSingleField1 {
                    file_to_generate: value,
                },
            ))
        }

        pub fn append_parameter<ScalarType>(
            self,
            value: ScalarType,
        ) -> CodeGeneratorRequestBuilder<(T, CodeGeneratorRequestSingleField2<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<str>,
        {
            CodeGeneratorRequestBuilder((
                self.0,
                CodeGeneratorRequestSingleField2 { parameter: value },
            ))
        }

        pub fn append_proto_file<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> CodeGeneratorRequestBuilder<(
            T,
            CodeGeneratorRequestSingleField15<ScalarType, RepeatedType>,
        )>
        where
            ScalarType:
                self::_puroro_root::google::protobuf::_puroro_traits::FileDescriptorProtoTrait,
            for<'a> &'a RepeatedType:
                ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            CodeGeneratorRequestBuilder((
                self.0,
                CodeGeneratorRequestSingleField15 { proto_file: value },
            ))
        }

        pub fn append_compiler_version<ScalarType>(
            self,
            value: ScalarType,
        ) -> CodeGeneratorRequestBuilder<(T, CodeGeneratorRequestSingleField3<ScalarType>)>
        where
            ScalarType:
                self::_puroro_root::google::protobuf::compiler::_puroro_traits::VersionTrait,
        {
            CodeGeneratorRequestBuilder((
                self.0,
                CodeGeneratorRequestSingleField3 {
                    compiler_version: value,
                },
            ))
        }

        pub fn build(self) -> T {
            self.0
        }
    }

    impl CodeGeneratorRequestBuilder<()> {
        pub fn new() -> Self {
            Self(())
        }
    }

    pub struct CodeGeneratorResponseSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>,
    {
        pub error: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::CodeGeneratorResponse>
        for CodeGeneratorResponseSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>,
    {
    }

    impl<ScalarType> super::_puroro_traits::CodeGeneratorResponseTrait
        for CodeGeneratorResponseSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>,
    {
        fn error_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::Some(self.error.as_ref())
        }
        type FileMessageType<'this>
        where
            Self: 'this,
        = ();
        type FileRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::FileMessageType<'this>>;
        fn file<'this>(&'this self) -> Self::FileRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite
        for CodeGeneratorResponseSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>,
        Self: super::_puroro_traits::CodeGeneratorResponseTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::CodeGeneratorResponseTrait>::error_opt(self),
                1,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for CodeGeneratorResponseSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>,
    {
        fn from(value: ScalarType) -> Self {
            Self { error: value }
        }
    }

    impl<ScalarType> ::std::clone::Clone for CodeGeneratorResponseSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                error: ::std::clone::Clone::clone(&self.error),
            }
        }
    }

    pub struct CodeGeneratorResponseSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone,
    {
        pub supported_features: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::CodeGeneratorResponse>
        for CodeGeneratorResponseSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone,
    {
    }

    impl<ScalarType> super::_puroro_traits::CodeGeneratorResponseTrait
        for CodeGeneratorResponseSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone,
    {
        fn supported_features_opt<'this>(&'this self) -> ::std::option::Option<u64> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.supported_features,
            )))
        }
        type FileMessageType<'this>
        where
            Self: 'this,
        = ();
        type FileRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::FileMessageType<'this>>;
        fn file<'this>(&'this self) -> Self::FileRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite
        for CodeGeneratorResponseSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone,
        Self: super::_puroro_traits::CodeGeneratorResponseTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::UInt64,
            >::ser_field(
                <Self as super::_puroro_traits::CodeGeneratorResponseTrait>::supported_features_opt(
                    self,
                ),
                2,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for CodeGeneratorResponseSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                supported_features: value,
            }
        }
    }

    impl<ScalarType> ::std::clone::Clone for CodeGeneratorResponseSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone,
        ScalarType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                supported_features: ::std::clone::Clone::clone(&self.supported_features),
            }
        }
    }

    pub struct CodeGeneratorResponseSingleField15<ScalarType, RepeatedType>
where

ScalarType:
    self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_traits::FileTrait,
for <'a> &'a RepeatedType: ::puroro::RepeatedField<'a> + 
    ::std::iter::IntoIterator<Item = &'a ScalarType>,
{
    pub file: RepeatedType,
}

    impl<ScalarType, RepeatedType> ::puroro::Message<super::CodeGeneratorResponse>
for CodeGeneratorResponseSingleField15<ScalarType, RepeatedType>
where

ScalarType:
    self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_traits::FileTrait,
for <'a> &'a RepeatedType: ::puroro::RepeatedField<'a> + 
    ::std::iter::IntoIterator<Item = &'a ScalarType>,
{}

    impl<ScalarType, RepeatedType> super::_puroro_traits::CodeGeneratorResponseTrait
for CodeGeneratorResponseSingleField15<ScalarType, RepeatedType>
where

ScalarType:
    self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_traits::FileTrait,
for <'a> &'a RepeatedType: ::puroro::RepeatedField<'a> + 
    ::std::iter::IntoIterator<Item = &'a ScalarType>,
{
type FileMessageType<'this> where Self: 'this = &'this ScalarType;

type FileRepeatedType<'this> where Self: 'this =
    &'this RepeatedType;

fn file<'this>(&'this self) -> Self::FileRepeatedType<'this> {
    &self.file
}
}

    impl<ScalarType, RepeatedType> ::puroro::internal::se::SerMessageToIoWrite
for CodeGeneratorResponseSingleField15<ScalarType, RepeatedType>
where

ScalarType:
    self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_traits::FileTrait,
for <'a> &'a RepeatedType: ::puroro::RepeatedField<'a> + 
    ::std::iter::IntoIterator<Item = &'a ScalarType>,
    Self: super::_puroro_traits::CodeGeneratorResponseTrait,
    for<'a> <Self as super::_puroro_traits::CodeGeneratorResponseTrait>::FileMessageType<'a> :
        ::puroro::internal::se::SerMessageToIoWrite,
{
    fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
    where
        W: ::std::io::Write,
    {
        ::puroro::internal::se::SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Message<<Self as super::_puroro_traits::CodeGeneratorResponseTrait>::FileMessageType<'_>>
        >::ser_field(
            <Self as super::_puroro_traits::CodeGeneratorResponseTrait>::file(self),
            15,
            out
        )?;
        ::std::result::Result::Ok(())
    }
}

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
for CodeGeneratorResponseSingleField15<ScalarType, RepeatedType>
where

ScalarType:
    self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_traits::FileTrait,
for <'a> &'a RepeatedType: ::puroro::RepeatedField<'a> + 
    ::std::iter::IntoIterator<Item = &'a ScalarType>,
{
    fn from(value: RepeatedType) -> Self {
        Self {
            file: value,
        }
    }
}

    impl<ScalarType, RepeatedType> ::std::clone::Clone
for CodeGeneratorResponseSingleField15<ScalarType, RepeatedType>
where

ScalarType:
    self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_traits::FileTrait,
for <'a> &'a RepeatedType: ::puroro::RepeatedField<'a> + 
    ::std::iter::IntoIterator<Item = &'a ScalarType>,
    RepeatedType: ::std::clone::Clone,
{
    fn clone(&self) -> Self {
        Self {
            file: ::std::clone::Clone::clone(&self.file),
        }
    }
}
    pub struct CodeGeneratorResponseBumpalo<'bump> {
    _bump: &'bump ::puroro::bumpalo::Bump,
    _bitfield: ::puroro::bitvec::array::BitArray<
        ::puroro::bitvec::order::Lsb0,
        [u32; (2 + 31) / 32],
    >,
    error: ::puroro::internal::Bare<::puroro::internal::NoAllocBumpString>,
    supported_features: ::puroro::internal::Bare<u64>,
    file: ::puroro::internal::NoAllocBumpVec<self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_impls::FileBumpalo<'bump>>,
}

    pub type CodeGeneratorResponseBumpaloOwned =
        ::puroro::BumpaloOwned<CodeGeneratorResponseBumpalo<'static>>;
    impl<'bump> CodeGeneratorResponseBumpalo<'bump> {
        pub fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
            #[allow(unused)]
            let bump_ref: &::puroro::bumpalo::Bump =
                unsafe { ::std::mem::transmute(::std::ops::Deref::deref(&bump)) };

            Self {
                _bump: bump,
                _bitfield: ::std::default::Default::default(),
                error: ::std::default::Default::default(),
                supported_features: ::std::default::Default::default(),
                file: ::std::default::Default::default(),
            }
        }
        pub fn error_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            if self._bitfield.get(0).map_or(false, |v| *v) {
                ::std::option::Option::Some(&self.error)
            } else {
                ::std::option::Option::None
            }
        }
        pub fn error<'this>(&'this self) -> &'this str {
            match self.error_opt() {
                ::std::option::Option::Some(x) => x,
                _ => ::std::default::Default::default(),
            }
        }

        pub fn has_error(&self) -> bool {
            self.error_opt().is_some()
        }
        pub fn supported_features_opt<'this>(&'this self) -> ::std::option::Option<u64> {
            if self._bitfield.get(1).map_or(false, |v| *v) {
                ::std::option::Option::Some(self.supported_features.inner())
            } else {
                ::std::option::Option::None
            }
        }
        pub fn supported_features<'this>(&'this self) -> u64 {
            match self.supported_features_opt() {
                ::std::option::Option::Some(x) => x,
                _ => ::std::default::Default::default(),
            }
        }

        pub fn has_supported_features(&self) -> bool {
            self.supported_features_opt().is_some()
        }
        pub fn file<'this>(&'this self) -> &'this[self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_impls::FileBumpalo<'this>]{
            unsafe { self.file.cast_item_unchecked() }
        }
        pub fn clear_error(&mut self) {
            self._bitfield.set(0, false);
        }
        pub fn error_mut<'this>(
            &'this mut self,
        ) -> impl 'this + ::std::ops::DerefMut<Target = ::puroro::bumpalo::collections::String<'bump>>
        {
            if !self.has_error() {
                self.error = ::std::default::Default::default();
                self._bitfield.set(0, true);
            }
            unsafe { self.error.as_mut_string_in(self._bump) }
        }
        pub fn clear_supported_features(&mut self) {
            self._bitfield.set(1, false);
        }
        pub fn supported_features_mut<'this>(&'this mut self) -> &'this mut u64 {
            if !self.has_supported_features() {
                self.supported_features = ::std::default::Default::default();
                self._bitfield.set(1, true);
            }
            &mut self.supported_features
        }
        pub fn file_mut<'this>(&'this mut self) -> ::puroro::internal::RefMutBumpVec<'bump, 'this, self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_impls::FileBumpalo<'bump>>{
            unsafe { self.file.as_mut_vec_in(self._bump) }
        }
    }
    impl<'bump> ::puroro::Message<super::_puroro_simple_impl::CodeGeneratorResponse>
        for CodeGeneratorResponseBumpalo<'bump>
    {
    }

    impl<'bump> ::puroro::BumpaloMessage<'bump> for CodeGeneratorResponseBumpalo<'bump> {
        fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
            Self::new_in(bump)
        }
    }

    impl<'bump> ::puroro::internal::BumpDefault<'bump> for CodeGeneratorResponseBumpalo<'bump> {
        fn default_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
            Self::new_in(bump)
        }
    }

    impl<'bump> super::_puroro_traits::CodeGeneratorResponseTrait
        for CodeGeneratorResponseBumpalo<'bump>
    {
        fn error_opt<'this>(&'this self) -> Option<&'this str> {
            <Self>::error_opt(self)
        }
        fn supported_features_opt<'this>(&'this self) -> Option<u64> {
            <Self>::supported_features_opt(self)
        }
        type FileMessageType<'this> where Self: 'this = &'this self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_impls::FileBumpalo<'this>;
        type FileRepeatedType<'this> where Self: 'this =
    &'this [self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_impls::FileBumpalo<'this>];

        fn file<'this>(&'this self) -> Self::FileRepeatedType<'this> {
            unsafe { self.file.cast_item_unchecked() }
        }
    }

    impl<'bump> ::puroro::internal::de::DeserMessageFromBytesIter
        for CodeGeneratorResponseBumpalo<'bump>
    {
        fn deser_field<'this, I>(
            &'this mut self,
            field_number: i32,
            data: ::puroro::internal::types::FieldData<
                &mut ::puroro::internal::de::from_iter::ScopedIter<I>,
            >,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro::internal::impls::bumpalo::de::DeserFieldFromBytesIter;
            match field_number {
            1 => {
                self._bitfield.set(0, true);
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::String
                >::deser_field(&mut self.error, data, self._bump)
            }
            2 => {
                self._bitfield.set(1, true);
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::UInt64
                >::deser_field(&mut self.supported_features, data, self._bump)
            }
            15 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_impls::FileBumpalo<'bump>>
                >::deser_field(&mut self.file, data, self._bump)
            }

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl<'bump> ::puroro::internal::se::SerMessageToIoWrite for CodeGeneratorResponseBumpalo<'bump>
    where
        Self: super::_puroro_traits::CodeGeneratorResponseTrait,
        for<'a> <Self as super::_puroro_traits::CodeGeneratorResponseTrait>::FileMessageType<'a>:
            ::puroro::internal::se::SerMessageToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::CodeGeneratorResponseTrait>::error_opt(self),
                1,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::UInt64,
            >::ser_field(
                <Self as super::_puroro_traits::CodeGeneratorResponseTrait>::supported_features_opt(
                    self,
                ),
                2,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    <Self as super::_puroro_traits::CodeGeneratorResponseTrait>::FileMessageType<
                        '_,
                    >,
                >,
            >::ser_field(
                <Self as super::_puroro_traits::CodeGeneratorResponseTrait>::file(self),
                15,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }
    pub struct CodeGeneratorResponseBuilder<T>(T);

    impl<T> CodeGeneratorResponseBuilder<T>
    where
        T: CodeGeneratorResponseTrait,
    {
        pub fn append_error<ScalarType>(
            self,
            value: ScalarType,
        ) -> CodeGeneratorResponseBuilder<(T, CodeGeneratorResponseSingleField1<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<str>,
        {
            CodeGeneratorResponseBuilder((
                self.0,
                CodeGeneratorResponseSingleField1 { error: value },
            ))
        }

        pub fn append_supported_features<ScalarType>(
            self,
            value: ScalarType,
        ) -> CodeGeneratorResponseBuilder<(T, CodeGeneratorResponseSingleField2<ScalarType>)>
        where
            ScalarType: ::std::convert::Into<u64> + ::std::clone::Clone,
        {
            CodeGeneratorResponseBuilder((
                self.0,
                CodeGeneratorResponseSingleField2 {
                    supported_features: value,
                },
            ))
        }

    pub fn append_file<ScalarType, RepeatedType>(self, value: RepeatedType)
        -> CodeGeneratorResponseBuilder<(T, CodeGeneratorResponseSingleField15<ScalarType, RepeatedType>)>
where

ScalarType:
    self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_traits::FileTrait,
for <'a> &'a RepeatedType: ::puroro::RepeatedField<'a> +
    ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
            CodeGeneratorResponseBuilder((
                self.0,
                CodeGeneratorResponseSingleField15 { file: value },
            ))
        }

        pub fn build(self) -> T {
            self.0
        }
    }

    impl CodeGeneratorResponseBuilder<()> {
        pub fn new() -> Self {
            Self(())
        }
    }
}
pub use _puroro_traits::*;
pub mod _puroro_traits {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }

    pub trait VersionTrait {
        fn major<'this>(&'this self) -> i32 {
            self.major_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_major<'this>(&'this self) -> bool {
            self.major_opt().is_some()
        }
        fn major_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::None
        }
        fn minor<'this>(&'this self) -> i32 {
            self.minor_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_minor<'this>(&'this self) -> bool {
            self.minor_opt().is_some()
        }
        fn minor_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::None
        }
        fn patch<'this>(&'this self) -> i32 {
            self.patch_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_patch<'this>(&'this self) -> bool {
            self.patch_opt().is_some()
        }
        fn patch_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::None
        }
        fn suffix<'this>(&'this self) -> &'this str {
            self.suffix_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_suffix<'this>(&'this self) -> bool {
            self.suffix_opt().is_some()
        }
        fn suffix_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::None
        }
    }

    macro_rules! version_delegate {
        ($ty:ty) => {
            fn major_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).major_opt()
            }
            fn minor_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).minor_opt()
            }
            fn patch_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).patch_opt()
            }
            fn suffix_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                (**self).suffix_opt()
            }
        };
    }

    impl<T> VersionTrait for &'_ T
    where
        T: VersionTrait,
    {
        version_delegate!(T);
    }

    impl<T> VersionTrait for &'_ mut T
    where
        T: VersionTrait,
    {
        version_delegate!(T);
    }

    impl<T> VersionTrait for ::std::boxed::Box<T>
    where
        T: VersionTrait,
    {
        version_delegate!(T);
    }

    impl<'bump, T> VersionTrait for ::puroro::bumpalo::boxed::Box<'bump, T>
    where
        T: VersionTrait,
    {
        version_delegate!(T);
    }

    impl<T> VersionTrait for ::puroro::BumpaloOwned<T>
    where
        T: VersionTrait,
    {
        version_delegate!(T);
    }
    impl VersionTrait for () {}
    impl<T, U> VersionTrait for (T, U)
    where
        T: VersionTrait,
        U: VersionTrait,
    {
        fn major_opt<'this>(&'this self) -> Option<i32> {
            <U as VersionTrait>::major_opt(&self.1)
                .or_else(|| <T as VersionTrait>::major_opt(&self.0))
        }
        fn minor_opt<'this>(&'this self) -> Option<i32> {
            <U as VersionTrait>::minor_opt(&self.1)
                .or_else(|| <T as VersionTrait>::minor_opt(&self.0))
        }
        fn patch_opt<'this>(&'this self) -> Option<i32> {
            <U as VersionTrait>::patch_opt(&self.1)
                .or_else(|| <T as VersionTrait>::patch_opt(&self.0))
        }
        fn suffix_opt<'this>(&'this self) -> Option<&'this str> {
            <U as VersionTrait>::suffix_opt(&self.1)
                .or_else(|| <T as VersionTrait>::suffix_opt(&self.0))
        }
    }
    impl<T, U> VersionTrait for ::puroro::Either<T, U>
    where
        T: VersionTrait,
        U: VersionTrait,
    {
        fn major_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().either(
                |t| <T as VersionTrait>::major_opt(t),
                |u| <U as VersionTrait>::major_opt(u),
            )
        }
        fn minor_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().either(
                |t| <T as VersionTrait>::minor_opt(t),
                |u| <U as VersionTrait>::minor_opt(u),
            )
        }
        fn patch_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().either(
                |t| <T as VersionTrait>::patch_opt(t),
                |u| <U as VersionTrait>::patch_opt(u),
            )
        }
        fn suffix_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().either(
                |t| <T as VersionTrait>::suffix_opt(t),
                |u| <U as VersionTrait>::suffix_opt(u),
            )
        }
    }
    impl<T> VersionTrait for ::std::option::Option<T>
    where
        T: VersionTrait,
    {
        fn major_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().and_then(|msg| msg.major_opt())
        }
        fn minor_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().and_then(|msg| msg.minor_opt())
        }
        fn patch_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().and_then(|msg| msg.patch_opt())
        }
        fn suffix_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().and_then(|msg| msg.suffix_opt())
        }
    }

    pub trait CodeGeneratorRequestTrait {
        type FileToGenerateRepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = &'this str>
        where
            Self: 'this;
        fn file_to_generate<'this>(&'this self) -> Self::FileToGenerateRepeatedType<'this>;
        fn parameter<'this>(&'this self) -> &'this str {
            self.parameter_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_parameter<'this>(&'this self) -> bool {
            self.parameter_opt().is_some()
        }
        fn parameter_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::None
        }
        type ProtoFileMessageType<'this>: self::_puroro_root::google::protobuf::_puroro_traits::FileDescriptorProtoTrait
            where Self: 'this;

        type ProtoFileRepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::ProtoFileMessageType<'this>>
        where
            Self: 'this;
        fn proto_file<'this>(&'this self) -> Self::ProtoFileRepeatedType<'this>;
        type CompilerVersionMessageType<'this>: self::_puroro_root::google::protobuf::compiler::_puroro_traits::VersionTrait
            where Self: 'this;
        fn compiler_version<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::CompilerVersionMessageType<'this>> {
            self.compiler_version_opt()
        }
        fn has_compiler_version<'this>(&'this self) -> bool {
            self.compiler_version_opt().is_some()
        }
        fn compiler_version_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::CompilerVersionMessageType<'this>> {
            ::std::option::Option::None
        }
    }

    macro_rules! code_generator_request_delegate {
        ($ty:ty) => {
            type FileToGenerateRepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::FileToGenerateRepeatedType<'this>;
            fn file_to_generate<'this>(&'this self) -> Self::FileToGenerateRepeatedType<'this> {
                (**self).file_to_generate()
            }
            fn parameter_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                (**self).parameter_opt()
            }
            type ProtoFileMessageType<'this>
            where
                Self: 'this,
            = <$ty>::ProtoFileMessageType<'this>;

            type ProtoFileRepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::ProtoFileRepeatedType<'this>;
            fn proto_file<'this>(&'this self) -> Self::ProtoFileRepeatedType<'this> {
                (**self).proto_file()
            }
            type CompilerVersionMessageType<'this>
            where
                Self: 'this,
            = <$ty>::CompilerVersionMessageType<'this>;
            fn compiler_version_opt<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::CompilerVersionMessageType<'this>> {
                (**self).compiler_version_opt()
            }
        };
    }

    impl<T> CodeGeneratorRequestTrait for &'_ T
    where
        T: CodeGeneratorRequestTrait,
    {
        code_generator_request_delegate!(T);
    }

    impl<T> CodeGeneratorRequestTrait for &'_ mut T
    where
        T: CodeGeneratorRequestTrait,
    {
        code_generator_request_delegate!(T);
    }

    impl<T> CodeGeneratorRequestTrait for ::std::boxed::Box<T>
    where
        T: CodeGeneratorRequestTrait,
    {
        code_generator_request_delegate!(T);
    }

    impl<'bump, T> CodeGeneratorRequestTrait for ::puroro::bumpalo::boxed::Box<'bump, T>
    where
        T: CodeGeneratorRequestTrait,
    {
        code_generator_request_delegate!(T);
    }

    impl<T> CodeGeneratorRequestTrait for ::puroro::BumpaloOwned<T>
    where
        T: CodeGeneratorRequestTrait,
    {
        code_generator_request_delegate!(T);
    }
    impl CodeGeneratorRequestTrait for () {
        type FileToGenerateRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn file_to_generate<'this>(&'this self) -> Self::FileToGenerateRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type ProtoFileMessageType<'this>
        where
            Self: 'this,
        = ();
        type ProtoFileRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::ProtoFileMessageType<'this>>;
        fn proto_file<'this>(&'this self) -> Self::ProtoFileRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type CompilerVersionMessageType<'this>
        where
            Self: 'this,
        = ();
    }
    impl<T, U> CodeGeneratorRequestTrait for (T, U)
    where
        T: CodeGeneratorRequestTrait,
        U: CodeGeneratorRequestTrait,
    {
        type FileToGenerateRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as CodeGeneratorRequestTrait>::FileToGenerateRepeatedType<'this>,
            <U as CodeGeneratorRequestTrait>::FileToGenerateRepeatedType<'this>,
        >;

        fn file_to_generate<'this>(&'this self) -> Self::FileToGenerateRepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as CodeGeneratorRequestTrait>::file_to_generate(&self.0),
                <U as CodeGeneratorRequestTrait>::file_to_generate(&self.1),
            )
        }
        fn parameter_opt<'this>(&'this self) -> Option<&'this str> {
            <U as CodeGeneratorRequestTrait>::parameter_opt(&self.1)
                .or_else(|| <T as CodeGeneratorRequestTrait>::parameter_opt(&self.0))
        }
        type ProtoFileMessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as CodeGeneratorRequestTrait>::ProtoFileMessageType<'this>,
            <U as CodeGeneratorRequestTrait>::ProtoFileMessageType<'this>,
        >;
        type ProtoFileRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedMessageField<
            <T as CodeGeneratorRequestTrait>::ProtoFileRepeatedType<'this>,
            <U as CodeGeneratorRequestTrait>::ProtoFileRepeatedType<'this>,
        >;

        fn proto_file<'this>(&'this self) -> Self::ProtoFileRepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
                <T as CodeGeneratorRequestTrait>::proto_file(&self.0),
                <U as CodeGeneratorRequestTrait>::proto_file(&self.1),
            )
        }
        type CompilerVersionMessageType<'this>
        where
            Self: 'this,
        = (
            ::std::option::Option<
                <T as CodeGeneratorRequestTrait>::CompilerVersionMessageType<'this>,
            >,
            ::std::option::Option<
                <U as CodeGeneratorRequestTrait>::CompilerVersionMessageType<'this>,
            >,
        );
        fn compiler_version_opt<'this>(
            &'this self,
        ) -> Option<Self::CompilerVersionMessageType<'this>> {
            match (
                <T as CodeGeneratorRequestTrait>::compiler_version_opt(&self.0),
                <U as CodeGeneratorRequestTrait>::compiler_version_opt(&self.1),
            ) {
                (None, None) => None,
                (Some(t), None) => Some((Some(t), None)),
                (None, Some(u)) => Some((None, Some(u))),
                (Some(t), Some(u)) => Some((Some(t), Some(u))),
            }
        }
    }
    impl<T, U> CodeGeneratorRequestTrait for ::puroro::Either<T, U>
    where
        T: CodeGeneratorRequestTrait,
        U: CodeGeneratorRequestTrait,
    {
        type FileToGenerateRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as CodeGeneratorRequestTrait>::FileToGenerateRepeatedType<'this>,
            <U as CodeGeneratorRequestTrait>::FileToGenerateRepeatedType<'this>,
        >;

        fn file_to_generate<'this>(&'this self) -> Self::FileToGenerateRepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as CodeGeneratorRequestTrait>::file_to_generate(t))
                    .map_right(|u| <U as CodeGeneratorRequestTrait>::file_to_generate(u)),
            )
        }
        fn parameter_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().either(
                |t| <T as CodeGeneratorRequestTrait>::parameter_opt(t),
                |u| <U as CodeGeneratorRequestTrait>::parameter_opt(u),
            )
        }
        type ProtoFileMessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as CodeGeneratorRequestTrait>::ProtoFileMessageType<'this>,
            <U as CodeGeneratorRequestTrait>::ProtoFileMessageType<'this>,
        >;
        type ProtoFileRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedMessageField<
            <T as CodeGeneratorRequestTrait>::ProtoFileRepeatedType<'this>,
            <U as CodeGeneratorRequestTrait>::ProtoFileRepeatedType<'this>,
        >;

        fn proto_file<'this>(&'this self) -> Self::ProtoFileRepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as CodeGeneratorRequestTrait>::proto_file(t))
                    .map_right(|u| <U as CodeGeneratorRequestTrait>::proto_file(u)),
            )
        }
        type CompilerVersionMessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as CodeGeneratorRequestTrait>::CompilerVersionMessageType<'this>,
            <U as CodeGeneratorRequestTrait>::CompilerVersionMessageType<'this>,
        >;
        fn compiler_version_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::CompilerVersionMessageType<'this>> {
            self.as_ref().either(
                |t| {
                    <T as CodeGeneratorRequestTrait>::compiler_version_opt(t)
                        .map(|t| ::puroro::Either::Left(t))
                },
                |u| {
                    <U as CodeGeneratorRequestTrait>::compiler_version_opt(u)
                        .map(|u| ::puroro::Either::Right(u))
                },
            )
        }
    }
    impl<T> CodeGeneratorRequestTrait for ::std::option::Option<T>
    where
        T: CodeGeneratorRequestTrait,
    {
        type FileToGenerateRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::option::OptionRepeatedField<
            T::FileToGenerateRepeatedType<'this>,
        >;
        fn file_to_generate<'this>(&'this self) -> Self::FileToGenerateRepeatedType<'this> {
            ::puroro::internal::impls::option::OptionRepeatedField::new(
                self.as_ref().map(|msg| msg.file_to_generate()),
            )
        }
        fn parameter_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().and_then(|msg| msg.parameter_opt())
        }
        type ProtoFileMessageType<'this>
        where
            Self: 'this,
        = T::ProtoFileMessageType<'this>;

        type ProtoFileRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::option::OptionRepeatedField<T::ProtoFileRepeatedType<'this>>;
        fn proto_file<'this>(&'this self) -> Self::ProtoFileRepeatedType<'this> {
            ::puroro::internal::impls::option::OptionRepeatedField::new(
                self.as_ref().map(|msg| msg.proto_file()),
            )
        }
        type CompilerVersionMessageType<'this>
        where
            Self: 'this,
        = T::CompilerVersionMessageType<'this>;
        fn compiler_version_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::CompilerVersionMessageType<'this>> {
            self.as_ref().and_then(|msg| msg.compiler_version_opt())
        }
    }

    pub trait CodeGeneratorResponseTrait {
        fn error<'this>(&'this self) -> &'this str {
            self.error_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_error<'this>(&'this self) -> bool {
            self.error_opt().is_some()
        }
        fn error_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::None
        }
        fn supported_features<'this>(&'this self) -> u64 {
            self.supported_features_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_supported_features<'this>(&'this self) -> bool {
            self.supported_features_opt().is_some()
        }
        fn supported_features_opt<'this>(&'this self) -> ::std::option::Option<u64> {
            ::std::option::Option::None
        }
        type FileMessageType<'this>: self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_traits::FileTrait
            where Self: 'this;

        type FileRepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::FileMessageType<'this>>
        where
            Self: 'this;
        fn file<'this>(&'this self) -> Self::FileRepeatedType<'this>;
    }

    macro_rules! code_generator_response_delegate {
        ($ty:ty) => {
            fn error_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                (**self).error_opt()
            }
            fn supported_features_opt<'this>(&'this self) -> ::std::option::Option<u64> {
                (**self).supported_features_opt()
            }
            type FileMessageType<'this>
            where
                Self: 'this,
            = <$ty>::FileMessageType<'this>;

            type FileRepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::FileRepeatedType<'this>;
            fn file<'this>(&'this self) -> Self::FileRepeatedType<'this> {
                (**self).file()
            }
        };
    }

    impl<T> CodeGeneratorResponseTrait for &'_ T
    where
        T: CodeGeneratorResponseTrait,
    {
        code_generator_response_delegate!(T);
    }

    impl<T> CodeGeneratorResponseTrait for &'_ mut T
    where
        T: CodeGeneratorResponseTrait,
    {
        code_generator_response_delegate!(T);
    }

    impl<T> CodeGeneratorResponseTrait for ::std::boxed::Box<T>
    where
        T: CodeGeneratorResponseTrait,
    {
        code_generator_response_delegate!(T);
    }

    impl<'bump, T> CodeGeneratorResponseTrait for ::puroro::bumpalo::boxed::Box<'bump, T>
    where
        T: CodeGeneratorResponseTrait,
    {
        code_generator_response_delegate!(T);
    }

    impl<T> CodeGeneratorResponseTrait for ::puroro::BumpaloOwned<T>
    where
        T: CodeGeneratorResponseTrait,
    {
        code_generator_response_delegate!(T);
    }
    impl CodeGeneratorResponseTrait for () {
        type FileMessageType<'this>
        where
            Self: 'this,
        = ();
        type FileRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::FileMessageType<'this>>;
        fn file<'this>(&'this self) -> Self::FileRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }
    impl<T, U> CodeGeneratorResponseTrait for (T, U)
    where
        T: CodeGeneratorResponseTrait,
        U: CodeGeneratorResponseTrait,
    {
        fn error_opt<'this>(&'this self) -> Option<&'this str> {
            <U as CodeGeneratorResponseTrait>::error_opt(&self.1)
                .or_else(|| <T as CodeGeneratorResponseTrait>::error_opt(&self.0))
        }
        fn supported_features_opt<'this>(&'this self) -> Option<u64> {
            <U as CodeGeneratorResponseTrait>::supported_features_opt(&self.1)
                .or_else(|| <T as CodeGeneratorResponseTrait>::supported_features_opt(&self.0))
        }
        type FileMessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as CodeGeneratorResponseTrait>::FileMessageType<'this>,
            <U as CodeGeneratorResponseTrait>::FileMessageType<'this>,
        >;
        type FileRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedMessageField<
            <T as CodeGeneratorResponseTrait>::FileRepeatedType<'this>,
            <U as CodeGeneratorResponseTrait>::FileRepeatedType<'this>,
        >;

        fn file<'this>(&'this self) -> Self::FileRepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
                <T as CodeGeneratorResponseTrait>::file(&self.0),
                <U as CodeGeneratorResponseTrait>::file(&self.1),
            )
        }
    }
    impl<T, U> CodeGeneratorResponseTrait for ::puroro::Either<T, U>
    where
        T: CodeGeneratorResponseTrait,
        U: CodeGeneratorResponseTrait,
    {
        fn error_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().either(
                |t| <T as CodeGeneratorResponseTrait>::error_opt(t),
                |u| <U as CodeGeneratorResponseTrait>::error_opt(u),
            )
        }
        fn supported_features_opt<'this>(&'this self) -> ::std::option::Option<u64> {
            self.as_ref().either(
                |t| <T as CodeGeneratorResponseTrait>::supported_features_opt(t),
                |u| <U as CodeGeneratorResponseTrait>::supported_features_opt(u),
            )
        }
        type FileMessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as CodeGeneratorResponseTrait>::FileMessageType<'this>,
            <U as CodeGeneratorResponseTrait>::FileMessageType<'this>,
        >;
        type FileRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedMessageField<
            <T as CodeGeneratorResponseTrait>::FileRepeatedType<'this>,
            <U as CodeGeneratorResponseTrait>::FileRepeatedType<'this>,
        >;

        fn file<'this>(&'this self) -> Self::FileRepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as CodeGeneratorResponseTrait>::file(t))
                    .map_right(|u| <U as CodeGeneratorResponseTrait>::file(u)),
            )
        }
    }
    impl<T> CodeGeneratorResponseTrait for ::std::option::Option<T>
    where
        T: CodeGeneratorResponseTrait,
    {
        fn error_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().and_then(|msg| msg.error_opt())
        }
        fn supported_features_opt<'this>(&'this self) -> ::std::option::Option<u64> {
            self.as_ref().and_then(|msg| msg.supported_features_opt())
        }
        type FileMessageType<'this>
        where
            Self: 'this,
        = T::FileMessageType<'this>;

        type FileRepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::option::OptionRepeatedField<T::FileRepeatedType<'this>>;
        fn file<'this>(&'this self) -> Self::FileRepeatedType<'this> {
            ::puroro::internal::impls::option::OptionRepeatedField::new(
                self.as_ref().map(|msg| msg.file()),
            )
        }
    }
}
pub use _puroro_nested::*;
pub mod _puroro_nested {
    pub mod version {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
    }
    pub mod code_generator_request {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
    }
    pub mod code_generator_response {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }

        pub use _puroro_simple_impl::File;
        pub mod _puroro_simple_impl {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }
            pub struct File {
            _bitfield: ::puroro::bitvec::array::BitArray<
                ::puroro::bitvec::order::Lsb0,
                [u32; (3 + 31) / 32],
            >,
            name: ::puroro::internal::Bare<::std::string::String>,
            insertion_point: ::puroro::internal::Bare<::std::string::String>,
            content: ::puroro::internal::Bare<::std::string::String>,
            generated_code_info: ::std::option::Option<::std::boxed::Box<self::_puroro_root::google::protobuf::_puroro_simple_impl::GeneratedCodeInfo>>,
        }
            impl ::puroro::Message<File> for File {}

            impl File {
                pub fn new() -> Self {
                    Self {
                        _bitfield: ::std::default::Default::default(),
                        name: ::std::default::Default::default(),
                        insertion_point: ::std::default::Default::default(),
                        content: ::std::default::Default::default(),
                        generated_code_info: ::std::default::Default::default(),
                    }
                }
                pub fn name_opt(&self) -> ::std::option::Option<&'_ str> {
                    if self._bitfield.get(0).map_or(false, |v| *v) {
                        ::std::option::Option::Some(&self.name)
                    } else {
                        ::std::option::Option::None
                    }
                }

                pub fn has_name(&self) -> bool {
                    Self::name_opt(self).is_some()
                }

                pub fn name(&self) -> &'_ str {
                    self.name_opt()
                        .unwrap_or(::std::default::Default::default())
                }
                pub fn insertion_point_opt(&self) -> ::std::option::Option<&'_ str> {
                    if self._bitfield.get(1).map_or(false, |v| *v) {
                        ::std::option::Option::Some(&self.insertion_point)
                    } else {
                        ::std::option::Option::None
                    }
                }

                pub fn has_insertion_point(&self) -> bool {
                    Self::insertion_point_opt(self).is_some()
                }

                pub fn insertion_point(&self) -> &'_ str {
                    self.insertion_point_opt()
                        .unwrap_or(::std::default::Default::default())
                }
                pub fn content_opt(&self) -> ::std::option::Option<&'_ str> {
                    if self._bitfield.get(2).map_or(false, |v| *v) {
                        ::std::option::Option::Some(&self.content)
                    } else {
                        ::std::option::Option::None
                    }
                }

                pub fn has_content(&self) -> bool {
                    Self::content_opt(self).is_some()
                }

                pub fn content(&self) -> &'_ str {
                    self.content_opt()
                        .unwrap_or(::std::default::Default::default())
                }
                pub fn generated_code_info_opt(&self) -> ::std::option::Option<&'_ self::_puroro_root::google::protobuf::_puroro_simple_impl::GeneratedCodeInfo>{
                    self.generated_code_info.as_deref()
                }

                pub fn has_generated_code_info(&self) -> bool {
                    Self::generated_code_info_opt(self).is_some()
                }
        
                pub fn generated_code_info(&self) -> ::std::option::Option<&'_ self::_puroro_root::google::protobuf::_puroro_simple_impl::GeneratedCodeInfo>{
                    self.generated_code_info_opt()
                }
                pub fn clear_name(&mut self) {
                    self._bitfield.set(0, false);
                }
                pub fn name_mut(&mut self) -> &'_ mut ::std::string::String {
                    if !self.has_name() {
                        self.name = ::std::default::Default::default();
                        self._bitfield.set(0, true);
                    }
                    &mut self.name
                }
                pub fn clear_insertion_point(&mut self) {
                    self._bitfield.set(1, false);
                }
                pub fn insertion_point_mut(&mut self) -> &'_ mut ::std::string::String {
                    if !self.has_insertion_point() {
                        self.insertion_point = ::std::default::Default::default();
                        self._bitfield.set(1, true);
                    }
                    &mut self.insertion_point
                }
                pub fn clear_content(&mut self) {
                    self._bitfield.set(2, false);
                }
                pub fn content_mut(&mut self) -> &'_ mut ::std::string::String {
                    if !self.has_content() {
                        self.content = ::std::default::Default::default();
                        self._bitfield.set(2, true);
                    }
                    &mut self.content
                }
                pub fn clear_generated_code_info(&mut self) {
                    self.generated_code_info = ::std::default::Default::default();
                }
                pub fn generated_code_info_mut(&mut self) -> &'_ mut self::_puroro_root::google::protobuf::_puroro_simple_impl::GeneratedCodeInfo{
                    if !self.has_generated_code_info() {
                        self.generated_code_info = ::std::default::Default::default();
                    }
                    self.generated_code_info
                        .get_or_insert_with(::std::default::Default::default)
                }
            }

            impl super::_puroro_traits::FileTrait for File {
                fn name_opt<'this>(&'this self) -> Option<&'this str> {
                    <self::File>::name_opt(self)
                }
                fn insertion_point_opt<'this>(&'this self) -> Option<&'this str> {
                    <self::File>::insertion_point_opt(self)
                }
                fn content_opt<'this>(&'this self) -> Option<&'this str> {
                    <self::File>::content_opt(self)
                }
                type GeneratedCodeInfoMessageType<'this> where Self: 'this = &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::GeneratedCodeInfo;
                fn generated_code_info_opt<'this>(
                    &'this self,
                ) -> Option<Self::GeneratedCodeInfoMessageType<'this>> {
                    <self::File>::generated_code_info_opt(self)
                }
            }

            impl ::puroro::MessageRepresentativeImpl for File {}

            impl ::puroro::internal::de::DeserMessageFromBytesIter for File {
                fn deser_field<I>(
                    &mut self,
                    field_number: i32,
                    data: ::puroro::internal::types::FieldData<
                        &mut ::puroro::internal::de::from_iter::ScopedIter<I>,
                    >,
                ) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    use ::puroro::internal::impls::simple::de::DeserFieldFromBytesIter;
                    match field_number {
                    1 => {
                        self._bitfield.set(0, true);
                        DeserFieldFromBytesIter::<
                            ::puroro::tags::Optional, ::puroro::tags::String
                        >::deser_field(&mut self.name, data)
                    }
                    2 => {
                        self._bitfield.set(1, true);
                        DeserFieldFromBytesIter::<
                            ::puroro::tags::Optional, ::puroro::tags::String
                        >::deser_field(&mut self.insertion_point, data)
                    }
                    15 => {
                        self._bitfield.set(2, true);
                        DeserFieldFromBytesIter::<
                            ::puroro::tags::Optional, ::puroro::tags::String
                        >::deser_field(&mut self.content, data)
                    }
                    16 => {
                        DeserFieldFromBytesIter::<
                            ::puroro::tags::Optional, ::puroro::tags::Message<::std::boxed::Box<self::_puroro_root::google::protobuf::_puroro_simple_impl::GeneratedCodeInfo>>
                        >::deser_field(&mut self.generated_code_info, data)
                    }
        
                    _ => unimplemented!("TODO: This case should be handled properly..."),
                }
                }
            }

            impl ::puroro::internal::se::SerMessageToIoWrite for File
            where
                Self: super::_puroro_traits::FileTrait,
            {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    ::puroro::internal::se::SerFieldToIoWrite::<
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >::ser_field(
                        <Self as super::_puroro_traits::FileTrait>::name_opt(self),
                        1,
                        out,
                    )?;
                    ::puroro::internal::se::SerFieldToIoWrite::<
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >::ser_field(
                        <Self as super::_puroro_traits::FileTrait>::insertion_point_opt(self),
                        2,
                        out,
                    )?;
                    ::puroro::internal::se::SerFieldToIoWrite::<
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >::ser_field(
                        <Self as super::_puroro_traits::FileTrait>::content_opt(self),
                        15,
                        out,
                    )?;
                    ::puroro::internal::se::SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::Message<<Self as super::_puroro_traits::FileTrait>::GeneratedCodeInfoMessageType<'_>>
                >::ser_field(
                    <Self as super::_puroro_traits::FileTrait>::generated_code_info_opt(self),
                    16,
                    out
                )?;
                    ::std::result::Result::Ok(())
                }
            }

            impl ::std::default::Default for File {
                fn default() -> Self {
                    Self::new()
                }
            }

            impl ::std::fmt::Debug for File
            where
                Self: super::_puroro_traits::FileTrait,
            {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.debug_struct("File")
                        .field("name", &self.name_opt())
                        .field("insertion_point", &self.insertion_point_opt())
                        .field("content", &self.content_opt())
                        .field("generated_code_info", &self.generated_code_info())
                        .finish()
                }
            }

            impl ::std::clone::Clone for File {
                fn clone(&self) -> Self {
                    Self {
                        _bitfield: ::std::clone::Clone::clone(&self._bitfield),
                        name: ::std::clone::Clone::clone(&self.name),
                        insertion_point: ::std::clone::Clone::clone(&self.insertion_point),
                        content: ::std::clone::Clone::clone(&self.content),
                        generated_code_info: ::std::clone::Clone::clone(&self.generated_code_info),
                    }
                }
            }

            impl ::std::cmp::PartialEq for File {
                fn eq(&self, rhs: &Self) -> bool {
                    self._bitfield == rhs._bitfield
                        && (self._bitfield.get(0).as_deref() != Some(&true)
                            || self.name == rhs.name)
                        && (self._bitfield.get(1).as_deref() != Some(&true)
                            || self.insertion_point == rhs.insertion_point)
                        && (self._bitfield.get(2).as_deref() != Some(&true)
                            || self.content == rhs.content)
                        && self.generated_code_info == rhs.generated_code_info
                        && true
                }
            }
        }

        pub use _puroro_impls::*;
        pub mod _puroro_impls {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }
            use super::_puroro_traits::*;

            pub struct FileSingleField1<ScalarType>
            where
                ScalarType: ::std::convert::AsRef<str>,
            {
                pub name: ScalarType,
            }

            impl<ScalarType> ::puroro::Message<super::File> for FileSingleField1<ScalarType> where
                ScalarType: ::std::convert::AsRef<str>
            {
            }

            impl<ScalarType> super::_puroro_traits::FileTrait for FileSingleField1<ScalarType>
            where
                ScalarType: ::std::convert::AsRef<str>,
            {
                fn name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                    ::std::option::Option::Some(self.name.as_ref())
                }
                type GeneratedCodeInfoMessageType<'this>
                where
                    Self: 'this,
                = ();
            }

            impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for FileSingleField1<ScalarType>
            where
                ScalarType: ::std::convert::AsRef<str>,
                Self: super::_puroro_traits::FileTrait,
            {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    ::puroro::internal::se::SerFieldToIoWrite::<
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >::ser_field(
                        <Self as super::_puroro_traits::FileTrait>::name_opt(self),
                        1,
                        out,
                    )?;
                    ::std::result::Result::Ok(())
                }
            }

            impl<ScalarType> ::std::convert::From<ScalarType> for FileSingleField1<ScalarType>
            where
                ScalarType: ::std::convert::AsRef<str>,
            {
                fn from(value: ScalarType) -> Self {
                    Self { name: value }
                }
            }

            impl<ScalarType> ::std::clone::Clone for FileSingleField1<ScalarType>
            where
                ScalarType: ::std::convert::AsRef<str>,
                ScalarType: ::std::clone::Clone,
            {
                fn clone(&self) -> Self {
                    Self {
                        name: ::std::clone::Clone::clone(&self.name),
                    }
                }
            }

            pub struct FileSingleField2<ScalarType>
            where
                ScalarType: ::std::convert::AsRef<str>,
            {
                pub insertion_point: ScalarType,
            }

            impl<ScalarType> ::puroro::Message<super::File> for FileSingleField2<ScalarType> where
                ScalarType: ::std::convert::AsRef<str>
            {
            }

            impl<ScalarType> super::_puroro_traits::FileTrait for FileSingleField2<ScalarType>
            where
                ScalarType: ::std::convert::AsRef<str>,
            {
                fn insertion_point_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                    ::std::option::Option::Some(self.insertion_point.as_ref())
                }
                type GeneratedCodeInfoMessageType<'this>
                where
                    Self: 'this,
                = ();
            }

            impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for FileSingleField2<ScalarType>
            where
                ScalarType: ::std::convert::AsRef<str>,
                Self: super::_puroro_traits::FileTrait,
            {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    ::puroro::internal::se::SerFieldToIoWrite::<
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >::ser_field(
                        <Self as super::_puroro_traits::FileTrait>::insertion_point_opt(self),
                        2,
                        out,
                    )?;
                    ::std::result::Result::Ok(())
                }
            }

            impl<ScalarType> ::std::convert::From<ScalarType> for FileSingleField2<ScalarType>
            where
                ScalarType: ::std::convert::AsRef<str>,
            {
                fn from(value: ScalarType) -> Self {
                    Self {
                        insertion_point: value,
                    }
                }
            }

            impl<ScalarType> ::std::clone::Clone for FileSingleField2<ScalarType>
            where
                ScalarType: ::std::convert::AsRef<str>,
                ScalarType: ::std::clone::Clone,
            {
                fn clone(&self) -> Self {
                    Self {
                        insertion_point: ::std::clone::Clone::clone(&self.insertion_point),
                    }
                }
            }

            pub struct FileSingleField15<ScalarType>
            where
                ScalarType: ::std::convert::AsRef<str>,
            {
                pub content: ScalarType,
            }

            impl<ScalarType> ::puroro::Message<super::File> for FileSingleField15<ScalarType> where
                ScalarType: ::std::convert::AsRef<str>
            {
            }

            impl<ScalarType> super::_puroro_traits::FileTrait for FileSingleField15<ScalarType>
            where
                ScalarType: ::std::convert::AsRef<str>,
            {
                fn content_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                    ::std::option::Option::Some(self.content.as_ref())
                }
                type GeneratedCodeInfoMessageType<'this>
                where
                    Self: 'this,
                = ();
            }

            impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for FileSingleField15<ScalarType>
            where
                ScalarType: ::std::convert::AsRef<str>,
                Self: super::_puroro_traits::FileTrait,
            {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    ::puroro::internal::se::SerFieldToIoWrite::<
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >::ser_field(
                        <Self as super::_puroro_traits::FileTrait>::content_opt(self),
                        15,
                        out,
                    )?;
                    ::std::result::Result::Ok(())
                }
            }

            impl<ScalarType> ::std::convert::From<ScalarType> for FileSingleField15<ScalarType>
            where
                ScalarType: ::std::convert::AsRef<str>,
            {
                fn from(value: ScalarType) -> Self {
                    Self { content: value }
                }
            }

            impl<ScalarType> ::std::clone::Clone for FileSingleField15<ScalarType>
            where
                ScalarType: ::std::convert::AsRef<str>,
                ScalarType: ::std::clone::Clone,
            {
                fn clone(&self) -> Self {
                    Self {
                        content: ::std::clone::Clone::clone(&self.content),
                    }
                }
            }

            pub struct FileSingleField16<ScalarType>
            where
                ScalarType:
                    self::_puroro_root::google::protobuf::_puroro_traits::GeneratedCodeInfoTrait,
            {
                pub generated_code_info: ScalarType,
            }

            impl<ScalarType> ::puroro::Message<super::File> for FileSingleField16<ScalarType> where
                ScalarType:
                    self::_puroro_root::google::protobuf::_puroro_traits::GeneratedCodeInfoTrait
            {
            }

            impl<ScalarType> super::_puroro_traits::FileTrait for FileSingleField16<ScalarType>
            where
                ScalarType:
                    self::_puroro_root::google::protobuf::_puroro_traits::GeneratedCodeInfoTrait,
            {
                type GeneratedCodeInfoMessageType<'this>
                where
                    Self: 'this,
                = &'this ScalarType;

                fn generated_code_info_opt<'this>(
                    &'this self,
                ) -> ::std::option::Option<Self::GeneratedCodeInfoMessageType<'this>>
                {
                    ::std::option::Option::Some(&self.generated_code_info)
                }
            }

            impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for FileSingleField16<ScalarType>
            where
                ScalarType:
                    self::_puroro_root::google::protobuf::_puroro_traits::GeneratedCodeInfoTrait,
                Self: super::_puroro_traits::FileTrait,
                for<'a> <Self as super::_puroro_traits::FileTrait>::GeneratedCodeInfoMessageType<'a>:
                    ::puroro::internal::se::SerMessageToIoWrite,
            {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    ::puroro::internal::se::SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::Message<<Self as super::_puroro_traits::FileTrait>::GeneratedCodeInfoMessageType<'_>>
                >::ser_field(
                    <Self as super::_puroro_traits::FileTrait>::generated_code_info_opt(self),
                    16,
                    out
                )?;
                    ::std::result::Result::Ok(())
                }
            }

            impl<ScalarType> ::std::convert::From<ScalarType> for FileSingleField16<ScalarType>
            where
                ScalarType:
                    self::_puroro_root::google::protobuf::_puroro_traits::GeneratedCodeInfoTrait,
            {
                fn from(value: ScalarType) -> Self {
                    Self {
                        generated_code_info: value,
                    }
                }
            }

            impl<ScalarType> ::std::clone::Clone for FileSingleField16<ScalarType>
            where
                ScalarType:
                    self::_puroro_root::google::protobuf::_puroro_traits::GeneratedCodeInfoTrait,
                ScalarType: ::std::clone::Clone,
            {
                fn clone(&self) -> Self {
                    Self {
                        generated_code_info: ::std::clone::Clone::clone(&self.generated_code_info),
                    }
                }
            }
            pub struct FileBumpalo<'bump> {
            _bump: &'bump ::puroro::bumpalo::Bump,
            _bitfield: ::puroro::bitvec::array::BitArray<
                ::puroro::bitvec::order::Lsb0,
                [u32; (3 + 31) / 32],
            >,
            name: ::puroro::internal::Bare<::puroro::internal::NoAllocBumpString>,
            insertion_point: ::puroro::internal::Bare<::puroro::internal::NoAllocBumpString>,
            content: ::puroro::internal::Bare<::puroro::internal::NoAllocBumpString>,
            generated_code_info: ::std::option::Option<::puroro::internal::NoAllocBumpBox<self::_puroro_root::google::protobuf::_puroro_impls::GeneratedCodeInfoBumpalo<'bump>>>,
        }

            pub type FileBumpaloOwned = ::puroro::BumpaloOwned<FileBumpalo<'static>>;
            impl<'bump> FileBumpalo<'bump> {
                pub fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
                    #[allow(unused)]
                    let bump_ref: &::puroro::bumpalo::Bump =
                        unsafe { ::std::mem::transmute(::std::ops::Deref::deref(&bump)) };

                    Self {
                        _bump: bump,
                        _bitfield: ::std::default::Default::default(),
                        name: ::std::default::Default::default(),
                        insertion_point: ::std::default::Default::default(),
                        content: ::std::default::Default::default(),
                        generated_code_info: ::std::default::Default::default(),
                    }
                }
                pub fn name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                    if self._bitfield.get(0).map_or(false, |v| *v) {
                        ::std::option::Option::Some(&self.name)
                    } else {
                        ::std::option::Option::None
                    }
                }
                pub fn name<'this>(&'this self) -> &'this str {
                    match self.name_opt() {
                        ::std::option::Option::Some(x) => x,
                        _ => ::std::default::Default::default(),
                    }
                }

                pub fn has_name(&self) -> bool {
                    self.name_opt().is_some()
                }
                pub fn insertion_point_opt<'this>(
                    &'this self,
                ) -> ::std::option::Option<&'this str> {
                    if self._bitfield.get(1).map_or(false, |v| *v) {
                        ::std::option::Option::Some(&self.insertion_point)
                    } else {
                        ::std::option::Option::None
                    }
                }
                pub fn insertion_point<'this>(&'this self) -> &'this str {
                    match self.insertion_point_opt() {
                        ::std::option::Option::Some(x) => x,
                        _ => ::std::default::Default::default(),
                    }
                }

                pub fn has_insertion_point(&self) -> bool {
                    self.insertion_point_opt().is_some()
                }
                pub fn content_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                    if self._bitfield.get(2).map_or(false, |v| *v) {
                        ::std::option::Option::Some(&self.content)
                    } else {
                        ::std::option::Option::None
                    }
                }
                pub fn content<'this>(&'this self) -> &'this str {
                    match self.content_opt() {
                        ::std::option::Option::Some(x) => x,
                        _ => ::std::default::Default::default(),
                    }
                }

                pub fn has_content(&self) -> bool {
                    self.content_opt().is_some()
                }
                pub fn generated_code_info_opt<'this>(&'this self) -> ::std::option::Option<&'this self::_puroro_root::google::protobuf::_puroro_impls::GeneratedCodeInfoBumpalo<'this>>{
                    self.generated_code_info
                        .as_ref()
                        .map(|x| unsafe { ::std::mem::transmute(::std::ops::Deref::deref(x)) })
                }
                pub fn generated_code_info<'this>(&'this self) -> ::std::option::Option<&'this self::_puroro_root::google::protobuf::_puroro_impls::GeneratedCodeInfoBumpalo<'this>>{
                    self.generated_code_info_opt()
                }

                pub fn has_generated_code_info(&self) -> bool {
                    self.generated_code_info_opt().is_some()
                }
                pub fn clear_name(&mut self) {
                    self._bitfield.set(0, false);
                }
                pub fn name_mut<'this>(
                    &'this mut self,
                ) -> impl 'this
                + ::std::ops::DerefMut<
                    Target = ::puroro::bumpalo::collections::String<'bump>,
                > {
                    if !self.has_name() {
                        self.name = ::std::default::Default::default();
                        self._bitfield.set(0, true);
                    }
                    unsafe { self.name.as_mut_string_in(self._bump) }
                }
                pub fn clear_insertion_point(&mut self) {
                    self._bitfield.set(1, false);
                }
                pub fn insertion_point_mut<'this>(
                    &'this mut self,
                ) -> impl 'this
                + ::std::ops::DerefMut<
                    Target = ::puroro::bumpalo::collections::String<'bump>,
                > {
                    if !self.has_insertion_point() {
                        self.insertion_point = ::std::default::Default::default();
                        self._bitfield.set(1, true);
                    }
                    unsafe { self.insertion_point.as_mut_string_in(self._bump) }
                }
                pub fn clear_content(&mut self) {
                    self._bitfield.set(2, false);
                }
                pub fn content_mut<'this>(
                    &'this mut self,
                ) -> impl 'this
                + ::std::ops::DerefMut<
                    Target = ::puroro::bumpalo::collections::String<'bump>,
                > {
                    if !self.has_content() {
                        self.content = ::std::default::Default::default();
                        self._bitfield.set(2, true);
                    }
                    unsafe { self.content.as_mut_string_in(self._bump) }
                }
                pub fn clear_generated_code_info(&mut self) {
                    self.generated_code_info = ::std::default::Default::default();
                }
                pub fn generated_code_info_mut<'this>(&'this mut self) -> &'this mut self::_puroro_root::google::protobuf::_puroro_impls::GeneratedCodeInfoBumpalo<'bump>{
                    if !self.has_generated_code_info() {
                        self.generated_code_info = ::std::default::Default::default();
                    }
                    let bump = self._bump;
                    self.generated_code_info.get_or_insert_with(|| {
                        ::puroro::internal::NoAllocBumpBox::new_in(
                            ::puroro::internal::BumpDefault::default_in(bump),
                            bump,
                        )
                    })
                }
            }
            impl<'bump> ::puroro::Message<super::_puroro_simple_impl::File> for FileBumpalo<'bump> {}

            impl<'bump> ::puroro::BumpaloMessage<'bump> for FileBumpalo<'bump> {
                fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
                    Self::new_in(bump)
                }
            }

            impl<'bump> ::puroro::internal::BumpDefault<'bump> for FileBumpalo<'bump> {
                fn default_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
                    Self::new_in(bump)
                }
            }

            impl<'bump> super::_puroro_traits::FileTrait for FileBumpalo<'bump> {
                fn name_opt<'this>(&'this self) -> Option<&'this str> {
                    <Self>::name_opt(self)
                }
                fn insertion_point_opt<'this>(&'this self) -> Option<&'this str> {
                    <Self>::insertion_point_opt(self)
                }
                fn content_opt<'this>(&'this self) -> Option<&'this str> {
                    <Self>::content_opt(self)
                }
                type GeneratedCodeInfoMessageType<'this> where Self: 'this = &'this self::_puroro_root::google::protobuf::_puroro_impls::GeneratedCodeInfoBumpalo<'this>;
                fn generated_code_info_opt<'this>(
                    &'this self,
                ) -> Option<Self::GeneratedCodeInfoMessageType<'this>> {
                    <Self>::generated_code_info_opt(self)
                }
            }

            impl<'bump> ::puroro::internal::de::DeserMessageFromBytesIter for FileBumpalo<'bump> {
                fn deser_field<'this, I>(
                    &'this mut self,
                    field_number: i32,
                    data: ::puroro::internal::types::FieldData<
                        &mut ::puroro::internal::de::from_iter::ScopedIter<I>,
                    >,
                ) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    use ::puroro::internal::impls::bumpalo::de::DeserFieldFromBytesIter;
                    match field_number {
                    1 => {
                        self._bitfield.set(0, true);
                        DeserFieldFromBytesIter::<
                            ::puroro::tags::Optional, ::puroro::tags::String
                        >::deser_field(&mut self.name, data, self._bump)
                    }
                    2 => {
                        self._bitfield.set(1, true);
                        DeserFieldFromBytesIter::<
                            ::puroro::tags::Optional, ::puroro::tags::String
                        >::deser_field(&mut self.insertion_point, data, self._bump)
                    }
                    15 => {
                        self._bitfield.set(2, true);
                        DeserFieldFromBytesIter::<
                            ::puroro::tags::Optional, ::puroro::tags::String
                        >::deser_field(&mut self.content, data, self._bump)
                    }
                    16 => {
                        DeserFieldFromBytesIter::<
                            ::puroro::tags::Optional, ::puroro::tags::Message<::puroro::internal::NoAllocBumpBox<self::_puroro_root::google::protobuf::_puroro_impls::GeneratedCodeInfoBumpalo<'bump>>>
                        >::deser_field(&mut self.generated_code_info, data, self._bump)
                    }
        
                    _ => unimplemented!("TODO: This case should be handled properly..."),
                }
                }
            }

            impl<'bump> ::puroro::internal::se::SerMessageToIoWrite for FileBumpalo<'bump>
            where
                Self: super::_puroro_traits::FileTrait,
                for<'a> <Self as super::_puroro_traits::FileTrait>::GeneratedCodeInfoMessageType<'a>:
                    ::puroro::internal::se::SerMessageToIoWrite,
            {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    ::puroro::internal::se::SerFieldToIoWrite::<
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >::ser_field(
                        <Self as super::_puroro_traits::FileTrait>::name_opt(self),
                        1,
                        out,
                    )?;
                    ::puroro::internal::se::SerFieldToIoWrite::<
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >::ser_field(
                        <Self as super::_puroro_traits::FileTrait>::insertion_point_opt(self),
                        2,
                        out,
                    )?;
                    ::puroro::internal::se::SerFieldToIoWrite::<
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >::ser_field(
                        <Self as super::_puroro_traits::FileTrait>::content_opt(self),
                        15,
                        out,
                    )?;
                    ::puroro::internal::se::SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::Message<<Self as super::_puroro_traits::FileTrait>::GeneratedCodeInfoMessageType<'_>>
                >::ser_field(
                    <Self as super::_puroro_traits::FileTrait>::generated_code_info_opt(self),
                    16,
                    out
                )?;
                    ::std::result::Result::Ok(())
                }
            }
            pub struct FileBuilder<T>(T);

            impl<T> FileBuilder<T>
            where
                T: FileTrait,
            {
                pub fn append_name<ScalarType>(
                    self,
                    value: ScalarType,
                ) -> FileBuilder<(T, FileSingleField1<ScalarType>)>
                where
                    ScalarType: ::std::convert::AsRef<str>,
                {
                    FileBuilder((self.0, FileSingleField1 { name: value }))
                }

                pub fn append_insertion_point<ScalarType>(
                    self,
                    value: ScalarType,
                ) -> FileBuilder<(T, FileSingleField2<ScalarType>)>
                where
                    ScalarType: ::std::convert::AsRef<str>,
                {
                    FileBuilder((
                        self.0,
                        FileSingleField2 {
                            insertion_point: value,
                        },
                    ))
                }

                pub fn append_content<ScalarType>(
                    self,
                    value: ScalarType,
                ) -> FileBuilder<(T, FileSingleField15<ScalarType>)>
                where
                    ScalarType: ::std::convert::AsRef<str>,
                {
                    FileBuilder((self.0, FileSingleField15 { content: value }))
                }
        
            pub fn append_generated_code_info<ScalarType>(self, value: ScalarType)
                -> FileBuilder<(T, FileSingleField16<ScalarType>)>
        where
        
        ScalarType:
            self::_puroro_root::google::protobuf::_puroro_traits::GeneratedCodeInfoTrait,
            {
                    FileBuilder((
                        self.0,
                        FileSingleField16 {
                            generated_code_info: value,
                        },
                    ))
                }

                pub fn build(self) -> T {
                    self.0
                }
            }

            impl FileBuilder<()> {
                pub fn new() -> Self {
                    Self(())
                }
            }
        }
        pub use _puroro_traits::*;
        pub mod _puroro_traits {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }

            pub trait FileTrait {
                fn name<'this>(&'this self) -> &'this str {
                    self.name_opt()
                        .unwrap_or_else(::std::default::Default::default)
                }
                fn has_name<'this>(&'this self) -> bool {
                    self.name_opt().is_some()
                }
                fn name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                    ::std::option::Option::None
                }
                fn insertion_point<'this>(&'this self) -> &'this str {
                    self.insertion_point_opt()
                        .unwrap_or_else(::std::default::Default::default)
                }
                fn has_insertion_point<'this>(&'this self) -> bool {
                    self.insertion_point_opt().is_some()
                }
                fn insertion_point_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                    ::std::option::Option::None
                }
                fn content<'this>(&'this self) -> &'this str {
                    self.content_opt()
                        .unwrap_or_else(::std::default::Default::default)
                }
                fn has_content<'this>(&'this self) -> bool {
                    self.content_opt().is_some()
                }
                fn content_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                    ::std::option::Option::None
                }
                type GeneratedCodeInfoMessageType<'this>: self::_puroro_root::google::protobuf::_puroro_traits::GeneratedCodeInfoTrait
                    where Self: 'this;
                fn generated_code_info<'this>(
                    &'this self,
                ) -> ::std::option::Option<Self::GeneratedCodeInfoMessageType<'this>>
                {
                    self.generated_code_info_opt()
                }
                fn has_generated_code_info<'this>(&'this self) -> bool {
                    self.generated_code_info_opt().is_some()
                }
                fn generated_code_info_opt<'this>(
                    &'this self,
                ) -> ::std::option::Option<Self::GeneratedCodeInfoMessageType<'this>>
                {
                    ::std::option::Option::None
                }
            }

            macro_rules! file_delegate {
                ($ty:ty) => {
                    fn name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                        (**self).name_opt()
                    }
                    fn insertion_point_opt<'this>(
                        &'this self,
                    ) -> ::std::option::Option<&'this str> {
                        (**self).insertion_point_opt()
                    }
                    fn content_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                        (**self).content_opt()
                    }
                    type GeneratedCodeInfoMessageType<'this>
                    where
                        Self: 'this,
                    = <$ty>::GeneratedCodeInfoMessageType<'this>;
                    fn generated_code_info_opt<'this>(
                        &'this self,
                    ) -> ::std::option::Option<Self::GeneratedCodeInfoMessageType<'this>> {
                        (**self).generated_code_info_opt()
                    }
                };
            }

            impl<T> FileTrait for &'_ T
            where
                T: FileTrait,
            {
                file_delegate!(T);
            }

            impl<T> FileTrait for &'_ mut T
            where
                T: FileTrait,
            {
                file_delegate!(T);
            }

            impl<T> FileTrait for ::std::boxed::Box<T>
            where
                T: FileTrait,
            {
                file_delegate!(T);
            }

            impl<'bump, T> FileTrait for ::puroro::bumpalo::boxed::Box<'bump, T>
            where
                T: FileTrait,
            {
                file_delegate!(T);
            }

            impl<T> FileTrait for ::puroro::BumpaloOwned<T>
            where
                T: FileTrait,
            {
                file_delegate!(T);
            }
            impl FileTrait for () {
                type GeneratedCodeInfoMessageType<'this>
                where
                    Self: 'this,
                = ();
            }
            impl<T, U> FileTrait for (T, U)
            where
                T: FileTrait,
                U: FileTrait,
            {
                fn name_opt<'this>(&'this self) -> Option<&'this str> {
                    <U as FileTrait>::name_opt(&self.1)
                        .or_else(|| <T as FileTrait>::name_opt(&self.0))
                }
                fn insertion_point_opt<'this>(&'this self) -> Option<&'this str> {
                    <U as FileTrait>::insertion_point_opt(&self.1)
                        .or_else(|| <T as FileTrait>::insertion_point_opt(&self.0))
                }
                fn content_opt<'this>(&'this self) -> Option<&'this str> {
                    <U as FileTrait>::content_opt(&self.1)
                        .or_else(|| <T as FileTrait>::content_opt(&self.0))
                }
                type GeneratedCodeInfoMessageType<'this>
                where
                    Self: 'this,
                = (
                    ::std::option::Option<<T as FileTrait>::GeneratedCodeInfoMessageType<'this>>,
                    ::std::option::Option<<U as FileTrait>::GeneratedCodeInfoMessageType<'this>>,
                );
                fn generated_code_info_opt<'this>(
                    &'this self,
                ) -> Option<Self::GeneratedCodeInfoMessageType<'this>> {
                    match (
                        <T as FileTrait>::generated_code_info_opt(&self.0),
                        <U as FileTrait>::generated_code_info_opt(&self.1),
                    ) {
                        (None, None) => None,
                        (Some(t), None) => Some((Some(t), None)),
                        (None, Some(u)) => Some((None, Some(u))),
                        (Some(t), Some(u)) => Some((Some(t), Some(u))),
                    }
                }
            }
            impl<T, U> FileTrait for ::puroro::Either<T, U>
            where
                T: FileTrait,
                U: FileTrait,
            {
                fn name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                    self.as_ref().either(
                        |t| <T as FileTrait>::name_opt(t),
                        |u| <U as FileTrait>::name_opt(u),
                    )
                }
                fn insertion_point_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                    self.as_ref().either(
                        |t| <T as FileTrait>::insertion_point_opt(t),
                        |u| <U as FileTrait>::insertion_point_opt(u),
                    )
                }
                fn content_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                    self.as_ref().either(
                        |t| <T as FileTrait>::content_opt(t),
                        |u| <U as FileTrait>::content_opt(u),
                    )
                }
                type GeneratedCodeInfoMessageType<'this>
                where
                    Self: 'this,
                = ::puroro::Either<
                    <T as FileTrait>::GeneratedCodeInfoMessageType<'this>,
                    <U as FileTrait>::GeneratedCodeInfoMessageType<'this>,
                >;
                fn generated_code_info_opt<'this>(
                    &'this self,
                ) -> ::std::option::Option<Self::GeneratedCodeInfoMessageType<'this>>
                {
                    self.as_ref().either(
                        |t| {
                            <T as FileTrait>::generated_code_info_opt(t)
                                .map(|t| ::puroro::Either::Left(t))
                        },
                        |u| {
                            <U as FileTrait>::generated_code_info_opt(u)
                                .map(|u| ::puroro::Either::Right(u))
                        },
                    )
                }
            }
            impl<T> FileTrait for ::std::option::Option<T>
            where
                T: FileTrait,
            {
                fn name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                    self.as_ref().and_then(|msg| msg.name_opt())
                }
                fn insertion_point_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                    self.as_ref().and_then(|msg| msg.insertion_point_opt())
                }
                fn content_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                    self.as_ref().and_then(|msg| msg.content_opt())
                }
                type GeneratedCodeInfoMessageType<'this>
                where
                    Self: 'this,
                = T::GeneratedCodeInfoMessageType<'this>;
                fn generated_code_info_opt<'this>(
                    &'this self,
                ) -> ::std::option::Option<Self::GeneratedCodeInfoMessageType<'this>>
                {
                    self.as_ref().and_then(|msg| msg.generated_code_info_opt())
                }
            }
        }
        #[derive(
            ::std::fmt::Debug, ::std::clone::Clone, ::std::marker::Copy, ::std::cmp::PartialEq,
        )]
        pub enum Feature {
            FeatureNone,
            FeatureProto3Optional,
        }

        impl ::puroro::Enum2 for Feature {}
        impl ::std::convert::TryFrom<i32> for Feature {
            type Error = i32;
            fn try_from(value: i32) -> ::std::result::Result<Self, i32> {
                ::std::result::Result::Ok(match value {
                    0 => Feature::FeatureNone,
                    1 => Feature::FeatureProto3Optional,
                    _ => Err(value)?,
                })
            }
        }

        impl ::std::convert::From<Feature> for i32 {
            fn from(value: Feature) -> i32 {
                match value {
                    Feature::FeatureNone => 0,
                    Feature::FeatureProto3Optional => 1,
                }
            }
        }

        impl ::std::default::Default for Feature {
            fn default() -> Self {
                Feature::FeatureNone
            }
        }

        impl<'bump> ::puroro::internal::BumpDefault<'bump> for Feature {
            fn default_in(_: &'bump ::puroro::bumpalo::Bump) -> Self {
                ::std::default::Default::default()
            }
        }
        pub use _puroro_nested::*;
        pub mod _puroro_nested {
            pub mod file {
                mod _puroro_root {
                    pub use super::super::super::_puroro_root::*;
                }
            }
        }
    }
}
