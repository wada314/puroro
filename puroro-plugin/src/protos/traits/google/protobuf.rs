#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

pub trait GeneratedCodeInfoTrait: ::std::clone::Clone {
    type AnnotationElement<'this>: self::generated_code_info::AnnotationTrait where Self: 'this;
    type AnnotationRepeated<'this>: ::puroro::RepeatedField::<::std::borrow::Cow::<'this, Self::AnnotationElement::<'this>>>
        where Self: 'this;
    fn annotation<'this>(&'this self) -> Self::AnnotationRepeated::<'this>;
}
pub mod generated_code_info {
pub trait AnnotationTrait: ::std::clone::Clone {
    type PathRepeated<'this>: ::puroro::RepeatedField::<i32>
        where Self: 'this;
    fn path<'this>(&'this self) -> Self::PathRepeated::<'this>;
    fn source_file<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>>;
    fn begin<'this>(&'this self) -> ::std::option::Option::<i32>;
    fn end<'this>(&'this self) -> ::std::option::Option::<i32>;
}
} // mod generated_code_info
pub trait SourceCodeInfoTrait: ::std::clone::Clone {
    type LocationElement<'this>: self::source_code_info::LocationTrait where Self: 'this;
    type LocationRepeated<'this>: ::puroro::RepeatedField::<::std::borrow::Cow::<'this, Self::LocationElement::<'this>>>
        where Self: 'this;
    fn location<'this>(&'this self) -> Self::LocationRepeated::<'this>;
}
pub mod source_code_info {
pub trait LocationTrait: ::std::clone::Clone {
    type PathRepeated<'this>: ::puroro::RepeatedField::<i32>
        where Self: 'this;
    fn path<'this>(&'this self) -> Self::PathRepeated::<'this>;
    type SpanRepeated<'this>: ::puroro::RepeatedField::<i32>
        where Self: 'this;
    fn span<'this>(&'this self) -> Self::SpanRepeated::<'this>;
    fn leading_comments<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>>;
    fn trailing_comments<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>>;
    type LeadingDetachedCommentsRepeated<'this>: ::puroro::RepeatedField::<::std::borrow::Cow::<'this, str>>
        where Self: 'this;
    fn leading_detached_comments<'this>(&'this self) -> Self::LeadingDetachedCommentsRepeated::<'this>;
}
} // mod source_code_info
pub trait UninterpretedOptionTrait: ::std::clone::Clone {
    type NameElement<'this>: self::uninterpreted_option::NamePartTrait where Self: 'this;
    type NameRepeated<'this>: ::puroro::RepeatedField::<::std::borrow::Cow::<'this, Self::NameElement::<'this>>>
        where Self: 'this;
    fn name<'this>(&'this self) -> Self::NameRepeated::<'this>;
    fn identifier_value<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>>;
    fn positive_int_value<'this>(&'this self) -> ::std::option::Option::<u64>;
    fn negative_int_value<'this>(&'this self) -> ::std::option::Option::<i64>;
    fn double_value<'this>(&'this self) -> ::std::option::Option::<f64>;
    fn string_value<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, [u8]>>;
    fn aggregate_value<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>>;
}
pub mod uninterpreted_option {
pub trait NamePartTrait: ::std::clone::Clone {
    fn name_part<'this>(&'this self) -> ::std::borrow::Cow::<'this, str>;
    fn is_extension<'this>(&'this self) -> bool;
}
} // mod uninterpreted_option
pub trait MethodOptionsTrait: ::std::clone::Clone {
    fn deprecated<'this>(&'this self) -> ::std::option::Option::<bool>;
    fn idempotency_level<'this>(&'this self) -> ::std::option::Option::<super::super::super::enums::google::protobuf::method_options::IdempotencyLevel>;
    type UninterpretedOptionElement<'this>: self::UninterpretedOptionTrait where Self: 'this;
    type UninterpretedOptionRepeated<'this>: ::puroro::RepeatedField::<::std::borrow::Cow::<'this, Self::UninterpretedOptionElement::<'this>>>
        where Self: 'this;
    fn uninterpreted_option<'this>(&'this self) -> Self::UninterpretedOptionRepeated::<'this>;
}
pub mod method_options {
} // mod method_options
pub trait ServiceOptionsTrait: ::std::clone::Clone {
    fn deprecated<'this>(&'this self) -> ::std::option::Option::<bool>;
    type UninterpretedOptionElement<'this>: self::UninterpretedOptionTrait where Self: 'this;
    type UninterpretedOptionRepeated<'this>: ::puroro::RepeatedField::<::std::borrow::Cow::<'this, Self::UninterpretedOptionElement::<'this>>>
        where Self: 'this;
    fn uninterpreted_option<'this>(&'this self) -> Self::UninterpretedOptionRepeated::<'this>;
}
pub trait EnumValueOptionsTrait: ::std::clone::Clone {
    fn deprecated<'this>(&'this self) -> ::std::option::Option::<bool>;
    type UninterpretedOptionElement<'this>: self::UninterpretedOptionTrait where Self: 'this;
    type UninterpretedOptionRepeated<'this>: ::puroro::RepeatedField::<::std::borrow::Cow::<'this, Self::UninterpretedOptionElement::<'this>>>
        where Self: 'this;
    fn uninterpreted_option<'this>(&'this self) -> Self::UninterpretedOptionRepeated::<'this>;
}
pub trait EnumOptionsTrait: ::std::clone::Clone {
    fn allow_alias<'this>(&'this self) -> ::std::option::Option::<bool>;
    fn deprecated<'this>(&'this self) -> ::std::option::Option::<bool>;
    type UninterpretedOptionElement<'this>: self::UninterpretedOptionTrait where Self: 'this;
    type UninterpretedOptionRepeated<'this>: ::puroro::RepeatedField::<::std::borrow::Cow::<'this, Self::UninterpretedOptionElement::<'this>>>
        where Self: 'this;
    fn uninterpreted_option<'this>(&'this self) -> Self::UninterpretedOptionRepeated::<'this>;
}
pub trait OneofOptionsTrait: ::std::clone::Clone {
    type UninterpretedOptionElement<'this>: self::UninterpretedOptionTrait where Self: 'this;
    type UninterpretedOptionRepeated<'this>: ::puroro::RepeatedField::<::std::borrow::Cow::<'this, Self::UninterpretedOptionElement::<'this>>>
        where Self: 'this;
    fn uninterpreted_option<'this>(&'this self) -> Self::UninterpretedOptionRepeated::<'this>;
}
pub trait FieldOptionsTrait: ::std::clone::Clone {
    fn ctype<'this>(&'this self) -> ::std::option::Option::<super::super::super::enums::google::protobuf::field_options::Ctype>;
    fn packed<'this>(&'this self) -> ::std::option::Option::<bool>;
    fn jstype<'this>(&'this self) -> ::std::option::Option::<super::super::super::enums::google::protobuf::field_options::Jstype>;
    fn lazy<'this>(&'this self) -> ::std::option::Option::<bool>;
    fn deprecated<'this>(&'this self) -> ::std::option::Option::<bool>;
    fn weak<'this>(&'this self) -> ::std::option::Option::<bool>;
    type UninterpretedOptionElement<'this>: self::UninterpretedOptionTrait where Self: 'this;
    type UninterpretedOptionRepeated<'this>: ::puroro::RepeatedField::<::std::borrow::Cow::<'this, Self::UninterpretedOptionElement::<'this>>>
        where Self: 'this;
    fn uninterpreted_option<'this>(&'this self) -> Self::UninterpretedOptionRepeated::<'this>;
}
pub mod field_options {
} // mod field_options
pub trait MessageOptionsTrait: ::std::clone::Clone {
    fn message_set_wire_format<'this>(&'this self) -> ::std::option::Option::<bool>;
    fn no_standard_descriptor_accessor<'this>(&'this self) -> ::std::option::Option::<bool>;
    fn deprecated<'this>(&'this self) -> ::std::option::Option::<bool>;
    fn map_entry<'this>(&'this self) -> ::std::option::Option::<bool>;
    type UninterpretedOptionElement<'this>: self::UninterpretedOptionTrait where Self: 'this;
    type UninterpretedOptionRepeated<'this>: ::puroro::RepeatedField::<::std::borrow::Cow::<'this, Self::UninterpretedOptionElement::<'this>>>
        where Self: 'this;
    fn uninterpreted_option<'this>(&'this self) -> Self::UninterpretedOptionRepeated::<'this>;
}
pub trait FileOptionsTrait: ::std::clone::Clone {
    fn java_package<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>>;
    fn java_outer_classname<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>>;
    fn java_multiple_files<'this>(&'this self) -> ::std::option::Option::<bool>;
    fn java_generate_equals_and_hash<'this>(&'this self) -> ::std::option::Option::<bool>;
    fn java_string_check_utf8<'this>(&'this self) -> ::std::option::Option::<bool>;
    fn optimize_for<'this>(&'this self) -> ::std::option::Option::<super::super::super::enums::google::protobuf::file_options::OptimizeMode>;
    fn go_package<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>>;
    fn cc_generic_services<'this>(&'this self) -> ::std::option::Option::<bool>;
    fn java_generic_services<'this>(&'this self) -> ::std::option::Option::<bool>;
    fn py_generic_services<'this>(&'this self) -> ::std::option::Option::<bool>;
    fn php_generic_services<'this>(&'this self) -> ::std::option::Option::<bool>;
    fn deprecated<'this>(&'this self) -> ::std::option::Option::<bool>;
    fn cc_enable_arenas<'this>(&'this self) -> ::std::option::Option::<bool>;
    fn objc_class_prefix<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>>;
    fn csharp_namespace<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>>;
    fn swift_prefix<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>>;
    fn php_class_prefix<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>>;
    fn php_namespace<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>>;
    fn php_metadata_namespace<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>>;
    fn ruby_package<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>>;
    type UninterpretedOptionElement<'this>: self::UninterpretedOptionTrait where Self: 'this;
    type UninterpretedOptionRepeated<'this>: ::puroro::RepeatedField::<::std::borrow::Cow::<'this, Self::UninterpretedOptionElement::<'this>>>
        where Self: 'this;
    fn uninterpreted_option<'this>(&'this self) -> Self::UninterpretedOptionRepeated::<'this>;
}
pub mod file_options {
} // mod file_options
pub trait MethodDescriptorProtoTrait: ::std::clone::Clone {
    fn name<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>>;
    fn input_type<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>>;
    fn output_type<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>>;
    type OptionsType<'this>: self::MethodOptionsTrait where Self: 'this;
    fn options<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, Self::OptionsType::<'this>>>;
    fn client_streaming<'this>(&'this self) -> ::std::option::Option::<bool>;
    fn server_streaming<'this>(&'this self) -> ::std::option::Option::<bool>;
}
pub trait ServiceDescriptorProtoTrait: ::std::clone::Clone {
    fn name<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>>;
    type MethodElement<'this>: self::MethodDescriptorProtoTrait where Self: 'this;
    type MethodRepeated<'this>: ::puroro::RepeatedField::<::std::borrow::Cow::<'this, Self::MethodElement::<'this>>>
        where Self: 'this;
    fn method<'this>(&'this self) -> Self::MethodRepeated::<'this>;
    type OptionsType<'this>: self::ServiceOptionsTrait where Self: 'this;
    fn options<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, Self::OptionsType::<'this>>>;
}
pub trait EnumValueDescriptorProtoTrait: ::std::clone::Clone {
    fn name<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>>;
    fn number<'this>(&'this self) -> ::std::option::Option::<i32>;
    type OptionsType<'this>: self::EnumValueOptionsTrait where Self: 'this;
    fn options<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, Self::OptionsType::<'this>>>;
}
pub trait EnumDescriptorProtoTrait: ::std::clone::Clone {
    fn name<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>>;
    type ValueElement<'this>: self::EnumValueDescriptorProtoTrait where Self: 'this;
    type ValueRepeated<'this>: ::puroro::RepeatedField::<::std::borrow::Cow::<'this, Self::ValueElement::<'this>>>
        where Self: 'this;
    fn value<'this>(&'this self) -> Self::ValueRepeated::<'this>;
    type OptionsType<'this>: self::EnumOptionsTrait where Self: 'this;
    fn options<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, Self::OptionsType::<'this>>>;
    type ReservedRangeElement<'this>: self::enum_descriptor_proto::EnumReservedRangeTrait where Self: 'this;
    type ReservedRangeRepeated<'this>: ::puroro::RepeatedField::<::std::borrow::Cow::<'this, Self::ReservedRangeElement::<'this>>>
        where Self: 'this;
    fn reserved_range<'this>(&'this self) -> Self::ReservedRangeRepeated::<'this>;
    type ReservedNameRepeated<'this>: ::puroro::RepeatedField::<::std::borrow::Cow::<'this, str>>
        where Self: 'this;
    fn reserved_name<'this>(&'this self) -> Self::ReservedNameRepeated::<'this>;
}
pub mod enum_descriptor_proto {
pub trait EnumReservedRangeTrait: ::std::clone::Clone {
    fn start<'this>(&'this self) -> ::std::option::Option::<i32>;
    fn end<'this>(&'this self) -> ::std::option::Option::<i32>;
}
} // mod enum_descriptor_proto
pub trait OneofDescriptorProtoTrait: ::std::clone::Clone {
    fn name<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>>;
    type OptionsType<'this>: self::OneofOptionsTrait where Self: 'this;
    fn options<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, Self::OptionsType::<'this>>>;
}
pub trait FieldDescriptorProtoTrait: ::std::clone::Clone {
    fn name<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>>;
    fn number<'this>(&'this self) -> ::std::option::Option::<i32>;
    fn label<'this>(&'this self) -> ::std::option::Option::<super::super::super::enums::google::protobuf::field_descriptor_proto::Label>;
    fn type_<'this>(&'this self) -> ::std::option::Option::<super::super::super::enums::google::protobuf::field_descriptor_proto::Type>;
    fn type_name<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>>;
    fn extendee<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>>;
    fn default_value<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>>;
    fn oneof_index<'this>(&'this self) -> ::std::option::Option::<i32>;
    fn json_name<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>>;
    type OptionsType<'this>: self::FieldOptionsTrait where Self: 'this;
    fn options<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, Self::OptionsType::<'this>>>;
    fn proto3_optional<'this>(&'this self) -> ::std::option::Option::<bool>;
}
pub mod field_descriptor_proto {
} // mod field_descriptor_proto
pub trait ExtensionRangeOptionsTrait: ::std::clone::Clone {
    type UninterpretedOptionElement<'this>: self::UninterpretedOptionTrait where Self: 'this;
    type UninterpretedOptionRepeated<'this>: ::puroro::RepeatedField::<::std::borrow::Cow::<'this, Self::UninterpretedOptionElement::<'this>>>
        where Self: 'this;
    fn uninterpreted_option<'this>(&'this self) -> Self::UninterpretedOptionRepeated::<'this>;
}
pub trait DescriptorProtoTrait: ::std::clone::Clone {
    fn name<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>>;
    type FieldElement<'this>: self::FieldDescriptorProtoTrait where Self: 'this;
    type FieldRepeated<'this>: ::puroro::RepeatedField::<::std::borrow::Cow::<'this, Self::FieldElement::<'this>>>
        where Self: 'this;
    fn field<'this>(&'this self) -> Self::FieldRepeated::<'this>;
    type ExtensionElement<'this>: self::FieldDescriptorProtoTrait where Self: 'this;
    type ExtensionRepeated<'this>: ::puroro::RepeatedField::<::std::borrow::Cow::<'this, Self::ExtensionElement::<'this>>>
        where Self: 'this;
    fn extension<'this>(&'this self) -> Self::ExtensionRepeated::<'this>;
    type NestedTypeElement<'this>: self::DescriptorProtoTrait where Self: 'this;
    type NestedTypeRepeated<'this>: ::puroro::RepeatedField::<::std::borrow::Cow::<'this, Self::NestedTypeElement::<'this>>>
        where Self: 'this;
    fn nested_type<'this>(&'this self) -> Self::NestedTypeRepeated::<'this>;
    type EnumTypeElement<'this>: self::EnumDescriptorProtoTrait where Self: 'this;
    type EnumTypeRepeated<'this>: ::puroro::RepeatedField::<::std::borrow::Cow::<'this, Self::EnumTypeElement::<'this>>>
        where Self: 'this;
    fn enum_type<'this>(&'this self) -> Self::EnumTypeRepeated::<'this>;
    type ExtensionRangeElement<'this>: self::descriptor_proto::ExtensionRangeTrait where Self: 'this;
    type ExtensionRangeRepeated<'this>: ::puroro::RepeatedField::<::std::borrow::Cow::<'this, Self::ExtensionRangeElement::<'this>>>
        where Self: 'this;
    fn extension_range<'this>(&'this self) -> Self::ExtensionRangeRepeated::<'this>;
    type OneofDeclElement<'this>: self::OneofDescriptorProtoTrait where Self: 'this;
    type OneofDeclRepeated<'this>: ::puroro::RepeatedField::<::std::borrow::Cow::<'this, Self::OneofDeclElement::<'this>>>
        where Self: 'this;
    fn oneof_decl<'this>(&'this self) -> Self::OneofDeclRepeated::<'this>;
    type OptionsType<'this>: self::MessageOptionsTrait where Self: 'this;
    fn options<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, Self::OptionsType::<'this>>>;
    type ReservedRangeElement<'this>: self::descriptor_proto::ReservedRangeTrait where Self: 'this;
    type ReservedRangeRepeated<'this>: ::puroro::RepeatedField::<::std::borrow::Cow::<'this, Self::ReservedRangeElement::<'this>>>
        where Self: 'this;
    fn reserved_range<'this>(&'this self) -> Self::ReservedRangeRepeated::<'this>;
    type ReservedNameRepeated<'this>: ::puroro::RepeatedField::<::std::borrow::Cow::<'this, str>>
        where Self: 'this;
    fn reserved_name<'this>(&'this self) -> Self::ReservedNameRepeated::<'this>;
}
pub mod descriptor_proto {
pub trait ReservedRangeTrait: ::std::clone::Clone {
    fn start<'this>(&'this self) -> ::std::option::Option::<i32>;
    fn end<'this>(&'this self) -> ::std::option::Option::<i32>;
}
pub trait ExtensionRangeTrait: ::std::clone::Clone {
    fn start<'this>(&'this self) -> ::std::option::Option::<i32>;
    fn end<'this>(&'this self) -> ::std::option::Option::<i32>;
    type OptionsType<'this>: super::ExtensionRangeOptionsTrait where Self: 'this;
    fn options<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, Self::OptionsType::<'this>>>;
}
} // mod descriptor_proto
pub trait FileDescriptorProtoTrait: ::std::clone::Clone {
    fn name<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>>;
    fn package<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>>;
    type DependencyRepeated<'this>: ::puroro::RepeatedField::<::std::borrow::Cow::<'this, str>>
        where Self: 'this;
    fn dependency<'this>(&'this self) -> Self::DependencyRepeated::<'this>;
    type PublicDependencyRepeated<'this>: ::puroro::RepeatedField::<i32>
        where Self: 'this;
    fn public_dependency<'this>(&'this self) -> Self::PublicDependencyRepeated::<'this>;
    type WeakDependencyRepeated<'this>: ::puroro::RepeatedField::<i32>
        where Self: 'this;
    fn weak_dependency<'this>(&'this self) -> Self::WeakDependencyRepeated::<'this>;
    type MessageTypeElement<'this>: self::DescriptorProtoTrait where Self: 'this;
    type MessageTypeRepeated<'this>: ::puroro::RepeatedField::<::std::borrow::Cow::<'this, Self::MessageTypeElement::<'this>>>
        where Self: 'this;
    fn message_type<'this>(&'this self) -> Self::MessageTypeRepeated::<'this>;
    type EnumTypeElement<'this>: self::EnumDescriptorProtoTrait where Self: 'this;
    type EnumTypeRepeated<'this>: ::puroro::RepeatedField::<::std::borrow::Cow::<'this, Self::EnumTypeElement::<'this>>>
        where Self: 'this;
    fn enum_type<'this>(&'this self) -> Self::EnumTypeRepeated::<'this>;
    type ServiceElement<'this>: self::ServiceDescriptorProtoTrait where Self: 'this;
    type ServiceRepeated<'this>: ::puroro::RepeatedField::<::std::borrow::Cow::<'this, Self::ServiceElement::<'this>>>
        where Self: 'this;
    fn service<'this>(&'this self) -> Self::ServiceRepeated::<'this>;
    type ExtensionElement<'this>: self::FieldDescriptorProtoTrait where Self: 'this;
    type ExtensionRepeated<'this>: ::puroro::RepeatedField::<::std::borrow::Cow::<'this, Self::ExtensionElement::<'this>>>
        where Self: 'this;
    fn extension<'this>(&'this self) -> Self::ExtensionRepeated::<'this>;
    type OptionsType<'this>: self::FileOptionsTrait where Self: 'this;
    fn options<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, Self::OptionsType::<'this>>>;
    type SourceCodeInfoType<'this>: self::SourceCodeInfoTrait where Self: 'this;
    fn source_code_info<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, Self::SourceCodeInfoType::<'this>>>;
    fn syntax<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>>;
}
pub trait FileDescriptorSetTrait: ::std::clone::Clone {
    type FileElement<'this>: self::FileDescriptorProtoTrait where Self: 'this;
    type FileRepeated<'this>: ::puroro::RepeatedField::<::std::borrow::Cow::<'this, Self::FileElement::<'this>>>
        where Self: 'this;
    fn file<'this>(&'this self) -> Self::FileRepeated::<'this>;
}

pub mod compiler;
