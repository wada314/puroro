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
    file: self::_pinternal::field_type::RepeatedMessageField::<
        self::_root::google::protobuf::FileDescriptorProto,
    >,
    _bitfield: self::_pinternal::bitvec::BitArray<0usize>,
}
impl FileDescriptorSet {
    pub fn file(&self) -> &[self::_root::google::protobuf::FileDescriptorProto] {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::FileDescriptorProto,
        > as RepeatedFieldType>::get_field(&self.file, &self._bitfield)
    }
    pub fn file_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::FileDescriptorProto> {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::FileDescriptorProto,
        > as RepeatedFieldType>::get_field_mut(&mut self.file, &mut self._bitfield)
    }
    pub fn clear_file(&mut self) {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::FileDescriptorProto,
        > as RepeatedFieldType>::clear(&mut self.file, &mut self._bitfield)
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
        use self::_pinternal::oneof_type::OneofUnion as _;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            match number {
                1i32 => {
                    <self::_pinternal::field_type::RepeatedMessageField::<
                        self::_root::google::protobuf::FileDescriptorProto,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
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
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::FileDescriptorProto,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.file,
            &self._bitfield,
            1i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for FileDescriptorSet {
    fn clone(&self) -> Self {
        Self {
            file: <self::_pinternal::field_type::RepeatedMessageField::<
                self::_root::google::protobuf::FileDescriptorProto,
            > as ::std::clone::Clone>::clone(&self.file),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
impl ::std::ops::Drop for FileDescriptorSet {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for FileDescriptorSet {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(FileDescriptorSet))
            .field(stringify!(file), &self.file())
            .finish()
    }
}
impl ::std::cmp::PartialEq for FileDescriptorSet {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
        true && self.file() == rhs.file()
    }
}
#[derive(::std::default::Default)]
pub struct FileDescriptorProto {
    name: self::_pinternal::field_type::OptionalUnsizedField::<
        ::std::string::String,
        self::_pinternal::tags::String,
        0usize,
    >,
    package: self::_pinternal::field_type::OptionalUnsizedField::<
        ::std::string::String,
        self::_pinternal::tags::String,
        1usize,
    >,
    dependency: self::_pinternal::field_type::RepeatedUnsizedField::<
        ::std::string::String,
        self::_pinternal::tags::String,
    >,
    public_dependency: self::_pinternal::field_type::RepeatedNumericalField::<
        i32,
        self::_pinternal::tags::Int32,
    >,
    weak_dependency: self::_pinternal::field_type::RepeatedNumericalField::<
        i32,
        self::_pinternal::tags::Int32,
    >,
    message_type: self::_pinternal::field_type::RepeatedMessageField::<
        self::_root::google::protobuf::DescriptorProto,
    >,
    enum_type: self::_pinternal::field_type::RepeatedMessageField::<
        self::_root::google::protobuf::EnumDescriptorProto,
    >,
    service: self::_pinternal::field_type::RepeatedMessageField::<
        self::_root::google::protobuf::ServiceDescriptorProto,
    >,
    extension: self::_pinternal::field_type::RepeatedMessageField::<
        self::_root::google::protobuf::FieldDescriptorProto,
    >,
    options: self::_pinternal::field_type::SingularHeapMessageField::<
        self::_root::google::protobuf::FileOptions,
    >,
    source_code_info: self::_pinternal::field_type::SingularHeapMessageField::<
        self::_root::google::protobuf::SourceCodeInfo,
    >,
    syntax: self::_pinternal::field_type::OptionalUnsizedField::<
        ::std::string::String,
        self::_pinternal::tags::String,
        2usize,
    >,
    _bitfield: self::_pinternal::bitvec::BitArray<1usize>,
}
impl FileDescriptorProto {
    pub fn name(&self) -> &str {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.name,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn name_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.name, &self._bitfield)
    }
    pub fn name_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.name,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_name(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.name, &self._bitfield)
            .is_some()
    }
    pub fn clear_name(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::clear(&mut self.name, &mut self._bitfield)
    }
    pub fn package(&self) -> &str {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            1usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.package,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn package_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.package, &self._bitfield)
    }
    pub fn package_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            1usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.package,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_package(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.package, &self._bitfield)
            .is_some()
    }
    pub fn clear_package(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            1usize,
        > as NonRepeatedFieldType>::clear(&mut self.package, &mut self._bitfield)
    }
    pub fn dependency(
        &self,
    ) -> &[impl ::std::ops::Deref::<
        Target = str,
    > + ::std::fmt::Debug + ::std::cmp::PartialEq] {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as RepeatedFieldType>::get_field(&self.dependency, &self._bitfield)
    }
    pub fn dependency_mut(&mut self) -> &mut ::std::vec::Vec::<::std::string::String> {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as RepeatedFieldType>::get_field_mut(&mut self.dependency, &mut self._bitfield)
    }
    pub fn clear_dependency(&mut self) {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as RepeatedFieldType>::clear(&mut self.dependency, &mut self._bitfield)
    }
    pub fn public_dependency(&self) -> &[i32] {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
        > as RepeatedFieldType>::get_field(&self.public_dependency, &self._bitfield)
    }
    pub fn public_dependency_mut(&mut self) -> &mut ::std::vec::Vec::<i32> {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
        > as RepeatedFieldType>::get_field_mut(
            &mut self.public_dependency,
            &mut self._bitfield,
        )
    }
    pub fn clear_public_dependency(&mut self) {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
        > as RepeatedFieldType>::clear(&mut self.public_dependency, &mut self._bitfield)
    }
    pub fn weak_dependency(&self) -> &[i32] {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
        > as RepeatedFieldType>::get_field(&self.weak_dependency, &self._bitfield)
    }
    pub fn weak_dependency_mut(&mut self) -> &mut ::std::vec::Vec::<i32> {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
        > as RepeatedFieldType>::get_field_mut(
            &mut self.weak_dependency,
            &mut self._bitfield,
        )
    }
    pub fn clear_weak_dependency(&mut self) {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
        > as RepeatedFieldType>::clear(&mut self.weak_dependency, &mut self._bitfield)
    }
    pub fn message_type(&self) -> &[self::_root::google::protobuf::DescriptorProto] {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::DescriptorProto,
        > as RepeatedFieldType>::get_field(&self.message_type, &self._bitfield)
    }
    pub fn message_type_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::DescriptorProto> {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::DescriptorProto,
        > as RepeatedFieldType>::get_field_mut(
            &mut self.message_type,
            &mut self._bitfield,
        )
    }
    pub fn clear_message_type(&mut self) {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::DescriptorProto,
        > as RepeatedFieldType>::clear(&mut self.message_type, &mut self._bitfield)
    }
    pub fn enum_type(&self) -> &[self::_root::google::protobuf::EnumDescriptorProto] {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::EnumDescriptorProto,
        > as RepeatedFieldType>::get_field(&self.enum_type, &self._bitfield)
    }
    pub fn enum_type_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::EnumDescriptorProto> {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::EnumDescriptorProto,
        > as RepeatedFieldType>::get_field_mut(&mut self.enum_type, &mut self._bitfield)
    }
    pub fn clear_enum_type(&mut self) {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::EnumDescriptorProto,
        > as RepeatedFieldType>::clear(&mut self.enum_type, &mut self._bitfield)
    }
    pub fn service(&self) -> &[self::_root::google::protobuf::ServiceDescriptorProto] {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::ServiceDescriptorProto,
        > as RepeatedFieldType>::get_field(&self.service, &self._bitfield)
    }
    pub fn service_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::ServiceDescriptorProto> {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::ServiceDescriptorProto,
        > as RepeatedFieldType>::get_field_mut(&mut self.service, &mut self._bitfield)
    }
    pub fn clear_service(&mut self) {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::ServiceDescriptorProto,
        > as RepeatedFieldType>::clear(&mut self.service, &mut self._bitfield)
    }
    pub fn extension(&self) -> &[self::_root::google::protobuf::FieldDescriptorProto] {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::FieldDescriptorProto,
        > as RepeatedFieldType>::get_field(&self.extension, &self._bitfield)
    }
    pub fn extension_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::FieldDescriptorProto> {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::FieldDescriptorProto,
        > as RepeatedFieldType>::get_field_mut(&mut self.extension, &mut self._bitfield)
    }
    pub fn clear_extension(&mut self) {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::FieldDescriptorProto,
        > as RepeatedFieldType>::clear(&mut self.extension, &mut self._bitfield)
    }
    pub fn options(
        &self,
    ) -> ::std::option::Option::<&self::_root::google::protobuf::FileOptions> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::FileOptions,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.options,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn options_opt(
        &self,
    ) -> ::std::option::Option::<&self::_root::google::protobuf::FileOptions> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::FileOptions,
        > as NonRepeatedFieldType>::get_field_opt(&self.options, &self._bitfield)
    }
    pub fn options_mut(&mut self) -> &mut self::_root::google::protobuf::FileOptions {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::FileOptions,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.options,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_options(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::FileOptions,
        > as NonRepeatedFieldType>::get_field_opt(&self.options, &self._bitfield)
            .is_some()
    }
    pub fn clear_options(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::FileOptions,
        > as NonRepeatedFieldType>::clear(&mut self.options, &mut self._bitfield)
    }
    pub fn source_code_info(
        &self,
    ) -> ::std::option::Option::<&self::_root::google::protobuf::SourceCodeInfo> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::SourceCodeInfo,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.source_code_info,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn source_code_info_opt(
        &self,
    ) -> ::std::option::Option::<&self::_root::google::protobuf::SourceCodeInfo> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::SourceCodeInfo,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.source_code_info,
            &self._bitfield,
        )
    }
    pub fn source_code_info_mut(
        &mut self,
    ) -> &mut self::_root::google::protobuf::SourceCodeInfo {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::SourceCodeInfo,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.source_code_info,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_source_code_info(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::SourceCodeInfo,
        > as NonRepeatedFieldType>::get_field_opt(
                &self.source_code_info,
                &self._bitfield,
            )
            .is_some()
    }
    pub fn clear_source_code_info(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::SourceCodeInfo,
        > as NonRepeatedFieldType>::clear(
            &mut self.source_code_info,
            &mut self._bitfield,
        )
    }
    pub fn syntax(&self) -> &str {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            2usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.syntax,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn syntax_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            2usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.syntax, &self._bitfield)
    }
    pub fn syntax_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            2usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.syntax,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_syntax(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            2usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.syntax, &self._bitfield)
            .is_some()
    }
    pub fn clear_syntax(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            2usize,
        > as NonRepeatedFieldType>::clear(&mut self.syntax, &mut self._bitfield)
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
        use self::_pinternal::oneof_type::OneofUnion as _;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            match number {
                1i32 => {
                    <self::_pinternal::field_type::OptionalUnsizedField::<
                        ::std::string::String,
                        self::_pinternal::tags::String,
                        0usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.name,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                2i32 => {
                    <self::_pinternal::field_type::OptionalUnsizedField::<
                        ::std::string::String,
                        self::_pinternal::tags::String,
                        1usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.package,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                3i32 => {
                    <self::_pinternal::field_type::RepeatedUnsizedField::<
                        ::std::string::String,
                        self::_pinternal::tags::String,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.dependency,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                10i32 => {
                    <self::_pinternal::field_type::RepeatedNumericalField::<
                        i32,
                        self::_pinternal::tags::Int32,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.public_dependency,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                11i32 => {
                    <self::_pinternal::field_type::RepeatedNumericalField::<
                        i32,
                        self::_pinternal::tags::Int32,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.weak_dependency,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                4i32 => {
                    <self::_pinternal::field_type::RepeatedMessageField::<
                        self::_root::google::protobuf::DescriptorProto,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.message_type,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                5i32 => {
                    <self::_pinternal::field_type::RepeatedMessageField::<
                        self::_root::google::protobuf::EnumDescriptorProto,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.enum_type,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                6i32 => {
                    <self::_pinternal::field_type::RepeatedMessageField::<
                        self::_root::google::protobuf::ServiceDescriptorProto,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.service,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                7i32 => {
                    <self::_pinternal::field_type::RepeatedMessageField::<
                        self::_root::google::protobuf::FieldDescriptorProto,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.extension,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                8i32 => {
                    <self::_pinternal::field_type::SingularHeapMessageField::<
                        self::_root::google::protobuf::FileOptions,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.options,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                9i32 => {
                    <self::_pinternal::field_type::SingularHeapMessageField::<
                        self::_root::google::protobuf::SourceCodeInfo,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.source_code_info,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                12i32 => {
                    <self::_pinternal::field_type::OptionalUnsizedField::<
                        ::std::string::String,
                        self::_pinternal::tags::String,
                        2usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.syntax,
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
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.name,
            &self._bitfield,
            1i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            1usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.package,
            &self._bitfield,
            2i32,
            out,
        )?;
        <self::_pinternal::field_type::RepeatedUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.dependency,
            &self._bitfield,
            3i32,
            out,
        )?;
        <self::_pinternal::field_type::RepeatedNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.public_dependency,
            &self._bitfield,
            10i32,
            out,
        )?;
        <self::_pinternal::field_type::RepeatedNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.weak_dependency,
            &self._bitfield,
            11i32,
            out,
        )?;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::DescriptorProto,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.message_type,
            &self._bitfield,
            4i32,
            out,
        )?;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::EnumDescriptorProto,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.enum_type,
            &self._bitfield,
            5i32,
            out,
        )?;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::ServiceDescriptorProto,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.service,
            &self._bitfield,
            6i32,
            out,
        )?;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::FieldDescriptorProto,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.extension,
            &self._bitfield,
            7i32,
            out,
        )?;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::FileOptions,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.options,
            &self._bitfield,
            8i32,
            out,
        )?;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::SourceCodeInfo,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.source_code_info,
            &self._bitfield,
            9i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            2usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.syntax,
            &self._bitfield,
            12i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for FileDescriptorProto {
    fn clone(&self) -> Self {
        Self {
            name: <self::_pinternal::field_type::OptionalUnsizedField::<
                ::std::string::String,
                self::_pinternal::tags::String,
                0usize,
            > as ::std::clone::Clone>::clone(&self.name),
            package: <self::_pinternal::field_type::OptionalUnsizedField::<
                ::std::string::String,
                self::_pinternal::tags::String,
                1usize,
            > as ::std::clone::Clone>::clone(&self.package),
            dependency: <self::_pinternal::field_type::RepeatedUnsizedField::<
                ::std::string::String,
                self::_pinternal::tags::String,
            > as ::std::clone::Clone>::clone(&self.dependency),
            public_dependency: <self::_pinternal::field_type::RepeatedNumericalField::<
                i32,
                self::_pinternal::tags::Int32,
            > as ::std::clone::Clone>::clone(&self.public_dependency),
            weak_dependency: <self::_pinternal::field_type::RepeatedNumericalField::<
                i32,
                self::_pinternal::tags::Int32,
            > as ::std::clone::Clone>::clone(&self.weak_dependency),
            message_type: <self::_pinternal::field_type::RepeatedMessageField::<
                self::_root::google::protobuf::DescriptorProto,
            > as ::std::clone::Clone>::clone(&self.message_type),
            enum_type: <self::_pinternal::field_type::RepeatedMessageField::<
                self::_root::google::protobuf::EnumDescriptorProto,
            > as ::std::clone::Clone>::clone(&self.enum_type),
            service: <self::_pinternal::field_type::RepeatedMessageField::<
                self::_root::google::protobuf::ServiceDescriptorProto,
            > as ::std::clone::Clone>::clone(&self.service),
            extension: <self::_pinternal::field_type::RepeatedMessageField::<
                self::_root::google::protobuf::FieldDescriptorProto,
            > as ::std::clone::Clone>::clone(&self.extension),
            options: <self::_pinternal::field_type::SingularHeapMessageField::<
                self::_root::google::protobuf::FileOptions,
            > as ::std::clone::Clone>::clone(&self.options),
            source_code_info: <self::_pinternal::field_type::SingularHeapMessageField::<
                self::_root::google::protobuf::SourceCodeInfo,
            > as ::std::clone::Clone>::clone(&self.source_code_info),
            syntax: <self::_pinternal::field_type::OptionalUnsizedField::<
                ::std::string::String,
                self::_pinternal::tags::String,
                2usize,
            > as ::std::clone::Clone>::clone(&self.syntax),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
impl ::std::ops::Drop for FileDescriptorProto {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for FileDescriptorProto {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(FileDescriptorProto))
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
            .field(stringify!(syntax), &self.syntax_opt())
            .finish()
    }
}
impl ::std::cmp::PartialEq for FileDescriptorProto {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
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
    }
}
#[derive(::std::default::Default)]
pub struct DescriptorProto {
    name: self::_pinternal::field_type::OptionalUnsizedField::<
        ::std::string::String,
        self::_pinternal::tags::String,
        0usize,
    >,
    field: self::_pinternal::field_type::RepeatedMessageField::<
        self::_root::google::protobuf::FieldDescriptorProto,
    >,
    extension: self::_pinternal::field_type::RepeatedMessageField::<
        self::_root::google::protobuf::FieldDescriptorProto,
    >,
    nested_type: self::_pinternal::field_type::RepeatedMessageField::<
        self::_root::google::protobuf::DescriptorProto,
    >,
    enum_type: self::_pinternal::field_type::RepeatedMessageField::<
        self::_root::google::protobuf::EnumDescriptorProto,
    >,
    extension_range: self::_pinternal::field_type::RepeatedMessageField::<
        self::_root::google::protobuf::descriptor_proto::ExtensionRange,
    >,
    oneof_decl: self::_pinternal::field_type::RepeatedMessageField::<
        self::_root::google::protobuf::OneofDescriptorProto,
    >,
    options: self::_pinternal::field_type::SingularHeapMessageField::<
        self::_root::google::protobuf::MessageOptions,
    >,
    reserved_range: self::_pinternal::field_type::RepeatedMessageField::<
        self::_root::google::protobuf::descriptor_proto::ReservedRange,
    >,
    reserved_name: self::_pinternal::field_type::RepeatedUnsizedField::<
        ::std::string::String,
        self::_pinternal::tags::String,
    >,
    _bitfield: self::_pinternal::bitvec::BitArray<1usize>,
}
impl DescriptorProto {
    pub fn name(&self) -> &str {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.name,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn name_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.name, &self._bitfield)
    }
    pub fn name_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.name,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_name(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.name, &self._bitfield)
            .is_some()
    }
    pub fn clear_name(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::clear(&mut self.name, &mut self._bitfield)
    }
    pub fn field(&self) -> &[self::_root::google::protobuf::FieldDescriptorProto] {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::FieldDescriptorProto,
        > as RepeatedFieldType>::get_field(&self.field, &self._bitfield)
    }
    pub fn field_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::FieldDescriptorProto> {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::FieldDescriptorProto,
        > as RepeatedFieldType>::get_field_mut(&mut self.field, &mut self._bitfield)
    }
    pub fn clear_field(&mut self) {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::FieldDescriptorProto,
        > as RepeatedFieldType>::clear(&mut self.field, &mut self._bitfield)
    }
    pub fn extension(&self) -> &[self::_root::google::protobuf::FieldDescriptorProto] {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::FieldDescriptorProto,
        > as RepeatedFieldType>::get_field(&self.extension, &self._bitfield)
    }
    pub fn extension_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::FieldDescriptorProto> {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::FieldDescriptorProto,
        > as RepeatedFieldType>::get_field_mut(&mut self.extension, &mut self._bitfield)
    }
    pub fn clear_extension(&mut self) {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::FieldDescriptorProto,
        > as RepeatedFieldType>::clear(&mut self.extension, &mut self._bitfield)
    }
    pub fn nested_type(&self) -> &[self::_root::google::protobuf::DescriptorProto] {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::DescriptorProto,
        > as RepeatedFieldType>::get_field(&self.nested_type, &self._bitfield)
    }
    pub fn nested_type_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::DescriptorProto> {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::DescriptorProto,
        > as RepeatedFieldType>::get_field_mut(
            &mut self.nested_type,
            &mut self._bitfield,
        )
    }
    pub fn clear_nested_type(&mut self) {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::DescriptorProto,
        > as RepeatedFieldType>::clear(&mut self.nested_type, &mut self._bitfield)
    }
    pub fn enum_type(&self) -> &[self::_root::google::protobuf::EnumDescriptorProto] {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::EnumDescriptorProto,
        > as RepeatedFieldType>::get_field(&self.enum_type, &self._bitfield)
    }
    pub fn enum_type_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::EnumDescriptorProto> {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::EnumDescriptorProto,
        > as RepeatedFieldType>::get_field_mut(&mut self.enum_type, &mut self._bitfield)
    }
    pub fn clear_enum_type(&mut self) {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::EnumDescriptorProto,
        > as RepeatedFieldType>::clear(&mut self.enum_type, &mut self._bitfield)
    }
    pub fn extension_range(
        &self,
    ) -> &[self::_root::google::protobuf::descriptor_proto::ExtensionRange] {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::descriptor_proto::ExtensionRange,
        > as RepeatedFieldType>::get_field(&self.extension_range, &self._bitfield)
    }
    pub fn extension_range_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<
        self::_root::google::protobuf::descriptor_proto::ExtensionRange,
    > {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::descriptor_proto::ExtensionRange,
        > as RepeatedFieldType>::get_field_mut(
            &mut self.extension_range,
            &mut self._bitfield,
        )
    }
    pub fn clear_extension_range(&mut self) {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::descriptor_proto::ExtensionRange,
        > as RepeatedFieldType>::clear(&mut self.extension_range, &mut self._bitfield)
    }
    pub fn oneof_decl(&self) -> &[self::_root::google::protobuf::OneofDescriptorProto] {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::OneofDescriptorProto,
        > as RepeatedFieldType>::get_field(&self.oneof_decl, &self._bitfield)
    }
    pub fn oneof_decl_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::OneofDescriptorProto> {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::OneofDescriptorProto,
        > as RepeatedFieldType>::get_field_mut(&mut self.oneof_decl, &mut self._bitfield)
    }
    pub fn clear_oneof_decl(&mut self) {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::OneofDescriptorProto,
        > as RepeatedFieldType>::clear(&mut self.oneof_decl, &mut self._bitfield)
    }
    pub fn options(
        &self,
    ) -> ::std::option::Option::<&self::_root::google::protobuf::MessageOptions> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::MessageOptions,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.options,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn options_opt(
        &self,
    ) -> ::std::option::Option::<&self::_root::google::protobuf::MessageOptions> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::MessageOptions,
        > as NonRepeatedFieldType>::get_field_opt(&self.options, &self._bitfield)
    }
    pub fn options_mut(&mut self) -> &mut self::_root::google::protobuf::MessageOptions {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::MessageOptions,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.options,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_options(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::MessageOptions,
        > as NonRepeatedFieldType>::get_field_opt(&self.options, &self._bitfield)
            .is_some()
    }
    pub fn clear_options(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::MessageOptions,
        > as NonRepeatedFieldType>::clear(&mut self.options, &mut self._bitfield)
    }
    pub fn reserved_range(
        &self,
    ) -> &[self::_root::google::protobuf::descriptor_proto::ReservedRange] {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::descriptor_proto::ReservedRange,
        > as RepeatedFieldType>::get_field(&self.reserved_range, &self._bitfield)
    }
    pub fn reserved_range_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<
        self::_root::google::protobuf::descriptor_proto::ReservedRange,
    > {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::descriptor_proto::ReservedRange,
        > as RepeatedFieldType>::get_field_mut(
            &mut self.reserved_range,
            &mut self._bitfield,
        )
    }
    pub fn clear_reserved_range(&mut self) {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::descriptor_proto::ReservedRange,
        > as RepeatedFieldType>::clear(&mut self.reserved_range, &mut self._bitfield)
    }
    pub fn reserved_name(
        &self,
    ) -> &[impl ::std::ops::Deref::<
        Target = str,
    > + ::std::fmt::Debug + ::std::cmp::PartialEq] {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as RepeatedFieldType>::get_field(&self.reserved_name, &self._bitfield)
    }
    pub fn reserved_name_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<::std::string::String> {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as RepeatedFieldType>::get_field_mut(
            &mut self.reserved_name,
            &mut self._bitfield,
        )
    }
    pub fn clear_reserved_name(&mut self) {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as RepeatedFieldType>::clear(&mut self.reserved_name, &mut self._bitfield)
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
        use self::_pinternal::oneof_type::OneofUnion as _;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            match number {
                1i32 => {
                    <self::_pinternal::field_type::OptionalUnsizedField::<
                        ::std::string::String,
                        self::_pinternal::tags::String,
                        0usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.name,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                2i32 => {
                    <self::_pinternal::field_type::RepeatedMessageField::<
                        self::_root::google::protobuf::FieldDescriptorProto,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.field,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                6i32 => {
                    <self::_pinternal::field_type::RepeatedMessageField::<
                        self::_root::google::protobuf::FieldDescriptorProto,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.extension,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                3i32 => {
                    <self::_pinternal::field_type::RepeatedMessageField::<
                        self::_root::google::protobuf::DescriptorProto,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.nested_type,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                4i32 => {
                    <self::_pinternal::field_type::RepeatedMessageField::<
                        self::_root::google::protobuf::EnumDescriptorProto,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.enum_type,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                5i32 => {
                    <self::_pinternal::field_type::RepeatedMessageField::<
                        self::_root::google::protobuf::descriptor_proto::ExtensionRange,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.extension_range,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                8i32 => {
                    <self::_pinternal::field_type::RepeatedMessageField::<
                        self::_root::google::protobuf::OneofDescriptorProto,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.oneof_decl,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                7i32 => {
                    <self::_pinternal::field_type::SingularHeapMessageField::<
                        self::_root::google::protobuf::MessageOptions,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.options,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                9i32 => {
                    <self::_pinternal::field_type::RepeatedMessageField::<
                        self::_root::google::protobuf::descriptor_proto::ReservedRange,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.reserved_range,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                10i32 => {
                    <self::_pinternal::field_type::RepeatedUnsizedField::<
                        ::std::string::String,
                        self::_pinternal::tags::String,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.reserved_name,
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
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.name,
            &self._bitfield,
            1i32,
            out,
        )?;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::FieldDescriptorProto,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.field,
            &self._bitfield,
            2i32,
            out,
        )?;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::FieldDescriptorProto,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.extension,
            &self._bitfield,
            6i32,
            out,
        )?;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::DescriptorProto,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.nested_type,
            &self._bitfield,
            3i32,
            out,
        )?;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::EnumDescriptorProto,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.enum_type,
            &self._bitfield,
            4i32,
            out,
        )?;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::descriptor_proto::ExtensionRange,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.extension_range,
            &self._bitfield,
            5i32,
            out,
        )?;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::OneofDescriptorProto,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.oneof_decl,
            &self._bitfield,
            8i32,
            out,
        )?;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::MessageOptions,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.options,
            &self._bitfield,
            7i32,
            out,
        )?;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::descriptor_proto::ReservedRange,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.reserved_range,
            &self._bitfield,
            9i32,
            out,
        )?;
        <self::_pinternal::field_type::RepeatedUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.reserved_name,
            &self._bitfield,
            10i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for DescriptorProto {
    fn clone(&self) -> Self {
        Self {
            name: <self::_pinternal::field_type::OptionalUnsizedField::<
                ::std::string::String,
                self::_pinternal::tags::String,
                0usize,
            > as ::std::clone::Clone>::clone(&self.name),
            field: <self::_pinternal::field_type::RepeatedMessageField::<
                self::_root::google::protobuf::FieldDescriptorProto,
            > as ::std::clone::Clone>::clone(&self.field),
            extension: <self::_pinternal::field_type::RepeatedMessageField::<
                self::_root::google::protobuf::FieldDescriptorProto,
            > as ::std::clone::Clone>::clone(&self.extension),
            nested_type: <self::_pinternal::field_type::RepeatedMessageField::<
                self::_root::google::protobuf::DescriptorProto,
            > as ::std::clone::Clone>::clone(&self.nested_type),
            enum_type: <self::_pinternal::field_type::RepeatedMessageField::<
                self::_root::google::protobuf::EnumDescriptorProto,
            > as ::std::clone::Clone>::clone(&self.enum_type),
            extension_range: <self::_pinternal::field_type::RepeatedMessageField::<
                self::_root::google::protobuf::descriptor_proto::ExtensionRange,
            > as ::std::clone::Clone>::clone(&self.extension_range),
            oneof_decl: <self::_pinternal::field_type::RepeatedMessageField::<
                self::_root::google::protobuf::OneofDescriptorProto,
            > as ::std::clone::Clone>::clone(&self.oneof_decl),
            options: <self::_pinternal::field_type::SingularHeapMessageField::<
                self::_root::google::protobuf::MessageOptions,
            > as ::std::clone::Clone>::clone(&self.options),
            reserved_range: <self::_pinternal::field_type::RepeatedMessageField::<
                self::_root::google::protobuf::descriptor_proto::ReservedRange,
            > as ::std::clone::Clone>::clone(&self.reserved_range),
            reserved_name: <self::_pinternal::field_type::RepeatedUnsizedField::<
                ::std::string::String,
                self::_pinternal::tags::String,
            > as ::std::clone::Clone>::clone(&self.reserved_name),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
impl ::std::ops::Drop for DescriptorProto {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for DescriptorProto {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(DescriptorProto))
            .field(stringify!(name), &self.name_opt())
            .field(stringify!(field), &self.field())
            .field(stringify!(extension), &self.extension())
            .field(stringify!(nested_type), &self.nested_type())
            .field(stringify!(enum_type), &self.enum_type())
            .field(stringify!(extension_range), &self.extension_range())
            .field(stringify!(oneof_decl), &self.oneof_decl())
            .field(stringify!(options), &self.options_opt())
            .field(stringify!(reserved_range), &self.reserved_range())
            .field(stringify!(reserved_name), &self.reserved_name())
            .finish()
    }
}
impl ::std::cmp::PartialEq for DescriptorProto {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
        true && self.name_opt() == rhs.name_opt() && self.field() == rhs.field()
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
#[derive(::std::default::Default)]
pub struct ExtensionRangeOptions {
    uninterpreted_option: self::_pinternal::field_type::RepeatedMessageField::<
        self::_root::google::protobuf::UninterpretedOption,
    >,
    _bitfield: self::_pinternal::bitvec::BitArray<0usize>,
}
impl ExtensionRangeOptions {
    pub fn uninterpreted_option(
        &self,
    ) -> &[self::_root::google::protobuf::UninterpretedOption] {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::get_field(&self.uninterpreted_option, &self._bitfield)
    }
    pub fn uninterpreted_option_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::UninterpretedOption> {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::get_field_mut(
            &mut self.uninterpreted_option,
            &mut self._bitfield,
        )
    }
    pub fn clear_uninterpreted_option(&mut self) {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::clear(
            &mut self.uninterpreted_option,
            &mut self._bitfield,
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
        use self::_pinternal::oneof_type::OneofUnion as _;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            match number {
                999i32 => {
                    <self::_pinternal::field_type::RepeatedMessageField::<
                        self::_root::google::protobuf::UninterpretedOption,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.uninterpreted_option,
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
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::UninterpretedOption,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.uninterpreted_option,
            &self._bitfield,
            999i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for ExtensionRangeOptions {
    fn clone(&self) -> Self {
        Self {
            uninterpreted_option: <self::_pinternal::field_type::RepeatedMessageField::<
                self::_root::google::protobuf::UninterpretedOption,
            > as ::std::clone::Clone>::clone(&self.uninterpreted_option),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
impl ::std::ops::Drop for ExtensionRangeOptions {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for ExtensionRangeOptions {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(ExtensionRangeOptions))
            .field(stringify!(uninterpreted_option), &self.uninterpreted_option())
            .finish()
    }
}
impl ::std::cmp::PartialEq for ExtensionRangeOptions {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
        true && self.uninterpreted_option() == rhs.uninterpreted_option()
    }
}
#[derive(::std::default::Default)]
pub struct FieldDescriptorProto {
    name: self::_pinternal::field_type::OptionalUnsizedField::<
        ::std::string::String,
        self::_pinternal::tags::String,
        0usize,
    >,
    number: self::_pinternal::field_type::OptionalNumericalField::<
        i32,
        self::_pinternal::tags::Int32,
        1usize,
    >,
    label: self::_pinternal::field_type::OptionalNumericalField::<
        self::_root::google::protobuf::field_descriptor_proto::Label,
        self::_pinternal::tags::Enum2::<
            self::_root::google::protobuf::field_descriptor_proto::Label,
        >,
        2usize,
    >,
    r#type: self::_pinternal::field_type::OptionalNumericalField::<
        self::_root::google::protobuf::field_descriptor_proto::Type,
        self::_pinternal::tags::Enum2::<
            self::_root::google::protobuf::field_descriptor_proto::Type,
        >,
        3usize,
    >,
    type_name: self::_pinternal::field_type::OptionalUnsizedField::<
        ::std::string::String,
        self::_pinternal::tags::String,
        4usize,
    >,
    extendee: self::_pinternal::field_type::OptionalUnsizedField::<
        ::std::string::String,
        self::_pinternal::tags::String,
        5usize,
    >,
    default_value: self::_pinternal::field_type::OptionalUnsizedField::<
        ::std::string::String,
        self::_pinternal::tags::String,
        6usize,
    >,
    oneof_index: self::_pinternal::field_type::OptionalNumericalField::<
        i32,
        self::_pinternal::tags::Int32,
        7usize,
    >,
    json_name: self::_pinternal::field_type::OptionalUnsizedField::<
        ::std::string::String,
        self::_pinternal::tags::String,
        8usize,
    >,
    options: self::_pinternal::field_type::SingularHeapMessageField::<
        self::_root::google::protobuf::FieldOptions,
    >,
    proto3_optional: self::_pinternal::field_type::OptionalNumericalField::<
        bool,
        self::_pinternal::tags::Bool,
        9usize,
    >,
    _bitfield: self::_pinternal::bitvec::BitArray<1usize>,
}
impl FieldDescriptorProto {
    pub fn name(&self) -> &str {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.name,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn name_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.name, &self._bitfield)
    }
    pub fn name_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.name,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_name(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.name, &self._bitfield)
            .is_some()
    }
    pub fn clear_name(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::clear(&mut self.name, &mut self._bitfield)
    }
    pub fn number(&self) -> i32 {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            1usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.number,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn number_opt(&self) -> ::std::option::Option::<i32> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.number, &self._bitfield)
    }
    pub fn number_mut(&mut self) -> &mut i32 {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            1usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.number,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_number(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.number, &self._bitfield)
            .is_some()
    }
    pub fn clear_number(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            1usize,
        > as NonRepeatedFieldType>::clear(&mut self.number, &mut self._bitfield)
    }
    pub fn label(&self) -> self::_root::google::protobuf::field_descriptor_proto::Label {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            self::_root::google::protobuf::field_descriptor_proto::Label,
            self::_pinternal::tags::Enum2::<
                self::_root::google::protobuf::field_descriptor_proto::Label,
            >,
            2usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.label,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn label_opt(
        &self,
    ) -> ::std::option::Option::<
        self::_root::google::protobuf::field_descriptor_proto::Label,
    > {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            self::_root::google::protobuf::field_descriptor_proto::Label,
            self::_pinternal::tags::Enum2::<
                self::_root::google::protobuf::field_descriptor_proto::Label,
            >,
            2usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.label, &self._bitfield)
    }
    pub fn label_mut(
        &mut self,
    ) -> &mut self::_root::google::protobuf::field_descriptor_proto::Label {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            self::_root::google::protobuf::field_descriptor_proto::Label,
            self::_pinternal::tags::Enum2::<
                self::_root::google::protobuf::field_descriptor_proto::Label,
            >,
            2usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.label,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_label(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            self::_root::google::protobuf::field_descriptor_proto::Label,
            self::_pinternal::tags::Enum2::<
                self::_root::google::protobuf::field_descriptor_proto::Label,
            >,
            2usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.label, &self._bitfield)
            .is_some()
    }
    pub fn clear_label(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            self::_root::google::protobuf::field_descriptor_proto::Label,
            self::_pinternal::tags::Enum2::<
                self::_root::google::protobuf::field_descriptor_proto::Label,
            >,
            2usize,
        > as NonRepeatedFieldType>::clear(&mut self.label, &mut self._bitfield)
    }
    pub fn r#type(&self) -> self::_root::google::protobuf::field_descriptor_proto::Type {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            self::_root::google::protobuf::field_descriptor_proto::Type,
            self::_pinternal::tags::Enum2::<
                self::_root::google::protobuf::field_descriptor_proto::Type,
            >,
            3usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.r#type,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn type_opt(
        &self,
    ) -> ::std::option::Option::<
        self::_root::google::protobuf::field_descriptor_proto::Type,
    > {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            self::_root::google::protobuf::field_descriptor_proto::Type,
            self::_pinternal::tags::Enum2::<
                self::_root::google::protobuf::field_descriptor_proto::Type,
            >,
            3usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.r#type, &self._bitfield)
    }
    pub fn type_mut(
        &mut self,
    ) -> &mut self::_root::google::protobuf::field_descriptor_proto::Type {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            self::_root::google::protobuf::field_descriptor_proto::Type,
            self::_pinternal::tags::Enum2::<
                self::_root::google::protobuf::field_descriptor_proto::Type,
            >,
            3usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.r#type,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_type(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            self::_root::google::protobuf::field_descriptor_proto::Type,
            self::_pinternal::tags::Enum2::<
                self::_root::google::protobuf::field_descriptor_proto::Type,
            >,
            3usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.r#type, &self._bitfield)
            .is_some()
    }
    pub fn clear_type(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            self::_root::google::protobuf::field_descriptor_proto::Type,
            self::_pinternal::tags::Enum2::<
                self::_root::google::protobuf::field_descriptor_proto::Type,
            >,
            3usize,
        > as NonRepeatedFieldType>::clear(&mut self.r#type, &mut self._bitfield)
    }
    pub fn type_name(&self) -> &str {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            4usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.type_name,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn type_name_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            4usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.type_name, &self._bitfield)
    }
    pub fn type_name_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            4usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.type_name,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_type_name(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            4usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.type_name, &self._bitfield)
            .is_some()
    }
    pub fn clear_type_name(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            4usize,
        > as NonRepeatedFieldType>::clear(&mut self.type_name, &mut self._bitfield)
    }
    pub fn extendee(&self) -> &str {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            5usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.extendee,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn extendee_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            5usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.extendee, &self._bitfield)
    }
    pub fn extendee_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            5usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.extendee,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_extendee(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            5usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.extendee, &self._bitfield)
            .is_some()
    }
    pub fn clear_extendee(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            5usize,
        > as NonRepeatedFieldType>::clear(&mut self.extendee, &mut self._bitfield)
    }
    pub fn default_value(&self) -> &str {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            6usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.default_value,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn default_value_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            6usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.default_value, &self._bitfield)
    }
    pub fn default_value_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            6usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.default_value,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_default_value(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            6usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.default_value, &self._bitfield)
            .is_some()
    }
    pub fn clear_default_value(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            6usize,
        > as NonRepeatedFieldType>::clear(&mut self.default_value, &mut self._bitfield)
    }
    pub fn oneof_index(&self) -> i32 {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            7usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.oneof_index,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn oneof_index_opt(&self) -> ::std::option::Option::<i32> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            7usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.oneof_index, &self._bitfield)
    }
    pub fn oneof_index_mut(&mut self) -> &mut i32 {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            7usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.oneof_index,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_oneof_index(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            7usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.oneof_index, &self._bitfield)
            .is_some()
    }
    pub fn clear_oneof_index(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            7usize,
        > as NonRepeatedFieldType>::clear(&mut self.oneof_index, &mut self._bitfield)
    }
    pub fn json_name(&self) -> &str {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            8usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.json_name,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn json_name_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            8usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.json_name, &self._bitfield)
    }
    pub fn json_name_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            8usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.json_name,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_json_name(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            8usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.json_name, &self._bitfield)
            .is_some()
    }
    pub fn clear_json_name(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            8usize,
        > as NonRepeatedFieldType>::clear(&mut self.json_name, &mut self._bitfield)
    }
    pub fn options(
        &self,
    ) -> ::std::option::Option::<&self::_root::google::protobuf::FieldOptions> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::FieldOptions,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.options,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn options_opt(
        &self,
    ) -> ::std::option::Option::<&self::_root::google::protobuf::FieldOptions> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::FieldOptions,
        > as NonRepeatedFieldType>::get_field_opt(&self.options, &self._bitfield)
    }
    pub fn options_mut(&mut self) -> &mut self::_root::google::protobuf::FieldOptions {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::FieldOptions,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.options,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_options(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::FieldOptions,
        > as NonRepeatedFieldType>::get_field_opt(&self.options, &self._bitfield)
            .is_some()
    }
    pub fn clear_options(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::FieldOptions,
        > as NonRepeatedFieldType>::clear(&mut self.options, &mut self._bitfield)
    }
    pub fn proto3_optional(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            9usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.proto3_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn proto3_optional_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            9usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.proto3_optional, &self._bitfield)
    }
    pub fn proto3_optional_mut(&mut self) -> &mut bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            9usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.proto3_optional,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_proto3_optional(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            9usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.proto3_optional, &self._bitfield)
            .is_some()
    }
    pub fn clear_proto3_optional(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            9usize,
        > as NonRepeatedFieldType>::clear(&mut self.proto3_optional, &mut self._bitfield)
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
        use self::_pinternal::oneof_type::OneofUnion as _;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            match number {
                1i32 => {
                    <self::_pinternal::field_type::OptionalUnsizedField::<
                        ::std::string::String,
                        self::_pinternal::tags::String,
                        0usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.name,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                3i32 => {
                    <self::_pinternal::field_type::OptionalNumericalField::<
                        i32,
                        self::_pinternal::tags::Int32,
                        1usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.number,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                4i32 => {
                    <self::_pinternal::field_type::OptionalNumericalField::<
                        self::_root::google::protobuf::field_descriptor_proto::Label,
                        self::_pinternal::tags::Enum2::<
                            self::_root::google::protobuf::field_descriptor_proto::Label,
                        >,
                        2usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.label,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                5i32 => {
                    <self::_pinternal::field_type::OptionalNumericalField::<
                        self::_root::google::protobuf::field_descriptor_proto::Type,
                        self::_pinternal::tags::Enum2::<
                            self::_root::google::protobuf::field_descriptor_proto::Type,
                        >,
                        3usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.r#type,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                6i32 => {
                    <self::_pinternal::field_type::OptionalUnsizedField::<
                        ::std::string::String,
                        self::_pinternal::tags::String,
                        4usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.type_name,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                2i32 => {
                    <self::_pinternal::field_type::OptionalUnsizedField::<
                        ::std::string::String,
                        self::_pinternal::tags::String,
                        5usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.extendee,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                7i32 => {
                    <self::_pinternal::field_type::OptionalUnsizedField::<
                        ::std::string::String,
                        self::_pinternal::tags::String,
                        6usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.default_value,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                9i32 => {
                    <self::_pinternal::field_type::OptionalNumericalField::<
                        i32,
                        self::_pinternal::tags::Int32,
                        7usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.oneof_index,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                10i32 => {
                    <self::_pinternal::field_type::OptionalUnsizedField::<
                        ::std::string::String,
                        self::_pinternal::tags::String,
                        8usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.json_name,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                8i32 => {
                    <self::_pinternal::field_type::SingularHeapMessageField::<
                        self::_root::google::protobuf::FieldOptions,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.options,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                17i32 => {
                    <self::_pinternal::field_type::OptionalNumericalField::<
                        bool,
                        self::_pinternal::tags::Bool,
                        9usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.proto3_optional,
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
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.name,
            &self._bitfield,
            1i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            1usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.number,
            &self._bitfield,
            3i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalNumericalField::<
            self::_root::google::protobuf::field_descriptor_proto::Label,
            self::_pinternal::tags::Enum2::<
                self::_root::google::protobuf::field_descriptor_proto::Label,
            >,
            2usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.label,
            &self._bitfield,
            4i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalNumericalField::<
            self::_root::google::protobuf::field_descriptor_proto::Type,
            self::_pinternal::tags::Enum2::<
                self::_root::google::protobuf::field_descriptor_proto::Type,
            >,
            3usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.r#type,
            &self._bitfield,
            5i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            4usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.type_name,
            &self._bitfield,
            6i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            5usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.extendee,
            &self._bitfield,
            2i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            6usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.default_value,
            &self._bitfield,
            7i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            7usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.oneof_index,
            &self._bitfield,
            9i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            8usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.json_name,
            &self._bitfield,
            10i32,
            out,
        )?;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::FieldOptions,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.options,
            &self._bitfield,
            8i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            9usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.proto3_optional,
            &self._bitfield,
            17i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for FieldDescriptorProto {
    fn clone(&self) -> Self {
        Self {
            name: <self::_pinternal::field_type::OptionalUnsizedField::<
                ::std::string::String,
                self::_pinternal::tags::String,
                0usize,
            > as ::std::clone::Clone>::clone(&self.name),
            number: <self::_pinternal::field_type::OptionalNumericalField::<
                i32,
                self::_pinternal::tags::Int32,
                1usize,
            > as ::std::clone::Clone>::clone(&self.number),
            label: <self::_pinternal::field_type::OptionalNumericalField::<
                self::_root::google::protobuf::field_descriptor_proto::Label,
                self::_pinternal::tags::Enum2::<
                    self::_root::google::protobuf::field_descriptor_proto::Label,
                >,
                2usize,
            > as ::std::clone::Clone>::clone(&self.label),
            r#type: <self::_pinternal::field_type::OptionalNumericalField::<
                self::_root::google::protobuf::field_descriptor_proto::Type,
                self::_pinternal::tags::Enum2::<
                    self::_root::google::protobuf::field_descriptor_proto::Type,
                >,
                3usize,
            > as ::std::clone::Clone>::clone(&self.r#type),
            type_name: <self::_pinternal::field_type::OptionalUnsizedField::<
                ::std::string::String,
                self::_pinternal::tags::String,
                4usize,
            > as ::std::clone::Clone>::clone(&self.type_name),
            extendee: <self::_pinternal::field_type::OptionalUnsizedField::<
                ::std::string::String,
                self::_pinternal::tags::String,
                5usize,
            > as ::std::clone::Clone>::clone(&self.extendee),
            default_value: <self::_pinternal::field_type::OptionalUnsizedField::<
                ::std::string::String,
                self::_pinternal::tags::String,
                6usize,
            > as ::std::clone::Clone>::clone(&self.default_value),
            oneof_index: <self::_pinternal::field_type::OptionalNumericalField::<
                i32,
                self::_pinternal::tags::Int32,
                7usize,
            > as ::std::clone::Clone>::clone(&self.oneof_index),
            json_name: <self::_pinternal::field_type::OptionalUnsizedField::<
                ::std::string::String,
                self::_pinternal::tags::String,
                8usize,
            > as ::std::clone::Clone>::clone(&self.json_name),
            options: <self::_pinternal::field_type::SingularHeapMessageField::<
                self::_root::google::protobuf::FieldOptions,
            > as ::std::clone::Clone>::clone(&self.options),
            proto3_optional: <self::_pinternal::field_type::OptionalNumericalField::<
                bool,
                self::_pinternal::tags::Bool,
                9usize,
            > as ::std::clone::Clone>::clone(&self.proto3_optional),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
impl ::std::ops::Drop for FieldDescriptorProto {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for FieldDescriptorProto {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(FieldDescriptorProto))
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
            .field(stringify!(proto3_optional), &self.proto3_optional_opt())
            .finish()
    }
}
impl ::std::cmp::PartialEq for FieldDescriptorProto {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
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
    }
}
#[derive(::std::default::Default)]
pub struct OneofDescriptorProto {
    name: self::_pinternal::field_type::OptionalUnsizedField::<
        ::std::string::String,
        self::_pinternal::tags::String,
        0usize,
    >,
    options: self::_pinternal::field_type::SingularHeapMessageField::<
        self::_root::google::protobuf::OneofOptions,
    >,
    _bitfield: self::_pinternal::bitvec::BitArray<1usize>,
}
impl OneofDescriptorProto {
    pub fn name(&self) -> &str {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.name,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn name_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.name, &self._bitfield)
    }
    pub fn name_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.name,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_name(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.name, &self._bitfield)
            .is_some()
    }
    pub fn clear_name(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::clear(&mut self.name, &mut self._bitfield)
    }
    pub fn options(
        &self,
    ) -> ::std::option::Option::<&self::_root::google::protobuf::OneofOptions> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::OneofOptions,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.options,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn options_opt(
        &self,
    ) -> ::std::option::Option::<&self::_root::google::protobuf::OneofOptions> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::OneofOptions,
        > as NonRepeatedFieldType>::get_field_opt(&self.options, &self._bitfield)
    }
    pub fn options_mut(&mut self) -> &mut self::_root::google::protobuf::OneofOptions {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::OneofOptions,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.options,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_options(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::OneofOptions,
        > as NonRepeatedFieldType>::get_field_opt(&self.options, &self._bitfield)
            .is_some()
    }
    pub fn clear_options(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::OneofOptions,
        > as NonRepeatedFieldType>::clear(&mut self.options, &mut self._bitfield)
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
        use self::_pinternal::oneof_type::OneofUnion as _;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            match number {
                1i32 => {
                    <self::_pinternal::field_type::OptionalUnsizedField::<
                        ::std::string::String,
                        self::_pinternal::tags::String,
                        0usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.name,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                2i32 => {
                    <self::_pinternal::field_type::SingularHeapMessageField::<
                        self::_root::google::protobuf::OneofOptions,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.options,
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
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.name,
            &self._bitfield,
            1i32,
            out,
        )?;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::OneofOptions,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.options,
            &self._bitfield,
            2i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for OneofDescriptorProto {
    fn clone(&self) -> Self {
        Self {
            name: <self::_pinternal::field_type::OptionalUnsizedField::<
                ::std::string::String,
                self::_pinternal::tags::String,
                0usize,
            > as ::std::clone::Clone>::clone(&self.name),
            options: <self::_pinternal::field_type::SingularHeapMessageField::<
                self::_root::google::protobuf::OneofOptions,
            > as ::std::clone::Clone>::clone(&self.options),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
impl ::std::ops::Drop for OneofDescriptorProto {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for OneofDescriptorProto {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(OneofDescriptorProto))
            .field(stringify!(name), &self.name_opt())
            .field(stringify!(options), &self.options_opt())
            .finish()
    }
}
impl ::std::cmp::PartialEq for OneofDescriptorProto {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
        true && self.name_opt() == rhs.name_opt()
            && self.options_opt() == rhs.options_opt()
    }
}
#[derive(::std::default::Default)]
pub struct EnumDescriptorProto {
    name: self::_pinternal::field_type::OptionalUnsizedField::<
        ::std::string::String,
        self::_pinternal::tags::String,
        0usize,
    >,
    value: self::_pinternal::field_type::RepeatedMessageField::<
        self::_root::google::protobuf::EnumValueDescriptorProto,
    >,
    options: self::_pinternal::field_type::SingularHeapMessageField::<
        self::_root::google::protobuf::EnumOptions,
    >,
    reserved_range: self::_pinternal::field_type::RepeatedMessageField::<
        self::_root::google::protobuf::enum_descriptor_proto::EnumReservedRange,
    >,
    reserved_name: self::_pinternal::field_type::RepeatedUnsizedField::<
        ::std::string::String,
        self::_pinternal::tags::String,
    >,
    _bitfield: self::_pinternal::bitvec::BitArray<1usize>,
}
impl EnumDescriptorProto {
    pub fn name(&self) -> &str {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.name,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn name_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.name, &self._bitfield)
    }
    pub fn name_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.name,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_name(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.name, &self._bitfield)
            .is_some()
    }
    pub fn clear_name(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::clear(&mut self.name, &mut self._bitfield)
    }
    pub fn value(&self) -> &[self::_root::google::protobuf::EnumValueDescriptorProto] {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::EnumValueDescriptorProto,
        > as RepeatedFieldType>::get_field(&self.value, &self._bitfield)
    }
    pub fn value_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<
        self::_root::google::protobuf::EnumValueDescriptorProto,
    > {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::EnumValueDescriptorProto,
        > as RepeatedFieldType>::get_field_mut(&mut self.value, &mut self._bitfield)
    }
    pub fn clear_value(&mut self) {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::EnumValueDescriptorProto,
        > as RepeatedFieldType>::clear(&mut self.value, &mut self._bitfield)
    }
    pub fn options(
        &self,
    ) -> ::std::option::Option::<&self::_root::google::protobuf::EnumOptions> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::EnumOptions,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.options,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn options_opt(
        &self,
    ) -> ::std::option::Option::<&self::_root::google::protobuf::EnumOptions> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::EnumOptions,
        > as NonRepeatedFieldType>::get_field_opt(&self.options, &self._bitfield)
    }
    pub fn options_mut(&mut self) -> &mut self::_root::google::protobuf::EnumOptions {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::EnumOptions,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.options,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_options(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::EnumOptions,
        > as NonRepeatedFieldType>::get_field_opt(&self.options, &self._bitfield)
            .is_some()
    }
    pub fn clear_options(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::EnumOptions,
        > as NonRepeatedFieldType>::clear(&mut self.options, &mut self._bitfield)
    }
    pub fn reserved_range(
        &self,
    ) -> &[self::_root::google::protobuf::enum_descriptor_proto::EnumReservedRange] {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::enum_descriptor_proto::EnumReservedRange,
        > as RepeatedFieldType>::get_field(&self.reserved_range, &self._bitfield)
    }
    pub fn reserved_range_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<
        self::_root::google::protobuf::enum_descriptor_proto::EnumReservedRange,
    > {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::enum_descriptor_proto::EnumReservedRange,
        > as RepeatedFieldType>::get_field_mut(
            &mut self.reserved_range,
            &mut self._bitfield,
        )
    }
    pub fn clear_reserved_range(&mut self) {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::enum_descriptor_proto::EnumReservedRange,
        > as RepeatedFieldType>::clear(&mut self.reserved_range, &mut self._bitfield)
    }
    pub fn reserved_name(
        &self,
    ) -> &[impl ::std::ops::Deref::<
        Target = str,
    > + ::std::fmt::Debug + ::std::cmp::PartialEq] {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as RepeatedFieldType>::get_field(&self.reserved_name, &self._bitfield)
    }
    pub fn reserved_name_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<::std::string::String> {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as RepeatedFieldType>::get_field_mut(
            &mut self.reserved_name,
            &mut self._bitfield,
        )
    }
    pub fn clear_reserved_name(&mut self) {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as RepeatedFieldType>::clear(&mut self.reserved_name, &mut self._bitfield)
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
        use self::_pinternal::oneof_type::OneofUnion as _;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            match number {
                1i32 => {
                    <self::_pinternal::field_type::OptionalUnsizedField::<
                        ::std::string::String,
                        self::_pinternal::tags::String,
                        0usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.name,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                2i32 => {
                    <self::_pinternal::field_type::RepeatedMessageField::<
                        self::_root::google::protobuf::EnumValueDescriptorProto,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.value,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                3i32 => {
                    <self::_pinternal::field_type::SingularHeapMessageField::<
                        self::_root::google::protobuf::EnumOptions,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.options,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                4i32 => {
                    <self::_pinternal::field_type::RepeatedMessageField::<
                        self::_root::google::protobuf::enum_descriptor_proto::EnumReservedRange,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.reserved_range,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                5i32 => {
                    <self::_pinternal::field_type::RepeatedUnsizedField::<
                        ::std::string::String,
                        self::_pinternal::tags::String,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.reserved_name,
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
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.name,
            &self._bitfield,
            1i32,
            out,
        )?;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::EnumValueDescriptorProto,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.value,
            &self._bitfield,
            2i32,
            out,
        )?;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::EnumOptions,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.options,
            &self._bitfield,
            3i32,
            out,
        )?;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::enum_descriptor_proto::EnumReservedRange,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.reserved_range,
            &self._bitfield,
            4i32,
            out,
        )?;
        <self::_pinternal::field_type::RepeatedUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.reserved_name,
            &self._bitfield,
            5i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for EnumDescriptorProto {
    fn clone(&self) -> Self {
        Self {
            name: <self::_pinternal::field_type::OptionalUnsizedField::<
                ::std::string::String,
                self::_pinternal::tags::String,
                0usize,
            > as ::std::clone::Clone>::clone(&self.name),
            value: <self::_pinternal::field_type::RepeatedMessageField::<
                self::_root::google::protobuf::EnumValueDescriptorProto,
            > as ::std::clone::Clone>::clone(&self.value),
            options: <self::_pinternal::field_type::SingularHeapMessageField::<
                self::_root::google::protobuf::EnumOptions,
            > as ::std::clone::Clone>::clone(&self.options),
            reserved_range: <self::_pinternal::field_type::RepeatedMessageField::<
                self::_root::google::protobuf::enum_descriptor_proto::EnumReservedRange,
            > as ::std::clone::Clone>::clone(&self.reserved_range),
            reserved_name: <self::_pinternal::field_type::RepeatedUnsizedField::<
                ::std::string::String,
                self::_pinternal::tags::String,
            > as ::std::clone::Clone>::clone(&self.reserved_name),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
impl ::std::ops::Drop for EnumDescriptorProto {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for EnumDescriptorProto {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(EnumDescriptorProto))
            .field(stringify!(name), &self.name_opt())
            .field(stringify!(value), &self.value())
            .field(stringify!(options), &self.options_opt())
            .field(stringify!(reserved_range), &self.reserved_range())
            .field(stringify!(reserved_name), &self.reserved_name())
            .finish()
    }
}
impl ::std::cmp::PartialEq for EnumDescriptorProto {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
        true && self.name_opt() == rhs.name_opt() && self.value() == rhs.value()
            && self.options_opt() == rhs.options_opt()
            && self.reserved_range() == rhs.reserved_range()
            && self.reserved_name() == rhs.reserved_name()
    }
}
#[derive(::std::default::Default)]
pub struct EnumValueDescriptorProto {
    name: self::_pinternal::field_type::OptionalUnsizedField::<
        ::std::string::String,
        self::_pinternal::tags::String,
        0usize,
    >,
    number: self::_pinternal::field_type::OptionalNumericalField::<
        i32,
        self::_pinternal::tags::Int32,
        1usize,
    >,
    options: self::_pinternal::field_type::SingularHeapMessageField::<
        self::_root::google::protobuf::EnumValueOptions,
    >,
    _bitfield: self::_pinternal::bitvec::BitArray<1usize>,
}
impl EnumValueDescriptorProto {
    pub fn name(&self) -> &str {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.name,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn name_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.name, &self._bitfield)
    }
    pub fn name_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.name,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_name(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.name, &self._bitfield)
            .is_some()
    }
    pub fn clear_name(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::clear(&mut self.name, &mut self._bitfield)
    }
    pub fn number(&self) -> i32 {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            1usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.number,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn number_opt(&self) -> ::std::option::Option::<i32> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.number, &self._bitfield)
    }
    pub fn number_mut(&mut self) -> &mut i32 {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            1usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.number,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_number(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.number, &self._bitfield)
            .is_some()
    }
    pub fn clear_number(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            1usize,
        > as NonRepeatedFieldType>::clear(&mut self.number, &mut self._bitfield)
    }
    pub fn options(
        &self,
    ) -> ::std::option::Option::<&self::_root::google::protobuf::EnumValueOptions> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::EnumValueOptions,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.options,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn options_opt(
        &self,
    ) -> ::std::option::Option::<&self::_root::google::protobuf::EnumValueOptions> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::EnumValueOptions,
        > as NonRepeatedFieldType>::get_field_opt(&self.options, &self._bitfield)
    }
    pub fn options_mut(
        &mut self,
    ) -> &mut self::_root::google::protobuf::EnumValueOptions {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::EnumValueOptions,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.options,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_options(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::EnumValueOptions,
        > as NonRepeatedFieldType>::get_field_opt(&self.options, &self._bitfield)
            .is_some()
    }
    pub fn clear_options(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::EnumValueOptions,
        > as NonRepeatedFieldType>::clear(&mut self.options, &mut self._bitfield)
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
        use self::_pinternal::oneof_type::OneofUnion as _;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            match number {
                1i32 => {
                    <self::_pinternal::field_type::OptionalUnsizedField::<
                        ::std::string::String,
                        self::_pinternal::tags::String,
                        0usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.name,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                2i32 => {
                    <self::_pinternal::field_type::OptionalNumericalField::<
                        i32,
                        self::_pinternal::tags::Int32,
                        1usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.number,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                3i32 => {
                    <self::_pinternal::field_type::SingularHeapMessageField::<
                        self::_root::google::protobuf::EnumValueOptions,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.options,
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
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.name,
            &self._bitfield,
            1i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            1usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.number,
            &self._bitfield,
            2i32,
            out,
        )?;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::EnumValueOptions,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.options,
            &self._bitfield,
            3i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for EnumValueDescriptorProto {
    fn clone(&self) -> Self {
        Self {
            name: <self::_pinternal::field_type::OptionalUnsizedField::<
                ::std::string::String,
                self::_pinternal::tags::String,
                0usize,
            > as ::std::clone::Clone>::clone(&self.name),
            number: <self::_pinternal::field_type::OptionalNumericalField::<
                i32,
                self::_pinternal::tags::Int32,
                1usize,
            > as ::std::clone::Clone>::clone(&self.number),
            options: <self::_pinternal::field_type::SingularHeapMessageField::<
                self::_root::google::protobuf::EnumValueOptions,
            > as ::std::clone::Clone>::clone(&self.options),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
impl ::std::ops::Drop for EnumValueDescriptorProto {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for EnumValueDescriptorProto {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(EnumValueDescriptorProto))
            .field(stringify!(name), &self.name_opt())
            .field(stringify!(number), &self.number_opt())
            .field(stringify!(options), &self.options_opt())
            .finish()
    }
}
impl ::std::cmp::PartialEq for EnumValueDescriptorProto {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
        true && self.name_opt() == rhs.name_opt()
            && self.number_opt() == rhs.number_opt()
            && self.options_opt() == rhs.options_opt()
    }
}
#[derive(::std::default::Default)]
pub struct ServiceDescriptorProto {
    name: self::_pinternal::field_type::OptionalUnsizedField::<
        ::std::string::String,
        self::_pinternal::tags::String,
        0usize,
    >,
    method: self::_pinternal::field_type::RepeatedMessageField::<
        self::_root::google::protobuf::MethodDescriptorProto,
    >,
    options: self::_pinternal::field_type::SingularHeapMessageField::<
        self::_root::google::protobuf::ServiceOptions,
    >,
    _bitfield: self::_pinternal::bitvec::BitArray<1usize>,
}
impl ServiceDescriptorProto {
    pub fn name(&self) -> &str {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.name,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn name_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.name, &self._bitfield)
    }
    pub fn name_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.name,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_name(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.name, &self._bitfield)
            .is_some()
    }
    pub fn clear_name(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::clear(&mut self.name, &mut self._bitfield)
    }
    pub fn method(&self) -> &[self::_root::google::protobuf::MethodDescriptorProto] {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::MethodDescriptorProto,
        > as RepeatedFieldType>::get_field(&self.method, &self._bitfield)
    }
    pub fn method_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::MethodDescriptorProto> {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::MethodDescriptorProto,
        > as RepeatedFieldType>::get_field_mut(&mut self.method, &mut self._bitfield)
    }
    pub fn clear_method(&mut self) {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::MethodDescriptorProto,
        > as RepeatedFieldType>::clear(&mut self.method, &mut self._bitfield)
    }
    pub fn options(
        &self,
    ) -> ::std::option::Option::<&self::_root::google::protobuf::ServiceOptions> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::ServiceOptions,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.options,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn options_opt(
        &self,
    ) -> ::std::option::Option::<&self::_root::google::protobuf::ServiceOptions> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::ServiceOptions,
        > as NonRepeatedFieldType>::get_field_opt(&self.options, &self._bitfield)
    }
    pub fn options_mut(&mut self) -> &mut self::_root::google::protobuf::ServiceOptions {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::ServiceOptions,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.options,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_options(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::ServiceOptions,
        > as NonRepeatedFieldType>::get_field_opt(&self.options, &self._bitfield)
            .is_some()
    }
    pub fn clear_options(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::ServiceOptions,
        > as NonRepeatedFieldType>::clear(&mut self.options, &mut self._bitfield)
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
        use self::_pinternal::oneof_type::OneofUnion as _;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            match number {
                1i32 => {
                    <self::_pinternal::field_type::OptionalUnsizedField::<
                        ::std::string::String,
                        self::_pinternal::tags::String,
                        0usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.name,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                2i32 => {
                    <self::_pinternal::field_type::RepeatedMessageField::<
                        self::_root::google::protobuf::MethodDescriptorProto,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.method,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                3i32 => {
                    <self::_pinternal::field_type::SingularHeapMessageField::<
                        self::_root::google::protobuf::ServiceOptions,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.options,
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
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.name,
            &self._bitfield,
            1i32,
            out,
        )?;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::MethodDescriptorProto,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.method,
            &self._bitfield,
            2i32,
            out,
        )?;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::ServiceOptions,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.options,
            &self._bitfield,
            3i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for ServiceDescriptorProto {
    fn clone(&self) -> Self {
        Self {
            name: <self::_pinternal::field_type::OptionalUnsizedField::<
                ::std::string::String,
                self::_pinternal::tags::String,
                0usize,
            > as ::std::clone::Clone>::clone(&self.name),
            method: <self::_pinternal::field_type::RepeatedMessageField::<
                self::_root::google::protobuf::MethodDescriptorProto,
            > as ::std::clone::Clone>::clone(&self.method),
            options: <self::_pinternal::field_type::SingularHeapMessageField::<
                self::_root::google::protobuf::ServiceOptions,
            > as ::std::clone::Clone>::clone(&self.options),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
impl ::std::ops::Drop for ServiceDescriptorProto {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for ServiceDescriptorProto {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(ServiceDescriptorProto))
            .field(stringify!(name), &self.name_opt())
            .field(stringify!(method), &self.method())
            .field(stringify!(options), &self.options_opt())
            .finish()
    }
}
impl ::std::cmp::PartialEq for ServiceDescriptorProto {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
        true && self.name_opt() == rhs.name_opt() && self.method() == rhs.method()
            && self.options_opt() == rhs.options_opt()
    }
}
#[derive(::std::default::Default)]
pub struct MethodDescriptorProto {
    name: self::_pinternal::field_type::OptionalUnsizedField::<
        ::std::string::String,
        self::_pinternal::tags::String,
        0usize,
    >,
    input_type: self::_pinternal::field_type::OptionalUnsizedField::<
        ::std::string::String,
        self::_pinternal::tags::String,
        1usize,
    >,
    output_type: self::_pinternal::field_type::OptionalUnsizedField::<
        ::std::string::String,
        self::_pinternal::tags::String,
        2usize,
    >,
    options: self::_pinternal::field_type::SingularHeapMessageField::<
        self::_root::google::protobuf::MethodOptions,
    >,
    client_streaming: self::_pinternal::field_type::OptionalNumericalField::<
        bool,
        self::_pinternal::tags::Bool,
        3usize,
    >,
    server_streaming: self::_pinternal::field_type::OptionalNumericalField::<
        bool,
        self::_pinternal::tags::Bool,
        4usize,
    >,
    _bitfield: self::_pinternal::bitvec::BitArray<1usize>,
}
impl MethodDescriptorProto {
    pub fn name(&self) -> &str {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.name,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn name_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.name, &self._bitfield)
    }
    pub fn name_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.name,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_name(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.name, &self._bitfield)
            .is_some()
    }
    pub fn clear_name(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::clear(&mut self.name, &mut self._bitfield)
    }
    pub fn input_type(&self) -> &str {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            1usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.input_type,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn input_type_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.input_type, &self._bitfield)
    }
    pub fn input_type_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            1usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.input_type,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_input_type(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.input_type, &self._bitfield)
            .is_some()
    }
    pub fn clear_input_type(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            1usize,
        > as NonRepeatedFieldType>::clear(&mut self.input_type, &mut self._bitfield)
    }
    pub fn output_type(&self) -> &str {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            2usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.output_type,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn output_type_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            2usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.output_type, &self._bitfield)
    }
    pub fn output_type_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            2usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.output_type,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_output_type(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            2usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.output_type, &self._bitfield)
            .is_some()
    }
    pub fn clear_output_type(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            2usize,
        > as NonRepeatedFieldType>::clear(&mut self.output_type, &mut self._bitfield)
    }
    pub fn options(
        &self,
    ) -> ::std::option::Option::<&self::_root::google::protobuf::MethodOptions> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::MethodOptions,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.options,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn options_opt(
        &self,
    ) -> ::std::option::Option::<&self::_root::google::protobuf::MethodOptions> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::MethodOptions,
        > as NonRepeatedFieldType>::get_field_opt(&self.options, &self._bitfield)
    }
    pub fn options_mut(&mut self) -> &mut self::_root::google::protobuf::MethodOptions {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::MethodOptions,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.options,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_options(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::MethodOptions,
        > as NonRepeatedFieldType>::get_field_opt(&self.options, &self._bitfield)
            .is_some()
    }
    pub fn clear_options(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::MethodOptions,
        > as NonRepeatedFieldType>::clear(&mut self.options, &mut self._bitfield)
    }
    pub fn client_streaming(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            3usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.client_streaming,
            &self._bitfield,
            || false,
        )
    }
    pub fn client_streaming_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            3usize,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.client_streaming,
            &self._bitfield,
        )
    }
    pub fn client_streaming_mut(&mut self) -> &mut bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            3usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.client_streaming,
            &mut self._bitfield,
            || false,
        )
    }
    pub fn has_client_streaming(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            3usize,
        > as NonRepeatedFieldType>::get_field_opt(
                &self.client_streaming,
                &self._bitfield,
            )
            .is_some()
    }
    pub fn clear_client_streaming(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            3usize,
        > as NonRepeatedFieldType>::clear(
            &mut self.client_streaming,
            &mut self._bitfield,
        )
    }
    pub fn server_streaming(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            4usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.server_streaming,
            &self._bitfield,
            || false,
        )
    }
    pub fn server_streaming_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            4usize,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.server_streaming,
            &self._bitfield,
        )
    }
    pub fn server_streaming_mut(&mut self) -> &mut bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            4usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.server_streaming,
            &mut self._bitfield,
            || false,
        )
    }
    pub fn has_server_streaming(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            4usize,
        > as NonRepeatedFieldType>::get_field_opt(
                &self.server_streaming,
                &self._bitfield,
            )
            .is_some()
    }
    pub fn clear_server_streaming(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            4usize,
        > as NonRepeatedFieldType>::clear(
            &mut self.server_streaming,
            &mut self._bitfield,
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
        use self::_pinternal::oneof_type::OneofUnion as _;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            match number {
                1i32 => {
                    <self::_pinternal::field_type::OptionalUnsizedField::<
                        ::std::string::String,
                        self::_pinternal::tags::String,
                        0usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.name,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                2i32 => {
                    <self::_pinternal::field_type::OptionalUnsizedField::<
                        ::std::string::String,
                        self::_pinternal::tags::String,
                        1usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.input_type,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                3i32 => {
                    <self::_pinternal::field_type::OptionalUnsizedField::<
                        ::std::string::String,
                        self::_pinternal::tags::String,
                        2usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.output_type,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                4i32 => {
                    <self::_pinternal::field_type::SingularHeapMessageField::<
                        self::_root::google::protobuf::MethodOptions,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.options,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                5i32 => {
                    <self::_pinternal::field_type::OptionalNumericalField::<
                        bool,
                        self::_pinternal::tags::Bool,
                        3usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.client_streaming,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                6i32 => {
                    <self::_pinternal::field_type::OptionalNumericalField::<
                        bool,
                        self::_pinternal::tags::Bool,
                        4usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.server_streaming,
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
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.name,
            &self._bitfield,
            1i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            1usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.input_type,
            &self._bitfield,
            2i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            2usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.output_type,
            &self._bitfield,
            3i32,
            out,
        )?;
        <self::_pinternal::field_type::SingularHeapMessageField::<
            self::_root::google::protobuf::MethodOptions,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.options,
            &self._bitfield,
            4i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            3usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.client_streaming,
            &self._bitfield,
            5i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            4usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.server_streaming,
            &self._bitfield,
            6i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for MethodDescriptorProto {
    fn clone(&self) -> Self {
        Self {
            name: <self::_pinternal::field_type::OptionalUnsizedField::<
                ::std::string::String,
                self::_pinternal::tags::String,
                0usize,
            > as ::std::clone::Clone>::clone(&self.name),
            input_type: <self::_pinternal::field_type::OptionalUnsizedField::<
                ::std::string::String,
                self::_pinternal::tags::String,
                1usize,
            > as ::std::clone::Clone>::clone(&self.input_type),
            output_type: <self::_pinternal::field_type::OptionalUnsizedField::<
                ::std::string::String,
                self::_pinternal::tags::String,
                2usize,
            > as ::std::clone::Clone>::clone(&self.output_type),
            options: <self::_pinternal::field_type::SingularHeapMessageField::<
                self::_root::google::protobuf::MethodOptions,
            > as ::std::clone::Clone>::clone(&self.options),
            client_streaming: <self::_pinternal::field_type::OptionalNumericalField::<
                bool,
                self::_pinternal::tags::Bool,
                3usize,
            > as ::std::clone::Clone>::clone(&self.client_streaming),
            server_streaming: <self::_pinternal::field_type::OptionalNumericalField::<
                bool,
                self::_pinternal::tags::Bool,
                4usize,
            > as ::std::clone::Clone>::clone(&self.server_streaming),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
impl ::std::ops::Drop for MethodDescriptorProto {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for MethodDescriptorProto {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(MethodDescriptorProto))
            .field(stringify!(name), &self.name_opt())
            .field(stringify!(input_type), &self.input_type_opt())
            .field(stringify!(output_type), &self.output_type_opt())
            .field(stringify!(options), &self.options_opt())
            .field(stringify!(client_streaming), &self.client_streaming_opt())
            .field(stringify!(server_streaming), &self.server_streaming_opt())
            .finish()
    }
}
impl ::std::cmp::PartialEq for MethodDescriptorProto {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
        true && self.name_opt() == rhs.name_opt()
            && self.input_type_opt() == rhs.input_type_opt()
            && self.output_type_opt() == rhs.output_type_opt()
            && self.options_opt() == rhs.options_opt()
            && self.client_streaming_opt() == rhs.client_streaming_opt()
            && self.server_streaming_opt() == rhs.server_streaming_opt()
    }
}
#[derive(::std::default::Default)]
pub struct FileOptions {
    java_package: self::_pinternal::field_type::OptionalUnsizedField::<
        ::std::string::String,
        self::_pinternal::tags::String,
        0usize,
    >,
    java_outer_classname: self::_pinternal::field_type::OptionalUnsizedField::<
        ::std::string::String,
        self::_pinternal::tags::String,
        1usize,
    >,
    java_multiple_files: self::_pinternal::field_type::OptionalNumericalField::<
        bool,
        self::_pinternal::tags::Bool,
        2usize,
    >,
    java_generate_equals_and_hash: self::_pinternal::field_type::OptionalNumericalField::<
        bool,
        self::_pinternal::tags::Bool,
        3usize,
    >,
    java_string_check_utf8: self::_pinternal::field_type::OptionalNumericalField::<
        bool,
        self::_pinternal::tags::Bool,
        4usize,
    >,
    optimize_for: self::_pinternal::field_type::OptionalNumericalField::<
        self::_root::google::protobuf::file_options::OptimizeMode,
        self::_pinternal::tags::Enum2::<
            self::_root::google::protobuf::file_options::OptimizeMode,
        >,
        5usize,
    >,
    go_package: self::_pinternal::field_type::OptionalUnsizedField::<
        ::std::string::String,
        self::_pinternal::tags::String,
        6usize,
    >,
    cc_generic_services: self::_pinternal::field_type::OptionalNumericalField::<
        bool,
        self::_pinternal::tags::Bool,
        7usize,
    >,
    java_generic_services: self::_pinternal::field_type::OptionalNumericalField::<
        bool,
        self::_pinternal::tags::Bool,
        8usize,
    >,
    py_generic_services: self::_pinternal::field_type::OptionalNumericalField::<
        bool,
        self::_pinternal::tags::Bool,
        9usize,
    >,
    php_generic_services: self::_pinternal::field_type::OptionalNumericalField::<
        bool,
        self::_pinternal::tags::Bool,
        10usize,
    >,
    deprecated: self::_pinternal::field_type::OptionalNumericalField::<
        bool,
        self::_pinternal::tags::Bool,
        11usize,
    >,
    cc_enable_arenas: self::_pinternal::field_type::OptionalNumericalField::<
        bool,
        self::_pinternal::tags::Bool,
        12usize,
    >,
    objc_class_prefix: self::_pinternal::field_type::OptionalUnsizedField::<
        ::std::string::String,
        self::_pinternal::tags::String,
        13usize,
    >,
    csharp_namespace: self::_pinternal::field_type::OptionalUnsizedField::<
        ::std::string::String,
        self::_pinternal::tags::String,
        14usize,
    >,
    swift_prefix: self::_pinternal::field_type::OptionalUnsizedField::<
        ::std::string::String,
        self::_pinternal::tags::String,
        15usize,
    >,
    php_class_prefix: self::_pinternal::field_type::OptionalUnsizedField::<
        ::std::string::String,
        self::_pinternal::tags::String,
        16usize,
    >,
    php_namespace: self::_pinternal::field_type::OptionalUnsizedField::<
        ::std::string::String,
        self::_pinternal::tags::String,
        17usize,
    >,
    php_metadata_namespace: self::_pinternal::field_type::OptionalUnsizedField::<
        ::std::string::String,
        self::_pinternal::tags::String,
        18usize,
    >,
    ruby_package: self::_pinternal::field_type::OptionalUnsizedField::<
        ::std::string::String,
        self::_pinternal::tags::String,
        19usize,
    >,
    uninterpreted_option: self::_pinternal::field_type::RepeatedMessageField::<
        self::_root::google::protobuf::UninterpretedOption,
    >,
    _bitfield: self::_pinternal::bitvec::BitArray<1usize>,
}
impl FileOptions {
    pub fn java_package(&self) -> &str {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.java_package,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn java_package_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.java_package, &self._bitfield)
    }
    pub fn java_package_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.java_package,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_java_package(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.java_package, &self._bitfield)
            .is_some()
    }
    pub fn clear_java_package(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::clear(&mut self.java_package, &mut self._bitfield)
    }
    pub fn java_outer_classname(&self) -> &str {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            1usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.java_outer_classname,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn java_outer_classname_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.java_outer_classname,
            &self._bitfield,
        )
    }
    pub fn java_outer_classname_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            1usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.java_outer_classname,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_java_outer_classname(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(
                &self.java_outer_classname,
                &self._bitfield,
            )
            .is_some()
    }
    pub fn clear_java_outer_classname(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            1usize,
        > as NonRepeatedFieldType>::clear(
            &mut self.java_outer_classname,
            &mut self._bitfield,
        )
    }
    pub fn java_multiple_files(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            2usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.java_multiple_files,
            &self._bitfield,
            || false,
        )
    }
    pub fn java_multiple_files_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            2usize,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.java_multiple_files,
            &self._bitfield,
        )
    }
    pub fn java_multiple_files_mut(&mut self) -> &mut bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            2usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.java_multiple_files,
            &mut self._bitfield,
            || false,
        )
    }
    pub fn has_java_multiple_files(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            2usize,
        > as NonRepeatedFieldType>::get_field_opt(
                &self.java_multiple_files,
                &self._bitfield,
            )
            .is_some()
    }
    pub fn clear_java_multiple_files(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            2usize,
        > as NonRepeatedFieldType>::clear(
            &mut self.java_multiple_files,
            &mut self._bitfield,
        )
    }
    pub fn java_generate_equals_and_hash(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            3usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.java_generate_equals_and_hash,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn java_generate_equals_and_hash_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            3usize,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.java_generate_equals_and_hash,
            &self._bitfield,
        )
    }
    pub fn java_generate_equals_and_hash_mut(&mut self) -> &mut bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            3usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.java_generate_equals_and_hash,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_java_generate_equals_and_hash(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            3usize,
        > as NonRepeatedFieldType>::get_field_opt(
                &self.java_generate_equals_and_hash,
                &self._bitfield,
            )
            .is_some()
    }
    pub fn clear_java_generate_equals_and_hash(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            3usize,
        > as NonRepeatedFieldType>::clear(
            &mut self.java_generate_equals_and_hash,
            &mut self._bitfield,
        )
    }
    pub fn java_string_check_utf8(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            4usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.java_string_check_utf8,
            &self._bitfield,
            || false,
        )
    }
    pub fn java_string_check_utf8_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            4usize,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.java_string_check_utf8,
            &self._bitfield,
        )
    }
    pub fn java_string_check_utf8_mut(&mut self) -> &mut bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            4usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.java_string_check_utf8,
            &mut self._bitfield,
            || false,
        )
    }
    pub fn has_java_string_check_utf8(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            4usize,
        > as NonRepeatedFieldType>::get_field_opt(
                &self.java_string_check_utf8,
                &self._bitfield,
            )
            .is_some()
    }
    pub fn clear_java_string_check_utf8(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            4usize,
        > as NonRepeatedFieldType>::clear(
            &mut self.java_string_check_utf8,
            &mut self._bitfield,
        )
    }
    pub fn optimize_for(
        &self,
    ) -> self::_root::google::protobuf::file_options::OptimizeMode {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            self::_root::google::protobuf::file_options::OptimizeMode,
            self::_pinternal::tags::Enum2::<
                self::_root::google::protobuf::file_options::OptimizeMode,
            >,
            5usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.optimize_for,
            &self._bitfield,
            || self::_root::google::protobuf::file_options::OptimizeMode::Speed,
        )
    }
    pub fn optimize_for_opt(
        &self,
    ) -> ::std::option::Option::<
        self::_root::google::protobuf::file_options::OptimizeMode,
    > {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            self::_root::google::protobuf::file_options::OptimizeMode,
            self::_pinternal::tags::Enum2::<
                self::_root::google::protobuf::file_options::OptimizeMode,
            >,
            5usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.optimize_for, &self._bitfield)
    }
    pub fn optimize_for_mut(
        &mut self,
    ) -> &mut self::_root::google::protobuf::file_options::OptimizeMode {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            self::_root::google::protobuf::file_options::OptimizeMode,
            self::_pinternal::tags::Enum2::<
                self::_root::google::protobuf::file_options::OptimizeMode,
            >,
            5usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.optimize_for,
            &mut self._bitfield,
            || self::_root::google::protobuf::file_options::OptimizeMode::Speed,
        )
    }
    pub fn has_optimize_for(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            self::_root::google::protobuf::file_options::OptimizeMode,
            self::_pinternal::tags::Enum2::<
                self::_root::google::protobuf::file_options::OptimizeMode,
            >,
            5usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.optimize_for, &self._bitfield)
            .is_some()
    }
    pub fn clear_optimize_for(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            self::_root::google::protobuf::file_options::OptimizeMode,
            self::_pinternal::tags::Enum2::<
                self::_root::google::protobuf::file_options::OptimizeMode,
            >,
            5usize,
        > as NonRepeatedFieldType>::clear(&mut self.optimize_for, &mut self._bitfield)
    }
    pub fn go_package(&self) -> &str {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            6usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.go_package,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn go_package_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            6usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.go_package, &self._bitfield)
    }
    pub fn go_package_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            6usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.go_package,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_go_package(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            6usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.go_package, &self._bitfield)
            .is_some()
    }
    pub fn clear_go_package(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            6usize,
        > as NonRepeatedFieldType>::clear(&mut self.go_package, &mut self._bitfield)
    }
    pub fn cc_generic_services(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            7usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.cc_generic_services,
            &self._bitfield,
            || false,
        )
    }
    pub fn cc_generic_services_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            7usize,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.cc_generic_services,
            &self._bitfield,
        )
    }
    pub fn cc_generic_services_mut(&mut self) -> &mut bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            7usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.cc_generic_services,
            &mut self._bitfield,
            || false,
        )
    }
    pub fn has_cc_generic_services(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            7usize,
        > as NonRepeatedFieldType>::get_field_opt(
                &self.cc_generic_services,
                &self._bitfield,
            )
            .is_some()
    }
    pub fn clear_cc_generic_services(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            7usize,
        > as NonRepeatedFieldType>::clear(
            &mut self.cc_generic_services,
            &mut self._bitfield,
        )
    }
    pub fn java_generic_services(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            8usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.java_generic_services,
            &self._bitfield,
            || false,
        )
    }
    pub fn java_generic_services_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            8usize,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.java_generic_services,
            &self._bitfield,
        )
    }
    pub fn java_generic_services_mut(&mut self) -> &mut bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            8usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.java_generic_services,
            &mut self._bitfield,
            || false,
        )
    }
    pub fn has_java_generic_services(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            8usize,
        > as NonRepeatedFieldType>::get_field_opt(
                &self.java_generic_services,
                &self._bitfield,
            )
            .is_some()
    }
    pub fn clear_java_generic_services(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            8usize,
        > as NonRepeatedFieldType>::clear(
            &mut self.java_generic_services,
            &mut self._bitfield,
        )
    }
    pub fn py_generic_services(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            9usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.py_generic_services,
            &self._bitfield,
            || false,
        )
    }
    pub fn py_generic_services_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            9usize,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.py_generic_services,
            &self._bitfield,
        )
    }
    pub fn py_generic_services_mut(&mut self) -> &mut bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            9usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.py_generic_services,
            &mut self._bitfield,
            || false,
        )
    }
    pub fn has_py_generic_services(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            9usize,
        > as NonRepeatedFieldType>::get_field_opt(
                &self.py_generic_services,
                &self._bitfield,
            )
            .is_some()
    }
    pub fn clear_py_generic_services(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            9usize,
        > as NonRepeatedFieldType>::clear(
            &mut self.py_generic_services,
            &mut self._bitfield,
        )
    }
    pub fn php_generic_services(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            10usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.php_generic_services,
            &self._bitfield,
            || false,
        )
    }
    pub fn php_generic_services_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            10usize,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.php_generic_services,
            &self._bitfield,
        )
    }
    pub fn php_generic_services_mut(&mut self) -> &mut bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            10usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.php_generic_services,
            &mut self._bitfield,
            || false,
        )
    }
    pub fn has_php_generic_services(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            10usize,
        > as NonRepeatedFieldType>::get_field_opt(
                &self.php_generic_services,
                &self._bitfield,
            )
            .is_some()
    }
    pub fn clear_php_generic_services(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            10usize,
        > as NonRepeatedFieldType>::clear(
            &mut self.php_generic_services,
            &mut self._bitfield,
        )
    }
    pub fn deprecated(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            11usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.deprecated,
            &self._bitfield,
            || false,
        )
    }
    pub fn deprecated_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            11usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.deprecated, &self._bitfield)
    }
    pub fn deprecated_mut(&mut self) -> &mut bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            11usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.deprecated,
            &mut self._bitfield,
            || false,
        )
    }
    pub fn has_deprecated(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            11usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.deprecated, &self._bitfield)
            .is_some()
    }
    pub fn clear_deprecated(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            11usize,
        > as NonRepeatedFieldType>::clear(&mut self.deprecated, &mut self._bitfield)
    }
    pub fn cc_enable_arenas(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            12usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.cc_enable_arenas,
            &self._bitfield,
            || true,
        )
    }
    pub fn cc_enable_arenas_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            12usize,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.cc_enable_arenas,
            &self._bitfield,
        )
    }
    pub fn cc_enable_arenas_mut(&mut self) -> &mut bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            12usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.cc_enable_arenas,
            &mut self._bitfield,
            || true,
        )
    }
    pub fn has_cc_enable_arenas(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            12usize,
        > as NonRepeatedFieldType>::get_field_opt(
                &self.cc_enable_arenas,
                &self._bitfield,
            )
            .is_some()
    }
    pub fn clear_cc_enable_arenas(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            12usize,
        > as NonRepeatedFieldType>::clear(
            &mut self.cc_enable_arenas,
            &mut self._bitfield,
        )
    }
    pub fn objc_class_prefix(&self) -> &str {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            13usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.objc_class_prefix,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn objc_class_prefix_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            13usize,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.objc_class_prefix,
            &self._bitfield,
        )
    }
    pub fn objc_class_prefix_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            13usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.objc_class_prefix,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_objc_class_prefix(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            13usize,
        > as NonRepeatedFieldType>::get_field_opt(
                &self.objc_class_prefix,
                &self._bitfield,
            )
            .is_some()
    }
    pub fn clear_objc_class_prefix(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            13usize,
        > as NonRepeatedFieldType>::clear(
            &mut self.objc_class_prefix,
            &mut self._bitfield,
        )
    }
    pub fn csharp_namespace(&self) -> &str {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            14usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.csharp_namespace,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn csharp_namespace_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            14usize,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.csharp_namespace,
            &self._bitfield,
        )
    }
    pub fn csharp_namespace_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            14usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.csharp_namespace,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_csharp_namespace(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            14usize,
        > as NonRepeatedFieldType>::get_field_opt(
                &self.csharp_namespace,
                &self._bitfield,
            )
            .is_some()
    }
    pub fn clear_csharp_namespace(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            14usize,
        > as NonRepeatedFieldType>::clear(
            &mut self.csharp_namespace,
            &mut self._bitfield,
        )
    }
    pub fn swift_prefix(&self) -> &str {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            15usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.swift_prefix,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn swift_prefix_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            15usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.swift_prefix, &self._bitfield)
    }
    pub fn swift_prefix_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            15usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.swift_prefix,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_swift_prefix(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            15usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.swift_prefix, &self._bitfield)
            .is_some()
    }
    pub fn clear_swift_prefix(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            15usize,
        > as NonRepeatedFieldType>::clear(&mut self.swift_prefix, &mut self._bitfield)
    }
    pub fn php_class_prefix(&self) -> &str {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            16usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.php_class_prefix,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn php_class_prefix_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            16usize,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.php_class_prefix,
            &self._bitfield,
        )
    }
    pub fn php_class_prefix_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            16usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.php_class_prefix,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_php_class_prefix(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            16usize,
        > as NonRepeatedFieldType>::get_field_opt(
                &self.php_class_prefix,
                &self._bitfield,
            )
            .is_some()
    }
    pub fn clear_php_class_prefix(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            16usize,
        > as NonRepeatedFieldType>::clear(
            &mut self.php_class_prefix,
            &mut self._bitfield,
        )
    }
    pub fn php_namespace(&self) -> &str {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            17usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.php_namespace,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn php_namespace_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            17usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.php_namespace, &self._bitfield)
    }
    pub fn php_namespace_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            17usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.php_namespace,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_php_namespace(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            17usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.php_namespace, &self._bitfield)
            .is_some()
    }
    pub fn clear_php_namespace(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            17usize,
        > as NonRepeatedFieldType>::clear(&mut self.php_namespace, &mut self._bitfield)
    }
    pub fn php_metadata_namespace(&self) -> &str {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            18usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.php_metadata_namespace,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn php_metadata_namespace_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            18usize,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.php_metadata_namespace,
            &self._bitfield,
        )
    }
    pub fn php_metadata_namespace_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            18usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.php_metadata_namespace,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_php_metadata_namespace(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            18usize,
        > as NonRepeatedFieldType>::get_field_opt(
                &self.php_metadata_namespace,
                &self._bitfield,
            )
            .is_some()
    }
    pub fn clear_php_metadata_namespace(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            18usize,
        > as NonRepeatedFieldType>::clear(
            &mut self.php_metadata_namespace,
            &mut self._bitfield,
        )
    }
    pub fn ruby_package(&self) -> &str {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            19usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.ruby_package,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn ruby_package_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            19usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.ruby_package, &self._bitfield)
    }
    pub fn ruby_package_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            19usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.ruby_package,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_ruby_package(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            19usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.ruby_package, &self._bitfield)
            .is_some()
    }
    pub fn clear_ruby_package(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            19usize,
        > as NonRepeatedFieldType>::clear(&mut self.ruby_package, &mut self._bitfield)
    }
    pub fn uninterpreted_option(
        &self,
    ) -> &[self::_root::google::protobuf::UninterpretedOption] {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::get_field(&self.uninterpreted_option, &self._bitfield)
    }
    pub fn uninterpreted_option_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::UninterpretedOption> {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::get_field_mut(
            &mut self.uninterpreted_option,
            &mut self._bitfield,
        )
    }
    pub fn clear_uninterpreted_option(&mut self) {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::clear(
            &mut self.uninterpreted_option,
            &mut self._bitfield,
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
        use self::_pinternal::oneof_type::OneofUnion as _;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            match number {
                1i32 => {
                    <self::_pinternal::field_type::OptionalUnsizedField::<
                        ::std::string::String,
                        self::_pinternal::tags::String,
                        0usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.java_package,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                8i32 => {
                    <self::_pinternal::field_type::OptionalUnsizedField::<
                        ::std::string::String,
                        self::_pinternal::tags::String,
                        1usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.java_outer_classname,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                10i32 => {
                    <self::_pinternal::field_type::OptionalNumericalField::<
                        bool,
                        self::_pinternal::tags::Bool,
                        2usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.java_multiple_files,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                20i32 => {
                    <self::_pinternal::field_type::OptionalNumericalField::<
                        bool,
                        self::_pinternal::tags::Bool,
                        3usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.java_generate_equals_and_hash,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                27i32 => {
                    <self::_pinternal::field_type::OptionalNumericalField::<
                        bool,
                        self::_pinternal::tags::Bool,
                        4usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.java_string_check_utf8,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                9i32 => {
                    <self::_pinternal::field_type::OptionalNumericalField::<
                        self::_root::google::protobuf::file_options::OptimizeMode,
                        self::_pinternal::tags::Enum2::<
                            self::_root::google::protobuf::file_options::OptimizeMode,
                        >,
                        5usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.optimize_for,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                11i32 => {
                    <self::_pinternal::field_type::OptionalUnsizedField::<
                        ::std::string::String,
                        self::_pinternal::tags::String,
                        6usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.go_package,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                16i32 => {
                    <self::_pinternal::field_type::OptionalNumericalField::<
                        bool,
                        self::_pinternal::tags::Bool,
                        7usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.cc_generic_services,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                17i32 => {
                    <self::_pinternal::field_type::OptionalNumericalField::<
                        bool,
                        self::_pinternal::tags::Bool,
                        8usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.java_generic_services,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                18i32 => {
                    <self::_pinternal::field_type::OptionalNumericalField::<
                        bool,
                        self::_pinternal::tags::Bool,
                        9usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.py_generic_services,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                42i32 => {
                    <self::_pinternal::field_type::OptionalNumericalField::<
                        bool,
                        self::_pinternal::tags::Bool,
                        10usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.php_generic_services,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                23i32 => {
                    <self::_pinternal::field_type::OptionalNumericalField::<
                        bool,
                        self::_pinternal::tags::Bool,
                        11usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.deprecated,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                31i32 => {
                    <self::_pinternal::field_type::OptionalNumericalField::<
                        bool,
                        self::_pinternal::tags::Bool,
                        12usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.cc_enable_arenas,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                36i32 => {
                    <self::_pinternal::field_type::OptionalUnsizedField::<
                        ::std::string::String,
                        self::_pinternal::tags::String,
                        13usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.objc_class_prefix,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                37i32 => {
                    <self::_pinternal::field_type::OptionalUnsizedField::<
                        ::std::string::String,
                        self::_pinternal::tags::String,
                        14usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.csharp_namespace,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                39i32 => {
                    <self::_pinternal::field_type::OptionalUnsizedField::<
                        ::std::string::String,
                        self::_pinternal::tags::String,
                        15usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.swift_prefix,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                40i32 => {
                    <self::_pinternal::field_type::OptionalUnsizedField::<
                        ::std::string::String,
                        self::_pinternal::tags::String,
                        16usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.php_class_prefix,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                41i32 => {
                    <self::_pinternal::field_type::OptionalUnsizedField::<
                        ::std::string::String,
                        self::_pinternal::tags::String,
                        17usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.php_namespace,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                44i32 => {
                    <self::_pinternal::field_type::OptionalUnsizedField::<
                        ::std::string::String,
                        self::_pinternal::tags::String,
                        18usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.php_metadata_namespace,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                45i32 => {
                    <self::_pinternal::field_type::OptionalUnsizedField::<
                        ::std::string::String,
                        self::_pinternal::tags::String,
                        19usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.ruby_package,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                999i32 => {
                    <self::_pinternal::field_type::RepeatedMessageField::<
                        self::_root::google::protobuf::UninterpretedOption,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.uninterpreted_option,
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
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.java_package,
            &self._bitfield,
            1i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            1usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.java_outer_classname,
            &self._bitfield,
            8i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            2usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.java_multiple_files,
            &self._bitfield,
            10i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            3usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.java_generate_equals_and_hash,
            &self._bitfield,
            20i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            4usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.java_string_check_utf8,
            &self._bitfield,
            27i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalNumericalField::<
            self::_root::google::protobuf::file_options::OptimizeMode,
            self::_pinternal::tags::Enum2::<
                self::_root::google::protobuf::file_options::OptimizeMode,
            >,
            5usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.optimize_for,
            &self._bitfield,
            9i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            6usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.go_package,
            &self._bitfield,
            11i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            7usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.cc_generic_services,
            &self._bitfield,
            16i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            8usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.java_generic_services,
            &self._bitfield,
            17i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            9usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.py_generic_services,
            &self._bitfield,
            18i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            10usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.php_generic_services,
            &self._bitfield,
            42i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            11usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.deprecated,
            &self._bitfield,
            23i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            12usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.cc_enable_arenas,
            &self._bitfield,
            31i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            13usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.objc_class_prefix,
            &self._bitfield,
            36i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            14usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.csharp_namespace,
            &self._bitfield,
            37i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            15usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.swift_prefix,
            &self._bitfield,
            39i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            16usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.php_class_prefix,
            &self._bitfield,
            40i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            17usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.php_namespace,
            &self._bitfield,
            41i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            18usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.php_metadata_namespace,
            &self._bitfield,
            44i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            19usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.ruby_package,
            &self._bitfield,
            45i32,
            out,
        )?;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::UninterpretedOption,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.uninterpreted_option,
            &self._bitfield,
            999i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for FileOptions {
    fn clone(&self) -> Self {
        Self {
            java_package: <self::_pinternal::field_type::OptionalUnsizedField::<
                ::std::string::String,
                self::_pinternal::tags::String,
                0usize,
            > as ::std::clone::Clone>::clone(&self.java_package),
            java_outer_classname: <self::_pinternal::field_type::OptionalUnsizedField::<
                ::std::string::String,
                self::_pinternal::tags::String,
                1usize,
            > as ::std::clone::Clone>::clone(&self.java_outer_classname),
            java_multiple_files: <self::_pinternal::field_type::OptionalNumericalField::<
                bool,
                self::_pinternal::tags::Bool,
                2usize,
            > as ::std::clone::Clone>::clone(&self.java_multiple_files),
            java_generate_equals_and_hash: <self::_pinternal::field_type::OptionalNumericalField::<
                bool,
                self::_pinternal::tags::Bool,
                3usize,
            > as ::std::clone::Clone>::clone(&self.java_generate_equals_and_hash),
            java_string_check_utf8: <self::_pinternal::field_type::OptionalNumericalField::<
                bool,
                self::_pinternal::tags::Bool,
                4usize,
            > as ::std::clone::Clone>::clone(&self.java_string_check_utf8),
            optimize_for: <self::_pinternal::field_type::OptionalNumericalField::<
                self::_root::google::protobuf::file_options::OptimizeMode,
                self::_pinternal::tags::Enum2::<
                    self::_root::google::protobuf::file_options::OptimizeMode,
                >,
                5usize,
            > as ::std::clone::Clone>::clone(&self.optimize_for),
            go_package: <self::_pinternal::field_type::OptionalUnsizedField::<
                ::std::string::String,
                self::_pinternal::tags::String,
                6usize,
            > as ::std::clone::Clone>::clone(&self.go_package),
            cc_generic_services: <self::_pinternal::field_type::OptionalNumericalField::<
                bool,
                self::_pinternal::tags::Bool,
                7usize,
            > as ::std::clone::Clone>::clone(&self.cc_generic_services),
            java_generic_services: <self::_pinternal::field_type::OptionalNumericalField::<
                bool,
                self::_pinternal::tags::Bool,
                8usize,
            > as ::std::clone::Clone>::clone(&self.java_generic_services),
            py_generic_services: <self::_pinternal::field_type::OptionalNumericalField::<
                bool,
                self::_pinternal::tags::Bool,
                9usize,
            > as ::std::clone::Clone>::clone(&self.py_generic_services),
            php_generic_services: <self::_pinternal::field_type::OptionalNumericalField::<
                bool,
                self::_pinternal::tags::Bool,
                10usize,
            > as ::std::clone::Clone>::clone(&self.php_generic_services),
            deprecated: <self::_pinternal::field_type::OptionalNumericalField::<
                bool,
                self::_pinternal::tags::Bool,
                11usize,
            > as ::std::clone::Clone>::clone(&self.deprecated),
            cc_enable_arenas: <self::_pinternal::field_type::OptionalNumericalField::<
                bool,
                self::_pinternal::tags::Bool,
                12usize,
            > as ::std::clone::Clone>::clone(&self.cc_enable_arenas),
            objc_class_prefix: <self::_pinternal::field_type::OptionalUnsizedField::<
                ::std::string::String,
                self::_pinternal::tags::String,
                13usize,
            > as ::std::clone::Clone>::clone(&self.objc_class_prefix),
            csharp_namespace: <self::_pinternal::field_type::OptionalUnsizedField::<
                ::std::string::String,
                self::_pinternal::tags::String,
                14usize,
            > as ::std::clone::Clone>::clone(&self.csharp_namespace),
            swift_prefix: <self::_pinternal::field_type::OptionalUnsizedField::<
                ::std::string::String,
                self::_pinternal::tags::String,
                15usize,
            > as ::std::clone::Clone>::clone(&self.swift_prefix),
            php_class_prefix: <self::_pinternal::field_type::OptionalUnsizedField::<
                ::std::string::String,
                self::_pinternal::tags::String,
                16usize,
            > as ::std::clone::Clone>::clone(&self.php_class_prefix),
            php_namespace: <self::_pinternal::field_type::OptionalUnsizedField::<
                ::std::string::String,
                self::_pinternal::tags::String,
                17usize,
            > as ::std::clone::Clone>::clone(&self.php_namespace),
            php_metadata_namespace: <self::_pinternal::field_type::OptionalUnsizedField::<
                ::std::string::String,
                self::_pinternal::tags::String,
                18usize,
            > as ::std::clone::Clone>::clone(&self.php_metadata_namespace),
            ruby_package: <self::_pinternal::field_type::OptionalUnsizedField::<
                ::std::string::String,
                self::_pinternal::tags::String,
                19usize,
            > as ::std::clone::Clone>::clone(&self.ruby_package),
            uninterpreted_option: <self::_pinternal::field_type::RepeatedMessageField::<
                self::_root::google::protobuf::UninterpretedOption,
            > as ::std::clone::Clone>::clone(&self.uninterpreted_option),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
impl ::std::ops::Drop for FileOptions {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for FileOptions {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(FileOptions))
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
            .field(stringify!(uninterpreted_option), &self.uninterpreted_option())
            .finish()
    }
}
impl ::std::cmp::PartialEq for FileOptions {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
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
    }
}
#[derive(::std::default::Default)]
pub struct MessageOptions {
    message_set_wire_format: self::_pinternal::field_type::OptionalNumericalField::<
        bool,
        self::_pinternal::tags::Bool,
        0usize,
    >,
    no_standard_descriptor_accessor: self::_pinternal::field_type::OptionalNumericalField::<
        bool,
        self::_pinternal::tags::Bool,
        1usize,
    >,
    deprecated: self::_pinternal::field_type::OptionalNumericalField::<
        bool,
        self::_pinternal::tags::Bool,
        2usize,
    >,
    map_entry: self::_pinternal::field_type::OptionalNumericalField::<
        bool,
        self::_pinternal::tags::Bool,
        3usize,
    >,
    uninterpreted_option: self::_pinternal::field_type::RepeatedMessageField::<
        self::_root::google::protobuf::UninterpretedOption,
    >,
    _bitfield: self::_pinternal::bitvec::BitArray<1usize>,
}
impl MessageOptions {
    pub fn message_set_wire_format(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            0usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.message_set_wire_format,
            &self._bitfield,
            || false,
        )
    }
    pub fn message_set_wire_format_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.message_set_wire_format,
            &self._bitfield,
        )
    }
    pub fn message_set_wire_format_mut(&mut self) -> &mut bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            0usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.message_set_wire_format,
            &mut self._bitfield,
            || false,
        )
    }
    pub fn has_message_set_wire_format(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(
                &self.message_set_wire_format,
                &self._bitfield,
            )
            .is_some()
    }
    pub fn clear_message_set_wire_format(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            0usize,
        > as NonRepeatedFieldType>::clear(
            &mut self.message_set_wire_format,
            &mut self._bitfield,
        )
    }
    pub fn no_standard_descriptor_accessor(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            1usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.no_standard_descriptor_accessor,
            &self._bitfield,
            || false,
        )
    }
    pub fn no_standard_descriptor_accessor_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.no_standard_descriptor_accessor,
            &self._bitfield,
        )
    }
    pub fn no_standard_descriptor_accessor_mut(&mut self) -> &mut bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            1usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.no_standard_descriptor_accessor,
            &mut self._bitfield,
            || false,
        )
    }
    pub fn has_no_standard_descriptor_accessor(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(
                &self.no_standard_descriptor_accessor,
                &self._bitfield,
            )
            .is_some()
    }
    pub fn clear_no_standard_descriptor_accessor(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            1usize,
        > as NonRepeatedFieldType>::clear(
            &mut self.no_standard_descriptor_accessor,
            &mut self._bitfield,
        )
    }
    pub fn deprecated(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            2usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.deprecated,
            &self._bitfield,
            || false,
        )
    }
    pub fn deprecated_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            2usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.deprecated, &self._bitfield)
    }
    pub fn deprecated_mut(&mut self) -> &mut bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            2usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.deprecated,
            &mut self._bitfield,
            || false,
        )
    }
    pub fn has_deprecated(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            2usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.deprecated, &self._bitfield)
            .is_some()
    }
    pub fn clear_deprecated(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            2usize,
        > as NonRepeatedFieldType>::clear(&mut self.deprecated, &mut self._bitfield)
    }
    pub fn map_entry(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            3usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.map_entry,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn map_entry_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            3usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.map_entry, &self._bitfield)
    }
    pub fn map_entry_mut(&mut self) -> &mut bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            3usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.map_entry,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_map_entry(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            3usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.map_entry, &self._bitfield)
            .is_some()
    }
    pub fn clear_map_entry(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            3usize,
        > as NonRepeatedFieldType>::clear(&mut self.map_entry, &mut self._bitfield)
    }
    pub fn uninterpreted_option(
        &self,
    ) -> &[self::_root::google::protobuf::UninterpretedOption] {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::get_field(&self.uninterpreted_option, &self._bitfield)
    }
    pub fn uninterpreted_option_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::UninterpretedOption> {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::get_field_mut(
            &mut self.uninterpreted_option,
            &mut self._bitfield,
        )
    }
    pub fn clear_uninterpreted_option(&mut self) {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::clear(
            &mut self.uninterpreted_option,
            &mut self._bitfield,
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
        use self::_pinternal::oneof_type::OneofUnion as _;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            match number {
                1i32 => {
                    <self::_pinternal::field_type::OptionalNumericalField::<
                        bool,
                        self::_pinternal::tags::Bool,
                        0usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.message_set_wire_format,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                2i32 => {
                    <self::_pinternal::field_type::OptionalNumericalField::<
                        bool,
                        self::_pinternal::tags::Bool,
                        1usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.no_standard_descriptor_accessor,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                3i32 => {
                    <self::_pinternal::field_type::OptionalNumericalField::<
                        bool,
                        self::_pinternal::tags::Bool,
                        2usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.deprecated,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                7i32 => {
                    <self::_pinternal::field_type::OptionalNumericalField::<
                        bool,
                        self::_pinternal::tags::Bool,
                        3usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.map_entry,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                999i32 => {
                    <self::_pinternal::field_type::RepeatedMessageField::<
                        self::_root::google::protobuf::UninterpretedOption,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.uninterpreted_option,
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
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            0usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.message_set_wire_format,
            &self._bitfield,
            1i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            1usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.no_standard_descriptor_accessor,
            &self._bitfield,
            2i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            2usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.deprecated,
            &self._bitfield,
            3i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            3usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.map_entry,
            &self._bitfield,
            7i32,
            out,
        )?;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::UninterpretedOption,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.uninterpreted_option,
            &self._bitfield,
            999i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for MessageOptions {
    fn clone(&self) -> Self {
        Self {
            message_set_wire_format: <self::_pinternal::field_type::OptionalNumericalField::<
                bool,
                self::_pinternal::tags::Bool,
                0usize,
            > as ::std::clone::Clone>::clone(&self.message_set_wire_format),
            no_standard_descriptor_accessor: <self::_pinternal::field_type::OptionalNumericalField::<
                bool,
                self::_pinternal::tags::Bool,
                1usize,
            > as ::std::clone::Clone>::clone(&self.no_standard_descriptor_accessor),
            deprecated: <self::_pinternal::field_type::OptionalNumericalField::<
                bool,
                self::_pinternal::tags::Bool,
                2usize,
            > as ::std::clone::Clone>::clone(&self.deprecated),
            map_entry: <self::_pinternal::field_type::OptionalNumericalField::<
                bool,
                self::_pinternal::tags::Bool,
                3usize,
            > as ::std::clone::Clone>::clone(&self.map_entry),
            uninterpreted_option: <self::_pinternal::field_type::RepeatedMessageField::<
                self::_root::google::protobuf::UninterpretedOption,
            > as ::std::clone::Clone>::clone(&self.uninterpreted_option),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
impl ::std::ops::Drop for MessageOptions {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for MessageOptions {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(MessageOptions))
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
            .field(stringify!(uninterpreted_option), &self.uninterpreted_option())
            .finish()
    }
}
impl ::std::cmp::PartialEq for MessageOptions {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
        true && self.message_set_wire_format_opt() == rhs.message_set_wire_format_opt()
            && self.no_standard_descriptor_accessor_opt()
                == rhs.no_standard_descriptor_accessor_opt()
            && self.deprecated_opt() == rhs.deprecated_opt()
            && self.map_entry_opt() == rhs.map_entry_opt()
            && self.uninterpreted_option() == rhs.uninterpreted_option()
    }
}
#[derive(::std::default::Default)]
pub struct FieldOptions {
    ctype: self::_pinternal::field_type::OptionalNumericalField::<
        self::_root::google::protobuf::field_options::CType,
        self::_pinternal::tags::Enum2::<
            self::_root::google::protobuf::field_options::CType,
        >,
        0usize,
    >,
    packed: self::_pinternal::field_type::OptionalNumericalField::<
        bool,
        self::_pinternal::tags::Bool,
        1usize,
    >,
    jstype: self::_pinternal::field_type::OptionalNumericalField::<
        self::_root::google::protobuf::field_options::JSType,
        self::_pinternal::tags::Enum2::<
            self::_root::google::protobuf::field_options::JSType,
        >,
        2usize,
    >,
    lazy: self::_pinternal::field_type::OptionalNumericalField::<
        bool,
        self::_pinternal::tags::Bool,
        3usize,
    >,
    deprecated: self::_pinternal::field_type::OptionalNumericalField::<
        bool,
        self::_pinternal::tags::Bool,
        4usize,
    >,
    weak: self::_pinternal::field_type::OptionalNumericalField::<
        bool,
        self::_pinternal::tags::Bool,
        5usize,
    >,
    uninterpreted_option: self::_pinternal::field_type::RepeatedMessageField::<
        self::_root::google::protobuf::UninterpretedOption,
    >,
    _bitfield: self::_pinternal::bitvec::BitArray<1usize>,
}
impl FieldOptions {
    pub fn ctype(&self) -> self::_root::google::protobuf::field_options::CType {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            self::_root::google::protobuf::field_options::CType,
            self::_pinternal::tags::Enum2::<
                self::_root::google::protobuf::field_options::CType,
            >,
            0usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.ctype,
            &self._bitfield,
            || self::_root::google::protobuf::field_options::CType::String,
        )
    }
    pub fn ctype_opt(
        &self,
    ) -> ::std::option::Option::<self::_root::google::protobuf::field_options::CType> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            self::_root::google::protobuf::field_options::CType,
            self::_pinternal::tags::Enum2::<
                self::_root::google::protobuf::field_options::CType,
            >,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.ctype, &self._bitfield)
    }
    pub fn ctype_mut(
        &mut self,
    ) -> &mut self::_root::google::protobuf::field_options::CType {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            self::_root::google::protobuf::field_options::CType,
            self::_pinternal::tags::Enum2::<
                self::_root::google::protobuf::field_options::CType,
            >,
            0usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.ctype,
            &mut self._bitfield,
            || self::_root::google::protobuf::field_options::CType::String,
        )
    }
    pub fn has_ctype(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            self::_root::google::protobuf::field_options::CType,
            self::_pinternal::tags::Enum2::<
                self::_root::google::protobuf::field_options::CType,
            >,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.ctype, &self._bitfield)
            .is_some()
    }
    pub fn clear_ctype(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            self::_root::google::protobuf::field_options::CType,
            self::_pinternal::tags::Enum2::<
                self::_root::google::protobuf::field_options::CType,
            >,
            0usize,
        > as NonRepeatedFieldType>::clear(&mut self.ctype, &mut self._bitfield)
    }
    pub fn packed(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            1usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.packed,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn packed_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.packed, &self._bitfield)
    }
    pub fn packed_mut(&mut self) -> &mut bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            1usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.packed,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_packed(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.packed, &self._bitfield)
            .is_some()
    }
    pub fn clear_packed(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            1usize,
        > as NonRepeatedFieldType>::clear(&mut self.packed, &mut self._bitfield)
    }
    pub fn jstype(&self) -> self::_root::google::protobuf::field_options::JSType {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            self::_root::google::protobuf::field_options::JSType,
            self::_pinternal::tags::Enum2::<
                self::_root::google::protobuf::field_options::JSType,
            >,
            2usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.jstype,
            &self._bitfield,
            || self::_root::google::protobuf::field_options::JSType::JsNormal,
        )
    }
    pub fn jstype_opt(
        &self,
    ) -> ::std::option::Option::<self::_root::google::protobuf::field_options::JSType> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            self::_root::google::protobuf::field_options::JSType,
            self::_pinternal::tags::Enum2::<
                self::_root::google::protobuf::field_options::JSType,
            >,
            2usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.jstype, &self._bitfield)
    }
    pub fn jstype_mut(
        &mut self,
    ) -> &mut self::_root::google::protobuf::field_options::JSType {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            self::_root::google::protobuf::field_options::JSType,
            self::_pinternal::tags::Enum2::<
                self::_root::google::protobuf::field_options::JSType,
            >,
            2usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.jstype,
            &mut self._bitfield,
            || self::_root::google::protobuf::field_options::JSType::JsNormal,
        )
    }
    pub fn has_jstype(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            self::_root::google::protobuf::field_options::JSType,
            self::_pinternal::tags::Enum2::<
                self::_root::google::protobuf::field_options::JSType,
            >,
            2usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.jstype, &self._bitfield)
            .is_some()
    }
    pub fn clear_jstype(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            self::_root::google::protobuf::field_options::JSType,
            self::_pinternal::tags::Enum2::<
                self::_root::google::protobuf::field_options::JSType,
            >,
            2usize,
        > as NonRepeatedFieldType>::clear(&mut self.jstype, &mut self._bitfield)
    }
    pub fn lazy(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            3usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.lazy,
            &self._bitfield,
            || false,
        )
    }
    pub fn lazy_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            3usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.lazy, &self._bitfield)
    }
    pub fn lazy_mut(&mut self) -> &mut bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            3usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.lazy,
            &mut self._bitfield,
            || false,
        )
    }
    pub fn has_lazy(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            3usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.lazy, &self._bitfield)
            .is_some()
    }
    pub fn clear_lazy(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            3usize,
        > as NonRepeatedFieldType>::clear(&mut self.lazy, &mut self._bitfield)
    }
    pub fn deprecated(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            4usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.deprecated,
            &self._bitfield,
            || false,
        )
    }
    pub fn deprecated_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            4usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.deprecated, &self._bitfield)
    }
    pub fn deprecated_mut(&mut self) -> &mut bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            4usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.deprecated,
            &mut self._bitfield,
            || false,
        )
    }
    pub fn has_deprecated(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            4usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.deprecated, &self._bitfield)
            .is_some()
    }
    pub fn clear_deprecated(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            4usize,
        > as NonRepeatedFieldType>::clear(&mut self.deprecated, &mut self._bitfield)
    }
    pub fn weak(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            5usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.weak,
            &self._bitfield,
            || false,
        )
    }
    pub fn weak_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            5usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.weak, &self._bitfield)
    }
    pub fn weak_mut(&mut self) -> &mut bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            5usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.weak,
            &mut self._bitfield,
            || false,
        )
    }
    pub fn has_weak(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            5usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.weak, &self._bitfield)
            .is_some()
    }
    pub fn clear_weak(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            5usize,
        > as NonRepeatedFieldType>::clear(&mut self.weak, &mut self._bitfield)
    }
    pub fn uninterpreted_option(
        &self,
    ) -> &[self::_root::google::protobuf::UninterpretedOption] {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::get_field(&self.uninterpreted_option, &self._bitfield)
    }
    pub fn uninterpreted_option_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::UninterpretedOption> {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::get_field_mut(
            &mut self.uninterpreted_option,
            &mut self._bitfield,
        )
    }
    pub fn clear_uninterpreted_option(&mut self) {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::clear(
            &mut self.uninterpreted_option,
            &mut self._bitfield,
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
        use self::_pinternal::oneof_type::OneofUnion as _;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            match number {
                1i32 => {
                    <self::_pinternal::field_type::OptionalNumericalField::<
                        self::_root::google::protobuf::field_options::CType,
                        self::_pinternal::tags::Enum2::<
                            self::_root::google::protobuf::field_options::CType,
                        >,
                        0usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.ctype,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                2i32 => {
                    <self::_pinternal::field_type::OptionalNumericalField::<
                        bool,
                        self::_pinternal::tags::Bool,
                        1usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.packed,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                6i32 => {
                    <self::_pinternal::field_type::OptionalNumericalField::<
                        self::_root::google::protobuf::field_options::JSType,
                        self::_pinternal::tags::Enum2::<
                            self::_root::google::protobuf::field_options::JSType,
                        >,
                        2usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.jstype,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                5i32 => {
                    <self::_pinternal::field_type::OptionalNumericalField::<
                        bool,
                        self::_pinternal::tags::Bool,
                        3usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.lazy,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                3i32 => {
                    <self::_pinternal::field_type::OptionalNumericalField::<
                        bool,
                        self::_pinternal::tags::Bool,
                        4usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.deprecated,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                10i32 => {
                    <self::_pinternal::field_type::OptionalNumericalField::<
                        bool,
                        self::_pinternal::tags::Bool,
                        5usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.weak,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                999i32 => {
                    <self::_pinternal::field_type::RepeatedMessageField::<
                        self::_root::google::protobuf::UninterpretedOption,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.uninterpreted_option,
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
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
        <self::_pinternal::field_type::OptionalNumericalField::<
            self::_root::google::protobuf::field_options::CType,
            self::_pinternal::tags::Enum2::<
                self::_root::google::protobuf::field_options::CType,
            >,
            0usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.ctype,
            &self._bitfield,
            1i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            1usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.packed,
            &self._bitfield,
            2i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalNumericalField::<
            self::_root::google::protobuf::field_options::JSType,
            self::_pinternal::tags::Enum2::<
                self::_root::google::protobuf::field_options::JSType,
            >,
            2usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.jstype,
            &self._bitfield,
            6i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            3usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.lazy,
            &self._bitfield,
            5i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            4usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.deprecated,
            &self._bitfield,
            3i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            5usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.weak,
            &self._bitfield,
            10i32,
            out,
        )?;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::UninterpretedOption,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.uninterpreted_option,
            &self._bitfield,
            999i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for FieldOptions {
    fn clone(&self) -> Self {
        Self {
            ctype: <self::_pinternal::field_type::OptionalNumericalField::<
                self::_root::google::protobuf::field_options::CType,
                self::_pinternal::tags::Enum2::<
                    self::_root::google::protobuf::field_options::CType,
                >,
                0usize,
            > as ::std::clone::Clone>::clone(&self.ctype),
            packed: <self::_pinternal::field_type::OptionalNumericalField::<
                bool,
                self::_pinternal::tags::Bool,
                1usize,
            > as ::std::clone::Clone>::clone(&self.packed),
            jstype: <self::_pinternal::field_type::OptionalNumericalField::<
                self::_root::google::protobuf::field_options::JSType,
                self::_pinternal::tags::Enum2::<
                    self::_root::google::protobuf::field_options::JSType,
                >,
                2usize,
            > as ::std::clone::Clone>::clone(&self.jstype),
            lazy: <self::_pinternal::field_type::OptionalNumericalField::<
                bool,
                self::_pinternal::tags::Bool,
                3usize,
            > as ::std::clone::Clone>::clone(&self.lazy),
            deprecated: <self::_pinternal::field_type::OptionalNumericalField::<
                bool,
                self::_pinternal::tags::Bool,
                4usize,
            > as ::std::clone::Clone>::clone(&self.deprecated),
            weak: <self::_pinternal::field_type::OptionalNumericalField::<
                bool,
                self::_pinternal::tags::Bool,
                5usize,
            > as ::std::clone::Clone>::clone(&self.weak),
            uninterpreted_option: <self::_pinternal::field_type::RepeatedMessageField::<
                self::_root::google::protobuf::UninterpretedOption,
            > as ::std::clone::Clone>::clone(&self.uninterpreted_option),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
impl ::std::ops::Drop for FieldOptions {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for FieldOptions {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(FieldOptions))
            .field(stringify!(ctype), &self.ctype_opt())
            .field(stringify!(packed), &self.packed_opt())
            .field(stringify!(jstype), &self.jstype_opt())
            .field(stringify!(lazy), &self.lazy_opt())
            .field(stringify!(deprecated), &self.deprecated_opt())
            .field(stringify!(weak), &self.weak_opt())
            .field(stringify!(uninterpreted_option), &self.uninterpreted_option())
            .finish()
    }
}
impl ::std::cmp::PartialEq for FieldOptions {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
        true && self.ctype_opt() == rhs.ctype_opt()
            && self.packed_opt() == rhs.packed_opt()
            && self.jstype_opt() == rhs.jstype_opt() && self.lazy_opt() == rhs.lazy_opt()
            && self.deprecated_opt() == rhs.deprecated_opt()
            && self.weak_opt() == rhs.weak_opt()
            && self.uninterpreted_option() == rhs.uninterpreted_option()
    }
}
#[derive(::std::default::Default)]
pub struct OneofOptions {
    uninterpreted_option: self::_pinternal::field_type::RepeatedMessageField::<
        self::_root::google::protobuf::UninterpretedOption,
    >,
    _bitfield: self::_pinternal::bitvec::BitArray<0usize>,
}
impl OneofOptions {
    pub fn uninterpreted_option(
        &self,
    ) -> &[self::_root::google::protobuf::UninterpretedOption] {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::get_field(&self.uninterpreted_option, &self._bitfield)
    }
    pub fn uninterpreted_option_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::UninterpretedOption> {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::get_field_mut(
            &mut self.uninterpreted_option,
            &mut self._bitfield,
        )
    }
    pub fn clear_uninterpreted_option(&mut self) {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::clear(
            &mut self.uninterpreted_option,
            &mut self._bitfield,
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
        use self::_pinternal::oneof_type::OneofUnion as _;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            match number {
                999i32 => {
                    <self::_pinternal::field_type::RepeatedMessageField::<
                        self::_root::google::protobuf::UninterpretedOption,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.uninterpreted_option,
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
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::UninterpretedOption,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.uninterpreted_option,
            &self._bitfield,
            999i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for OneofOptions {
    fn clone(&self) -> Self {
        Self {
            uninterpreted_option: <self::_pinternal::field_type::RepeatedMessageField::<
                self::_root::google::protobuf::UninterpretedOption,
            > as ::std::clone::Clone>::clone(&self.uninterpreted_option),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
impl ::std::ops::Drop for OneofOptions {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for OneofOptions {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(OneofOptions))
            .field(stringify!(uninterpreted_option), &self.uninterpreted_option())
            .finish()
    }
}
impl ::std::cmp::PartialEq for OneofOptions {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
        true && self.uninterpreted_option() == rhs.uninterpreted_option()
    }
}
#[derive(::std::default::Default)]
pub struct EnumOptions {
    allow_alias: self::_pinternal::field_type::OptionalNumericalField::<
        bool,
        self::_pinternal::tags::Bool,
        0usize,
    >,
    deprecated: self::_pinternal::field_type::OptionalNumericalField::<
        bool,
        self::_pinternal::tags::Bool,
        1usize,
    >,
    uninterpreted_option: self::_pinternal::field_type::RepeatedMessageField::<
        self::_root::google::protobuf::UninterpretedOption,
    >,
    _bitfield: self::_pinternal::bitvec::BitArray<1usize>,
}
impl EnumOptions {
    pub fn allow_alias(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            0usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.allow_alias,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn allow_alias_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.allow_alias, &self._bitfield)
    }
    pub fn allow_alias_mut(&mut self) -> &mut bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            0usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.allow_alias,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_allow_alias(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.allow_alias, &self._bitfield)
            .is_some()
    }
    pub fn clear_allow_alias(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            0usize,
        > as NonRepeatedFieldType>::clear(&mut self.allow_alias, &mut self._bitfield)
    }
    pub fn deprecated(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            1usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.deprecated,
            &self._bitfield,
            || false,
        )
    }
    pub fn deprecated_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.deprecated, &self._bitfield)
    }
    pub fn deprecated_mut(&mut self) -> &mut bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            1usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.deprecated,
            &mut self._bitfield,
            || false,
        )
    }
    pub fn has_deprecated(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.deprecated, &self._bitfield)
            .is_some()
    }
    pub fn clear_deprecated(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            1usize,
        > as NonRepeatedFieldType>::clear(&mut self.deprecated, &mut self._bitfield)
    }
    pub fn uninterpreted_option(
        &self,
    ) -> &[self::_root::google::protobuf::UninterpretedOption] {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::get_field(&self.uninterpreted_option, &self._bitfield)
    }
    pub fn uninterpreted_option_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::UninterpretedOption> {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::get_field_mut(
            &mut self.uninterpreted_option,
            &mut self._bitfield,
        )
    }
    pub fn clear_uninterpreted_option(&mut self) {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::clear(
            &mut self.uninterpreted_option,
            &mut self._bitfield,
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
        use self::_pinternal::oneof_type::OneofUnion as _;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            match number {
                2i32 => {
                    <self::_pinternal::field_type::OptionalNumericalField::<
                        bool,
                        self::_pinternal::tags::Bool,
                        0usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.allow_alias,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                3i32 => {
                    <self::_pinternal::field_type::OptionalNumericalField::<
                        bool,
                        self::_pinternal::tags::Bool,
                        1usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.deprecated,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                999i32 => {
                    <self::_pinternal::field_type::RepeatedMessageField::<
                        self::_root::google::protobuf::UninterpretedOption,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.uninterpreted_option,
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
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            0usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.allow_alias,
            &self._bitfield,
            2i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            1usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.deprecated,
            &self._bitfield,
            3i32,
            out,
        )?;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::UninterpretedOption,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.uninterpreted_option,
            &self._bitfield,
            999i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for EnumOptions {
    fn clone(&self) -> Self {
        Self {
            allow_alias: <self::_pinternal::field_type::OptionalNumericalField::<
                bool,
                self::_pinternal::tags::Bool,
                0usize,
            > as ::std::clone::Clone>::clone(&self.allow_alias),
            deprecated: <self::_pinternal::field_type::OptionalNumericalField::<
                bool,
                self::_pinternal::tags::Bool,
                1usize,
            > as ::std::clone::Clone>::clone(&self.deprecated),
            uninterpreted_option: <self::_pinternal::field_type::RepeatedMessageField::<
                self::_root::google::protobuf::UninterpretedOption,
            > as ::std::clone::Clone>::clone(&self.uninterpreted_option),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
impl ::std::ops::Drop for EnumOptions {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for EnumOptions {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(EnumOptions))
            .field(stringify!(allow_alias), &self.allow_alias_opt())
            .field(stringify!(deprecated), &self.deprecated_opt())
            .field(stringify!(uninterpreted_option), &self.uninterpreted_option())
            .finish()
    }
}
impl ::std::cmp::PartialEq for EnumOptions {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
        true && self.allow_alias_opt() == rhs.allow_alias_opt()
            && self.deprecated_opt() == rhs.deprecated_opt()
            && self.uninterpreted_option() == rhs.uninterpreted_option()
    }
}
#[derive(::std::default::Default)]
pub struct EnumValueOptions {
    deprecated: self::_pinternal::field_type::OptionalNumericalField::<
        bool,
        self::_pinternal::tags::Bool,
        0usize,
    >,
    uninterpreted_option: self::_pinternal::field_type::RepeatedMessageField::<
        self::_root::google::protobuf::UninterpretedOption,
    >,
    _bitfield: self::_pinternal::bitvec::BitArray<1usize>,
}
impl EnumValueOptions {
    pub fn deprecated(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            0usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.deprecated,
            &self._bitfield,
            || false,
        )
    }
    pub fn deprecated_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.deprecated, &self._bitfield)
    }
    pub fn deprecated_mut(&mut self) -> &mut bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            0usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.deprecated,
            &mut self._bitfield,
            || false,
        )
    }
    pub fn has_deprecated(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.deprecated, &self._bitfield)
            .is_some()
    }
    pub fn clear_deprecated(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            0usize,
        > as NonRepeatedFieldType>::clear(&mut self.deprecated, &mut self._bitfield)
    }
    pub fn uninterpreted_option(
        &self,
    ) -> &[self::_root::google::protobuf::UninterpretedOption] {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::get_field(&self.uninterpreted_option, &self._bitfield)
    }
    pub fn uninterpreted_option_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::UninterpretedOption> {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::get_field_mut(
            &mut self.uninterpreted_option,
            &mut self._bitfield,
        )
    }
    pub fn clear_uninterpreted_option(&mut self) {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::clear(
            &mut self.uninterpreted_option,
            &mut self._bitfield,
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
        use self::_pinternal::oneof_type::OneofUnion as _;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            match number {
                1i32 => {
                    <self::_pinternal::field_type::OptionalNumericalField::<
                        bool,
                        self::_pinternal::tags::Bool,
                        0usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.deprecated,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                999i32 => {
                    <self::_pinternal::field_type::RepeatedMessageField::<
                        self::_root::google::protobuf::UninterpretedOption,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.uninterpreted_option,
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
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            0usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.deprecated,
            &self._bitfield,
            1i32,
            out,
        )?;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::UninterpretedOption,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.uninterpreted_option,
            &self._bitfield,
            999i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for EnumValueOptions {
    fn clone(&self) -> Self {
        Self {
            deprecated: <self::_pinternal::field_type::OptionalNumericalField::<
                bool,
                self::_pinternal::tags::Bool,
                0usize,
            > as ::std::clone::Clone>::clone(&self.deprecated),
            uninterpreted_option: <self::_pinternal::field_type::RepeatedMessageField::<
                self::_root::google::protobuf::UninterpretedOption,
            > as ::std::clone::Clone>::clone(&self.uninterpreted_option),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
impl ::std::ops::Drop for EnumValueOptions {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for EnumValueOptions {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(EnumValueOptions))
            .field(stringify!(deprecated), &self.deprecated_opt())
            .field(stringify!(uninterpreted_option), &self.uninterpreted_option())
            .finish()
    }
}
impl ::std::cmp::PartialEq for EnumValueOptions {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
        true && self.deprecated_opt() == rhs.deprecated_opt()
            && self.uninterpreted_option() == rhs.uninterpreted_option()
    }
}
#[derive(::std::default::Default)]
pub struct ServiceOptions {
    deprecated: self::_pinternal::field_type::OptionalNumericalField::<
        bool,
        self::_pinternal::tags::Bool,
        0usize,
    >,
    uninterpreted_option: self::_pinternal::field_type::RepeatedMessageField::<
        self::_root::google::protobuf::UninterpretedOption,
    >,
    _bitfield: self::_pinternal::bitvec::BitArray<1usize>,
}
impl ServiceOptions {
    pub fn deprecated(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            0usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.deprecated,
            &self._bitfield,
            || false,
        )
    }
    pub fn deprecated_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.deprecated, &self._bitfield)
    }
    pub fn deprecated_mut(&mut self) -> &mut bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            0usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.deprecated,
            &mut self._bitfield,
            || false,
        )
    }
    pub fn has_deprecated(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.deprecated, &self._bitfield)
            .is_some()
    }
    pub fn clear_deprecated(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            0usize,
        > as NonRepeatedFieldType>::clear(&mut self.deprecated, &mut self._bitfield)
    }
    pub fn uninterpreted_option(
        &self,
    ) -> &[self::_root::google::protobuf::UninterpretedOption] {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::get_field(&self.uninterpreted_option, &self._bitfield)
    }
    pub fn uninterpreted_option_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::UninterpretedOption> {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::get_field_mut(
            &mut self.uninterpreted_option,
            &mut self._bitfield,
        )
    }
    pub fn clear_uninterpreted_option(&mut self) {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::clear(
            &mut self.uninterpreted_option,
            &mut self._bitfield,
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
        use self::_pinternal::oneof_type::OneofUnion as _;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            match number {
                33i32 => {
                    <self::_pinternal::field_type::OptionalNumericalField::<
                        bool,
                        self::_pinternal::tags::Bool,
                        0usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.deprecated,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                999i32 => {
                    <self::_pinternal::field_type::RepeatedMessageField::<
                        self::_root::google::protobuf::UninterpretedOption,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.uninterpreted_option,
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
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            0usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.deprecated,
            &self._bitfield,
            33i32,
            out,
        )?;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::UninterpretedOption,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.uninterpreted_option,
            &self._bitfield,
            999i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for ServiceOptions {
    fn clone(&self) -> Self {
        Self {
            deprecated: <self::_pinternal::field_type::OptionalNumericalField::<
                bool,
                self::_pinternal::tags::Bool,
                0usize,
            > as ::std::clone::Clone>::clone(&self.deprecated),
            uninterpreted_option: <self::_pinternal::field_type::RepeatedMessageField::<
                self::_root::google::protobuf::UninterpretedOption,
            > as ::std::clone::Clone>::clone(&self.uninterpreted_option),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
impl ::std::ops::Drop for ServiceOptions {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for ServiceOptions {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(ServiceOptions))
            .field(stringify!(deprecated), &self.deprecated_opt())
            .field(stringify!(uninterpreted_option), &self.uninterpreted_option())
            .finish()
    }
}
impl ::std::cmp::PartialEq for ServiceOptions {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
        true && self.deprecated_opt() == rhs.deprecated_opt()
            && self.uninterpreted_option() == rhs.uninterpreted_option()
    }
}
#[derive(::std::default::Default)]
pub struct MethodOptions {
    deprecated: self::_pinternal::field_type::OptionalNumericalField::<
        bool,
        self::_pinternal::tags::Bool,
        0usize,
    >,
    idempotency_level: self::_pinternal::field_type::OptionalNumericalField::<
        self::_root::google::protobuf::method_options::IdempotencyLevel,
        self::_pinternal::tags::Enum2::<
            self::_root::google::protobuf::method_options::IdempotencyLevel,
        >,
        1usize,
    >,
    uninterpreted_option: self::_pinternal::field_type::RepeatedMessageField::<
        self::_root::google::protobuf::UninterpretedOption,
    >,
    _bitfield: self::_pinternal::bitvec::BitArray<1usize>,
}
impl MethodOptions {
    pub fn deprecated(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            0usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.deprecated,
            &self._bitfield,
            || false,
        )
    }
    pub fn deprecated_opt(&self) -> ::std::option::Option::<bool> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.deprecated, &self._bitfield)
    }
    pub fn deprecated_mut(&mut self) -> &mut bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            0usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.deprecated,
            &mut self._bitfield,
            || false,
        )
    }
    pub fn has_deprecated(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.deprecated, &self._bitfield)
            .is_some()
    }
    pub fn clear_deprecated(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            0usize,
        > as NonRepeatedFieldType>::clear(&mut self.deprecated, &mut self._bitfield)
    }
    pub fn idempotency_level(
        &self,
    ) -> self::_root::google::protobuf::method_options::IdempotencyLevel {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            self::_root::google::protobuf::method_options::IdempotencyLevel,
            self::_pinternal::tags::Enum2::<
                self::_root::google::protobuf::method_options::IdempotencyLevel,
            >,
            1usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.idempotency_level,
            &self._bitfield,
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
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            self::_root::google::protobuf::method_options::IdempotencyLevel,
            self::_pinternal::tags::Enum2::<
                self::_root::google::protobuf::method_options::IdempotencyLevel,
            >,
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.idempotency_level,
            &self._bitfield,
        )
    }
    pub fn idempotency_level_mut(
        &mut self,
    ) -> &mut self::_root::google::protobuf::method_options::IdempotencyLevel {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            self::_root::google::protobuf::method_options::IdempotencyLevel,
            self::_pinternal::tags::Enum2::<
                self::_root::google::protobuf::method_options::IdempotencyLevel,
            >,
            1usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.idempotency_level,
            &mut self._bitfield,
            || {
                self::_root::google::protobuf::method_options::IdempotencyLevel::IdempotencyUnknown
            },
        )
    }
    pub fn has_idempotency_level(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            self::_root::google::protobuf::method_options::IdempotencyLevel,
            self::_pinternal::tags::Enum2::<
                self::_root::google::protobuf::method_options::IdempotencyLevel,
            >,
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(
                &self.idempotency_level,
                &self._bitfield,
            )
            .is_some()
    }
    pub fn clear_idempotency_level(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            self::_root::google::protobuf::method_options::IdempotencyLevel,
            self::_pinternal::tags::Enum2::<
                self::_root::google::protobuf::method_options::IdempotencyLevel,
            >,
            1usize,
        > as NonRepeatedFieldType>::clear(
            &mut self.idempotency_level,
            &mut self._bitfield,
        )
    }
    pub fn uninterpreted_option(
        &self,
    ) -> &[self::_root::google::protobuf::UninterpretedOption] {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::get_field(&self.uninterpreted_option, &self._bitfield)
    }
    pub fn uninterpreted_option_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_root::google::protobuf::UninterpretedOption> {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::get_field_mut(
            &mut self.uninterpreted_option,
            &mut self._bitfield,
        )
    }
    pub fn clear_uninterpreted_option(&mut self) {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::clear(
            &mut self.uninterpreted_option,
            &mut self._bitfield,
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
        use self::_pinternal::oneof_type::OneofUnion as _;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            match number {
                33i32 => {
                    <self::_pinternal::field_type::OptionalNumericalField::<
                        bool,
                        self::_pinternal::tags::Bool,
                        0usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.deprecated,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                34i32 => {
                    <self::_pinternal::field_type::OptionalNumericalField::<
                        self::_root::google::protobuf::method_options::IdempotencyLevel,
                        self::_pinternal::tags::Enum2::<
                            self::_root::google::protobuf::method_options::IdempotencyLevel,
                        >,
                        1usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.idempotency_level,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                999i32 => {
                    <self::_pinternal::field_type::RepeatedMessageField::<
                        self::_root::google::protobuf::UninterpretedOption,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.uninterpreted_option,
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
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
        <self::_pinternal::field_type::OptionalNumericalField::<
            bool,
            self::_pinternal::tags::Bool,
            0usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.deprecated,
            &self._bitfield,
            33i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalNumericalField::<
            self::_root::google::protobuf::method_options::IdempotencyLevel,
            self::_pinternal::tags::Enum2::<
                self::_root::google::protobuf::method_options::IdempotencyLevel,
            >,
            1usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.idempotency_level,
            &self._bitfield,
            34i32,
            out,
        )?;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::UninterpretedOption,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.uninterpreted_option,
            &self._bitfield,
            999i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for MethodOptions {
    fn clone(&self) -> Self {
        Self {
            deprecated: <self::_pinternal::field_type::OptionalNumericalField::<
                bool,
                self::_pinternal::tags::Bool,
                0usize,
            > as ::std::clone::Clone>::clone(&self.deprecated),
            idempotency_level: <self::_pinternal::field_type::OptionalNumericalField::<
                self::_root::google::protobuf::method_options::IdempotencyLevel,
                self::_pinternal::tags::Enum2::<
                    self::_root::google::protobuf::method_options::IdempotencyLevel,
                >,
                1usize,
            > as ::std::clone::Clone>::clone(&self.idempotency_level),
            uninterpreted_option: <self::_pinternal::field_type::RepeatedMessageField::<
                self::_root::google::protobuf::UninterpretedOption,
            > as ::std::clone::Clone>::clone(&self.uninterpreted_option),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
impl ::std::ops::Drop for MethodOptions {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for MethodOptions {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(MethodOptions))
            .field(stringify!(deprecated), &self.deprecated_opt())
            .field(stringify!(idempotency_level), &self.idempotency_level_opt())
            .field(stringify!(uninterpreted_option), &self.uninterpreted_option())
            .finish()
    }
}
impl ::std::cmp::PartialEq for MethodOptions {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
        true && self.deprecated_opt() == rhs.deprecated_opt()
            && self.idempotency_level_opt() == rhs.idempotency_level_opt()
            && self.uninterpreted_option() == rhs.uninterpreted_option()
    }
}
#[derive(::std::default::Default)]
pub struct UninterpretedOption {
    name: self::_pinternal::field_type::RepeatedMessageField::<
        self::_root::google::protobuf::uninterpreted_option::NamePart,
    >,
    identifier_value: self::_pinternal::field_type::OptionalUnsizedField::<
        ::std::string::String,
        self::_pinternal::tags::String,
        0usize,
    >,
    positive_int_value: self::_pinternal::field_type::OptionalNumericalField::<
        u64,
        self::_pinternal::tags::UInt64,
        1usize,
    >,
    negative_int_value: self::_pinternal::field_type::OptionalNumericalField::<
        i64,
        self::_pinternal::tags::Int64,
        2usize,
    >,
    double_value: self::_pinternal::field_type::OptionalNumericalField::<
        f64,
        self::_pinternal::tags::Double,
        3usize,
    >,
    string_value: self::_pinternal::field_type::OptionalUnsizedField::<
        ::std::vec::Vec<u8>,
        self::_pinternal::tags::Bytes,
        4usize,
    >,
    aggregate_value: self::_pinternal::field_type::OptionalUnsizedField::<
        ::std::string::String,
        self::_pinternal::tags::String,
        5usize,
    >,
    _bitfield: self::_pinternal::bitvec::BitArray<1usize>,
}
impl UninterpretedOption {
    pub fn name(
        &self,
    ) -> &[self::_root::google::protobuf::uninterpreted_option::NamePart] {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::uninterpreted_option::NamePart,
        > as RepeatedFieldType>::get_field(&self.name, &self._bitfield)
    }
    pub fn name_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<
        self::_root::google::protobuf::uninterpreted_option::NamePart,
    > {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::uninterpreted_option::NamePart,
        > as RepeatedFieldType>::get_field_mut(&mut self.name, &mut self._bitfield)
    }
    pub fn clear_name(&mut self) {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::uninterpreted_option::NamePart,
        > as RepeatedFieldType>::clear(&mut self.name, &mut self._bitfield)
    }
    pub fn identifier_value(&self) -> &str {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.identifier_value,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn identifier_value_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.identifier_value,
            &self._bitfield,
        )
    }
    pub fn identifier_value_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.identifier_value,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_identifier_value(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(
                &self.identifier_value,
                &self._bitfield,
            )
            .is_some()
    }
    pub fn clear_identifier_value(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::clear(
            &mut self.identifier_value,
            &mut self._bitfield,
        )
    }
    pub fn positive_int_value(&self) -> u64 {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            u64,
            self::_pinternal::tags::UInt64,
            1usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.positive_int_value,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn positive_int_value_opt(&self) -> ::std::option::Option::<u64> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            u64,
            self::_pinternal::tags::UInt64,
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.positive_int_value,
            &self._bitfield,
        )
    }
    pub fn positive_int_value_mut(&mut self) -> &mut u64 {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            u64,
            self::_pinternal::tags::UInt64,
            1usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.positive_int_value,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_positive_int_value(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            u64,
            self::_pinternal::tags::UInt64,
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(
                &self.positive_int_value,
                &self._bitfield,
            )
            .is_some()
    }
    pub fn clear_positive_int_value(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            u64,
            self::_pinternal::tags::UInt64,
            1usize,
        > as NonRepeatedFieldType>::clear(
            &mut self.positive_int_value,
            &mut self._bitfield,
        )
    }
    pub fn negative_int_value(&self) -> i64 {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            i64,
            self::_pinternal::tags::Int64,
            2usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.negative_int_value,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn negative_int_value_opt(&self) -> ::std::option::Option::<i64> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            i64,
            self::_pinternal::tags::Int64,
            2usize,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.negative_int_value,
            &self._bitfield,
        )
    }
    pub fn negative_int_value_mut(&mut self) -> &mut i64 {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            i64,
            self::_pinternal::tags::Int64,
            2usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.negative_int_value,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_negative_int_value(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            i64,
            self::_pinternal::tags::Int64,
            2usize,
        > as NonRepeatedFieldType>::get_field_opt(
                &self.negative_int_value,
                &self._bitfield,
            )
            .is_some()
    }
    pub fn clear_negative_int_value(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            i64,
            self::_pinternal::tags::Int64,
            2usize,
        > as NonRepeatedFieldType>::clear(
            &mut self.negative_int_value,
            &mut self._bitfield,
        )
    }
    pub fn double_value(&self) -> f64 {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            f64,
            self::_pinternal::tags::Double,
            3usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.double_value,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn double_value_opt(&self) -> ::std::option::Option::<f64> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            f64,
            self::_pinternal::tags::Double,
            3usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.double_value, &self._bitfield)
    }
    pub fn double_value_mut(&mut self) -> &mut f64 {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            f64,
            self::_pinternal::tags::Double,
            3usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.double_value,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_double_value(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            f64,
            self::_pinternal::tags::Double,
            3usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.double_value, &self._bitfield)
            .is_some()
    }
    pub fn clear_double_value(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalNumericalField::<
            f64,
            self::_pinternal::tags::Double,
            3usize,
        > as NonRepeatedFieldType>::clear(&mut self.double_value, &mut self._bitfield)
    }
    pub fn string_value(&self) -> &[u8] {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::vec::Vec<u8>,
            self::_pinternal::tags::Bytes,
            4usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.string_value,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn string_value_opt(&self) -> ::std::option::Option::<&[u8]> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::vec::Vec<u8>,
            self::_pinternal::tags::Bytes,
            4usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.string_value, &self._bitfield)
    }
    pub fn string_value_mut(&mut self) -> &mut ::std::vec::Vec::<u8> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::vec::Vec<u8>,
            self::_pinternal::tags::Bytes,
            4usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.string_value,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_string_value(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::vec::Vec<u8>,
            self::_pinternal::tags::Bytes,
            4usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.string_value, &self._bitfield)
            .is_some()
    }
    pub fn clear_string_value(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::vec::Vec<u8>,
            self::_pinternal::tags::Bytes,
            4usize,
        > as NonRepeatedFieldType>::clear(&mut self.string_value, &mut self._bitfield)
    }
    pub fn aggregate_value(&self) -> &str {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            5usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.aggregate_value,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn aggregate_value_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            5usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.aggregate_value, &self._bitfield)
    }
    pub fn aggregate_value_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            5usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.aggregate_value,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_aggregate_value(&self) -> bool {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            5usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.aggregate_value, &self._bitfield)
            .is_some()
    }
    pub fn clear_aggregate_value(&mut self) {
        use self::_pinternal::field_type::NonRepeatedFieldType;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            5usize,
        > as NonRepeatedFieldType>::clear(&mut self.aggregate_value, &mut self._bitfield)
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
        use self::_pinternal::oneof_type::OneofUnion as _;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            match number {
                2i32 => {
                    <self::_pinternal::field_type::RepeatedMessageField::<
                        self::_root::google::protobuf::uninterpreted_option::NamePart,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.name,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                3i32 => {
                    <self::_pinternal::field_type::OptionalUnsizedField::<
                        ::std::string::String,
                        self::_pinternal::tags::String,
                        0usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.identifier_value,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                4i32 => {
                    <self::_pinternal::field_type::OptionalNumericalField::<
                        u64,
                        self::_pinternal::tags::UInt64,
                        1usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.positive_int_value,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                5i32 => {
                    <self::_pinternal::field_type::OptionalNumericalField::<
                        i64,
                        self::_pinternal::tags::Int64,
                        2usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.negative_int_value,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                6i32 => {
                    <self::_pinternal::field_type::OptionalNumericalField::<
                        f64,
                        self::_pinternal::tags::Double,
                        3usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.double_value,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                7i32 => {
                    <self::_pinternal::field_type::OptionalUnsizedField::<
                        ::std::vec::Vec<u8>,
                        self::_pinternal::tags::Bytes,
                        4usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.string_value,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                8i32 => {
                    <self::_pinternal::field_type::OptionalUnsizedField::<
                        ::std::string::String,
                        self::_pinternal::tags::String,
                        5usize,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.aggregate_value,
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
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::uninterpreted_option::NamePart,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.name,
            &self._bitfield,
            2i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.identifier_value,
            &self._bitfield,
            3i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalNumericalField::<
            u64,
            self::_pinternal::tags::UInt64,
            1usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.positive_int_value,
            &self._bitfield,
            4i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalNumericalField::<
            i64,
            self::_pinternal::tags::Int64,
            2usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.negative_int_value,
            &self._bitfield,
            5i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalNumericalField::<
            f64,
            self::_pinternal::tags::Double,
            3usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.double_value,
            &self._bitfield,
            6i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::vec::Vec<u8>,
            self::_pinternal::tags::Bytes,
            4usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.string_value,
            &self._bitfield,
            7i32,
            out,
        )?;
        <self::_pinternal::field_type::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            5usize,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.aggregate_value,
            &self._bitfield,
            8i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for UninterpretedOption {
    fn clone(&self) -> Self {
        Self {
            name: <self::_pinternal::field_type::RepeatedMessageField::<
                self::_root::google::protobuf::uninterpreted_option::NamePart,
            > as ::std::clone::Clone>::clone(&self.name),
            identifier_value: <self::_pinternal::field_type::OptionalUnsizedField::<
                ::std::string::String,
                self::_pinternal::tags::String,
                0usize,
            > as ::std::clone::Clone>::clone(&self.identifier_value),
            positive_int_value: <self::_pinternal::field_type::OptionalNumericalField::<
                u64,
                self::_pinternal::tags::UInt64,
                1usize,
            > as ::std::clone::Clone>::clone(&self.positive_int_value),
            negative_int_value: <self::_pinternal::field_type::OptionalNumericalField::<
                i64,
                self::_pinternal::tags::Int64,
                2usize,
            > as ::std::clone::Clone>::clone(&self.negative_int_value),
            double_value: <self::_pinternal::field_type::OptionalNumericalField::<
                f64,
                self::_pinternal::tags::Double,
                3usize,
            > as ::std::clone::Clone>::clone(&self.double_value),
            string_value: <self::_pinternal::field_type::OptionalUnsizedField::<
                ::std::vec::Vec<u8>,
                self::_pinternal::tags::Bytes,
                4usize,
            > as ::std::clone::Clone>::clone(&self.string_value),
            aggregate_value: <self::_pinternal::field_type::OptionalUnsizedField::<
                ::std::string::String,
                self::_pinternal::tags::String,
                5usize,
            > as ::std::clone::Clone>::clone(&self.aggregate_value),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
impl ::std::ops::Drop for UninterpretedOption {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for UninterpretedOption {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(UninterpretedOption))
            .field(stringify!(name), &self.name())
            .field(stringify!(identifier_value), &self.identifier_value_opt())
            .field(stringify!(positive_int_value), &self.positive_int_value_opt())
            .field(stringify!(negative_int_value), &self.negative_int_value_opt())
            .field(stringify!(double_value), &self.double_value_opt())
            .field(stringify!(string_value), &self.string_value_opt())
            .field(stringify!(aggregate_value), &self.aggregate_value_opt())
            .finish()
    }
}
impl ::std::cmp::PartialEq for UninterpretedOption {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
        true && self.name() == rhs.name()
            && self.identifier_value_opt() == rhs.identifier_value_opt()
            && self.positive_int_value_opt() == rhs.positive_int_value_opt()
            && self.negative_int_value_opt() == rhs.negative_int_value_opt()
            && self.double_value_opt() == rhs.double_value_opt()
            && self.string_value_opt() == rhs.string_value_opt()
            && self.aggregate_value_opt() == rhs.aggregate_value_opt()
    }
}
#[derive(::std::default::Default)]
pub struct SourceCodeInfo {
    location: self::_pinternal::field_type::RepeatedMessageField::<
        self::_root::google::protobuf::source_code_info::Location,
    >,
    _bitfield: self::_pinternal::bitvec::BitArray<0usize>,
}
impl SourceCodeInfo {
    pub fn location(
        &self,
    ) -> &[self::_root::google::protobuf::source_code_info::Location] {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::source_code_info::Location,
        > as RepeatedFieldType>::get_field(&self.location, &self._bitfield)
    }
    pub fn location_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<
        self::_root::google::protobuf::source_code_info::Location,
    > {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::source_code_info::Location,
        > as RepeatedFieldType>::get_field_mut(&mut self.location, &mut self._bitfield)
    }
    pub fn clear_location(&mut self) {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::source_code_info::Location,
        > as RepeatedFieldType>::clear(&mut self.location, &mut self._bitfield)
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
        use self::_pinternal::oneof_type::OneofUnion as _;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            match number {
                1i32 => {
                    <self::_pinternal::field_type::RepeatedMessageField::<
                        self::_root::google::protobuf::source_code_info::Location,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.location,
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
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::source_code_info::Location,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.location,
            &self._bitfield,
            1i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for SourceCodeInfo {
    fn clone(&self) -> Self {
        Self {
            location: <self::_pinternal::field_type::RepeatedMessageField::<
                self::_root::google::protobuf::source_code_info::Location,
            > as ::std::clone::Clone>::clone(&self.location),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
impl ::std::ops::Drop for SourceCodeInfo {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for SourceCodeInfo {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(SourceCodeInfo))
            .field(stringify!(location), &self.location())
            .finish()
    }
}
impl ::std::cmp::PartialEq for SourceCodeInfo {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
        true && self.location() == rhs.location()
    }
}
#[derive(::std::default::Default)]
pub struct GeneratedCodeInfo {
    annotation: self::_pinternal::field_type::RepeatedMessageField::<
        self::_root::google::protobuf::generated_code_info::Annotation,
    >,
    _bitfield: self::_pinternal::bitvec::BitArray<0usize>,
}
impl GeneratedCodeInfo {
    pub fn annotation(
        &self,
    ) -> &[self::_root::google::protobuf::generated_code_info::Annotation] {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::generated_code_info::Annotation,
        > as RepeatedFieldType>::get_field(&self.annotation, &self._bitfield)
    }
    pub fn annotation_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<
        self::_root::google::protobuf::generated_code_info::Annotation,
    > {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::generated_code_info::Annotation,
        > as RepeatedFieldType>::get_field_mut(&mut self.annotation, &mut self._bitfield)
    }
    pub fn clear_annotation(&mut self) {
        use self::_pinternal::field_type::RepeatedFieldType;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::generated_code_info::Annotation,
        > as RepeatedFieldType>::clear(&mut self.annotation, &mut self._bitfield)
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
        use self::_pinternal::oneof_type::OneofUnion as _;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            match number {
                1i32 => {
                    <self::_pinternal::field_type::RepeatedMessageField::<
                        self::_root::google::protobuf::generated_code_info::Annotation,
                    > as self::_pinternal::field_type::FieldType>::deser_from_iter(
                        &mut self.annotation,
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
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
        <self::_pinternal::field_type::RepeatedMessageField::<
            self::_root::google::protobuf::generated_code_info::Annotation,
        > as self::_pinternal::field_type::FieldType>::ser_to_write(
            &self.annotation,
            &self._bitfield,
            1i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for GeneratedCodeInfo {
    fn clone(&self) -> Self {
        Self {
            annotation: <self::_pinternal::field_type::RepeatedMessageField::<
                self::_root::google::protobuf::generated_code_info::Annotation,
            > as ::std::clone::Clone>::clone(&self.annotation),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
impl ::std::ops::Drop for GeneratedCodeInfo {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for GeneratedCodeInfo {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(GeneratedCodeInfo))
            .field(stringify!(annotation), &self.annotation())
            .finish()
    }
}
impl ::std::cmp::PartialEq for GeneratedCodeInfo {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::oneof_type::OneofUnion as _;
        true && self.annotation() == rhs.annotation()
    }
}
