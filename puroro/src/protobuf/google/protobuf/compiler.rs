mod _root {
    #[allow(unused)]
    pub(crate) use super::super::_root::*;
}
mod _puroro {
    #[allow(unused)]
    pub(crate) use super::_root::_puroro::*;
}
mod _pinternal {
    #[allow(unused)]
    pub(crate) use super::_root::_pinternal::*;
}
pub mod code_generator_response;
#[derive(::std::default::Default)]
/** The version number of protocol compiler.
*/
pub struct Version {
    body: self::_root::google::protobuf::compiler::_view::VersionView,
}
impl Version {
    pub fn major_mut(&mut self) -> &mut i32 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.body.fields.major,
            self.body.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_major(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(&mut self.body.fields.major, self.body.shared.bitfield_mut())
    }
    pub fn minor_mut(&mut self) -> &mut i32 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.body.fields.minor,
            self.body.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_minor(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(&mut self.body.fields.minor, self.body.shared.bitfield_mut())
    }
    pub fn patch_mut(&mut self) -> &mut i32 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.body.fields.patch,
            self.body.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_patch(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(&mut self.body.fields.patch, self.body.shared.bitfield_mut())
    }
    pub fn suffix_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.body.fields.suffix,
            self.body.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_suffix(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(
            &mut self.body.fields.suffix,
            self.body.shared.bitfield_mut(),
        )
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
        iter: I,
    ) -> self::_puroro::Result<()> {
        let mut pos_iter = self::_pinternal::PosIter::new(iter);
        let mut scoped_iter = self::_pinternal::ScopedIter::from_mut_pos_iter(&mut pos_iter);
        <Self as self::_pinternal::MessageInternal>::merge_from_scoped_bytes_iter(
            self,
            &mut scoped_iter,
        )?;
        scoped_iter.drop_and_check_scope_completed()?;
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)] out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        self::_pinternal::FieldType::ser_to_write(
            &self.body.fields.major,
            self.body.shared.bitfield(),
            1i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.body.fields.minor,
            self.body.shared.bitfield(),
            2i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.body.fields.patch,
            self.body.shared.bitfield(),
            3i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.body.fields.suffix,
            self.body.shared.bitfield(),
            4i32,
            out,
        )?;
        self.shared.unknown_fields().ser_to_write(out)?;
        ::std::result::Result::Ok(())
    }
}
impl self::_pinternal::MessageInternal for Version {
    fn merge_from_scoped_bytes_iter<'a, I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>>(
        &mut self,
        iter: &mut self::_pinternal::ScopedIter<'a, I>,
    ) -> self::_puroro::Result<()> {
        use self::_pinternal::ser::FieldData;
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        use self::_puroro::PuroroError;
        #[allow(unused)]
        use ::std::result::Result;
        #[allow(unused)]
        use ::std::result::Result::{Err, Ok};
        #[allow(unused)]
        use ::std::vec::Vec;
        while let Some((number, field_data)) = FieldData::from_bytes_scoped_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    1i32 => self::_pinternal::FieldType::deser_from_field_data(
                        &mut self.body.fields.major,
                        self.body.shared.bitfield_mut(),
                        field_data,
                    )?,
                    2i32 => self::_pinternal::FieldType::deser_from_field_data(
                        &mut self.body.fields.minor,
                        self.body.shared.bitfield_mut(),
                        field_data,
                    )?,
                    3i32 => self::_pinternal::FieldType::deser_from_field_data(
                        &mut self.body.fields.patch,
                        self.body.shared.bitfield_mut(),
                        field_data,
                    )?,
                    4i32 => self::_pinternal::FieldType::deser_from_field_data(
                        &mut self.body.fields.suffix,
                        self.body.shared.bitfield_mut(),
                        field_data,
                    )?,
                    _ => {
                        let field_data = field_data
                            .map(|iter| iter.collect::<Result<Vec<_>, _>>())
                            .transpose()?;
                        Err(PuroroError::UnknownFieldNumber(field_data))?
                    }
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(PuroroError::UnknownFieldNumber(field_data)) => {
                    self.body
                        .shared
                        .unknown_fields_mut()
                        .push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
}
impl ::std::borrow::Borrow<self::_root::google::protobuf::compiler::_view::VersionView>
    for Version
{
    fn borrow(&self) -> &self::_root::google::protobuf::compiler::_view::VersionView {
        &self.body
    }
}
impl ::std::clone::Clone for Version {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use ::std::borrow::ToOwned;
        ToOwned::to_owned(&self.body)
    }
}
impl ::std::fmt::Debug for Version {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        <self::_root::google::protobuf::compiler::_view::VersionView as ::std::fmt::Debug>::fmt(
            &self.body, fmt,
        )
    }
}
impl ::std::ops::Deref for Version {
    type Target = self::_root::google::protobuf::compiler::_view::VersionView;
    fn deref(&self) -> &Self::Target {
        &self.body
    }
}
impl ::std::cmp::PartialEq for Version {
    fn eq(&self, rhs: &Self) -> bool {
        &self.body == &rhs.body
    }
}
#[derive(::std::default::Default)]
/** An encoded CodeGeneratorRequest is written to the plugin's stdin.
*/
pub struct CodeGeneratorRequest {
    body: self::_root::google::protobuf::compiler::_view::CodeGeneratorRequestView,
}
impl CodeGeneratorRequest {
    pub fn file_to_generate_mut(&mut self) -> &mut ::std::vec::Vec<::std::string::String> {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.body.fields.file_to_generate,
            self.body.shared.bitfield_mut(),
        )
    }
    pub fn clear_file_to_generate(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(
            &mut self.body.fields.file_to_generate,
            self.body.shared.bitfield_mut(),
        )
    }
    pub fn parameter_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.body.fields.parameter,
            self.body.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_parameter(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(
            &mut self.body.fields.parameter,
            self.body.shared.bitfield_mut(),
        )
    }
    pub fn proto_file_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec<self::_root::google::protobuf::FileDescriptorProto> {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.body.fields.proto_file,
            self.body.shared.bitfield_mut(),
        )
    }
    pub fn clear_proto_file(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(
            &mut self.body.fields.proto_file,
            self.body.shared.bitfield_mut(),
        )
    }
    pub fn compiler_version_mut(
        &mut self,
    ) -> &mut self::_root::google::protobuf::compiler::Version {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.body.fields.compiler_version,
            self.body.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_compiler_version(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(
            &mut self.body.fields.compiler_version,
            self.body.shared.bitfield_mut(),
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
        iter: I,
    ) -> self::_puroro::Result<()> {
        let mut pos_iter = self::_pinternal::PosIter::new(iter);
        let mut scoped_iter = self::_pinternal::ScopedIter::from_mut_pos_iter(&mut pos_iter);
        <Self as self::_pinternal::MessageInternal>::merge_from_scoped_bytes_iter(
            self,
            &mut scoped_iter,
        )?;
        scoped_iter.drop_and_check_scope_completed()?;
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)] out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        self::_pinternal::FieldType::ser_to_write(
            &self.body.fields.file_to_generate,
            self.body.shared.bitfield(),
            1i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.body.fields.parameter,
            self.body.shared.bitfield(),
            2i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.body.fields.proto_file,
            self.body.shared.bitfield(),
            15i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.body.fields.compiler_version,
            self.body.shared.bitfield(),
            3i32,
            out,
        )?;
        self.shared.unknown_fields().ser_to_write(out)?;
        ::std::result::Result::Ok(())
    }
}
impl self::_pinternal::MessageInternal for CodeGeneratorRequest {
    fn merge_from_scoped_bytes_iter<'a, I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>>(
        &mut self,
        iter: &mut self::_pinternal::ScopedIter<'a, I>,
    ) -> self::_puroro::Result<()> {
        use self::_pinternal::ser::FieldData;
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        use self::_puroro::PuroroError;
        #[allow(unused)]
        use ::std::result::Result;
        #[allow(unused)]
        use ::std::result::Result::{Err, Ok};
        #[allow(unused)]
        use ::std::vec::Vec;
        while let Some((number, field_data)) = FieldData::from_bytes_scoped_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    1i32 => self::_pinternal::FieldType::deser_from_field_data(
                        &mut self.body.fields.file_to_generate,
                        self.body.shared.bitfield_mut(),
                        field_data,
                    )?,
                    2i32 => self::_pinternal::FieldType::deser_from_field_data(
                        &mut self.body.fields.parameter,
                        self.body.shared.bitfield_mut(),
                        field_data,
                    )?,
                    15i32 => self::_pinternal::FieldType::deser_from_field_data(
                        &mut self.body.fields.proto_file,
                        self.body.shared.bitfield_mut(),
                        field_data,
                    )?,
                    3i32 => self::_pinternal::FieldType::deser_from_field_data(
                        &mut self.body.fields.compiler_version,
                        self.body.shared.bitfield_mut(),
                        field_data,
                    )?,
                    _ => {
                        let field_data = field_data
                            .map(|iter| iter.collect::<Result<Vec<_>, _>>())
                            .transpose()?;
                        Err(PuroroError::UnknownFieldNumber(field_data))?
                    }
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(PuroroError::UnknownFieldNumber(field_data)) => {
                    self.body
                        .shared
                        .unknown_fields_mut()
                        .push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
}
impl ::std::borrow::Borrow<self::_root::google::protobuf::compiler::_view::CodeGeneratorRequestView>
    for CodeGeneratorRequest
{
    fn borrow(&self) -> &self::_root::google::protobuf::compiler::_view::CodeGeneratorRequestView {
        &self.body
    }
}
impl ::std::clone::Clone for CodeGeneratorRequest {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use ::std::borrow::ToOwned;
        ToOwned::to_owned(&self.body)
    }
}
impl ::std::fmt::Debug for CodeGeneratorRequest {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        <self::_root::google::protobuf::compiler::_view::CodeGeneratorRequestView as ::std::fmt::Debug>::fmt(
            &self.body,
            fmt,
        )
    }
}
impl ::std::ops::Deref for CodeGeneratorRequest {
    type Target = self::_root::google::protobuf::compiler::_view::CodeGeneratorRequestView;
    fn deref(&self) -> &Self::Target {
        &self.body
    }
}
impl ::std::cmp::PartialEq for CodeGeneratorRequest {
    fn eq(&self, rhs: &Self) -> bool {
        &self.body == &rhs.body
    }
}
#[derive(::std::default::Default)]
/** The plugin writes an encoded CodeGeneratorResponse to stdout.
*/
pub struct CodeGeneratorResponse {
    body: self::_root::google::protobuf::compiler::_view::CodeGeneratorResponseView,
}
impl CodeGeneratorResponse {
    pub fn error_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.body.fields.error,
            self.body.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_error(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(&mut self.body.fields.error, self.body.shared.bitfield_mut())
    }
    pub fn supported_features_mut(&mut self) -> &mut u64 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.body.fields.supported_features,
            self.body.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_supported_features(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(
            &mut self.body.fields.supported_features,
            self.body.shared.bitfield_mut(),
        )
    }
    pub fn file_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec<self::_root::google::protobuf::compiler::code_generator_response::File>
    {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.body.fields.file,
            self.body.shared.bitfield_mut(),
        )
    }
    pub fn clear_file(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(&mut self.body.fields.file, self.body.shared.bitfield_mut())
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
        iter: I,
    ) -> self::_puroro::Result<()> {
        let mut pos_iter = self::_pinternal::PosIter::new(iter);
        let mut scoped_iter = self::_pinternal::ScopedIter::from_mut_pos_iter(&mut pos_iter);
        <Self as self::_pinternal::MessageInternal>::merge_from_scoped_bytes_iter(
            self,
            &mut scoped_iter,
        )?;
        scoped_iter.drop_and_check_scope_completed()?;
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)] out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        self::_pinternal::FieldType::ser_to_write(
            &self.body.fields.error,
            self.body.shared.bitfield(),
            1i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.body.fields.supported_features,
            self.body.shared.bitfield(),
            2i32,
            out,
        )?;
        self::_pinternal::FieldType::ser_to_write(
            &self.body.fields.file,
            self.body.shared.bitfield(),
            15i32,
            out,
        )?;
        self.shared.unknown_fields().ser_to_write(out)?;
        ::std::result::Result::Ok(())
    }
}
impl self::_pinternal::MessageInternal for CodeGeneratorResponse {
    fn merge_from_scoped_bytes_iter<'a, I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>>(
        &mut self,
        iter: &mut self::_pinternal::ScopedIter<'a, I>,
    ) -> self::_puroro::Result<()> {
        use self::_pinternal::ser::FieldData;
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        use self::_puroro::PuroroError;
        #[allow(unused)]
        use ::std::result::Result;
        #[allow(unused)]
        use ::std::result::Result::{Err, Ok};
        #[allow(unused)]
        use ::std::vec::Vec;
        while let Some((number, field_data)) = FieldData::from_bytes_scoped_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    1i32 => self::_pinternal::FieldType::deser_from_field_data(
                        &mut self.body.fields.error,
                        self.body.shared.bitfield_mut(),
                        field_data,
                    )?,
                    2i32 => self::_pinternal::FieldType::deser_from_field_data(
                        &mut self.body.fields.supported_features,
                        self.body.shared.bitfield_mut(),
                        field_data,
                    )?,
                    15i32 => self::_pinternal::FieldType::deser_from_field_data(
                        &mut self.body.fields.file,
                        self.body.shared.bitfield_mut(),
                        field_data,
                    )?,
                    _ => {
                        let field_data = field_data
                            .map(|iter| iter.collect::<Result<Vec<_>, _>>())
                            .transpose()?;
                        Err(PuroroError::UnknownFieldNumber(field_data))?
                    }
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(PuroroError::UnknownFieldNumber(field_data)) => {
                    self.body
                        .shared
                        .unknown_fields_mut()
                        .push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
}
impl
    ::std::borrow::Borrow<self::_root::google::protobuf::compiler::_view::CodeGeneratorResponseView>
    for CodeGeneratorResponse
{
    fn borrow(&self) -> &self::_root::google::protobuf::compiler::_view::CodeGeneratorResponseView {
        &self.body
    }
}
impl ::std::clone::Clone for CodeGeneratorResponse {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use ::std::borrow::ToOwned;
        ToOwned::to_owned(&self.body)
    }
}
impl ::std::fmt::Debug for CodeGeneratorResponse {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        <self::_root::google::protobuf::compiler::_view::CodeGeneratorResponseView as ::std::fmt::Debug>::fmt(
            &self.body,
            fmt,
        )
    }
}
impl ::std::ops::Deref for CodeGeneratorResponse {
    type Target = self::_root::google::protobuf::compiler::_view::CodeGeneratorResponseView;
    fn deref(&self) -> &Self::Target {
        &self.body
    }
}
impl ::std::cmp::PartialEq for CodeGeneratorResponse {
    fn eq(&self, rhs: &Self) -> bool {
        &self.body == &rhs.body
    }
}
#[doc(hidden)]
pub mod _view {
    mod _root {
        #[allow(unused)]
        pub(crate) use super::super::_root::*;
    }
    mod _puroro {
        #[allow(unused)]
        pub(crate) use super::_root::_puroro::*;
    }
    mod _pinternal {
        #[allow(unused)]
        pub(crate) use super::_root::_pinternal::*;
    }
    #[derive(::std::default::Default)]
    pub struct VersionView {
        pub(super) fields: self::_root::google::protobuf::compiler::_fields::VersionFields<
            self::_pinternal::OptionalNumericalField<i32, self::_pinternal::tags::Int32, 0usize>,
            self::_pinternal::OptionalNumericalField<i32, self::_pinternal::tags::Int32, 1usize>,
            self::_pinternal::OptionalNumericalField<i32, self::_pinternal::tags::Int32, 2usize>,
            self::_pinternal::OptionalUnsizedField<
                ::std::string::String,
                self::_pinternal::tags::String,
                3usize,
            >,
        >,
        pub(super) shared: self::_pinternal::SharedItemsImpl<1usize>,
    }
    impl VersionView {
        pub fn major(&self) -> i32 {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.major,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn major_opt(&self) -> ::std::option::Option<i32> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(&self.fields.major, self.shared.bitfield())
        }
        pub fn has_major(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(&self.fields.major, self.shared.bitfield())
                .is_some()
        }
        pub fn minor(&self) -> i32 {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.minor,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn minor_opt(&self) -> ::std::option::Option<i32> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(&self.fields.minor, self.shared.bitfield())
        }
        pub fn has_minor(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(&self.fields.minor, self.shared.bitfield())
                .is_some()
        }
        pub fn patch(&self) -> i32 {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.patch,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn patch_opt(&self) -> ::std::option::Option<i32> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(&self.fields.patch, self.shared.bitfield())
        }
        pub fn has_patch(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(&self.fields.patch, self.shared.bitfield())
                .is_some()
        }
        pub fn suffix(&self) -> &str {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.suffix,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        /** A suffix for alpha, beta or rc release, e.g., "alpha-1", "rc2". It should
         be empty for mainline stable releases.
        */
        pub fn suffix_opt(&self) -> ::std::option::Option<&str> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(&self.fields.suffix, self.shared.bitfield())
        }
        pub fn has_suffix(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(&self.fields.suffix, self.shared.bitfield())
                .is_some()
        }
    }
    impl ::std::ops::Drop for VersionView {
        fn drop(&mut self) {
            #[allow(unused)]
            use self::_pinternal::{OneofUnion as _, SharedItems as _};
        }
    }
    impl ::std::fmt::Debug for VersionView {
        fn fmt(
            &self,
            fmt: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            let mut debug_struct = fmt.debug_struct(stringify!(VersionView));
            debug_struct
                .field(stringify!(major), &self.major_opt())
                .field(stringify!(minor), &self.minor_opt())
                .field(stringify!(patch), &self.patch_opt())
                .field(stringify!(suffix), &self.suffix_opt());
            self.shared
                .unknown_fields()
                .debug_struct_fields(&mut debug_struct)?;
            debug_struct.finish()
        }
    }
    impl ::std::cmp::PartialEq for VersionView {
        fn eq(&self, rhs: &Self) -> bool {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::SharedItems as _;
            true && self.major_opt() == rhs.major_opt()
                && self.minor_opt() == rhs.minor_opt()
                && self.patch_opt() == rhs.patch_opt()
                && self.suffix_opt() == rhs.suffix_opt()
                && self.shared.unknown_fields() == rhs.shared.unknown_fields()
        }
    }
    impl ::std::borrow::ToOwned for VersionView {
        type Owned = self::_root::google::protobuf::compiler::Version;
        fn to_owned(&self) -> Self::Owned {
            #[allow(unused)]
            use self::_pinternal::SharedItems;
            self::_root::google::protobuf::compiler::Version {
                body: Self {
                    fields: self::_root::google::protobuf::compiler::_fields::VersionFields {
                        major: ::std::clone::Clone::clone(&self.fields.major),
                        minor: ::std::clone::Clone::clone(&self.fields.minor),
                        patch: ::std::clone::Clone::clone(&self.fields.patch),
                        suffix: ::std::clone::Clone::clone(&self.fields.suffix),
                    },
                    shared: ::std::clone::Clone::clone(&self.shared),
                },
            }
        }
    }
    #[derive(::std::default::Default)]
    pub struct CodeGeneratorRequestView {
        pub(super) fields:
            self::_root::google::protobuf::compiler::_fields::CodeGeneratorRequestFields<
                self::_pinternal::RepeatedUnsizedField<
                    ::std::string::String,
                    self::_pinternal::tags::String,
                >,
                self::_pinternal::OptionalUnsizedField<
                    ::std::string::String,
                    self::_pinternal::tags::String,
                    0usize,
                >,
                self::_pinternal::RepeatedMessageField<
                    self::_root::google::protobuf::FileDescriptorProto,
                >,
                self::_pinternal::SingularHeapMessageField<
                    self::_root::google::protobuf::compiler::Version,
                >,
            >,
        pub(super) shared: self::_pinternal::SharedItemsImpl<1usize>,
    }
    impl CodeGeneratorRequestView {
        /** The .proto files that were explicitly listed on the command-line.  The
         code generator should generate code only for these files.  Each file's
         descriptor will be included in proto_file, below.
        */
        pub fn file_to_generate(
            &self,
        ) -> impl '_ + self::_puroro::repeated::RepeatedFieldView<'_, Item = str> {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field(&self.fields.file_to_generate, self.shared.bitfield())
        }
        pub fn parameter(&self) -> &str {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.parameter,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        /** The generator parameter passed on the command-line.
        */
        pub fn parameter_opt(&self) -> ::std::option::Option<&str> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(&self.fields.parameter, self.shared.bitfield())
        }
        pub fn has_parameter(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(&self.fields.parameter, self.shared.bitfield())
                .is_some()
        }
        /** FileDescriptorProtos for all files in files_to_generate and everything
         they import.  The files will appear in topological order, so each file
         appears before any file that imports it.

         protoc guarantees that all proto_files will be written after
         the fields above, even though this is not technically guaranteed by the
         protobuf wire format.  This theoretically could allow a plugin to stream
         in the FileDescriptorProtos and handle them one by one rather than read
         the entire set into memory at once.  However, as of this writing, this
         is not similarly optimized on protoc's end -- it will store all fields in
         memory at once before sending them to the plugin.

         Type names of fields and extensions in the FileDescriptorProto are always
         fully qualified.
        */
        pub fn proto_file(
            &self,
        ) -> impl '_
        + self::_puroro::repeated::RepeatedFieldView<
            '_,
            Item = self::_root::google::protobuf::_view::FileDescriptorProtoView,
        > {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field(&self.fields.proto_file, self.shared.bitfield())
        }
        pub fn compiler_version(
            &self,
        ) -> ::std::option::Option<&self::_root::google::protobuf::compiler::_view::VersionView>
        {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.compiler_version,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        /** The version number of protocol compiler.
        */
        pub fn compiler_version_opt(
            &self,
        ) -> ::std::option::Option<&self::_root::google::protobuf::compiler::_view::VersionView>
        {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.compiler_version,
                self.shared.bitfield(),
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
    }
    impl ::std::ops::Drop for CodeGeneratorRequestView {
        fn drop(&mut self) {
            #[allow(unused)]
            use self::_pinternal::{OneofUnion as _, SharedItems as _};
        }
    }
    impl ::std::fmt::Debug for CodeGeneratorRequestView {
        fn fmt(
            &self,
            fmt: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            let mut debug_struct = fmt.debug_struct(stringify!(CodeGeneratorRequestView));
            debug_struct
                .field(
                    stringify!(file_to_generate),
                    &self
                        .file_to_generate()
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>()
                        .as_slice(),
                )
                .field(stringify!(parameter), &self.parameter_opt())
                .field(
                    stringify!(proto_file),
                    &self
                        .proto_file()
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>()
                        .as_slice(),
                )
                .field(stringify!(compiler_version), &self.compiler_version_opt());
            self.shared
                .unknown_fields()
                .debug_struct_fields(&mut debug_struct)?;
            debug_struct.finish()
        }
    }
    impl ::std::cmp::PartialEq for CodeGeneratorRequestView {
        fn eq(&self, rhs: &Self) -> bool {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::SharedItems as _;
            true && self
                .file_to_generate()
                .into_iter()
                .eq(rhs.file_to_generate())
                && self.parameter_opt() == rhs.parameter_opt()
                && self.proto_file().into_iter().eq(rhs.proto_file())
                && self.compiler_version_opt() == rhs.compiler_version_opt()
                && self.shared.unknown_fields() == rhs.shared.unknown_fields()
        }
    }
    impl ::std::borrow::ToOwned for CodeGeneratorRequestView {
        type Owned = self::_root::google::protobuf::compiler::CodeGeneratorRequest;
        fn to_owned(&self) -> Self::Owned {
            #[allow(unused)]
            use self::_pinternal::SharedItems;
            self::_root::google::protobuf::compiler::CodeGeneratorRequest {
                body: Self {
                    fields: self::_root::google::protobuf::compiler::_fields::CodeGeneratorRequestFields {
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
                },
            }
        }
    }
    #[derive(::std::default::Default)]
    pub struct CodeGeneratorResponseView {
        pub(super) fields:
            self::_root::google::protobuf::compiler::_fields::CodeGeneratorResponseFields<
                self::_pinternal::OptionalUnsizedField<
                    ::std::string::String,
                    self::_pinternal::tags::String,
                    0usize,
                >,
                self::_pinternal::OptionalNumericalField<
                    u64,
                    self::_pinternal::tags::UInt64,
                    1usize,
                >,
                self::_pinternal::RepeatedMessageField<
                    self::_root::google::protobuf::compiler::code_generator_response::File,
                >,
            >,
        pub(super) shared: self::_pinternal::SharedItemsImpl<1usize>,
    }
    impl CodeGeneratorResponseView {
        pub fn error(&self) -> &str {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.error,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        /** Error message.  If non-empty, code generation failed.  The plugin process
         should exit with status code zero even if it reports an error in this way.

         This should be used to indicate errors in .proto files which prevent the
         code generator from generating correct code.  Errors which indicate a
         problem in protoc itself -- such as the input CodeGeneratorRequest being
         unparseable -- should be reported by writing a message to stderr and
         exiting with a non-zero status code.
        */
        pub fn error_opt(&self) -> ::std::option::Option<&str> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(&self.fields.error, self.shared.bitfield())
        }
        pub fn has_error(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(&self.fields.error, self.shared.bitfield())
                .is_some()
        }
        pub fn supported_features(&self) -> u64 {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.supported_features,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        /** A bitmask of supported features that the code generator supports.
         This is a bitwise "or" of values from the Feature enum.
        */
        pub fn supported_features_opt(&self) -> ::std::option::Option<u64> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.supported_features,
                self.shared.bitfield(),
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
        pub fn file(
            &self,
        ) -> impl '_ + self::_puroro::repeated::RepeatedFieldView<
            '_,
            Item = self::_root::google::protobuf::compiler::code_generator_response::_view::FileView,
        >{
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field(&self.fields.file, self.shared.bitfield())
        }
    }
    impl ::std::ops::Drop for CodeGeneratorResponseView {
        fn drop(&mut self) {
            #[allow(unused)]
            use self::_pinternal::{OneofUnion as _, SharedItems as _};
        }
    }
    impl ::std::fmt::Debug for CodeGeneratorResponseView {
        fn fmt(
            &self,
            fmt: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            let mut debug_struct = fmt.debug_struct(stringify!(CodeGeneratorResponseView));
            debug_struct
                .field(stringify!(error), &self.error_opt())
                .field(
                    stringify!(supported_features),
                    &self.supported_features_opt(),
                )
                .field(
                    stringify!(file),
                    &self
                        .file()
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>()
                        .as_slice(),
                );
            self.shared
                .unknown_fields()
                .debug_struct_fields(&mut debug_struct)?;
            debug_struct.finish()
        }
    }
    impl ::std::cmp::PartialEq for CodeGeneratorResponseView {
        fn eq(&self, rhs: &Self) -> bool {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::SharedItems as _;
            true && self.error_opt() == rhs.error_opt()
                && self.supported_features_opt() == rhs.supported_features_opt()
                && self.file().into_iter().eq(rhs.file())
                && self.shared.unknown_fields() == rhs.shared.unknown_fields()
        }
    }
    impl ::std::borrow::ToOwned for CodeGeneratorResponseView {
        type Owned = self::_root::google::protobuf::compiler::CodeGeneratorResponse;
        fn to_owned(&self) -> Self::Owned {
            #[allow(unused)]
            use self::_pinternal::SharedItems;
            self::_root::google::protobuf::compiler::CodeGeneratorResponse {
                body: Self {
                    fields: self::_root::google::protobuf::compiler::_fields::CodeGeneratorResponseFields {
                        error: ::std::clone::Clone::clone(&self.fields.error),
                        supported_features: ::std::clone::Clone::clone(
                            &self.fields.supported_features,
                        ),
                        file: ::std::clone::Clone::clone(&self.fields.file),
                    },
                    shared: ::std::clone::Clone::clone(&self.shared),
                },
            }
        }
    }
}
#[doc(inline)]
pub use self::_view::*;
#[doc(hidden)]
pub mod _fields {
    mod _root {
        #[allow(unused)]
        pub(crate) use super::super::_root::*;
    }
    mod _puroro {
        #[allow(unused)]
        pub(crate) use super::_root::_puroro::*;
    }
    mod _pinternal {
        #[allow(unused)]
        pub(crate) use super::_root::_pinternal::*;
    }
    #[derive(::std::default::Default)]
    pub struct VersionFields<TMajor, TMinor, TPatch, TSuffix> {
        pub major: TMajor,
        pub minor: TMinor,
        pub patch: TPatch,
        pub suffix: TSuffix,
    }
    #[derive(::std::default::Default)]
    pub struct CodeGeneratorRequestFields<TFileToGenerate, TParameter, TProtoFile, TCompilerVersion> {
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
#[doc(hidden)]
pub use self::_fields::*;
