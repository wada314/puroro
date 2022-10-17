// A generated source code by puroro library
// package google.protobuf

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub mod _puroro {
    pub use ::puroro::*;
}

#[derive(Default)]
pub struct FileDescriptorSet {
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.FileDescriptorProto")))
    file: self::_puroro::internal::field_type::RepeatedMessageField<
        _puroro_root::google::protobuf::FileDescriptorProto,
    >,

    _bitfield: self::_puroro::bitvec::BitArray<0>,
}

impl FileDescriptorSet {
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.FileDescriptorProto")))
    pub fn file(&self) -> &[_puroro_root::google::protobuf::FileDescriptorProto] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::FileDescriptorProto,
        > as RepeatedFieldType>::get_field(&self.file, &self._bitfield)
    }
    pub fn file_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec<_puroro_root::google::protobuf::FileDescriptorProto> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::FileDescriptorProto,
        > as RepeatedFieldType>::mut_field(&mut self.file, &mut self._bitfield)
    }
    pub fn clear_file(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::FileDescriptorProto,
        > as RepeatedFieldType>::clear(&mut self.file, &mut self._bitfield)
    }
}

impl self::_puroro::Message for FileDescriptorSet {
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
                1 => <self::_puroro::internal::field_type::RepeatedMessageField<
                    _puroro_root::google::protobuf::FileDescriptorProto,
                > as FieldType>::deser_from_iter(
                    &mut self.file, &mut self._bitfield, field_data
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
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::FileDescriptorProto,
        > as FieldType>::ser_to_write(&self.file, &self._bitfield, 1, out)?;

        Ok(())
    }
}

impl ::std::clone::Clone for FileDescriptorSet {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::clone::Clone;
        Self {
            file: Clone::clone(&self.file),

            _bitfield: Clone::clone(&self._bitfield),
        }
    }
}

impl ::std::fmt::Debug for FileDescriptorSet {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct("FileDescriptorSet")
            .field("file", &self.file())
            .finish()
    }
}

impl ::std::cmp::PartialEq for FileDescriptorSet {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;

        true && self.file() == rhs.file()
    }
}

impl ::std::ops::Drop for FileDescriptorSet {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
    }
}

#[derive(Default)]
pub struct FileDescriptorProto {
    // Optional, LengthDelimited(String)
    name: self::_puroro::internal::field_type::OptionalStringField<0>,
    // Optional, LengthDelimited(String)
    package: self::_puroro::internal::field_type::OptionalStringField<1>,
    // Repeated, LengthDelimited(String)
    dependency: self::_puroro::internal::field_type::RepeatedStringField,
    // Repeated, Variant(Int32)
    public_dependency: self::_puroro::internal::field_type::RepeatedNumericalField<
        i32,
        self::_puroro::tags::Int32,
    >,
    // Repeated, Variant(Int32)
    weak_dependency: self::_puroro::internal::field_type::RepeatedNumericalField<
        i32,
        self::_puroro::tags::Int32,
    >,
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.DescriptorProto")))
    message_type: self::_puroro::internal::field_type::RepeatedMessageField<
        _puroro_root::google::protobuf::DescriptorProto,
    >,
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.EnumDescriptorProto")))
    enum_type: self::_puroro::internal::field_type::RepeatedMessageField<
        _puroro_root::google::protobuf::EnumDescriptorProto,
    >,
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.ServiceDescriptorProto")))
    service: self::_puroro::internal::field_type::RepeatedMessageField<
        _puroro_root::google::protobuf::ServiceDescriptorProto,
    >,
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.FieldDescriptorProto")))
    extension: self::_puroro::internal::field_type::RepeatedMessageField<
        _puroro_root::google::protobuf::FieldDescriptorProto,
    >,
    // Optional, LengthDelimited(Message(Fqtn(".google.protobuf.FileOptions")))
    options: self::_puroro::internal::field_type::SingularHeapMessageField<
        _puroro_root::google::protobuf::FileOptions,
    >,
    // Optional, LengthDelimited(Message(Fqtn(".google.protobuf.SourceCodeInfo")))
    source_code_info: self::_puroro::internal::field_type::SingularHeapMessageField<
        _puroro_root::google::protobuf::SourceCodeInfo,
    >,
    // Optional, LengthDelimited(String)
    syntax: self::_puroro::internal::field_type::OptionalStringField<4>,

    _bitfield: self::_puroro::bitvec::BitArray<1>,
}

impl FileDescriptorProto {
    // Optional, LengthDelimited(String)
    pub fn name(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field(
            &self.name, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn name_opt(&self) -> ::std::option::Option<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field_opt(
            &self.name, &self._bitfield,
        )
    }
    pub fn has_name(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field_opt(
            &self.name, &self._bitfield,
        ).is_some()
    }
    pub fn name_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::mut_field(
            &mut self.name, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_name(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField<0> as NonRepeatedFieldType>::clear(
            &mut self.name,
            &mut self._bitfield,
        )
    }
    // Optional, LengthDelimited(String)
    pub fn package(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<1> as NonRepeatedFieldType>::get_field(
            &self.package, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn package_opt(&self) -> ::std::option::Option<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<1> as NonRepeatedFieldType>::get_field_opt(
            &self.package, &self._bitfield,
        )
    }
    pub fn has_package(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<1> as NonRepeatedFieldType>::get_field_opt(
            &self.package, &self._bitfield,
        ).is_some()
    }
    pub fn package_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<1> as NonRepeatedFieldType>::mut_field(
            &mut self.package, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_package(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField<1> as NonRepeatedFieldType>::clear(
            &mut self.package,
            &mut self._bitfield,
        )
    }
    // Repeated, LengthDelimited(String)
    pub fn dependency(&self) -> &[::std::string::String] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedStringField as RepeatedFieldType>::get_field(
            &self.dependency,
            &self._bitfield,
        )
    }
    pub fn dependency_mut(&mut self) -> &mut ::std::vec::Vec<::std::string::String> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedStringField as RepeatedFieldType>::mut_field(
            &mut self.dependency,
            &mut self._bitfield,
        )
    }
    pub fn clear_dependency(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedStringField as RepeatedFieldType>::clear(
            &mut self.dependency,
            &mut self._bitfield,
        )
    }
    // Repeated, Variant(Int32)
    pub fn public_dependency(&self) -> &[i32] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<i32, self::_puroro::tags::Int32> as RepeatedFieldType>::get_field(
            &self.public_dependency, &self._bitfield, 
        )
    }
    pub fn public_dependency_mut(&mut self) -> &mut ::std::vec::Vec<i32> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<i32, self::_puroro::tags::Int32> as RepeatedFieldType>::mut_field(
            &mut self.public_dependency, &mut self._bitfield, 
        )
    }
    pub fn clear_public_dependency(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<i32, self::_puroro::tags::Int32> as RepeatedFieldType>::clear(
            &mut self.public_dependency, &mut self._bitfield, 
        )
    }
    // Repeated, Variant(Int32)
    pub fn weak_dependency(&self) -> &[i32] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<i32, self::_puroro::tags::Int32> as RepeatedFieldType>::get_field(
            &self.weak_dependency, &self._bitfield, 
        )
    }
    pub fn weak_dependency_mut(&mut self) -> &mut ::std::vec::Vec<i32> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<i32, self::_puroro::tags::Int32> as RepeatedFieldType>::mut_field(
            &mut self.weak_dependency, &mut self._bitfield, 
        )
    }
    pub fn clear_weak_dependency(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<i32, self::_puroro::tags::Int32> as RepeatedFieldType>::clear(
            &mut self.weak_dependency, &mut self._bitfield, 
        )
    }
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.DescriptorProto")))
    pub fn message_type(&self) -> &[_puroro_root::google::protobuf::DescriptorProto] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::DescriptorProto,
        > as RepeatedFieldType>::get_field(&self.message_type, &self._bitfield)
    }
    pub fn message_type_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec<_puroro_root::google::protobuf::DescriptorProto> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::DescriptorProto,
        > as RepeatedFieldType>::mut_field(&mut self.message_type, &mut self._bitfield)
    }
    pub fn clear_message_type(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::DescriptorProto,
        > as RepeatedFieldType>::clear(&mut self.message_type, &mut self._bitfield)
    }
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.EnumDescriptorProto")))
    pub fn enum_type(&self) -> &[_puroro_root::google::protobuf::EnumDescriptorProto] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::EnumDescriptorProto,
        > as RepeatedFieldType>::get_field(&self.enum_type, &self._bitfield)
    }
    pub fn enum_type_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec<_puroro_root::google::protobuf::EnumDescriptorProto> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::EnumDescriptorProto,
        > as RepeatedFieldType>::mut_field(&mut self.enum_type, &mut self._bitfield)
    }
    pub fn clear_enum_type(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::EnumDescriptorProto,
        > as RepeatedFieldType>::clear(&mut self.enum_type, &mut self._bitfield)
    }
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.ServiceDescriptorProto")))
    pub fn service(&self) -> &[_puroro_root::google::protobuf::ServiceDescriptorProto] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::ServiceDescriptorProto,
        > as RepeatedFieldType>::get_field(&self.service, &self._bitfield)
    }
    pub fn service_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec<_puroro_root::google::protobuf::ServiceDescriptorProto> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::ServiceDescriptorProto,
        > as RepeatedFieldType>::mut_field(&mut self.service, &mut self._bitfield)
    }
    pub fn clear_service(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::ServiceDescriptorProto,
        > as RepeatedFieldType>::clear(&mut self.service, &mut self._bitfield)
    }
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.FieldDescriptorProto")))
    pub fn extension(&self) -> &[_puroro_root::google::protobuf::FieldDescriptorProto] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::FieldDescriptorProto,
        > as RepeatedFieldType>::get_field(&self.extension, &self._bitfield)
    }
    pub fn extension_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec<_puroro_root::google::protobuf::FieldDescriptorProto> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::FieldDescriptorProto,
        > as RepeatedFieldType>::mut_field(&mut self.extension, &mut self._bitfield)
    }
    pub fn clear_extension(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::FieldDescriptorProto,
        > as RepeatedFieldType>::clear(&mut self.extension, &mut self._bitfield)
    }
    // Optional, LengthDelimited(Message(Fqtn(".google.protobuf.FileOptions")))
    pub fn options(&self) -> ::std::option::Option<&_puroro_root::google::protobuf::FileOptions> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::FileOptions,
        > as NonRepeatedFieldType>::get_field(
            &self.options,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn options_opt(
        &self,
    ) -> ::std::option::Option<&_puroro_root::google::protobuf::FileOptions> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::FileOptions,
        > as NonRepeatedFieldType>::get_field_opt(&self.options, &self._bitfield)
    }
    pub fn has_options(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::FileOptions,
        > as NonRepeatedFieldType>::get_field_opt(&self.options, &self._bitfield)
        .is_some()
    }
    pub fn options_mut(&mut self) -> &mut _puroro_root::google::protobuf::FileOptions {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::FileOptions,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.options,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_options(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::FileOptions,
        > as NonRepeatedFieldType>::clear(&mut self.options, &mut self._bitfield)
    }
    // Optional, LengthDelimited(Message(Fqtn(".google.protobuf.SourceCodeInfo")))
    pub fn source_code_info(
        &self,
    ) -> ::std::option::Option<&_puroro_root::google::protobuf::SourceCodeInfo> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::SourceCodeInfo,
        > as NonRepeatedFieldType>::get_field(
            &self.source_code_info,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn source_code_info_opt(
        &self,
    ) -> ::std::option::Option<&_puroro_root::google::protobuf::SourceCodeInfo> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::SourceCodeInfo,
        > as NonRepeatedFieldType>::get_field_opt(&self.source_code_info, &self._bitfield)
    }
    pub fn has_source_code_info(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::SourceCodeInfo,
        > as NonRepeatedFieldType>::get_field_opt(&self.source_code_info, &self._bitfield)
        .is_some()
    }
    pub fn source_code_info_mut(&mut self) -> &mut _puroro_root::google::protobuf::SourceCodeInfo {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::SourceCodeInfo,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.source_code_info,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_source_code_info(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::SourceCodeInfo,
        > as NonRepeatedFieldType>::clear(&mut self.source_code_info, &mut self._bitfield)
    }
    // Optional, LengthDelimited(String)
    pub fn syntax(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<4> as NonRepeatedFieldType>::get_field(
            &self.syntax, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn syntax_opt(&self) -> ::std::option::Option<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<4> as NonRepeatedFieldType>::get_field_opt(
            &self.syntax, &self._bitfield,
        )
    }
    pub fn has_syntax(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<4> as NonRepeatedFieldType>::get_field_opt(
            &self.syntax, &self._bitfield,
        ).is_some()
    }
    pub fn syntax_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<4> as NonRepeatedFieldType>::mut_field(
            &mut self.syntax, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_syntax(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField<4> as NonRepeatedFieldType>::clear(
            &mut self.syntax,
            &mut self._bitfield,
        )
    }
}

impl self::_puroro::Message for FileDescriptorProto {
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
                    &mut self.name,
                    &mut self._bitfield,
                    field_data,
                )?,
                2 => <self::_puroro::internal::field_type::OptionalStringField::<1> as FieldType>::deser_from_iter(
                    &mut self.package,
                    &mut self._bitfield,
                    field_data,
                )?,
                3 => <self::_puroro::internal::field_type::RepeatedStringField as FieldType>::deser_from_iter(
                    &mut self.dependency,
                    &mut self._bitfield,
                    field_data,
                )?,
                10 => <self::_puroro::internal::field_type::RepeatedNumericalField::<i32, self::_puroro::tags::Int32> as FieldType>::deser_from_iter(
                    &mut self.public_dependency,
                    &mut self._bitfield,
                    field_data,
                )?,
                11 => <self::_puroro::internal::field_type::RepeatedNumericalField::<i32, self::_puroro::tags::Int32> as FieldType>::deser_from_iter(
                    &mut self.weak_dependency,
                    &mut self._bitfield,
                    field_data,
                )?,
                4 => <self::_puroro::internal::field_type::RepeatedMessageField::<_puroro_root::google::protobuf::DescriptorProto> as FieldType>::deser_from_iter(
                    &mut self.message_type,
                    &mut self._bitfield,
                    field_data,
                )?,
                5 => <self::_puroro::internal::field_type::RepeatedMessageField::<_puroro_root::google::protobuf::EnumDescriptorProto> as FieldType>::deser_from_iter(
                    &mut self.enum_type,
                    &mut self._bitfield,
                    field_data,
                )?,
                6 => <self::_puroro::internal::field_type::RepeatedMessageField::<_puroro_root::google::protobuf::ServiceDescriptorProto> as FieldType>::deser_from_iter(
                    &mut self.service,
                    &mut self._bitfield,
                    field_data,
                )?,
                7 => <self::_puroro::internal::field_type::RepeatedMessageField::<_puroro_root::google::protobuf::FieldDescriptorProto> as FieldType>::deser_from_iter(
                    &mut self.extension,
                    &mut self._bitfield,
                    field_data,
                )?,
                8 => <self::_puroro::internal::field_type::SingularHeapMessageField::<_puroro_root::google::protobuf::FileOptions> as FieldType>::deser_from_iter(
                    &mut self.options,
                    &mut self._bitfield,
                    field_data,
                )?,
                9 => <self::_puroro::internal::field_type::SingularHeapMessageField::<_puroro_root::google::protobuf::SourceCodeInfo> as FieldType>::deser_from_iter(
                    &mut self.source_code_info,
                    &mut self._bitfield,
                    field_data,
                )?,
                12 => <self::_puroro::internal::field_type::OptionalStringField::<4> as FieldType>::deser_from_iter(
                    &mut self.syntax,
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
            &self.name,
            &self._bitfield,
            1,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalStringField<1> as FieldType>::ser_to_write(
            &self.package,
            &self._bitfield,
            2,
            out,
        )?;
        <self::_puroro::internal::field_type::RepeatedStringField as FieldType>::ser_to_write(
            &self.dependency,
            &self._bitfield,
            3,
            out,
        )?;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            i32,
            self::_puroro::tags::Int32,
        > as FieldType>::ser_to_write(&self.public_dependency, &self._bitfield, 10, out)?;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            i32,
            self::_puroro::tags::Int32,
        > as FieldType>::ser_to_write(&self.weak_dependency, &self._bitfield, 11, out)?;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::DescriptorProto,
        > as FieldType>::ser_to_write(&self.message_type, &self._bitfield, 4, out)?;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::EnumDescriptorProto,
        > as FieldType>::ser_to_write(&self.enum_type, &self._bitfield, 5, out)?;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::ServiceDescriptorProto,
        > as FieldType>::ser_to_write(&self.service, &self._bitfield, 6, out)?;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::FieldDescriptorProto,
        > as FieldType>::ser_to_write(&self.extension, &self._bitfield, 7, out)?;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::FileOptions,
        > as FieldType>::ser_to_write(&self.options, &self._bitfield, 8, out)?;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::SourceCodeInfo,
        > as FieldType>::ser_to_write(&self.source_code_info, &self._bitfield, 9, out)?;
        <self::_puroro::internal::field_type::OptionalStringField<4> as FieldType>::ser_to_write(
            &self.syntax,
            &self._bitfield,
            12,
            out,
        )?;

        Ok(())
    }
}

impl ::std::clone::Clone for FileDescriptorProto {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::clone::Clone;
        Self {
            name: Clone::clone(&self.name),
            package: Clone::clone(&self.package),
            dependency: Clone::clone(&self.dependency),
            public_dependency: Clone::clone(&self.public_dependency),
            weak_dependency: Clone::clone(&self.weak_dependency),
            message_type: Clone::clone(&self.message_type),
            enum_type: Clone::clone(&self.enum_type),
            service: Clone::clone(&self.service),
            extension: Clone::clone(&self.extension),
            options: Clone::clone(&self.options),
            source_code_info: Clone::clone(&self.source_code_info),
            syntax: Clone::clone(&self.syntax),

            _bitfield: Clone::clone(&self._bitfield),
        }
    }
}

impl ::std::fmt::Debug for FileDescriptorProto {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct("FileDescriptorProto")
            .field("name", &self.name())
            .field("package", &self.package())
            .field("dependency", &self.dependency())
            .field("public_dependency", &self.public_dependency())
            .field("weak_dependency", &self.weak_dependency())
            .field("message_type", &self.message_type())
            .field("enum_type", &self.enum_type())
            .field("service", &self.service())
            .field("extension", &self.extension())
            .field("options", &self.options())
            .field("source_code_info", &self.source_code_info())
            .field("syntax", &self.syntax())
            .finish()
    }
}

impl ::std::cmp::PartialEq for FileDescriptorProto {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;

        true && self.name_opt() == rhs.name_opt()
            && self.package_opt() == rhs.package_opt()
            && self.dependency() == rhs.dependency()
            && self.public_dependency() == rhs.public_dependency()
            && self.weak_dependency() == rhs.weak_dependency()
            && self.message_type() == rhs.message_type()
            && self.enum_type() == rhs.enum_type()
            && self.service() == rhs.service()
            && self.extension() == rhs.extension()
            && self.options_opt() == rhs.options_opt()
            && self.source_code_info_opt() == rhs.source_code_info_opt()
            && self.syntax_opt() == rhs.syntax_opt()
    }
}

impl ::std::ops::Drop for FileDescriptorProto {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
    }
}

#[derive(Default)]
pub struct DescriptorProto {
    // Optional, LengthDelimited(String)
    name: self::_puroro::internal::field_type::OptionalStringField<0>,
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.FieldDescriptorProto")))
    field: self::_puroro::internal::field_type::RepeatedMessageField<
        _puroro_root::google::protobuf::FieldDescriptorProto,
    >,
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.FieldDescriptorProto")))
    extension: self::_puroro::internal::field_type::RepeatedMessageField<
        _puroro_root::google::protobuf::FieldDescriptorProto,
    >,
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.DescriptorProto")))
    nested_type: self::_puroro::internal::field_type::RepeatedMessageField<
        _puroro_root::google::protobuf::DescriptorProto,
    >,
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.EnumDescriptorProto")))
    enum_type: self::_puroro::internal::field_type::RepeatedMessageField<
        _puroro_root::google::protobuf::EnumDescriptorProto,
    >,
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.DescriptorProto.ExtensionRange")))
    extension_range: self::_puroro::internal::field_type::RepeatedMessageField<
        _puroro_root::google::protobuf::descriptor_proto::ExtensionRange,
    >,
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.OneofDescriptorProto")))
    oneof_decl: self::_puroro::internal::field_type::RepeatedMessageField<
        _puroro_root::google::protobuf::OneofDescriptorProto,
    >,
    // Optional, LengthDelimited(Message(Fqtn(".google.protobuf.MessageOptions")))
    options: self::_puroro::internal::field_type::SingularHeapMessageField<
        _puroro_root::google::protobuf::MessageOptions,
    >,
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.DescriptorProto.ReservedRange")))
    reserved_range: self::_puroro::internal::field_type::RepeatedMessageField<
        _puroro_root::google::protobuf::descriptor_proto::ReservedRange,
    >,
    // Repeated, LengthDelimited(String)
    reserved_name: self::_puroro::internal::field_type::RepeatedStringField,

    _bitfield: self::_puroro::bitvec::BitArray<1>,
}

impl DescriptorProto {
    // Optional, LengthDelimited(String)
    pub fn name(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field(
            &self.name, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn name_opt(&self) -> ::std::option::Option<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field_opt(
            &self.name, &self._bitfield,
        )
    }
    pub fn has_name(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field_opt(
            &self.name, &self._bitfield,
        ).is_some()
    }
    pub fn name_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::mut_field(
            &mut self.name, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_name(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField<0> as NonRepeatedFieldType>::clear(
            &mut self.name,
            &mut self._bitfield,
        )
    }
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.FieldDescriptorProto")))
    pub fn field(&self) -> &[_puroro_root::google::protobuf::FieldDescriptorProto] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::FieldDescriptorProto,
        > as RepeatedFieldType>::get_field(&self.field, &self._bitfield)
    }
    pub fn field_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec<_puroro_root::google::protobuf::FieldDescriptorProto> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::FieldDescriptorProto,
        > as RepeatedFieldType>::mut_field(&mut self.field, &mut self._bitfield)
    }
    pub fn clear_field(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::FieldDescriptorProto,
        > as RepeatedFieldType>::clear(&mut self.field, &mut self._bitfield)
    }
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.FieldDescriptorProto")))
    pub fn extension(&self) -> &[_puroro_root::google::protobuf::FieldDescriptorProto] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::FieldDescriptorProto,
        > as RepeatedFieldType>::get_field(&self.extension, &self._bitfield)
    }
    pub fn extension_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec<_puroro_root::google::protobuf::FieldDescriptorProto> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::FieldDescriptorProto,
        > as RepeatedFieldType>::mut_field(&mut self.extension, &mut self._bitfield)
    }
    pub fn clear_extension(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::FieldDescriptorProto,
        > as RepeatedFieldType>::clear(&mut self.extension, &mut self._bitfield)
    }
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.DescriptorProto")))
    pub fn nested_type(&self) -> &[_puroro_root::google::protobuf::DescriptorProto] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::DescriptorProto,
        > as RepeatedFieldType>::get_field(&self.nested_type, &self._bitfield)
    }
    pub fn nested_type_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec<_puroro_root::google::protobuf::DescriptorProto> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::DescriptorProto,
        > as RepeatedFieldType>::mut_field(&mut self.nested_type, &mut self._bitfield)
    }
    pub fn clear_nested_type(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::DescriptorProto,
        > as RepeatedFieldType>::clear(&mut self.nested_type, &mut self._bitfield)
    }
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.EnumDescriptorProto")))
    pub fn enum_type(&self) -> &[_puroro_root::google::protobuf::EnumDescriptorProto] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::EnumDescriptorProto,
        > as RepeatedFieldType>::get_field(&self.enum_type, &self._bitfield)
    }
    pub fn enum_type_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec<_puroro_root::google::protobuf::EnumDescriptorProto> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::EnumDescriptorProto,
        > as RepeatedFieldType>::mut_field(&mut self.enum_type, &mut self._bitfield)
    }
    pub fn clear_enum_type(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::EnumDescriptorProto,
        > as RepeatedFieldType>::clear(&mut self.enum_type, &mut self._bitfield)
    }
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.DescriptorProto.ExtensionRange")))
    pub fn extension_range(
        &self,
    ) -> &[_puroro_root::google::protobuf::descriptor_proto::ExtensionRange] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::descriptor_proto::ExtensionRange,
        > as RepeatedFieldType>::get_field(&self.extension_range, &self._bitfield)
    }
    pub fn extension_range_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec<_puroro_root::google::protobuf::descriptor_proto::ExtensionRange>
    {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::descriptor_proto::ExtensionRange,
        > as RepeatedFieldType>::mut_field(&mut self.extension_range, &mut self._bitfield)
    }
    pub fn clear_extension_range(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::descriptor_proto::ExtensionRange,
        > as RepeatedFieldType>::clear(&mut self.extension_range, &mut self._bitfield)
    }
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.OneofDescriptorProto")))
    pub fn oneof_decl(&self) -> &[_puroro_root::google::protobuf::OneofDescriptorProto] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::OneofDescriptorProto,
        > as RepeatedFieldType>::get_field(&self.oneof_decl, &self._bitfield)
    }
    pub fn oneof_decl_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec<_puroro_root::google::protobuf::OneofDescriptorProto> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::OneofDescriptorProto,
        > as RepeatedFieldType>::mut_field(&mut self.oneof_decl, &mut self._bitfield)
    }
    pub fn clear_oneof_decl(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::OneofDescriptorProto,
        > as RepeatedFieldType>::clear(&mut self.oneof_decl, &mut self._bitfield)
    }
    // Optional, LengthDelimited(Message(Fqtn(".google.protobuf.MessageOptions")))
    pub fn options(
        &self,
    ) -> ::std::option::Option<&_puroro_root::google::protobuf::MessageOptions> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::MessageOptions,
        > as NonRepeatedFieldType>::get_field(
            &self.options,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn options_opt(
        &self,
    ) -> ::std::option::Option<&_puroro_root::google::protobuf::MessageOptions> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::MessageOptions,
        > as NonRepeatedFieldType>::get_field_opt(&self.options, &self._bitfield)
    }
    pub fn has_options(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::MessageOptions,
        > as NonRepeatedFieldType>::get_field_opt(&self.options, &self._bitfield)
        .is_some()
    }
    pub fn options_mut(&mut self) -> &mut _puroro_root::google::protobuf::MessageOptions {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::MessageOptions,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.options,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_options(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::MessageOptions,
        > as NonRepeatedFieldType>::clear(&mut self.options, &mut self._bitfield)
    }
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.DescriptorProto.ReservedRange")))
    pub fn reserved_range(
        &self,
    ) -> &[_puroro_root::google::protobuf::descriptor_proto::ReservedRange] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::descriptor_proto::ReservedRange,
        > as RepeatedFieldType>::get_field(&self.reserved_range, &self._bitfield)
    }
    pub fn reserved_range_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec<_puroro_root::google::protobuf::descriptor_proto::ReservedRange> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::descriptor_proto::ReservedRange,
        > as RepeatedFieldType>::mut_field(&mut self.reserved_range, &mut self._bitfield)
    }
    pub fn clear_reserved_range(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::descriptor_proto::ReservedRange,
        > as RepeatedFieldType>::clear(&mut self.reserved_range, &mut self._bitfield)
    }
    // Repeated, LengthDelimited(String)
    pub fn reserved_name(&self) -> &[::std::string::String] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedStringField as RepeatedFieldType>::get_field(
            &self.reserved_name,
            &self._bitfield,
        )
    }
    pub fn reserved_name_mut(&mut self) -> &mut ::std::vec::Vec<::std::string::String> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedStringField as RepeatedFieldType>::mut_field(
            &mut self.reserved_name,
            &mut self._bitfield,
        )
    }
    pub fn clear_reserved_name(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedStringField as RepeatedFieldType>::clear(
            &mut self.reserved_name,
            &mut self._bitfield,
        )
    }
}

impl self::_puroro::Message for DescriptorProto {
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
                    &mut self.name,
                    &mut self._bitfield,
                    field_data,
                )?,
                2 => <self::_puroro::internal::field_type::RepeatedMessageField::<_puroro_root::google::protobuf::FieldDescriptorProto> as FieldType>::deser_from_iter(
                    &mut self.field,
                    &mut self._bitfield,
                    field_data,
                )?,
                6 => <self::_puroro::internal::field_type::RepeatedMessageField::<_puroro_root::google::protobuf::FieldDescriptorProto> as FieldType>::deser_from_iter(
                    &mut self.extension,
                    &mut self._bitfield,
                    field_data,
                )?,
                3 => <self::_puroro::internal::field_type::RepeatedMessageField::<_puroro_root::google::protobuf::DescriptorProto> as FieldType>::deser_from_iter(
                    &mut self.nested_type,
                    &mut self._bitfield,
                    field_data,
                )?,
                4 => <self::_puroro::internal::field_type::RepeatedMessageField::<_puroro_root::google::protobuf::EnumDescriptorProto> as FieldType>::deser_from_iter(
                    &mut self.enum_type,
                    &mut self._bitfield,
                    field_data,
                )?,
                5 => <self::_puroro::internal::field_type::RepeatedMessageField::<_puroro_root::google::protobuf::descriptor_proto::ExtensionRange> as FieldType>::deser_from_iter(
                    &mut self.extension_range,
                    &mut self._bitfield,
                    field_data,
                )?,
                8 => <self::_puroro::internal::field_type::RepeatedMessageField::<_puroro_root::google::protobuf::OneofDescriptorProto> as FieldType>::deser_from_iter(
                    &mut self.oneof_decl,
                    &mut self._bitfield,
                    field_data,
                )?,
                7 => <self::_puroro::internal::field_type::SingularHeapMessageField::<_puroro_root::google::protobuf::MessageOptions> as FieldType>::deser_from_iter(
                    &mut self.options,
                    &mut self._bitfield,
                    field_data,
                )?,
                9 => <self::_puroro::internal::field_type::RepeatedMessageField::<_puroro_root::google::protobuf::descriptor_proto::ReservedRange> as FieldType>::deser_from_iter(
                    &mut self.reserved_range,
                    &mut self._bitfield,
                    field_data,
                )?,
                10 => <self::_puroro::internal::field_type::RepeatedStringField as FieldType>::deser_from_iter(
                    &mut self.reserved_name,
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
            &self.name,
            &self._bitfield,
            1,
            out,
        )?;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::FieldDescriptorProto,
        > as FieldType>::ser_to_write(&self.field, &self._bitfield, 2, out)?;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::FieldDescriptorProto,
        > as FieldType>::ser_to_write(&self.extension, &self._bitfield, 6, out)?;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::DescriptorProto,
        > as FieldType>::ser_to_write(&self.nested_type, &self._bitfield, 3, out)?;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::EnumDescriptorProto,
        > as FieldType>::ser_to_write(&self.enum_type, &self._bitfield, 4, out)?;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::descriptor_proto::ExtensionRange,
        > as FieldType>::ser_to_write(&self.extension_range, &self._bitfield, 5, out)?;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::OneofDescriptorProto,
        > as FieldType>::ser_to_write(&self.oneof_decl, &self._bitfield, 8, out)?;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::MessageOptions,
        > as FieldType>::ser_to_write(&self.options, &self._bitfield, 7, out)?;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::descriptor_proto::ReservedRange,
        > as FieldType>::ser_to_write(&self.reserved_range, &self._bitfield, 9, out)?;
        <self::_puroro::internal::field_type::RepeatedStringField as FieldType>::ser_to_write(
            &self.reserved_name,
            &self._bitfield,
            10,
            out,
        )?;

        Ok(())
    }
}

impl ::std::clone::Clone for DescriptorProto {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::clone::Clone;
        Self {
            name: Clone::clone(&self.name),
            field: Clone::clone(&self.field),
            extension: Clone::clone(&self.extension),
            nested_type: Clone::clone(&self.nested_type),
            enum_type: Clone::clone(&self.enum_type),
            extension_range: Clone::clone(&self.extension_range),
            oneof_decl: Clone::clone(&self.oneof_decl),
            options: Clone::clone(&self.options),
            reserved_range: Clone::clone(&self.reserved_range),
            reserved_name: Clone::clone(&self.reserved_name),

            _bitfield: Clone::clone(&self._bitfield),
        }
    }
}

impl ::std::fmt::Debug for DescriptorProto {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct("DescriptorProto")
            .field("name", &self.name())
            .field("field", &self.field())
            .field("extension", &self.extension())
            .field("nested_type", &self.nested_type())
            .field("enum_type", &self.enum_type())
            .field("extension_range", &self.extension_range())
            .field("oneof_decl", &self.oneof_decl())
            .field("options", &self.options())
            .field("reserved_range", &self.reserved_range())
            .field("reserved_name", &self.reserved_name())
            .finish()
    }
}

impl ::std::cmp::PartialEq for DescriptorProto {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;

        true && self.name_opt() == rhs.name_opt()
            && self.field() == rhs.field()
            && self.extension() == rhs.extension()
            && self.nested_type() == rhs.nested_type()
            && self.enum_type() == rhs.enum_type()
            && self.extension_range() == rhs.extension_range()
            && self.oneof_decl() == rhs.oneof_decl()
            && self.options_opt() == rhs.options_opt()
            && self.reserved_range() == rhs.reserved_range()
            && self.reserved_name() == rhs.reserved_name()
    }
}

impl ::std::ops::Drop for DescriptorProto {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
    }
}

#[derive(Default)]
pub struct ExtensionRangeOptions {
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.UninterpretedOption")))
    uninterpreted_option: self::_puroro::internal::field_type::RepeatedMessageField<
        _puroro_root::google::protobuf::UninterpretedOption,
    >,

    _bitfield: self::_puroro::bitvec::BitArray<0>,
}

impl ExtensionRangeOptions {
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.UninterpretedOption")))
    pub fn uninterpreted_option(&self) -> &[_puroro_root::google::protobuf::UninterpretedOption] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::get_field(&self.uninterpreted_option, &self._bitfield)
    }
    pub fn uninterpreted_option_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec<_puroro_root::google::protobuf::UninterpretedOption> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::mut_field(
            &mut self.uninterpreted_option, &mut self._bitfield
        )
    }
    pub fn clear_uninterpreted_option(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::clear(&mut self.uninterpreted_option, &mut self._bitfield)
    }
}

impl self::_puroro::Message for ExtensionRangeOptions {
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
                999 => <self::_puroro::internal::field_type::RepeatedMessageField<
                    _puroro_root::google::protobuf::UninterpretedOption,
                > as FieldType>::deser_from_iter(
                    &mut self.uninterpreted_option,
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
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::UninterpretedOption,
        > as FieldType>::ser_to_write(
            &self.uninterpreted_option, &self._bitfield, 999, out
        )?;

        Ok(())
    }
}

impl ::std::clone::Clone for ExtensionRangeOptions {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::clone::Clone;
        Self {
            uninterpreted_option: Clone::clone(&self.uninterpreted_option),

            _bitfield: Clone::clone(&self._bitfield),
        }
    }
}

impl ::std::fmt::Debug for ExtensionRangeOptions {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct("ExtensionRangeOptions")
            .field("uninterpreted_option", &self.uninterpreted_option())
            .finish()
    }
}

impl ::std::cmp::PartialEq for ExtensionRangeOptions {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;

        true && self.uninterpreted_option() == rhs.uninterpreted_option()
    }
}

impl ::std::ops::Drop for ExtensionRangeOptions {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
    }
}

#[derive(Default)]
pub struct FieldDescriptorProto {
    // Optional, LengthDelimited(String)
    name: self::_puroro::internal::field_type::OptionalStringField<0>,
    // Optional, Variant(Int32)
    number: self::_puroro::internal::field_type::OptionalNumericalField<
        i32,
        self::_puroro::tags::Int32,
        1,
    >,
    // Optional, Variant(Enum2(Fqtn(".google.protobuf.FieldDescriptorProto.Label")))
    label: self::_puroro::internal::field_type::OptionalNumericalField<
        _puroro_root::google::protobuf::field_descriptor_proto::Label,
        self::_puroro::tags::Enum2<_puroro_root::google::protobuf::field_descriptor_proto::Label>,
        2,
    >,
    // Optional, Variant(Enum2(Fqtn(".google.protobuf.FieldDescriptorProto.Type")))
    r#type: self::_puroro::internal::field_type::OptionalNumericalField<
        _puroro_root::google::protobuf::field_descriptor_proto::Type,
        self::_puroro::tags::Enum2<_puroro_root::google::protobuf::field_descriptor_proto::Type>,
        3,
    >,
    // Optional, LengthDelimited(String)
    type_name: self::_puroro::internal::field_type::OptionalStringField<4>,
    // Optional, LengthDelimited(String)
    extendee: self::_puroro::internal::field_type::OptionalStringField<5>,
    // Optional, LengthDelimited(String)
    default_value: self::_puroro::internal::field_type::OptionalStringField<6>,
    // Optional, Variant(Int32)
    oneof_index: self::_puroro::internal::field_type::OptionalNumericalField<
        i32,
        self::_puroro::tags::Int32,
        7,
    >,
    // Optional, LengthDelimited(String)
    json_name: self::_puroro::internal::field_type::OptionalStringField<8>,
    // Optional, LengthDelimited(Message(Fqtn(".google.protobuf.FieldOptions")))
    options: self::_puroro::internal::field_type::SingularHeapMessageField<
        _puroro_root::google::protobuf::FieldOptions,
    >,
    // Optional, Variant(Bool)
    proto3_optional: self::_puroro::internal::field_type::OptionalNumericalField<
        bool,
        self::_puroro::tags::Bool,
        10,
    >,

    _bitfield: self::_puroro::bitvec::BitArray<2>,
}

impl FieldDescriptorProto {
    // Optional, LengthDelimited(String)
    pub fn name(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field(
            &self.name, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn name_opt(&self) -> ::std::option::Option<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field_opt(
            &self.name, &self._bitfield,
        )
    }
    pub fn has_name(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field_opt(
            &self.name, &self._bitfield,
        ).is_some()
    }
    pub fn name_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::mut_field(
            &mut self.name, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_name(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField<0> as NonRepeatedFieldType>::clear(
            &mut self.name,
            &mut self._bitfield,
        )
    }
    // Optional, Variant(Int32)
    pub fn number(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            1,
        > as NonRepeatedFieldType>::get_field(
            &self.number,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn number_opt(&self) -> ::std::option::Option<i32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            1,
        > as NonRepeatedFieldType>::get_field_opt(&self.number, &self._bitfield)
    }
    pub fn has_number(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            1,
        > as NonRepeatedFieldType>::get_field_opt(&self.number, &self._bitfield)
        .is_some()
    }
    pub fn number_mut(&mut self) -> &mut i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            1,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.number,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_number(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            1,
        > as NonRepeatedFieldType>::clear(&mut self.number, &mut self._bitfield)
    }
    // Optional, Variant(Enum2(Fqtn(".google.protobuf.FieldDescriptorProto.Label")))
    pub fn label(&self) -> _puroro_root::google::protobuf::field_descriptor_proto::Label {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::google::protobuf::field_descriptor_proto::Label,
            self::_puroro::tags::Enum2<
                _puroro_root::google::protobuf::field_descriptor_proto::Label,
            >,
            2,
        > as NonRepeatedFieldType>::get_field(
            &self.label,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn label_opt(
        &self,
    ) -> ::std::option::Option<_puroro_root::google::protobuf::field_descriptor_proto::Label> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::google::protobuf::field_descriptor_proto::Label,
            self::_puroro::tags::Enum2<
                _puroro_root::google::protobuf::field_descriptor_proto::Label,
            >,
            2,
        > as NonRepeatedFieldType>::get_field_opt(&self.label, &self._bitfield)
    }
    pub fn has_label(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::google::protobuf::field_descriptor_proto::Label,
            self::_puroro::tags::Enum2<
                _puroro_root::google::protobuf::field_descriptor_proto::Label,
            >,
            2,
        > as NonRepeatedFieldType>::get_field_opt(&self.label, &self._bitfield)
        .is_some()
    }
    pub fn label_mut(
        &mut self,
    ) -> &mut _puroro_root::google::protobuf::field_descriptor_proto::Label {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::google::protobuf::field_descriptor_proto::Label,
            self::_puroro::tags::Enum2<
                _puroro_root::google::protobuf::field_descriptor_proto::Label,
            >,
            2,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.label,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_label(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::google::protobuf::field_descriptor_proto::Label,
            self::_puroro::tags::Enum2<
                _puroro_root::google::protobuf::field_descriptor_proto::Label,
            >,
            2,
        > as NonRepeatedFieldType>::clear(&mut self.label, &mut self._bitfield)
    }
    // Optional, Variant(Enum2(Fqtn(".google.protobuf.FieldDescriptorProto.Type")))
    pub fn r#type(&self) -> _puroro_root::google::protobuf::field_descriptor_proto::Type {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::google::protobuf::field_descriptor_proto::Type,
            self::_puroro::tags::Enum2<
                _puroro_root::google::protobuf::field_descriptor_proto::Type,
            >,
            3,
        > as NonRepeatedFieldType>::get_field(
            &self.r#type,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn type_opt(
        &self,
    ) -> ::std::option::Option<_puroro_root::google::protobuf::field_descriptor_proto::Type> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::google::protobuf::field_descriptor_proto::Type,
            self::_puroro::tags::Enum2<
                _puroro_root::google::protobuf::field_descriptor_proto::Type,
            >,
            3,
        > as NonRepeatedFieldType>::get_field_opt(&self.r#type, &self._bitfield)
    }
    pub fn has_type(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::google::protobuf::field_descriptor_proto::Type,
            self::_puroro::tags::Enum2<
                _puroro_root::google::protobuf::field_descriptor_proto::Type,
            >,
            3,
        > as NonRepeatedFieldType>::get_field_opt(&self.r#type, &self._bitfield)
        .is_some()
    }
    pub fn type_mut(
        &mut self,
    ) -> &mut _puroro_root::google::protobuf::field_descriptor_proto::Type {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::google::protobuf::field_descriptor_proto::Type,
            self::_puroro::tags::Enum2<
                _puroro_root::google::protobuf::field_descriptor_proto::Type,
            >,
            3,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.r#type,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_type(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::google::protobuf::field_descriptor_proto::Type,
            self::_puroro::tags::Enum2<
                _puroro_root::google::protobuf::field_descriptor_proto::Type,
            >,
            3,
        > as NonRepeatedFieldType>::clear(&mut self.r#type, &mut self._bitfield)
    }
    // Optional, LengthDelimited(String)
    pub fn type_name(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<4> as NonRepeatedFieldType>::get_field(
            &self.type_name, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn type_name_opt(&self) -> ::std::option::Option<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<4> as NonRepeatedFieldType>::get_field_opt(
            &self.type_name, &self._bitfield,
        )
    }
    pub fn has_type_name(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<4> as NonRepeatedFieldType>::get_field_opt(
            &self.type_name, &self._bitfield,
        ).is_some()
    }
    pub fn type_name_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<4> as NonRepeatedFieldType>::mut_field(
            &mut self.type_name, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_type_name(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField<4> as NonRepeatedFieldType>::clear(
            &mut self.type_name,
            &mut self._bitfield,
        )
    }
    // Optional, LengthDelimited(String)
    pub fn extendee(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<5> as NonRepeatedFieldType>::get_field(
            &self.extendee, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn extendee_opt(&self) -> ::std::option::Option<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<5> as NonRepeatedFieldType>::get_field_opt(
            &self.extendee, &self._bitfield,
        )
    }
    pub fn has_extendee(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<5> as NonRepeatedFieldType>::get_field_opt(
            &self.extendee, &self._bitfield,
        ).is_some()
    }
    pub fn extendee_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<5> as NonRepeatedFieldType>::mut_field(
            &mut self.extendee, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_extendee(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField<5> as NonRepeatedFieldType>::clear(
            &mut self.extendee,
            &mut self._bitfield,
        )
    }
    // Optional, LengthDelimited(String)
    pub fn default_value(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<6> as NonRepeatedFieldType>::get_field(
            &self.default_value, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn default_value_opt(&self) -> ::std::option::Option<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<6> as NonRepeatedFieldType>::get_field_opt(
            &self.default_value, &self._bitfield,
        )
    }
    pub fn has_default_value(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<6> as NonRepeatedFieldType>::get_field_opt(
            &self.default_value, &self._bitfield,
        ).is_some()
    }
    pub fn default_value_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<6> as NonRepeatedFieldType>::mut_field(
            &mut self.default_value, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_default_value(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField<6> as NonRepeatedFieldType>::clear(
            &mut self.default_value,
            &mut self._bitfield,
        )
    }
    // Optional, Variant(Int32)
    pub fn oneof_index(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            7,
        > as NonRepeatedFieldType>::get_field(
            &self.oneof_index,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn oneof_index_opt(&self) -> ::std::option::Option<i32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            7,
        > as NonRepeatedFieldType>::get_field_opt(&self.oneof_index, &self._bitfield)
    }
    pub fn has_oneof_index(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            7,
        > as NonRepeatedFieldType>::get_field_opt(&self.oneof_index, &self._bitfield)
        .is_some()
    }
    pub fn oneof_index_mut(&mut self) -> &mut i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            7,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.oneof_index,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_oneof_index(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            7,
        > as NonRepeatedFieldType>::clear(&mut self.oneof_index, &mut self._bitfield)
    }
    // Optional, LengthDelimited(String)
    pub fn json_name(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<8> as NonRepeatedFieldType>::get_field(
            &self.json_name, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn json_name_opt(&self) -> ::std::option::Option<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<8> as NonRepeatedFieldType>::get_field_opt(
            &self.json_name, &self._bitfield,
        )
    }
    pub fn has_json_name(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<8> as NonRepeatedFieldType>::get_field_opt(
            &self.json_name, &self._bitfield,
        ).is_some()
    }
    pub fn json_name_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<8> as NonRepeatedFieldType>::mut_field(
            &mut self.json_name, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_json_name(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField<8> as NonRepeatedFieldType>::clear(
            &mut self.json_name,
            &mut self._bitfield,
        )
    }
    // Optional, LengthDelimited(Message(Fqtn(".google.protobuf.FieldOptions")))
    pub fn options(&self) -> ::std::option::Option<&_puroro_root::google::protobuf::FieldOptions> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::FieldOptions,
        > as NonRepeatedFieldType>::get_field(
            &self.options,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn options_opt(
        &self,
    ) -> ::std::option::Option<&_puroro_root::google::protobuf::FieldOptions> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::FieldOptions,
        > as NonRepeatedFieldType>::get_field_opt(&self.options, &self._bitfield)
    }
    pub fn has_options(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::FieldOptions,
        > as NonRepeatedFieldType>::get_field_opt(&self.options, &self._bitfield)
        .is_some()
    }
    pub fn options_mut(&mut self) -> &mut _puroro_root::google::protobuf::FieldOptions {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::FieldOptions,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.options,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_options(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::FieldOptions,
        > as NonRepeatedFieldType>::clear(&mut self.options, &mut self._bitfield)
    }
    // Optional, Variant(Bool)
    pub fn proto3_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            10,
        > as NonRepeatedFieldType>::get_field(
            &self.proto3_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn proto3_optional_opt(&self) -> ::std::option::Option<bool> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            10,
        > as NonRepeatedFieldType>::get_field_opt(&self.proto3_optional, &self._bitfield)
    }
    pub fn has_proto3_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            10,
        > as NonRepeatedFieldType>::get_field_opt(&self.proto3_optional, &self._bitfield)
        .is_some()
    }
    pub fn proto3_optional_mut(&mut self) -> &mut bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            10,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.proto3_optional,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_proto3_optional(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            10,
        > as NonRepeatedFieldType>::clear(&mut self.proto3_optional, &mut self._bitfield)
    }
}

impl self::_puroro::Message for FieldDescriptorProto {
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
                    &mut self.name,
                    &mut self._bitfield,
                    field_data,
                )?,
                3 => <self::_puroro::internal::field_type::OptionalNumericalField::<i32, self::_puroro::tags::Int32, 1> as FieldType>::deser_from_iter(
                    &mut self.number,
                    &mut self._bitfield,
                    field_data,
                )?,
                4 => <self::_puroro::internal::field_type::OptionalNumericalField::<_puroro_root::google::protobuf::field_descriptor_proto::Label, self::_puroro::tags::Enum2<_puroro_root::google::protobuf::field_descriptor_proto::Label>, 2> as FieldType>::deser_from_iter(
                    &mut self.label,
                    &mut self._bitfield,
                    field_data,
                )?,
                5 => <self::_puroro::internal::field_type::OptionalNumericalField::<_puroro_root::google::protobuf::field_descriptor_proto::Type, self::_puroro::tags::Enum2<_puroro_root::google::protobuf::field_descriptor_proto::Type>, 3> as FieldType>::deser_from_iter(
                    &mut self.r#type,
                    &mut self._bitfield,
                    field_data,
                )?,
                6 => <self::_puroro::internal::field_type::OptionalStringField::<4> as FieldType>::deser_from_iter(
                    &mut self.type_name,
                    &mut self._bitfield,
                    field_data,
                )?,
                2 => <self::_puroro::internal::field_type::OptionalStringField::<5> as FieldType>::deser_from_iter(
                    &mut self.extendee,
                    &mut self._bitfield,
                    field_data,
                )?,
                7 => <self::_puroro::internal::field_type::OptionalStringField::<6> as FieldType>::deser_from_iter(
                    &mut self.default_value,
                    &mut self._bitfield,
                    field_data,
                )?,
                9 => <self::_puroro::internal::field_type::OptionalNumericalField::<i32, self::_puroro::tags::Int32, 7> as FieldType>::deser_from_iter(
                    &mut self.oneof_index,
                    &mut self._bitfield,
                    field_data,
                )?,
                10 => <self::_puroro::internal::field_type::OptionalStringField::<8> as FieldType>::deser_from_iter(
                    &mut self.json_name,
                    &mut self._bitfield,
                    field_data,
                )?,
                8 => <self::_puroro::internal::field_type::SingularHeapMessageField::<_puroro_root::google::protobuf::FieldOptions> as FieldType>::deser_from_iter(
                    &mut self.options,
                    &mut self._bitfield,
                    field_data,
                )?,
                17 => <self::_puroro::internal::field_type::OptionalNumericalField::<bool, self::_puroro::tags::Bool, 10> as FieldType>::deser_from_iter(
                    &mut self.proto3_optional,
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
            &self.name,
            &self._bitfield,
            1,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            1,
        > as FieldType>::ser_to_write(&self.number, &self._bitfield, 3, out)?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::google::protobuf::field_descriptor_proto::Label,
            self::_puroro::tags::Enum2<
                _puroro_root::google::protobuf::field_descriptor_proto::Label,
            >,
            2,
        > as FieldType>::ser_to_write(&self.label, &self._bitfield, 4, out)?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::google::protobuf::field_descriptor_proto::Type,
            self::_puroro::tags::Enum2<
                _puroro_root::google::protobuf::field_descriptor_proto::Type,
            >,
            3,
        > as FieldType>::ser_to_write(&self.r#type, &self._bitfield, 5, out)?;
        <self::_puroro::internal::field_type::OptionalStringField<4> as FieldType>::ser_to_write(
            &self.type_name,
            &self._bitfield,
            6,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalStringField<5> as FieldType>::ser_to_write(
            &self.extendee,
            &self._bitfield,
            2,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalStringField<6> as FieldType>::ser_to_write(
            &self.default_value,
            &self._bitfield,
            7,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            7,
        > as FieldType>::ser_to_write(&self.oneof_index, &self._bitfield, 9, out)?;
        <self::_puroro::internal::field_type::OptionalStringField<8> as FieldType>::ser_to_write(
            &self.json_name,
            &self._bitfield,
            10,
            out,
        )?;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::FieldOptions,
        > as FieldType>::ser_to_write(&self.options, &self._bitfield, 8, out)?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            10,
        > as FieldType>::ser_to_write(&self.proto3_optional, &self._bitfield, 17, out)?;

        Ok(())
    }
}

impl ::std::clone::Clone for FieldDescriptorProto {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::clone::Clone;
        Self {
            name: Clone::clone(&self.name),
            number: Clone::clone(&self.number),
            label: Clone::clone(&self.label),
            r#type: Clone::clone(&self.r#type),
            type_name: Clone::clone(&self.type_name),
            extendee: Clone::clone(&self.extendee),
            default_value: Clone::clone(&self.default_value),
            oneof_index: Clone::clone(&self.oneof_index),
            json_name: Clone::clone(&self.json_name),
            options: Clone::clone(&self.options),
            proto3_optional: Clone::clone(&self.proto3_optional),

            _bitfield: Clone::clone(&self._bitfield),
        }
    }
}

impl ::std::fmt::Debug for FieldDescriptorProto {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct("FieldDescriptorProto")
            .field("name", &self.name())
            .field("number", &self.number())
            .field("label", &self.label())
            .field("r#type", &self.r#type())
            .field("type_name", &self.type_name())
            .field("extendee", &self.extendee())
            .field("default_value", &self.default_value())
            .field("oneof_index", &self.oneof_index())
            .field("json_name", &self.json_name())
            .field("options", &self.options())
            .field("proto3_optional", &self.proto3_optional())
            .finish()
    }
}

impl ::std::cmp::PartialEq for FieldDescriptorProto {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;

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
    }
}

impl ::std::ops::Drop for FieldDescriptorProto {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
    }
}

#[derive(Default)]
pub struct OneofDescriptorProto {
    // Optional, LengthDelimited(String)
    name: self::_puroro::internal::field_type::OptionalStringField<0>,
    // Optional, LengthDelimited(Message(Fqtn(".google.protobuf.OneofOptions")))
    options: self::_puroro::internal::field_type::SingularHeapMessageField<
        _puroro_root::google::protobuf::OneofOptions,
    >,

    _bitfield: self::_puroro::bitvec::BitArray<1>,
}

impl OneofDescriptorProto {
    // Optional, LengthDelimited(String)
    pub fn name(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field(
            &self.name, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn name_opt(&self) -> ::std::option::Option<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field_opt(
            &self.name, &self._bitfield,
        )
    }
    pub fn has_name(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field_opt(
            &self.name, &self._bitfield,
        ).is_some()
    }
    pub fn name_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::mut_field(
            &mut self.name, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_name(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField<0> as NonRepeatedFieldType>::clear(
            &mut self.name,
            &mut self._bitfield,
        )
    }
    // Optional, LengthDelimited(Message(Fqtn(".google.protobuf.OneofOptions")))
    pub fn options(&self) -> ::std::option::Option<&_puroro_root::google::protobuf::OneofOptions> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::OneofOptions,
        > as NonRepeatedFieldType>::get_field(
            &self.options,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn options_opt(
        &self,
    ) -> ::std::option::Option<&_puroro_root::google::protobuf::OneofOptions> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::OneofOptions,
        > as NonRepeatedFieldType>::get_field_opt(&self.options, &self._bitfield)
    }
    pub fn has_options(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::OneofOptions,
        > as NonRepeatedFieldType>::get_field_opt(&self.options, &self._bitfield)
        .is_some()
    }
    pub fn options_mut(&mut self) -> &mut _puroro_root::google::protobuf::OneofOptions {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::OneofOptions,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.options,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_options(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::OneofOptions,
        > as NonRepeatedFieldType>::clear(&mut self.options, &mut self._bitfield)
    }
}

impl self::_puroro::Message for OneofDescriptorProto {
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
                    &mut self.name,
                    &mut self._bitfield,
                    field_data,
                )?,
                2 => <self::_puroro::internal::field_type::SingularHeapMessageField::<_puroro_root::google::protobuf::OneofOptions> as FieldType>::deser_from_iter(
                    &mut self.options,
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
            &self.name,
            &self._bitfield,
            1,
            out,
        )?;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::OneofOptions,
        > as FieldType>::ser_to_write(&self.options, &self._bitfield, 2, out)?;

        Ok(())
    }
}

impl ::std::clone::Clone for OneofDescriptorProto {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::clone::Clone;
        Self {
            name: Clone::clone(&self.name),
            options: Clone::clone(&self.options),

            _bitfield: Clone::clone(&self._bitfield),
        }
    }
}

impl ::std::fmt::Debug for OneofDescriptorProto {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct("OneofDescriptorProto")
            .field("name", &self.name())
            .field("options", &self.options())
            .finish()
    }
}

impl ::std::cmp::PartialEq for OneofDescriptorProto {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;

        true && self.name_opt() == rhs.name_opt() && self.options_opt() == rhs.options_opt()
    }
}

impl ::std::ops::Drop for OneofDescriptorProto {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
    }
}

#[derive(Default)]
pub struct EnumDescriptorProto {
    // Optional, LengthDelimited(String)
    name: self::_puroro::internal::field_type::OptionalStringField<0>,
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.EnumValueDescriptorProto")))
    value: self::_puroro::internal::field_type::RepeatedMessageField<
        _puroro_root::google::protobuf::EnumValueDescriptorProto,
    >,
    // Optional, LengthDelimited(Message(Fqtn(".google.protobuf.EnumOptions")))
    options: self::_puroro::internal::field_type::SingularHeapMessageField<
        _puroro_root::google::protobuf::EnumOptions,
    >,
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.EnumDescriptorProto.EnumReservedRange")))
    reserved_range: self::_puroro::internal::field_type::RepeatedMessageField<
        _puroro_root::google::protobuf::enum_descriptor_proto::EnumReservedRange,
    >,
    // Repeated, LengthDelimited(String)
    reserved_name: self::_puroro::internal::field_type::RepeatedStringField,

    _bitfield: self::_puroro::bitvec::BitArray<1>,
}

impl EnumDescriptorProto {
    // Optional, LengthDelimited(String)
    pub fn name(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field(
            &self.name, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn name_opt(&self) -> ::std::option::Option<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field_opt(
            &self.name, &self._bitfield,
        )
    }
    pub fn has_name(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field_opt(
            &self.name, &self._bitfield,
        ).is_some()
    }
    pub fn name_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::mut_field(
            &mut self.name, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_name(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField<0> as NonRepeatedFieldType>::clear(
            &mut self.name,
            &mut self._bitfield,
        )
    }
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.EnumValueDescriptorProto")))
    pub fn value(&self) -> &[_puroro_root::google::protobuf::EnumValueDescriptorProto] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::EnumValueDescriptorProto,
        > as RepeatedFieldType>::get_field(&self.value, &self._bitfield)
    }
    pub fn value_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec<_puroro_root::google::protobuf::EnumValueDescriptorProto> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::EnumValueDescriptorProto,
        > as RepeatedFieldType>::mut_field(&mut self.value, &mut self._bitfield)
    }
    pub fn clear_value(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::EnumValueDescriptorProto,
        > as RepeatedFieldType>::clear(&mut self.value, &mut self._bitfield)
    }
    // Optional, LengthDelimited(Message(Fqtn(".google.protobuf.EnumOptions")))
    pub fn options(&self) -> ::std::option::Option<&_puroro_root::google::protobuf::EnumOptions> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::EnumOptions,
        > as NonRepeatedFieldType>::get_field(
            &self.options,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn options_opt(
        &self,
    ) -> ::std::option::Option<&_puroro_root::google::protobuf::EnumOptions> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::EnumOptions,
        > as NonRepeatedFieldType>::get_field_opt(&self.options, &self._bitfield)
    }
    pub fn has_options(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::EnumOptions,
        > as NonRepeatedFieldType>::get_field_opt(&self.options, &self._bitfield)
        .is_some()
    }
    pub fn options_mut(&mut self) -> &mut _puroro_root::google::protobuf::EnumOptions {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::EnumOptions,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.options,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_options(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::EnumOptions,
        > as NonRepeatedFieldType>::clear(&mut self.options, &mut self._bitfield)
    }
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.EnumDescriptorProto.EnumReservedRange")))
    pub fn reserved_range(
        &self,
    ) -> &[_puroro_root::google::protobuf::enum_descriptor_proto::EnumReservedRange] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::enum_descriptor_proto::EnumReservedRange,
        > as RepeatedFieldType>::get_field(&self.reserved_range, &self._bitfield)
    }
    pub fn reserved_range_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec<
        _puroro_root::google::protobuf::enum_descriptor_proto::EnumReservedRange,
    > {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::enum_descriptor_proto::EnumReservedRange,
        > as RepeatedFieldType>::mut_field(&mut self.reserved_range, &mut self._bitfield)
    }
    pub fn clear_reserved_range(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::enum_descriptor_proto::EnumReservedRange,
        > as RepeatedFieldType>::clear(&mut self.reserved_range, &mut self._bitfield)
    }
    // Repeated, LengthDelimited(String)
    pub fn reserved_name(&self) -> &[::std::string::String] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedStringField as RepeatedFieldType>::get_field(
            &self.reserved_name,
            &self._bitfield,
        )
    }
    pub fn reserved_name_mut(&mut self) -> &mut ::std::vec::Vec<::std::string::String> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedStringField as RepeatedFieldType>::mut_field(
            &mut self.reserved_name,
            &mut self._bitfield,
        )
    }
    pub fn clear_reserved_name(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedStringField as RepeatedFieldType>::clear(
            &mut self.reserved_name,
            &mut self._bitfield,
        )
    }
}

impl self::_puroro::Message for EnumDescriptorProto {
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
                    &mut self.name,
                    &mut self._bitfield,
                    field_data,
                )?,
                2 => <self::_puroro::internal::field_type::RepeatedMessageField::<_puroro_root::google::protobuf::EnumValueDescriptorProto> as FieldType>::deser_from_iter(
                    &mut self.value,
                    &mut self._bitfield,
                    field_data,
                )?,
                3 => <self::_puroro::internal::field_type::SingularHeapMessageField::<_puroro_root::google::protobuf::EnumOptions> as FieldType>::deser_from_iter(
                    &mut self.options,
                    &mut self._bitfield,
                    field_data,
                )?,
                4 => <self::_puroro::internal::field_type::RepeatedMessageField::<_puroro_root::google::protobuf::enum_descriptor_proto::EnumReservedRange> as FieldType>::deser_from_iter(
                    &mut self.reserved_range,
                    &mut self._bitfield,
                    field_data,
                )?,
                5 => <self::_puroro::internal::field_type::RepeatedStringField as FieldType>::deser_from_iter(
                    &mut self.reserved_name,
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
            &self.name,
            &self._bitfield,
            1,
            out,
        )?;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::EnumValueDescriptorProto,
        > as FieldType>::ser_to_write(&self.value, &self._bitfield, 2, out)?;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::EnumOptions,
        > as FieldType>::ser_to_write(&self.options, &self._bitfield, 3, out)?;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::enum_descriptor_proto::EnumReservedRange,
        > as FieldType>::ser_to_write(&self.reserved_range, &self._bitfield, 4, out)?;
        <self::_puroro::internal::field_type::RepeatedStringField as FieldType>::ser_to_write(
            &self.reserved_name,
            &self._bitfield,
            5,
            out,
        )?;

        Ok(())
    }
}

impl ::std::clone::Clone for EnumDescriptorProto {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::clone::Clone;
        Self {
            name: Clone::clone(&self.name),
            value: Clone::clone(&self.value),
            options: Clone::clone(&self.options),
            reserved_range: Clone::clone(&self.reserved_range),
            reserved_name: Clone::clone(&self.reserved_name),

            _bitfield: Clone::clone(&self._bitfield),
        }
    }
}

impl ::std::fmt::Debug for EnumDescriptorProto {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct("EnumDescriptorProto")
            .field("name", &self.name())
            .field("value", &self.value())
            .field("options", &self.options())
            .field("reserved_range", &self.reserved_range())
            .field("reserved_name", &self.reserved_name())
            .finish()
    }
}

impl ::std::cmp::PartialEq for EnumDescriptorProto {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;

        true && self.name_opt() == rhs.name_opt()
            && self.value() == rhs.value()
            && self.options_opt() == rhs.options_opt()
            && self.reserved_range() == rhs.reserved_range()
            && self.reserved_name() == rhs.reserved_name()
    }
}

impl ::std::ops::Drop for EnumDescriptorProto {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
    }
}

#[derive(Default)]
pub struct EnumValueDescriptorProto {
    // Optional, LengthDelimited(String)
    name: self::_puroro::internal::field_type::OptionalStringField<0>,
    // Optional, Variant(Int32)
    number: self::_puroro::internal::field_type::OptionalNumericalField<
        i32,
        self::_puroro::tags::Int32,
        1,
    >,
    // Optional, LengthDelimited(Message(Fqtn(".google.protobuf.EnumValueOptions")))
    options: self::_puroro::internal::field_type::SingularHeapMessageField<
        _puroro_root::google::protobuf::EnumValueOptions,
    >,

    _bitfield: self::_puroro::bitvec::BitArray<1>,
}

impl EnumValueDescriptorProto {
    // Optional, LengthDelimited(String)
    pub fn name(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field(
            &self.name, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn name_opt(&self) -> ::std::option::Option<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field_opt(
            &self.name, &self._bitfield,
        )
    }
    pub fn has_name(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field_opt(
            &self.name, &self._bitfield,
        ).is_some()
    }
    pub fn name_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::mut_field(
            &mut self.name, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_name(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField<0> as NonRepeatedFieldType>::clear(
            &mut self.name,
            &mut self._bitfield,
        )
    }
    // Optional, Variant(Int32)
    pub fn number(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            1,
        > as NonRepeatedFieldType>::get_field(
            &self.number,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn number_opt(&self) -> ::std::option::Option<i32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            1,
        > as NonRepeatedFieldType>::get_field_opt(&self.number, &self._bitfield)
    }
    pub fn has_number(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            1,
        > as NonRepeatedFieldType>::get_field_opt(&self.number, &self._bitfield)
        .is_some()
    }
    pub fn number_mut(&mut self) -> &mut i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            1,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.number,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_number(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            1,
        > as NonRepeatedFieldType>::clear(&mut self.number, &mut self._bitfield)
    }
    // Optional, LengthDelimited(Message(Fqtn(".google.protobuf.EnumValueOptions")))
    pub fn options(
        &self,
    ) -> ::std::option::Option<&_puroro_root::google::protobuf::EnumValueOptions> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::EnumValueOptions,
        > as NonRepeatedFieldType>::get_field(
            &self.options,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn options_opt(
        &self,
    ) -> ::std::option::Option<&_puroro_root::google::protobuf::EnumValueOptions> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::EnumValueOptions,
        > as NonRepeatedFieldType>::get_field_opt(&self.options, &self._bitfield)
    }
    pub fn has_options(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::EnumValueOptions,
        > as NonRepeatedFieldType>::get_field_opt(&self.options, &self._bitfield)
        .is_some()
    }
    pub fn options_mut(&mut self) -> &mut _puroro_root::google::protobuf::EnumValueOptions {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::EnumValueOptions,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.options,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_options(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::EnumValueOptions,
        > as NonRepeatedFieldType>::clear(&mut self.options, &mut self._bitfield)
    }
}

impl self::_puroro::Message for EnumValueDescriptorProto {
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
                    &mut self.name,
                    &mut self._bitfield,
                    field_data,
                )?,
                2 => <self::_puroro::internal::field_type::OptionalNumericalField::<i32, self::_puroro::tags::Int32, 1> as FieldType>::deser_from_iter(
                    &mut self.number,
                    &mut self._bitfield,
                    field_data,
                )?,
                3 => <self::_puroro::internal::field_type::SingularHeapMessageField::<_puroro_root::google::protobuf::EnumValueOptions> as FieldType>::deser_from_iter(
                    &mut self.options,
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
            &self.name,
            &self._bitfield,
            1,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            1,
        > as FieldType>::ser_to_write(&self.number, &self._bitfield, 2, out)?;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::EnumValueOptions,
        > as FieldType>::ser_to_write(&self.options, &self._bitfield, 3, out)?;

        Ok(())
    }
}

impl ::std::clone::Clone for EnumValueDescriptorProto {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::clone::Clone;
        Self {
            name: Clone::clone(&self.name),
            number: Clone::clone(&self.number),
            options: Clone::clone(&self.options),

            _bitfield: Clone::clone(&self._bitfield),
        }
    }
}

impl ::std::fmt::Debug for EnumValueDescriptorProto {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct("EnumValueDescriptorProto")
            .field("name", &self.name())
            .field("number", &self.number())
            .field("options", &self.options())
            .finish()
    }
}

impl ::std::cmp::PartialEq for EnumValueDescriptorProto {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;

        true && self.name_opt() == rhs.name_opt()
            && self.number_opt() == rhs.number_opt()
            && self.options_opt() == rhs.options_opt()
    }
}

impl ::std::ops::Drop for EnumValueDescriptorProto {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
    }
}

#[derive(Default)]
pub struct ServiceDescriptorProto {
    // Optional, LengthDelimited(String)
    name: self::_puroro::internal::field_type::OptionalStringField<0>,
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.MethodDescriptorProto")))
    method: self::_puroro::internal::field_type::RepeatedMessageField<
        _puroro_root::google::protobuf::MethodDescriptorProto,
    >,
    // Optional, LengthDelimited(Message(Fqtn(".google.protobuf.ServiceOptions")))
    options: self::_puroro::internal::field_type::SingularHeapMessageField<
        _puroro_root::google::protobuf::ServiceOptions,
    >,

    _bitfield: self::_puroro::bitvec::BitArray<1>,
}

impl ServiceDescriptorProto {
    // Optional, LengthDelimited(String)
    pub fn name(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field(
            &self.name, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn name_opt(&self) -> ::std::option::Option<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field_opt(
            &self.name, &self._bitfield,
        )
    }
    pub fn has_name(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field_opt(
            &self.name, &self._bitfield,
        ).is_some()
    }
    pub fn name_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::mut_field(
            &mut self.name, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_name(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField<0> as NonRepeatedFieldType>::clear(
            &mut self.name,
            &mut self._bitfield,
        )
    }
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.MethodDescriptorProto")))
    pub fn method(&self) -> &[_puroro_root::google::protobuf::MethodDescriptorProto] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::MethodDescriptorProto,
        > as RepeatedFieldType>::get_field(&self.method, &self._bitfield)
    }
    pub fn method_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec<_puroro_root::google::protobuf::MethodDescriptorProto> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::MethodDescriptorProto,
        > as RepeatedFieldType>::mut_field(&mut self.method, &mut self._bitfield)
    }
    pub fn clear_method(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::MethodDescriptorProto,
        > as RepeatedFieldType>::clear(&mut self.method, &mut self._bitfield)
    }
    // Optional, LengthDelimited(Message(Fqtn(".google.protobuf.ServiceOptions")))
    pub fn options(
        &self,
    ) -> ::std::option::Option<&_puroro_root::google::protobuf::ServiceOptions> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::ServiceOptions,
        > as NonRepeatedFieldType>::get_field(
            &self.options,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn options_opt(
        &self,
    ) -> ::std::option::Option<&_puroro_root::google::protobuf::ServiceOptions> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::ServiceOptions,
        > as NonRepeatedFieldType>::get_field_opt(&self.options, &self._bitfield)
    }
    pub fn has_options(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::ServiceOptions,
        > as NonRepeatedFieldType>::get_field_opt(&self.options, &self._bitfield)
        .is_some()
    }
    pub fn options_mut(&mut self) -> &mut _puroro_root::google::protobuf::ServiceOptions {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::ServiceOptions,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.options,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_options(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::ServiceOptions,
        > as NonRepeatedFieldType>::clear(&mut self.options, &mut self._bitfield)
    }
}

impl self::_puroro::Message for ServiceDescriptorProto {
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
                    &mut self.name,
                    &mut self._bitfield,
                    field_data,
                )?,
                2 => <self::_puroro::internal::field_type::RepeatedMessageField::<_puroro_root::google::protobuf::MethodDescriptorProto> as FieldType>::deser_from_iter(
                    &mut self.method,
                    &mut self._bitfield,
                    field_data,
                )?,
                3 => <self::_puroro::internal::field_type::SingularHeapMessageField::<_puroro_root::google::protobuf::ServiceOptions> as FieldType>::deser_from_iter(
                    &mut self.options,
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
            &self.name,
            &self._bitfield,
            1,
            out,
        )?;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::MethodDescriptorProto,
        > as FieldType>::ser_to_write(&self.method, &self._bitfield, 2, out)?;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::ServiceOptions,
        > as FieldType>::ser_to_write(&self.options, &self._bitfield, 3, out)?;

        Ok(())
    }
}

impl ::std::clone::Clone for ServiceDescriptorProto {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::clone::Clone;
        Self {
            name: Clone::clone(&self.name),
            method: Clone::clone(&self.method),
            options: Clone::clone(&self.options),

            _bitfield: Clone::clone(&self._bitfield),
        }
    }
}

impl ::std::fmt::Debug for ServiceDescriptorProto {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct("ServiceDescriptorProto")
            .field("name", &self.name())
            .field("method", &self.method())
            .field("options", &self.options())
            .finish()
    }
}

impl ::std::cmp::PartialEq for ServiceDescriptorProto {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;

        true && self.name_opt() == rhs.name_opt()
            && self.method() == rhs.method()
            && self.options_opt() == rhs.options_opt()
    }
}

impl ::std::ops::Drop for ServiceDescriptorProto {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
    }
}

#[derive(Default)]
pub struct MethodDescriptorProto {
    // Optional, LengthDelimited(String)
    name: self::_puroro::internal::field_type::OptionalStringField<0>,
    // Optional, LengthDelimited(String)
    input_type: self::_puroro::internal::field_type::OptionalStringField<1>,
    // Optional, LengthDelimited(String)
    output_type: self::_puroro::internal::field_type::OptionalStringField<2>,
    // Optional, LengthDelimited(Message(Fqtn(".google.protobuf.MethodOptions")))
    options: self::_puroro::internal::field_type::SingularHeapMessageField<
        _puroro_root::google::protobuf::MethodOptions,
    >,
    // Optional, Variant(Bool)
    client_streaming: self::_puroro::internal::field_type::OptionalNumericalField<
        bool,
        self::_puroro::tags::Bool,
        4,
    >,
    // Optional, Variant(Bool)
    server_streaming: self::_puroro::internal::field_type::OptionalNumericalField<
        bool,
        self::_puroro::tags::Bool,
        5,
    >,

    _bitfield: self::_puroro::bitvec::BitArray<1>,
}

impl MethodDescriptorProto {
    // Optional, LengthDelimited(String)
    pub fn name(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field(
            &self.name, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn name_opt(&self) -> ::std::option::Option<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field_opt(
            &self.name, &self._bitfield,
        )
    }
    pub fn has_name(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field_opt(
            &self.name, &self._bitfield,
        ).is_some()
    }
    pub fn name_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::mut_field(
            &mut self.name, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_name(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField<0> as NonRepeatedFieldType>::clear(
            &mut self.name,
            &mut self._bitfield,
        )
    }
    // Optional, LengthDelimited(String)
    pub fn input_type(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<1> as NonRepeatedFieldType>::get_field(
            &self.input_type, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn input_type_opt(&self) -> ::std::option::Option<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<1> as NonRepeatedFieldType>::get_field_opt(
            &self.input_type, &self._bitfield,
        )
    }
    pub fn has_input_type(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<1> as NonRepeatedFieldType>::get_field_opt(
            &self.input_type, &self._bitfield,
        ).is_some()
    }
    pub fn input_type_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<1> as NonRepeatedFieldType>::mut_field(
            &mut self.input_type, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_input_type(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField<1> as NonRepeatedFieldType>::clear(
            &mut self.input_type,
            &mut self._bitfield,
        )
    }
    // Optional, LengthDelimited(String)
    pub fn output_type(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<2> as NonRepeatedFieldType>::get_field(
            &self.output_type, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn output_type_opt(&self) -> ::std::option::Option<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<2> as NonRepeatedFieldType>::get_field_opt(
            &self.output_type, &self._bitfield,
        )
    }
    pub fn has_output_type(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<2> as NonRepeatedFieldType>::get_field_opt(
            &self.output_type, &self._bitfield,
        ).is_some()
    }
    pub fn output_type_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<2> as NonRepeatedFieldType>::mut_field(
            &mut self.output_type, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_output_type(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField<2> as NonRepeatedFieldType>::clear(
            &mut self.output_type,
            &mut self._bitfield,
        )
    }
    // Optional, LengthDelimited(Message(Fqtn(".google.protobuf.MethodOptions")))
    pub fn options(&self) -> ::std::option::Option<&_puroro_root::google::protobuf::MethodOptions> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::MethodOptions,
        > as NonRepeatedFieldType>::get_field(
            &self.options,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn options_opt(
        &self,
    ) -> ::std::option::Option<&_puroro_root::google::protobuf::MethodOptions> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::MethodOptions,
        > as NonRepeatedFieldType>::get_field_opt(&self.options, &self._bitfield)
    }
    pub fn has_options(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::MethodOptions,
        > as NonRepeatedFieldType>::get_field_opt(&self.options, &self._bitfield)
        .is_some()
    }
    pub fn options_mut(&mut self) -> &mut _puroro_root::google::protobuf::MethodOptions {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::MethodOptions,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.options,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_options(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::MethodOptions,
        > as NonRepeatedFieldType>::clear(&mut self.options, &mut self._bitfield)
    }
    // Optional, Variant(Bool)
    pub fn client_streaming(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            4,
        > as NonRepeatedFieldType>::get_field(
            &self.client_streaming,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn client_streaming_opt(&self) -> ::std::option::Option<bool> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            4,
        > as NonRepeatedFieldType>::get_field_opt(&self.client_streaming, &self._bitfield)
    }
    pub fn has_client_streaming(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            4,
        > as NonRepeatedFieldType>::get_field_opt(&self.client_streaming, &self._bitfield)
        .is_some()
    }
    pub fn client_streaming_mut(&mut self) -> &mut bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            4,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.client_streaming,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_client_streaming(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            4,
        > as NonRepeatedFieldType>::clear(&mut self.client_streaming, &mut self._bitfield)
    }
    // Optional, Variant(Bool)
    pub fn server_streaming(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            5,
        > as NonRepeatedFieldType>::get_field(
            &self.server_streaming,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn server_streaming_opt(&self) -> ::std::option::Option<bool> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            5,
        > as NonRepeatedFieldType>::get_field_opt(&self.server_streaming, &self._bitfield)
    }
    pub fn has_server_streaming(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            5,
        > as NonRepeatedFieldType>::get_field_opt(&self.server_streaming, &self._bitfield)
        .is_some()
    }
    pub fn server_streaming_mut(&mut self) -> &mut bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            5,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.server_streaming,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_server_streaming(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            5,
        > as NonRepeatedFieldType>::clear(&mut self.server_streaming, &mut self._bitfield)
    }
}

impl self::_puroro::Message for MethodDescriptorProto {
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
                    &mut self.name,
                    &mut self._bitfield,
                    field_data,
                )?,
                2 => <self::_puroro::internal::field_type::OptionalStringField::<1> as FieldType>::deser_from_iter(
                    &mut self.input_type,
                    &mut self._bitfield,
                    field_data,
                )?,
                3 => <self::_puroro::internal::field_type::OptionalStringField::<2> as FieldType>::deser_from_iter(
                    &mut self.output_type,
                    &mut self._bitfield,
                    field_data,
                )?,
                4 => <self::_puroro::internal::field_type::SingularHeapMessageField::<_puroro_root::google::protobuf::MethodOptions> as FieldType>::deser_from_iter(
                    &mut self.options,
                    &mut self._bitfield,
                    field_data,
                )?,
                5 => <self::_puroro::internal::field_type::OptionalNumericalField::<bool, self::_puroro::tags::Bool, 4> as FieldType>::deser_from_iter(
                    &mut self.client_streaming,
                    &mut self._bitfield,
                    field_data,
                )?,
                6 => <self::_puroro::internal::field_type::OptionalNumericalField::<bool, self::_puroro::tags::Bool, 5> as FieldType>::deser_from_iter(
                    &mut self.server_streaming,
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
            &self.name,
            &self._bitfield,
            1,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalStringField<1> as FieldType>::ser_to_write(
            &self.input_type,
            &self._bitfield,
            2,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalStringField<2> as FieldType>::ser_to_write(
            &self.output_type,
            &self._bitfield,
            3,
            out,
        )?;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::google::protobuf::MethodOptions,
        > as FieldType>::ser_to_write(&self.options, &self._bitfield, 4, out)?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            4,
        > as FieldType>::ser_to_write(&self.client_streaming, &self._bitfield, 5, out)?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            5,
        > as FieldType>::ser_to_write(&self.server_streaming, &self._bitfield, 6, out)?;

        Ok(())
    }
}

impl ::std::clone::Clone for MethodDescriptorProto {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::clone::Clone;
        Self {
            name: Clone::clone(&self.name),
            input_type: Clone::clone(&self.input_type),
            output_type: Clone::clone(&self.output_type),
            options: Clone::clone(&self.options),
            client_streaming: Clone::clone(&self.client_streaming),
            server_streaming: Clone::clone(&self.server_streaming),

            _bitfield: Clone::clone(&self._bitfield),
        }
    }
}

impl ::std::fmt::Debug for MethodDescriptorProto {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct("MethodDescriptorProto")
            .field("name", &self.name())
            .field("input_type", &self.input_type())
            .field("output_type", &self.output_type())
            .field("options", &self.options())
            .field("client_streaming", &self.client_streaming())
            .field("server_streaming", &self.server_streaming())
            .finish()
    }
}

impl ::std::cmp::PartialEq for MethodDescriptorProto {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;

        true && self.name_opt() == rhs.name_opt()
            && self.input_type_opt() == rhs.input_type_opt()
            && self.output_type_opt() == rhs.output_type_opt()
            && self.options_opt() == rhs.options_opt()
            && self.client_streaming_opt() == rhs.client_streaming_opt()
            && self.server_streaming_opt() == rhs.server_streaming_opt()
    }
}

impl ::std::ops::Drop for MethodDescriptorProto {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
    }
}

#[derive(Default)]
pub struct FileOptions {
    // Optional, LengthDelimited(String)
    java_package: self::_puroro::internal::field_type::OptionalStringField<0>,
    // Optional, LengthDelimited(String)
    java_outer_classname: self::_puroro::internal::field_type::OptionalStringField<1>,
    // Optional, Variant(Bool)
    java_multiple_files: self::_puroro::internal::field_type::OptionalNumericalField<
        bool,
        self::_puroro::tags::Bool,
        2,
    >,
    // Optional, Variant(Bool)
    java_generate_equals_and_hash: self::_puroro::internal::field_type::OptionalNumericalField<
        bool,
        self::_puroro::tags::Bool,
        3,
    >,
    // Optional, Variant(Bool)
    java_string_check_utf8: self::_puroro::internal::field_type::OptionalNumericalField<
        bool,
        self::_puroro::tags::Bool,
        4,
    >,
    // Optional, Variant(Enum2(Fqtn(".google.protobuf.FileOptions.OptimizeMode")))
    optimize_for: self::_puroro::internal::field_type::OptionalNumericalField<
        _puroro_root::google::protobuf::file_options::OptimizeMode,
        self::_puroro::tags::Enum2<_puroro_root::google::protobuf::file_options::OptimizeMode>,
        5,
    >,
    // Optional, LengthDelimited(String)
    go_package: self::_puroro::internal::field_type::OptionalStringField<6>,
    // Optional, Variant(Bool)
    cc_generic_services: self::_puroro::internal::field_type::OptionalNumericalField<
        bool,
        self::_puroro::tags::Bool,
        7,
    >,
    // Optional, Variant(Bool)
    java_generic_services: self::_puroro::internal::field_type::OptionalNumericalField<
        bool,
        self::_puroro::tags::Bool,
        8,
    >,
    // Optional, Variant(Bool)
    py_generic_services: self::_puroro::internal::field_type::OptionalNumericalField<
        bool,
        self::_puroro::tags::Bool,
        9,
    >,
    // Optional, Variant(Bool)
    php_generic_services: self::_puroro::internal::field_type::OptionalNumericalField<
        bool,
        self::_puroro::tags::Bool,
        10,
    >,
    // Optional, Variant(Bool)
    deprecated: self::_puroro::internal::field_type::OptionalNumericalField<
        bool,
        self::_puroro::tags::Bool,
        11,
    >,
    // Optional, Variant(Bool)
    cc_enable_arenas: self::_puroro::internal::field_type::OptionalNumericalField<
        bool,
        self::_puroro::tags::Bool,
        12,
    >,
    // Optional, LengthDelimited(String)
    objc_class_prefix: self::_puroro::internal::field_type::OptionalStringField<13>,
    // Optional, LengthDelimited(String)
    csharp_namespace: self::_puroro::internal::field_type::OptionalStringField<14>,
    // Optional, LengthDelimited(String)
    swift_prefix: self::_puroro::internal::field_type::OptionalStringField<15>,
    // Optional, LengthDelimited(String)
    php_class_prefix: self::_puroro::internal::field_type::OptionalStringField<16>,
    // Optional, LengthDelimited(String)
    php_namespace: self::_puroro::internal::field_type::OptionalStringField<17>,
    // Optional, LengthDelimited(String)
    php_metadata_namespace: self::_puroro::internal::field_type::OptionalStringField<18>,
    // Optional, LengthDelimited(String)
    ruby_package: self::_puroro::internal::field_type::OptionalStringField<19>,
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.UninterpretedOption")))
    uninterpreted_option: self::_puroro::internal::field_type::RepeatedMessageField<
        _puroro_root::google::protobuf::UninterpretedOption,
    >,

    _bitfield: self::_puroro::bitvec::BitArray<3>,
}

impl FileOptions {
    // Optional, LengthDelimited(String)
    pub fn java_package(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field(
            &self.java_package, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn java_package_opt(&self) -> ::std::option::Option<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field_opt(
            &self.java_package, &self._bitfield,
        )
    }
    pub fn has_java_package(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field_opt(
            &self.java_package, &self._bitfield,
        ).is_some()
    }
    pub fn java_package_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::mut_field(
            &mut self.java_package, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_java_package(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField<0> as NonRepeatedFieldType>::clear(
            &mut self.java_package,
            &mut self._bitfield,
        )
    }
    // Optional, LengthDelimited(String)
    pub fn java_outer_classname(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<1> as NonRepeatedFieldType>::get_field(
            &self.java_outer_classname, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn java_outer_classname_opt(&self) -> ::std::option::Option<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<1> as NonRepeatedFieldType>::get_field_opt(
            &self.java_outer_classname, &self._bitfield,
        )
    }
    pub fn has_java_outer_classname(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<1> as NonRepeatedFieldType>::get_field_opt(
            &self.java_outer_classname, &self._bitfield,
        ).is_some()
    }
    pub fn java_outer_classname_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<1> as NonRepeatedFieldType>::mut_field(
            &mut self.java_outer_classname, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_java_outer_classname(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField<1> as NonRepeatedFieldType>::clear(
            &mut self.java_outer_classname,
            &mut self._bitfield,
        )
    }
    // Optional, Variant(Bool)
    pub fn java_multiple_files(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            2,
        > as NonRepeatedFieldType>::get_field(
            &self.java_multiple_files,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn java_multiple_files_opt(&self) -> ::std::option::Option<bool> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            2,
        > as NonRepeatedFieldType>::get_field_opt(&self.java_multiple_files, &self._bitfield)
    }
    pub fn has_java_multiple_files(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            2,
        > as NonRepeatedFieldType>::get_field_opt(&self.java_multiple_files, &self._bitfield)
        .is_some()
    }
    pub fn java_multiple_files_mut(&mut self) -> &mut bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            2,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.java_multiple_files,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_java_multiple_files(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            2,
        > as NonRepeatedFieldType>::clear(&mut self.java_multiple_files, &mut self._bitfield)
    }
    // Optional, Variant(Bool)
    pub fn java_generate_equals_and_hash(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            3,
        > as NonRepeatedFieldType>::get_field(
            &self.java_generate_equals_and_hash,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn java_generate_equals_and_hash_opt(&self) -> ::std::option::Option<bool> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            3,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.java_generate_equals_and_hash,
            &self._bitfield,
        )
    }
    pub fn has_java_generate_equals_and_hash(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            3,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.java_generate_equals_and_hash,
            &self._bitfield,
        )
        .is_some()
    }
    pub fn java_generate_equals_and_hash_mut(&mut self) -> &mut bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            3,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.java_generate_equals_and_hash,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_java_generate_equals_and_hash(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            3,
        > as NonRepeatedFieldType>::clear(
            &mut self.java_generate_equals_and_hash,
            &mut self._bitfield,
        )
    }
    // Optional, Variant(Bool)
    pub fn java_string_check_utf8(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            4,
        > as NonRepeatedFieldType>::get_field(
            &self.java_string_check_utf8,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn java_string_check_utf8_opt(&self) -> ::std::option::Option<bool> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            4,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.java_string_check_utf8, &self._bitfield
        )
    }
    pub fn has_java_string_check_utf8(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            4,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.java_string_check_utf8, &self._bitfield
        )
        .is_some()
    }
    pub fn java_string_check_utf8_mut(&mut self) -> &mut bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            4,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.java_string_check_utf8,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_java_string_check_utf8(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            4,
        > as NonRepeatedFieldType>::clear(
            &mut self.java_string_check_utf8, &mut self._bitfield
        )
    }
    // Optional, Variant(Enum2(Fqtn(".google.protobuf.FileOptions.OptimizeMode")))
    pub fn optimize_for(&self) -> _puroro_root::google::protobuf::file_options::OptimizeMode {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::google::protobuf::file_options::OptimizeMode,
            self::_puroro::tags::Enum2<_puroro_root::google::protobuf::file_options::OptimizeMode>,
            5,
        > as NonRepeatedFieldType>::get_field(
            &self.optimize_for,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn optimize_for_opt(
        &self,
    ) -> ::std::option::Option<_puroro_root::google::protobuf::file_options::OptimizeMode> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::google::protobuf::file_options::OptimizeMode,
            self::_puroro::tags::Enum2<_puroro_root::google::protobuf::file_options::OptimizeMode>,
            5,
        > as NonRepeatedFieldType>::get_field_opt(&self.optimize_for, &self._bitfield)
    }
    pub fn has_optimize_for(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::google::protobuf::file_options::OptimizeMode,
            self::_puroro::tags::Enum2<_puroro_root::google::protobuf::file_options::OptimizeMode>,
            5,
        > as NonRepeatedFieldType>::get_field_opt(&self.optimize_for, &self._bitfield)
        .is_some()
    }
    pub fn optimize_for_mut(
        &mut self,
    ) -> &mut _puroro_root::google::protobuf::file_options::OptimizeMode {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::google::protobuf::file_options::OptimizeMode,
            self::_puroro::tags::Enum2<_puroro_root::google::protobuf::file_options::OptimizeMode>,
            5,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.optimize_for,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_optimize_for(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::google::protobuf::file_options::OptimizeMode,
            self::_puroro::tags::Enum2<_puroro_root::google::protobuf::file_options::OptimizeMode>,
            5,
        > as NonRepeatedFieldType>::clear(&mut self.optimize_for, &mut self._bitfield)
    }
    // Optional, LengthDelimited(String)
    pub fn go_package(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<6> as NonRepeatedFieldType>::get_field(
            &self.go_package, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn go_package_opt(&self) -> ::std::option::Option<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<6> as NonRepeatedFieldType>::get_field_opt(
            &self.go_package, &self._bitfield,
        )
    }
    pub fn has_go_package(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<6> as NonRepeatedFieldType>::get_field_opt(
            &self.go_package, &self._bitfield,
        ).is_some()
    }
    pub fn go_package_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<6> as NonRepeatedFieldType>::mut_field(
            &mut self.go_package, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_go_package(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField<6> as NonRepeatedFieldType>::clear(
            &mut self.go_package,
            &mut self._bitfield,
        )
    }
    // Optional, Variant(Bool)
    pub fn cc_generic_services(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            7,
        > as NonRepeatedFieldType>::get_field(
            &self.cc_generic_services,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn cc_generic_services_opt(&self) -> ::std::option::Option<bool> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            7,
        > as NonRepeatedFieldType>::get_field_opt(&self.cc_generic_services, &self._bitfield)
    }
    pub fn has_cc_generic_services(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            7,
        > as NonRepeatedFieldType>::get_field_opt(&self.cc_generic_services, &self._bitfield)
        .is_some()
    }
    pub fn cc_generic_services_mut(&mut self) -> &mut bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            7,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.cc_generic_services,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_cc_generic_services(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            7,
        > as NonRepeatedFieldType>::clear(&mut self.cc_generic_services, &mut self._bitfield)
    }
    // Optional, Variant(Bool)
    pub fn java_generic_services(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            8,
        > as NonRepeatedFieldType>::get_field(
            &self.java_generic_services,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn java_generic_services_opt(&self) -> ::std::option::Option<bool> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            8,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.java_generic_services, &self._bitfield
        )
    }
    pub fn has_java_generic_services(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            8,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.java_generic_services, &self._bitfield
        )
        .is_some()
    }
    pub fn java_generic_services_mut(&mut self) -> &mut bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            8,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.java_generic_services,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_java_generic_services(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            8,
        > as NonRepeatedFieldType>::clear(
            &mut self.java_generic_services, &mut self._bitfield
        )
    }
    // Optional, Variant(Bool)
    pub fn py_generic_services(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            9,
        > as NonRepeatedFieldType>::get_field(
            &self.py_generic_services,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn py_generic_services_opt(&self) -> ::std::option::Option<bool> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            9,
        > as NonRepeatedFieldType>::get_field_opt(&self.py_generic_services, &self._bitfield)
    }
    pub fn has_py_generic_services(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            9,
        > as NonRepeatedFieldType>::get_field_opt(&self.py_generic_services, &self._bitfield)
        .is_some()
    }
    pub fn py_generic_services_mut(&mut self) -> &mut bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            9,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.py_generic_services,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_py_generic_services(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            9,
        > as NonRepeatedFieldType>::clear(&mut self.py_generic_services, &mut self._bitfield)
    }
    // Optional, Variant(Bool)
    pub fn php_generic_services(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            10,
        > as NonRepeatedFieldType>::get_field(
            &self.php_generic_services,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn php_generic_services_opt(&self) -> ::std::option::Option<bool> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            10,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.php_generic_services, &self._bitfield
        )
    }
    pub fn has_php_generic_services(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            10,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.php_generic_services, &self._bitfield
        )
        .is_some()
    }
    pub fn php_generic_services_mut(&mut self) -> &mut bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            10,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.php_generic_services,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_php_generic_services(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            10,
        > as NonRepeatedFieldType>::clear(
            &mut self.php_generic_services, &mut self._bitfield
        )
    }
    // Optional, Variant(Bool)
    pub fn deprecated(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            11,
        > as NonRepeatedFieldType>::get_field(
            &self.deprecated,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn deprecated_opt(&self) -> ::std::option::Option<bool> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            11,
        > as NonRepeatedFieldType>::get_field_opt(&self.deprecated, &self._bitfield)
    }
    pub fn has_deprecated(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            11,
        > as NonRepeatedFieldType>::get_field_opt(&self.deprecated, &self._bitfield)
        .is_some()
    }
    pub fn deprecated_mut(&mut self) -> &mut bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            11,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.deprecated,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_deprecated(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            11,
        > as NonRepeatedFieldType>::clear(&mut self.deprecated, &mut self._bitfield)
    }
    // Optional, Variant(Bool)
    pub fn cc_enable_arenas(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            12,
        > as NonRepeatedFieldType>::get_field(
            &self.cc_enable_arenas,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn cc_enable_arenas_opt(&self) -> ::std::option::Option<bool> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            12,
        > as NonRepeatedFieldType>::get_field_opt(&self.cc_enable_arenas, &self._bitfield)
    }
    pub fn has_cc_enable_arenas(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            12,
        > as NonRepeatedFieldType>::get_field_opt(&self.cc_enable_arenas, &self._bitfield)
        .is_some()
    }
    pub fn cc_enable_arenas_mut(&mut self) -> &mut bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            12,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.cc_enable_arenas,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_cc_enable_arenas(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            12,
        > as NonRepeatedFieldType>::clear(&mut self.cc_enable_arenas, &mut self._bitfield)
    }
    // Optional, LengthDelimited(String)
    pub fn objc_class_prefix(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<13> as NonRepeatedFieldType>::get_field(
            &self.objc_class_prefix, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn objc_class_prefix_opt(&self) -> ::std::option::Option<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<13> as NonRepeatedFieldType>::get_field_opt(
            &self.objc_class_prefix, &self._bitfield,
        )
    }
    pub fn has_objc_class_prefix(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<13> as NonRepeatedFieldType>::get_field_opt(
            &self.objc_class_prefix, &self._bitfield,
        ).is_some()
    }
    pub fn objc_class_prefix_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<13> as NonRepeatedFieldType>::mut_field(
            &mut self.objc_class_prefix, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_objc_class_prefix(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<13> as NonRepeatedFieldType>::clear(
            &mut self.objc_class_prefix, &mut self._bitfield,
        )
    }
    // Optional, LengthDelimited(String)
    pub fn csharp_namespace(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<14> as NonRepeatedFieldType>::get_field(
            &self.csharp_namespace, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn csharp_namespace_opt(&self) -> ::std::option::Option<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<14> as NonRepeatedFieldType>::get_field_opt(
            &self.csharp_namespace, &self._bitfield,
        )
    }
    pub fn has_csharp_namespace(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<14> as NonRepeatedFieldType>::get_field_opt(
            &self.csharp_namespace, &self._bitfield,
        ).is_some()
    }
    pub fn csharp_namespace_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<14> as NonRepeatedFieldType>::mut_field(
            &mut self.csharp_namespace, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_csharp_namespace(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<14> as NonRepeatedFieldType>::clear(
            &mut self.csharp_namespace, &mut self._bitfield,
        )
    }
    // Optional, LengthDelimited(String)
    pub fn swift_prefix(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<15> as NonRepeatedFieldType>::get_field(
            &self.swift_prefix, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn swift_prefix_opt(&self) -> ::std::option::Option<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<15> as NonRepeatedFieldType>::get_field_opt(
            &self.swift_prefix, &self._bitfield,
        )
    }
    pub fn has_swift_prefix(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<15> as NonRepeatedFieldType>::get_field_opt(
            &self.swift_prefix, &self._bitfield,
        ).is_some()
    }
    pub fn swift_prefix_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<15> as NonRepeatedFieldType>::mut_field(
            &mut self.swift_prefix, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_swift_prefix(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<15> as NonRepeatedFieldType>::clear(
            &mut self.swift_prefix, &mut self._bitfield,
        )
    }
    // Optional, LengthDelimited(String)
    pub fn php_class_prefix(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<16> as NonRepeatedFieldType>::get_field(
            &self.php_class_prefix, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn php_class_prefix_opt(&self) -> ::std::option::Option<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<16> as NonRepeatedFieldType>::get_field_opt(
            &self.php_class_prefix, &self._bitfield,
        )
    }
    pub fn has_php_class_prefix(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<16> as NonRepeatedFieldType>::get_field_opt(
            &self.php_class_prefix, &self._bitfield,
        ).is_some()
    }
    pub fn php_class_prefix_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<16> as NonRepeatedFieldType>::mut_field(
            &mut self.php_class_prefix, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_php_class_prefix(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<16> as NonRepeatedFieldType>::clear(
            &mut self.php_class_prefix, &mut self._bitfield,
        )
    }
    // Optional, LengthDelimited(String)
    pub fn php_namespace(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<17> as NonRepeatedFieldType>::get_field(
            &self.php_namespace, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn php_namespace_opt(&self) -> ::std::option::Option<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<17> as NonRepeatedFieldType>::get_field_opt(
            &self.php_namespace, &self._bitfield,
        )
    }
    pub fn has_php_namespace(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<17> as NonRepeatedFieldType>::get_field_opt(
            &self.php_namespace, &self._bitfield,
        ).is_some()
    }
    pub fn php_namespace_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<17> as NonRepeatedFieldType>::mut_field(
            &mut self.php_namespace, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_php_namespace(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<17> as NonRepeatedFieldType>::clear(
            &mut self.php_namespace, &mut self._bitfield,
        )
    }
    // Optional, LengthDelimited(String)
    pub fn php_metadata_namespace(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<18> as NonRepeatedFieldType>::get_field(
            &self.php_metadata_namespace, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn php_metadata_namespace_opt(&self) -> ::std::option::Option<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<18> as NonRepeatedFieldType>::get_field_opt(
            &self.php_metadata_namespace, &self._bitfield,
        )
    }
    pub fn has_php_metadata_namespace(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<18> as NonRepeatedFieldType>::get_field_opt(
            &self.php_metadata_namespace, &self._bitfield,
        ).is_some()
    }
    pub fn php_metadata_namespace_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<18> as NonRepeatedFieldType>::mut_field(
            &mut self.php_metadata_namespace, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_php_metadata_namespace(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<18> as NonRepeatedFieldType>::clear(
            &mut self.php_metadata_namespace, &mut self._bitfield,
        )
    }
    // Optional, LengthDelimited(String)
    pub fn ruby_package(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<19> as NonRepeatedFieldType>::get_field(
            &self.ruby_package, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn ruby_package_opt(&self) -> ::std::option::Option<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<19> as NonRepeatedFieldType>::get_field_opt(
            &self.ruby_package, &self._bitfield,
        )
    }
    pub fn has_ruby_package(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<19> as NonRepeatedFieldType>::get_field_opt(
            &self.ruby_package, &self._bitfield,
        ).is_some()
    }
    pub fn ruby_package_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<19> as NonRepeatedFieldType>::mut_field(
            &mut self.ruby_package, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_ruby_package(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<19> as NonRepeatedFieldType>::clear(
            &mut self.ruby_package, &mut self._bitfield,
        )
    }
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.UninterpretedOption")))
    pub fn uninterpreted_option(&self) -> &[_puroro_root::google::protobuf::UninterpretedOption] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::get_field(&self.uninterpreted_option, &self._bitfield)
    }
    pub fn uninterpreted_option_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec<_puroro_root::google::protobuf::UninterpretedOption> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::mut_field(
            &mut self.uninterpreted_option, &mut self._bitfield
        )
    }
    pub fn clear_uninterpreted_option(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::clear(&mut self.uninterpreted_option, &mut self._bitfield)
    }
}

impl self::_puroro::Message for FileOptions {
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
                    &mut self.java_package,
                    &mut self._bitfield,
                    field_data,
                )?,
                8 => <self::_puroro::internal::field_type::OptionalStringField::<1> as FieldType>::deser_from_iter(
                    &mut self.java_outer_classname,
                    &mut self._bitfield,
                    field_data,
                )?,
                10 => <self::_puroro::internal::field_type::OptionalNumericalField::<bool, self::_puroro::tags::Bool, 2> as FieldType>::deser_from_iter(
                    &mut self.java_multiple_files,
                    &mut self._bitfield,
                    field_data,
                )?,
                20 => <self::_puroro::internal::field_type::OptionalNumericalField::<bool, self::_puroro::tags::Bool, 3> as FieldType>::deser_from_iter(
                    &mut self.java_generate_equals_and_hash,
                    &mut self._bitfield,
                    field_data,
                )?,
                27 => <self::_puroro::internal::field_type::OptionalNumericalField::<bool, self::_puroro::tags::Bool, 4> as FieldType>::deser_from_iter(
                    &mut self.java_string_check_utf8,
                    &mut self._bitfield,
                    field_data,
                )?,
                9 => <self::_puroro::internal::field_type::OptionalNumericalField::<_puroro_root::google::protobuf::file_options::OptimizeMode, self::_puroro::tags::Enum2<_puroro_root::google::protobuf::file_options::OptimizeMode>, 5> as FieldType>::deser_from_iter(
                    &mut self.optimize_for,
                    &mut self._bitfield,
                    field_data,
                )?,
                11 => <self::_puroro::internal::field_type::OptionalStringField::<6> as FieldType>::deser_from_iter(
                    &mut self.go_package,
                    &mut self._bitfield,
                    field_data,
                )?,
                16 => <self::_puroro::internal::field_type::OptionalNumericalField::<bool, self::_puroro::tags::Bool, 7> as FieldType>::deser_from_iter(
                    &mut self.cc_generic_services,
                    &mut self._bitfield,
                    field_data,
                )?,
                17 => <self::_puroro::internal::field_type::OptionalNumericalField::<bool, self::_puroro::tags::Bool, 8> as FieldType>::deser_from_iter(
                    &mut self.java_generic_services,
                    &mut self._bitfield,
                    field_data,
                )?,
                18 => <self::_puroro::internal::field_type::OptionalNumericalField::<bool, self::_puroro::tags::Bool, 9> as FieldType>::deser_from_iter(
                    &mut self.py_generic_services,
                    &mut self._bitfield,
                    field_data,
                )?,
                42 => <self::_puroro::internal::field_type::OptionalNumericalField::<bool, self::_puroro::tags::Bool, 10> as FieldType>::deser_from_iter(
                    &mut self.php_generic_services,
                    &mut self._bitfield,
                    field_data,
                )?,
                23 => <self::_puroro::internal::field_type::OptionalNumericalField::<bool, self::_puroro::tags::Bool, 11> as FieldType>::deser_from_iter(
                    &mut self.deprecated,
                    &mut self._bitfield,
                    field_data,
                )?,
                31 => <self::_puroro::internal::field_type::OptionalNumericalField::<bool, self::_puroro::tags::Bool, 12> as FieldType>::deser_from_iter(
                    &mut self.cc_enable_arenas,
                    &mut self._bitfield,
                    field_data,
                )?,
                36 => <self::_puroro::internal::field_type::OptionalStringField::<13> as FieldType>::deser_from_iter(
                    &mut self.objc_class_prefix,
                    &mut self._bitfield,
                    field_data,
                )?,
                37 => <self::_puroro::internal::field_type::OptionalStringField::<14> as FieldType>::deser_from_iter(
                    &mut self.csharp_namespace,
                    &mut self._bitfield,
                    field_data,
                )?,
                39 => <self::_puroro::internal::field_type::OptionalStringField::<15> as FieldType>::deser_from_iter(
                    &mut self.swift_prefix,
                    &mut self._bitfield,
                    field_data,
                )?,
                40 => <self::_puroro::internal::field_type::OptionalStringField::<16> as FieldType>::deser_from_iter(
                    &mut self.php_class_prefix,
                    &mut self._bitfield,
                    field_data,
                )?,
                41 => <self::_puroro::internal::field_type::OptionalStringField::<17> as FieldType>::deser_from_iter(
                    &mut self.php_namespace,
                    &mut self._bitfield,
                    field_data,
                )?,
                44 => <self::_puroro::internal::field_type::OptionalStringField::<18> as FieldType>::deser_from_iter(
                    &mut self.php_metadata_namespace,
                    &mut self._bitfield,
                    field_data,
                )?,
                45 => <self::_puroro::internal::field_type::OptionalStringField::<19> as FieldType>::deser_from_iter(
                    &mut self.ruby_package,
                    &mut self._bitfield,
                    field_data,
                )?,
                999 => <self::_puroro::internal::field_type::RepeatedMessageField::<_puroro_root::google::protobuf::UninterpretedOption> as FieldType>::deser_from_iter(
                    &mut self.uninterpreted_option,
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
            &self.java_package,
            &self._bitfield,
            1,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalStringField<1> as FieldType>::ser_to_write(
            &self.java_outer_classname,
            &self._bitfield,
            8,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            2,
        > as FieldType>::ser_to_write(&self.java_multiple_files, &self._bitfield, 10, out)?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            3,
        > as FieldType>::ser_to_write(
            &self.java_generate_equals_and_hash,
            &self._bitfield,
            20,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            4,
        > as FieldType>::ser_to_write(
            &self.java_string_check_utf8, &self._bitfield, 27, out
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::google::protobuf::file_options::OptimizeMode,
            self::_puroro::tags::Enum2<_puroro_root::google::protobuf::file_options::OptimizeMode>,
            5,
        > as FieldType>::ser_to_write(&self.optimize_for, &self._bitfield, 9, out)?;
        <self::_puroro::internal::field_type::OptionalStringField<6> as FieldType>::ser_to_write(
            &self.go_package,
            &self._bitfield,
            11,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            7,
        > as FieldType>::ser_to_write(&self.cc_generic_services, &self._bitfield, 16, out)?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            8,
        > as FieldType>::ser_to_write(
            &self.java_generic_services, &self._bitfield, 17, out
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            9,
        > as FieldType>::ser_to_write(&self.py_generic_services, &self._bitfield, 18, out)?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            10,
        > as FieldType>::ser_to_write(&self.php_generic_services, &self._bitfield, 42, out)?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            11,
        > as FieldType>::ser_to_write(&self.deprecated, &self._bitfield, 23, out)?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            12,
        > as FieldType>::ser_to_write(&self.cc_enable_arenas, &self._bitfield, 31, out)?;
        <self::_puroro::internal::field_type::OptionalStringField<13> as FieldType>::ser_to_write(
            &self.objc_class_prefix,
            &self._bitfield,
            36,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalStringField<14> as FieldType>::ser_to_write(
            &self.csharp_namespace,
            &self._bitfield,
            37,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalStringField<15> as FieldType>::ser_to_write(
            &self.swift_prefix,
            &self._bitfield,
            39,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalStringField<16> as FieldType>::ser_to_write(
            &self.php_class_prefix,
            &self._bitfield,
            40,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalStringField<17> as FieldType>::ser_to_write(
            &self.php_namespace,
            &self._bitfield,
            41,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalStringField<18> as FieldType>::ser_to_write(
            &self.php_metadata_namespace,
            &self._bitfield,
            44,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalStringField<19> as FieldType>::ser_to_write(
            &self.ruby_package,
            &self._bitfield,
            45,
            out,
        )?;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::UninterpretedOption,
        > as FieldType>::ser_to_write(
            &self.uninterpreted_option, &self._bitfield, 999, out
        )?;

        Ok(())
    }
}

impl ::std::clone::Clone for FileOptions {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::clone::Clone;
        Self {
            java_package: Clone::clone(&self.java_package),
            java_outer_classname: Clone::clone(&self.java_outer_classname),
            java_multiple_files: Clone::clone(&self.java_multiple_files),
            java_generate_equals_and_hash: Clone::clone(&self.java_generate_equals_and_hash),
            java_string_check_utf8: Clone::clone(&self.java_string_check_utf8),
            optimize_for: Clone::clone(&self.optimize_for),
            go_package: Clone::clone(&self.go_package),
            cc_generic_services: Clone::clone(&self.cc_generic_services),
            java_generic_services: Clone::clone(&self.java_generic_services),
            py_generic_services: Clone::clone(&self.py_generic_services),
            php_generic_services: Clone::clone(&self.php_generic_services),
            deprecated: Clone::clone(&self.deprecated),
            cc_enable_arenas: Clone::clone(&self.cc_enable_arenas),
            objc_class_prefix: Clone::clone(&self.objc_class_prefix),
            csharp_namespace: Clone::clone(&self.csharp_namespace),
            swift_prefix: Clone::clone(&self.swift_prefix),
            php_class_prefix: Clone::clone(&self.php_class_prefix),
            php_namespace: Clone::clone(&self.php_namespace),
            php_metadata_namespace: Clone::clone(&self.php_metadata_namespace),
            ruby_package: Clone::clone(&self.ruby_package),
            uninterpreted_option: Clone::clone(&self.uninterpreted_option),

            _bitfield: Clone::clone(&self._bitfield),
        }
    }
}

impl ::std::fmt::Debug for FileOptions {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct("FileOptions")
            .field("java_package", &self.java_package())
            .field("java_outer_classname", &self.java_outer_classname())
            .field("java_multiple_files", &self.java_multiple_files())
            .field(
                "java_generate_equals_and_hash",
                &self.java_generate_equals_and_hash(),
            )
            .field("java_string_check_utf8", &self.java_string_check_utf8())
            .field("optimize_for", &self.optimize_for())
            .field("go_package", &self.go_package())
            .field("cc_generic_services", &self.cc_generic_services())
            .field("java_generic_services", &self.java_generic_services())
            .field("py_generic_services", &self.py_generic_services())
            .field("php_generic_services", &self.php_generic_services())
            .field("deprecated", &self.deprecated())
            .field("cc_enable_arenas", &self.cc_enable_arenas())
            .field("objc_class_prefix", &self.objc_class_prefix())
            .field("csharp_namespace", &self.csharp_namespace())
            .field("swift_prefix", &self.swift_prefix())
            .field("php_class_prefix", &self.php_class_prefix())
            .field("php_namespace", &self.php_namespace())
            .field("php_metadata_namespace", &self.php_metadata_namespace())
            .field("ruby_package", &self.ruby_package())
            .field("uninterpreted_option", &self.uninterpreted_option())
            .finish()
    }
}

impl ::std::cmp::PartialEq for FileOptions {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;

        true && self.java_package_opt() == rhs.java_package_opt()
            && self.java_outer_classname_opt() == rhs.java_outer_classname_opt()
            && self.java_multiple_files_opt() == rhs.java_multiple_files_opt()
            && self.java_generate_equals_and_hash_opt() == rhs.java_generate_equals_and_hash_opt()
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
    }
}

impl ::std::ops::Drop for FileOptions {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
    }
}

#[derive(Default)]
pub struct MessageOptions {
    // Optional, Variant(Bool)
    message_set_wire_format: self::_puroro::internal::field_type::OptionalNumericalField<
        bool,
        self::_puroro::tags::Bool,
        0,
    >,
    // Optional, Variant(Bool)
    no_standard_descriptor_accessor: self::_puroro::internal::field_type::OptionalNumericalField<
        bool,
        self::_puroro::tags::Bool,
        1,
    >,
    // Optional, Variant(Bool)
    deprecated: self::_puroro::internal::field_type::OptionalNumericalField<
        bool,
        self::_puroro::tags::Bool,
        2,
    >,
    // Optional, Variant(Bool)
    map_entry: self::_puroro::internal::field_type::OptionalNumericalField<
        bool,
        self::_puroro::tags::Bool,
        3,
    >,
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.UninterpretedOption")))
    uninterpreted_option: self::_puroro::internal::field_type::RepeatedMessageField<
        _puroro_root::google::protobuf::UninterpretedOption,
    >,

    _bitfield: self::_puroro::bitvec::BitArray<1>,
}

impl MessageOptions {
    // Optional, Variant(Bool)
    pub fn message_set_wire_format(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            0,
        > as NonRepeatedFieldType>::get_field(
            &self.message_set_wire_format,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn message_set_wire_format_opt(&self) -> ::std::option::Option<bool> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            0,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.message_set_wire_format, &self._bitfield
        )
    }
    pub fn has_message_set_wire_format(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            0,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.message_set_wire_format, &self._bitfield
        )
        .is_some()
    }
    pub fn message_set_wire_format_mut(&mut self) -> &mut bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            0,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.message_set_wire_format,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_message_set_wire_format(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            0,
        > as NonRepeatedFieldType>::clear(
            &mut self.message_set_wire_format, &mut self._bitfield
        )
    }
    // Optional, Variant(Bool)
    pub fn no_standard_descriptor_accessor(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            1,
        > as NonRepeatedFieldType>::get_field(
            &self.no_standard_descriptor_accessor,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn no_standard_descriptor_accessor_opt(&self) -> ::std::option::Option<bool> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            1,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.no_standard_descriptor_accessor,
            &self._bitfield,
        )
    }
    pub fn has_no_standard_descriptor_accessor(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            1,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.no_standard_descriptor_accessor,
            &self._bitfield,
        )
        .is_some()
    }
    pub fn no_standard_descriptor_accessor_mut(&mut self) -> &mut bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            1,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.no_standard_descriptor_accessor,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_no_standard_descriptor_accessor(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            1,
        > as NonRepeatedFieldType>::clear(
            &mut self.no_standard_descriptor_accessor,
            &mut self._bitfield,
        )
    }
    // Optional, Variant(Bool)
    pub fn deprecated(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            2,
        > as NonRepeatedFieldType>::get_field(
            &self.deprecated,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn deprecated_opt(&self) -> ::std::option::Option<bool> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            2,
        > as NonRepeatedFieldType>::get_field_opt(&self.deprecated, &self._bitfield)
    }
    pub fn has_deprecated(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            2,
        > as NonRepeatedFieldType>::get_field_opt(&self.deprecated, &self._bitfield)
        .is_some()
    }
    pub fn deprecated_mut(&mut self) -> &mut bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            2,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.deprecated,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_deprecated(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            2,
        > as NonRepeatedFieldType>::clear(&mut self.deprecated, &mut self._bitfield)
    }
    // Optional, Variant(Bool)
    pub fn map_entry(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            3,
        > as NonRepeatedFieldType>::get_field(
            &self.map_entry,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn map_entry_opt(&self) -> ::std::option::Option<bool> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            3,
        > as NonRepeatedFieldType>::get_field_opt(&self.map_entry, &self._bitfield)
    }
    pub fn has_map_entry(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            3,
        > as NonRepeatedFieldType>::get_field_opt(&self.map_entry, &self._bitfield)
        .is_some()
    }
    pub fn map_entry_mut(&mut self) -> &mut bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            3,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.map_entry,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_map_entry(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            3,
        > as NonRepeatedFieldType>::clear(&mut self.map_entry, &mut self._bitfield)
    }
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.UninterpretedOption")))
    pub fn uninterpreted_option(&self) -> &[_puroro_root::google::protobuf::UninterpretedOption] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::get_field(&self.uninterpreted_option, &self._bitfield)
    }
    pub fn uninterpreted_option_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec<_puroro_root::google::protobuf::UninterpretedOption> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::mut_field(
            &mut self.uninterpreted_option, &mut self._bitfield
        )
    }
    pub fn clear_uninterpreted_option(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::clear(&mut self.uninterpreted_option, &mut self._bitfield)
    }
}

impl self::_puroro::Message for MessageOptions {
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
                1 => <self::_puroro::internal::field_type::OptionalNumericalField<
                    bool,
                    self::_puroro::tags::Bool,
                    0,
                > as FieldType>::deser_from_iter(
                    &mut self.message_set_wire_format,
                    &mut self._bitfield,
                    field_data,
                )?,
                2 => <self::_puroro::internal::field_type::OptionalNumericalField<
                    bool,
                    self::_puroro::tags::Bool,
                    1,
                > as FieldType>::deser_from_iter(
                    &mut self.no_standard_descriptor_accessor,
                    &mut self._bitfield,
                    field_data,
                )?,
                3 => <self::_puroro::internal::field_type::OptionalNumericalField<
                    bool,
                    self::_puroro::tags::Bool,
                    2,
                > as FieldType>::deser_from_iter(
                    &mut self.deprecated,
                    &mut self._bitfield,
                    field_data,
                )?,
                7 => <self::_puroro::internal::field_type::OptionalNumericalField<
                    bool,
                    self::_puroro::tags::Bool,
                    3,
                > as FieldType>::deser_from_iter(
                    &mut self.map_entry,
                    &mut self._bitfield,
                    field_data,
                )?,
                999 => <self::_puroro::internal::field_type::RepeatedMessageField<
                    _puroro_root::google::protobuf::UninterpretedOption,
                > as FieldType>::deser_from_iter(
                    &mut self.uninterpreted_option,
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
            bool,
            self::_puroro::tags::Bool,
            0,
        > as FieldType>::ser_to_write(
            &self.message_set_wire_format, &self._bitfield, 1, out
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            1,
        > as FieldType>::ser_to_write(
            &self.no_standard_descriptor_accessor,
            &self._bitfield,
            2,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            2,
        > as FieldType>::ser_to_write(&self.deprecated, &self._bitfield, 3, out)?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            3,
        > as FieldType>::ser_to_write(&self.map_entry, &self._bitfield, 7, out)?;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::UninterpretedOption,
        > as FieldType>::ser_to_write(
            &self.uninterpreted_option, &self._bitfield, 999, out
        )?;

        Ok(())
    }
}

impl ::std::clone::Clone for MessageOptions {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::clone::Clone;
        Self {
            message_set_wire_format: Clone::clone(&self.message_set_wire_format),
            no_standard_descriptor_accessor: Clone::clone(&self.no_standard_descriptor_accessor),
            deprecated: Clone::clone(&self.deprecated),
            map_entry: Clone::clone(&self.map_entry),
            uninterpreted_option: Clone::clone(&self.uninterpreted_option),

            _bitfield: Clone::clone(&self._bitfield),
        }
    }
}

impl ::std::fmt::Debug for MessageOptions {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct("MessageOptions")
            .field("message_set_wire_format", &self.message_set_wire_format())
            .field(
                "no_standard_descriptor_accessor",
                &self.no_standard_descriptor_accessor(),
            )
            .field("deprecated", &self.deprecated())
            .field("map_entry", &self.map_entry())
            .field("uninterpreted_option", &self.uninterpreted_option())
            .finish()
    }
}

impl ::std::cmp::PartialEq for MessageOptions {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;

        true && self.message_set_wire_format_opt() == rhs.message_set_wire_format_opt()
            && self.no_standard_descriptor_accessor_opt()
                == rhs.no_standard_descriptor_accessor_opt()
            && self.deprecated_opt() == rhs.deprecated_opt()
            && self.map_entry_opt() == rhs.map_entry_opt()
            && self.uninterpreted_option() == rhs.uninterpreted_option()
    }
}

impl ::std::ops::Drop for MessageOptions {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
    }
}

#[derive(Default)]
pub struct FieldOptions {
    // Optional, Variant(Enum2(Fqtn(".google.protobuf.FieldOptions.CType")))
    ctype: self::_puroro::internal::field_type::OptionalNumericalField<
        _puroro_root::google::protobuf::field_options::CType,
        self::_puroro::tags::Enum2<_puroro_root::google::protobuf::field_options::CType>,
        0,
    >,
    // Optional, Variant(Bool)
    packed: self::_puroro::internal::field_type::OptionalNumericalField<
        bool,
        self::_puroro::tags::Bool,
        1,
    >,
    // Optional, Variant(Enum2(Fqtn(".google.protobuf.FieldOptions.JSType")))
    jstype: self::_puroro::internal::field_type::OptionalNumericalField<
        _puroro_root::google::protobuf::field_options::JSType,
        self::_puroro::tags::Enum2<_puroro_root::google::protobuf::field_options::JSType>,
        2,
    >,
    // Optional, Variant(Bool)
    lazy: self::_puroro::internal::field_type::OptionalNumericalField<
        bool,
        self::_puroro::tags::Bool,
        3,
    >,
    // Optional, Variant(Bool)
    deprecated: self::_puroro::internal::field_type::OptionalNumericalField<
        bool,
        self::_puroro::tags::Bool,
        4,
    >,
    // Optional, Variant(Bool)
    weak: self::_puroro::internal::field_type::OptionalNumericalField<
        bool,
        self::_puroro::tags::Bool,
        5,
    >,
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.UninterpretedOption")))
    uninterpreted_option: self::_puroro::internal::field_type::RepeatedMessageField<
        _puroro_root::google::protobuf::UninterpretedOption,
    >,

    _bitfield: self::_puroro::bitvec::BitArray<1>,
}

impl FieldOptions {
    // Optional, Variant(Enum2(Fqtn(".google.protobuf.FieldOptions.CType")))
    pub fn ctype(&self) -> _puroro_root::google::protobuf::field_options::CType {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::google::protobuf::field_options::CType,
            self::_puroro::tags::Enum2<_puroro_root::google::protobuf::field_options::CType>,
            0,
        > as NonRepeatedFieldType>::get_field(
            &self.ctype,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn ctype_opt(
        &self,
    ) -> ::std::option::Option<_puroro_root::google::protobuf::field_options::CType> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::google::protobuf::field_options::CType,
            self::_puroro::tags::Enum2<_puroro_root::google::protobuf::field_options::CType>,
            0,
        > as NonRepeatedFieldType>::get_field_opt(&self.ctype, &self._bitfield)
    }
    pub fn has_ctype(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::google::protobuf::field_options::CType,
            self::_puroro::tags::Enum2<_puroro_root::google::protobuf::field_options::CType>,
            0,
        > as NonRepeatedFieldType>::get_field_opt(&self.ctype, &self._bitfield)
        .is_some()
    }
    pub fn ctype_mut(&mut self) -> &mut _puroro_root::google::protobuf::field_options::CType {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::google::protobuf::field_options::CType,
            self::_puroro::tags::Enum2<_puroro_root::google::protobuf::field_options::CType>,
            0,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.ctype,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_ctype(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::google::protobuf::field_options::CType,
            self::_puroro::tags::Enum2<_puroro_root::google::protobuf::field_options::CType>,
            0,
        > as NonRepeatedFieldType>::clear(&mut self.ctype, &mut self._bitfield)
    }
    // Optional, Variant(Bool)
    pub fn packed(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            1,
        > as NonRepeatedFieldType>::get_field(
            &self.packed,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn packed_opt(&self) -> ::std::option::Option<bool> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            1,
        > as NonRepeatedFieldType>::get_field_opt(&self.packed, &self._bitfield)
    }
    pub fn has_packed(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            1,
        > as NonRepeatedFieldType>::get_field_opt(&self.packed, &self._bitfield)
        .is_some()
    }
    pub fn packed_mut(&mut self) -> &mut bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            1,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.packed,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_packed(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            1,
        > as NonRepeatedFieldType>::clear(&mut self.packed, &mut self._bitfield)
    }
    // Optional, Variant(Enum2(Fqtn(".google.protobuf.FieldOptions.JSType")))
    pub fn jstype(&self) -> _puroro_root::google::protobuf::field_options::JSType {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::google::protobuf::field_options::JSType,
            self::_puroro::tags::Enum2<_puroro_root::google::protobuf::field_options::JSType>,
            2,
        > as NonRepeatedFieldType>::get_field(
            &self.jstype,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn jstype_opt(
        &self,
    ) -> ::std::option::Option<_puroro_root::google::protobuf::field_options::JSType> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::google::protobuf::field_options::JSType,
            self::_puroro::tags::Enum2<_puroro_root::google::protobuf::field_options::JSType>,
            2,
        > as NonRepeatedFieldType>::get_field_opt(&self.jstype, &self._bitfield)
    }
    pub fn has_jstype(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::google::protobuf::field_options::JSType,
            self::_puroro::tags::Enum2<_puroro_root::google::protobuf::field_options::JSType>,
            2,
        > as NonRepeatedFieldType>::get_field_opt(&self.jstype, &self._bitfield)
        .is_some()
    }
    pub fn jstype_mut(&mut self) -> &mut _puroro_root::google::protobuf::field_options::JSType {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::google::protobuf::field_options::JSType,
            self::_puroro::tags::Enum2<_puroro_root::google::protobuf::field_options::JSType>,
            2,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.jstype,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_jstype(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::google::protobuf::field_options::JSType,
            self::_puroro::tags::Enum2<_puroro_root::google::protobuf::field_options::JSType>,
            2,
        > as NonRepeatedFieldType>::clear(&mut self.jstype, &mut self._bitfield)
    }
    // Optional, Variant(Bool)
    pub fn lazy(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            3,
        > as NonRepeatedFieldType>::get_field(
            &self.lazy,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn lazy_opt(&self) -> ::std::option::Option<bool> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            3,
        > as NonRepeatedFieldType>::get_field_opt(&self.lazy, &self._bitfield)
    }
    pub fn has_lazy(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            3,
        > as NonRepeatedFieldType>::get_field_opt(&self.lazy, &self._bitfield)
        .is_some()
    }
    pub fn lazy_mut(&mut self) -> &mut bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            3,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.lazy,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_lazy(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            3,
        > as NonRepeatedFieldType>::clear(&mut self.lazy, &mut self._bitfield)
    }
    // Optional, Variant(Bool)
    pub fn deprecated(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            4,
        > as NonRepeatedFieldType>::get_field(
            &self.deprecated,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn deprecated_opt(&self) -> ::std::option::Option<bool> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            4,
        > as NonRepeatedFieldType>::get_field_opt(&self.deprecated, &self._bitfield)
    }
    pub fn has_deprecated(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            4,
        > as NonRepeatedFieldType>::get_field_opt(&self.deprecated, &self._bitfield)
        .is_some()
    }
    pub fn deprecated_mut(&mut self) -> &mut bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            4,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.deprecated,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_deprecated(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            4,
        > as NonRepeatedFieldType>::clear(&mut self.deprecated, &mut self._bitfield)
    }
    // Optional, Variant(Bool)
    pub fn weak(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            5,
        > as NonRepeatedFieldType>::get_field(
            &self.weak,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn weak_opt(&self) -> ::std::option::Option<bool> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            5,
        > as NonRepeatedFieldType>::get_field_opt(&self.weak, &self._bitfield)
    }
    pub fn has_weak(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            5,
        > as NonRepeatedFieldType>::get_field_opt(&self.weak, &self._bitfield)
        .is_some()
    }
    pub fn weak_mut(&mut self) -> &mut bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            5,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.weak,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_weak(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            5,
        > as NonRepeatedFieldType>::clear(&mut self.weak, &mut self._bitfield)
    }
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.UninterpretedOption")))
    pub fn uninterpreted_option(&self) -> &[_puroro_root::google::protobuf::UninterpretedOption] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::get_field(&self.uninterpreted_option, &self._bitfield)
    }
    pub fn uninterpreted_option_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec<_puroro_root::google::protobuf::UninterpretedOption> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::mut_field(
            &mut self.uninterpreted_option, &mut self._bitfield
        )
    }
    pub fn clear_uninterpreted_option(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::clear(&mut self.uninterpreted_option, &mut self._bitfield)
    }
}

impl self::_puroro::Message for FieldOptions {
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
                1 => <self::_puroro::internal::field_type::OptionalNumericalField<
                    _puroro_root::google::protobuf::field_options::CType,
                    self::_puroro::tags::Enum2<
                        _puroro_root::google::protobuf::field_options::CType,
                    >,
                    0,
                > as FieldType>::deser_from_iter(
                    &mut self.ctype, &mut self._bitfield, field_data
                )?,
                2 => <self::_puroro::internal::field_type::OptionalNumericalField<
                    bool,
                    self::_puroro::tags::Bool,
                    1,
                > as FieldType>::deser_from_iter(
                    &mut self.packed,
                    &mut self._bitfield,
                    field_data,
                )?,
                6 => <self::_puroro::internal::field_type::OptionalNumericalField<
                    _puroro_root::google::protobuf::field_options::JSType,
                    self::_puroro::tags::Enum2<
                        _puroro_root::google::protobuf::field_options::JSType,
                    >,
                    2,
                > as FieldType>::deser_from_iter(
                    &mut self.jstype,
                    &mut self._bitfield,
                    field_data,
                )?,
                5 => <self::_puroro::internal::field_type::OptionalNumericalField<
                    bool,
                    self::_puroro::tags::Bool,
                    3,
                > as FieldType>::deser_from_iter(
                    &mut self.lazy, &mut self._bitfield, field_data
                )?,
                3 => <self::_puroro::internal::field_type::OptionalNumericalField<
                    bool,
                    self::_puroro::tags::Bool,
                    4,
                > as FieldType>::deser_from_iter(
                    &mut self.deprecated,
                    &mut self._bitfield,
                    field_data,
                )?,
                10 => <self::_puroro::internal::field_type::OptionalNumericalField<
                    bool,
                    self::_puroro::tags::Bool,
                    5,
                > as FieldType>::deser_from_iter(
                    &mut self.weak, &mut self._bitfield, field_data
                )?,
                999 => <self::_puroro::internal::field_type::RepeatedMessageField<
                    _puroro_root::google::protobuf::UninterpretedOption,
                > as FieldType>::deser_from_iter(
                    &mut self.uninterpreted_option,
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
            _puroro_root::google::protobuf::field_options::CType,
            self::_puroro::tags::Enum2<_puroro_root::google::protobuf::field_options::CType>,
            0,
        > as FieldType>::ser_to_write(&self.ctype, &self._bitfield, 1, out)?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            1,
        > as FieldType>::ser_to_write(&self.packed, &self._bitfield, 2, out)?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::google::protobuf::field_options::JSType,
            self::_puroro::tags::Enum2<_puroro_root::google::protobuf::field_options::JSType>,
            2,
        > as FieldType>::ser_to_write(&self.jstype, &self._bitfield, 6, out)?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            3,
        > as FieldType>::ser_to_write(&self.lazy, &self._bitfield, 5, out)?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            4,
        > as FieldType>::ser_to_write(&self.deprecated, &self._bitfield, 3, out)?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            5,
        > as FieldType>::ser_to_write(&self.weak, &self._bitfield, 10, out)?;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::UninterpretedOption,
        > as FieldType>::ser_to_write(
            &self.uninterpreted_option, &self._bitfield, 999, out
        )?;

        Ok(())
    }
}

impl ::std::clone::Clone for FieldOptions {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::clone::Clone;
        Self {
            ctype: Clone::clone(&self.ctype),
            packed: Clone::clone(&self.packed),
            jstype: Clone::clone(&self.jstype),
            lazy: Clone::clone(&self.lazy),
            deprecated: Clone::clone(&self.deprecated),
            weak: Clone::clone(&self.weak),
            uninterpreted_option: Clone::clone(&self.uninterpreted_option),

            _bitfield: Clone::clone(&self._bitfield),
        }
    }
}

impl ::std::fmt::Debug for FieldOptions {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct("FieldOptions")
            .field("ctype", &self.ctype())
            .field("packed", &self.packed())
            .field("jstype", &self.jstype())
            .field("lazy", &self.lazy())
            .field("deprecated", &self.deprecated())
            .field("weak", &self.weak())
            .field("uninterpreted_option", &self.uninterpreted_option())
            .finish()
    }
}

impl ::std::cmp::PartialEq for FieldOptions {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;

        true && self.ctype_opt() == rhs.ctype_opt()
            && self.packed_opt() == rhs.packed_opt()
            && self.jstype_opt() == rhs.jstype_opt()
            && self.lazy_opt() == rhs.lazy_opt()
            && self.deprecated_opt() == rhs.deprecated_opt()
            && self.weak_opt() == rhs.weak_opt()
            && self.uninterpreted_option() == rhs.uninterpreted_option()
    }
}

impl ::std::ops::Drop for FieldOptions {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
    }
}

#[derive(Default)]
pub struct OneofOptions {
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.UninterpretedOption")))
    uninterpreted_option: self::_puroro::internal::field_type::RepeatedMessageField<
        _puroro_root::google::protobuf::UninterpretedOption,
    >,

    _bitfield: self::_puroro::bitvec::BitArray<0>,
}

impl OneofOptions {
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.UninterpretedOption")))
    pub fn uninterpreted_option(&self) -> &[_puroro_root::google::protobuf::UninterpretedOption] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::get_field(&self.uninterpreted_option, &self._bitfield)
    }
    pub fn uninterpreted_option_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec<_puroro_root::google::protobuf::UninterpretedOption> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::mut_field(
            &mut self.uninterpreted_option, &mut self._bitfield
        )
    }
    pub fn clear_uninterpreted_option(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::clear(&mut self.uninterpreted_option, &mut self._bitfield)
    }
}

impl self::_puroro::Message for OneofOptions {
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
                999 => <self::_puroro::internal::field_type::RepeatedMessageField<
                    _puroro_root::google::protobuf::UninterpretedOption,
                > as FieldType>::deser_from_iter(
                    &mut self.uninterpreted_option,
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
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::UninterpretedOption,
        > as FieldType>::ser_to_write(
            &self.uninterpreted_option, &self._bitfield, 999, out
        )?;

        Ok(())
    }
}

impl ::std::clone::Clone for OneofOptions {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::clone::Clone;
        Self {
            uninterpreted_option: Clone::clone(&self.uninterpreted_option),

            _bitfield: Clone::clone(&self._bitfield),
        }
    }
}

impl ::std::fmt::Debug for OneofOptions {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct("OneofOptions")
            .field("uninterpreted_option", &self.uninterpreted_option())
            .finish()
    }
}

impl ::std::cmp::PartialEq for OneofOptions {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;

        true && self.uninterpreted_option() == rhs.uninterpreted_option()
    }
}

impl ::std::ops::Drop for OneofOptions {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
    }
}

#[derive(Default)]
pub struct EnumOptions {
    // Optional, Variant(Bool)
    allow_alias: self::_puroro::internal::field_type::OptionalNumericalField<
        bool,
        self::_puroro::tags::Bool,
        0,
    >,
    // Optional, Variant(Bool)
    deprecated: self::_puroro::internal::field_type::OptionalNumericalField<
        bool,
        self::_puroro::tags::Bool,
        1,
    >,
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.UninterpretedOption")))
    uninterpreted_option: self::_puroro::internal::field_type::RepeatedMessageField<
        _puroro_root::google::protobuf::UninterpretedOption,
    >,

    _bitfield: self::_puroro::bitvec::BitArray<1>,
}

impl EnumOptions {
    // Optional, Variant(Bool)
    pub fn allow_alias(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            0,
        > as NonRepeatedFieldType>::get_field(
            &self.allow_alias,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn allow_alias_opt(&self) -> ::std::option::Option<bool> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            0,
        > as NonRepeatedFieldType>::get_field_opt(&self.allow_alias, &self._bitfield)
    }
    pub fn has_allow_alias(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            0,
        > as NonRepeatedFieldType>::get_field_opt(&self.allow_alias, &self._bitfield)
        .is_some()
    }
    pub fn allow_alias_mut(&mut self) -> &mut bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            0,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.allow_alias,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_allow_alias(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            0,
        > as NonRepeatedFieldType>::clear(&mut self.allow_alias, &mut self._bitfield)
    }
    // Optional, Variant(Bool)
    pub fn deprecated(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            1,
        > as NonRepeatedFieldType>::get_field(
            &self.deprecated,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn deprecated_opt(&self) -> ::std::option::Option<bool> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            1,
        > as NonRepeatedFieldType>::get_field_opt(&self.deprecated, &self._bitfield)
    }
    pub fn has_deprecated(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            1,
        > as NonRepeatedFieldType>::get_field_opt(&self.deprecated, &self._bitfield)
        .is_some()
    }
    pub fn deprecated_mut(&mut self) -> &mut bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            1,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.deprecated,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_deprecated(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            1,
        > as NonRepeatedFieldType>::clear(&mut self.deprecated, &mut self._bitfield)
    }
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.UninterpretedOption")))
    pub fn uninterpreted_option(&self) -> &[_puroro_root::google::protobuf::UninterpretedOption] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::get_field(&self.uninterpreted_option, &self._bitfield)
    }
    pub fn uninterpreted_option_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec<_puroro_root::google::protobuf::UninterpretedOption> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::mut_field(
            &mut self.uninterpreted_option, &mut self._bitfield
        )
    }
    pub fn clear_uninterpreted_option(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::clear(&mut self.uninterpreted_option, &mut self._bitfield)
    }
}

impl self::_puroro::Message for EnumOptions {
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
                2 => <self::_puroro::internal::field_type::OptionalNumericalField<
                    bool,
                    self::_puroro::tags::Bool,
                    0,
                > as FieldType>::deser_from_iter(
                    &mut self.allow_alias,
                    &mut self._bitfield,
                    field_data,
                )?,
                3 => <self::_puroro::internal::field_type::OptionalNumericalField<
                    bool,
                    self::_puroro::tags::Bool,
                    1,
                > as FieldType>::deser_from_iter(
                    &mut self.deprecated,
                    &mut self._bitfield,
                    field_data,
                )?,
                999 => <self::_puroro::internal::field_type::RepeatedMessageField<
                    _puroro_root::google::protobuf::UninterpretedOption,
                > as FieldType>::deser_from_iter(
                    &mut self.uninterpreted_option,
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
            bool,
            self::_puroro::tags::Bool,
            0,
        > as FieldType>::ser_to_write(&self.allow_alias, &self._bitfield, 2, out)?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            1,
        > as FieldType>::ser_to_write(&self.deprecated, &self._bitfield, 3, out)?;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::UninterpretedOption,
        > as FieldType>::ser_to_write(
            &self.uninterpreted_option, &self._bitfield, 999, out
        )?;

        Ok(())
    }
}

impl ::std::clone::Clone for EnumOptions {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::clone::Clone;
        Self {
            allow_alias: Clone::clone(&self.allow_alias),
            deprecated: Clone::clone(&self.deprecated),
            uninterpreted_option: Clone::clone(&self.uninterpreted_option),

            _bitfield: Clone::clone(&self._bitfield),
        }
    }
}

impl ::std::fmt::Debug for EnumOptions {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct("EnumOptions")
            .field("allow_alias", &self.allow_alias())
            .field("deprecated", &self.deprecated())
            .field("uninterpreted_option", &self.uninterpreted_option())
            .finish()
    }
}

impl ::std::cmp::PartialEq for EnumOptions {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;

        true && self.allow_alias_opt() == rhs.allow_alias_opt()
            && self.deprecated_opt() == rhs.deprecated_opt()
            && self.uninterpreted_option() == rhs.uninterpreted_option()
    }
}

impl ::std::ops::Drop for EnumOptions {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
    }
}

#[derive(Default)]
pub struct EnumValueOptions {
    // Optional, Variant(Bool)
    deprecated: self::_puroro::internal::field_type::OptionalNumericalField<
        bool,
        self::_puroro::tags::Bool,
        0,
    >,
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.UninterpretedOption")))
    uninterpreted_option: self::_puroro::internal::field_type::RepeatedMessageField<
        _puroro_root::google::protobuf::UninterpretedOption,
    >,

    _bitfield: self::_puroro::bitvec::BitArray<1>,
}

impl EnumValueOptions {
    // Optional, Variant(Bool)
    pub fn deprecated(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            0,
        > as NonRepeatedFieldType>::get_field(
            &self.deprecated,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn deprecated_opt(&self) -> ::std::option::Option<bool> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            0,
        > as NonRepeatedFieldType>::get_field_opt(&self.deprecated, &self._bitfield)
    }
    pub fn has_deprecated(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            0,
        > as NonRepeatedFieldType>::get_field_opt(&self.deprecated, &self._bitfield)
        .is_some()
    }
    pub fn deprecated_mut(&mut self) -> &mut bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            0,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.deprecated,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_deprecated(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            0,
        > as NonRepeatedFieldType>::clear(&mut self.deprecated, &mut self._bitfield)
    }
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.UninterpretedOption")))
    pub fn uninterpreted_option(&self) -> &[_puroro_root::google::protobuf::UninterpretedOption] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::get_field(&self.uninterpreted_option, &self._bitfield)
    }
    pub fn uninterpreted_option_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec<_puroro_root::google::protobuf::UninterpretedOption> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::mut_field(
            &mut self.uninterpreted_option, &mut self._bitfield
        )
    }
    pub fn clear_uninterpreted_option(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::clear(&mut self.uninterpreted_option, &mut self._bitfield)
    }
}

impl self::_puroro::Message for EnumValueOptions {
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
                1 => <self::_puroro::internal::field_type::OptionalNumericalField<
                    bool,
                    self::_puroro::tags::Bool,
                    0,
                > as FieldType>::deser_from_iter(
                    &mut self.deprecated,
                    &mut self._bitfield,
                    field_data,
                )?,
                999 => <self::_puroro::internal::field_type::RepeatedMessageField<
                    _puroro_root::google::protobuf::UninterpretedOption,
                > as FieldType>::deser_from_iter(
                    &mut self.uninterpreted_option,
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
            bool,
            self::_puroro::tags::Bool,
            0,
        > as FieldType>::ser_to_write(&self.deprecated, &self._bitfield, 1, out)?;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::UninterpretedOption,
        > as FieldType>::ser_to_write(
            &self.uninterpreted_option, &self._bitfield, 999, out
        )?;

        Ok(())
    }
}

impl ::std::clone::Clone for EnumValueOptions {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::clone::Clone;
        Self {
            deprecated: Clone::clone(&self.deprecated),
            uninterpreted_option: Clone::clone(&self.uninterpreted_option),

            _bitfield: Clone::clone(&self._bitfield),
        }
    }
}

impl ::std::fmt::Debug for EnumValueOptions {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct("EnumValueOptions")
            .field("deprecated", &self.deprecated())
            .field("uninterpreted_option", &self.uninterpreted_option())
            .finish()
    }
}

impl ::std::cmp::PartialEq for EnumValueOptions {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;

        true && self.deprecated_opt() == rhs.deprecated_opt()
            && self.uninterpreted_option() == rhs.uninterpreted_option()
    }
}

impl ::std::ops::Drop for EnumValueOptions {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
    }
}

#[derive(Default)]
pub struct ServiceOptions {
    // Optional, Variant(Bool)
    deprecated: self::_puroro::internal::field_type::OptionalNumericalField<
        bool,
        self::_puroro::tags::Bool,
        0,
    >,
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.UninterpretedOption")))
    uninterpreted_option: self::_puroro::internal::field_type::RepeatedMessageField<
        _puroro_root::google::protobuf::UninterpretedOption,
    >,

    _bitfield: self::_puroro::bitvec::BitArray<1>,
}

impl ServiceOptions {
    // Optional, Variant(Bool)
    pub fn deprecated(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            0,
        > as NonRepeatedFieldType>::get_field(
            &self.deprecated,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn deprecated_opt(&self) -> ::std::option::Option<bool> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            0,
        > as NonRepeatedFieldType>::get_field_opt(&self.deprecated, &self._bitfield)
    }
    pub fn has_deprecated(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            0,
        > as NonRepeatedFieldType>::get_field_opt(&self.deprecated, &self._bitfield)
        .is_some()
    }
    pub fn deprecated_mut(&mut self) -> &mut bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            0,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.deprecated,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_deprecated(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            0,
        > as NonRepeatedFieldType>::clear(&mut self.deprecated, &mut self._bitfield)
    }
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.UninterpretedOption")))
    pub fn uninterpreted_option(&self) -> &[_puroro_root::google::protobuf::UninterpretedOption] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::get_field(&self.uninterpreted_option, &self._bitfield)
    }
    pub fn uninterpreted_option_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec<_puroro_root::google::protobuf::UninterpretedOption> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::mut_field(
            &mut self.uninterpreted_option, &mut self._bitfield
        )
    }
    pub fn clear_uninterpreted_option(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::clear(&mut self.uninterpreted_option, &mut self._bitfield)
    }
}

impl self::_puroro::Message for ServiceOptions {
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
                33 => <self::_puroro::internal::field_type::OptionalNumericalField<
                    bool,
                    self::_puroro::tags::Bool,
                    0,
                > as FieldType>::deser_from_iter(
                    &mut self.deprecated,
                    &mut self._bitfield,
                    field_data,
                )?,
                999 => <self::_puroro::internal::field_type::RepeatedMessageField<
                    _puroro_root::google::protobuf::UninterpretedOption,
                > as FieldType>::deser_from_iter(
                    &mut self.uninterpreted_option,
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
            bool,
            self::_puroro::tags::Bool,
            0,
        > as FieldType>::ser_to_write(&self.deprecated, &self._bitfield, 33, out)?;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::UninterpretedOption,
        > as FieldType>::ser_to_write(
            &self.uninterpreted_option, &self._bitfield, 999, out
        )?;

        Ok(())
    }
}

impl ::std::clone::Clone for ServiceOptions {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::clone::Clone;
        Self {
            deprecated: Clone::clone(&self.deprecated),
            uninterpreted_option: Clone::clone(&self.uninterpreted_option),

            _bitfield: Clone::clone(&self._bitfield),
        }
    }
}

impl ::std::fmt::Debug for ServiceOptions {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct("ServiceOptions")
            .field("deprecated", &self.deprecated())
            .field("uninterpreted_option", &self.uninterpreted_option())
            .finish()
    }
}

impl ::std::cmp::PartialEq for ServiceOptions {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;

        true && self.deprecated_opt() == rhs.deprecated_opt()
            && self.uninterpreted_option() == rhs.uninterpreted_option()
    }
}

impl ::std::ops::Drop for ServiceOptions {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
    }
}

#[derive(Default)]
pub struct MethodOptions {
    // Optional, Variant(Bool)
    deprecated: self::_puroro::internal::field_type::OptionalNumericalField<
        bool,
        self::_puroro::tags::Bool,
        0,
    >,
    // Optional, Variant(Enum2(Fqtn(".google.protobuf.MethodOptions.IdempotencyLevel")))
    idempotency_level: self::_puroro::internal::field_type::OptionalNumericalField<
        _puroro_root::google::protobuf::method_options::IdempotencyLevel,
        self::_puroro::tags::Enum2<
            _puroro_root::google::protobuf::method_options::IdempotencyLevel,
        >,
        1,
    >,
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.UninterpretedOption")))
    uninterpreted_option: self::_puroro::internal::field_type::RepeatedMessageField<
        _puroro_root::google::protobuf::UninterpretedOption,
    >,

    _bitfield: self::_puroro::bitvec::BitArray<1>,
}

impl MethodOptions {
    // Optional, Variant(Bool)
    pub fn deprecated(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            0,
        > as NonRepeatedFieldType>::get_field(
            &self.deprecated,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn deprecated_opt(&self) -> ::std::option::Option<bool> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            0,
        > as NonRepeatedFieldType>::get_field_opt(&self.deprecated, &self._bitfield)
    }
    pub fn has_deprecated(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            0,
        > as NonRepeatedFieldType>::get_field_opt(&self.deprecated, &self._bitfield)
        .is_some()
    }
    pub fn deprecated_mut(&mut self) -> &mut bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            0,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.deprecated,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_deprecated(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            bool,
            self::_puroro::tags::Bool,
            0,
        > as NonRepeatedFieldType>::clear(&mut self.deprecated, &mut self._bitfield)
    }
    // Optional, Variant(Enum2(Fqtn(".google.protobuf.MethodOptions.IdempotencyLevel")))
    pub fn idempotency_level(
        &self,
    ) -> _puroro_root::google::protobuf::method_options::IdempotencyLevel {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::google::protobuf::method_options::IdempotencyLevel,
            self::_puroro::tags::Enum2<
                _puroro_root::google::protobuf::method_options::IdempotencyLevel,
            >,
            1,
        > as NonRepeatedFieldType>::get_field(
            &self.idempotency_level,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn idempotency_level_opt(
        &self,
    ) -> ::std::option::Option<_puroro_root::google::protobuf::method_options::IdempotencyLevel>
    {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::google::protobuf::method_options::IdempotencyLevel,
            self::_puroro::tags::Enum2<
                _puroro_root::google::protobuf::method_options::IdempotencyLevel,
            >,
            1,
        > as NonRepeatedFieldType>::get_field_opt(&self.idempotency_level, &self._bitfield)
    }
    pub fn has_idempotency_level(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::google::protobuf::method_options::IdempotencyLevel,
            self::_puroro::tags::Enum2<
                _puroro_root::google::protobuf::method_options::IdempotencyLevel,
            >,
            1,
        > as NonRepeatedFieldType>::get_field_opt(&self.idempotency_level, &self._bitfield)
        .is_some()
    }
    pub fn idempotency_level_mut(
        &mut self,
    ) -> &mut _puroro_root::google::protobuf::method_options::IdempotencyLevel {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::google::protobuf::method_options::IdempotencyLevel,
            self::_puroro::tags::Enum2<
                _puroro_root::google::protobuf::method_options::IdempotencyLevel,
            >,
            1,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.idempotency_level,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_idempotency_level(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::google::protobuf::method_options::IdempotencyLevel,
            self::_puroro::tags::Enum2<
                _puroro_root::google::protobuf::method_options::IdempotencyLevel,
            >,
            1,
        > as NonRepeatedFieldType>::clear(&mut self.idempotency_level, &mut self._bitfield)
    }
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.UninterpretedOption")))
    pub fn uninterpreted_option(&self) -> &[_puroro_root::google::protobuf::UninterpretedOption] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::get_field(&self.uninterpreted_option, &self._bitfield)
    }
    pub fn uninterpreted_option_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec<_puroro_root::google::protobuf::UninterpretedOption> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::mut_field(
            &mut self.uninterpreted_option, &mut self._bitfield
        )
    }
    pub fn clear_uninterpreted_option(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::clear(&mut self.uninterpreted_option, &mut self._bitfield)
    }
}

impl self::_puroro::Message for MethodOptions {
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
                33 => <self::_puroro::internal::field_type::OptionalNumericalField<
                    bool,
                    self::_puroro::tags::Bool,
                    0,
                > as FieldType>::deser_from_iter(
                    &mut self.deprecated,
                    &mut self._bitfield,
                    field_data,
                )?,
                34 => <self::_puroro::internal::field_type::OptionalNumericalField<
                    _puroro_root::google::protobuf::method_options::IdempotencyLevel,
                    self::_puroro::tags::Enum2<
                        _puroro_root::google::protobuf::method_options::IdempotencyLevel,
                    >,
                    1,
                > as FieldType>::deser_from_iter(
                    &mut self.idempotency_level,
                    &mut self._bitfield,
                    field_data,
                )?,
                999 => <self::_puroro::internal::field_type::RepeatedMessageField<
                    _puroro_root::google::protobuf::UninterpretedOption,
                > as FieldType>::deser_from_iter(
                    &mut self.uninterpreted_option,
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
            bool,
            self::_puroro::tags::Bool,
            0,
        > as FieldType>::ser_to_write(&self.deprecated, &self._bitfield, 33, out)?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::google::protobuf::method_options::IdempotencyLevel,
            self::_puroro::tags::Enum2<
                _puroro_root::google::protobuf::method_options::IdempotencyLevel,
            >,
            1,
        > as FieldType>::ser_to_write(&self.idempotency_level, &self._bitfield, 34, out)?;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::UninterpretedOption,
        > as FieldType>::ser_to_write(
            &self.uninterpreted_option, &self._bitfield, 999, out
        )?;

        Ok(())
    }
}

impl ::std::clone::Clone for MethodOptions {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::clone::Clone;
        Self {
            deprecated: Clone::clone(&self.deprecated),
            idempotency_level: Clone::clone(&self.idempotency_level),
            uninterpreted_option: Clone::clone(&self.uninterpreted_option),

            _bitfield: Clone::clone(&self._bitfield),
        }
    }
}

impl ::std::fmt::Debug for MethodOptions {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct("MethodOptions")
            .field("deprecated", &self.deprecated())
            .field("idempotency_level", &self.idempotency_level())
            .field("uninterpreted_option", &self.uninterpreted_option())
            .finish()
    }
}

impl ::std::cmp::PartialEq for MethodOptions {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;

        true && self.deprecated_opt() == rhs.deprecated_opt()
            && self.idempotency_level_opt() == rhs.idempotency_level_opt()
            && self.uninterpreted_option() == rhs.uninterpreted_option()
    }
}

impl ::std::ops::Drop for MethodOptions {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
    }
}

#[derive(Default)]
pub struct UninterpretedOption {
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.UninterpretedOption.NamePart")))
    name: self::_puroro::internal::field_type::RepeatedMessageField<
        _puroro_root::google::protobuf::uninterpreted_option::NamePart,
    >,
    // Optional, LengthDelimited(String)
    identifier_value: self::_puroro::internal::field_type::OptionalStringField<0>,
    // Optional, Variant(UInt64)
    positive_int_value: self::_puroro::internal::field_type::OptionalNumericalField<
        u64,
        self::_puroro::tags::UInt64,
        1,
    >,
    // Optional, Variant(Int64)
    negative_int_value: self::_puroro::internal::field_type::OptionalNumericalField<
        i64,
        self::_puroro::tags::Int64,
        2,
    >,
    // Optional, Bits64(Double)
    double_value: self::_puroro::internal::field_type::OptionalNumericalField<
        f64,
        self::_puroro::tags::Double,
        3,
    >,
    // Optional, LengthDelimited(Bytes)
    string_value: self::_puroro::internal::field_type::OptionalBytesField<4>,
    // Optional, LengthDelimited(String)
    aggregate_value: self::_puroro::internal::field_type::OptionalStringField<5>,

    _bitfield: self::_puroro::bitvec::BitArray<1>,
}

impl UninterpretedOption {
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.UninterpretedOption.NamePart")))
    pub fn name(&self) -> &[_puroro_root::google::protobuf::uninterpreted_option::NamePart] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::uninterpreted_option::NamePart,
        > as RepeatedFieldType>::get_field(&self.name, &self._bitfield)
    }
    pub fn name_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec<_puroro_root::google::protobuf::uninterpreted_option::NamePart> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::uninterpreted_option::NamePart,
        > as RepeatedFieldType>::mut_field(&mut self.name, &mut self._bitfield)
    }
    pub fn clear_name(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::uninterpreted_option::NamePart,
        > as RepeatedFieldType>::clear(&mut self.name, &mut self._bitfield)
    }
    // Optional, LengthDelimited(String)
    pub fn identifier_value(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field(
            &self.identifier_value, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn identifier_value_opt(&self) -> ::std::option::Option<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field_opt(
            &self.identifier_value, &self._bitfield,
        )
    }
    pub fn has_identifier_value(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field_opt(
            &self.identifier_value, &self._bitfield,
        ).is_some()
    }
    pub fn identifier_value_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::mut_field(
            &mut self.identifier_value, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_identifier_value(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField<0> as NonRepeatedFieldType>::clear(
            &mut self.identifier_value,
            &mut self._bitfield,
        )
    }
    // Optional, Variant(UInt64)
    pub fn positive_int_value(&self) -> u64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u64,
            self::_puroro::tags::UInt64,
            1,
        > as NonRepeatedFieldType>::get_field(
            &self.positive_int_value,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn positive_int_value_opt(&self) -> ::std::option::Option<u64> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u64,
            self::_puroro::tags::UInt64,
            1,
        > as NonRepeatedFieldType>::get_field_opt(&self.positive_int_value, &self._bitfield)
    }
    pub fn has_positive_int_value(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u64,
            self::_puroro::tags::UInt64,
            1,
        > as NonRepeatedFieldType>::get_field_opt(&self.positive_int_value, &self._bitfield)
        .is_some()
    }
    pub fn positive_int_value_mut(&mut self) -> &mut u64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u64,
            self::_puroro::tags::UInt64,
            1,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.positive_int_value,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_positive_int_value(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u64,
            self::_puroro::tags::UInt64,
            1,
        > as NonRepeatedFieldType>::clear(&mut self.positive_int_value, &mut self._bitfield)
    }
    // Optional, Variant(Int64)
    pub fn negative_int_value(&self) -> i64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::Int64,
            2,
        > as NonRepeatedFieldType>::get_field(
            &self.negative_int_value,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn negative_int_value_opt(&self) -> ::std::option::Option<i64> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::Int64,
            2,
        > as NonRepeatedFieldType>::get_field_opt(&self.negative_int_value, &self._bitfield)
    }
    pub fn has_negative_int_value(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::Int64,
            2,
        > as NonRepeatedFieldType>::get_field_opt(&self.negative_int_value, &self._bitfield)
        .is_some()
    }
    pub fn negative_int_value_mut(&mut self) -> &mut i64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::Int64,
            2,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.negative_int_value,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_negative_int_value(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::Int64,
            2,
        > as NonRepeatedFieldType>::clear(&mut self.negative_int_value, &mut self._bitfield)
    }
    // Optional, Bits64(Double)
    pub fn double_value(&self) -> f64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            f64,
            self::_puroro::tags::Double,
            3,
        > as NonRepeatedFieldType>::get_field(
            &self.double_value,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn double_value_opt(&self) -> ::std::option::Option<f64> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            f64,
            self::_puroro::tags::Double,
            3,
        > as NonRepeatedFieldType>::get_field_opt(&self.double_value, &self._bitfield)
    }
    pub fn has_double_value(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            f64,
            self::_puroro::tags::Double,
            3,
        > as NonRepeatedFieldType>::get_field_opt(&self.double_value, &self._bitfield)
        .is_some()
    }
    pub fn double_value_mut(&mut self) -> &mut f64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            f64,
            self::_puroro::tags::Double,
            3,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.double_value,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_double_value(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            f64,
            self::_puroro::tags::Double,
            3,
        > as NonRepeatedFieldType>::clear(&mut self.double_value, &mut self._bitfield)
    }
    // Optional, LengthDelimited(Bytes)
    pub fn string_value(&self) -> &[u8] {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalBytesField<4> as NonRepeatedFieldType>::get_field(
            &self.string_value, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn string_value_opt(&self) -> ::std::option::Option<&[u8]> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalBytesField<4> as NonRepeatedFieldType>::get_field_opt(
            &self.string_value, &self._bitfield,
        )
    }
    pub fn has_string_value(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalBytesField<4> as NonRepeatedFieldType>::get_field_opt(
            &self.string_value, &self._bitfield,
        ).is_some()
    }
    pub fn string_value_mut(&mut self) -> &mut ::std::vec::Vec<u8> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalBytesField<4> as NonRepeatedFieldType>::mut_field(
            &mut self.string_value, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_string_value(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalBytesField<4> as NonRepeatedFieldType>::clear(
            &mut self.string_value,
            &mut self._bitfield,
        )
    }
    // Optional, LengthDelimited(String)
    pub fn aggregate_value(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<5> as NonRepeatedFieldType>::get_field(
            &self.aggregate_value, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn aggregate_value_opt(&self) -> ::std::option::Option<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<5> as NonRepeatedFieldType>::get_field_opt(
            &self.aggregate_value, &self._bitfield,
        )
    }
    pub fn has_aggregate_value(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<5> as NonRepeatedFieldType>::get_field_opt(
            &self.aggregate_value, &self._bitfield,
        ).is_some()
    }
    pub fn aggregate_value_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<5> as NonRepeatedFieldType>::mut_field(
            &mut self.aggregate_value, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_aggregate_value(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField<5> as NonRepeatedFieldType>::clear(
            &mut self.aggregate_value,
            &mut self._bitfield,
        )
    }
}

impl self::_puroro::Message for UninterpretedOption {
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
                2 => <self::_puroro::internal::field_type::RepeatedMessageField::<_puroro_root::google::protobuf::uninterpreted_option::NamePart> as FieldType>::deser_from_iter(
                    &mut self.name,
                    &mut self._bitfield,
                    field_data,
                )?,
                3 => <self::_puroro::internal::field_type::OptionalStringField::<0> as FieldType>::deser_from_iter(
                    &mut self.identifier_value,
                    &mut self._bitfield,
                    field_data,
                )?,
                4 => <self::_puroro::internal::field_type::OptionalNumericalField::<u64, self::_puroro::tags::UInt64, 1> as FieldType>::deser_from_iter(
                    &mut self.positive_int_value,
                    &mut self._bitfield,
                    field_data,
                )?,
                5 => <self::_puroro::internal::field_type::OptionalNumericalField::<i64, self::_puroro::tags::Int64, 2> as FieldType>::deser_from_iter(
                    &mut self.negative_int_value,
                    &mut self._bitfield,
                    field_data,
                )?,
                6 => <self::_puroro::internal::field_type::OptionalNumericalField::<f64, self::_puroro::tags::Double, 3> as FieldType>::deser_from_iter(
                    &mut self.double_value,
                    &mut self._bitfield,
                    field_data,
                )?,
                7 => <self::_puroro::internal::field_type::OptionalBytesField<4> as FieldType>::deser_from_iter(
                    &mut self.string_value,
                    &mut self._bitfield,
                    field_data,
                )?,
                8 => <self::_puroro::internal::field_type::OptionalStringField::<5> as FieldType>::deser_from_iter(
                    &mut self.aggregate_value,
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
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::uninterpreted_option::NamePart,
        > as FieldType>::ser_to_write(&self.name, &self._bitfield, 2, out)?;
        <self::_puroro::internal::field_type::OptionalStringField<0> as FieldType>::ser_to_write(
            &self.identifier_value,
            &self._bitfield,
            3,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            u64,
            self::_puroro::tags::UInt64,
            1,
        > as FieldType>::ser_to_write(&self.positive_int_value, &self._bitfield, 4, out)?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i64,
            self::_puroro::tags::Int64,
            2,
        > as FieldType>::ser_to_write(&self.negative_int_value, &self._bitfield, 5, out)?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            f64,
            self::_puroro::tags::Double,
            3,
        > as FieldType>::ser_to_write(&self.double_value, &self._bitfield, 6, out)?;
        <self::_puroro::internal::field_type::OptionalBytesField<4> as FieldType>::ser_to_write(
            &self.string_value,
            &self._bitfield,
            7,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalStringField<5> as FieldType>::ser_to_write(
            &self.aggregate_value,
            &self._bitfield,
            8,
            out,
        )?;

        Ok(())
    }
}

impl ::std::clone::Clone for UninterpretedOption {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::clone::Clone;
        Self {
            name: Clone::clone(&self.name),
            identifier_value: Clone::clone(&self.identifier_value),
            positive_int_value: Clone::clone(&self.positive_int_value),
            negative_int_value: Clone::clone(&self.negative_int_value),
            double_value: Clone::clone(&self.double_value),
            string_value: Clone::clone(&self.string_value),
            aggregate_value: Clone::clone(&self.aggregate_value),

            _bitfield: Clone::clone(&self._bitfield),
        }
    }
}

impl ::std::fmt::Debug for UninterpretedOption {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct("UninterpretedOption")
            .field("name", &self.name())
            .field("identifier_value", &self.identifier_value())
            .field("positive_int_value", &self.positive_int_value())
            .field("negative_int_value", &self.negative_int_value())
            .field("double_value", &self.double_value())
            .field("string_value", &self.string_value())
            .field("aggregate_value", &self.aggregate_value())
            .finish()
    }
}

impl ::std::cmp::PartialEq for UninterpretedOption {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;

        true && self.name() == rhs.name()
            && self.identifier_value_opt() == rhs.identifier_value_opt()
            && self.positive_int_value_opt() == rhs.positive_int_value_opt()
            && self.negative_int_value_opt() == rhs.negative_int_value_opt()
            && self.double_value_opt() == rhs.double_value_opt()
            && self.string_value_opt() == rhs.string_value_opt()
            && self.aggregate_value_opt() == rhs.aggregate_value_opt()
    }
}

impl ::std::ops::Drop for UninterpretedOption {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
    }
}

#[derive(Default)]
pub struct SourceCodeInfo {
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.SourceCodeInfo.Location")))
    location: self::_puroro::internal::field_type::RepeatedMessageField<
        _puroro_root::google::protobuf::source_code_info::Location,
    >,

    _bitfield: self::_puroro::bitvec::BitArray<0>,
}

impl SourceCodeInfo {
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.SourceCodeInfo.Location")))
    pub fn location(&self) -> &[_puroro_root::google::protobuf::source_code_info::Location] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::source_code_info::Location,
        > as RepeatedFieldType>::get_field(&self.location, &self._bitfield)
    }
    pub fn location_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec<_puroro_root::google::protobuf::source_code_info::Location> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::source_code_info::Location,
        > as RepeatedFieldType>::mut_field(&mut self.location, &mut self._bitfield)
    }
    pub fn clear_location(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::source_code_info::Location,
        > as RepeatedFieldType>::clear(&mut self.location, &mut self._bitfield)
    }
}

impl self::_puroro::Message for SourceCodeInfo {
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
                1 => <self::_puroro::internal::field_type::RepeatedMessageField<
                    _puroro_root::google::protobuf::source_code_info::Location,
                > as FieldType>::deser_from_iter(
                    &mut self.location,
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
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::source_code_info::Location,
        > as FieldType>::ser_to_write(&self.location, &self._bitfield, 1, out)?;

        Ok(())
    }
}

impl ::std::clone::Clone for SourceCodeInfo {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::clone::Clone;
        Self {
            location: Clone::clone(&self.location),

            _bitfield: Clone::clone(&self._bitfield),
        }
    }
}

impl ::std::fmt::Debug for SourceCodeInfo {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct("SourceCodeInfo")
            .field("location", &self.location())
            .finish()
    }
}

impl ::std::cmp::PartialEq for SourceCodeInfo {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;

        true && self.location() == rhs.location()
    }
}

impl ::std::ops::Drop for SourceCodeInfo {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
    }
}

#[derive(Default)]
pub struct GeneratedCodeInfo {
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.GeneratedCodeInfo.Annotation")))
    annotation: self::_puroro::internal::field_type::RepeatedMessageField<
        _puroro_root::google::protobuf::generated_code_info::Annotation,
    >,

    _bitfield: self::_puroro::bitvec::BitArray<0>,
}

impl GeneratedCodeInfo {
    // Repeated, LengthDelimited(Message(Fqtn(".google.protobuf.GeneratedCodeInfo.Annotation")))
    pub fn annotation(&self) -> &[_puroro_root::google::protobuf::generated_code_info::Annotation] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::generated_code_info::Annotation,
        > as RepeatedFieldType>::get_field(&self.annotation, &self._bitfield)
    }
    pub fn annotation_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec<_puroro_root::google::protobuf::generated_code_info::Annotation> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::generated_code_info::Annotation,
        > as RepeatedFieldType>::mut_field(&mut self.annotation, &mut self._bitfield)
    }
    pub fn clear_annotation(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::generated_code_info::Annotation,
        > as RepeatedFieldType>::clear(&mut self.annotation, &mut self._bitfield)
    }
}

impl self::_puroro::Message for GeneratedCodeInfo {
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
                1 => <self::_puroro::internal::field_type::RepeatedMessageField<
                    _puroro_root::google::protobuf::generated_code_info::Annotation,
                > as FieldType>::deser_from_iter(
                    &mut self.annotation,
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
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::google::protobuf::generated_code_info::Annotation,
        > as FieldType>::ser_to_write(&self.annotation, &self._bitfield, 1, out)?;

        Ok(())
    }
}

impl ::std::clone::Clone for GeneratedCodeInfo {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::clone::Clone;
        Self {
            annotation: Clone::clone(&self.annotation),

            _bitfield: Clone::clone(&self._bitfield),
        }
    }
}

impl ::std::fmt::Debug for GeneratedCodeInfo {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct("GeneratedCodeInfo")
            .field("annotation", &self.annotation())
            .finish()
    }
}

impl ::std::cmp::PartialEq for GeneratedCodeInfo {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;

        true && self.annotation() == rhs.annotation()
    }
}

impl ::std::ops::Drop for GeneratedCodeInfo {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
    }
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
