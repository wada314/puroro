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
#[derive(::std::cmp::PartialEq)]
/** The protocol compiler can output a FileDescriptorSet containing the .proto
 files it parses.
*/
pub struct FileDescriptorSet(
    ::std::boxed::Box<self::_root::google::protobuf::_view::FileDescriptorSetView>,
);
impl FileDescriptorSet {
    pub fn file_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::FileDescriptorProto> {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.0.fields.file,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn clear_file(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(&mut self.0.fields.file, self.0.shared.bitfield_mut())
    }
    pub const FILE_FIELD_NUMBER: i32 = 1i32;
}
impl self::_puroro::Message for FileDescriptorSet {
    type ViewType = self::_root::google::protobuf::_view::FileDescriptorSetView;
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
        let mut scoped_iter = self::_pinternal::ScopedIter::from_mut_pos_iter(
            &mut pos_iter,
        );
        <Self as self::_pinternal::MessageInternal>::merge_from_scoped_bytes_iter(
            self,
            &mut scoped_iter,
        )?;
        scoped_iter.drop_and_check_scope_completed()?;
        Ok(())
    }
}
impl self::_pinternal::MessageInternal for FileDescriptorSet {
    fn merge_from_scoped_bytes_iter<
        'a,
        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
    >(
        &mut self,
        iter: &mut self::_pinternal::ScopedIter<'a, I>,
    ) -> self::_puroro::Result<()> {
        use self::_pinternal::ser::FieldData;
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        #[allow(unused)]
        use ::std::result::Result;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        #[allow(unused)]
        use ::std::vec::Vec;
        use self::_puroro::PuroroError;
        while let Some((number, field_data))
            = FieldData::from_bytes_scoped_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    1i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FileDescriptorSetView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.file,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    _ => {
                        let field_data = field_data
                            .map(|iter| { iter.collect::<Result<Vec<_>, _>>() })
                            .transpose()?;
                        Err(PuroroError::UnknownFieldNumber(field_data))?
                    }
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(PuroroError::UnknownFieldNumber(field_data)) => {
                    self.0.shared.unknown_fields_mut().push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
}
impl ::std::borrow::Borrow<self::_root::google::protobuf::_view::FileDescriptorSetView>
for FileDescriptorSet {
    fn borrow(&self) -> &self::_root::google::protobuf::_view::FileDescriptorSetView {
        &self
    }
}
impl ::std::clone::Clone for FileDescriptorSet {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use ::std::borrow::ToOwned;
        <self::_root::google::protobuf::_view::FileDescriptorSetView as ToOwned>::to_owned(
            &self,
        )
    }
}
impl ::std::fmt::Debug for FileDescriptorSet {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        <self::_root::google::protobuf::_view::FileDescriptorSetView as ::std::fmt::Debug>::fmt(
            &self,
            fmt,
        )
    }
}
impl ::std::ops::Deref for FileDescriptorSet {
    type Target = self::_root::google::protobuf::_view::FileDescriptorSetView;
    fn deref(&self) -> &Self::Target {
        <::std::boxed::Box<_> as ::std::ops::Deref>::deref(&self.0)
    }
}
#[derive(::std::default::Default)]
#[derive(::std::cmp::PartialEq)]
/** Describes a complete .proto file.
*/
pub struct FileDescriptorProto(
    ::std::boxed::Box<self::_root::google::protobuf::_view::FileDescriptorProtoView>,
);
impl FileDescriptorProto {
    pub fn name_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.name,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_name(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.name,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn package_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.package,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_package(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.package,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn dependency_mut(&mut self) -> &mut ::std::vec::Vec::<::std::string::String> {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.0.fields.dependency,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn clear_dependency(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(
            &mut self.0.fields.dependency,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn public_dependency_mut(&mut self) -> &mut ::std::vec::Vec::<i32> {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.0.fields.public_dependency,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn clear_public_dependency(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(
            &mut self.0.fields.public_dependency,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn weak_dependency_mut(&mut self) -> &mut ::std::vec::Vec::<i32> {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.0.fields.weak_dependency,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn clear_weak_dependency(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(
            &mut self.0.fields.weak_dependency,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn message_type_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::DescriptorProto> {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.0.fields.message_type,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn clear_message_type(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(
            &mut self.0.fields.message_type,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn enum_type_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::EnumDescriptorProto> {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.0.fields.enum_type,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn clear_enum_type(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(
            &mut self.0.fields.enum_type,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn service_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::ServiceDescriptorProto> {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.0.fields.service,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn clear_service(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(
            &mut self.0.fields.service,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn extension_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::FieldDescriptorProto> {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.0.fields.extension,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn clear_extension(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(
            &mut self.0.fields.extension,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn options_mut(&mut self) -> &mut self::_root::google::protobuf::FileOptions {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.options,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_options(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.options,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn source_code_info_mut(
        &mut self,
    ) -> &mut self::_root::google::protobuf::SourceCodeInfo {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.source_code_info,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_source_code_info(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.source_code_info,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn syntax_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.syntax,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_syntax(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.syntax,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub const NAME_FIELD_NUMBER: i32 = 1i32;
    pub const PACKAGE_FIELD_NUMBER: i32 = 2i32;
    pub const DEPENDENCY_FIELD_NUMBER: i32 = 3i32;
    pub const PUBLIC_DEPENDENCY_FIELD_NUMBER: i32 = 10i32;
    pub const WEAK_DEPENDENCY_FIELD_NUMBER: i32 = 11i32;
    pub const MESSAGE_TYPE_FIELD_NUMBER: i32 = 4i32;
    pub const ENUM_TYPE_FIELD_NUMBER: i32 = 5i32;
    pub const SERVICE_FIELD_NUMBER: i32 = 6i32;
    pub const EXTENSION_FIELD_NUMBER: i32 = 7i32;
    pub const OPTIONS_FIELD_NUMBER: i32 = 8i32;
    pub const SOURCE_CODE_INFO_FIELD_NUMBER: i32 = 9i32;
    pub const SYNTAX_FIELD_NUMBER: i32 = 12i32;
}
impl self::_puroro::Message for FileDescriptorProto {
    type ViewType = self::_root::google::protobuf::_view::FileDescriptorProtoView;
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
        let mut scoped_iter = self::_pinternal::ScopedIter::from_mut_pos_iter(
            &mut pos_iter,
        );
        <Self as self::_pinternal::MessageInternal>::merge_from_scoped_bytes_iter(
            self,
            &mut scoped_iter,
        )?;
        scoped_iter.drop_and_check_scope_completed()?;
        Ok(())
    }
}
impl self::_pinternal::MessageInternal for FileDescriptorProto {
    fn merge_from_scoped_bytes_iter<
        'a,
        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
    >(
        &mut self,
        iter: &mut self::_pinternal::ScopedIter<'a, I>,
    ) -> self::_puroro::Result<()> {
        use self::_pinternal::ser::FieldData;
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        #[allow(unused)]
        use ::std::result::Result;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        #[allow(unused)]
        use ::std::vec::Vec;
        use self::_puroro::PuroroError;
        while let Some((number, field_data))
            = FieldData::from_bytes_scoped_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    1i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FileDescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.name,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    2i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FileDescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.package,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    3i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FileDescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.dependency,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    10i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FileDescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.public_dependency,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    11i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FileDescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.weak_dependency,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    4i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FileDescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.message_type,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    5i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FileDescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.enum_type,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    6i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FileDescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.service,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    7i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FileDescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.extension,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    8i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FileDescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.options,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    9i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FileDescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.source_code_info,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    12i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FileDescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.syntax,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    _ => {
                        let field_data = field_data
                            .map(|iter| { iter.collect::<Result<Vec<_>, _>>() })
                            .transpose()?;
                        Err(PuroroError::UnknownFieldNumber(field_data))?
                    }
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(PuroroError::UnknownFieldNumber(field_data)) => {
                    self.0.shared.unknown_fields_mut().push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
}
impl ::std::borrow::Borrow<self::_root::google::protobuf::_view::FileDescriptorProtoView>
for FileDescriptorProto {
    fn borrow(&self) -> &self::_root::google::protobuf::_view::FileDescriptorProtoView {
        &self
    }
}
impl ::std::clone::Clone for FileDescriptorProto {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use ::std::borrow::ToOwned;
        <self::_root::google::protobuf::_view::FileDescriptorProtoView as ToOwned>::to_owned(
            &self,
        )
    }
}
impl ::std::fmt::Debug for FileDescriptorProto {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        <self::_root::google::protobuf::_view::FileDescriptorProtoView as ::std::fmt::Debug>::fmt(
            &self,
            fmt,
        )
    }
}
impl ::std::ops::Deref for FileDescriptorProto {
    type Target = self::_root::google::protobuf::_view::FileDescriptorProtoView;
    fn deref(&self) -> &Self::Target {
        <::std::boxed::Box<_> as ::std::ops::Deref>::deref(&self.0)
    }
}
#[derive(::std::default::Default)]
#[derive(::std::cmp::PartialEq)]
/** Describes a message type.
*/
pub struct DescriptorProto(
    ::std::boxed::Box<self::_root::google::protobuf::_view::DescriptorProtoView>,
);
impl DescriptorProto {
    pub fn name_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::DescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.name,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_name(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::DescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.name,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn field_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::FieldDescriptorProto> {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.0.fields.field,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn clear_field(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(&mut self.0.fields.field, self.0.shared.bitfield_mut())
    }
    pub fn extension_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::FieldDescriptorProto> {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.0.fields.extension,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn clear_extension(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(
            &mut self.0.fields.extension,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn nested_type_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::DescriptorProto> {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.0.fields.nested_type,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn clear_nested_type(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(
            &mut self.0.fields.nested_type,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn enum_type_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::EnumDescriptorProto> {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.0.fields.enum_type,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn clear_enum_type(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(
            &mut self.0.fields.enum_type,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn extension_range_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<
        self::_root::google::protobuf::descriptor_proto::ExtensionRange,
    > {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.0.fields.extension_range,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn clear_extension_range(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(
            &mut self.0.fields.extension_range,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn oneof_decl_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::OneofDescriptorProto> {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.0.fields.oneof_decl,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn clear_oneof_decl(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(
            &mut self.0.fields.oneof_decl,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn options_mut(&mut self) -> &mut self::_root::google::protobuf::MessageOptions {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::DescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.options,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_options(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::DescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.options,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn reserved_range_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<
        self::_root::google::protobuf::descriptor_proto::ReservedRange,
    > {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.0.fields.reserved_range,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn clear_reserved_range(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(
            &mut self.0.fields.reserved_range,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn reserved_name_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<::std::string::String> {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.0.fields.reserved_name,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn clear_reserved_name(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(
            &mut self.0.fields.reserved_name,
            self.0.shared.bitfield_mut(),
        )
    }
    pub const NAME_FIELD_NUMBER: i32 = 1i32;
    pub const FIELD_FIELD_NUMBER: i32 = 2i32;
    pub const EXTENSION_FIELD_NUMBER: i32 = 6i32;
    pub const NESTED_TYPE_FIELD_NUMBER: i32 = 3i32;
    pub const ENUM_TYPE_FIELD_NUMBER: i32 = 4i32;
    pub const EXTENSION_RANGE_FIELD_NUMBER: i32 = 5i32;
    pub const ONEOF_DECL_FIELD_NUMBER: i32 = 8i32;
    pub const OPTIONS_FIELD_NUMBER: i32 = 7i32;
    pub const RESERVED_RANGE_FIELD_NUMBER: i32 = 9i32;
    pub const RESERVED_NAME_FIELD_NUMBER: i32 = 10i32;
}
impl self::_puroro::Message for DescriptorProto {
    type ViewType = self::_root::google::protobuf::_view::DescriptorProtoView;
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
        let mut scoped_iter = self::_pinternal::ScopedIter::from_mut_pos_iter(
            &mut pos_iter,
        );
        <Self as self::_pinternal::MessageInternal>::merge_from_scoped_bytes_iter(
            self,
            &mut scoped_iter,
        )?;
        scoped_iter.drop_and_check_scope_completed()?;
        Ok(())
    }
}
impl self::_pinternal::MessageInternal for DescriptorProto {
    fn merge_from_scoped_bytes_iter<
        'a,
        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
    >(
        &mut self,
        iter: &mut self::_pinternal::ScopedIter<'a, I>,
    ) -> self::_puroro::Result<()> {
        use self::_pinternal::ser::FieldData;
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        #[allow(unused)]
        use ::std::result::Result;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        #[allow(unused)]
        use ::std::vec::Vec;
        use self::_puroro::PuroroError;
        while let Some((number, field_data))
            = FieldData::from_bytes_scoped_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    1i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::DescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.name,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    2i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::DescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.field,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    6i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::DescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.extension,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    3i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::DescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.nested_type,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    4i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::DescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.enum_type,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    5i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::DescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.extension_range,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    8i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::DescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.oneof_decl,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    7i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::DescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.options,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    9i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::DescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.reserved_range,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    10i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::DescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.reserved_name,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    _ => {
                        let field_data = field_data
                            .map(|iter| { iter.collect::<Result<Vec<_>, _>>() })
                            .transpose()?;
                        Err(PuroroError::UnknownFieldNumber(field_data))?
                    }
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(PuroroError::UnknownFieldNumber(field_data)) => {
                    self.0.shared.unknown_fields_mut().push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
}
impl ::std::borrow::Borrow<self::_root::google::protobuf::_view::DescriptorProtoView>
for DescriptorProto {
    fn borrow(&self) -> &self::_root::google::protobuf::_view::DescriptorProtoView {
        &self
    }
}
impl ::std::clone::Clone for DescriptorProto {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use ::std::borrow::ToOwned;
        <self::_root::google::protobuf::_view::DescriptorProtoView as ToOwned>::to_owned(
            &self,
        )
    }
}
impl ::std::fmt::Debug for DescriptorProto {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        <self::_root::google::protobuf::_view::DescriptorProtoView as ::std::fmt::Debug>::fmt(
            &self,
            fmt,
        )
    }
}
impl ::std::ops::Deref for DescriptorProto {
    type Target = self::_root::google::protobuf::_view::DescriptorProtoView;
    fn deref(&self) -> &Self::Target {
        <::std::boxed::Box<_> as ::std::ops::Deref>::deref(&self.0)
    }
}
#[derive(::std::default::Default)]
#[derive(::std::cmp::PartialEq)]
pub struct ExtensionRangeOptions(
    ::std::boxed::Box<self::_root::google::protobuf::_view::ExtensionRangeOptionsView>,
);
impl ExtensionRangeOptions {
    pub fn uninterpreted_option_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::UninterpretedOption> {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.0.fields.uninterpreted_option,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn clear_uninterpreted_option(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(
            &mut self.0.fields.uninterpreted_option,
            self.0.shared.bitfield_mut(),
        )
    }
    pub const UNINTERPRETED_OPTION_FIELD_NUMBER: i32 = 999i32;
}
impl self::_puroro::Message for ExtensionRangeOptions {
    type ViewType = self::_root::google::protobuf::_view::ExtensionRangeOptionsView;
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
        let mut scoped_iter = self::_pinternal::ScopedIter::from_mut_pos_iter(
            &mut pos_iter,
        );
        <Self as self::_pinternal::MessageInternal>::merge_from_scoped_bytes_iter(
            self,
            &mut scoped_iter,
        )?;
        scoped_iter.drop_and_check_scope_completed()?;
        Ok(())
    }
}
impl self::_pinternal::MessageInternal for ExtensionRangeOptions {
    fn merge_from_scoped_bytes_iter<
        'a,
        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
    >(
        &mut self,
        iter: &mut self::_pinternal::ScopedIter<'a, I>,
    ) -> self::_puroro::Result<()> {
        use self::_pinternal::ser::FieldData;
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        #[allow(unused)]
        use ::std::result::Result;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        #[allow(unused)]
        use ::std::vec::Vec;
        use self::_puroro::PuroroError;
        while let Some((number, field_data))
            = FieldData::from_bytes_scoped_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    999i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::ExtensionRangeOptionsView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.uninterpreted_option,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    _ => {
                        let field_data = field_data
                            .map(|iter| { iter.collect::<Result<Vec<_>, _>>() })
                            .transpose()?;
                        Err(PuroroError::UnknownFieldNumber(field_data))?
                    }
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(PuroroError::UnknownFieldNumber(field_data)) => {
                    self.0.shared.unknown_fields_mut().push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
}
impl ::std::borrow::Borrow<
    self::_root::google::protobuf::_view::ExtensionRangeOptionsView,
> for ExtensionRangeOptions {
    fn borrow(
        &self,
    ) -> &self::_root::google::protobuf::_view::ExtensionRangeOptionsView {
        &self
    }
}
impl ::std::clone::Clone for ExtensionRangeOptions {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use ::std::borrow::ToOwned;
        <self::_root::google::protobuf::_view::ExtensionRangeOptionsView as ToOwned>::to_owned(
            &self,
        )
    }
}
impl ::std::fmt::Debug for ExtensionRangeOptions {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        <self::_root::google::protobuf::_view::ExtensionRangeOptionsView as ::std::fmt::Debug>::fmt(
            &self,
            fmt,
        )
    }
}
impl ::std::ops::Deref for ExtensionRangeOptions {
    type Target = self::_root::google::protobuf::_view::ExtensionRangeOptionsView;
    fn deref(&self) -> &Self::Target {
        <::std::boxed::Box<_> as ::std::ops::Deref>::deref(&self.0)
    }
}
#[derive(::std::default::Default)]
#[derive(::std::cmp::PartialEq)]
/** Describes a field within a message.
*/
pub struct FieldDescriptorProto(
    ::std::boxed::Box<self::_root::google::protobuf::_view::FieldDescriptorProtoView>,
);
impl FieldDescriptorProto {
    pub fn name_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FieldDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.name,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_name(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FieldDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.name,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn number_mut(&mut self) -> &mut i32 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FieldDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.number,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_number(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FieldDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.number,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn label_mut(
        &mut self,
    ) -> &mut self::_root::google::protobuf::field_descriptor_proto::Label {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FieldDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.label,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_label(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FieldDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.label,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn type_mut(
        &mut self,
    ) -> &mut self::_root::google::protobuf::field_descriptor_proto::Type {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FieldDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.r#type,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_type(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FieldDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.r#type,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn type_name_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FieldDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.type_name,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_type_name(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FieldDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.type_name,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn extendee_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FieldDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.extendee,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_extendee(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FieldDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.extendee,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn default_value_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FieldDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.default_value,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_default_value(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FieldDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.default_value,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn oneof_index_mut(&mut self) -> &mut i32 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FieldDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.oneof_index,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_oneof_index(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FieldDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.oneof_index,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn json_name_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FieldDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.json_name,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_json_name(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FieldDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.json_name,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn options_mut(&mut self) -> &mut self::_root::google::protobuf::FieldOptions {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FieldDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.options,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_options(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FieldDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.options,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn proto3_optional_mut(&mut self) -> &mut bool {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FieldDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.proto3_optional,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_proto3_optional(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FieldDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.proto3_optional,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub const NAME_FIELD_NUMBER: i32 = 1i32;
    pub const NUMBER_FIELD_NUMBER: i32 = 3i32;
    pub const LABEL_FIELD_NUMBER: i32 = 4i32;
    pub const TYPE_FIELD_NUMBER: i32 = 5i32;
    pub const TYPE_NAME_FIELD_NUMBER: i32 = 6i32;
    pub const EXTENDEE_FIELD_NUMBER: i32 = 2i32;
    pub const DEFAULT_VALUE_FIELD_NUMBER: i32 = 7i32;
    pub const ONEOF_INDEX_FIELD_NUMBER: i32 = 9i32;
    pub const JSON_NAME_FIELD_NUMBER: i32 = 10i32;
    pub const OPTIONS_FIELD_NUMBER: i32 = 8i32;
    pub const PROTO3_OPTIONAL_FIELD_NUMBER: i32 = 17i32;
}
impl self::_puroro::Message for FieldDescriptorProto {
    type ViewType = self::_root::google::protobuf::_view::FieldDescriptorProtoView;
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
        let mut scoped_iter = self::_pinternal::ScopedIter::from_mut_pos_iter(
            &mut pos_iter,
        );
        <Self as self::_pinternal::MessageInternal>::merge_from_scoped_bytes_iter(
            self,
            &mut scoped_iter,
        )?;
        scoped_iter.drop_and_check_scope_completed()?;
        Ok(())
    }
}
impl self::_pinternal::MessageInternal for FieldDescriptorProto {
    fn merge_from_scoped_bytes_iter<
        'a,
        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
    >(
        &mut self,
        iter: &mut self::_pinternal::ScopedIter<'a, I>,
    ) -> self::_puroro::Result<()> {
        use self::_pinternal::ser::FieldData;
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        #[allow(unused)]
        use ::std::result::Result;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        #[allow(unused)]
        use ::std::vec::Vec;
        use self::_puroro::PuroroError;
        while let Some((number, field_data))
            = FieldData::from_bytes_scoped_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    1i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FieldDescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.name,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    3i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FieldDescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.number,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    4i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FieldDescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.label,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    5i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FieldDescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.r#type,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    6i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FieldDescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.type_name,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    2i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FieldDescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.extendee,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    7i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FieldDescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.default_value,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    9i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FieldDescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.oneof_index,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    10i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FieldDescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.json_name,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    8i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FieldDescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.options,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    17i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FieldDescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.proto3_optional,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    _ => {
                        let field_data = field_data
                            .map(|iter| { iter.collect::<Result<Vec<_>, _>>() })
                            .transpose()?;
                        Err(PuroroError::UnknownFieldNumber(field_data))?
                    }
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(PuroroError::UnknownFieldNumber(field_data)) => {
                    self.0.shared.unknown_fields_mut().push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
}
impl ::std::borrow::Borrow<
    self::_root::google::protobuf::_view::FieldDescriptorProtoView,
> for FieldDescriptorProto {
    fn borrow(&self) -> &self::_root::google::protobuf::_view::FieldDescriptorProtoView {
        &self
    }
}
impl ::std::clone::Clone for FieldDescriptorProto {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use ::std::borrow::ToOwned;
        <self::_root::google::protobuf::_view::FieldDescriptorProtoView as ToOwned>::to_owned(
            &self,
        )
    }
}
impl ::std::fmt::Debug for FieldDescriptorProto {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        <self::_root::google::protobuf::_view::FieldDescriptorProtoView as ::std::fmt::Debug>::fmt(
            &self,
            fmt,
        )
    }
}
impl ::std::ops::Deref for FieldDescriptorProto {
    type Target = self::_root::google::protobuf::_view::FieldDescriptorProtoView;
    fn deref(&self) -> &Self::Target {
        <::std::boxed::Box<_> as ::std::ops::Deref>::deref(&self.0)
    }
}
#[derive(::std::default::Default)]
#[derive(::std::cmp::PartialEq)]
/** Describes a oneof.
*/
pub struct OneofDescriptorProto(
    ::std::boxed::Box<self::_root::google::protobuf::_view::OneofDescriptorProtoView>,
);
impl OneofDescriptorProto {
    pub fn name_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::OneofDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.name,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_name(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::OneofDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.name,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn options_mut(&mut self) -> &mut self::_root::google::protobuf::OneofOptions {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::OneofDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.options,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_options(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::OneofDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.options,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub const NAME_FIELD_NUMBER: i32 = 1i32;
    pub const OPTIONS_FIELD_NUMBER: i32 = 2i32;
}
impl self::_puroro::Message for OneofDescriptorProto {
    type ViewType = self::_root::google::protobuf::_view::OneofDescriptorProtoView;
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
        let mut scoped_iter = self::_pinternal::ScopedIter::from_mut_pos_iter(
            &mut pos_iter,
        );
        <Self as self::_pinternal::MessageInternal>::merge_from_scoped_bytes_iter(
            self,
            &mut scoped_iter,
        )?;
        scoped_iter.drop_and_check_scope_completed()?;
        Ok(())
    }
}
impl self::_pinternal::MessageInternal for OneofDescriptorProto {
    fn merge_from_scoped_bytes_iter<
        'a,
        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
    >(
        &mut self,
        iter: &mut self::_pinternal::ScopedIter<'a, I>,
    ) -> self::_puroro::Result<()> {
        use self::_pinternal::ser::FieldData;
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        #[allow(unused)]
        use ::std::result::Result;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        #[allow(unused)]
        use ::std::vec::Vec;
        use self::_puroro::PuroroError;
        while let Some((number, field_data))
            = FieldData::from_bytes_scoped_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    1i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::OneofDescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.name,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    2i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::OneofDescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.options,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    _ => {
                        let field_data = field_data
                            .map(|iter| { iter.collect::<Result<Vec<_>, _>>() })
                            .transpose()?;
                        Err(PuroroError::UnknownFieldNumber(field_data))?
                    }
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(PuroroError::UnknownFieldNumber(field_data)) => {
                    self.0.shared.unknown_fields_mut().push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
}
impl ::std::borrow::Borrow<
    self::_root::google::protobuf::_view::OneofDescriptorProtoView,
> for OneofDescriptorProto {
    fn borrow(&self) -> &self::_root::google::protobuf::_view::OneofDescriptorProtoView {
        &self
    }
}
impl ::std::clone::Clone for OneofDescriptorProto {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use ::std::borrow::ToOwned;
        <self::_root::google::protobuf::_view::OneofDescriptorProtoView as ToOwned>::to_owned(
            &self,
        )
    }
}
impl ::std::fmt::Debug for OneofDescriptorProto {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        <self::_root::google::protobuf::_view::OneofDescriptorProtoView as ::std::fmt::Debug>::fmt(
            &self,
            fmt,
        )
    }
}
impl ::std::ops::Deref for OneofDescriptorProto {
    type Target = self::_root::google::protobuf::_view::OneofDescriptorProtoView;
    fn deref(&self) -> &Self::Target {
        <::std::boxed::Box<_> as ::std::ops::Deref>::deref(&self.0)
    }
}
#[derive(::std::default::Default)]
#[derive(::std::cmp::PartialEq)]
/** Describes an enum type.
*/
pub struct EnumDescriptorProto(
    ::std::boxed::Box<self::_root::google::protobuf::_view::EnumDescriptorProtoView>,
);
impl EnumDescriptorProto {
    pub fn name_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::EnumDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.name,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_name(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::EnumDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.name,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn value_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<
        self::_root::google::protobuf::EnumValueDescriptorProto,
    > {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.0.fields.value,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn clear_value(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(&mut self.0.fields.value, self.0.shared.bitfield_mut())
    }
    pub fn options_mut(&mut self) -> &mut self::_root::google::protobuf::EnumOptions {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::EnumDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.options,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_options(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::EnumDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.options,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn reserved_range_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<
        self::_root::google::protobuf::enum_descriptor_proto::EnumReservedRange,
    > {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.0.fields.reserved_range,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn clear_reserved_range(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(
            &mut self.0.fields.reserved_range,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn reserved_name_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<::std::string::String> {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.0.fields.reserved_name,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn clear_reserved_name(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(
            &mut self.0.fields.reserved_name,
            self.0.shared.bitfield_mut(),
        )
    }
    pub const NAME_FIELD_NUMBER: i32 = 1i32;
    pub const VALUE_FIELD_NUMBER: i32 = 2i32;
    pub const OPTIONS_FIELD_NUMBER: i32 = 3i32;
    pub const RESERVED_RANGE_FIELD_NUMBER: i32 = 4i32;
    pub const RESERVED_NAME_FIELD_NUMBER: i32 = 5i32;
}
impl self::_puroro::Message for EnumDescriptorProto {
    type ViewType = self::_root::google::protobuf::_view::EnumDescriptorProtoView;
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
        let mut scoped_iter = self::_pinternal::ScopedIter::from_mut_pos_iter(
            &mut pos_iter,
        );
        <Self as self::_pinternal::MessageInternal>::merge_from_scoped_bytes_iter(
            self,
            &mut scoped_iter,
        )?;
        scoped_iter.drop_and_check_scope_completed()?;
        Ok(())
    }
}
impl self::_pinternal::MessageInternal for EnumDescriptorProto {
    fn merge_from_scoped_bytes_iter<
        'a,
        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
    >(
        &mut self,
        iter: &mut self::_pinternal::ScopedIter<'a, I>,
    ) -> self::_puroro::Result<()> {
        use self::_pinternal::ser::FieldData;
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        #[allow(unused)]
        use ::std::result::Result;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        #[allow(unused)]
        use ::std::vec::Vec;
        use self::_puroro::PuroroError;
        while let Some((number, field_data))
            = FieldData::from_bytes_scoped_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    1i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::EnumDescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.name,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    2i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::EnumDescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.value,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    3i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::EnumDescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.options,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    4i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::EnumDescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.reserved_range,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    5i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::EnumDescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.reserved_name,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    _ => {
                        let field_data = field_data
                            .map(|iter| { iter.collect::<Result<Vec<_>, _>>() })
                            .transpose()?;
                        Err(PuroroError::UnknownFieldNumber(field_data))?
                    }
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(PuroroError::UnknownFieldNumber(field_data)) => {
                    self.0.shared.unknown_fields_mut().push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
}
impl ::std::borrow::Borrow<self::_root::google::protobuf::_view::EnumDescriptorProtoView>
for EnumDescriptorProto {
    fn borrow(&self) -> &self::_root::google::protobuf::_view::EnumDescriptorProtoView {
        &self
    }
}
impl ::std::clone::Clone for EnumDescriptorProto {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use ::std::borrow::ToOwned;
        <self::_root::google::protobuf::_view::EnumDescriptorProtoView as ToOwned>::to_owned(
            &self,
        )
    }
}
impl ::std::fmt::Debug for EnumDescriptorProto {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        <self::_root::google::protobuf::_view::EnumDescriptorProtoView as ::std::fmt::Debug>::fmt(
            &self,
            fmt,
        )
    }
}
impl ::std::ops::Deref for EnumDescriptorProto {
    type Target = self::_root::google::protobuf::_view::EnumDescriptorProtoView;
    fn deref(&self) -> &Self::Target {
        <::std::boxed::Box<_> as ::std::ops::Deref>::deref(&self.0)
    }
}
#[derive(::std::default::Default)]
#[derive(::std::cmp::PartialEq)]
/** Describes a value within an enum.
*/
pub struct EnumValueDescriptorProto(
    ::std::boxed::Box<self::_root::google::protobuf::_view::EnumValueDescriptorProtoView>,
);
impl EnumValueDescriptorProto {
    pub fn name_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::EnumValueDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.name,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_name(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::EnumValueDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.name,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn number_mut(&mut self) -> &mut i32 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::EnumValueDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.number,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_number(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::EnumValueDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.number,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn options_mut(
        &mut self,
    ) -> &mut self::_root::google::protobuf::EnumValueOptions {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::EnumValueDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.options,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_options(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::EnumValueDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.options,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub const NAME_FIELD_NUMBER: i32 = 1i32;
    pub const NUMBER_FIELD_NUMBER: i32 = 2i32;
    pub const OPTIONS_FIELD_NUMBER: i32 = 3i32;
}
impl self::_puroro::Message for EnumValueDescriptorProto {
    type ViewType = self::_root::google::protobuf::_view::EnumValueDescriptorProtoView;
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
        let mut scoped_iter = self::_pinternal::ScopedIter::from_mut_pos_iter(
            &mut pos_iter,
        );
        <Self as self::_pinternal::MessageInternal>::merge_from_scoped_bytes_iter(
            self,
            &mut scoped_iter,
        )?;
        scoped_iter.drop_and_check_scope_completed()?;
        Ok(())
    }
}
impl self::_pinternal::MessageInternal for EnumValueDescriptorProto {
    fn merge_from_scoped_bytes_iter<
        'a,
        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
    >(
        &mut self,
        iter: &mut self::_pinternal::ScopedIter<'a, I>,
    ) -> self::_puroro::Result<()> {
        use self::_pinternal::ser::FieldData;
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        #[allow(unused)]
        use ::std::result::Result;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        #[allow(unused)]
        use ::std::vec::Vec;
        use self::_puroro::PuroroError;
        while let Some((number, field_data))
            = FieldData::from_bytes_scoped_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    1i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::EnumValueDescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.name,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    2i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::EnumValueDescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.number,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    3i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::EnumValueDescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.options,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    _ => {
                        let field_data = field_data
                            .map(|iter| { iter.collect::<Result<Vec<_>, _>>() })
                            .transpose()?;
                        Err(PuroroError::UnknownFieldNumber(field_data))?
                    }
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(PuroroError::UnknownFieldNumber(field_data)) => {
                    self.0.shared.unknown_fields_mut().push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
}
impl ::std::borrow::Borrow<
    self::_root::google::protobuf::_view::EnumValueDescriptorProtoView,
> for EnumValueDescriptorProto {
    fn borrow(
        &self,
    ) -> &self::_root::google::protobuf::_view::EnumValueDescriptorProtoView {
        &self
    }
}
impl ::std::clone::Clone for EnumValueDescriptorProto {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use ::std::borrow::ToOwned;
        <self::_root::google::protobuf::_view::EnumValueDescriptorProtoView as ToOwned>::to_owned(
            &self,
        )
    }
}
impl ::std::fmt::Debug for EnumValueDescriptorProto {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        <self::_root::google::protobuf::_view::EnumValueDescriptorProtoView as ::std::fmt::Debug>::fmt(
            &self,
            fmt,
        )
    }
}
impl ::std::ops::Deref for EnumValueDescriptorProto {
    type Target = self::_root::google::protobuf::_view::EnumValueDescriptorProtoView;
    fn deref(&self) -> &Self::Target {
        <::std::boxed::Box<_> as ::std::ops::Deref>::deref(&self.0)
    }
}
#[derive(::std::default::Default)]
#[derive(::std::cmp::PartialEq)]
/** Describes a service.
*/
pub struct ServiceDescriptorProto(
    ::std::boxed::Box<self::_root::google::protobuf::_view::ServiceDescriptorProtoView>,
);
impl ServiceDescriptorProto {
    pub fn name_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::ServiceDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.name,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_name(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::ServiceDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.name,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn method_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::MethodDescriptorProto> {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.0.fields.method,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn clear_method(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(&mut self.0.fields.method, self.0.shared.bitfield_mut())
    }
    pub fn options_mut(&mut self) -> &mut self::_root::google::protobuf::ServiceOptions {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::ServiceDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.options,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_options(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::ServiceDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.options,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub const NAME_FIELD_NUMBER: i32 = 1i32;
    pub const METHOD_FIELD_NUMBER: i32 = 2i32;
    pub const OPTIONS_FIELD_NUMBER: i32 = 3i32;
}
impl self::_puroro::Message for ServiceDescriptorProto {
    type ViewType = self::_root::google::protobuf::_view::ServiceDescriptorProtoView;
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
        let mut scoped_iter = self::_pinternal::ScopedIter::from_mut_pos_iter(
            &mut pos_iter,
        );
        <Self as self::_pinternal::MessageInternal>::merge_from_scoped_bytes_iter(
            self,
            &mut scoped_iter,
        )?;
        scoped_iter.drop_and_check_scope_completed()?;
        Ok(())
    }
}
impl self::_pinternal::MessageInternal for ServiceDescriptorProto {
    fn merge_from_scoped_bytes_iter<
        'a,
        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
    >(
        &mut self,
        iter: &mut self::_pinternal::ScopedIter<'a, I>,
    ) -> self::_puroro::Result<()> {
        use self::_pinternal::ser::FieldData;
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        #[allow(unused)]
        use ::std::result::Result;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        #[allow(unused)]
        use ::std::vec::Vec;
        use self::_puroro::PuroroError;
        while let Some((number, field_data))
            = FieldData::from_bytes_scoped_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    1i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::ServiceDescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.name,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    2i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::ServiceDescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.method,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    3i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::ServiceDescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.options,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    _ => {
                        let field_data = field_data
                            .map(|iter| { iter.collect::<Result<Vec<_>, _>>() })
                            .transpose()?;
                        Err(PuroroError::UnknownFieldNumber(field_data))?
                    }
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(PuroroError::UnknownFieldNumber(field_data)) => {
                    self.0.shared.unknown_fields_mut().push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
}
impl ::std::borrow::Borrow<
    self::_root::google::protobuf::_view::ServiceDescriptorProtoView,
> for ServiceDescriptorProto {
    fn borrow(
        &self,
    ) -> &self::_root::google::protobuf::_view::ServiceDescriptorProtoView {
        &self
    }
}
impl ::std::clone::Clone for ServiceDescriptorProto {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use ::std::borrow::ToOwned;
        <self::_root::google::protobuf::_view::ServiceDescriptorProtoView as ToOwned>::to_owned(
            &self,
        )
    }
}
impl ::std::fmt::Debug for ServiceDescriptorProto {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        <self::_root::google::protobuf::_view::ServiceDescriptorProtoView as ::std::fmt::Debug>::fmt(
            &self,
            fmt,
        )
    }
}
impl ::std::ops::Deref for ServiceDescriptorProto {
    type Target = self::_root::google::protobuf::_view::ServiceDescriptorProtoView;
    fn deref(&self) -> &Self::Target {
        <::std::boxed::Box<_> as ::std::ops::Deref>::deref(&self.0)
    }
}
#[derive(::std::default::Default)]
#[derive(::std::cmp::PartialEq)]
/** Describes a method of a service.
*/
pub struct MethodDescriptorProto(
    ::std::boxed::Box<self::_root::google::protobuf::_view::MethodDescriptorProtoView>,
);
impl MethodDescriptorProto {
    pub fn name_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::MethodDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.name,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_name(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::MethodDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.name,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn input_type_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::MethodDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.input_type,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_input_type(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::MethodDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.input_type,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn output_type_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::MethodDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.output_type,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_output_type(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::MethodDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.output_type,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn options_mut(&mut self) -> &mut self::_root::google::protobuf::MethodOptions {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::MethodDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.options,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_options(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::MethodDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.options,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn client_streaming_mut(&mut self) -> &mut bool {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::MethodDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.client_streaming,
            mut_view.shared.bitfield_mut(),
            || false,
        )
    }
    pub fn clear_client_streaming(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::MethodDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.client_streaming,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn server_streaming_mut(&mut self) -> &mut bool {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::MethodDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.server_streaming,
            mut_view.shared.bitfield_mut(),
            || false,
        )
    }
    pub fn clear_server_streaming(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::MethodDescriptorProtoView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.server_streaming,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub const NAME_FIELD_NUMBER: i32 = 1i32;
    pub const INPUT_TYPE_FIELD_NUMBER: i32 = 2i32;
    pub const OUTPUT_TYPE_FIELD_NUMBER: i32 = 3i32;
    pub const OPTIONS_FIELD_NUMBER: i32 = 4i32;
    pub const CLIENT_STREAMING_FIELD_NUMBER: i32 = 5i32;
    pub const SERVER_STREAMING_FIELD_NUMBER: i32 = 6i32;
}
impl self::_puroro::Message for MethodDescriptorProto {
    type ViewType = self::_root::google::protobuf::_view::MethodDescriptorProtoView;
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
        let mut scoped_iter = self::_pinternal::ScopedIter::from_mut_pos_iter(
            &mut pos_iter,
        );
        <Self as self::_pinternal::MessageInternal>::merge_from_scoped_bytes_iter(
            self,
            &mut scoped_iter,
        )?;
        scoped_iter.drop_and_check_scope_completed()?;
        Ok(())
    }
}
impl self::_pinternal::MessageInternal for MethodDescriptorProto {
    fn merge_from_scoped_bytes_iter<
        'a,
        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
    >(
        &mut self,
        iter: &mut self::_pinternal::ScopedIter<'a, I>,
    ) -> self::_puroro::Result<()> {
        use self::_pinternal::ser::FieldData;
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        #[allow(unused)]
        use ::std::result::Result;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        #[allow(unused)]
        use ::std::vec::Vec;
        use self::_puroro::PuroroError;
        while let Some((number, field_data))
            = FieldData::from_bytes_scoped_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    1i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::MethodDescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.name,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    2i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::MethodDescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.input_type,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    3i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::MethodDescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.output_type,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    4i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::MethodDescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.options,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    5i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::MethodDescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.client_streaming,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    6i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::MethodDescriptorProtoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.server_streaming,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    _ => {
                        let field_data = field_data
                            .map(|iter| { iter.collect::<Result<Vec<_>, _>>() })
                            .transpose()?;
                        Err(PuroroError::UnknownFieldNumber(field_data))?
                    }
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(PuroroError::UnknownFieldNumber(field_data)) => {
                    self.0.shared.unknown_fields_mut().push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
}
impl ::std::borrow::Borrow<
    self::_root::google::protobuf::_view::MethodDescriptorProtoView,
> for MethodDescriptorProto {
    fn borrow(
        &self,
    ) -> &self::_root::google::protobuf::_view::MethodDescriptorProtoView {
        &self
    }
}
impl ::std::clone::Clone for MethodDescriptorProto {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use ::std::borrow::ToOwned;
        <self::_root::google::protobuf::_view::MethodDescriptorProtoView as ToOwned>::to_owned(
            &self,
        )
    }
}
impl ::std::fmt::Debug for MethodDescriptorProto {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        <self::_root::google::protobuf::_view::MethodDescriptorProtoView as ::std::fmt::Debug>::fmt(
            &self,
            fmt,
        )
    }
}
impl ::std::ops::Deref for MethodDescriptorProto {
    type Target = self::_root::google::protobuf::_view::MethodDescriptorProtoView;
    fn deref(&self) -> &Self::Target {
        <::std::boxed::Box<_> as ::std::ops::Deref>::deref(&self.0)
    }
}
#[derive(::std::default::Default)]
#[derive(::std::cmp::PartialEq)]
pub struct FileOptions(
    ::std::boxed::Box<self::_root::google::protobuf::_view::FileOptionsView>,
);
impl FileOptions {
    pub fn java_package_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.java_package,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_java_package(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.java_package,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn java_outer_classname_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.java_outer_classname,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_java_outer_classname(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.java_outer_classname,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn java_multiple_files_mut(&mut self) -> &mut bool {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.java_multiple_files,
            mut_view.shared.bitfield_mut(),
            || false,
        )
    }
    pub fn clear_java_multiple_files(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.java_multiple_files,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn java_generate_equals_and_hash_mut(&mut self) -> &mut bool {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.java_generate_equals_and_hash,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_java_generate_equals_and_hash(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.java_generate_equals_and_hash,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn java_string_check_utf8_mut(&mut self) -> &mut bool {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.java_string_check_utf8,
            mut_view.shared.bitfield_mut(),
            || false,
        )
    }
    pub fn clear_java_string_check_utf8(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.java_string_check_utf8,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn optimize_for_mut(
        &mut self,
    ) -> &mut self::_root::google::protobuf::file_options::OptimizeMode {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.optimize_for,
            mut_view.shared.bitfield_mut(),
            || self::_root::google::protobuf::file_options::OptimizeMode::Speed,
        )
    }
    pub fn clear_optimize_for(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.optimize_for,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn go_package_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.go_package,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_go_package(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.go_package,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn cc_generic_services_mut(&mut self) -> &mut bool {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.cc_generic_services,
            mut_view.shared.bitfield_mut(),
            || false,
        )
    }
    pub fn clear_cc_generic_services(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.cc_generic_services,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn java_generic_services_mut(&mut self) -> &mut bool {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.java_generic_services,
            mut_view.shared.bitfield_mut(),
            || false,
        )
    }
    pub fn clear_java_generic_services(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.java_generic_services,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn py_generic_services_mut(&mut self) -> &mut bool {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.py_generic_services,
            mut_view.shared.bitfield_mut(),
            || false,
        )
    }
    pub fn clear_py_generic_services(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.py_generic_services,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn php_generic_services_mut(&mut self) -> &mut bool {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.php_generic_services,
            mut_view.shared.bitfield_mut(),
            || false,
        )
    }
    pub fn clear_php_generic_services(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.php_generic_services,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn deprecated_mut(&mut self) -> &mut bool {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.deprecated,
            mut_view.shared.bitfield_mut(),
            || false,
        )
    }
    pub fn clear_deprecated(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.deprecated,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn cc_enable_arenas_mut(&mut self) -> &mut bool {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.cc_enable_arenas,
            mut_view.shared.bitfield_mut(),
            || true,
        )
    }
    pub fn clear_cc_enable_arenas(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.cc_enable_arenas,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn objc_class_prefix_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.objc_class_prefix,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_objc_class_prefix(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.objc_class_prefix,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn csharp_namespace_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.csharp_namespace,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_csharp_namespace(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.csharp_namespace,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn swift_prefix_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.swift_prefix,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_swift_prefix(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.swift_prefix,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn php_class_prefix_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.php_class_prefix,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_php_class_prefix(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.php_class_prefix,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn php_namespace_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.php_namespace,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_php_namespace(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.php_namespace,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn php_metadata_namespace_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.php_metadata_namespace,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_php_metadata_namespace(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.php_metadata_namespace,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn ruby_package_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.ruby_package,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_ruby_package(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.ruby_package,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn uninterpreted_option_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::UninterpretedOption> {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.0.fields.uninterpreted_option,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn clear_uninterpreted_option(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(
            &mut self.0.fields.uninterpreted_option,
            self.0.shared.bitfield_mut(),
        )
    }
    pub const JAVA_PACKAGE_FIELD_NUMBER: i32 = 1i32;
    pub const JAVA_OUTER_CLASSNAME_FIELD_NUMBER: i32 = 8i32;
    pub const JAVA_MULTIPLE_FILES_FIELD_NUMBER: i32 = 10i32;
    pub const JAVA_GENERATE_EQUALS_AND_HASH_FIELD_NUMBER: i32 = 20i32;
    pub const JAVA_STRING_CHECK_UTF8_FIELD_NUMBER: i32 = 27i32;
    pub const OPTIMIZE_FOR_FIELD_NUMBER: i32 = 9i32;
    pub const GO_PACKAGE_FIELD_NUMBER: i32 = 11i32;
    pub const CC_GENERIC_SERVICES_FIELD_NUMBER: i32 = 16i32;
    pub const JAVA_GENERIC_SERVICES_FIELD_NUMBER: i32 = 17i32;
    pub const PY_GENERIC_SERVICES_FIELD_NUMBER: i32 = 18i32;
    pub const PHP_GENERIC_SERVICES_FIELD_NUMBER: i32 = 42i32;
    pub const DEPRECATED_FIELD_NUMBER: i32 = 23i32;
    pub const CC_ENABLE_ARENAS_FIELD_NUMBER: i32 = 31i32;
    pub const OBJC_CLASS_PREFIX_FIELD_NUMBER: i32 = 36i32;
    pub const CSHARP_NAMESPACE_FIELD_NUMBER: i32 = 37i32;
    pub const SWIFT_PREFIX_FIELD_NUMBER: i32 = 39i32;
    pub const PHP_CLASS_PREFIX_FIELD_NUMBER: i32 = 40i32;
    pub const PHP_NAMESPACE_FIELD_NUMBER: i32 = 41i32;
    pub const PHP_METADATA_NAMESPACE_FIELD_NUMBER: i32 = 44i32;
    pub const RUBY_PACKAGE_FIELD_NUMBER: i32 = 45i32;
    pub const UNINTERPRETED_OPTION_FIELD_NUMBER: i32 = 999i32;
}
impl self::_puroro::Message for FileOptions {
    type ViewType = self::_root::google::protobuf::_view::FileOptionsView;
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
        let mut scoped_iter = self::_pinternal::ScopedIter::from_mut_pos_iter(
            &mut pos_iter,
        );
        <Self as self::_pinternal::MessageInternal>::merge_from_scoped_bytes_iter(
            self,
            &mut scoped_iter,
        )?;
        scoped_iter.drop_and_check_scope_completed()?;
        Ok(())
    }
}
impl self::_pinternal::MessageInternal for FileOptions {
    fn merge_from_scoped_bytes_iter<
        'a,
        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
    >(
        &mut self,
        iter: &mut self::_pinternal::ScopedIter<'a, I>,
    ) -> self::_puroro::Result<()> {
        use self::_pinternal::ser::FieldData;
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        #[allow(unused)]
        use ::std::result::Result;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        #[allow(unused)]
        use ::std::vec::Vec;
        use self::_puroro::PuroroError;
        while let Some((number, field_data))
            = FieldData::from_bytes_scoped_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    1i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.java_package,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    8i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.java_outer_classname,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    10i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.java_multiple_files,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    20i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.java_generate_equals_and_hash,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    27i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.java_string_check_utf8,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    9i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.optimize_for,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    11i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.go_package,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    16i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.cc_generic_services,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    17i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.java_generic_services,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    18i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.py_generic_services,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    42i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.php_generic_services,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    23i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.deprecated,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    31i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.cc_enable_arenas,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    36i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.objc_class_prefix,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    37i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.csharp_namespace,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    39i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.swift_prefix,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    40i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.php_class_prefix,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    41i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.php_namespace,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    44i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.php_metadata_namespace,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    45i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.ruby_package,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    999i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FileOptionsView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.uninterpreted_option,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    _ => {
                        let field_data = field_data
                            .map(|iter| { iter.collect::<Result<Vec<_>, _>>() })
                            .transpose()?;
                        Err(PuroroError::UnknownFieldNumber(field_data))?
                    }
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(PuroroError::UnknownFieldNumber(field_data)) => {
                    self.0.shared.unknown_fields_mut().push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
}
impl ::std::borrow::Borrow<self::_root::google::protobuf::_view::FileOptionsView>
for FileOptions {
    fn borrow(&self) -> &self::_root::google::protobuf::_view::FileOptionsView {
        &self
    }
}
impl ::std::clone::Clone for FileOptions {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use ::std::borrow::ToOwned;
        <self::_root::google::protobuf::_view::FileOptionsView as ToOwned>::to_owned(
            &self,
        )
    }
}
impl ::std::fmt::Debug for FileOptions {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        <self::_root::google::protobuf::_view::FileOptionsView as ::std::fmt::Debug>::fmt(
            &self,
            fmt,
        )
    }
}
impl ::std::ops::Deref for FileOptions {
    type Target = self::_root::google::protobuf::_view::FileOptionsView;
    fn deref(&self) -> &Self::Target {
        <::std::boxed::Box<_> as ::std::ops::Deref>::deref(&self.0)
    }
}
#[derive(::std::default::Default)]
#[derive(::std::cmp::PartialEq)]
pub struct MessageOptions(
    ::std::boxed::Box<self::_root::google::protobuf::_view::MessageOptionsView>,
);
impl MessageOptions {
    pub fn message_set_wire_format_mut(&mut self) -> &mut bool {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::MessageOptionsView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.message_set_wire_format,
            mut_view.shared.bitfield_mut(),
            || false,
        )
    }
    pub fn clear_message_set_wire_format(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::MessageOptionsView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.message_set_wire_format,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn no_standard_descriptor_accessor_mut(&mut self) -> &mut bool {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::MessageOptionsView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.no_standard_descriptor_accessor,
            mut_view.shared.bitfield_mut(),
            || false,
        )
    }
    pub fn clear_no_standard_descriptor_accessor(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::MessageOptionsView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.no_standard_descriptor_accessor,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn deprecated_mut(&mut self) -> &mut bool {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::MessageOptionsView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.deprecated,
            mut_view.shared.bitfield_mut(),
            || false,
        )
    }
    pub fn clear_deprecated(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::MessageOptionsView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.deprecated,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn map_entry_mut(&mut self) -> &mut bool {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::MessageOptionsView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.map_entry,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_map_entry(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::MessageOptionsView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.map_entry,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn uninterpreted_option_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::UninterpretedOption> {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.0.fields.uninterpreted_option,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn clear_uninterpreted_option(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(
            &mut self.0.fields.uninterpreted_option,
            self.0.shared.bitfield_mut(),
        )
    }
    pub const MESSAGE_SET_WIRE_FORMAT_FIELD_NUMBER: i32 = 1i32;
    pub const NO_STANDARD_DESCRIPTOR_ACCESSOR_FIELD_NUMBER: i32 = 2i32;
    pub const DEPRECATED_FIELD_NUMBER: i32 = 3i32;
    pub const MAP_ENTRY_FIELD_NUMBER: i32 = 7i32;
    pub const UNINTERPRETED_OPTION_FIELD_NUMBER: i32 = 999i32;
}
impl self::_puroro::Message for MessageOptions {
    type ViewType = self::_root::google::protobuf::_view::MessageOptionsView;
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
        let mut scoped_iter = self::_pinternal::ScopedIter::from_mut_pos_iter(
            &mut pos_iter,
        );
        <Self as self::_pinternal::MessageInternal>::merge_from_scoped_bytes_iter(
            self,
            &mut scoped_iter,
        )?;
        scoped_iter.drop_and_check_scope_completed()?;
        Ok(())
    }
}
impl self::_pinternal::MessageInternal for MessageOptions {
    fn merge_from_scoped_bytes_iter<
        'a,
        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
    >(
        &mut self,
        iter: &mut self::_pinternal::ScopedIter<'a, I>,
    ) -> self::_puroro::Result<()> {
        use self::_pinternal::ser::FieldData;
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        #[allow(unused)]
        use ::std::result::Result;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        #[allow(unused)]
        use ::std::vec::Vec;
        use self::_puroro::PuroroError;
        while let Some((number, field_data))
            = FieldData::from_bytes_scoped_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    1i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::MessageOptionsView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.message_set_wire_format,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    2i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::MessageOptionsView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.no_standard_descriptor_accessor,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    3i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::MessageOptionsView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.deprecated,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    7i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::MessageOptionsView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.map_entry,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    999i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::MessageOptionsView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.uninterpreted_option,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    _ => {
                        let field_data = field_data
                            .map(|iter| { iter.collect::<Result<Vec<_>, _>>() })
                            .transpose()?;
                        Err(PuroroError::UnknownFieldNumber(field_data))?
                    }
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(PuroroError::UnknownFieldNumber(field_data)) => {
                    self.0.shared.unknown_fields_mut().push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
}
impl ::std::borrow::Borrow<self::_root::google::protobuf::_view::MessageOptionsView>
for MessageOptions {
    fn borrow(&self) -> &self::_root::google::protobuf::_view::MessageOptionsView {
        &self
    }
}
impl ::std::clone::Clone for MessageOptions {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use ::std::borrow::ToOwned;
        <self::_root::google::protobuf::_view::MessageOptionsView as ToOwned>::to_owned(
            &self,
        )
    }
}
impl ::std::fmt::Debug for MessageOptions {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        <self::_root::google::protobuf::_view::MessageOptionsView as ::std::fmt::Debug>::fmt(
            &self,
            fmt,
        )
    }
}
impl ::std::ops::Deref for MessageOptions {
    type Target = self::_root::google::protobuf::_view::MessageOptionsView;
    fn deref(&self) -> &Self::Target {
        <::std::boxed::Box<_> as ::std::ops::Deref>::deref(&self.0)
    }
}
#[derive(::std::default::Default)]
#[derive(::std::cmp::PartialEq)]
pub struct FieldOptions(
    ::std::boxed::Box<self::_root::google::protobuf::_view::FieldOptionsView>,
);
impl FieldOptions {
    pub fn ctype_mut(
        &mut self,
    ) -> &mut self::_root::google::protobuf::field_options::CType {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FieldOptionsView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.ctype,
            mut_view.shared.bitfield_mut(),
            || self::_root::google::protobuf::field_options::CType::String,
        )
    }
    pub fn clear_ctype(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FieldOptionsView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.ctype,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn packed_mut(&mut self) -> &mut bool {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FieldOptionsView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.packed,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_packed(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FieldOptionsView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.packed,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn jstype_mut(
        &mut self,
    ) -> &mut self::_root::google::protobuf::field_options::JSType {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FieldOptionsView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.jstype,
            mut_view.shared.bitfield_mut(),
            || self::_root::google::protobuf::field_options::JSType::JsNormal,
        )
    }
    pub fn clear_jstype(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FieldOptionsView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.jstype,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn lazy_mut(&mut self) -> &mut bool {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FieldOptionsView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.lazy,
            mut_view.shared.bitfield_mut(),
            || false,
        )
    }
    pub fn clear_lazy(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FieldOptionsView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.lazy,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn deprecated_mut(&mut self) -> &mut bool {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FieldOptionsView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.deprecated,
            mut_view.shared.bitfield_mut(),
            || false,
        )
    }
    pub fn clear_deprecated(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FieldOptionsView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.deprecated,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn weak_mut(&mut self) -> &mut bool {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FieldOptionsView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.weak,
            mut_view.shared.bitfield_mut(),
            || false,
        )
    }
    pub fn clear_weak(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::FieldOptionsView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.weak,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn uninterpreted_option_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::UninterpretedOption> {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.0.fields.uninterpreted_option,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn clear_uninterpreted_option(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(
            &mut self.0.fields.uninterpreted_option,
            self.0.shared.bitfield_mut(),
        )
    }
    pub const CTYPE_FIELD_NUMBER: i32 = 1i32;
    pub const PACKED_FIELD_NUMBER: i32 = 2i32;
    pub const JSTYPE_FIELD_NUMBER: i32 = 6i32;
    pub const LAZY_FIELD_NUMBER: i32 = 5i32;
    pub const DEPRECATED_FIELD_NUMBER: i32 = 3i32;
    pub const WEAK_FIELD_NUMBER: i32 = 10i32;
    pub const UNINTERPRETED_OPTION_FIELD_NUMBER: i32 = 999i32;
}
impl self::_puroro::Message for FieldOptions {
    type ViewType = self::_root::google::protobuf::_view::FieldOptionsView;
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
        let mut scoped_iter = self::_pinternal::ScopedIter::from_mut_pos_iter(
            &mut pos_iter,
        );
        <Self as self::_pinternal::MessageInternal>::merge_from_scoped_bytes_iter(
            self,
            &mut scoped_iter,
        )?;
        scoped_iter.drop_and_check_scope_completed()?;
        Ok(())
    }
}
impl self::_pinternal::MessageInternal for FieldOptions {
    fn merge_from_scoped_bytes_iter<
        'a,
        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
    >(
        &mut self,
        iter: &mut self::_pinternal::ScopedIter<'a, I>,
    ) -> self::_puroro::Result<()> {
        use self::_pinternal::ser::FieldData;
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        #[allow(unused)]
        use ::std::result::Result;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        #[allow(unused)]
        use ::std::vec::Vec;
        use self::_puroro::PuroroError;
        while let Some((number, field_data))
            = FieldData::from_bytes_scoped_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    1i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FieldOptionsView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.ctype,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    2i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FieldOptionsView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.packed,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    6i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FieldOptionsView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.jstype,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    5i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FieldOptionsView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.lazy,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    3i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FieldOptionsView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.deprecated,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    10i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FieldOptionsView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.weak,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    999i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::FieldOptionsView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.uninterpreted_option,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    _ => {
                        let field_data = field_data
                            .map(|iter| { iter.collect::<Result<Vec<_>, _>>() })
                            .transpose()?;
                        Err(PuroroError::UnknownFieldNumber(field_data))?
                    }
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(PuroroError::UnknownFieldNumber(field_data)) => {
                    self.0.shared.unknown_fields_mut().push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
}
impl ::std::borrow::Borrow<self::_root::google::protobuf::_view::FieldOptionsView>
for FieldOptions {
    fn borrow(&self) -> &self::_root::google::protobuf::_view::FieldOptionsView {
        &self
    }
}
impl ::std::clone::Clone for FieldOptions {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use ::std::borrow::ToOwned;
        <self::_root::google::protobuf::_view::FieldOptionsView as ToOwned>::to_owned(
            &self,
        )
    }
}
impl ::std::fmt::Debug for FieldOptions {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        <self::_root::google::protobuf::_view::FieldOptionsView as ::std::fmt::Debug>::fmt(
            &self,
            fmt,
        )
    }
}
impl ::std::ops::Deref for FieldOptions {
    type Target = self::_root::google::protobuf::_view::FieldOptionsView;
    fn deref(&self) -> &Self::Target {
        <::std::boxed::Box<_> as ::std::ops::Deref>::deref(&self.0)
    }
}
#[derive(::std::default::Default)]
#[derive(::std::cmp::PartialEq)]
pub struct OneofOptions(
    ::std::boxed::Box<self::_root::google::protobuf::_view::OneofOptionsView>,
);
impl OneofOptions {
    pub fn uninterpreted_option_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::UninterpretedOption> {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.0.fields.uninterpreted_option,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn clear_uninterpreted_option(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(
            &mut self.0.fields.uninterpreted_option,
            self.0.shared.bitfield_mut(),
        )
    }
    pub const UNINTERPRETED_OPTION_FIELD_NUMBER: i32 = 999i32;
}
impl self::_puroro::Message for OneofOptions {
    type ViewType = self::_root::google::protobuf::_view::OneofOptionsView;
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
        let mut scoped_iter = self::_pinternal::ScopedIter::from_mut_pos_iter(
            &mut pos_iter,
        );
        <Self as self::_pinternal::MessageInternal>::merge_from_scoped_bytes_iter(
            self,
            &mut scoped_iter,
        )?;
        scoped_iter.drop_and_check_scope_completed()?;
        Ok(())
    }
}
impl self::_pinternal::MessageInternal for OneofOptions {
    fn merge_from_scoped_bytes_iter<
        'a,
        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
    >(
        &mut self,
        iter: &mut self::_pinternal::ScopedIter<'a, I>,
    ) -> self::_puroro::Result<()> {
        use self::_pinternal::ser::FieldData;
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        #[allow(unused)]
        use ::std::result::Result;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        #[allow(unused)]
        use ::std::vec::Vec;
        use self::_puroro::PuroroError;
        while let Some((number, field_data))
            = FieldData::from_bytes_scoped_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    999i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::OneofOptionsView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.uninterpreted_option,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    _ => {
                        let field_data = field_data
                            .map(|iter| { iter.collect::<Result<Vec<_>, _>>() })
                            .transpose()?;
                        Err(PuroroError::UnknownFieldNumber(field_data))?
                    }
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(PuroroError::UnknownFieldNumber(field_data)) => {
                    self.0.shared.unknown_fields_mut().push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
}
impl ::std::borrow::Borrow<self::_root::google::protobuf::_view::OneofOptionsView>
for OneofOptions {
    fn borrow(&self) -> &self::_root::google::protobuf::_view::OneofOptionsView {
        &self
    }
}
impl ::std::clone::Clone for OneofOptions {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use ::std::borrow::ToOwned;
        <self::_root::google::protobuf::_view::OneofOptionsView as ToOwned>::to_owned(
            &self,
        )
    }
}
impl ::std::fmt::Debug for OneofOptions {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        <self::_root::google::protobuf::_view::OneofOptionsView as ::std::fmt::Debug>::fmt(
            &self,
            fmt,
        )
    }
}
impl ::std::ops::Deref for OneofOptions {
    type Target = self::_root::google::protobuf::_view::OneofOptionsView;
    fn deref(&self) -> &Self::Target {
        <::std::boxed::Box<_> as ::std::ops::Deref>::deref(&self.0)
    }
}
#[derive(::std::default::Default)]
#[derive(::std::cmp::PartialEq)]
pub struct EnumOptions(
    ::std::boxed::Box<self::_root::google::protobuf::_view::EnumOptionsView>,
);
impl EnumOptions {
    pub fn allow_alias_mut(&mut self) -> &mut bool {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::EnumOptionsView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.allow_alias,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_allow_alias(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::EnumOptionsView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.allow_alias,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn deprecated_mut(&mut self) -> &mut bool {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::EnumOptionsView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.deprecated,
            mut_view.shared.bitfield_mut(),
            || false,
        )
    }
    pub fn clear_deprecated(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::EnumOptionsView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.deprecated,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn uninterpreted_option_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::UninterpretedOption> {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.0.fields.uninterpreted_option,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn clear_uninterpreted_option(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(
            &mut self.0.fields.uninterpreted_option,
            self.0.shared.bitfield_mut(),
        )
    }
    pub const ALLOW_ALIAS_FIELD_NUMBER: i32 = 2i32;
    pub const DEPRECATED_FIELD_NUMBER: i32 = 3i32;
    pub const UNINTERPRETED_OPTION_FIELD_NUMBER: i32 = 999i32;
}
impl self::_puroro::Message for EnumOptions {
    type ViewType = self::_root::google::protobuf::_view::EnumOptionsView;
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
        let mut scoped_iter = self::_pinternal::ScopedIter::from_mut_pos_iter(
            &mut pos_iter,
        );
        <Self as self::_pinternal::MessageInternal>::merge_from_scoped_bytes_iter(
            self,
            &mut scoped_iter,
        )?;
        scoped_iter.drop_and_check_scope_completed()?;
        Ok(())
    }
}
impl self::_pinternal::MessageInternal for EnumOptions {
    fn merge_from_scoped_bytes_iter<
        'a,
        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
    >(
        &mut self,
        iter: &mut self::_pinternal::ScopedIter<'a, I>,
    ) -> self::_puroro::Result<()> {
        use self::_pinternal::ser::FieldData;
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        #[allow(unused)]
        use ::std::result::Result;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        #[allow(unused)]
        use ::std::vec::Vec;
        use self::_puroro::PuroroError;
        while let Some((number, field_data))
            = FieldData::from_bytes_scoped_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    2i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::EnumOptionsView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.allow_alias,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    3i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::EnumOptionsView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.deprecated,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    999i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::EnumOptionsView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.uninterpreted_option,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    _ => {
                        let field_data = field_data
                            .map(|iter| { iter.collect::<Result<Vec<_>, _>>() })
                            .transpose()?;
                        Err(PuroroError::UnknownFieldNumber(field_data))?
                    }
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(PuroroError::UnknownFieldNumber(field_data)) => {
                    self.0.shared.unknown_fields_mut().push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
}
impl ::std::borrow::Borrow<self::_root::google::protobuf::_view::EnumOptionsView>
for EnumOptions {
    fn borrow(&self) -> &self::_root::google::protobuf::_view::EnumOptionsView {
        &self
    }
}
impl ::std::clone::Clone for EnumOptions {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use ::std::borrow::ToOwned;
        <self::_root::google::protobuf::_view::EnumOptionsView as ToOwned>::to_owned(
            &self,
        )
    }
}
impl ::std::fmt::Debug for EnumOptions {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        <self::_root::google::protobuf::_view::EnumOptionsView as ::std::fmt::Debug>::fmt(
            &self,
            fmt,
        )
    }
}
impl ::std::ops::Deref for EnumOptions {
    type Target = self::_root::google::protobuf::_view::EnumOptionsView;
    fn deref(&self) -> &Self::Target {
        <::std::boxed::Box<_> as ::std::ops::Deref>::deref(&self.0)
    }
}
#[derive(::std::default::Default)]
#[derive(::std::cmp::PartialEq)]
pub struct EnumValueOptions(
    ::std::boxed::Box<self::_root::google::protobuf::_view::EnumValueOptionsView>,
);
impl EnumValueOptions {
    pub fn deprecated_mut(&mut self) -> &mut bool {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::EnumValueOptionsView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.deprecated,
            mut_view.shared.bitfield_mut(),
            || false,
        )
    }
    pub fn clear_deprecated(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::EnumValueOptionsView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.deprecated,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn uninterpreted_option_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::UninterpretedOption> {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.0.fields.uninterpreted_option,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn clear_uninterpreted_option(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(
            &mut self.0.fields.uninterpreted_option,
            self.0.shared.bitfield_mut(),
        )
    }
    pub const DEPRECATED_FIELD_NUMBER: i32 = 1i32;
    pub const UNINTERPRETED_OPTION_FIELD_NUMBER: i32 = 999i32;
}
impl self::_puroro::Message for EnumValueOptions {
    type ViewType = self::_root::google::protobuf::_view::EnumValueOptionsView;
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
        let mut scoped_iter = self::_pinternal::ScopedIter::from_mut_pos_iter(
            &mut pos_iter,
        );
        <Self as self::_pinternal::MessageInternal>::merge_from_scoped_bytes_iter(
            self,
            &mut scoped_iter,
        )?;
        scoped_iter.drop_and_check_scope_completed()?;
        Ok(())
    }
}
impl self::_pinternal::MessageInternal for EnumValueOptions {
    fn merge_from_scoped_bytes_iter<
        'a,
        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
    >(
        &mut self,
        iter: &mut self::_pinternal::ScopedIter<'a, I>,
    ) -> self::_puroro::Result<()> {
        use self::_pinternal::ser::FieldData;
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        #[allow(unused)]
        use ::std::result::Result;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        #[allow(unused)]
        use ::std::vec::Vec;
        use self::_puroro::PuroroError;
        while let Some((number, field_data))
            = FieldData::from_bytes_scoped_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    1i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::EnumValueOptionsView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.deprecated,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    999i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::EnumValueOptionsView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.uninterpreted_option,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    _ => {
                        let field_data = field_data
                            .map(|iter| { iter.collect::<Result<Vec<_>, _>>() })
                            .transpose()?;
                        Err(PuroroError::UnknownFieldNumber(field_data))?
                    }
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(PuroroError::UnknownFieldNumber(field_data)) => {
                    self.0.shared.unknown_fields_mut().push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
}
impl ::std::borrow::Borrow<self::_root::google::protobuf::_view::EnumValueOptionsView>
for EnumValueOptions {
    fn borrow(&self) -> &self::_root::google::protobuf::_view::EnumValueOptionsView {
        &self
    }
}
impl ::std::clone::Clone for EnumValueOptions {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use ::std::borrow::ToOwned;
        <self::_root::google::protobuf::_view::EnumValueOptionsView as ToOwned>::to_owned(
            &self,
        )
    }
}
impl ::std::fmt::Debug for EnumValueOptions {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        <self::_root::google::protobuf::_view::EnumValueOptionsView as ::std::fmt::Debug>::fmt(
            &self,
            fmt,
        )
    }
}
impl ::std::ops::Deref for EnumValueOptions {
    type Target = self::_root::google::protobuf::_view::EnumValueOptionsView;
    fn deref(&self) -> &Self::Target {
        <::std::boxed::Box<_> as ::std::ops::Deref>::deref(&self.0)
    }
}
#[derive(::std::default::Default)]
#[derive(::std::cmp::PartialEq)]
pub struct ServiceOptions(
    ::std::boxed::Box<self::_root::google::protobuf::_view::ServiceOptionsView>,
);
impl ServiceOptions {
    pub fn deprecated_mut(&mut self) -> &mut bool {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::ServiceOptionsView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.deprecated,
            mut_view.shared.bitfield_mut(),
            || false,
        )
    }
    pub fn clear_deprecated(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::ServiceOptionsView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.deprecated,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn uninterpreted_option_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::UninterpretedOption> {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.0.fields.uninterpreted_option,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn clear_uninterpreted_option(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(
            &mut self.0.fields.uninterpreted_option,
            self.0.shared.bitfield_mut(),
        )
    }
    pub const DEPRECATED_FIELD_NUMBER: i32 = 33i32;
    pub const UNINTERPRETED_OPTION_FIELD_NUMBER: i32 = 999i32;
}
impl self::_puroro::Message for ServiceOptions {
    type ViewType = self::_root::google::protobuf::_view::ServiceOptionsView;
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
        let mut scoped_iter = self::_pinternal::ScopedIter::from_mut_pos_iter(
            &mut pos_iter,
        );
        <Self as self::_pinternal::MessageInternal>::merge_from_scoped_bytes_iter(
            self,
            &mut scoped_iter,
        )?;
        scoped_iter.drop_and_check_scope_completed()?;
        Ok(())
    }
}
impl self::_pinternal::MessageInternal for ServiceOptions {
    fn merge_from_scoped_bytes_iter<
        'a,
        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
    >(
        &mut self,
        iter: &mut self::_pinternal::ScopedIter<'a, I>,
    ) -> self::_puroro::Result<()> {
        use self::_pinternal::ser::FieldData;
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        #[allow(unused)]
        use ::std::result::Result;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        #[allow(unused)]
        use ::std::vec::Vec;
        use self::_puroro::PuroroError;
        while let Some((number, field_data))
            = FieldData::from_bytes_scoped_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    33i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::ServiceOptionsView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.deprecated,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    999i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::ServiceOptionsView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.uninterpreted_option,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    _ => {
                        let field_data = field_data
                            .map(|iter| { iter.collect::<Result<Vec<_>, _>>() })
                            .transpose()?;
                        Err(PuroroError::UnknownFieldNumber(field_data))?
                    }
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(PuroroError::UnknownFieldNumber(field_data)) => {
                    self.0.shared.unknown_fields_mut().push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
}
impl ::std::borrow::Borrow<self::_root::google::protobuf::_view::ServiceOptionsView>
for ServiceOptions {
    fn borrow(&self) -> &self::_root::google::protobuf::_view::ServiceOptionsView {
        &self
    }
}
impl ::std::clone::Clone for ServiceOptions {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use ::std::borrow::ToOwned;
        <self::_root::google::protobuf::_view::ServiceOptionsView as ToOwned>::to_owned(
            &self,
        )
    }
}
impl ::std::fmt::Debug for ServiceOptions {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        <self::_root::google::protobuf::_view::ServiceOptionsView as ::std::fmt::Debug>::fmt(
            &self,
            fmt,
        )
    }
}
impl ::std::ops::Deref for ServiceOptions {
    type Target = self::_root::google::protobuf::_view::ServiceOptionsView;
    fn deref(&self) -> &Self::Target {
        <::std::boxed::Box<_> as ::std::ops::Deref>::deref(&self.0)
    }
}
#[derive(::std::default::Default)]
#[derive(::std::cmp::PartialEq)]
pub struct MethodOptions(
    ::std::boxed::Box<self::_root::google::protobuf::_view::MethodOptionsView>,
);
impl MethodOptions {
    pub fn deprecated_mut(&mut self) -> &mut bool {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::MethodOptionsView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.deprecated,
            mut_view.shared.bitfield_mut(),
            || false,
        )
    }
    pub fn clear_deprecated(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::MethodOptionsView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.deprecated,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn idempotency_level_mut(
        &mut self,
    ) -> &mut self::_root::google::protobuf::method_options::IdempotencyLevel {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::MethodOptionsView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.idempotency_level,
            mut_view.shared.bitfield_mut(),
            || {
                self::_root::google::protobuf::method_options::IdempotencyLevel::IdempotencyUnknown
            },
        )
    }
    pub fn clear_idempotency_level(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::MethodOptionsView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.idempotency_level,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn uninterpreted_option_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::UninterpretedOption> {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.0.fields.uninterpreted_option,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn clear_uninterpreted_option(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(
            &mut self.0.fields.uninterpreted_option,
            self.0.shared.bitfield_mut(),
        )
    }
    pub const DEPRECATED_FIELD_NUMBER: i32 = 33i32;
    pub const IDEMPOTENCY_LEVEL_FIELD_NUMBER: i32 = 34i32;
    pub const UNINTERPRETED_OPTION_FIELD_NUMBER: i32 = 999i32;
}
impl self::_puroro::Message for MethodOptions {
    type ViewType = self::_root::google::protobuf::_view::MethodOptionsView;
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
        let mut scoped_iter = self::_pinternal::ScopedIter::from_mut_pos_iter(
            &mut pos_iter,
        );
        <Self as self::_pinternal::MessageInternal>::merge_from_scoped_bytes_iter(
            self,
            &mut scoped_iter,
        )?;
        scoped_iter.drop_and_check_scope_completed()?;
        Ok(())
    }
}
impl self::_pinternal::MessageInternal for MethodOptions {
    fn merge_from_scoped_bytes_iter<
        'a,
        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
    >(
        &mut self,
        iter: &mut self::_pinternal::ScopedIter<'a, I>,
    ) -> self::_puroro::Result<()> {
        use self::_pinternal::ser::FieldData;
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        #[allow(unused)]
        use ::std::result::Result;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        #[allow(unused)]
        use ::std::vec::Vec;
        use self::_puroro::PuroroError;
        while let Some((number, field_data))
            = FieldData::from_bytes_scoped_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    33i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::MethodOptionsView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.deprecated,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    34i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::MethodOptionsView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.idempotency_level,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    999i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::MethodOptionsView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.uninterpreted_option,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    _ => {
                        let field_data = field_data
                            .map(|iter| { iter.collect::<Result<Vec<_>, _>>() })
                            .transpose()?;
                        Err(PuroroError::UnknownFieldNumber(field_data))?
                    }
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(PuroroError::UnknownFieldNumber(field_data)) => {
                    self.0.shared.unknown_fields_mut().push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
}
impl ::std::borrow::Borrow<self::_root::google::protobuf::_view::MethodOptionsView>
for MethodOptions {
    fn borrow(&self) -> &self::_root::google::protobuf::_view::MethodOptionsView {
        &self
    }
}
impl ::std::clone::Clone for MethodOptions {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use ::std::borrow::ToOwned;
        <self::_root::google::protobuf::_view::MethodOptionsView as ToOwned>::to_owned(
            &self,
        )
    }
}
impl ::std::fmt::Debug for MethodOptions {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        <self::_root::google::protobuf::_view::MethodOptionsView as ::std::fmt::Debug>::fmt(
            &self,
            fmt,
        )
    }
}
impl ::std::ops::Deref for MethodOptions {
    type Target = self::_root::google::protobuf::_view::MethodOptionsView;
    fn deref(&self) -> &Self::Target {
        <::std::boxed::Box<_> as ::std::ops::Deref>::deref(&self.0)
    }
}
#[derive(::std::default::Default)]
#[derive(::std::cmp::PartialEq)]
/** A message representing a option the parser does not recognize. This only
 appears in options protos created by the compiler::Parser class.
 DescriptorPool resolves these when building Descriptor objects. Therefore,
 options protos in descriptor objects (e.g. returned by Descriptor::options(),
 or produced by Descriptor::CopyTo()) will never have UninterpretedOptions
 in them.
*/
pub struct UninterpretedOption(
    ::std::boxed::Box<self::_root::google::protobuf::_view::UninterpretedOptionView>,
);
impl UninterpretedOption {
    pub fn name_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<
        self::_root::google::protobuf::uninterpreted_option::NamePart,
    > {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.0.fields.name,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn clear_name(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(&mut self.0.fields.name, self.0.shared.bitfield_mut())
    }
    pub fn identifier_value_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::UninterpretedOptionView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.identifier_value,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_identifier_value(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::UninterpretedOptionView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.identifier_value,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn positive_int_value_mut(&mut self) -> &mut u64 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::UninterpretedOptionView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.positive_int_value,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_positive_int_value(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::UninterpretedOptionView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.positive_int_value,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn negative_int_value_mut(&mut self) -> &mut i64 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::UninterpretedOptionView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.negative_int_value,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_negative_int_value(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::UninterpretedOptionView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.negative_int_value,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn double_value_mut(&mut self) -> &mut f64 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::UninterpretedOptionView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.double_value,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_double_value(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::UninterpretedOptionView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.double_value,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn string_value_mut(&mut self) -> &mut ::std::vec::Vec::<u8> {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::UninterpretedOptionView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.string_value,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_string_value(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::UninterpretedOptionView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.string_value,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn aggregate_value_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::UninterpretedOptionView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.aggregate_value,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_aggregate_value(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::_view::UninterpretedOptionView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.aggregate_value,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub const NAME_FIELD_NUMBER: i32 = 2i32;
    pub const IDENTIFIER_VALUE_FIELD_NUMBER: i32 = 3i32;
    pub const POSITIVE_INT_VALUE_FIELD_NUMBER: i32 = 4i32;
    pub const NEGATIVE_INT_VALUE_FIELD_NUMBER: i32 = 5i32;
    pub const DOUBLE_VALUE_FIELD_NUMBER: i32 = 6i32;
    pub const STRING_VALUE_FIELD_NUMBER: i32 = 7i32;
    pub const AGGREGATE_VALUE_FIELD_NUMBER: i32 = 8i32;
}
impl self::_puroro::Message for UninterpretedOption {
    type ViewType = self::_root::google::protobuf::_view::UninterpretedOptionView;
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
        let mut scoped_iter = self::_pinternal::ScopedIter::from_mut_pos_iter(
            &mut pos_iter,
        );
        <Self as self::_pinternal::MessageInternal>::merge_from_scoped_bytes_iter(
            self,
            &mut scoped_iter,
        )?;
        scoped_iter.drop_and_check_scope_completed()?;
        Ok(())
    }
}
impl self::_pinternal::MessageInternal for UninterpretedOption {
    fn merge_from_scoped_bytes_iter<
        'a,
        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
    >(
        &mut self,
        iter: &mut self::_pinternal::ScopedIter<'a, I>,
    ) -> self::_puroro::Result<()> {
        use self::_pinternal::ser::FieldData;
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        #[allow(unused)]
        use ::std::result::Result;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        #[allow(unused)]
        use ::std::vec::Vec;
        use self::_puroro::PuroroError;
        while let Some((number, field_data))
            = FieldData::from_bytes_scoped_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    2i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::UninterpretedOptionView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.name,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    3i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::UninterpretedOptionView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.identifier_value,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    4i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::UninterpretedOptionView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.positive_int_value,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    5i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::UninterpretedOptionView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.negative_int_value,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    6i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::UninterpretedOptionView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.double_value,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    7i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::UninterpretedOptionView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.string_value,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    8i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::UninterpretedOptionView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.aggregate_value,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    _ => {
                        let field_data = field_data
                            .map(|iter| { iter.collect::<Result<Vec<_>, _>>() })
                            .transpose()?;
                        Err(PuroroError::UnknownFieldNumber(field_data))?
                    }
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(PuroroError::UnknownFieldNumber(field_data)) => {
                    self.0.shared.unknown_fields_mut().push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
}
impl ::std::borrow::Borrow<self::_root::google::protobuf::_view::UninterpretedOptionView>
for UninterpretedOption {
    fn borrow(&self) -> &self::_root::google::protobuf::_view::UninterpretedOptionView {
        &self
    }
}
impl ::std::clone::Clone for UninterpretedOption {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use ::std::borrow::ToOwned;
        <self::_root::google::protobuf::_view::UninterpretedOptionView as ToOwned>::to_owned(
            &self,
        )
    }
}
impl ::std::fmt::Debug for UninterpretedOption {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        <self::_root::google::protobuf::_view::UninterpretedOptionView as ::std::fmt::Debug>::fmt(
            &self,
            fmt,
        )
    }
}
impl ::std::ops::Deref for UninterpretedOption {
    type Target = self::_root::google::protobuf::_view::UninterpretedOptionView;
    fn deref(&self) -> &Self::Target {
        <::std::boxed::Box<_> as ::std::ops::Deref>::deref(&self.0)
    }
}
#[derive(::std::default::Default)]
#[derive(::std::cmp::PartialEq)]
/** Encapsulates information about the original source file from which a
 FileDescriptorProto was generated.
*/
pub struct SourceCodeInfo(
    ::std::boxed::Box<self::_root::google::protobuf::_view::SourceCodeInfoView>,
);
impl SourceCodeInfo {
    pub fn location_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<
        self::_root::google::protobuf::source_code_info::Location,
    > {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.0.fields.location,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn clear_location(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(
            &mut self.0.fields.location,
            self.0.shared.bitfield_mut(),
        )
    }
    pub const LOCATION_FIELD_NUMBER: i32 = 1i32;
}
impl self::_puroro::Message for SourceCodeInfo {
    type ViewType = self::_root::google::protobuf::_view::SourceCodeInfoView;
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
        let mut scoped_iter = self::_pinternal::ScopedIter::from_mut_pos_iter(
            &mut pos_iter,
        );
        <Self as self::_pinternal::MessageInternal>::merge_from_scoped_bytes_iter(
            self,
            &mut scoped_iter,
        )?;
        scoped_iter.drop_and_check_scope_completed()?;
        Ok(())
    }
}
impl self::_pinternal::MessageInternal for SourceCodeInfo {
    fn merge_from_scoped_bytes_iter<
        'a,
        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
    >(
        &mut self,
        iter: &mut self::_pinternal::ScopedIter<'a, I>,
    ) -> self::_puroro::Result<()> {
        use self::_pinternal::ser::FieldData;
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        #[allow(unused)]
        use ::std::result::Result;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        #[allow(unused)]
        use ::std::vec::Vec;
        use self::_puroro::PuroroError;
        while let Some((number, field_data))
            = FieldData::from_bytes_scoped_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    1i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::SourceCodeInfoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.location,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    _ => {
                        let field_data = field_data
                            .map(|iter| { iter.collect::<Result<Vec<_>, _>>() })
                            .transpose()?;
                        Err(PuroroError::UnknownFieldNumber(field_data))?
                    }
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(PuroroError::UnknownFieldNumber(field_data)) => {
                    self.0.shared.unknown_fields_mut().push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
}
impl ::std::borrow::Borrow<self::_root::google::protobuf::_view::SourceCodeInfoView>
for SourceCodeInfo {
    fn borrow(&self) -> &self::_root::google::protobuf::_view::SourceCodeInfoView {
        &self
    }
}
impl ::std::clone::Clone for SourceCodeInfo {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use ::std::borrow::ToOwned;
        <self::_root::google::protobuf::_view::SourceCodeInfoView as ToOwned>::to_owned(
            &self,
        )
    }
}
impl ::std::fmt::Debug for SourceCodeInfo {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        <self::_root::google::protobuf::_view::SourceCodeInfoView as ::std::fmt::Debug>::fmt(
            &self,
            fmt,
        )
    }
}
impl ::std::ops::Deref for SourceCodeInfo {
    type Target = self::_root::google::protobuf::_view::SourceCodeInfoView;
    fn deref(&self) -> &Self::Target {
        <::std::boxed::Box<_> as ::std::ops::Deref>::deref(&self.0)
    }
}
#[derive(::std::default::Default)]
#[derive(::std::cmp::PartialEq)]
/** Describes the relationship between generated code and its original source
 file. A GeneratedCodeInfo message is associated with only one generated
 source file, but may contain references to different source .proto files.
*/
pub struct GeneratedCodeInfo(
    ::std::boxed::Box<self::_root::google::protobuf::_view::GeneratedCodeInfoView>,
);
impl GeneratedCodeInfo {
    pub fn annotation_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<
        self::_root::google::protobuf::generated_code_info::Annotation,
    > {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.0.fields.annotation,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn clear_annotation(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(
            &mut self.0.fields.annotation,
            self.0.shared.bitfield_mut(),
        )
    }
    pub const ANNOTATION_FIELD_NUMBER: i32 = 1i32;
}
impl self::_puroro::Message for GeneratedCodeInfo {
    type ViewType = self::_root::google::protobuf::_view::GeneratedCodeInfoView;
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
        let mut scoped_iter = self::_pinternal::ScopedIter::from_mut_pos_iter(
            &mut pos_iter,
        );
        <Self as self::_pinternal::MessageInternal>::merge_from_scoped_bytes_iter(
            self,
            &mut scoped_iter,
        )?;
        scoped_iter.drop_and_check_scope_completed()?;
        Ok(())
    }
}
impl self::_pinternal::MessageInternal for GeneratedCodeInfo {
    fn merge_from_scoped_bytes_iter<
        'a,
        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
    >(
        &mut self,
        iter: &mut self::_pinternal::ScopedIter<'a, I>,
    ) -> self::_puroro::Result<()> {
        use self::_pinternal::ser::FieldData;
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        #[allow(unused)]
        use ::std::result::Result;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        #[allow(unused)]
        use ::std::vec::Vec;
        use self::_puroro::PuroroError;
        while let Some((number, field_data))
            = FieldData::from_bytes_scoped_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    1i32 => {
                        let view_ref: &mut self::_root::google::protobuf::_view::GeneratedCodeInfoView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.annotation,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    _ => {
                        let field_data = field_data
                            .map(|iter| { iter.collect::<Result<Vec<_>, _>>() })
                            .transpose()?;
                        Err(PuroroError::UnknownFieldNumber(field_data))?
                    }
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(PuroroError::UnknownFieldNumber(field_data)) => {
                    self.0.shared.unknown_fields_mut().push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
}
impl ::std::borrow::Borrow<self::_root::google::protobuf::_view::GeneratedCodeInfoView>
for GeneratedCodeInfo {
    fn borrow(&self) -> &self::_root::google::protobuf::_view::GeneratedCodeInfoView {
        &self
    }
}
impl ::std::clone::Clone for GeneratedCodeInfo {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use ::std::borrow::ToOwned;
        <self::_root::google::protobuf::_view::GeneratedCodeInfoView as ToOwned>::to_owned(
            &self,
        )
    }
}
impl ::std::fmt::Debug for GeneratedCodeInfo {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        <self::_root::google::protobuf::_view::GeneratedCodeInfoView as ::std::fmt::Debug>::fmt(
            &self,
            fmt,
        )
    }
}
impl ::std::ops::Deref for GeneratedCodeInfo {
    type Target = self::_root::google::protobuf::_view::GeneratedCodeInfoView;
    fn deref(&self) -> &Self::Target {
        <::std::boxed::Box<_> as ::std::ops::Deref>::deref(&self.0)
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
    pub struct FileDescriptorSetView {
        pub(super) fields: self::_root::google::protobuf::_fields::FileDescriptorSetFields::<
            self::_pinternal::RepeatedMessageField::<
                self::_root::google::protobuf::FileDescriptorProto,
            >,
        >,
        pub(super) shared: self::_pinternal::SharedItemsImpl<0usize>,
    }
    impl FileDescriptorSetView {
        pub fn file(
            &self,
        ) -> impl '_ + self::_puroro::repeated::RepeatedFieldView<
            '_,
            Item = self::_root::google::protobuf::_view::FileDescriptorProtoView,
        > {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field(&self.fields.file, self.shared.bitfield())
        }
    }
    impl self::_puroro::MessageView for self::FileDescriptorSetView {
        type MessageType = self::_root::google::protobuf::FileDescriptorSet;
        fn to_bytes<W: ::std::io::Write>(
            &self,
            #[allow(unused)]
            out: &mut W,
        ) -> self::_puroro::Result<()> {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.file,
                self.shared.bitfield(),
                1i32,
                out,
            )?;
            self.shared.unknown_fields().ser_to_write(out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl self::_pinternal::MessageViewInternal for self::FileDescriptorSetView {
        fn new_boxed() -> ::std::boxed::Box<Self> {
            use self::_pinternal::SharedItems as _;
            let mut shared: self::_pinternal::SharedItemsImpl::<0usize> = ::std::default::Default::default();
            let fields = self::_root::google::protobuf::_fields::FileDescriptorSetFields {
                file: self::_pinternal::FieldType::new(shared.bitfield_mut()),
            };
            ::std::boxed::Box::new(Self { fields, shared })
        }
    }
    impl ::std::ops::Drop for FileDescriptorSetView {
        fn drop(&mut self) {
            #[allow(unused)]
            use self::_pinternal::{OneofUnion as _, SharedItems as _};
        }
    }
    impl ::std::fmt::Debug for FileDescriptorSetView {
        fn fmt(
            &self,
            fmt: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            let mut debug_struct = fmt.debug_struct(stringify!(FileDescriptorSetView));
            debug_struct
                .field(
                    stringify!(file),
                    &self.file().into_iter().collect::<::std::vec::Vec<_>>().as_slice(),
                );
            self.shared.unknown_fields().debug_struct_fields(&mut debug_struct)?;
            debug_struct.finish()
        }
    }
    impl ::std::cmp::PartialEq for FileDescriptorSetView {
        fn eq(&self, rhs: &Self) -> bool {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::SharedItems as _;
            true && self.file().into_iter().eq(rhs.file())
                && self.shared.unknown_fields() == rhs.shared.unknown_fields()
        }
    }
    impl ::std::borrow::ToOwned for FileDescriptorSetView {
        type Owned = self::_root::google::protobuf::FileDescriptorSet;
        fn to_owned(&self) -> Self::Owned {
            #[allow(unused)]
            use self::_pinternal::SharedItems;
            self::_root::google::protobuf::FileDescriptorSet(
                ::std::boxed::Box::new(Self {
                    fields: self::_root::google::protobuf::_fields::FileDescriptorSetFields {
                        file: ::std::clone::Clone::clone(&self.fields.file),
                    },
                    shared: ::std::clone::Clone::clone(&self.shared),
                }),
            )
        }
    }
    #[derive(::std::default::Default)]
    pub struct FileDescriptorProtoView {
        pub(super) fields: self::_root::google::protobuf::_fields::FileDescriptorProtoFields::<
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
            self::_pinternal::RepeatedNumericalField::<
                i32,
                self::_pinternal::tags::Int32,
            >,
            self::_pinternal::RepeatedNumericalField::<
                i32,
                self::_pinternal::tags::Int32,
            >,
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
            self::_pinternal::SingularMessageField::<
                self::_root::google::protobuf::FileOptions,
            >,
            self::_pinternal::SingularMessageField::<
                self::_root::google::protobuf::SourceCodeInfo,
            >,
            self::_pinternal::OptionalUnsizedField::<
                ::std::string::String,
                self::_pinternal::tags::String,
                2usize,
            >,
        >,
        pub(super) shared: self::_pinternal::SharedItemsImpl<1usize>,
    }
    impl FileDescriptorProtoView {
        pub fn name(&self) -> &str {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.name,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        /** file name, relative to root of source tree
*/
        pub fn name_opt(&self) -> ::std::option::Option::<&str> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.name,
                self.shared.bitfield(),
            )
        }
        pub fn has_name(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.name,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn package(&self) -> &str {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.package,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        /** e.g. "foo", "foo.bar", etc.
*/
        pub fn package_opt(&self) -> ::std::option::Option::<&str> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.package,
                self.shared.bitfield(),
            )
        }
        pub fn has_package(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.package,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        /** Names of files imported by this file.
*/
        pub fn dependency(
            &self,
        ) -> impl '_ + self::_puroro::repeated::RepeatedFieldView<'_, Item = str> {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field(&self.fields.dependency, self.shared.bitfield())
        }
        /** Indexes of the public imported files in the dependency list above.
*/
        pub fn public_dependency(
            &self,
        ) -> impl '_ + self::_puroro::repeated::RepeatedFieldView<'_, Item = i32> {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field(
                &self.fields.public_dependency,
                self.shared.bitfield(),
            )
        }
        /** Indexes of the weak imported files in the dependency list.
 For Google-internal migration only. Do not use.
*/
        pub fn weak_dependency(
            &self,
        ) -> impl '_ + self::_puroro::repeated::RepeatedFieldView<'_, Item = i32> {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field(
                &self.fields.weak_dependency,
                self.shared.bitfield(),
            )
        }
        /** All top-level definitions in this file.
*/
        pub fn message_type(
            &self,
        ) -> impl '_ + self::_puroro::repeated::RepeatedFieldView<
            '_,
            Item = self::_root::google::protobuf::_view::DescriptorProtoView,
        > {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field(
                &self.fields.message_type,
                self.shared.bitfield(),
            )
        }
        pub fn enum_type(
            &self,
        ) -> impl '_ + self::_puroro::repeated::RepeatedFieldView<
            '_,
            Item = self::_root::google::protobuf::_view::EnumDescriptorProtoView,
        > {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field(&self.fields.enum_type, self.shared.bitfield())
        }
        pub fn service(
            &self,
        ) -> impl '_ + self::_puroro::repeated::RepeatedFieldView<
            '_,
            Item = self::_root::google::protobuf::_view::ServiceDescriptorProtoView,
        > {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field(&self.fields.service, self.shared.bitfield())
        }
        pub fn extension(
            &self,
        ) -> impl '_ + self::_puroro::repeated::RepeatedFieldView<
            '_,
            Item = self::_root::google::protobuf::_view::FieldDescriptorProtoView,
        > {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field(&self.fields.extension, self.shared.bitfield())
        }
        pub fn options(
            &self,
        ) -> ::std::option::Option::<
            &self::_root::google::protobuf::_view::FileOptionsView,
        > {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.options,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn options_opt(
            &self,
        ) -> ::std::option::Option::<
            &self::_root::google::protobuf::_view::FileOptionsView,
        > {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.options,
                self.shared.bitfield(),
            )
        }
        pub fn has_options(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.options,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn source_code_info(
            &self,
        ) -> ::std::option::Option::<
            &self::_root::google::protobuf::_view::SourceCodeInfoView,
        > {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.source_code_info,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        /** This field contains optional information about the original source code.
 You may safely remove this entire field without harming runtime
 functionality of the descriptors -- the information is needed only by
 development tools.
*/
        pub fn source_code_info_opt(
            &self,
        ) -> ::std::option::Option::<
            &self::_root::google::protobuf::_view::SourceCodeInfoView,
        > {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.source_code_info,
                self.shared.bitfield(),
            )
        }
        pub fn has_source_code_info(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.source_code_info,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn syntax(&self) -> &str {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.syntax,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        /** The syntax of the proto file.
 The supported values are "proto2" and "proto3".
*/
        pub fn syntax_opt(&self) -> ::std::option::Option::<&str> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.syntax,
                self.shared.bitfield(),
            )
        }
        pub fn has_syntax(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.syntax,
                    self.shared.bitfield(),
                )
                .is_some()
        }
    }
    impl self::_puroro::MessageView for self::FileDescriptorProtoView {
        type MessageType = self::_root::google::protobuf::FileDescriptorProto;
        fn to_bytes<W: ::std::io::Write>(
            &self,
            #[allow(unused)]
            out: &mut W,
        ) -> self::_puroro::Result<()> {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.name,
                self.shared.bitfield(),
                1i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.package,
                self.shared.bitfield(),
                2i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.dependency,
                self.shared.bitfield(),
                3i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.public_dependency,
                self.shared.bitfield(),
                10i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.weak_dependency,
                self.shared.bitfield(),
                11i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.message_type,
                self.shared.bitfield(),
                4i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.enum_type,
                self.shared.bitfield(),
                5i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.service,
                self.shared.bitfield(),
                6i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.extension,
                self.shared.bitfield(),
                7i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.options,
                self.shared.bitfield(),
                8i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.source_code_info,
                self.shared.bitfield(),
                9i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.syntax,
                self.shared.bitfield(),
                12i32,
                out,
            )?;
            self.shared.unknown_fields().ser_to_write(out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl self::_pinternal::MessageViewInternal for self::FileDescriptorProtoView {
        fn new_boxed() -> ::std::boxed::Box<Self> {
            use self::_pinternal::SharedItems as _;
            let mut shared: self::_pinternal::SharedItemsImpl::<1usize> = ::std::default::Default::default();
            let fields = self::_root::google::protobuf::_fields::FileDescriptorProtoFields {
                name: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                package: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                dependency: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                public_dependency: self::_pinternal::FieldType::new(
                    shared.bitfield_mut(),
                ),
                weak_dependency: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                message_type: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                enum_type: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                service: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                extension: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                options: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                source_code_info: self::_pinternal::FieldType::new(
                    shared.bitfield_mut(),
                ),
                syntax: self::_pinternal::FieldType::new(shared.bitfield_mut()),
            };
            ::std::boxed::Box::new(Self { fields, shared })
        }
    }
    impl ::std::ops::Drop for FileDescriptorProtoView {
        fn drop(&mut self) {
            #[allow(unused)]
            use self::_pinternal::{OneofUnion as _, SharedItems as _};
        }
    }
    impl ::std::fmt::Debug for FileDescriptorProtoView {
        fn fmt(
            &self,
            fmt: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            let mut debug_struct = fmt.debug_struct(stringify!(FileDescriptorProtoView));
            debug_struct
                .field(stringify!(name), &self.name_opt())
                .field(stringify!(package), &self.package_opt())
                .field(
                    stringify!(dependency),
                    &self
                        .dependency()
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>()
                        .as_slice(),
                )
                .field(
                    stringify!(public_dependency),
                    &self
                        .public_dependency()
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>()
                        .as_slice(),
                )
                .field(
                    stringify!(weak_dependency),
                    &self
                        .weak_dependency()
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>()
                        .as_slice(),
                )
                .field(
                    stringify!(message_type),
                    &self
                        .message_type()
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>()
                        .as_slice(),
                )
                .field(
                    stringify!(enum_type),
                    &self
                        .enum_type()
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>()
                        .as_slice(),
                )
                .field(
                    stringify!(service),
                    &self
                        .service()
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>()
                        .as_slice(),
                )
                .field(
                    stringify!(extension),
                    &self
                        .extension()
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>()
                        .as_slice(),
                )
                .field(stringify!(options), &self.options_opt())
                .field(stringify!(source_code_info), &self.source_code_info_opt())
                .field(stringify!(syntax), &self.syntax_opt());
            self.shared.unknown_fields().debug_struct_fields(&mut debug_struct)?;
            debug_struct.finish()
        }
    }
    impl ::std::cmp::PartialEq for FileDescriptorProtoView {
        fn eq(&self, rhs: &Self) -> bool {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::SharedItems as _;
            true && self.name_opt() == rhs.name_opt()
                && self.package_opt() == rhs.package_opt()
                && self.dependency().into_iter().eq(rhs.dependency())
                && self.public_dependency().into_iter().eq(rhs.public_dependency())
                && self.weak_dependency().into_iter().eq(rhs.weak_dependency())
                && self.message_type().into_iter().eq(rhs.message_type())
                && self.enum_type().into_iter().eq(rhs.enum_type())
                && self.service().into_iter().eq(rhs.service())
                && self.extension().into_iter().eq(rhs.extension())
                && self.options_opt() == rhs.options_opt()
                && self.source_code_info_opt() == rhs.source_code_info_opt()
                && self.syntax_opt() == rhs.syntax_opt()
                && self.shared.unknown_fields() == rhs.shared.unknown_fields()
        }
    }
    impl ::std::borrow::ToOwned for FileDescriptorProtoView {
        type Owned = self::_root::google::protobuf::FileDescriptorProto;
        fn to_owned(&self) -> Self::Owned {
            #[allow(unused)]
            use self::_pinternal::SharedItems;
            self::_root::google::protobuf::FileDescriptorProto(
                ::std::boxed::Box::new(Self {
                    fields: self::_root::google::protobuf::_fields::FileDescriptorProtoFields {
                        name: ::std::clone::Clone::clone(&self.fields.name),
                        package: ::std::clone::Clone::clone(&self.fields.package),
                        dependency: ::std::clone::Clone::clone(&self.fields.dependency),
                        public_dependency: ::std::clone::Clone::clone(
                            &self.fields.public_dependency,
                        ),
                        weak_dependency: ::std::clone::Clone::clone(
                            &self.fields.weak_dependency,
                        ),
                        message_type: ::std::clone::Clone::clone(
                            &self.fields.message_type,
                        ),
                        enum_type: ::std::clone::Clone::clone(&self.fields.enum_type),
                        service: ::std::clone::Clone::clone(&self.fields.service),
                        extension: ::std::clone::Clone::clone(&self.fields.extension),
                        options: ::std::clone::Clone::clone(&self.fields.options),
                        source_code_info: ::std::clone::Clone::clone(
                            &self.fields.source_code_info,
                        ),
                        syntax: ::std::clone::Clone::clone(&self.fields.syntax),
                    },
                    shared: ::std::clone::Clone::clone(&self.shared),
                }),
            )
        }
    }
    #[derive(::std::default::Default)]
    pub struct DescriptorProtoView {
        pub(super) fields: self::_root::google::protobuf::_fields::DescriptorProtoFields::<
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
            self::_pinternal::SingularMessageField::<
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
        pub(super) shared: self::_pinternal::SharedItemsImpl<1usize>,
    }
    impl DescriptorProtoView {
        pub fn name(&self) -> &str {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.name,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn name_opt(&self) -> ::std::option::Option::<&str> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.name,
                self.shared.bitfield(),
            )
        }
        pub fn has_name(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.name,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn field(
            &self,
        ) -> impl '_ + self::_puroro::repeated::RepeatedFieldView<
            '_,
            Item = self::_root::google::protobuf::_view::FieldDescriptorProtoView,
        > {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field(&self.fields.field, self.shared.bitfield())
        }
        pub fn extension(
            &self,
        ) -> impl '_ + self::_puroro::repeated::RepeatedFieldView<
            '_,
            Item = self::_root::google::protobuf::_view::FieldDescriptorProtoView,
        > {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field(&self.fields.extension, self.shared.bitfield())
        }
        pub fn nested_type(
            &self,
        ) -> impl '_ + self::_puroro::repeated::RepeatedFieldView<
            '_,
            Item = self::_root::google::protobuf::_view::DescriptorProtoView,
        > {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field(
                &self.fields.nested_type,
                self.shared.bitfield(),
            )
        }
        pub fn enum_type(
            &self,
        ) -> impl '_ + self::_puroro::repeated::RepeatedFieldView<
            '_,
            Item = self::_root::google::protobuf::_view::EnumDescriptorProtoView,
        > {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field(&self.fields.enum_type, self.shared.bitfield())
        }
        pub fn extension_range(
            &self,
        ) -> impl '_ + self::_puroro::repeated::RepeatedFieldView<
            '_,
            Item = self::_root::google::protobuf::descriptor_proto::_view::ExtensionRangeView,
        > {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field(
                &self.fields.extension_range,
                self.shared.bitfield(),
            )
        }
        pub fn oneof_decl(
            &self,
        ) -> impl '_ + self::_puroro::repeated::RepeatedFieldView<
            '_,
            Item = self::_root::google::protobuf::_view::OneofDescriptorProtoView,
        > {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field(&self.fields.oneof_decl, self.shared.bitfield())
        }
        pub fn options(
            &self,
        ) -> ::std::option::Option::<
            &self::_root::google::protobuf::_view::MessageOptionsView,
        > {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.options,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn options_opt(
            &self,
        ) -> ::std::option::Option::<
            &self::_root::google::protobuf::_view::MessageOptionsView,
        > {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.options,
                self.shared.bitfield(),
            )
        }
        pub fn has_options(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.options,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn reserved_range(
            &self,
        ) -> impl '_ + self::_puroro::repeated::RepeatedFieldView<
            '_,
            Item = self::_root::google::protobuf::descriptor_proto::_view::ReservedRangeView,
        > {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field(
                &self.fields.reserved_range,
                self.shared.bitfield(),
            )
        }
        /** Reserved field names, which may not be used by fields in the same message.
 A given name may only be reserved once.
*/
        pub fn reserved_name(
            &self,
        ) -> impl '_ + self::_puroro::repeated::RepeatedFieldView<'_, Item = str> {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field(
                &self.fields.reserved_name,
                self.shared.bitfield(),
            )
        }
    }
    impl self::_puroro::MessageView for self::DescriptorProtoView {
        type MessageType = self::_root::google::protobuf::DescriptorProto;
        fn to_bytes<W: ::std::io::Write>(
            &self,
            #[allow(unused)]
            out: &mut W,
        ) -> self::_puroro::Result<()> {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.name,
                self.shared.bitfield(),
                1i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.field,
                self.shared.bitfield(),
                2i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.extension,
                self.shared.bitfield(),
                6i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.nested_type,
                self.shared.bitfield(),
                3i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.enum_type,
                self.shared.bitfield(),
                4i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.extension_range,
                self.shared.bitfield(),
                5i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.oneof_decl,
                self.shared.bitfield(),
                8i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.options,
                self.shared.bitfield(),
                7i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.reserved_range,
                self.shared.bitfield(),
                9i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.reserved_name,
                self.shared.bitfield(),
                10i32,
                out,
            )?;
            self.shared.unknown_fields().ser_to_write(out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl self::_pinternal::MessageViewInternal for self::DescriptorProtoView {
        fn new_boxed() -> ::std::boxed::Box<Self> {
            use self::_pinternal::SharedItems as _;
            let mut shared: self::_pinternal::SharedItemsImpl::<1usize> = ::std::default::Default::default();
            let fields = self::_root::google::protobuf::_fields::DescriptorProtoFields {
                name: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                field: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                extension: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                nested_type: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                enum_type: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                extension_range: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                oneof_decl: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                options: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                reserved_range: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                reserved_name: self::_pinternal::FieldType::new(shared.bitfield_mut()),
            };
            ::std::boxed::Box::new(Self { fields, shared })
        }
    }
    impl ::std::ops::Drop for DescriptorProtoView {
        fn drop(&mut self) {
            #[allow(unused)]
            use self::_pinternal::{OneofUnion as _, SharedItems as _};
        }
    }
    impl ::std::fmt::Debug for DescriptorProtoView {
        fn fmt(
            &self,
            fmt: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            let mut debug_struct = fmt.debug_struct(stringify!(DescriptorProtoView));
            debug_struct
                .field(stringify!(name), &self.name_opt())
                .field(
                    stringify!(field),
                    &self.field().into_iter().collect::<::std::vec::Vec<_>>().as_slice(),
                )
                .field(
                    stringify!(extension),
                    &self
                        .extension()
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>()
                        .as_slice(),
                )
                .field(
                    stringify!(nested_type),
                    &self
                        .nested_type()
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>()
                        .as_slice(),
                )
                .field(
                    stringify!(enum_type),
                    &self
                        .enum_type()
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>()
                        .as_slice(),
                )
                .field(
                    stringify!(extension_range),
                    &self
                        .extension_range()
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>()
                        .as_slice(),
                )
                .field(
                    stringify!(oneof_decl),
                    &self
                        .oneof_decl()
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>()
                        .as_slice(),
                )
                .field(stringify!(options), &self.options_opt())
                .field(
                    stringify!(reserved_range),
                    &self
                        .reserved_range()
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>()
                        .as_slice(),
                )
                .field(
                    stringify!(reserved_name),
                    &self
                        .reserved_name()
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>()
                        .as_slice(),
                );
            self.shared.unknown_fields().debug_struct_fields(&mut debug_struct)?;
            debug_struct.finish()
        }
    }
    impl ::std::cmp::PartialEq for DescriptorProtoView {
        fn eq(&self, rhs: &Self) -> bool {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::SharedItems as _;
            true && self.name_opt() == rhs.name_opt()
                && self.field().into_iter().eq(rhs.field())
                && self.extension().into_iter().eq(rhs.extension())
                && self.nested_type().into_iter().eq(rhs.nested_type())
                && self.enum_type().into_iter().eq(rhs.enum_type())
                && self.extension_range().into_iter().eq(rhs.extension_range())
                && self.oneof_decl().into_iter().eq(rhs.oneof_decl())
                && self.options_opt() == rhs.options_opt()
                && self.reserved_range().into_iter().eq(rhs.reserved_range())
                && self.reserved_name().into_iter().eq(rhs.reserved_name())
                && self.shared.unknown_fields() == rhs.shared.unknown_fields()
        }
    }
    impl ::std::borrow::ToOwned for DescriptorProtoView {
        type Owned = self::_root::google::protobuf::DescriptorProto;
        fn to_owned(&self) -> Self::Owned {
            #[allow(unused)]
            use self::_pinternal::SharedItems;
            self::_root::google::protobuf::DescriptorProto(
                ::std::boxed::Box::new(Self {
                    fields: self::_root::google::protobuf::_fields::DescriptorProtoFields {
                        name: ::std::clone::Clone::clone(&self.fields.name),
                        field: ::std::clone::Clone::clone(&self.fields.field),
                        extension: ::std::clone::Clone::clone(&self.fields.extension),
                        nested_type: ::std::clone::Clone::clone(
                            &self.fields.nested_type,
                        ),
                        enum_type: ::std::clone::Clone::clone(&self.fields.enum_type),
                        extension_range: ::std::clone::Clone::clone(
                            &self.fields.extension_range,
                        ),
                        oneof_decl: ::std::clone::Clone::clone(&self.fields.oneof_decl),
                        options: ::std::clone::Clone::clone(&self.fields.options),
                        reserved_range: ::std::clone::Clone::clone(
                            &self.fields.reserved_range,
                        ),
                        reserved_name: ::std::clone::Clone::clone(
                            &self.fields.reserved_name,
                        ),
                    },
                    shared: ::std::clone::Clone::clone(&self.shared),
                }),
            )
        }
    }
    #[derive(::std::default::Default)]
    pub struct ExtensionRangeOptionsView {
        pub(super) fields: self::_root::google::protobuf::_fields::ExtensionRangeOptionsFields::<
            self::_pinternal::RepeatedMessageField::<
                self::_root::google::protobuf::UninterpretedOption,
            >,
        >,
        pub(super) shared: self::_pinternal::SharedItemsImpl<0usize>,
    }
    impl ExtensionRangeOptionsView {
        /** The parser stores options it doesn't recognize here. See above.
*/
        pub fn uninterpreted_option(
            &self,
        ) -> impl '_ + self::_puroro::repeated::RepeatedFieldView<
            '_,
            Item = self::_root::google::protobuf::_view::UninterpretedOptionView,
        > {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field(
                &self.fields.uninterpreted_option,
                self.shared.bitfield(),
            )
        }
    }
    impl self::_puroro::MessageView for self::ExtensionRangeOptionsView {
        type MessageType = self::_root::google::protobuf::ExtensionRangeOptions;
        fn to_bytes<W: ::std::io::Write>(
            &self,
            #[allow(unused)]
            out: &mut W,
        ) -> self::_puroro::Result<()> {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.uninterpreted_option,
                self.shared.bitfield(),
                999i32,
                out,
            )?;
            self.shared.unknown_fields().ser_to_write(out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl self::_pinternal::MessageViewInternal for self::ExtensionRangeOptionsView {
        fn new_boxed() -> ::std::boxed::Box<Self> {
            use self::_pinternal::SharedItems as _;
            let mut shared: self::_pinternal::SharedItemsImpl::<0usize> = ::std::default::Default::default();
            let fields = self::_root::google::protobuf::_fields::ExtensionRangeOptionsFields {
                uninterpreted_option: self::_pinternal::FieldType::new(
                    shared.bitfield_mut(),
                ),
            };
            ::std::boxed::Box::new(Self { fields, shared })
        }
    }
    impl ::std::ops::Drop for ExtensionRangeOptionsView {
        fn drop(&mut self) {
            #[allow(unused)]
            use self::_pinternal::{OneofUnion as _, SharedItems as _};
        }
    }
    impl ::std::fmt::Debug for ExtensionRangeOptionsView {
        fn fmt(
            &self,
            fmt: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            let mut debug_struct = fmt
                .debug_struct(stringify!(ExtensionRangeOptionsView));
            debug_struct
                .field(
                    stringify!(uninterpreted_option),
                    &self
                        .uninterpreted_option()
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>()
                        .as_slice(),
                );
            self.shared.unknown_fields().debug_struct_fields(&mut debug_struct)?;
            debug_struct.finish()
        }
    }
    impl ::std::cmp::PartialEq for ExtensionRangeOptionsView {
        fn eq(&self, rhs: &Self) -> bool {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::SharedItems as _;
            true
                && self.uninterpreted_option().into_iter().eq(rhs.uninterpreted_option())
                && self.shared.unknown_fields() == rhs.shared.unknown_fields()
        }
    }
    impl ::std::borrow::ToOwned for ExtensionRangeOptionsView {
        type Owned = self::_root::google::protobuf::ExtensionRangeOptions;
        fn to_owned(&self) -> Self::Owned {
            #[allow(unused)]
            use self::_pinternal::SharedItems;
            self::_root::google::protobuf::ExtensionRangeOptions(
                ::std::boxed::Box::new(Self {
                    fields: self::_root::google::protobuf::_fields::ExtensionRangeOptionsFields {
                        uninterpreted_option: ::std::clone::Clone::clone(
                            &self.fields.uninterpreted_option,
                        ),
                    },
                    shared: ::std::clone::Clone::clone(&self.shared),
                }),
            )
        }
    }
    #[derive(::std::default::Default)]
    pub struct FieldDescriptorProtoView {
        pub(super) fields: self::_root::google::protobuf::_fields::FieldDescriptorProtoFields::<
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
            self::_pinternal::SingularMessageField::<
                self::_root::google::protobuf::FieldOptions,
            >,
            self::_pinternal::OptionalNumericalField::<
                bool,
                self::_pinternal::tags::Bool,
                9usize,
            >,
        >,
        pub(super) shared: self::_pinternal::SharedItemsImpl<1usize>,
    }
    impl FieldDescriptorProtoView {
        pub fn name(&self) -> &str {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.name,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn name_opt(&self) -> ::std::option::Option::<&str> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.name,
                self.shared.bitfield(),
            )
        }
        pub fn has_name(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.name,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn number(&self) -> i32 {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.number,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn number_opt(&self) -> ::std::option::Option::<i32> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.number,
                self.shared.bitfield(),
            )
        }
        pub fn has_number(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.number,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn label(
            &self,
        ) -> self::_root::google::protobuf::field_descriptor_proto::Label {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.label,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn label_opt(
            &self,
        ) -> ::std::option::Option::<
            self::_root::google::protobuf::field_descriptor_proto::Label,
        > {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.label,
                self.shared.bitfield(),
            )
        }
        pub fn has_label(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.label,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn r#type(
            &self,
        ) -> self::_root::google::protobuf::field_descriptor_proto::Type {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.r#type,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        /** If type_name is set, this need not be set.  If both this and type_name
 are set, this must be one of TYPE_ENUM, TYPE_MESSAGE or TYPE_GROUP.
*/
        pub fn type_opt(
            &self,
        ) -> ::std::option::Option::<
            self::_root::google::protobuf::field_descriptor_proto::Type,
        > {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.r#type,
                self.shared.bitfield(),
            )
        }
        pub fn has_type(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.r#type,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn type_name(&self) -> &str {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.type_name,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        /** For message and enum types, this is the name of the type.  If the name
 starts with a '.', it is fully-qualified.  Otherwise, C++-like scoping
 rules are used to find the type (i.e. first the nested types within this
 message are searched, then within the parent, on up to the root
 namespace).
*/
        pub fn type_name_opt(&self) -> ::std::option::Option::<&str> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.type_name,
                self.shared.bitfield(),
            )
        }
        pub fn has_type_name(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.type_name,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn extendee(&self) -> &str {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.extendee,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        /** For extensions, this is the name of the type being extended.  It is
 resolved in the same manner as type_name.
*/
        pub fn extendee_opt(&self) -> ::std::option::Option::<&str> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.extendee,
                self.shared.bitfield(),
            )
        }
        pub fn has_extendee(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.extendee,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn default_value(&self) -> &str {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.default_value,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        /** For numeric types, contains the original text representation of the value.
 For booleans, "true" or "false".
 For strings, contains the default text contents (not escaped in any way).
 For bytes, contains the C escaped value.  All bytes >= 128 are escaped.
 TODO(kenton):  Base-64 encode?
*/
        pub fn default_value_opt(&self) -> ::std::option::Option::<&str> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.default_value,
                self.shared.bitfield(),
            )
        }
        pub fn has_default_value(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.default_value,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn oneof_index(&self) -> i32 {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.oneof_index,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        /** If set, gives the index of a oneof in the containing type's oneof_decl
 list.  This field is a member of that oneof.
*/
        pub fn oneof_index_opt(&self) -> ::std::option::Option::<i32> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.oneof_index,
                self.shared.bitfield(),
            )
        }
        pub fn has_oneof_index(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.oneof_index,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn json_name(&self) -> &str {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.json_name,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        /** JSON name of this field. The value is set by protocol compiler. If the
 user has set a "json_name" option on this field, that option's value
 will be used. Otherwise, it's deduced from the field's name by converting
 it to camelCase.
*/
        pub fn json_name_opt(&self) -> ::std::option::Option::<&str> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.json_name,
                self.shared.bitfield(),
            )
        }
        pub fn has_json_name(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.json_name,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn options(
            &self,
        ) -> ::std::option::Option::<
            &self::_root::google::protobuf::_view::FieldOptionsView,
        > {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.options,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn options_opt(
            &self,
        ) -> ::std::option::Option::<
            &self::_root::google::protobuf::_view::FieldOptionsView,
        > {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.options,
                self.shared.bitfield(),
            )
        }
        pub fn has_options(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.options,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn proto3_optional(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.proto3_optional,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        /** If true, this is a proto3 "optional". When a proto3 field is optional, it
 tracks presence regardless of field type.

 When proto3_optional is true, this field must be belong to a oneof to
 signal to old proto3 clients that presence is tracked for this field. This
 oneof is known as a "synthetic" oneof, and this field must be its sole
 member (each proto3 optional field gets its own synthetic oneof). Synthetic
 oneofs exist in the descriptor only, and do not generate any API. Synthetic
 oneofs must be ordered after all "real" oneofs.

 For message fields, proto3_optional doesn't create any semantic change,
 since non-repeated message fields always track presence. However it still
 indicates the semantic detail of whether the user wrote "optional" or not.
 This can be useful for round-tripping the .proto file. For consistency we
 give message fields a synthetic oneof also, even though it is not required
 to track presence. This is especially important because the parser can't
 tell if a field is a message or an enum, so it must always create a
 synthetic oneof.

 Proto2 optional fields do not set this flag, because they already indicate
 optional with `LABEL_OPTIONAL`.
*/
        pub fn proto3_optional_opt(&self) -> ::std::option::Option::<bool> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.proto3_optional,
                self.shared.bitfield(),
            )
        }
        pub fn has_proto3_optional(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.proto3_optional,
                    self.shared.bitfield(),
                )
                .is_some()
        }
    }
    impl self::_puroro::MessageView for self::FieldDescriptorProtoView {
        type MessageType = self::_root::google::protobuf::FieldDescriptorProto;
        fn to_bytes<W: ::std::io::Write>(
            &self,
            #[allow(unused)]
            out: &mut W,
        ) -> self::_puroro::Result<()> {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.name,
                self.shared.bitfield(),
                1i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.number,
                self.shared.bitfield(),
                3i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.label,
                self.shared.bitfield(),
                4i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.r#type,
                self.shared.bitfield(),
                5i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.type_name,
                self.shared.bitfield(),
                6i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.extendee,
                self.shared.bitfield(),
                2i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.default_value,
                self.shared.bitfield(),
                7i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.oneof_index,
                self.shared.bitfield(),
                9i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.json_name,
                self.shared.bitfield(),
                10i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.options,
                self.shared.bitfield(),
                8i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.proto3_optional,
                self.shared.bitfield(),
                17i32,
                out,
            )?;
            self.shared.unknown_fields().ser_to_write(out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl self::_pinternal::MessageViewInternal for self::FieldDescriptorProtoView {
        fn new_boxed() -> ::std::boxed::Box<Self> {
            use self::_pinternal::SharedItems as _;
            let mut shared: self::_pinternal::SharedItemsImpl::<1usize> = ::std::default::Default::default();
            let fields = self::_root::google::protobuf::_fields::FieldDescriptorProtoFields {
                name: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                number: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                label: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                r#type: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                type_name: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                extendee: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                default_value: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                oneof_index: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                json_name: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                options: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                proto3_optional: self::_pinternal::FieldType::new(shared.bitfield_mut()),
            };
            ::std::boxed::Box::new(Self { fields, shared })
        }
    }
    impl ::std::ops::Drop for FieldDescriptorProtoView {
        fn drop(&mut self) {
            #[allow(unused)]
            use self::_pinternal::{OneofUnion as _, SharedItems as _};
        }
    }
    impl ::std::fmt::Debug for FieldDescriptorProtoView {
        fn fmt(
            &self,
            fmt: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            let mut debug_struct = fmt
                .debug_struct(stringify!(FieldDescriptorProtoView));
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
            self.shared.unknown_fields().debug_struct_fields(&mut debug_struct)?;
            debug_struct.finish()
        }
    }
    impl ::std::cmp::PartialEq for FieldDescriptorProtoView {
        fn eq(&self, rhs: &Self) -> bool {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::SharedItems as _;
            true && self.name_opt() == rhs.name_opt()
                && self.number_opt() == rhs.number_opt()
                && self.label_opt() == rhs.label_opt()
                && self.type_opt() == rhs.type_opt()
                && self.type_name_opt() == rhs.type_name_opt()
                && self.extendee_opt() == rhs.extendee_opt()
                && self.default_value_opt() == rhs.default_value_opt()
                && self.oneof_index_opt() == rhs.oneof_index_opt()
                && self.json_name_opt() == rhs.json_name_opt()
                && self.options_opt() == rhs.options_opt()
                && self.proto3_optional_opt() == rhs.proto3_optional_opt()
                && self.shared.unknown_fields() == rhs.shared.unknown_fields()
        }
    }
    impl ::std::borrow::ToOwned for FieldDescriptorProtoView {
        type Owned = self::_root::google::protobuf::FieldDescriptorProto;
        fn to_owned(&self) -> Self::Owned {
            #[allow(unused)]
            use self::_pinternal::SharedItems;
            self::_root::google::protobuf::FieldDescriptorProto(
                ::std::boxed::Box::new(Self {
                    fields: self::_root::google::protobuf::_fields::FieldDescriptorProtoFields {
                        name: ::std::clone::Clone::clone(&self.fields.name),
                        number: ::std::clone::Clone::clone(&self.fields.number),
                        label: ::std::clone::Clone::clone(&self.fields.label),
                        r#type: ::std::clone::Clone::clone(&self.fields.r#type),
                        type_name: ::std::clone::Clone::clone(&self.fields.type_name),
                        extendee: ::std::clone::Clone::clone(&self.fields.extendee),
                        default_value: ::std::clone::Clone::clone(
                            &self.fields.default_value,
                        ),
                        oneof_index: ::std::clone::Clone::clone(
                            &self.fields.oneof_index,
                        ),
                        json_name: ::std::clone::Clone::clone(&self.fields.json_name),
                        options: ::std::clone::Clone::clone(&self.fields.options),
                        proto3_optional: ::std::clone::Clone::clone(
                            &self.fields.proto3_optional,
                        ),
                    },
                    shared: ::std::clone::Clone::clone(&self.shared),
                }),
            )
        }
    }
    #[derive(::std::default::Default)]
    pub struct OneofDescriptorProtoView {
        pub(super) fields: self::_root::google::protobuf::_fields::OneofDescriptorProtoFields::<
            self::_pinternal::OptionalUnsizedField::<
                ::std::string::String,
                self::_pinternal::tags::String,
                0usize,
            >,
            self::_pinternal::SingularMessageField::<
                self::_root::google::protobuf::OneofOptions,
            >,
        >,
        pub(super) shared: self::_pinternal::SharedItemsImpl<1usize>,
    }
    impl OneofDescriptorProtoView {
        pub fn name(&self) -> &str {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.name,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn name_opt(&self) -> ::std::option::Option::<&str> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.name,
                self.shared.bitfield(),
            )
        }
        pub fn has_name(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.name,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn options(
            &self,
        ) -> ::std::option::Option::<
            &self::_root::google::protobuf::_view::OneofOptionsView,
        > {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.options,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn options_opt(
            &self,
        ) -> ::std::option::Option::<
            &self::_root::google::protobuf::_view::OneofOptionsView,
        > {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.options,
                self.shared.bitfield(),
            )
        }
        pub fn has_options(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.options,
                    self.shared.bitfield(),
                )
                .is_some()
        }
    }
    impl self::_puroro::MessageView for self::OneofDescriptorProtoView {
        type MessageType = self::_root::google::protobuf::OneofDescriptorProto;
        fn to_bytes<W: ::std::io::Write>(
            &self,
            #[allow(unused)]
            out: &mut W,
        ) -> self::_puroro::Result<()> {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.name,
                self.shared.bitfield(),
                1i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.options,
                self.shared.bitfield(),
                2i32,
                out,
            )?;
            self.shared.unknown_fields().ser_to_write(out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl self::_pinternal::MessageViewInternal for self::OneofDescriptorProtoView {
        fn new_boxed() -> ::std::boxed::Box<Self> {
            use self::_pinternal::SharedItems as _;
            let mut shared: self::_pinternal::SharedItemsImpl::<1usize> = ::std::default::Default::default();
            let fields = self::_root::google::protobuf::_fields::OneofDescriptorProtoFields {
                name: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                options: self::_pinternal::FieldType::new(shared.bitfield_mut()),
            };
            ::std::boxed::Box::new(Self { fields, shared })
        }
    }
    impl ::std::ops::Drop for OneofDescriptorProtoView {
        fn drop(&mut self) {
            #[allow(unused)]
            use self::_pinternal::{OneofUnion as _, SharedItems as _};
        }
    }
    impl ::std::fmt::Debug for OneofDescriptorProtoView {
        fn fmt(
            &self,
            fmt: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            let mut debug_struct = fmt
                .debug_struct(stringify!(OneofDescriptorProtoView));
            debug_struct
                .field(stringify!(name), &self.name_opt())
                .field(stringify!(options), &self.options_opt());
            self.shared.unknown_fields().debug_struct_fields(&mut debug_struct)?;
            debug_struct.finish()
        }
    }
    impl ::std::cmp::PartialEq for OneofDescriptorProtoView {
        fn eq(&self, rhs: &Self) -> bool {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::SharedItems as _;
            true && self.name_opt() == rhs.name_opt()
                && self.options_opt() == rhs.options_opt()
                && self.shared.unknown_fields() == rhs.shared.unknown_fields()
        }
    }
    impl ::std::borrow::ToOwned for OneofDescriptorProtoView {
        type Owned = self::_root::google::protobuf::OneofDescriptorProto;
        fn to_owned(&self) -> Self::Owned {
            #[allow(unused)]
            use self::_pinternal::SharedItems;
            self::_root::google::protobuf::OneofDescriptorProto(
                ::std::boxed::Box::new(Self {
                    fields: self::_root::google::protobuf::_fields::OneofDescriptorProtoFields {
                        name: ::std::clone::Clone::clone(&self.fields.name),
                        options: ::std::clone::Clone::clone(&self.fields.options),
                    },
                    shared: ::std::clone::Clone::clone(&self.shared),
                }),
            )
        }
    }
    #[derive(::std::default::Default)]
    pub struct EnumDescriptorProtoView {
        pub(super) fields: self::_root::google::protobuf::_fields::EnumDescriptorProtoFields::<
            self::_pinternal::OptionalUnsizedField::<
                ::std::string::String,
                self::_pinternal::tags::String,
                0usize,
            >,
            self::_pinternal::RepeatedMessageField::<
                self::_root::google::protobuf::EnumValueDescriptorProto,
            >,
            self::_pinternal::SingularMessageField::<
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
        pub(super) shared: self::_pinternal::SharedItemsImpl<1usize>,
    }
    impl EnumDescriptorProtoView {
        pub fn name(&self) -> &str {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.name,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn name_opt(&self) -> ::std::option::Option::<&str> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.name,
                self.shared.bitfield(),
            )
        }
        pub fn has_name(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.name,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn value(
            &self,
        ) -> impl '_ + self::_puroro::repeated::RepeatedFieldView<
            '_,
            Item = self::_root::google::protobuf::_view::EnumValueDescriptorProtoView,
        > {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field(&self.fields.value, self.shared.bitfield())
        }
        pub fn options(
            &self,
        ) -> ::std::option::Option::<
            &self::_root::google::protobuf::_view::EnumOptionsView,
        > {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.options,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn options_opt(
            &self,
        ) -> ::std::option::Option::<
            &self::_root::google::protobuf::_view::EnumOptionsView,
        > {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.options,
                self.shared.bitfield(),
            )
        }
        pub fn has_options(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.options,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        /** Range of reserved numeric values. Reserved numeric values may not be used
 by enum values in the same enum declaration. Reserved ranges may not
 overlap.
*/
        pub fn reserved_range(
            &self,
        ) -> impl '_ + self::_puroro::repeated::RepeatedFieldView<
            '_,
            Item = self::_root::google::protobuf::enum_descriptor_proto::_view::EnumReservedRangeView,
        > {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field(
                &self.fields.reserved_range,
                self.shared.bitfield(),
            )
        }
        /** Reserved enum value names, which may not be reused. A given name may only
 be reserved once.
*/
        pub fn reserved_name(
            &self,
        ) -> impl '_ + self::_puroro::repeated::RepeatedFieldView<'_, Item = str> {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field(
                &self.fields.reserved_name,
                self.shared.bitfield(),
            )
        }
    }
    impl self::_puroro::MessageView for self::EnumDescriptorProtoView {
        type MessageType = self::_root::google::protobuf::EnumDescriptorProto;
        fn to_bytes<W: ::std::io::Write>(
            &self,
            #[allow(unused)]
            out: &mut W,
        ) -> self::_puroro::Result<()> {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.name,
                self.shared.bitfield(),
                1i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.value,
                self.shared.bitfield(),
                2i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.options,
                self.shared.bitfield(),
                3i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.reserved_range,
                self.shared.bitfield(),
                4i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.reserved_name,
                self.shared.bitfield(),
                5i32,
                out,
            )?;
            self.shared.unknown_fields().ser_to_write(out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl self::_pinternal::MessageViewInternal for self::EnumDescriptorProtoView {
        fn new_boxed() -> ::std::boxed::Box<Self> {
            use self::_pinternal::SharedItems as _;
            let mut shared: self::_pinternal::SharedItemsImpl::<1usize> = ::std::default::Default::default();
            let fields = self::_root::google::protobuf::_fields::EnumDescriptorProtoFields {
                name: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                value: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                options: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                reserved_range: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                reserved_name: self::_pinternal::FieldType::new(shared.bitfield_mut()),
            };
            ::std::boxed::Box::new(Self { fields, shared })
        }
    }
    impl ::std::ops::Drop for EnumDescriptorProtoView {
        fn drop(&mut self) {
            #[allow(unused)]
            use self::_pinternal::{OneofUnion as _, SharedItems as _};
        }
    }
    impl ::std::fmt::Debug for EnumDescriptorProtoView {
        fn fmt(
            &self,
            fmt: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            let mut debug_struct = fmt.debug_struct(stringify!(EnumDescriptorProtoView));
            debug_struct
                .field(stringify!(name), &self.name_opt())
                .field(
                    stringify!(value),
                    &self.value().into_iter().collect::<::std::vec::Vec<_>>().as_slice(),
                )
                .field(stringify!(options), &self.options_opt())
                .field(
                    stringify!(reserved_range),
                    &self
                        .reserved_range()
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>()
                        .as_slice(),
                )
                .field(
                    stringify!(reserved_name),
                    &self
                        .reserved_name()
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>()
                        .as_slice(),
                );
            self.shared.unknown_fields().debug_struct_fields(&mut debug_struct)?;
            debug_struct.finish()
        }
    }
    impl ::std::cmp::PartialEq for EnumDescriptorProtoView {
        fn eq(&self, rhs: &Self) -> bool {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::SharedItems as _;
            true && self.name_opt() == rhs.name_opt()
                && self.value().into_iter().eq(rhs.value())
                && self.options_opt() == rhs.options_opt()
                && self.reserved_range().into_iter().eq(rhs.reserved_range())
                && self.reserved_name().into_iter().eq(rhs.reserved_name())
                && self.shared.unknown_fields() == rhs.shared.unknown_fields()
        }
    }
    impl ::std::borrow::ToOwned for EnumDescriptorProtoView {
        type Owned = self::_root::google::protobuf::EnumDescriptorProto;
        fn to_owned(&self) -> Self::Owned {
            #[allow(unused)]
            use self::_pinternal::SharedItems;
            self::_root::google::protobuf::EnumDescriptorProto(
                ::std::boxed::Box::new(Self {
                    fields: self::_root::google::protobuf::_fields::EnumDescriptorProtoFields {
                        name: ::std::clone::Clone::clone(&self.fields.name),
                        value: ::std::clone::Clone::clone(&self.fields.value),
                        options: ::std::clone::Clone::clone(&self.fields.options),
                        reserved_range: ::std::clone::Clone::clone(
                            &self.fields.reserved_range,
                        ),
                        reserved_name: ::std::clone::Clone::clone(
                            &self.fields.reserved_name,
                        ),
                    },
                    shared: ::std::clone::Clone::clone(&self.shared),
                }),
            )
        }
    }
    #[derive(::std::default::Default)]
    pub struct EnumValueDescriptorProtoView {
        pub(super) fields: self::_root::google::protobuf::_fields::EnumValueDescriptorProtoFields::<
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
            self::_pinternal::SingularMessageField::<
                self::_root::google::protobuf::EnumValueOptions,
            >,
        >,
        pub(super) shared: self::_pinternal::SharedItemsImpl<1usize>,
    }
    impl EnumValueDescriptorProtoView {
        pub fn name(&self) -> &str {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.name,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn name_opt(&self) -> ::std::option::Option::<&str> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.name,
                self.shared.bitfield(),
            )
        }
        pub fn has_name(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.name,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn number(&self) -> i32 {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.number,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn number_opt(&self) -> ::std::option::Option::<i32> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.number,
                self.shared.bitfield(),
            )
        }
        pub fn has_number(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.number,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn options(
            &self,
        ) -> ::std::option::Option::<
            &self::_root::google::protobuf::_view::EnumValueOptionsView,
        > {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.options,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn options_opt(
            &self,
        ) -> ::std::option::Option::<
            &self::_root::google::protobuf::_view::EnumValueOptionsView,
        > {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.options,
                self.shared.bitfield(),
            )
        }
        pub fn has_options(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.options,
                    self.shared.bitfield(),
                )
                .is_some()
        }
    }
    impl self::_puroro::MessageView for self::EnumValueDescriptorProtoView {
        type MessageType = self::_root::google::protobuf::EnumValueDescriptorProto;
        fn to_bytes<W: ::std::io::Write>(
            &self,
            #[allow(unused)]
            out: &mut W,
        ) -> self::_puroro::Result<()> {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.name,
                self.shared.bitfield(),
                1i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.number,
                self.shared.bitfield(),
                2i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.options,
                self.shared.bitfield(),
                3i32,
                out,
            )?;
            self.shared.unknown_fields().ser_to_write(out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl self::_pinternal::MessageViewInternal for self::EnumValueDescriptorProtoView {
        fn new_boxed() -> ::std::boxed::Box<Self> {
            use self::_pinternal::SharedItems as _;
            let mut shared: self::_pinternal::SharedItemsImpl::<1usize> = ::std::default::Default::default();
            let fields = self::_root::google::protobuf::_fields::EnumValueDescriptorProtoFields {
                name: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                number: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                options: self::_pinternal::FieldType::new(shared.bitfield_mut()),
            };
            ::std::boxed::Box::new(Self { fields, shared })
        }
    }
    impl ::std::ops::Drop for EnumValueDescriptorProtoView {
        fn drop(&mut self) {
            #[allow(unused)]
            use self::_pinternal::{OneofUnion as _, SharedItems as _};
        }
    }
    impl ::std::fmt::Debug for EnumValueDescriptorProtoView {
        fn fmt(
            &self,
            fmt: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            let mut debug_struct = fmt
                .debug_struct(stringify!(EnumValueDescriptorProtoView));
            debug_struct
                .field(stringify!(name), &self.name_opt())
                .field(stringify!(number), &self.number_opt())
                .field(stringify!(options), &self.options_opt());
            self.shared.unknown_fields().debug_struct_fields(&mut debug_struct)?;
            debug_struct.finish()
        }
    }
    impl ::std::cmp::PartialEq for EnumValueDescriptorProtoView {
        fn eq(&self, rhs: &Self) -> bool {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::SharedItems as _;
            true && self.name_opt() == rhs.name_opt()
                && self.number_opt() == rhs.number_opt()
                && self.options_opt() == rhs.options_opt()
                && self.shared.unknown_fields() == rhs.shared.unknown_fields()
        }
    }
    impl ::std::borrow::ToOwned for EnumValueDescriptorProtoView {
        type Owned = self::_root::google::protobuf::EnumValueDescriptorProto;
        fn to_owned(&self) -> Self::Owned {
            #[allow(unused)]
            use self::_pinternal::SharedItems;
            self::_root::google::protobuf::EnumValueDescriptorProto(
                ::std::boxed::Box::new(Self {
                    fields: self::_root::google::protobuf::_fields::EnumValueDescriptorProtoFields {
                        name: ::std::clone::Clone::clone(&self.fields.name),
                        number: ::std::clone::Clone::clone(&self.fields.number),
                        options: ::std::clone::Clone::clone(&self.fields.options),
                    },
                    shared: ::std::clone::Clone::clone(&self.shared),
                }),
            )
        }
    }
    #[derive(::std::default::Default)]
    pub struct ServiceDescriptorProtoView {
        pub(super) fields: self::_root::google::protobuf::_fields::ServiceDescriptorProtoFields::<
            self::_pinternal::OptionalUnsizedField::<
                ::std::string::String,
                self::_pinternal::tags::String,
                0usize,
            >,
            self::_pinternal::RepeatedMessageField::<
                self::_root::google::protobuf::MethodDescriptorProto,
            >,
            self::_pinternal::SingularMessageField::<
                self::_root::google::protobuf::ServiceOptions,
            >,
        >,
        pub(super) shared: self::_pinternal::SharedItemsImpl<1usize>,
    }
    impl ServiceDescriptorProtoView {
        pub fn name(&self) -> &str {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.name,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn name_opt(&self) -> ::std::option::Option::<&str> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.name,
                self.shared.bitfield(),
            )
        }
        pub fn has_name(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.name,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn method(
            &self,
        ) -> impl '_ + self::_puroro::repeated::RepeatedFieldView<
            '_,
            Item = self::_root::google::protobuf::_view::MethodDescriptorProtoView,
        > {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field(&self.fields.method, self.shared.bitfield())
        }
        pub fn options(
            &self,
        ) -> ::std::option::Option::<
            &self::_root::google::protobuf::_view::ServiceOptionsView,
        > {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.options,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn options_opt(
            &self,
        ) -> ::std::option::Option::<
            &self::_root::google::protobuf::_view::ServiceOptionsView,
        > {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.options,
                self.shared.bitfield(),
            )
        }
        pub fn has_options(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.options,
                    self.shared.bitfield(),
                )
                .is_some()
        }
    }
    impl self::_puroro::MessageView for self::ServiceDescriptorProtoView {
        type MessageType = self::_root::google::protobuf::ServiceDescriptorProto;
        fn to_bytes<W: ::std::io::Write>(
            &self,
            #[allow(unused)]
            out: &mut W,
        ) -> self::_puroro::Result<()> {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.name,
                self.shared.bitfield(),
                1i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.method,
                self.shared.bitfield(),
                2i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.options,
                self.shared.bitfield(),
                3i32,
                out,
            )?;
            self.shared.unknown_fields().ser_to_write(out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl self::_pinternal::MessageViewInternal for self::ServiceDescriptorProtoView {
        fn new_boxed() -> ::std::boxed::Box<Self> {
            use self::_pinternal::SharedItems as _;
            let mut shared: self::_pinternal::SharedItemsImpl::<1usize> = ::std::default::Default::default();
            let fields = self::_root::google::protobuf::_fields::ServiceDescriptorProtoFields {
                name: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                method: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                options: self::_pinternal::FieldType::new(shared.bitfield_mut()),
            };
            ::std::boxed::Box::new(Self { fields, shared })
        }
    }
    impl ::std::ops::Drop for ServiceDescriptorProtoView {
        fn drop(&mut self) {
            #[allow(unused)]
            use self::_pinternal::{OneofUnion as _, SharedItems as _};
        }
    }
    impl ::std::fmt::Debug for ServiceDescriptorProtoView {
        fn fmt(
            &self,
            fmt: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            let mut debug_struct = fmt
                .debug_struct(stringify!(ServiceDescriptorProtoView));
            debug_struct
                .field(stringify!(name), &self.name_opt())
                .field(
                    stringify!(method),
                    &self.method().into_iter().collect::<::std::vec::Vec<_>>().as_slice(),
                )
                .field(stringify!(options), &self.options_opt());
            self.shared.unknown_fields().debug_struct_fields(&mut debug_struct)?;
            debug_struct.finish()
        }
    }
    impl ::std::cmp::PartialEq for ServiceDescriptorProtoView {
        fn eq(&self, rhs: &Self) -> bool {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::SharedItems as _;
            true && self.name_opt() == rhs.name_opt()
                && self.method().into_iter().eq(rhs.method())
                && self.options_opt() == rhs.options_opt()
                && self.shared.unknown_fields() == rhs.shared.unknown_fields()
        }
    }
    impl ::std::borrow::ToOwned for ServiceDescriptorProtoView {
        type Owned = self::_root::google::protobuf::ServiceDescriptorProto;
        fn to_owned(&self) -> Self::Owned {
            #[allow(unused)]
            use self::_pinternal::SharedItems;
            self::_root::google::protobuf::ServiceDescriptorProto(
                ::std::boxed::Box::new(Self {
                    fields: self::_root::google::protobuf::_fields::ServiceDescriptorProtoFields {
                        name: ::std::clone::Clone::clone(&self.fields.name),
                        method: ::std::clone::Clone::clone(&self.fields.method),
                        options: ::std::clone::Clone::clone(&self.fields.options),
                    },
                    shared: ::std::clone::Clone::clone(&self.shared),
                }),
            )
        }
    }
    #[derive(::std::default::Default)]
    pub struct MethodDescriptorProtoView {
        pub(super) fields: self::_root::google::protobuf::_fields::MethodDescriptorProtoFields::<
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
            self::_pinternal::SingularMessageField::<
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
        pub(super) shared: self::_pinternal::SharedItemsImpl<1usize>,
    }
    impl MethodDescriptorProtoView {
        pub fn name(&self) -> &str {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.name,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn name_opt(&self) -> ::std::option::Option::<&str> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.name,
                self.shared.bitfield(),
            )
        }
        pub fn has_name(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.name,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn input_type(&self) -> &str {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.input_type,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        /** Input and output type names.  These are resolved in the same way as
 FieldDescriptorProto.type_name, but must refer to a message type.
*/
        pub fn input_type_opt(&self) -> ::std::option::Option::<&str> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.input_type,
                self.shared.bitfield(),
            )
        }
        pub fn has_input_type(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.input_type,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn output_type(&self) -> &str {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.output_type,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn output_type_opt(&self) -> ::std::option::Option::<&str> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.output_type,
                self.shared.bitfield(),
            )
        }
        pub fn has_output_type(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.output_type,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn options(
            &self,
        ) -> ::std::option::Option::<
            &self::_root::google::protobuf::_view::MethodOptionsView,
        > {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.options,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn options_opt(
            &self,
        ) -> ::std::option::Option::<
            &self::_root::google::protobuf::_view::MethodOptionsView,
        > {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.options,
                self.shared.bitfield(),
            )
        }
        pub fn has_options(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.options,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn client_streaming(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.client_streaming,
                self.shared.bitfield(),
                || false,
            )
        }
        /** Identifies if client streams multiple client messages
*/
        pub fn client_streaming_opt(&self) -> ::std::option::Option::<bool> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.client_streaming,
                self.shared.bitfield(),
            )
        }
        pub fn has_client_streaming(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.client_streaming,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn server_streaming(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.server_streaming,
                self.shared.bitfield(),
                || false,
            )
        }
        /** Identifies if server streams multiple server messages
*/
        pub fn server_streaming_opt(&self) -> ::std::option::Option::<bool> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.server_streaming,
                self.shared.bitfield(),
            )
        }
        pub fn has_server_streaming(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.server_streaming,
                    self.shared.bitfield(),
                )
                .is_some()
        }
    }
    impl self::_puroro::MessageView for self::MethodDescriptorProtoView {
        type MessageType = self::_root::google::protobuf::MethodDescriptorProto;
        fn to_bytes<W: ::std::io::Write>(
            &self,
            #[allow(unused)]
            out: &mut W,
        ) -> self::_puroro::Result<()> {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.name,
                self.shared.bitfield(),
                1i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.input_type,
                self.shared.bitfield(),
                2i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.output_type,
                self.shared.bitfield(),
                3i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.options,
                self.shared.bitfield(),
                4i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.client_streaming,
                self.shared.bitfield(),
                5i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.server_streaming,
                self.shared.bitfield(),
                6i32,
                out,
            )?;
            self.shared.unknown_fields().ser_to_write(out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl self::_pinternal::MessageViewInternal for self::MethodDescriptorProtoView {
        fn new_boxed() -> ::std::boxed::Box<Self> {
            use self::_pinternal::SharedItems as _;
            let mut shared: self::_pinternal::SharedItemsImpl::<1usize> = ::std::default::Default::default();
            let fields = self::_root::google::protobuf::_fields::MethodDescriptorProtoFields {
                name: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                input_type: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                output_type: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                options: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                client_streaming: self::_pinternal::FieldType::new(
                    shared.bitfield_mut(),
                ),
                server_streaming: self::_pinternal::FieldType::new(shared.bitfield_mut()),
            };
            ::std::boxed::Box::new(Self { fields, shared })
        }
    }
    impl ::std::ops::Drop for MethodDescriptorProtoView {
        fn drop(&mut self) {
            #[allow(unused)]
            use self::_pinternal::{OneofUnion as _, SharedItems as _};
        }
    }
    impl ::std::fmt::Debug for MethodDescriptorProtoView {
        fn fmt(
            &self,
            fmt: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            let mut debug_struct = fmt
                .debug_struct(stringify!(MethodDescriptorProtoView));
            debug_struct
                .field(stringify!(name), &self.name_opt())
                .field(stringify!(input_type), &self.input_type_opt())
                .field(stringify!(output_type), &self.output_type_opt())
                .field(stringify!(options), &self.options_opt())
                .field(stringify!(client_streaming), &self.client_streaming_opt())
                .field(stringify!(server_streaming), &self.server_streaming_opt());
            self.shared.unknown_fields().debug_struct_fields(&mut debug_struct)?;
            debug_struct.finish()
        }
    }
    impl ::std::cmp::PartialEq for MethodDescriptorProtoView {
        fn eq(&self, rhs: &Self) -> bool {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::SharedItems as _;
            true && self.name_opt() == rhs.name_opt()
                && self.input_type_opt() == rhs.input_type_opt()
                && self.output_type_opt() == rhs.output_type_opt()
                && self.options_opt() == rhs.options_opt()
                && self.client_streaming_opt() == rhs.client_streaming_opt()
                && self.server_streaming_opt() == rhs.server_streaming_opt()
                && self.shared.unknown_fields() == rhs.shared.unknown_fields()
        }
    }
    impl ::std::borrow::ToOwned for MethodDescriptorProtoView {
        type Owned = self::_root::google::protobuf::MethodDescriptorProto;
        fn to_owned(&self) -> Self::Owned {
            #[allow(unused)]
            use self::_pinternal::SharedItems;
            self::_root::google::protobuf::MethodDescriptorProto(
                ::std::boxed::Box::new(Self {
                    fields: self::_root::google::protobuf::_fields::MethodDescriptorProtoFields {
                        name: ::std::clone::Clone::clone(&self.fields.name),
                        input_type: ::std::clone::Clone::clone(&self.fields.input_type),
                        output_type: ::std::clone::Clone::clone(
                            &self.fields.output_type,
                        ),
                        options: ::std::clone::Clone::clone(&self.fields.options),
                        client_streaming: ::std::clone::Clone::clone(
                            &self.fields.client_streaming,
                        ),
                        server_streaming: ::std::clone::Clone::clone(
                            &self.fields.server_streaming,
                        ),
                    },
                    shared: ::std::clone::Clone::clone(&self.shared),
                }),
            )
        }
    }
    #[derive(::std::default::Default)]
    pub struct FileOptionsView {
        pub(super) fields: self::_root::google::protobuf::_fields::FileOptionsFields::<
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
        pub(super) shared: self::_pinternal::SharedItemsImpl<1usize>,
    }
    impl FileOptionsView {
        pub fn java_package(&self) -> &str {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.java_package,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        /** Sets the Java package where classes generated from this .proto will be
 placed.  By default, the proto package is used, but this is often
 inappropriate because proto packages do not normally start with backwards
 domain names.
*/
        pub fn java_package_opt(&self) -> ::std::option::Option::<&str> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.java_package,
                self.shared.bitfield(),
            )
        }
        pub fn has_java_package(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.java_package,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn java_outer_classname(&self) -> &str {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.java_outer_classname,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        /** Controls the name of the wrapper Java class generated for the .proto file.
 That class will always contain the .proto file's getDescriptor() method as
 well as any top-level extensions defined in the .proto file.
 If java_multiple_files is disabled, then all the other classes from the
 .proto file will be nested inside the single wrapper outer class.
*/
        pub fn java_outer_classname_opt(&self) -> ::std::option::Option::<&str> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.java_outer_classname,
                self.shared.bitfield(),
            )
        }
        pub fn has_java_outer_classname(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.java_outer_classname,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn java_multiple_files(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.java_multiple_files,
                self.shared.bitfield(),
                || false,
            )
        }
        /** If enabled, then the Java code generator will generate a separate .java
 file for each top-level message, enum, and service defined in the .proto
 file.  Thus, these types will *not* be nested inside the wrapper class
 named by java_outer_classname.  However, the wrapper class will still be
 generated to contain the file's getDescriptor() method as well as any
 top-level extensions defined in the file.
*/
        pub fn java_multiple_files_opt(&self) -> ::std::option::Option::<bool> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.java_multiple_files,
                self.shared.bitfield(),
            )
        }
        pub fn has_java_multiple_files(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.java_multiple_files,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn java_generate_equals_and_hash(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.java_generate_equals_and_hash,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        /** This option does nothing.
*/
        pub fn java_generate_equals_and_hash_opt(
            &self,
        ) -> ::std::option::Option::<bool> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.java_generate_equals_and_hash,
                self.shared.bitfield(),
            )
        }
        pub fn has_java_generate_equals_and_hash(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.java_generate_equals_and_hash,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn java_string_check_utf8(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.java_string_check_utf8,
                self.shared.bitfield(),
                || false,
            )
        }
        /** If set true, then the Java2 code generator will generate code that
 throws an exception whenever an attempt is made to assign a non-UTF-8
 byte sequence to a string field.
 Message reflection will do the same.
 However, an extension field still accepts non-UTF-8 byte sequences.
 This option has no effect on when used with the lite runtime.
*/
        pub fn java_string_check_utf8_opt(&self) -> ::std::option::Option::<bool> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.java_string_check_utf8,
                self.shared.bitfield(),
            )
        }
        pub fn has_java_string_check_utf8(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.java_string_check_utf8,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn optimize_for(
            &self,
        ) -> self::_root::google::protobuf::file_options::OptimizeMode {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.optimize_for,
                self.shared.bitfield(),
                || self::_root::google::protobuf::file_options::OptimizeMode::Speed,
            )
        }
        pub fn optimize_for_opt(
            &self,
        ) -> ::std::option::Option::<
            self::_root::google::protobuf::file_options::OptimizeMode,
        > {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.optimize_for,
                self.shared.bitfield(),
            )
        }
        pub fn has_optimize_for(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.optimize_for,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn go_package(&self) -> &str {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.go_package,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        /** Sets the Go package where structs generated from this .proto will be
 placed. If omitted, the Go package will be derived from the following:
   - The basename of the package import path, if provided.
   - Otherwise, the package statement in the .proto file, if present.
   - Otherwise, the basename of the .proto file, without extension.
*/
        pub fn go_package_opt(&self) -> ::std::option::Option::<&str> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.go_package,
                self.shared.bitfield(),
            )
        }
        pub fn has_go_package(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.go_package,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn cc_generic_services(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.cc_generic_services,
                self.shared.bitfield(),
                || false,
            )
        }
        /** Should generic services be generated in each language?  "Generic" services
 are not specific to any particular RPC system.  They are generated by the
 main code generators in each language (without additional plugins).
 Generic services were the only kind of service generation supported by
 early versions of google.protobuf.

 Generic services are now considered deprecated in favor of using plugins
 that generate code specific to your particular RPC system.  Therefore,
 these default to false.  Old code which depends on generic services should
 explicitly set them to true.
*/
        pub fn cc_generic_services_opt(&self) -> ::std::option::Option::<bool> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.cc_generic_services,
                self.shared.bitfield(),
            )
        }
        pub fn has_cc_generic_services(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.cc_generic_services,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn java_generic_services(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.java_generic_services,
                self.shared.bitfield(),
                || false,
            )
        }
        pub fn java_generic_services_opt(&self) -> ::std::option::Option::<bool> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.java_generic_services,
                self.shared.bitfield(),
            )
        }
        pub fn has_java_generic_services(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.java_generic_services,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn py_generic_services(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.py_generic_services,
                self.shared.bitfield(),
                || false,
            )
        }
        pub fn py_generic_services_opt(&self) -> ::std::option::Option::<bool> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.py_generic_services,
                self.shared.bitfield(),
            )
        }
        pub fn has_py_generic_services(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.py_generic_services,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn php_generic_services(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.php_generic_services,
                self.shared.bitfield(),
                || false,
            )
        }
        pub fn php_generic_services_opt(&self) -> ::std::option::Option::<bool> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.php_generic_services,
                self.shared.bitfield(),
            )
        }
        pub fn has_php_generic_services(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.php_generic_services,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn deprecated(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.deprecated,
                self.shared.bitfield(),
                || false,
            )
        }
        /** Is this file deprecated?
 Depending on the target platform, this can emit Deprecated annotations
 for everything in the file, or it will be completely ignored; in the very
 least, this is a formalization for deprecating files.
*/
        pub fn deprecated_opt(&self) -> ::std::option::Option::<bool> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.deprecated,
                self.shared.bitfield(),
            )
        }
        pub fn has_deprecated(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.deprecated,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn cc_enable_arenas(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.cc_enable_arenas,
                self.shared.bitfield(),
                || true,
            )
        }
        /** Enables the use of arenas for the proto messages in this file. This applies
 only to generated classes for C++.
*/
        pub fn cc_enable_arenas_opt(&self) -> ::std::option::Option::<bool> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.cc_enable_arenas,
                self.shared.bitfield(),
            )
        }
        pub fn has_cc_enable_arenas(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.cc_enable_arenas,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn objc_class_prefix(&self) -> &str {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.objc_class_prefix,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        /** Sets the objective c class prefix which is prepended to all objective c
 generated classes from this .proto. There is no default.
*/
        pub fn objc_class_prefix_opt(&self) -> ::std::option::Option::<&str> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.objc_class_prefix,
                self.shared.bitfield(),
            )
        }
        pub fn has_objc_class_prefix(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.objc_class_prefix,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn csharp_namespace(&self) -> &str {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.csharp_namespace,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        /** Namespace for generated classes; defaults to the package.
*/
        pub fn csharp_namespace_opt(&self) -> ::std::option::Option::<&str> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.csharp_namespace,
                self.shared.bitfield(),
            )
        }
        pub fn has_csharp_namespace(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.csharp_namespace,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn swift_prefix(&self) -> &str {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.swift_prefix,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        /** By default Swift generators will take the proto package and CamelCase it
 replacing '.' with underscore and use that to prefix the types/symbols
 defined. When this options is provided, they will use this value instead
 to prefix the types/symbols defined.
*/
        pub fn swift_prefix_opt(&self) -> ::std::option::Option::<&str> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.swift_prefix,
                self.shared.bitfield(),
            )
        }
        pub fn has_swift_prefix(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.swift_prefix,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn php_class_prefix(&self) -> &str {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.php_class_prefix,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        /** Sets the php class prefix which is prepended to all php generated classes
 from this .proto. Default is empty.
*/
        pub fn php_class_prefix_opt(&self) -> ::std::option::Option::<&str> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.php_class_prefix,
                self.shared.bitfield(),
            )
        }
        pub fn has_php_class_prefix(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.php_class_prefix,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn php_namespace(&self) -> &str {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.php_namespace,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        /** Use this option to change the namespace of php generated classes. Default
 is empty. When this option is empty, the package name will be used for
 determining the namespace.
*/
        pub fn php_namespace_opt(&self) -> ::std::option::Option::<&str> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.php_namespace,
                self.shared.bitfield(),
            )
        }
        pub fn has_php_namespace(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.php_namespace,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn php_metadata_namespace(&self) -> &str {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.php_metadata_namespace,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        /** Use this option to change the namespace of php generated metadata classes.
 Default is empty. When this option is empty, the proto file name will be
 used for determining the namespace.
*/
        pub fn php_metadata_namespace_opt(&self) -> ::std::option::Option::<&str> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.php_metadata_namespace,
                self.shared.bitfield(),
            )
        }
        pub fn has_php_metadata_namespace(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.php_metadata_namespace,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn ruby_package(&self) -> &str {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.ruby_package,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        /** Use this option to change the package of ruby generated classes. Default
 is empty. When this option is not set, the package name will be used for
 determining the ruby package.
*/
        pub fn ruby_package_opt(&self) -> ::std::option::Option::<&str> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.ruby_package,
                self.shared.bitfield(),
            )
        }
        pub fn has_ruby_package(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.ruby_package,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        /** The parser stores options it doesn't recognize here.
 See the documentation for the "Options" section above.
*/
        pub fn uninterpreted_option(
            &self,
        ) -> impl '_ + self::_puroro::repeated::RepeatedFieldView<
            '_,
            Item = self::_root::google::protobuf::_view::UninterpretedOptionView,
        > {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field(
                &self.fields.uninterpreted_option,
                self.shared.bitfield(),
            )
        }
    }
    impl self::_puroro::MessageView for self::FileOptionsView {
        type MessageType = self::_root::google::protobuf::FileOptions;
        fn to_bytes<W: ::std::io::Write>(
            &self,
            #[allow(unused)]
            out: &mut W,
        ) -> self::_puroro::Result<()> {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.java_package,
                self.shared.bitfield(),
                1i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.java_outer_classname,
                self.shared.bitfield(),
                8i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.java_multiple_files,
                self.shared.bitfield(),
                10i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.java_generate_equals_and_hash,
                self.shared.bitfield(),
                20i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.java_string_check_utf8,
                self.shared.bitfield(),
                27i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.optimize_for,
                self.shared.bitfield(),
                9i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.go_package,
                self.shared.bitfield(),
                11i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.cc_generic_services,
                self.shared.bitfield(),
                16i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.java_generic_services,
                self.shared.bitfield(),
                17i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.py_generic_services,
                self.shared.bitfield(),
                18i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.php_generic_services,
                self.shared.bitfield(),
                42i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.deprecated,
                self.shared.bitfield(),
                23i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.cc_enable_arenas,
                self.shared.bitfield(),
                31i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.objc_class_prefix,
                self.shared.bitfield(),
                36i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.csharp_namespace,
                self.shared.bitfield(),
                37i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.swift_prefix,
                self.shared.bitfield(),
                39i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.php_class_prefix,
                self.shared.bitfield(),
                40i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.php_namespace,
                self.shared.bitfield(),
                41i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.php_metadata_namespace,
                self.shared.bitfield(),
                44i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.ruby_package,
                self.shared.bitfield(),
                45i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.uninterpreted_option,
                self.shared.bitfield(),
                999i32,
                out,
            )?;
            self.shared.unknown_fields().ser_to_write(out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl self::_pinternal::MessageViewInternal for self::FileOptionsView {
        fn new_boxed() -> ::std::boxed::Box<Self> {
            use self::_pinternal::SharedItems as _;
            let mut shared: self::_pinternal::SharedItemsImpl::<1usize> = ::std::default::Default::default();
            let fields = self::_root::google::protobuf::_fields::FileOptionsFields {
                java_package: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                java_outer_classname: self::_pinternal::FieldType::new(
                    shared.bitfield_mut(),
                ),
                java_multiple_files: self::_pinternal::FieldType::new(
                    shared.bitfield_mut(),
                ),
                java_generate_equals_and_hash: self::_pinternal::FieldType::new(
                    shared.bitfield_mut(),
                ),
                java_string_check_utf8: self::_pinternal::FieldType::new(
                    shared.bitfield_mut(),
                ),
                optimize_for: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                go_package: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                cc_generic_services: self::_pinternal::FieldType::new(
                    shared.bitfield_mut(),
                ),
                java_generic_services: self::_pinternal::FieldType::new(
                    shared.bitfield_mut(),
                ),
                py_generic_services: self::_pinternal::FieldType::new(
                    shared.bitfield_mut(),
                ),
                php_generic_services: self::_pinternal::FieldType::new(
                    shared.bitfield_mut(),
                ),
                deprecated: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                cc_enable_arenas: self::_pinternal::FieldType::new(
                    shared.bitfield_mut(),
                ),
                objc_class_prefix: self::_pinternal::FieldType::new(
                    shared.bitfield_mut(),
                ),
                csharp_namespace: self::_pinternal::FieldType::new(
                    shared.bitfield_mut(),
                ),
                swift_prefix: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                php_class_prefix: self::_pinternal::FieldType::new(
                    shared.bitfield_mut(),
                ),
                php_namespace: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                php_metadata_namespace: self::_pinternal::FieldType::new(
                    shared.bitfield_mut(),
                ),
                ruby_package: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                uninterpreted_option: self::_pinternal::FieldType::new(
                    shared.bitfield_mut(),
                ),
            };
            ::std::boxed::Box::new(Self { fields, shared })
        }
    }
    impl ::std::ops::Drop for FileOptionsView {
        fn drop(&mut self) {
            #[allow(unused)]
            use self::_pinternal::{OneofUnion as _, SharedItems as _};
        }
    }
    impl ::std::fmt::Debug for FileOptionsView {
        fn fmt(
            &self,
            fmt: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            let mut debug_struct = fmt.debug_struct(stringify!(FileOptionsView));
            debug_struct
                .field(stringify!(java_package), &self.java_package_opt())
                .field(
                    stringify!(java_outer_classname),
                    &self.java_outer_classname_opt(),
                )
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
                .field(
                    stringify!(java_generic_services),
                    &self.java_generic_services_opt(),
                )
                .field(stringify!(py_generic_services), &self.py_generic_services_opt())
                .field(
                    stringify!(php_generic_services),
                    &self.php_generic_services_opt(),
                )
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
                .field(
                    stringify!(uninterpreted_option),
                    &self
                        .uninterpreted_option()
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>()
                        .as_slice(),
                );
            self.shared.unknown_fields().debug_struct_fields(&mut debug_struct)?;
            debug_struct.finish()
        }
    }
    impl ::std::cmp::PartialEq for FileOptionsView {
        fn eq(&self, rhs: &Self) -> bool {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::SharedItems as _;
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
                && self.uninterpreted_option().into_iter().eq(rhs.uninterpreted_option())
                && self.shared.unknown_fields() == rhs.shared.unknown_fields()
        }
    }
    impl ::std::borrow::ToOwned for FileOptionsView {
        type Owned = self::_root::google::protobuf::FileOptions;
        fn to_owned(&self) -> Self::Owned {
            #[allow(unused)]
            use self::_pinternal::SharedItems;
            self::_root::google::protobuf::FileOptions(
                ::std::boxed::Box::new(Self {
                    fields: self::_root::google::protobuf::_fields::FileOptionsFields {
                        java_package: ::std::clone::Clone::clone(
                            &self.fields.java_package,
                        ),
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
                        optimize_for: ::std::clone::Clone::clone(
                            &self.fields.optimize_for,
                        ),
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
                        swift_prefix: ::std::clone::Clone::clone(
                            &self.fields.swift_prefix,
                        ),
                        php_class_prefix: ::std::clone::Clone::clone(
                            &self.fields.php_class_prefix,
                        ),
                        php_namespace: ::std::clone::Clone::clone(
                            &self.fields.php_namespace,
                        ),
                        php_metadata_namespace: ::std::clone::Clone::clone(
                            &self.fields.php_metadata_namespace,
                        ),
                        ruby_package: ::std::clone::Clone::clone(
                            &self.fields.ruby_package,
                        ),
                        uninterpreted_option: ::std::clone::Clone::clone(
                            &self.fields.uninterpreted_option,
                        ),
                    },
                    shared: ::std::clone::Clone::clone(&self.shared),
                }),
            )
        }
    }
    #[derive(::std::default::Default)]
    pub struct MessageOptionsView {
        pub(super) fields: self::_root::google::protobuf::_fields::MessageOptionsFields::<
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
        pub(super) shared: self::_pinternal::SharedItemsImpl<1usize>,
    }
    impl MessageOptionsView {
        pub fn message_set_wire_format(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.message_set_wire_format,
                self.shared.bitfield(),
                || false,
            )
        }
        /** Set true to use the old proto1 MessageSet wire format for extensions.
 This is provided for backwards-compatibility with the MessageSet wire
 format.  You should not use this for any other reason:  It's less
 efficient, has fewer features, and is more complicated.

 The message must be defined exactly as follows:
   message Foo {
     option message_set_wire_format = true;
     extensions 4 to max;
   }
 Note that the message cannot have any defined fields; MessageSets only
 have extensions.

 All extensions of your type must be singular messages; e.g. they cannot
 be int32s, enums, or repeated messages.

 Because this is an option, the above two restrictions are not enforced by
 the protocol compiler.
*/
        pub fn message_set_wire_format_opt(&self) -> ::std::option::Option::<bool> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.message_set_wire_format,
                self.shared.bitfield(),
            )
        }
        pub fn has_message_set_wire_format(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.message_set_wire_format,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn no_standard_descriptor_accessor(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.no_standard_descriptor_accessor,
                self.shared.bitfield(),
                || false,
            )
        }
        /** Disables the generation of the standard "descriptor()" accessor, which can
 conflict with a field of the same name.  This is meant to make migration
 from proto1 easier; new code should avoid fields named "descriptor".
*/
        pub fn no_standard_descriptor_accessor_opt(
            &self,
        ) -> ::std::option::Option::<bool> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.no_standard_descriptor_accessor,
                self.shared.bitfield(),
            )
        }
        pub fn has_no_standard_descriptor_accessor(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.no_standard_descriptor_accessor,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn deprecated(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.deprecated,
                self.shared.bitfield(),
                || false,
            )
        }
        /** Is this message deprecated?
 Depending on the target platform, this can emit Deprecated annotations
 for the message, or it will be completely ignored; in the very least,
 this is a formalization for deprecating messages.
*/
        pub fn deprecated_opt(&self) -> ::std::option::Option::<bool> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.deprecated,
                self.shared.bitfield(),
            )
        }
        pub fn has_deprecated(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.deprecated,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn map_entry(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.map_entry,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        /** Whether the message is an automatically generated map entry type for the
 maps field.

 For maps fields:
     map<KeyType, ValueType> map_field = 1;
 The parsed descriptor looks like:
     message MapFieldEntry {
         option map_entry = true;
         optional KeyType key = 1;
         optional ValueType value = 2;
     }
     repeated MapFieldEntry map_field = 1;

 Implementations may choose not to generate the map_entry=true message, but
 use a native map in the target language to hold the keys and values.
 The reflection APIs in such implementations still need to work as
 if the field is a repeated message field.

 NOTE: Do not set the option in .proto files. Always use the maps syntax
 instead. The option should only be implicitly set by the proto compiler
 parser.
*/
        pub fn map_entry_opt(&self) -> ::std::option::Option::<bool> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.map_entry,
                self.shared.bitfield(),
            )
        }
        pub fn has_map_entry(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.map_entry,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        /** The parser stores options it doesn't recognize here. See above.
*/
        pub fn uninterpreted_option(
            &self,
        ) -> impl '_ + self::_puroro::repeated::RepeatedFieldView<
            '_,
            Item = self::_root::google::protobuf::_view::UninterpretedOptionView,
        > {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field(
                &self.fields.uninterpreted_option,
                self.shared.bitfield(),
            )
        }
    }
    impl self::_puroro::MessageView for self::MessageOptionsView {
        type MessageType = self::_root::google::protobuf::MessageOptions;
        fn to_bytes<W: ::std::io::Write>(
            &self,
            #[allow(unused)]
            out: &mut W,
        ) -> self::_puroro::Result<()> {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.message_set_wire_format,
                self.shared.bitfield(),
                1i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.no_standard_descriptor_accessor,
                self.shared.bitfield(),
                2i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.deprecated,
                self.shared.bitfield(),
                3i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.map_entry,
                self.shared.bitfield(),
                7i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.uninterpreted_option,
                self.shared.bitfield(),
                999i32,
                out,
            )?;
            self.shared.unknown_fields().ser_to_write(out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl self::_pinternal::MessageViewInternal for self::MessageOptionsView {
        fn new_boxed() -> ::std::boxed::Box<Self> {
            use self::_pinternal::SharedItems as _;
            let mut shared: self::_pinternal::SharedItemsImpl::<1usize> = ::std::default::Default::default();
            let fields = self::_root::google::protobuf::_fields::MessageOptionsFields {
                message_set_wire_format: self::_pinternal::FieldType::new(
                    shared.bitfield_mut(),
                ),
                no_standard_descriptor_accessor: self::_pinternal::FieldType::new(
                    shared.bitfield_mut(),
                ),
                deprecated: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                map_entry: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                uninterpreted_option: self::_pinternal::FieldType::new(
                    shared.bitfield_mut(),
                ),
            };
            ::std::boxed::Box::new(Self { fields, shared })
        }
    }
    impl ::std::ops::Drop for MessageOptionsView {
        fn drop(&mut self) {
            #[allow(unused)]
            use self::_pinternal::{OneofUnion as _, SharedItems as _};
        }
    }
    impl ::std::fmt::Debug for MessageOptionsView {
        fn fmt(
            &self,
            fmt: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            let mut debug_struct = fmt.debug_struct(stringify!(MessageOptionsView));
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
                .field(
                    stringify!(uninterpreted_option),
                    &self
                        .uninterpreted_option()
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>()
                        .as_slice(),
                );
            self.shared.unknown_fields().debug_struct_fields(&mut debug_struct)?;
            debug_struct.finish()
        }
    }
    impl ::std::cmp::PartialEq for MessageOptionsView {
        fn eq(&self, rhs: &Self) -> bool {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::SharedItems as _;
            true
                && self.message_set_wire_format_opt()
                    == rhs.message_set_wire_format_opt()
                && self.no_standard_descriptor_accessor_opt()
                    == rhs.no_standard_descriptor_accessor_opt()
                && self.deprecated_opt() == rhs.deprecated_opt()
                && self.map_entry_opt() == rhs.map_entry_opt()
                && self.uninterpreted_option().into_iter().eq(rhs.uninterpreted_option())
                && self.shared.unknown_fields() == rhs.shared.unknown_fields()
        }
    }
    impl ::std::borrow::ToOwned for MessageOptionsView {
        type Owned = self::_root::google::protobuf::MessageOptions;
        fn to_owned(&self) -> Self::Owned {
            #[allow(unused)]
            use self::_pinternal::SharedItems;
            self::_root::google::protobuf::MessageOptions(
                ::std::boxed::Box::new(Self {
                    fields: self::_root::google::protobuf::_fields::MessageOptionsFields {
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
                    shared: ::std::clone::Clone::clone(&self.shared),
                }),
            )
        }
    }
    #[derive(::std::default::Default)]
    pub struct FieldOptionsView {
        pub(super) fields: self::_root::google::protobuf::_fields::FieldOptionsFields::<
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
        pub(super) shared: self::_pinternal::SharedItemsImpl<1usize>,
    }
    impl FieldOptionsView {
        pub fn ctype(&self) -> self::_root::google::protobuf::field_options::CType {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.ctype,
                self.shared.bitfield(),
                || self::_root::google::protobuf::field_options::CType::String,
            )
        }
        /** The ctype option instructs the C++ code generator to use a different
 representation of the field than it normally would.  See the specific
 options below.  This option is not yet implemented in the open source
 release -- sorry, we'll try to include it in a future version!
*/
        pub fn ctype_opt(
            &self,
        ) -> ::std::option::Option::<
            self::_root::google::protobuf::field_options::CType,
        > {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.ctype,
                self.shared.bitfield(),
            )
        }
        pub fn has_ctype(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.ctype,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn packed(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.packed,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        /** The packed option can be enabled for repeated primitive fields to enable
 a more efficient representation on the wire. Rather than repeatedly
 writing the tag and type for each element, the entire array is encoded as
 a single length-delimited blob. In proto3, only explicit setting it to
 false will avoid using packed encoding.
*/
        pub fn packed_opt(&self) -> ::std::option::Option::<bool> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.packed,
                self.shared.bitfield(),
            )
        }
        pub fn has_packed(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.packed,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn jstype(&self) -> self::_root::google::protobuf::field_options::JSType {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.jstype,
                self.shared.bitfield(),
                || self::_root::google::protobuf::field_options::JSType::JsNormal,
            )
        }
        /** The jstype option determines the JavaScript type used for values of the
 field.  The option is permitted only for 64 bit integral and fixed types
 (int64, uint64, sint64, fixed64, sfixed64).  A field with jstype JS_STRING
 is represented as JavaScript string, which avoids loss of precision that
 can happen when a large value is converted to a floating point JavaScript.
 Specifying JS_NUMBER for the jstype causes the generated JavaScript code to
 use the JavaScript "number" type.  The behavior of the default option
 JS_NORMAL is implementation dependent.

 This option is an enum to permit additional types to be added, e.g.
 goog.math.Integer.
*/
        pub fn jstype_opt(
            &self,
        ) -> ::std::option::Option::<
            self::_root::google::protobuf::field_options::JSType,
        > {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.jstype,
                self.shared.bitfield(),
            )
        }
        pub fn has_jstype(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.jstype,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn lazy(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.lazy,
                self.shared.bitfield(),
                || false,
            )
        }
        /** Should this field be parsed lazily?  Lazy applies only to message-type
 fields.  It means that when the outer message is initially parsed, the
 inner message's contents will not be parsed but instead stored in encoded
 form.  The inner message will actually be parsed when it is first accessed.

 This is only a hint.  Implementations are free to choose whether to use
 eager or lazy parsing regardless of the value of this option.  However,
 setting this option true suggests that the protocol author believes that
 using lazy parsing on this field is worth the additional bookkeeping
 overhead typically needed to implement it.

 This option does not affect the public interface of any generated code;
 all method signatures remain the same.  Furthermore, thread-safety of the
 interface is not affected by this option; const methods remain safe to
 call from multiple threads concurrently, while non-const methods continue
 to require exclusive access.


 Note that implementations may choose not to check required fields within
 a lazy sub-message.  That is, calling IsInitialized() on the outer message
 may return true even if the inner message has missing required fields.
 This is necessary because otherwise the inner message would have to be
 parsed in order to perform the check, defeating the purpose of lazy
 parsing.  An implementation which chooses not to check required fields
 must be consistent about it.  That is, for any particular sub-message, the
 implementation must either *always* check its required fields, or *never*
 check its required fields, regardless of whether or not the message has
 been parsed.
*/
        pub fn lazy_opt(&self) -> ::std::option::Option::<bool> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.lazy,
                self.shared.bitfield(),
            )
        }
        pub fn has_lazy(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.lazy,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn deprecated(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.deprecated,
                self.shared.bitfield(),
                || false,
            )
        }
        /** Is this field deprecated?
 Depending on the target platform, this can emit Deprecated annotations
 for accessors, or it will be completely ignored; in the very least, this
 is a formalization for deprecating fields.
*/
        pub fn deprecated_opt(&self) -> ::std::option::Option::<bool> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.deprecated,
                self.shared.bitfield(),
            )
        }
        pub fn has_deprecated(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.deprecated,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn weak(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.weak,
                self.shared.bitfield(),
                || false,
            )
        }
        /** For Google-internal migration only. Do not use.
*/
        pub fn weak_opt(&self) -> ::std::option::Option::<bool> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.weak,
                self.shared.bitfield(),
            )
        }
        pub fn has_weak(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.weak,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        /** The parser stores options it doesn't recognize here. See above.
*/
        pub fn uninterpreted_option(
            &self,
        ) -> impl '_ + self::_puroro::repeated::RepeatedFieldView<
            '_,
            Item = self::_root::google::protobuf::_view::UninterpretedOptionView,
        > {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field(
                &self.fields.uninterpreted_option,
                self.shared.bitfield(),
            )
        }
    }
    impl self::_puroro::MessageView for self::FieldOptionsView {
        type MessageType = self::_root::google::protobuf::FieldOptions;
        fn to_bytes<W: ::std::io::Write>(
            &self,
            #[allow(unused)]
            out: &mut W,
        ) -> self::_puroro::Result<()> {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.ctype,
                self.shared.bitfield(),
                1i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.packed,
                self.shared.bitfield(),
                2i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.jstype,
                self.shared.bitfield(),
                6i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.lazy,
                self.shared.bitfield(),
                5i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.deprecated,
                self.shared.bitfield(),
                3i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.weak,
                self.shared.bitfield(),
                10i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.uninterpreted_option,
                self.shared.bitfield(),
                999i32,
                out,
            )?;
            self.shared.unknown_fields().ser_to_write(out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl self::_pinternal::MessageViewInternal for self::FieldOptionsView {
        fn new_boxed() -> ::std::boxed::Box<Self> {
            use self::_pinternal::SharedItems as _;
            let mut shared: self::_pinternal::SharedItemsImpl::<1usize> = ::std::default::Default::default();
            let fields = self::_root::google::protobuf::_fields::FieldOptionsFields {
                ctype: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                packed: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                jstype: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                lazy: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                deprecated: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                weak: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                uninterpreted_option: self::_pinternal::FieldType::new(
                    shared.bitfield_mut(),
                ),
            };
            ::std::boxed::Box::new(Self { fields, shared })
        }
    }
    impl ::std::ops::Drop for FieldOptionsView {
        fn drop(&mut self) {
            #[allow(unused)]
            use self::_pinternal::{OneofUnion as _, SharedItems as _};
        }
    }
    impl ::std::fmt::Debug for FieldOptionsView {
        fn fmt(
            &self,
            fmt: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            let mut debug_struct = fmt.debug_struct(stringify!(FieldOptionsView));
            debug_struct
                .field(stringify!(ctype), &self.ctype_opt())
                .field(stringify!(packed), &self.packed_opt())
                .field(stringify!(jstype), &self.jstype_opt())
                .field(stringify!(lazy), &self.lazy_opt())
                .field(stringify!(deprecated), &self.deprecated_opt())
                .field(stringify!(weak), &self.weak_opt())
                .field(
                    stringify!(uninterpreted_option),
                    &self
                        .uninterpreted_option()
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>()
                        .as_slice(),
                );
            self.shared.unknown_fields().debug_struct_fields(&mut debug_struct)?;
            debug_struct.finish()
        }
    }
    impl ::std::cmp::PartialEq for FieldOptionsView {
        fn eq(&self, rhs: &Self) -> bool {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::SharedItems as _;
            true && self.ctype_opt() == rhs.ctype_opt()
                && self.packed_opt() == rhs.packed_opt()
                && self.jstype_opt() == rhs.jstype_opt()
                && self.lazy_opt() == rhs.lazy_opt()
                && self.deprecated_opt() == rhs.deprecated_opt()
                && self.weak_opt() == rhs.weak_opt()
                && self.uninterpreted_option().into_iter().eq(rhs.uninterpreted_option())
                && self.shared.unknown_fields() == rhs.shared.unknown_fields()
        }
    }
    impl ::std::borrow::ToOwned for FieldOptionsView {
        type Owned = self::_root::google::protobuf::FieldOptions;
        fn to_owned(&self) -> Self::Owned {
            #[allow(unused)]
            use self::_pinternal::SharedItems;
            self::_root::google::protobuf::FieldOptions(
                ::std::boxed::Box::new(Self {
                    fields: self::_root::google::protobuf::_fields::FieldOptionsFields {
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
                    shared: ::std::clone::Clone::clone(&self.shared),
                }),
            )
        }
    }
    #[derive(::std::default::Default)]
    pub struct OneofOptionsView {
        pub(super) fields: self::_root::google::protobuf::_fields::OneofOptionsFields::<
            self::_pinternal::RepeatedMessageField::<
                self::_root::google::protobuf::UninterpretedOption,
            >,
        >,
        pub(super) shared: self::_pinternal::SharedItemsImpl<0usize>,
    }
    impl OneofOptionsView {
        /** The parser stores options it doesn't recognize here. See above.
*/
        pub fn uninterpreted_option(
            &self,
        ) -> impl '_ + self::_puroro::repeated::RepeatedFieldView<
            '_,
            Item = self::_root::google::protobuf::_view::UninterpretedOptionView,
        > {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field(
                &self.fields.uninterpreted_option,
                self.shared.bitfield(),
            )
        }
    }
    impl self::_puroro::MessageView for self::OneofOptionsView {
        type MessageType = self::_root::google::protobuf::OneofOptions;
        fn to_bytes<W: ::std::io::Write>(
            &self,
            #[allow(unused)]
            out: &mut W,
        ) -> self::_puroro::Result<()> {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.uninterpreted_option,
                self.shared.bitfield(),
                999i32,
                out,
            )?;
            self.shared.unknown_fields().ser_to_write(out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl self::_pinternal::MessageViewInternal for self::OneofOptionsView {
        fn new_boxed() -> ::std::boxed::Box<Self> {
            use self::_pinternal::SharedItems as _;
            let mut shared: self::_pinternal::SharedItemsImpl::<0usize> = ::std::default::Default::default();
            let fields = self::_root::google::protobuf::_fields::OneofOptionsFields {
                uninterpreted_option: self::_pinternal::FieldType::new(
                    shared.bitfield_mut(),
                ),
            };
            ::std::boxed::Box::new(Self { fields, shared })
        }
    }
    impl ::std::ops::Drop for OneofOptionsView {
        fn drop(&mut self) {
            #[allow(unused)]
            use self::_pinternal::{OneofUnion as _, SharedItems as _};
        }
    }
    impl ::std::fmt::Debug for OneofOptionsView {
        fn fmt(
            &self,
            fmt: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            let mut debug_struct = fmt.debug_struct(stringify!(OneofOptionsView));
            debug_struct
                .field(
                    stringify!(uninterpreted_option),
                    &self
                        .uninterpreted_option()
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>()
                        .as_slice(),
                );
            self.shared.unknown_fields().debug_struct_fields(&mut debug_struct)?;
            debug_struct.finish()
        }
    }
    impl ::std::cmp::PartialEq for OneofOptionsView {
        fn eq(&self, rhs: &Self) -> bool {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::SharedItems as _;
            true
                && self.uninterpreted_option().into_iter().eq(rhs.uninterpreted_option())
                && self.shared.unknown_fields() == rhs.shared.unknown_fields()
        }
    }
    impl ::std::borrow::ToOwned for OneofOptionsView {
        type Owned = self::_root::google::protobuf::OneofOptions;
        fn to_owned(&self) -> Self::Owned {
            #[allow(unused)]
            use self::_pinternal::SharedItems;
            self::_root::google::protobuf::OneofOptions(
                ::std::boxed::Box::new(Self {
                    fields: self::_root::google::protobuf::_fields::OneofOptionsFields {
                        uninterpreted_option: ::std::clone::Clone::clone(
                            &self.fields.uninterpreted_option,
                        ),
                    },
                    shared: ::std::clone::Clone::clone(&self.shared),
                }),
            )
        }
    }
    #[derive(::std::default::Default)]
    pub struct EnumOptionsView {
        pub(super) fields: self::_root::google::protobuf::_fields::EnumOptionsFields::<
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
        pub(super) shared: self::_pinternal::SharedItemsImpl<1usize>,
    }
    impl EnumOptionsView {
        pub fn allow_alias(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.allow_alias,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        /** Set this option to true to allow mapping different tag names to the same
 value.
*/
        pub fn allow_alias_opt(&self) -> ::std::option::Option::<bool> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.allow_alias,
                self.shared.bitfield(),
            )
        }
        pub fn has_allow_alias(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.allow_alias,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn deprecated(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.deprecated,
                self.shared.bitfield(),
                || false,
            )
        }
        /** Is this enum deprecated?
 Depending on the target platform, this can emit Deprecated annotations
 for the enum, or it will be completely ignored; in the very least, this
 is a formalization for deprecating enums.
*/
        pub fn deprecated_opt(&self) -> ::std::option::Option::<bool> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.deprecated,
                self.shared.bitfield(),
            )
        }
        pub fn has_deprecated(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.deprecated,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        /** The parser stores options it doesn't recognize here. See above.
*/
        pub fn uninterpreted_option(
            &self,
        ) -> impl '_ + self::_puroro::repeated::RepeatedFieldView<
            '_,
            Item = self::_root::google::protobuf::_view::UninterpretedOptionView,
        > {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field(
                &self.fields.uninterpreted_option,
                self.shared.bitfield(),
            )
        }
    }
    impl self::_puroro::MessageView for self::EnumOptionsView {
        type MessageType = self::_root::google::protobuf::EnumOptions;
        fn to_bytes<W: ::std::io::Write>(
            &self,
            #[allow(unused)]
            out: &mut W,
        ) -> self::_puroro::Result<()> {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.allow_alias,
                self.shared.bitfield(),
                2i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.deprecated,
                self.shared.bitfield(),
                3i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.uninterpreted_option,
                self.shared.bitfield(),
                999i32,
                out,
            )?;
            self.shared.unknown_fields().ser_to_write(out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl self::_pinternal::MessageViewInternal for self::EnumOptionsView {
        fn new_boxed() -> ::std::boxed::Box<Self> {
            use self::_pinternal::SharedItems as _;
            let mut shared: self::_pinternal::SharedItemsImpl::<1usize> = ::std::default::Default::default();
            let fields = self::_root::google::protobuf::_fields::EnumOptionsFields {
                allow_alias: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                deprecated: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                uninterpreted_option: self::_pinternal::FieldType::new(
                    shared.bitfield_mut(),
                ),
            };
            ::std::boxed::Box::new(Self { fields, shared })
        }
    }
    impl ::std::ops::Drop for EnumOptionsView {
        fn drop(&mut self) {
            #[allow(unused)]
            use self::_pinternal::{OneofUnion as _, SharedItems as _};
        }
    }
    impl ::std::fmt::Debug for EnumOptionsView {
        fn fmt(
            &self,
            fmt: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            let mut debug_struct = fmt.debug_struct(stringify!(EnumOptionsView));
            debug_struct
                .field(stringify!(allow_alias), &self.allow_alias_opt())
                .field(stringify!(deprecated), &self.deprecated_opt())
                .field(
                    stringify!(uninterpreted_option),
                    &self
                        .uninterpreted_option()
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>()
                        .as_slice(),
                );
            self.shared.unknown_fields().debug_struct_fields(&mut debug_struct)?;
            debug_struct.finish()
        }
    }
    impl ::std::cmp::PartialEq for EnumOptionsView {
        fn eq(&self, rhs: &Self) -> bool {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::SharedItems as _;
            true && self.allow_alias_opt() == rhs.allow_alias_opt()
                && self.deprecated_opt() == rhs.deprecated_opt()
                && self.uninterpreted_option().into_iter().eq(rhs.uninterpreted_option())
                && self.shared.unknown_fields() == rhs.shared.unknown_fields()
        }
    }
    impl ::std::borrow::ToOwned for EnumOptionsView {
        type Owned = self::_root::google::protobuf::EnumOptions;
        fn to_owned(&self) -> Self::Owned {
            #[allow(unused)]
            use self::_pinternal::SharedItems;
            self::_root::google::protobuf::EnumOptions(
                ::std::boxed::Box::new(Self {
                    fields: self::_root::google::protobuf::_fields::EnumOptionsFields {
                        allow_alias: ::std::clone::Clone::clone(
                            &self.fields.allow_alias,
                        ),
                        deprecated: ::std::clone::Clone::clone(&self.fields.deprecated),
                        uninterpreted_option: ::std::clone::Clone::clone(
                            &self.fields.uninterpreted_option,
                        ),
                    },
                    shared: ::std::clone::Clone::clone(&self.shared),
                }),
            )
        }
    }
    #[derive(::std::default::Default)]
    pub struct EnumValueOptionsView {
        pub(super) fields: self::_root::google::protobuf::_fields::EnumValueOptionsFields::<
            self::_pinternal::OptionalNumericalField::<
                bool,
                self::_pinternal::tags::Bool,
                0usize,
            >,
            self::_pinternal::RepeatedMessageField::<
                self::_root::google::protobuf::UninterpretedOption,
            >,
        >,
        pub(super) shared: self::_pinternal::SharedItemsImpl<1usize>,
    }
    impl EnumValueOptionsView {
        pub fn deprecated(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.deprecated,
                self.shared.bitfield(),
                || false,
            )
        }
        /** Is this enum value deprecated?
 Depending on the target platform, this can emit Deprecated annotations
 for the enum value, or it will be completely ignored; in the very least,
 this is a formalization for deprecating enum values.
*/
        pub fn deprecated_opt(&self) -> ::std::option::Option::<bool> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.deprecated,
                self.shared.bitfield(),
            )
        }
        pub fn has_deprecated(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.deprecated,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        /** The parser stores options it doesn't recognize here. See above.
*/
        pub fn uninterpreted_option(
            &self,
        ) -> impl '_ + self::_puroro::repeated::RepeatedFieldView<
            '_,
            Item = self::_root::google::protobuf::_view::UninterpretedOptionView,
        > {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field(
                &self.fields.uninterpreted_option,
                self.shared.bitfield(),
            )
        }
    }
    impl self::_puroro::MessageView for self::EnumValueOptionsView {
        type MessageType = self::_root::google::protobuf::EnumValueOptions;
        fn to_bytes<W: ::std::io::Write>(
            &self,
            #[allow(unused)]
            out: &mut W,
        ) -> self::_puroro::Result<()> {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.deprecated,
                self.shared.bitfield(),
                1i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.uninterpreted_option,
                self.shared.bitfield(),
                999i32,
                out,
            )?;
            self.shared.unknown_fields().ser_to_write(out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl self::_pinternal::MessageViewInternal for self::EnumValueOptionsView {
        fn new_boxed() -> ::std::boxed::Box<Self> {
            use self::_pinternal::SharedItems as _;
            let mut shared: self::_pinternal::SharedItemsImpl::<1usize> = ::std::default::Default::default();
            let fields = self::_root::google::protobuf::_fields::EnumValueOptionsFields {
                deprecated: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                uninterpreted_option: self::_pinternal::FieldType::new(
                    shared.bitfield_mut(),
                ),
            };
            ::std::boxed::Box::new(Self { fields, shared })
        }
    }
    impl ::std::ops::Drop for EnumValueOptionsView {
        fn drop(&mut self) {
            #[allow(unused)]
            use self::_pinternal::{OneofUnion as _, SharedItems as _};
        }
    }
    impl ::std::fmt::Debug for EnumValueOptionsView {
        fn fmt(
            &self,
            fmt: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            let mut debug_struct = fmt.debug_struct(stringify!(EnumValueOptionsView));
            debug_struct
                .field(stringify!(deprecated), &self.deprecated_opt())
                .field(
                    stringify!(uninterpreted_option),
                    &self
                        .uninterpreted_option()
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>()
                        .as_slice(),
                );
            self.shared.unknown_fields().debug_struct_fields(&mut debug_struct)?;
            debug_struct.finish()
        }
    }
    impl ::std::cmp::PartialEq for EnumValueOptionsView {
        fn eq(&self, rhs: &Self) -> bool {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::SharedItems as _;
            true && self.deprecated_opt() == rhs.deprecated_opt()
                && self.uninterpreted_option().into_iter().eq(rhs.uninterpreted_option())
                && self.shared.unknown_fields() == rhs.shared.unknown_fields()
        }
    }
    impl ::std::borrow::ToOwned for EnumValueOptionsView {
        type Owned = self::_root::google::protobuf::EnumValueOptions;
        fn to_owned(&self) -> Self::Owned {
            #[allow(unused)]
            use self::_pinternal::SharedItems;
            self::_root::google::protobuf::EnumValueOptions(
                ::std::boxed::Box::new(Self {
                    fields: self::_root::google::protobuf::_fields::EnumValueOptionsFields {
                        deprecated: ::std::clone::Clone::clone(&self.fields.deprecated),
                        uninterpreted_option: ::std::clone::Clone::clone(
                            &self.fields.uninterpreted_option,
                        ),
                    },
                    shared: ::std::clone::Clone::clone(&self.shared),
                }),
            )
        }
    }
    #[derive(::std::default::Default)]
    pub struct ServiceOptionsView {
        pub(super) fields: self::_root::google::protobuf::_fields::ServiceOptionsFields::<
            self::_pinternal::OptionalNumericalField::<
                bool,
                self::_pinternal::tags::Bool,
                0usize,
            >,
            self::_pinternal::RepeatedMessageField::<
                self::_root::google::protobuf::UninterpretedOption,
            >,
        >,
        pub(super) shared: self::_pinternal::SharedItemsImpl<1usize>,
    }
    impl ServiceOptionsView {
        pub fn deprecated(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.deprecated,
                self.shared.bitfield(),
                || false,
            )
        }
        /** Is this service deprecated?
 Depending on the target platform, this can emit Deprecated annotations
 for the service, or it will be completely ignored; in the very least,
 this is a formalization for deprecating services.
*/
        pub fn deprecated_opt(&self) -> ::std::option::Option::<bool> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.deprecated,
                self.shared.bitfield(),
            )
        }
        pub fn has_deprecated(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.deprecated,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        /** The parser stores options it doesn't recognize here. See above.
*/
        pub fn uninterpreted_option(
            &self,
        ) -> impl '_ + self::_puroro::repeated::RepeatedFieldView<
            '_,
            Item = self::_root::google::protobuf::_view::UninterpretedOptionView,
        > {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field(
                &self.fields.uninterpreted_option,
                self.shared.bitfield(),
            )
        }
    }
    impl self::_puroro::MessageView for self::ServiceOptionsView {
        type MessageType = self::_root::google::protobuf::ServiceOptions;
        fn to_bytes<W: ::std::io::Write>(
            &self,
            #[allow(unused)]
            out: &mut W,
        ) -> self::_puroro::Result<()> {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.deprecated,
                self.shared.bitfield(),
                33i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.uninterpreted_option,
                self.shared.bitfield(),
                999i32,
                out,
            )?;
            self.shared.unknown_fields().ser_to_write(out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl self::_pinternal::MessageViewInternal for self::ServiceOptionsView {
        fn new_boxed() -> ::std::boxed::Box<Self> {
            use self::_pinternal::SharedItems as _;
            let mut shared: self::_pinternal::SharedItemsImpl::<1usize> = ::std::default::Default::default();
            let fields = self::_root::google::protobuf::_fields::ServiceOptionsFields {
                deprecated: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                uninterpreted_option: self::_pinternal::FieldType::new(
                    shared.bitfield_mut(),
                ),
            };
            ::std::boxed::Box::new(Self { fields, shared })
        }
    }
    impl ::std::ops::Drop for ServiceOptionsView {
        fn drop(&mut self) {
            #[allow(unused)]
            use self::_pinternal::{OneofUnion as _, SharedItems as _};
        }
    }
    impl ::std::fmt::Debug for ServiceOptionsView {
        fn fmt(
            &self,
            fmt: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            let mut debug_struct = fmt.debug_struct(stringify!(ServiceOptionsView));
            debug_struct
                .field(stringify!(deprecated), &self.deprecated_opt())
                .field(
                    stringify!(uninterpreted_option),
                    &self
                        .uninterpreted_option()
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>()
                        .as_slice(),
                );
            self.shared.unknown_fields().debug_struct_fields(&mut debug_struct)?;
            debug_struct.finish()
        }
    }
    impl ::std::cmp::PartialEq for ServiceOptionsView {
        fn eq(&self, rhs: &Self) -> bool {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::SharedItems as _;
            true && self.deprecated_opt() == rhs.deprecated_opt()
                && self.uninterpreted_option().into_iter().eq(rhs.uninterpreted_option())
                && self.shared.unknown_fields() == rhs.shared.unknown_fields()
        }
    }
    impl ::std::borrow::ToOwned for ServiceOptionsView {
        type Owned = self::_root::google::protobuf::ServiceOptions;
        fn to_owned(&self) -> Self::Owned {
            #[allow(unused)]
            use self::_pinternal::SharedItems;
            self::_root::google::protobuf::ServiceOptions(
                ::std::boxed::Box::new(Self {
                    fields: self::_root::google::protobuf::_fields::ServiceOptionsFields {
                        deprecated: ::std::clone::Clone::clone(&self.fields.deprecated),
                        uninterpreted_option: ::std::clone::Clone::clone(
                            &self.fields.uninterpreted_option,
                        ),
                    },
                    shared: ::std::clone::Clone::clone(&self.shared),
                }),
            )
        }
    }
    #[derive(::std::default::Default)]
    pub struct MethodOptionsView {
        pub(super) fields: self::_root::google::protobuf::_fields::MethodOptionsFields::<
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
        pub(super) shared: self::_pinternal::SharedItemsImpl<1usize>,
    }
    impl MethodOptionsView {
        pub fn deprecated(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.deprecated,
                self.shared.bitfield(),
                || false,
            )
        }
        /** Is this method deprecated?
 Depending on the target platform, this can emit Deprecated annotations
 for the method, or it will be completely ignored; in the very least,
 this is a formalization for deprecating methods.
*/
        pub fn deprecated_opt(&self) -> ::std::option::Option::<bool> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.deprecated,
                self.shared.bitfield(),
            )
        }
        pub fn has_deprecated(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.deprecated,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn idempotency_level(
            &self,
        ) -> self::_root::google::protobuf::method_options::IdempotencyLevel {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.idempotency_level,
                self.shared.bitfield(),
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
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.idempotency_level,
                self.shared.bitfield(),
            )
        }
        pub fn has_idempotency_level(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.idempotency_level,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        /** The parser stores options it doesn't recognize here. See above.
*/
        pub fn uninterpreted_option(
            &self,
        ) -> impl '_ + self::_puroro::repeated::RepeatedFieldView<
            '_,
            Item = self::_root::google::protobuf::_view::UninterpretedOptionView,
        > {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field(
                &self.fields.uninterpreted_option,
                self.shared.bitfield(),
            )
        }
    }
    impl self::_puroro::MessageView for self::MethodOptionsView {
        type MessageType = self::_root::google::protobuf::MethodOptions;
        fn to_bytes<W: ::std::io::Write>(
            &self,
            #[allow(unused)]
            out: &mut W,
        ) -> self::_puroro::Result<()> {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.deprecated,
                self.shared.bitfield(),
                33i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.idempotency_level,
                self.shared.bitfield(),
                34i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.uninterpreted_option,
                self.shared.bitfield(),
                999i32,
                out,
            )?;
            self.shared.unknown_fields().ser_to_write(out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl self::_pinternal::MessageViewInternal for self::MethodOptionsView {
        fn new_boxed() -> ::std::boxed::Box<Self> {
            use self::_pinternal::SharedItems as _;
            let mut shared: self::_pinternal::SharedItemsImpl::<1usize> = ::std::default::Default::default();
            let fields = self::_root::google::protobuf::_fields::MethodOptionsFields {
                deprecated: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                idempotency_level: self::_pinternal::FieldType::new(
                    shared.bitfield_mut(),
                ),
                uninterpreted_option: self::_pinternal::FieldType::new(
                    shared.bitfield_mut(),
                ),
            };
            ::std::boxed::Box::new(Self { fields, shared })
        }
    }
    impl ::std::ops::Drop for MethodOptionsView {
        fn drop(&mut self) {
            #[allow(unused)]
            use self::_pinternal::{OneofUnion as _, SharedItems as _};
        }
    }
    impl ::std::fmt::Debug for MethodOptionsView {
        fn fmt(
            &self,
            fmt: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            let mut debug_struct = fmt.debug_struct(stringify!(MethodOptionsView));
            debug_struct
                .field(stringify!(deprecated), &self.deprecated_opt())
                .field(stringify!(idempotency_level), &self.idempotency_level_opt())
                .field(
                    stringify!(uninterpreted_option),
                    &self
                        .uninterpreted_option()
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>()
                        .as_slice(),
                );
            self.shared.unknown_fields().debug_struct_fields(&mut debug_struct)?;
            debug_struct.finish()
        }
    }
    impl ::std::cmp::PartialEq for MethodOptionsView {
        fn eq(&self, rhs: &Self) -> bool {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::SharedItems as _;
            true && self.deprecated_opt() == rhs.deprecated_opt()
                && self.idempotency_level_opt() == rhs.idempotency_level_opt()
                && self.uninterpreted_option().into_iter().eq(rhs.uninterpreted_option())
                && self.shared.unknown_fields() == rhs.shared.unknown_fields()
        }
    }
    impl ::std::borrow::ToOwned for MethodOptionsView {
        type Owned = self::_root::google::protobuf::MethodOptions;
        fn to_owned(&self) -> Self::Owned {
            #[allow(unused)]
            use self::_pinternal::SharedItems;
            self::_root::google::protobuf::MethodOptions(
                ::std::boxed::Box::new(Self {
                    fields: self::_root::google::protobuf::_fields::MethodOptionsFields {
                        deprecated: ::std::clone::Clone::clone(&self.fields.deprecated),
                        idempotency_level: ::std::clone::Clone::clone(
                            &self.fields.idempotency_level,
                        ),
                        uninterpreted_option: ::std::clone::Clone::clone(
                            &self.fields.uninterpreted_option,
                        ),
                    },
                    shared: ::std::clone::Clone::clone(&self.shared),
                }),
            )
        }
    }
    #[derive(::std::default::Default)]
    pub struct UninterpretedOptionView {
        pub(super) fields: self::_root::google::protobuf::_fields::UninterpretedOptionFields::<
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
        pub(super) shared: self::_pinternal::SharedItemsImpl<1usize>,
    }
    impl UninterpretedOptionView {
        pub fn name(
            &self,
        ) -> impl '_ + self::_puroro::repeated::RepeatedFieldView<
            '_,
            Item = self::_root::google::protobuf::uninterpreted_option::_view::NamePartView,
        > {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field(&self.fields.name, self.shared.bitfield())
        }
        pub fn identifier_value(&self) -> &str {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.identifier_value,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        /** The value of the uninterpreted option, in whatever type the tokenizer
 identified it as during parsing. Exactly one of these should be set.
*/
        pub fn identifier_value_opt(&self) -> ::std::option::Option::<&str> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.identifier_value,
                self.shared.bitfield(),
            )
        }
        pub fn has_identifier_value(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.identifier_value,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn positive_int_value(&self) -> u64 {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.positive_int_value,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn positive_int_value_opt(&self) -> ::std::option::Option::<u64> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.positive_int_value,
                self.shared.bitfield(),
            )
        }
        pub fn has_positive_int_value(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.positive_int_value,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn negative_int_value(&self) -> i64 {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.negative_int_value,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn negative_int_value_opt(&self) -> ::std::option::Option::<i64> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.negative_int_value,
                self.shared.bitfield(),
            )
        }
        pub fn has_negative_int_value(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.negative_int_value,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn double_value(&self) -> f64 {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.double_value,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn double_value_opt(&self) -> ::std::option::Option::<f64> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.double_value,
                self.shared.bitfield(),
            )
        }
        pub fn has_double_value(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.double_value,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn string_value(&self) -> &[u8] {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.string_value,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn string_value_opt(&self) -> ::std::option::Option::<&[u8]> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.string_value,
                self.shared.bitfield(),
            )
        }
        pub fn has_string_value(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.string_value,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn aggregate_value(&self) -> &str {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.aggregate_value,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn aggregate_value_opt(&self) -> ::std::option::Option::<&str> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.aggregate_value,
                self.shared.bitfield(),
            )
        }
        pub fn has_aggregate_value(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.aggregate_value,
                    self.shared.bitfield(),
                )
                .is_some()
        }
    }
    impl self::_puroro::MessageView for self::UninterpretedOptionView {
        type MessageType = self::_root::google::protobuf::UninterpretedOption;
        fn to_bytes<W: ::std::io::Write>(
            &self,
            #[allow(unused)]
            out: &mut W,
        ) -> self::_puroro::Result<()> {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.name,
                self.shared.bitfield(),
                2i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.identifier_value,
                self.shared.bitfield(),
                3i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.positive_int_value,
                self.shared.bitfield(),
                4i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.negative_int_value,
                self.shared.bitfield(),
                5i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.double_value,
                self.shared.bitfield(),
                6i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.string_value,
                self.shared.bitfield(),
                7i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.aggregate_value,
                self.shared.bitfield(),
                8i32,
                out,
            )?;
            self.shared.unknown_fields().ser_to_write(out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl self::_pinternal::MessageViewInternal for self::UninterpretedOptionView {
        fn new_boxed() -> ::std::boxed::Box<Self> {
            use self::_pinternal::SharedItems as _;
            let mut shared: self::_pinternal::SharedItemsImpl::<1usize> = ::std::default::Default::default();
            let fields = self::_root::google::protobuf::_fields::UninterpretedOptionFields {
                name: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                identifier_value: self::_pinternal::FieldType::new(
                    shared.bitfield_mut(),
                ),
                positive_int_value: self::_pinternal::FieldType::new(
                    shared.bitfield_mut(),
                ),
                negative_int_value: self::_pinternal::FieldType::new(
                    shared.bitfield_mut(),
                ),
                double_value: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                string_value: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                aggregate_value: self::_pinternal::FieldType::new(shared.bitfield_mut()),
            };
            ::std::boxed::Box::new(Self { fields, shared })
        }
    }
    impl ::std::ops::Drop for UninterpretedOptionView {
        fn drop(&mut self) {
            #[allow(unused)]
            use self::_pinternal::{OneofUnion as _, SharedItems as _};
        }
    }
    impl ::std::fmt::Debug for UninterpretedOptionView {
        fn fmt(
            &self,
            fmt: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            let mut debug_struct = fmt.debug_struct(stringify!(UninterpretedOptionView));
            debug_struct
                .field(
                    stringify!(name),
                    &self.name().into_iter().collect::<::std::vec::Vec<_>>().as_slice(),
                )
                .field(stringify!(identifier_value), &self.identifier_value_opt())
                .field(stringify!(positive_int_value), &self.positive_int_value_opt())
                .field(stringify!(negative_int_value), &self.negative_int_value_opt())
                .field(stringify!(double_value), &self.double_value_opt())
                .field(stringify!(string_value), &self.string_value_opt())
                .field(stringify!(aggregate_value), &self.aggregate_value_opt());
            self.shared.unknown_fields().debug_struct_fields(&mut debug_struct)?;
            debug_struct.finish()
        }
    }
    impl ::std::cmp::PartialEq for UninterpretedOptionView {
        fn eq(&self, rhs: &Self) -> bool {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::SharedItems as _;
            true && self.name().into_iter().eq(rhs.name())
                && self.identifier_value_opt() == rhs.identifier_value_opt()
                && self.positive_int_value_opt() == rhs.positive_int_value_opt()
                && self.negative_int_value_opt() == rhs.negative_int_value_opt()
                && self.double_value_opt() == rhs.double_value_opt()
                && self.string_value_opt() == rhs.string_value_opt()
                && self.aggregate_value_opt() == rhs.aggregate_value_opt()
                && self.shared.unknown_fields() == rhs.shared.unknown_fields()
        }
    }
    impl ::std::borrow::ToOwned for UninterpretedOptionView {
        type Owned = self::_root::google::protobuf::UninterpretedOption;
        fn to_owned(&self) -> Self::Owned {
            #[allow(unused)]
            use self::_pinternal::SharedItems;
            self::_root::google::protobuf::UninterpretedOption(
                ::std::boxed::Box::new(Self {
                    fields: self::_root::google::protobuf::_fields::UninterpretedOptionFields {
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
                        double_value: ::std::clone::Clone::clone(
                            &self.fields.double_value,
                        ),
                        string_value: ::std::clone::Clone::clone(
                            &self.fields.string_value,
                        ),
                        aggregate_value: ::std::clone::Clone::clone(
                            &self.fields.aggregate_value,
                        ),
                    },
                    shared: ::std::clone::Clone::clone(&self.shared),
                }),
            )
        }
    }
    #[derive(::std::default::Default)]
    pub struct SourceCodeInfoView {
        pub(super) fields: self::_root::google::protobuf::_fields::SourceCodeInfoFields::<
            self::_pinternal::RepeatedMessageField::<
                self::_root::google::protobuf::source_code_info::Location,
            >,
        >,
        pub(super) shared: self::_pinternal::SharedItemsImpl<0usize>,
    }
    impl SourceCodeInfoView {
        /** A Location identifies a piece of source code in a .proto file which
 corresponds to a particular definition.  This information is intended
 to be useful to IDEs, code indexers, documentation generators, and similar
 tools.

 For example, say we have a file like:
   message Foo {
     optional string foo = 1;
   }
 Let's look at just the field definition:
   optional string foo = 1;
   ^       ^^     ^^  ^  ^^^
   a       bc     de  f  ghi
 We have the following locations:
   span   path               represents
   [a,i)  [ 4, 0, 2, 0 ]     The whole field definition.
   [a,b)  [ 4, 0, 2, 0, 4 ]  The label (optional).
   [c,d)  [ 4, 0, 2, 0, 5 ]  The type (string).
   [e,f)  [ 4, 0, 2, 0, 1 ]  The name (foo).
   [g,h)  [ 4, 0, 2, 0, 3 ]  The number (1).

 Notes:
 - A location may refer to a repeated field itself (i.e. not to any
   particular index within it).  This is used whenever a set of elements are
   logically enclosed in a single code segment.  For example, an entire
   extend block (possibly containing multiple extension definitions) will
   have an outer location whose path refers to the "extensions" repeated
   field without an index.
 - Multiple locations may have the same path.  This happens when a single
   logical declaration is spread out across multiple places.  The most
   obvious example is the "extend" block again -- there may be multiple
   extend blocks in the same scope, each of which will have the same path.
 - A location's span is not always a subset of its parent's span.  For
   example, the "extendee" of an extension declaration appears at the
   beginning of the "extend" block and is shared by all extensions within
   the block.
 - Just because a location's span is a subset of some other location's span
   does not mean that it is a descendant.  For example, a "group" defines
   both a type and a field in a single declaration.  Thus, the locations
   corresponding to the type and field and their components will overlap.
 - Code which tries to interpret locations should probably be designed to
   ignore those that it doesn't understand, as more types of locations could
   be recorded in the future.
*/
        pub fn location(
            &self,
        ) -> impl '_ + self::_puroro::repeated::RepeatedFieldView<
            '_,
            Item = self::_root::google::protobuf::source_code_info::_view::LocationView,
        > {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field(&self.fields.location, self.shared.bitfield())
        }
    }
    impl self::_puroro::MessageView for self::SourceCodeInfoView {
        type MessageType = self::_root::google::protobuf::SourceCodeInfo;
        fn to_bytes<W: ::std::io::Write>(
            &self,
            #[allow(unused)]
            out: &mut W,
        ) -> self::_puroro::Result<()> {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.location,
                self.shared.bitfield(),
                1i32,
                out,
            )?;
            self.shared.unknown_fields().ser_to_write(out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl self::_pinternal::MessageViewInternal for self::SourceCodeInfoView {
        fn new_boxed() -> ::std::boxed::Box<Self> {
            use self::_pinternal::SharedItems as _;
            let mut shared: self::_pinternal::SharedItemsImpl::<0usize> = ::std::default::Default::default();
            let fields = self::_root::google::protobuf::_fields::SourceCodeInfoFields {
                location: self::_pinternal::FieldType::new(shared.bitfield_mut()),
            };
            ::std::boxed::Box::new(Self { fields, shared })
        }
    }
    impl ::std::ops::Drop for SourceCodeInfoView {
        fn drop(&mut self) {
            #[allow(unused)]
            use self::_pinternal::{OneofUnion as _, SharedItems as _};
        }
    }
    impl ::std::fmt::Debug for SourceCodeInfoView {
        fn fmt(
            &self,
            fmt: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            let mut debug_struct = fmt.debug_struct(stringify!(SourceCodeInfoView));
            debug_struct
                .field(
                    stringify!(location),
                    &self
                        .location()
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>()
                        .as_slice(),
                );
            self.shared.unknown_fields().debug_struct_fields(&mut debug_struct)?;
            debug_struct.finish()
        }
    }
    impl ::std::cmp::PartialEq for SourceCodeInfoView {
        fn eq(&self, rhs: &Self) -> bool {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::SharedItems as _;
            true && self.location().into_iter().eq(rhs.location())
                && self.shared.unknown_fields() == rhs.shared.unknown_fields()
        }
    }
    impl ::std::borrow::ToOwned for SourceCodeInfoView {
        type Owned = self::_root::google::protobuf::SourceCodeInfo;
        fn to_owned(&self) -> Self::Owned {
            #[allow(unused)]
            use self::_pinternal::SharedItems;
            self::_root::google::protobuf::SourceCodeInfo(
                ::std::boxed::Box::new(Self {
                    fields: self::_root::google::protobuf::_fields::SourceCodeInfoFields {
                        location: ::std::clone::Clone::clone(&self.fields.location),
                    },
                    shared: ::std::clone::Clone::clone(&self.shared),
                }),
            )
        }
    }
    #[derive(::std::default::Default)]
    pub struct GeneratedCodeInfoView {
        pub(super) fields: self::_root::google::protobuf::_fields::GeneratedCodeInfoFields::<
            self::_pinternal::RepeatedMessageField::<
                self::_root::google::protobuf::generated_code_info::Annotation,
            >,
        >,
        pub(super) shared: self::_pinternal::SharedItemsImpl<0usize>,
    }
    impl GeneratedCodeInfoView {
        /** An Annotation connects some span of text in generated code to an element
 of its generating .proto file.
*/
        pub fn annotation(
            &self,
        ) -> impl '_ + self::_puroro::repeated::RepeatedFieldView<
            '_,
            Item = self::_root::google::protobuf::generated_code_info::_view::AnnotationView,
        > {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field(&self.fields.annotation, self.shared.bitfield())
        }
    }
    impl self::_puroro::MessageView for self::GeneratedCodeInfoView {
        type MessageType = self::_root::google::protobuf::GeneratedCodeInfo;
        fn to_bytes<W: ::std::io::Write>(
            &self,
            #[allow(unused)]
            out: &mut W,
        ) -> self::_puroro::Result<()> {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.annotation,
                self.shared.bitfield(),
                1i32,
                out,
            )?;
            self.shared.unknown_fields().ser_to_write(out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl self::_pinternal::MessageViewInternal for self::GeneratedCodeInfoView {
        fn new_boxed() -> ::std::boxed::Box<Self> {
            use self::_pinternal::SharedItems as _;
            let mut shared: self::_pinternal::SharedItemsImpl::<0usize> = ::std::default::Default::default();
            let fields = self::_root::google::protobuf::_fields::GeneratedCodeInfoFields {
                annotation: self::_pinternal::FieldType::new(shared.bitfield_mut()),
            };
            ::std::boxed::Box::new(Self { fields, shared })
        }
    }
    impl ::std::ops::Drop for GeneratedCodeInfoView {
        fn drop(&mut self) {
            #[allow(unused)]
            use self::_pinternal::{OneofUnion as _, SharedItems as _};
        }
    }
    impl ::std::fmt::Debug for GeneratedCodeInfoView {
        fn fmt(
            &self,
            fmt: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            let mut debug_struct = fmt.debug_struct(stringify!(GeneratedCodeInfoView));
            debug_struct
                .field(
                    stringify!(annotation),
                    &self
                        .annotation()
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>()
                        .as_slice(),
                );
            self.shared.unknown_fields().debug_struct_fields(&mut debug_struct)?;
            debug_struct.finish()
        }
    }
    impl ::std::cmp::PartialEq for GeneratedCodeInfoView {
        fn eq(&self, rhs: &Self) -> bool {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::SharedItems as _;
            true && self.annotation().into_iter().eq(rhs.annotation())
                && self.shared.unknown_fields() == rhs.shared.unknown_fields()
        }
    }
    impl ::std::borrow::ToOwned for GeneratedCodeInfoView {
        type Owned = self::_root::google::protobuf::GeneratedCodeInfo;
        fn to_owned(&self) -> Self::Owned {
            #[allow(unused)]
            use self::_pinternal::SharedItems;
            self::_root::google::protobuf::GeneratedCodeInfo(
                ::std::boxed::Box::new(Self {
                    fields: self::_root::google::protobuf::_fields::GeneratedCodeInfoFields {
                        annotation: ::std::clone::Clone::clone(&self.fields.annotation),
                    },
                    shared: ::std::clone::Clone::clone(&self.shared),
                }),
            )
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
#[doc(hidden)]
pub use self::_fields::*;
