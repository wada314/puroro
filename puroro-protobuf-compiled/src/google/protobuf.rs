pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}
pub mod _puroro {
    pub use ::puroro::*;
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
    file: self::_puroro::internal::field_type::RepeatedMessageField::<
        self::_puroro_root::google::protobuf::FileDescriptorProto,
    >,
    _bitfield: self::_puroro::bitvec::BitArray<0usize>,
}
impl FileDescriptorSet {
    pub fn file(&self) -> &[self::_puroro_root::google::protobuf::FileDescriptorProto] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField::<
            self::_puroro_root::google::protobuf::FileDescriptorProto,
        > as RepeatedFieldType>::get_field(&self.file, &self._bitfield)
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
        #[allow(unused)]
        use ::std::result::Result::Ok;
        #[allow(unused)]
        use ::std::option::Option::Some;
        #[allow(unused)]
        use self::_puroro::internal::field_type::FieldType;
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        use self::_puroro::internal::ser::FieldData;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            todo!()
        }
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use ::std::result::Result::Ok;
        Ok(todo!())
    }
}
impl ::std::clone::Clone for FileDescriptorSet {
    fn clone(&self) -> Self {
        Self {
            file: <self::_puroro::internal::field_type::RepeatedMessageField::<
                self::_puroro_root::google::protobuf::FileDescriptorProto,
            > as ::std::clone::Clone>::clone(&self.file),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
#[derive(::std::default::Default)]
pub struct FileDescriptorProto {
    name: self::_puroro::internal::field_type::OptionalStringField::<0usize>,
    package: self::_puroro::internal::field_type::OptionalStringField::<1usize>,
    dependency: self::_puroro::internal::field_type::RepeatedStringField,
    public_dependency: self::_puroro::internal::field_type::RepeatedNumericalField::<
        i32,
        self::_puroro::tags::Int32,
    >,
    weak_dependency: self::_puroro::internal::field_type::RepeatedNumericalField::<
        i32,
        self::_puroro::tags::Int32,
    >,
    message_type: self::_puroro::internal::field_type::RepeatedMessageField::<
        self::_puroro_root::google::protobuf::DescriptorProto,
    >,
    enum_type: self::_puroro::internal::field_type::RepeatedMessageField::<
        self::_puroro_root::google::protobuf::EnumDescriptorProto,
    >,
    service: self::_puroro::internal::field_type::RepeatedMessageField::<
        self::_puroro_root::google::protobuf::ServiceDescriptorProto,
    >,
    extension: self::_puroro::internal::field_type::RepeatedMessageField::<
        self::_puroro_root::google::protobuf::FieldDescriptorProto,
    >,
    options: self::_puroro::internal::field_type::SingularHeapMessageField::<
        self::_puroro_root::google::protobuf::FileOptions,
    >,
    source_code_info: self::_puroro::internal::field_type::SingularHeapMessageField::<
        self::_puroro_root::google::protobuf::SourceCodeInfo,
    >,
    syntax: self::_puroro::internal::field_type::OptionalStringField::<2usize>,
    _bitfield: self::_puroro::bitvec::BitArray<1usize>,
}
impl FileDescriptorProto {
    pub fn name(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            0usize,
        > as NonRepeatedFieldType>::get_field(
            &self.name,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn package(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            1usize,
        > as NonRepeatedFieldType>::get_field(
            &self.package,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn dependency(&self) -> &[impl ::std::ops::Deref::<Target = str>] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedStringField as RepeatedFieldType>::get_field(
            &self.dependency,
            &self._bitfield,
        )
    }
    pub fn public_dependency(&self) -> &[i32] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            i32,
            self::_puroro::tags::Int32,
        > as RepeatedFieldType>::get_field(&self.public_dependency, &self._bitfield)
    }
    pub fn weak_dependency(&self) -> &[i32] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            i32,
            self::_puroro::tags::Int32,
        > as RepeatedFieldType>::get_field(&self.weak_dependency, &self._bitfield)
    }
    pub fn message_type(
        &self,
    ) -> &[self::_puroro_root::google::protobuf::DescriptorProto] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField::<
            self::_puroro_root::google::protobuf::DescriptorProto,
        > as RepeatedFieldType>::get_field(&self.message_type, &self._bitfield)
    }
    pub fn enum_type(
        &self,
    ) -> &[self::_puroro_root::google::protobuf::EnumDescriptorProto] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField::<
            self::_puroro_root::google::protobuf::EnumDescriptorProto,
        > as RepeatedFieldType>::get_field(&self.enum_type, &self._bitfield)
    }
    pub fn service(
        &self,
    ) -> &[self::_puroro_root::google::protobuf::ServiceDescriptorProto] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField::<
            self::_puroro_root::google::protobuf::ServiceDescriptorProto,
        > as RepeatedFieldType>::get_field(&self.service, &self._bitfield)
    }
    pub fn extension(
        &self,
    ) -> &[self::_puroro_root::google::protobuf::FieldDescriptorProto] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField::<
            self::_puroro_root::google::protobuf::FieldDescriptorProto,
        > as RepeatedFieldType>::get_field(&self.extension, &self._bitfield)
    }
    pub fn options(
        &self,
    ) -> ::std::option::Option::<&self::_puroro_root::google::protobuf::FileOptions> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField::<
            self::_puroro_root::google::protobuf::FileOptions,
        > as NonRepeatedFieldType>::get_field(
            &self.options,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn source_code_info(
        &self,
    ) -> ::std::option::Option::<&self::_puroro_root::google::protobuf::SourceCodeInfo> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField::<
            self::_puroro_root::google::protobuf::SourceCodeInfo,
        > as NonRepeatedFieldType>::get_field(
            &self.source_code_info,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn syntax(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            2usize,
        > as NonRepeatedFieldType>::get_field(
            &self.syntax,
            &self._bitfield,
            ::std::default::Default::default,
        )
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
        #[allow(unused)]
        use ::std::result::Result::Ok;
        #[allow(unused)]
        use ::std::option::Option::Some;
        #[allow(unused)]
        use self::_puroro::internal::field_type::FieldType;
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        use self::_puroro::internal::ser::FieldData;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            todo!()
        }
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use ::std::result::Result::Ok;
        Ok(todo!())
    }
}
impl ::std::clone::Clone for FileDescriptorProto {
    fn clone(&self) -> Self {
        Self {
            name: <self::_puroro::internal::field_type::OptionalStringField::<
                0usize,
            > as ::std::clone::Clone>::clone(&self.name),
            package: <self::_puroro::internal::field_type::OptionalStringField::<
                1usize,
            > as ::std::clone::Clone>::clone(&self.package),
            dependency: <self::_puroro::internal::field_type::RepeatedStringField as ::std::clone::Clone>::clone(
                &self.dependency,
            ),
            public_dependency: <self::_puroro::internal::field_type::RepeatedNumericalField::<
                i32,
                self::_puroro::tags::Int32,
            > as ::std::clone::Clone>::clone(&self.public_dependency),
            weak_dependency: <self::_puroro::internal::field_type::RepeatedNumericalField::<
                i32,
                self::_puroro::tags::Int32,
            > as ::std::clone::Clone>::clone(&self.weak_dependency),
            message_type: <self::_puroro::internal::field_type::RepeatedMessageField::<
                self::_puroro_root::google::protobuf::DescriptorProto,
            > as ::std::clone::Clone>::clone(&self.message_type),
            enum_type: <self::_puroro::internal::field_type::RepeatedMessageField::<
                self::_puroro_root::google::protobuf::EnumDescriptorProto,
            > as ::std::clone::Clone>::clone(&self.enum_type),
            service: <self::_puroro::internal::field_type::RepeatedMessageField::<
                self::_puroro_root::google::protobuf::ServiceDescriptorProto,
            > as ::std::clone::Clone>::clone(&self.service),
            extension: <self::_puroro::internal::field_type::RepeatedMessageField::<
                self::_puroro_root::google::protobuf::FieldDescriptorProto,
            > as ::std::clone::Clone>::clone(&self.extension),
            options: <self::_puroro::internal::field_type::SingularHeapMessageField::<
                self::_puroro_root::google::protobuf::FileOptions,
            > as ::std::clone::Clone>::clone(&self.options),
            source_code_info: <self::_puroro::internal::field_type::SingularHeapMessageField::<
                self::_puroro_root::google::protobuf::SourceCodeInfo,
            > as ::std::clone::Clone>::clone(&self.source_code_info),
            syntax: <self::_puroro::internal::field_type::OptionalStringField::<
                2usize,
            > as ::std::clone::Clone>::clone(&self.syntax),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
#[derive(::std::default::Default)]
pub struct DescriptorProto {
    name: self::_puroro::internal::field_type::OptionalStringField::<0usize>,
    field: self::_puroro::internal::field_type::RepeatedMessageField::<
        self::_puroro_root::google::protobuf::FieldDescriptorProto,
    >,
    extension: self::_puroro::internal::field_type::RepeatedMessageField::<
        self::_puroro_root::google::protobuf::FieldDescriptorProto,
    >,
    nested_type: self::_puroro::internal::field_type::RepeatedMessageField::<
        self::_puroro_root::google::protobuf::DescriptorProto,
    >,
    enum_type: self::_puroro::internal::field_type::RepeatedMessageField::<
        self::_puroro_root::google::protobuf::EnumDescriptorProto,
    >,
    extension_range: self::_puroro::internal::field_type::RepeatedMessageField::<
        self::_puroro_root::google::protobuf::descriptor_proto::ExtensionRange,
    >,
    oneof_decl: self::_puroro::internal::field_type::RepeatedMessageField::<
        self::_puroro_root::google::protobuf::OneofDescriptorProto,
    >,
    options: self::_puroro::internal::field_type::SingularHeapMessageField::<
        self::_puroro_root::google::protobuf::MessageOptions,
    >,
    reserved_range: self::_puroro::internal::field_type::RepeatedMessageField::<
        self::_puroro_root::google::protobuf::descriptor_proto::ReservedRange,
    >,
    reserved_name: self::_puroro::internal::field_type::RepeatedStringField,
    _bitfield: self::_puroro::bitvec::BitArray<1usize>,
}
impl DescriptorProto {
    pub fn name(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            0usize,
        > as NonRepeatedFieldType>::get_field(
            &self.name,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn field(
        &self,
    ) -> &[self::_puroro_root::google::protobuf::FieldDescriptorProto] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField::<
            self::_puroro_root::google::protobuf::FieldDescriptorProto,
        > as RepeatedFieldType>::get_field(&self.field, &self._bitfield)
    }
    pub fn extension(
        &self,
    ) -> &[self::_puroro_root::google::protobuf::FieldDescriptorProto] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField::<
            self::_puroro_root::google::protobuf::FieldDescriptorProto,
        > as RepeatedFieldType>::get_field(&self.extension, &self._bitfield)
    }
    pub fn nested_type(
        &self,
    ) -> &[self::_puroro_root::google::protobuf::DescriptorProto] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField::<
            self::_puroro_root::google::protobuf::DescriptorProto,
        > as RepeatedFieldType>::get_field(&self.nested_type, &self._bitfield)
    }
    pub fn enum_type(
        &self,
    ) -> &[self::_puroro_root::google::protobuf::EnumDescriptorProto] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField::<
            self::_puroro_root::google::protobuf::EnumDescriptorProto,
        > as RepeatedFieldType>::get_field(&self.enum_type, &self._bitfield)
    }
    pub fn extension_range(
        &self,
    ) -> &[self::_puroro_root::google::protobuf::descriptor_proto::ExtensionRange] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField::<
            self::_puroro_root::google::protobuf::descriptor_proto::ExtensionRange,
        > as RepeatedFieldType>::get_field(&self.extension_range, &self._bitfield)
    }
    pub fn oneof_decl(
        &self,
    ) -> &[self::_puroro_root::google::protobuf::OneofDescriptorProto] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField::<
            self::_puroro_root::google::protobuf::OneofDescriptorProto,
        > as RepeatedFieldType>::get_field(&self.oneof_decl, &self._bitfield)
    }
    pub fn options(
        &self,
    ) -> ::std::option::Option::<&self::_puroro_root::google::protobuf::MessageOptions> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField::<
            self::_puroro_root::google::protobuf::MessageOptions,
        > as NonRepeatedFieldType>::get_field(
            &self.options,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn reserved_range(
        &self,
    ) -> &[self::_puroro_root::google::protobuf::descriptor_proto::ReservedRange] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField::<
            self::_puroro_root::google::protobuf::descriptor_proto::ReservedRange,
        > as RepeatedFieldType>::get_field(&self.reserved_range, &self._bitfield)
    }
    pub fn reserved_name(&self) -> &[impl ::std::ops::Deref::<Target = str>] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedStringField as RepeatedFieldType>::get_field(
            &self.reserved_name,
            &self._bitfield,
        )
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
        #[allow(unused)]
        use ::std::result::Result::Ok;
        #[allow(unused)]
        use ::std::option::Option::Some;
        #[allow(unused)]
        use self::_puroro::internal::field_type::FieldType;
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        use self::_puroro::internal::ser::FieldData;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            todo!()
        }
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use ::std::result::Result::Ok;
        Ok(todo!())
    }
}
impl ::std::clone::Clone for DescriptorProto {
    fn clone(&self) -> Self {
        Self {
            name: <self::_puroro::internal::field_type::OptionalStringField::<
                0usize,
            > as ::std::clone::Clone>::clone(&self.name),
            field: <self::_puroro::internal::field_type::RepeatedMessageField::<
                self::_puroro_root::google::protobuf::FieldDescriptorProto,
            > as ::std::clone::Clone>::clone(&self.field),
            extension: <self::_puroro::internal::field_type::RepeatedMessageField::<
                self::_puroro_root::google::protobuf::FieldDescriptorProto,
            > as ::std::clone::Clone>::clone(&self.extension),
            nested_type: <self::_puroro::internal::field_type::RepeatedMessageField::<
                self::_puroro_root::google::protobuf::DescriptorProto,
            > as ::std::clone::Clone>::clone(&self.nested_type),
            enum_type: <self::_puroro::internal::field_type::RepeatedMessageField::<
                self::_puroro_root::google::protobuf::EnumDescriptorProto,
            > as ::std::clone::Clone>::clone(&self.enum_type),
            extension_range: <self::_puroro::internal::field_type::RepeatedMessageField::<
                self::_puroro_root::google::protobuf::descriptor_proto::ExtensionRange,
            > as ::std::clone::Clone>::clone(&self.extension_range),
            oneof_decl: <self::_puroro::internal::field_type::RepeatedMessageField::<
                self::_puroro_root::google::protobuf::OneofDescriptorProto,
            > as ::std::clone::Clone>::clone(&self.oneof_decl),
            options: <self::_puroro::internal::field_type::SingularHeapMessageField::<
                self::_puroro_root::google::protobuf::MessageOptions,
            > as ::std::clone::Clone>::clone(&self.options),
            reserved_range: <self::_puroro::internal::field_type::RepeatedMessageField::<
                self::_puroro_root::google::protobuf::descriptor_proto::ReservedRange,
            > as ::std::clone::Clone>::clone(&self.reserved_range),
            reserved_name: <self::_puroro::internal::field_type::RepeatedStringField as ::std::clone::Clone>::clone(
                &self.reserved_name,
            ),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
#[derive(::std::default::Default)]
pub struct ExtensionRangeOptions {
    uninterpreted_option: self::_puroro::internal::field_type::RepeatedMessageField::<
        self::_puroro_root::google::protobuf::UninterpretedOption,
    >,
    _bitfield: self::_puroro::bitvec::BitArray<0usize>,
}
impl ExtensionRangeOptions {
    pub fn uninterpreted_option(
        &self,
    ) -> &[self::_puroro_root::google::protobuf::UninterpretedOption] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField::<
            self::_puroro_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::get_field(&self.uninterpreted_option, &self._bitfield)
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
        #[allow(unused)]
        use ::std::result::Result::Ok;
        #[allow(unused)]
        use ::std::option::Option::Some;
        #[allow(unused)]
        use self::_puroro::internal::field_type::FieldType;
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        use self::_puroro::internal::ser::FieldData;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            todo!()
        }
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use ::std::result::Result::Ok;
        Ok(todo!())
    }
}
impl ::std::clone::Clone for ExtensionRangeOptions {
    fn clone(&self) -> Self {
        Self {
            uninterpreted_option: <self::_puroro::internal::field_type::RepeatedMessageField::<
                self::_puroro_root::google::protobuf::UninterpretedOption,
            > as ::std::clone::Clone>::clone(&self.uninterpreted_option),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
#[derive(::std::default::Default)]
pub struct FieldDescriptorProto {
    name: self::_puroro::internal::field_type::OptionalStringField::<0usize>,
    number: self::_puroro::internal::field_type::OptionalNumericalField::<
        i32,
        self::_puroro::tags::Int32,
        1usize,
    >,
    label: self::_puroro::internal::field_type::OptionalNumericalField::<
        self::_puroro_root::google::protobuf::field_descriptor_proto::Label,
        self::_puroro::tags::Enum2::<
            self::_puroro_root::google::protobuf::field_descriptor_proto::Label,
        >,
        2usize,
    >,
    r#type: self::_puroro::internal::field_type::OptionalNumericalField::<
        self::_puroro_root::google::protobuf::field_descriptor_proto::Type,
        self::_puroro::tags::Enum2::<
            self::_puroro_root::google::protobuf::field_descriptor_proto::Type,
        >,
        3usize,
    >,
    type_name: self::_puroro::internal::field_type::OptionalStringField::<4usize>,
    extendee: self::_puroro::internal::field_type::OptionalStringField::<5usize>,
    default_value: self::_puroro::internal::field_type::OptionalStringField::<6usize>,
    oneof_index: self::_puroro::internal::field_type::OptionalNumericalField::<
        i32,
        self::_puroro::tags::Int32,
        7usize,
    >,
    json_name: self::_puroro::internal::field_type::OptionalStringField::<8usize>,
    options: self::_puroro::internal::field_type::SingularHeapMessageField::<
        self::_puroro_root::google::protobuf::FieldOptions,
    >,
    proto3_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        bool,
        self::_puroro::tags::Bool,
        9usize,
    >,
    _bitfield: self::_puroro::bitvec::BitArray<1usize>,
}
impl FieldDescriptorProto {
    pub fn name(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            0usize,
        > as NonRepeatedFieldType>::get_field(
            &self.name,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn number(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            1usize,
        > as NonRepeatedFieldType>::get_field(
            &self.number,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn label(
        &self,
    ) -> self::_puroro_root::google::protobuf::field_descriptor_proto::Label {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            self::_puroro_root::google::protobuf::field_descriptor_proto::Label,
            self::_puroro::tags::Enum2::<
                self::_puroro_root::google::protobuf::field_descriptor_proto::Label,
            >,
            2usize,
        > as NonRepeatedFieldType>::get_field(
            &self.label,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn r#type(
        &self,
    ) -> self::_puroro_root::google::protobuf::field_descriptor_proto::Type {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            self::_puroro_root::google::protobuf::field_descriptor_proto::Type,
            self::_puroro::tags::Enum2::<
                self::_puroro_root::google::protobuf::field_descriptor_proto::Type,
            >,
            3usize,
        > as NonRepeatedFieldType>::get_field(
            &self.r#type,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn type_name(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            4usize,
        > as NonRepeatedFieldType>::get_field(
            &self.type_name,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn extendee(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            5usize,
        > as NonRepeatedFieldType>::get_field(
            &self.extendee,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn default_value(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            6usize,
        > as NonRepeatedFieldType>::get_field(
            &self.default_value,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn oneof_index(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            7usize,
        > as NonRepeatedFieldType>::get_field(
            &self.oneof_index,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn json_name(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            8usize,
        > as NonRepeatedFieldType>::get_field(
            &self.json_name,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn options(
        &self,
    ) -> ::std::option::Option::<&self::_puroro_root::google::protobuf::FieldOptions> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField::<
            self::_puroro_root::google::protobuf::FieldOptions,
        > as NonRepeatedFieldType>::get_field(
            &self.options,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn proto3_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            bool,
            self::_puroro::tags::Bool,
            9usize,
        > as NonRepeatedFieldType>::get_field(
            &self.proto3_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
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
        #[allow(unused)]
        use ::std::result::Result::Ok;
        #[allow(unused)]
        use ::std::option::Option::Some;
        #[allow(unused)]
        use self::_puroro::internal::field_type::FieldType;
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        use self::_puroro::internal::ser::FieldData;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            todo!()
        }
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use ::std::result::Result::Ok;
        Ok(todo!())
    }
}
impl ::std::clone::Clone for FieldDescriptorProto {
    fn clone(&self) -> Self {
        Self {
            name: <self::_puroro::internal::field_type::OptionalStringField::<
                0usize,
            > as ::std::clone::Clone>::clone(&self.name),
            number: <self::_puroro::internal::field_type::OptionalNumericalField::<
                i32,
                self::_puroro::tags::Int32,
                1usize,
            > as ::std::clone::Clone>::clone(&self.number),
            label: <self::_puroro::internal::field_type::OptionalNumericalField::<
                self::_puroro_root::google::protobuf::field_descriptor_proto::Label,
                self::_puroro::tags::Enum2::<
                    self::_puroro_root::google::protobuf::field_descriptor_proto::Label,
                >,
                2usize,
            > as ::std::clone::Clone>::clone(&self.label),
            r#type: <self::_puroro::internal::field_type::OptionalNumericalField::<
                self::_puroro_root::google::protobuf::field_descriptor_proto::Type,
                self::_puroro::tags::Enum2::<
                    self::_puroro_root::google::protobuf::field_descriptor_proto::Type,
                >,
                3usize,
            > as ::std::clone::Clone>::clone(&self.r#type),
            type_name: <self::_puroro::internal::field_type::OptionalStringField::<
                4usize,
            > as ::std::clone::Clone>::clone(&self.type_name),
            extendee: <self::_puroro::internal::field_type::OptionalStringField::<
                5usize,
            > as ::std::clone::Clone>::clone(&self.extendee),
            default_value: <self::_puroro::internal::field_type::OptionalStringField::<
                6usize,
            > as ::std::clone::Clone>::clone(&self.default_value),
            oneof_index: <self::_puroro::internal::field_type::OptionalNumericalField::<
                i32,
                self::_puroro::tags::Int32,
                7usize,
            > as ::std::clone::Clone>::clone(&self.oneof_index),
            json_name: <self::_puroro::internal::field_type::OptionalStringField::<
                8usize,
            > as ::std::clone::Clone>::clone(&self.json_name),
            options: <self::_puroro::internal::field_type::SingularHeapMessageField::<
                self::_puroro_root::google::protobuf::FieldOptions,
            > as ::std::clone::Clone>::clone(&self.options),
            proto3_optional: <self::_puroro::internal::field_type::OptionalNumericalField::<
                bool,
                self::_puroro::tags::Bool,
                9usize,
            > as ::std::clone::Clone>::clone(&self.proto3_optional),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
#[derive(::std::default::Default)]
pub struct OneofDescriptorProto {
    name: self::_puroro::internal::field_type::OptionalStringField::<0usize>,
    options: self::_puroro::internal::field_type::SingularHeapMessageField::<
        self::_puroro_root::google::protobuf::OneofOptions,
    >,
    _bitfield: self::_puroro::bitvec::BitArray<1usize>,
}
impl OneofDescriptorProto {
    pub fn name(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            0usize,
        > as NonRepeatedFieldType>::get_field(
            &self.name,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn options(
        &self,
    ) -> ::std::option::Option::<&self::_puroro_root::google::protobuf::OneofOptions> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField::<
            self::_puroro_root::google::protobuf::OneofOptions,
        > as NonRepeatedFieldType>::get_field(
            &self.options,
            &self._bitfield,
            ::std::default::Default::default,
        )
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
        #[allow(unused)]
        use ::std::result::Result::Ok;
        #[allow(unused)]
        use ::std::option::Option::Some;
        #[allow(unused)]
        use self::_puroro::internal::field_type::FieldType;
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        use self::_puroro::internal::ser::FieldData;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            todo!()
        }
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use ::std::result::Result::Ok;
        Ok(todo!())
    }
}
impl ::std::clone::Clone for OneofDescriptorProto {
    fn clone(&self) -> Self {
        Self {
            name: <self::_puroro::internal::field_type::OptionalStringField::<
                0usize,
            > as ::std::clone::Clone>::clone(&self.name),
            options: <self::_puroro::internal::field_type::SingularHeapMessageField::<
                self::_puroro_root::google::protobuf::OneofOptions,
            > as ::std::clone::Clone>::clone(&self.options),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
#[derive(::std::default::Default)]
pub struct EnumDescriptorProto {
    name: self::_puroro::internal::field_type::OptionalStringField::<0usize>,
    value: self::_puroro::internal::field_type::RepeatedMessageField::<
        self::_puroro_root::google::protobuf::EnumValueDescriptorProto,
    >,
    options: self::_puroro::internal::field_type::SingularHeapMessageField::<
        self::_puroro_root::google::protobuf::EnumOptions,
    >,
    reserved_range: self::_puroro::internal::field_type::RepeatedMessageField::<
        self::_puroro_root::google::protobuf::enum_descriptor_proto::EnumReservedRange,
    >,
    reserved_name: self::_puroro::internal::field_type::RepeatedStringField,
    _bitfield: self::_puroro::bitvec::BitArray<1usize>,
}
impl EnumDescriptorProto {
    pub fn name(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            0usize,
        > as NonRepeatedFieldType>::get_field(
            &self.name,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn value(
        &self,
    ) -> &[self::_puroro_root::google::protobuf::EnumValueDescriptorProto] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField::<
            self::_puroro_root::google::protobuf::EnumValueDescriptorProto,
        > as RepeatedFieldType>::get_field(&self.value, &self._bitfield)
    }
    pub fn options(
        &self,
    ) -> ::std::option::Option::<&self::_puroro_root::google::protobuf::EnumOptions> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField::<
            self::_puroro_root::google::protobuf::EnumOptions,
        > as NonRepeatedFieldType>::get_field(
            &self.options,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn reserved_range(
        &self,
    ) -> &[self::_puroro_root::google::protobuf::enum_descriptor_proto::EnumReservedRange] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField::<
            self::_puroro_root::google::protobuf::enum_descriptor_proto::EnumReservedRange,
        > as RepeatedFieldType>::get_field(&self.reserved_range, &self._bitfield)
    }
    pub fn reserved_name(&self) -> &[impl ::std::ops::Deref::<Target = str>] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedStringField as RepeatedFieldType>::get_field(
            &self.reserved_name,
            &self._bitfield,
        )
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
        #[allow(unused)]
        use ::std::result::Result::Ok;
        #[allow(unused)]
        use ::std::option::Option::Some;
        #[allow(unused)]
        use self::_puroro::internal::field_type::FieldType;
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        use self::_puroro::internal::ser::FieldData;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            todo!()
        }
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use ::std::result::Result::Ok;
        Ok(todo!())
    }
}
impl ::std::clone::Clone for EnumDescriptorProto {
    fn clone(&self) -> Self {
        Self {
            name: <self::_puroro::internal::field_type::OptionalStringField::<
                0usize,
            > as ::std::clone::Clone>::clone(&self.name),
            value: <self::_puroro::internal::field_type::RepeatedMessageField::<
                self::_puroro_root::google::protobuf::EnumValueDescriptorProto,
            > as ::std::clone::Clone>::clone(&self.value),
            options: <self::_puroro::internal::field_type::SingularHeapMessageField::<
                self::_puroro_root::google::protobuf::EnumOptions,
            > as ::std::clone::Clone>::clone(&self.options),
            reserved_range: <self::_puroro::internal::field_type::RepeatedMessageField::<
                self::_puroro_root::google::protobuf::enum_descriptor_proto::EnumReservedRange,
            > as ::std::clone::Clone>::clone(&self.reserved_range),
            reserved_name: <self::_puroro::internal::field_type::RepeatedStringField as ::std::clone::Clone>::clone(
                &self.reserved_name,
            ),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
#[derive(::std::default::Default)]
pub struct EnumValueDescriptorProto {
    name: self::_puroro::internal::field_type::OptionalStringField::<0usize>,
    number: self::_puroro::internal::field_type::OptionalNumericalField::<
        i32,
        self::_puroro::tags::Int32,
        1usize,
    >,
    options: self::_puroro::internal::field_type::SingularHeapMessageField::<
        self::_puroro_root::google::protobuf::EnumValueOptions,
    >,
    _bitfield: self::_puroro::bitvec::BitArray<1usize>,
}
impl EnumValueDescriptorProto {
    pub fn name(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            0usize,
        > as NonRepeatedFieldType>::get_field(
            &self.name,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn number(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            1usize,
        > as NonRepeatedFieldType>::get_field(
            &self.number,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn options(
        &self,
    ) -> ::std::option::Option::<
        &self::_puroro_root::google::protobuf::EnumValueOptions,
    > {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField::<
            self::_puroro_root::google::protobuf::EnumValueOptions,
        > as NonRepeatedFieldType>::get_field(
            &self.options,
            &self._bitfield,
            ::std::default::Default::default,
        )
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
        #[allow(unused)]
        use ::std::result::Result::Ok;
        #[allow(unused)]
        use ::std::option::Option::Some;
        #[allow(unused)]
        use self::_puroro::internal::field_type::FieldType;
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        use self::_puroro::internal::ser::FieldData;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            todo!()
        }
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use ::std::result::Result::Ok;
        Ok(todo!())
    }
}
impl ::std::clone::Clone for EnumValueDescriptorProto {
    fn clone(&self) -> Self {
        Self {
            name: <self::_puroro::internal::field_type::OptionalStringField::<
                0usize,
            > as ::std::clone::Clone>::clone(&self.name),
            number: <self::_puroro::internal::field_type::OptionalNumericalField::<
                i32,
                self::_puroro::tags::Int32,
                1usize,
            > as ::std::clone::Clone>::clone(&self.number),
            options: <self::_puroro::internal::field_type::SingularHeapMessageField::<
                self::_puroro_root::google::protobuf::EnumValueOptions,
            > as ::std::clone::Clone>::clone(&self.options),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
#[derive(::std::default::Default)]
pub struct ServiceDescriptorProto {
    name: self::_puroro::internal::field_type::OptionalStringField::<0usize>,
    method: self::_puroro::internal::field_type::RepeatedMessageField::<
        self::_puroro_root::google::protobuf::MethodDescriptorProto,
    >,
    options: self::_puroro::internal::field_type::SingularHeapMessageField::<
        self::_puroro_root::google::protobuf::ServiceOptions,
    >,
    _bitfield: self::_puroro::bitvec::BitArray<1usize>,
}
impl ServiceDescriptorProto {
    pub fn name(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            0usize,
        > as NonRepeatedFieldType>::get_field(
            &self.name,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn method(
        &self,
    ) -> &[self::_puroro_root::google::protobuf::MethodDescriptorProto] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField::<
            self::_puroro_root::google::protobuf::MethodDescriptorProto,
        > as RepeatedFieldType>::get_field(&self.method, &self._bitfield)
    }
    pub fn options(
        &self,
    ) -> ::std::option::Option::<&self::_puroro_root::google::protobuf::ServiceOptions> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField::<
            self::_puroro_root::google::protobuf::ServiceOptions,
        > as NonRepeatedFieldType>::get_field(
            &self.options,
            &self._bitfield,
            ::std::default::Default::default,
        )
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
        #[allow(unused)]
        use ::std::result::Result::Ok;
        #[allow(unused)]
        use ::std::option::Option::Some;
        #[allow(unused)]
        use self::_puroro::internal::field_type::FieldType;
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        use self::_puroro::internal::ser::FieldData;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            todo!()
        }
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use ::std::result::Result::Ok;
        Ok(todo!())
    }
}
impl ::std::clone::Clone for ServiceDescriptorProto {
    fn clone(&self) -> Self {
        Self {
            name: <self::_puroro::internal::field_type::OptionalStringField::<
                0usize,
            > as ::std::clone::Clone>::clone(&self.name),
            method: <self::_puroro::internal::field_type::RepeatedMessageField::<
                self::_puroro_root::google::protobuf::MethodDescriptorProto,
            > as ::std::clone::Clone>::clone(&self.method),
            options: <self::_puroro::internal::field_type::SingularHeapMessageField::<
                self::_puroro_root::google::protobuf::ServiceOptions,
            > as ::std::clone::Clone>::clone(&self.options),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
#[derive(::std::default::Default)]
pub struct MethodDescriptorProto {
    name: self::_puroro::internal::field_type::OptionalStringField::<0usize>,
    input_type: self::_puroro::internal::field_type::OptionalStringField::<1usize>,
    output_type: self::_puroro::internal::field_type::OptionalStringField::<2usize>,
    options: self::_puroro::internal::field_type::SingularHeapMessageField::<
        self::_puroro_root::google::protobuf::MethodOptions,
    >,
    client_streaming: self::_puroro::internal::field_type::OptionalNumericalField::<
        bool,
        self::_puroro::tags::Bool,
        3usize,
    >,
    server_streaming: self::_puroro::internal::field_type::OptionalNumericalField::<
        bool,
        self::_puroro::tags::Bool,
        4usize,
    >,
    _bitfield: self::_puroro::bitvec::BitArray<1usize>,
}
impl MethodDescriptorProto {
    pub fn name(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            0usize,
        > as NonRepeatedFieldType>::get_field(
            &self.name,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn input_type(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            1usize,
        > as NonRepeatedFieldType>::get_field(
            &self.input_type,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn output_type(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            2usize,
        > as NonRepeatedFieldType>::get_field(
            &self.output_type,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn options(
        &self,
    ) -> ::std::option::Option::<&self::_puroro_root::google::protobuf::MethodOptions> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField::<
            self::_puroro_root::google::protobuf::MethodOptions,
        > as NonRepeatedFieldType>::get_field(
            &self.options,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn client_streaming(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            bool,
            self::_puroro::tags::Bool,
            3usize,
        > as NonRepeatedFieldType>::get_field(
            &self.client_streaming,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn server_streaming(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            bool,
            self::_puroro::tags::Bool,
            4usize,
        > as NonRepeatedFieldType>::get_field(
            &self.server_streaming,
            &self._bitfield,
            ::std::default::Default::default,
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
        #[allow(unused)]
        use ::std::result::Result::Ok;
        #[allow(unused)]
        use ::std::option::Option::Some;
        #[allow(unused)]
        use self::_puroro::internal::field_type::FieldType;
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        use self::_puroro::internal::ser::FieldData;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            todo!()
        }
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use ::std::result::Result::Ok;
        Ok(todo!())
    }
}
impl ::std::clone::Clone for MethodDescriptorProto {
    fn clone(&self) -> Self {
        Self {
            name: <self::_puroro::internal::field_type::OptionalStringField::<
                0usize,
            > as ::std::clone::Clone>::clone(&self.name),
            input_type: <self::_puroro::internal::field_type::OptionalStringField::<
                1usize,
            > as ::std::clone::Clone>::clone(&self.input_type),
            output_type: <self::_puroro::internal::field_type::OptionalStringField::<
                2usize,
            > as ::std::clone::Clone>::clone(&self.output_type),
            options: <self::_puroro::internal::field_type::SingularHeapMessageField::<
                self::_puroro_root::google::protobuf::MethodOptions,
            > as ::std::clone::Clone>::clone(&self.options),
            client_streaming: <self::_puroro::internal::field_type::OptionalNumericalField::<
                bool,
                self::_puroro::tags::Bool,
                3usize,
            > as ::std::clone::Clone>::clone(&self.client_streaming),
            server_streaming: <self::_puroro::internal::field_type::OptionalNumericalField::<
                bool,
                self::_puroro::tags::Bool,
                4usize,
            > as ::std::clone::Clone>::clone(&self.server_streaming),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
#[derive(::std::default::Default)]
pub struct FileOptions {
    java_package: self::_puroro::internal::field_type::OptionalStringField::<0usize>,
    java_outer_classname: self::_puroro::internal::field_type::OptionalStringField::<
        1usize,
    >,
    java_multiple_files: self::_puroro::internal::field_type::OptionalNumericalField::<
        bool,
        self::_puroro::tags::Bool,
        2usize,
    >,
    java_generate_equals_and_hash: self::_puroro::internal::field_type::OptionalNumericalField::<
        bool,
        self::_puroro::tags::Bool,
        3usize,
    >,
    java_string_check_utf8: self::_puroro::internal::field_type::OptionalNumericalField::<
        bool,
        self::_puroro::tags::Bool,
        4usize,
    >,
    optimize_for: self::_puroro::internal::field_type::OptionalNumericalField::<
        self::_puroro_root::google::protobuf::file_options::OptimizeMode,
        self::_puroro::tags::Enum2::<
            self::_puroro_root::google::protobuf::file_options::OptimizeMode,
        >,
        5usize,
    >,
    go_package: self::_puroro::internal::field_type::OptionalStringField::<6usize>,
    cc_generic_services: self::_puroro::internal::field_type::OptionalNumericalField::<
        bool,
        self::_puroro::tags::Bool,
        7usize,
    >,
    java_generic_services: self::_puroro::internal::field_type::OptionalNumericalField::<
        bool,
        self::_puroro::tags::Bool,
        8usize,
    >,
    py_generic_services: self::_puroro::internal::field_type::OptionalNumericalField::<
        bool,
        self::_puroro::tags::Bool,
        9usize,
    >,
    php_generic_services: self::_puroro::internal::field_type::OptionalNumericalField::<
        bool,
        self::_puroro::tags::Bool,
        10usize,
    >,
    deprecated: self::_puroro::internal::field_type::OptionalNumericalField::<
        bool,
        self::_puroro::tags::Bool,
        11usize,
    >,
    cc_enable_arenas: self::_puroro::internal::field_type::OptionalNumericalField::<
        bool,
        self::_puroro::tags::Bool,
        12usize,
    >,
    objc_class_prefix: self::_puroro::internal::field_type::OptionalStringField::<
        13usize,
    >,
    csharp_namespace: self::_puroro::internal::field_type::OptionalStringField::<
        14usize,
    >,
    swift_prefix: self::_puroro::internal::field_type::OptionalStringField::<15usize>,
    php_class_prefix: self::_puroro::internal::field_type::OptionalStringField::<
        16usize,
    >,
    php_namespace: self::_puroro::internal::field_type::OptionalStringField::<17usize>,
    php_metadata_namespace: self::_puroro::internal::field_type::OptionalStringField::<
        18usize,
    >,
    ruby_package: self::_puroro::internal::field_type::OptionalStringField::<19usize>,
    uninterpreted_option: self::_puroro::internal::field_type::RepeatedMessageField::<
        self::_puroro_root::google::protobuf::UninterpretedOption,
    >,
    _bitfield: self::_puroro::bitvec::BitArray<1usize>,
}
impl FileOptions {
    pub fn java_package(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            0usize,
        > as NonRepeatedFieldType>::get_field(
            &self.java_package,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn java_outer_classname(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            1usize,
        > as NonRepeatedFieldType>::get_field(
            &self.java_outer_classname,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn java_multiple_files(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            bool,
            self::_puroro::tags::Bool,
            2usize,
        > as NonRepeatedFieldType>::get_field(
            &self.java_multiple_files,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn java_generate_equals_and_hash(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            bool,
            self::_puroro::tags::Bool,
            3usize,
        > as NonRepeatedFieldType>::get_field(
            &self.java_generate_equals_and_hash,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn java_string_check_utf8(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            bool,
            self::_puroro::tags::Bool,
            4usize,
        > as NonRepeatedFieldType>::get_field(
            &self.java_string_check_utf8,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn optimize_for(
        &self,
    ) -> self::_puroro_root::google::protobuf::file_options::OptimizeMode {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            self::_puroro_root::google::protobuf::file_options::OptimizeMode,
            self::_puroro::tags::Enum2::<
                self::_puroro_root::google::protobuf::file_options::OptimizeMode,
            >,
            5usize,
        > as NonRepeatedFieldType>::get_field(
            &self.optimize_for,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn go_package(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            6usize,
        > as NonRepeatedFieldType>::get_field(
            &self.go_package,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn cc_generic_services(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            bool,
            self::_puroro::tags::Bool,
            7usize,
        > as NonRepeatedFieldType>::get_field(
            &self.cc_generic_services,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn java_generic_services(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            bool,
            self::_puroro::tags::Bool,
            8usize,
        > as NonRepeatedFieldType>::get_field(
            &self.java_generic_services,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn py_generic_services(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            bool,
            self::_puroro::tags::Bool,
            9usize,
        > as NonRepeatedFieldType>::get_field(
            &self.py_generic_services,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn php_generic_services(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            bool,
            self::_puroro::tags::Bool,
            10usize,
        > as NonRepeatedFieldType>::get_field(
            &self.php_generic_services,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn deprecated(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            bool,
            self::_puroro::tags::Bool,
            11usize,
        > as NonRepeatedFieldType>::get_field(
            &self.deprecated,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn cc_enable_arenas(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            bool,
            self::_puroro::tags::Bool,
            12usize,
        > as NonRepeatedFieldType>::get_field(
            &self.cc_enable_arenas,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn objc_class_prefix(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            13usize,
        > as NonRepeatedFieldType>::get_field(
            &self.objc_class_prefix,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn csharp_namespace(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            14usize,
        > as NonRepeatedFieldType>::get_field(
            &self.csharp_namespace,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn swift_prefix(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            15usize,
        > as NonRepeatedFieldType>::get_field(
            &self.swift_prefix,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn php_class_prefix(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            16usize,
        > as NonRepeatedFieldType>::get_field(
            &self.php_class_prefix,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn php_namespace(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            17usize,
        > as NonRepeatedFieldType>::get_field(
            &self.php_namespace,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn php_metadata_namespace(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            18usize,
        > as NonRepeatedFieldType>::get_field(
            &self.php_metadata_namespace,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn ruby_package(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            19usize,
        > as NonRepeatedFieldType>::get_field(
            &self.ruby_package,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn uninterpreted_option(
        &self,
    ) -> &[self::_puroro_root::google::protobuf::UninterpretedOption] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField::<
            self::_puroro_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::get_field(&self.uninterpreted_option, &self._bitfield)
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
        #[allow(unused)]
        use ::std::result::Result::Ok;
        #[allow(unused)]
        use ::std::option::Option::Some;
        #[allow(unused)]
        use self::_puroro::internal::field_type::FieldType;
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        use self::_puroro::internal::ser::FieldData;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            todo!()
        }
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use ::std::result::Result::Ok;
        Ok(todo!())
    }
}
impl ::std::clone::Clone for FileOptions {
    fn clone(&self) -> Self {
        Self {
            java_package: <self::_puroro::internal::field_type::OptionalStringField::<
                0usize,
            > as ::std::clone::Clone>::clone(&self.java_package),
            java_outer_classname: <self::_puroro::internal::field_type::OptionalStringField::<
                1usize,
            > as ::std::clone::Clone>::clone(&self.java_outer_classname),
            java_multiple_files: <self::_puroro::internal::field_type::OptionalNumericalField::<
                bool,
                self::_puroro::tags::Bool,
                2usize,
            > as ::std::clone::Clone>::clone(&self.java_multiple_files),
            java_generate_equals_and_hash: <self::_puroro::internal::field_type::OptionalNumericalField::<
                bool,
                self::_puroro::tags::Bool,
                3usize,
            > as ::std::clone::Clone>::clone(&self.java_generate_equals_and_hash),
            java_string_check_utf8: <self::_puroro::internal::field_type::OptionalNumericalField::<
                bool,
                self::_puroro::tags::Bool,
                4usize,
            > as ::std::clone::Clone>::clone(&self.java_string_check_utf8),
            optimize_for: <self::_puroro::internal::field_type::OptionalNumericalField::<
                self::_puroro_root::google::protobuf::file_options::OptimizeMode,
                self::_puroro::tags::Enum2::<
                    self::_puroro_root::google::protobuf::file_options::OptimizeMode,
                >,
                5usize,
            > as ::std::clone::Clone>::clone(&self.optimize_for),
            go_package: <self::_puroro::internal::field_type::OptionalStringField::<
                6usize,
            > as ::std::clone::Clone>::clone(&self.go_package),
            cc_generic_services: <self::_puroro::internal::field_type::OptionalNumericalField::<
                bool,
                self::_puroro::tags::Bool,
                7usize,
            > as ::std::clone::Clone>::clone(&self.cc_generic_services),
            java_generic_services: <self::_puroro::internal::field_type::OptionalNumericalField::<
                bool,
                self::_puroro::tags::Bool,
                8usize,
            > as ::std::clone::Clone>::clone(&self.java_generic_services),
            py_generic_services: <self::_puroro::internal::field_type::OptionalNumericalField::<
                bool,
                self::_puroro::tags::Bool,
                9usize,
            > as ::std::clone::Clone>::clone(&self.py_generic_services),
            php_generic_services: <self::_puroro::internal::field_type::OptionalNumericalField::<
                bool,
                self::_puroro::tags::Bool,
                10usize,
            > as ::std::clone::Clone>::clone(&self.php_generic_services),
            deprecated: <self::_puroro::internal::field_type::OptionalNumericalField::<
                bool,
                self::_puroro::tags::Bool,
                11usize,
            > as ::std::clone::Clone>::clone(&self.deprecated),
            cc_enable_arenas: <self::_puroro::internal::field_type::OptionalNumericalField::<
                bool,
                self::_puroro::tags::Bool,
                12usize,
            > as ::std::clone::Clone>::clone(&self.cc_enable_arenas),
            objc_class_prefix: <self::_puroro::internal::field_type::OptionalStringField::<
                13usize,
            > as ::std::clone::Clone>::clone(&self.objc_class_prefix),
            csharp_namespace: <self::_puroro::internal::field_type::OptionalStringField::<
                14usize,
            > as ::std::clone::Clone>::clone(&self.csharp_namespace),
            swift_prefix: <self::_puroro::internal::field_type::OptionalStringField::<
                15usize,
            > as ::std::clone::Clone>::clone(&self.swift_prefix),
            php_class_prefix: <self::_puroro::internal::field_type::OptionalStringField::<
                16usize,
            > as ::std::clone::Clone>::clone(&self.php_class_prefix),
            php_namespace: <self::_puroro::internal::field_type::OptionalStringField::<
                17usize,
            > as ::std::clone::Clone>::clone(&self.php_namespace),
            php_metadata_namespace: <self::_puroro::internal::field_type::OptionalStringField::<
                18usize,
            > as ::std::clone::Clone>::clone(&self.php_metadata_namespace),
            ruby_package: <self::_puroro::internal::field_type::OptionalStringField::<
                19usize,
            > as ::std::clone::Clone>::clone(&self.ruby_package),
            uninterpreted_option: <self::_puroro::internal::field_type::RepeatedMessageField::<
                self::_puroro_root::google::protobuf::UninterpretedOption,
            > as ::std::clone::Clone>::clone(&self.uninterpreted_option),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
#[derive(::std::default::Default)]
pub struct MessageOptions {
    message_set_wire_format: self::_puroro::internal::field_type::OptionalNumericalField::<
        bool,
        self::_puroro::tags::Bool,
        0usize,
    >,
    no_standard_descriptor_accessor: self::_puroro::internal::field_type::OptionalNumericalField::<
        bool,
        self::_puroro::tags::Bool,
        1usize,
    >,
    deprecated: self::_puroro::internal::field_type::OptionalNumericalField::<
        bool,
        self::_puroro::tags::Bool,
        2usize,
    >,
    map_entry: self::_puroro::internal::field_type::OptionalNumericalField::<
        bool,
        self::_puroro::tags::Bool,
        3usize,
    >,
    uninterpreted_option: self::_puroro::internal::field_type::RepeatedMessageField::<
        self::_puroro_root::google::protobuf::UninterpretedOption,
    >,
    _bitfield: self::_puroro::bitvec::BitArray<1usize>,
}
impl MessageOptions {
    pub fn message_set_wire_format(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            bool,
            self::_puroro::tags::Bool,
            0usize,
        > as NonRepeatedFieldType>::get_field(
            &self.message_set_wire_format,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn no_standard_descriptor_accessor(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            bool,
            self::_puroro::tags::Bool,
            1usize,
        > as NonRepeatedFieldType>::get_field(
            &self.no_standard_descriptor_accessor,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn deprecated(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            bool,
            self::_puroro::tags::Bool,
            2usize,
        > as NonRepeatedFieldType>::get_field(
            &self.deprecated,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn map_entry(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            bool,
            self::_puroro::tags::Bool,
            3usize,
        > as NonRepeatedFieldType>::get_field(
            &self.map_entry,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn uninterpreted_option(
        &self,
    ) -> &[self::_puroro_root::google::protobuf::UninterpretedOption] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField::<
            self::_puroro_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::get_field(&self.uninterpreted_option, &self._bitfield)
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
        #[allow(unused)]
        use ::std::result::Result::Ok;
        #[allow(unused)]
        use ::std::option::Option::Some;
        #[allow(unused)]
        use self::_puroro::internal::field_type::FieldType;
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        use self::_puroro::internal::ser::FieldData;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            todo!()
        }
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use ::std::result::Result::Ok;
        Ok(todo!())
    }
}
impl ::std::clone::Clone for MessageOptions {
    fn clone(&self) -> Self {
        Self {
            message_set_wire_format: <self::_puroro::internal::field_type::OptionalNumericalField::<
                bool,
                self::_puroro::tags::Bool,
                0usize,
            > as ::std::clone::Clone>::clone(&self.message_set_wire_format),
            no_standard_descriptor_accessor: <self::_puroro::internal::field_type::OptionalNumericalField::<
                bool,
                self::_puroro::tags::Bool,
                1usize,
            > as ::std::clone::Clone>::clone(&self.no_standard_descriptor_accessor),
            deprecated: <self::_puroro::internal::field_type::OptionalNumericalField::<
                bool,
                self::_puroro::tags::Bool,
                2usize,
            > as ::std::clone::Clone>::clone(&self.deprecated),
            map_entry: <self::_puroro::internal::field_type::OptionalNumericalField::<
                bool,
                self::_puroro::tags::Bool,
                3usize,
            > as ::std::clone::Clone>::clone(&self.map_entry),
            uninterpreted_option: <self::_puroro::internal::field_type::RepeatedMessageField::<
                self::_puroro_root::google::protobuf::UninterpretedOption,
            > as ::std::clone::Clone>::clone(&self.uninterpreted_option),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
#[derive(::std::default::Default)]
pub struct FieldOptions {
    ctype: self::_puroro::internal::field_type::OptionalNumericalField::<
        self::_puroro_root::google::protobuf::field_options::CType,
        self::_puroro::tags::Enum2::<
            self::_puroro_root::google::protobuf::field_options::CType,
        >,
        0usize,
    >,
    packed: self::_puroro::internal::field_type::OptionalNumericalField::<
        bool,
        self::_puroro::tags::Bool,
        1usize,
    >,
    jstype: self::_puroro::internal::field_type::OptionalNumericalField::<
        self::_puroro_root::google::protobuf::field_options::JSType,
        self::_puroro::tags::Enum2::<
            self::_puroro_root::google::protobuf::field_options::JSType,
        >,
        2usize,
    >,
    lazy: self::_puroro::internal::field_type::OptionalNumericalField::<
        bool,
        self::_puroro::tags::Bool,
        3usize,
    >,
    deprecated: self::_puroro::internal::field_type::OptionalNumericalField::<
        bool,
        self::_puroro::tags::Bool,
        4usize,
    >,
    weak: self::_puroro::internal::field_type::OptionalNumericalField::<
        bool,
        self::_puroro::tags::Bool,
        5usize,
    >,
    uninterpreted_option: self::_puroro::internal::field_type::RepeatedMessageField::<
        self::_puroro_root::google::protobuf::UninterpretedOption,
    >,
    _bitfield: self::_puroro::bitvec::BitArray<1usize>,
}
impl FieldOptions {
    pub fn ctype(&self) -> self::_puroro_root::google::protobuf::field_options::CType {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            self::_puroro_root::google::protobuf::field_options::CType,
            self::_puroro::tags::Enum2::<
                self::_puroro_root::google::protobuf::field_options::CType,
            >,
            0usize,
        > as NonRepeatedFieldType>::get_field(
            &self.ctype,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn packed(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            bool,
            self::_puroro::tags::Bool,
            1usize,
        > as NonRepeatedFieldType>::get_field(
            &self.packed,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn jstype(&self) -> self::_puroro_root::google::protobuf::field_options::JSType {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            self::_puroro_root::google::protobuf::field_options::JSType,
            self::_puroro::tags::Enum2::<
                self::_puroro_root::google::protobuf::field_options::JSType,
            >,
            2usize,
        > as NonRepeatedFieldType>::get_field(
            &self.jstype,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn lazy(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            bool,
            self::_puroro::tags::Bool,
            3usize,
        > as NonRepeatedFieldType>::get_field(
            &self.lazy,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn deprecated(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            bool,
            self::_puroro::tags::Bool,
            4usize,
        > as NonRepeatedFieldType>::get_field(
            &self.deprecated,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn weak(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            bool,
            self::_puroro::tags::Bool,
            5usize,
        > as NonRepeatedFieldType>::get_field(
            &self.weak,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn uninterpreted_option(
        &self,
    ) -> &[self::_puroro_root::google::protobuf::UninterpretedOption] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField::<
            self::_puroro_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::get_field(&self.uninterpreted_option, &self._bitfield)
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
        #[allow(unused)]
        use ::std::result::Result::Ok;
        #[allow(unused)]
        use ::std::option::Option::Some;
        #[allow(unused)]
        use self::_puroro::internal::field_type::FieldType;
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        use self::_puroro::internal::ser::FieldData;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            todo!()
        }
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use ::std::result::Result::Ok;
        Ok(todo!())
    }
}
impl ::std::clone::Clone for FieldOptions {
    fn clone(&self) -> Self {
        Self {
            ctype: <self::_puroro::internal::field_type::OptionalNumericalField::<
                self::_puroro_root::google::protobuf::field_options::CType,
                self::_puroro::tags::Enum2::<
                    self::_puroro_root::google::protobuf::field_options::CType,
                >,
                0usize,
            > as ::std::clone::Clone>::clone(&self.ctype),
            packed: <self::_puroro::internal::field_type::OptionalNumericalField::<
                bool,
                self::_puroro::tags::Bool,
                1usize,
            > as ::std::clone::Clone>::clone(&self.packed),
            jstype: <self::_puroro::internal::field_type::OptionalNumericalField::<
                self::_puroro_root::google::protobuf::field_options::JSType,
                self::_puroro::tags::Enum2::<
                    self::_puroro_root::google::protobuf::field_options::JSType,
                >,
                2usize,
            > as ::std::clone::Clone>::clone(&self.jstype),
            lazy: <self::_puroro::internal::field_type::OptionalNumericalField::<
                bool,
                self::_puroro::tags::Bool,
                3usize,
            > as ::std::clone::Clone>::clone(&self.lazy),
            deprecated: <self::_puroro::internal::field_type::OptionalNumericalField::<
                bool,
                self::_puroro::tags::Bool,
                4usize,
            > as ::std::clone::Clone>::clone(&self.deprecated),
            weak: <self::_puroro::internal::field_type::OptionalNumericalField::<
                bool,
                self::_puroro::tags::Bool,
                5usize,
            > as ::std::clone::Clone>::clone(&self.weak),
            uninterpreted_option: <self::_puroro::internal::field_type::RepeatedMessageField::<
                self::_puroro_root::google::protobuf::UninterpretedOption,
            > as ::std::clone::Clone>::clone(&self.uninterpreted_option),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
#[derive(::std::default::Default)]
pub struct OneofOptions {
    uninterpreted_option: self::_puroro::internal::field_type::RepeatedMessageField::<
        self::_puroro_root::google::protobuf::UninterpretedOption,
    >,
    _bitfield: self::_puroro::bitvec::BitArray<0usize>,
}
impl OneofOptions {
    pub fn uninterpreted_option(
        &self,
    ) -> &[self::_puroro_root::google::protobuf::UninterpretedOption] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField::<
            self::_puroro_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::get_field(&self.uninterpreted_option, &self._bitfield)
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
        #[allow(unused)]
        use ::std::result::Result::Ok;
        #[allow(unused)]
        use ::std::option::Option::Some;
        #[allow(unused)]
        use self::_puroro::internal::field_type::FieldType;
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        use self::_puroro::internal::ser::FieldData;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            todo!()
        }
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use ::std::result::Result::Ok;
        Ok(todo!())
    }
}
impl ::std::clone::Clone for OneofOptions {
    fn clone(&self) -> Self {
        Self {
            uninterpreted_option: <self::_puroro::internal::field_type::RepeatedMessageField::<
                self::_puroro_root::google::protobuf::UninterpretedOption,
            > as ::std::clone::Clone>::clone(&self.uninterpreted_option),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
#[derive(::std::default::Default)]
pub struct EnumOptions {
    allow_alias: self::_puroro::internal::field_type::OptionalNumericalField::<
        bool,
        self::_puroro::tags::Bool,
        0usize,
    >,
    deprecated: self::_puroro::internal::field_type::OptionalNumericalField::<
        bool,
        self::_puroro::tags::Bool,
        1usize,
    >,
    uninterpreted_option: self::_puroro::internal::field_type::RepeatedMessageField::<
        self::_puroro_root::google::protobuf::UninterpretedOption,
    >,
    _bitfield: self::_puroro::bitvec::BitArray<1usize>,
}
impl EnumOptions {
    pub fn allow_alias(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            bool,
            self::_puroro::tags::Bool,
            0usize,
        > as NonRepeatedFieldType>::get_field(
            &self.allow_alias,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn deprecated(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            bool,
            self::_puroro::tags::Bool,
            1usize,
        > as NonRepeatedFieldType>::get_field(
            &self.deprecated,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn uninterpreted_option(
        &self,
    ) -> &[self::_puroro_root::google::protobuf::UninterpretedOption] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField::<
            self::_puroro_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::get_field(&self.uninterpreted_option, &self._bitfield)
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
        #[allow(unused)]
        use ::std::result::Result::Ok;
        #[allow(unused)]
        use ::std::option::Option::Some;
        #[allow(unused)]
        use self::_puroro::internal::field_type::FieldType;
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        use self::_puroro::internal::ser::FieldData;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            todo!()
        }
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use ::std::result::Result::Ok;
        Ok(todo!())
    }
}
impl ::std::clone::Clone for EnumOptions {
    fn clone(&self) -> Self {
        Self {
            allow_alias: <self::_puroro::internal::field_type::OptionalNumericalField::<
                bool,
                self::_puroro::tags::Bool,
                0usize,
            > as ::std::clone::Clone>::clone(&self.allow_alias),
            deprecated: <self::_puroro::internal::field_type::OptionalNumericalField::<
                bool,
                self::_puroro::tags::Bool,
                1usize,
            > as ::std::clone::Clone>::clone(&self.deprecated),
            uninterpreted_option: <self::_puroro::internal::field_type::RepeatedMessageField::<
                self::_puroro_root::google::protobuf::UninterpretedOption,
            > as ::std::clone::Clone>::clone(&self.uninterpreted_option),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
#[derive(::std::default::Default)]
pub struct EnumValueOptions {
    deprecated: self::_puroro::internal::field_type::OptionalNumericalField::<
        bool,
        self::_puroro::tags::Bool,
        0usize,
    >,
    uninterpreted_option: self::_puroro::internal::field_type::RepeatedMessageField::<
        self::_puroro_root::google::protobuf::UninterpretedOption,
    >,
    _bitfield: self::_puroro::bitvec::BitArray<1usize>,
}
impl EnumValueOptions {
    pub fn deprecated(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            bool,
            self::_puroro::tags::Bool,
            0usize,
        > as NonRepeatedFieldType>::get_field(
            &self.deprecated,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn uninterpreted_option(
        &self,
    ) -> &[self::_puroro_root::google::protobuf::UninterpretedOption] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField::<
            self::_puroro_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::get_field(&self.uninterpreted_option, &self._bitfield)
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
        #[allow(unused)]
        use ::std::result::Result::Ok;
        #[allow(unused)]
        use ::std::option::Option::Some;
        #[allow(unused)]
        use self::_puroro::internal::field_type::FieldType;
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        use self::_puroro::internal::ser::FieldData;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            todo!()
        }
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use ::std::result::Result::Ok;
        Ok(todo!())
    }
}
impl ::std::clone::Clone for EnumValueOptions {
    fn clone(&self) -> Self {
        Self {
            deprecated: <self::_puroro::internal::field_type::OptionalNumericalField::<
                bool,
                self::_puroro::tags::Bool,
                0usize,
            > as ::std::clone::Clone>::clone(&self.deprecated),
            uninterpreted_option: <self::_puroro::internal::field_type::RepeatedMessageField::<
                self::_puroro_root::google::protobuf::UninterpretedOption,
            > as ::std::clone::Clone>::clone(&self.uninterpreted_option),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
#[derive(::std::default::Default)]
pub struct ServiceOptions {
    deprecated: self::_puroro::internal::field_type::OptionalNumericalField::<
        bool,
        self::_puroro::tags::Bool,
        0usize,
    >,
    uninterpreted_option: self::_puroro::internal::field_type::RepeatedMessageField::<
        self::_puroro_root::google::protobuf::UninterpretedOption,
    >,
    _bitfield: self::_puroro::bitvec::BitArray<1usize>,
}
impl ServiceOptions {
    pub fn deprecated(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            bool,
            self::_puroro::tags::Bool,
            0usize,
        > as NonRepeatedFieldType>::get_field(
            &self.deprecated,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn uninterpreted_option(
        &self,
    ) -> &[self::_puroro_root::google::protobuf::UninterpretedOption] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField::<
            self::_puroro_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::get_field(&self.uninterpreted_option, &self._bitfield)
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
        #[allow(unused)]
        use ::std::result::Result::Ok;
        #[allow(unused)]
        use ::std::option::Option::Some;
        #[allow(unused)]
        use self::_puroro::internal::field_type::FieldType;
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        use self::_puroro::internal::ser::FieldData;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            todo!()
        }
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use ::std::result::Result::Ok;
        Ok(todo!())
    }
}
impl ::std::clone::Clone for ServiceOptions {
    fn clone(&self) -> Self {
        Self {
            deprecated: <self::_puroro::internal::field_type::OptionalNumericalField::<
                bool,
                self::_puroro::tags::Bool,
                0usize,
            > as ::std::clone::Clone>::clone(&self.deprecated),
            uninterpreted_option: <self::_puroro::internal::field_type::RepeatedMessageField::<
                self::_puroro_root::google::protobuf::UninterpretedOption,
            > as ::std::clone::Clone>::clone(&self.uninterpreted_option),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
#[derive(::std::default::Default)]
pub struct MethodOptions {
    deprecated: self::_puroro::internal::field_type::OptionalNumericalField::<
        bool,
        self::_puroro::tags::Bool,
        0usize,
    >,
    idempotency_level: self::_puroro::internal::field_type::OptionalNumericalField::<
        self::_puroro_root::google::protobuf::method_options::IdempotencyLevel,
        self::_puroro::tags::Enum2::<
            self::_puroro_root::google::protobuf::method_options::IdempotencyLevel,
        >,
        1usize,
    >,
    uninterpreted_option: self::_puroro::internal::field_type::RepeatedMessageField::<
        self::_puroro_root::google::protobuf::UninterpretedOption,
    >,
    _bitfield: self::_puroro::bitvec::BitArray<1usize>,
}
impl MethodOptions {
    pub fn deprecated(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            bool,
            self::_puroro::tags::Bool,
            0usize,
        > as NonRepeatedFieldType>::get_field(
            &self.deprecated,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn idempotency_level(
        &self,
    ) -> self::_puroro_root::google::protobuf::method_options::IdempotencyLevel {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            self::_puroro_root::google::protobuf::method_options::IdempotencyLevel,
            self::_puroro::tags::Enum2::<
                self::_puroro_root::google::protobuf::method_options::IdempotencyLevel,
            >,
            1usize,
        > as NonRepeatedFieldType>::get_field(
            &self.idempotency_level,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn uninterpreted_option(
        &self,
    ) -> &[self::_puroro_root::google::protobuf::UninterpretedOption] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField::<
            self::_puroro_root::google::protobuf::UninterpretedOption,
        > as RepeatedFieldType>::get_field(&self.uninterpreted_option, &self._bitfield)
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
        #[allow(unused)]
        use ::std::result::Result::Ok;
        #[allow(unused)]
        use ::std::option::Option::Some;
        #[allow(unused)]
        use self::_puroro::internal::field_type::FieldType;
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        use self::_puroro::internal::ser::FieldData;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            todo!()
        }
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use ::std::result::Result::Ok;
        Ok(todo!())
    }
}
impl ::std::clone::Clone for MethodOptions {
    fn clone(&self) -> Self {
        Self {
            deprecated: <self::_puroro::internal::field_type::OptionalNumericalField::<
                bool,
                self::_puroro::tags::Bool,
                0usize,
            > as ::std::clone::Clone>::clone(&self.deprecated),
            idempotency_level: <self::_puroro::internal::field_type::OptionalNumericalField::<
                self::_puroro_root::google::protobuf::method_options::IdempotencyLevel,
                self::_puroro::tags::Enum2::<
                    self::_puroro_root::google::protobuf::method_options::IdempotencyLevel,
                >,
                1usize,
            > as ::std::clone::Clone>::clone(&self.idempotency_level),
            uninterpreted_option: <self::_puroro::internal::field_type::RepeatedMessageField::<
                self::_puroro_root::google::protobuf::UninterpretedOption,
            > as ::std::clone::Clone>::clone(&self.uninterpreted_option),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
#[derive(::std::default::Default)]
pub struct UninterpretedOption {
    name: self::_puroro::internal::field_type::RepeatedMessageField::<
        self::_puroro_root::google::protobuf::uninterpreted_option::NamePart,
    >,
    identifier_value: self::_puroro::internal::field_type::OptionalStringField::<0usize>,
    positive_int_value: self::_puroro::internal::field_type::OptionalNumericalField::<
        u64,
        self::_puroro::tags::UInt64,
        1usize,
    >,
    negative_int_value: self::_puroro::internal::field_type::OptionalNumericalField::<
        i64,
        self::_puroro::tags::Int64,
        2usize,
    >,
    double_value: self::_puroro::internal::field_type::OptionalNumericalField::<
        f64,
        self::_puroro::tags::Double,
        3usize,
    >,
    string_value: self::_puroro::internal::field_type::OptionalBytesField::<4usize>,
    aggregate_value: self::_puroro::internal::field_type::OptionalStringField::<5usize>,
    _bitfield: self::_puroro::bitvec::BitArray<1usize>,
}
impl UninterpretedOption {
    pub fn name(
        &self,
    ) -> &[self::_puroro_root::google::protobuf::uninterpreted_option::NamePart] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField::<
            self::_puroro_root::google::protobuf::uninterpreted_option::NamePart,
        > as RepeatedFieldType>::get_field(&self.name, &self._bitfield)
    }
    pub fn identifier_value(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            0usize,
        > as NonRepeatedFieldType>::get_field(
            &self.identifier_value,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn positive_int_value(&self) -> u64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            u64,
            self::_puroro::tags::UInt64,
            1usize,
        > as NonRepeatedFieldType>::get_field(
            &self.positive_int_value,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn negative_int_value(&self) -> i64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i64,
            self::_puroro::tags::Int64,
            2usize,
        > as NonRepeatedFieldType>::get_field(
            &self.negative_int_value,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn double_value(&self) -> f64 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            f64,
            self::_puroro::tags::Double,
            3usize,
        > as NonRepeatedFieldType>::get_field(
            &self.double_value,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn string_value(&self) -> &[u8] {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalBytesField::<
            4usize,
        > as NonRepeatedFieldType>::get_field(
            &self.string_value,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn aggregate_value(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            5usize,
        > as NonRepeatedFieldType>::get_field(
            &self.aggregate_value,
            &self._bitfield,
            ::std::default::Default::default,
        )
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
        #[allow(unused)]
        use ::std::result::Result::Ok;
        #[allow(unused)]
        use ::std::option::Option::Some;
        #[allow(unused)]
        use self::_puroro::internal::field_type::FieldType;
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        use self::_puroro::internal::ser::FieldData;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            todo!()
        }
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use ::std::result::Result::Ok;
        Ok(todo!())
    }
}
impl ::std::clone::Clone for UninterpretedOption {
    fn clone(&self) -> Self {
        Self {
            name: <self::_puroro::internal::field_type::RepeatedMessageField::<
                self::_puroro_root::google::protobuf::uninterpreted_option::NamePart,
            > as ::std::clone::Clone>::clone(&self.name),
            identifier_value: <self::_puroro::internal::field_type::OptionalStringField::<
                0usize,
            > as ::std::clone::Clone>::clone(&self.identifier_value),
            positive_int_value: <self::_puroro::internal::field_type::OptionalNumericalField::<
                u64,
                self::_puroro::tags::UInt64,
                1usize,
            > as ::std::clone::Clone>::clone(&self.positive_int_value),
            negative_int_value: <self::_puroro::internal::field_type::OptionalNumericalField::<
                i64,
                self::_puroro::tags::Int64,
                2usize,
            > as ::std::clone::Clone>::clone(&self.negative_int_value),
            double_value: <self::_puroro::internal::field_type::OptionalNumericalField::<
                f64,
                self::_puroro::tags::Double,
                3usize,
            > as ::std::clone::Clone>::clone(&self.double_value),
            string_value: <self::_puroro::internal::field_type::OptionalBytesField::<
                4usize,
            > as ::std::clone::Clone>::clone(&self.string_value),
            aggregate_value: <self::_puroro::internal::field_type::OptionalStringField::<
                5usize,
            > as ::std::clone::Clone>::clone(&self.aggregate_value),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
#[derive(::std::default::Default)]
pub struct SourceCodeInfo {
    location: self::_puroro::internal::field_type::RepeatedMessageField::<
        self::_puroro_root::google::protobuf::source_code_info::Location,
    >,
    _bitfield: self::_puroro::bitvec::BitArray<0usize>,
}
impl SourceCodeInfo {
    pub fn location(
        &self,
    ) -> &[self::_puroro_root::google::protobuf::source_code_info::Location] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField::<
            self::_puroro_root::google::protobuf::source_code_info::Location,
        > as RepeatedFieldType>::get_field(&self.location, &self._bitfield)
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
        #[allow(unused)]
        use ::std::result::Result::Ok;
        #[allow(unused)]
        use ::std::option::Option::Some;
        #[allow(unused)]
        use self::_puroro::internal::field_type::FieldType;
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        use self::_puroro::internal::ser::FieldData;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            todo!()
        }
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use ::std::result::Result::Ok;
        Ok(todo!())
    }
}
impl ::std::clone::Clone for SourceCodeInfo {
    fn clone(&self) -> Self {
        Self {
            location: <self::_puroro::internal::field_type::RepeatedMessageField::<
                self::_puroro_root::google::protobuf::source_code_info::Location,
            > as ::std::clone::Clone>::clone(&self.location),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
#[derive(::std::default::Default)]
pub struct GeneratedCodeInfo {
    annotation: self::_puroro::internal::field_type::RepeatedMessageField::<
        self::_puroro_root::google::protobuf::generated_code_info::Annotation,
    >,
    _bitfield: self::_puroro::bitvec::BitArray<0usize>,
}
impl GeneratedCodeInfo {
    pub fn annotation(
        &self,
    ) -> &[self::_puroro_root::google::protobuf::generated_code_info::Annotation] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField::<
            self::_puroro_root::google::protobuf::generated_code_info::Annotation,
        > as RepeatedFieldType>::get_field(&self.annotation, &self._bitfield)
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
        #[allow(unused)]
        use ::std::result::Result::Ok;
        #[allow(unused)]
        use ::std::option::Option::Some;
        #[allow(unused)]
        use self::_puroro::internal::field_type::FieldType;
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        use self::_puroro::internal::ser::FieldData;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            todo!()
        }
        Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use ::std::result::Result::Ok;
        Ok(todo!())
    }
}
impl ::std::clone::Clone for GeneratedCodeInfo {
    fn clone(&self) -> Self {
        Self {
            annotation: <self::_puroro::internal::field_type::RepeatedMessageField::<
                self::_puroro_root::google::protobuf::generated_code_info::Annotation,
            > as ::std::clone::Clone>::clone(&self.annotation),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
