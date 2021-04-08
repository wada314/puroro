pub struct GeneratedCodeInfo {
    annotation: ::std::vec::Vec<super::super::google::protobuf::generated_code_info::Annotation>,
}
mod generated_code_info {
    pub struct Annotation {
        path: ::std::vec::Vec<i32>,
        source_file: String,
        begin: i32,
        end: i32,
    }
}
pub struct SourceCodeInfo {
    location: ::std::vec::Vec<super::super::google::protobuf::source_code_info::Location>,
}
mod source_code_info {
    pub struct Location {
        path: ::std::vec::Vec<i32>,
        span: ::std::vec::Vec<i32>,
        leading_comments: String,
        trailing_comments: String,
        leading_detached_comments: ::std::vec::Vec<String>,
    }
}
pub struct UninterpretedOption {
    name: ::std::vec::Vec<super::super::google::protobuf::uninterpreted_option::NamePart>,
    identifier_value: String,
    positive_int_value: u64,
    negative_int_value: i64,
    double_value: f64,
    string_value: ::std::vec::Vec<u8>,
    aggregate_value: String,
}
mod uninterpreted_option {
    pub struct NamePart {
        name_part: String,
        is_extension: bool,
    }
}
pub struct MethodOptions {
    deprecated: bool,
    idempotency_level: ::std::result::Result<super::super::google::protobuf::method_options::IdempotencyLevel, i32>,
    uninterpreted_option: ::std::vec::Vec<super::super::google::protobuf::UninterpretedOption>,
}
mod method_options {
    pub enum IdempotencyLevel {
        IdempotencyUnknown = 0,
        NoSideEffects = 1,
        Idempotent = 2,
    }
}
pub struct ServiceOptions {
    deprecated: bool,
    uninterpreted_option: ::std::vec::Vec<super::super::google::protobuf::UninterpretedOption>,
}
pub struct EnumValueOptions {
    deprecated: bool,
    uninterpreted_option: ::std::vec::Vec<super::super::google::protobuf::UninterpretedOption>,
}
pub struct EnumOptions {
    allow_alias: bool,
    deprecated: bool,
    uninterpreted_option: ::std::vec::Vec<super::super::google::protobuf::UninterpretedOption>,
}
pub struct OneofOptions {
    uninterpreted_option: ::std::vec::Vec<super::super::google::protobuf::UninterpretedOption>,
}
pub struct FieldOptions {
    ctype: ::std::result::Result<super::super::google::protobuf::field_options::CType, i32>,
    packed: bool,
    jstype: ::std::result::Result<super::super::google::protobuf::field_options::JSType, i32>,
    lazy: bool,
    deprecated: bool,
    weak: bool,
    uninterpreted_option: ::std::vec::Vec<super::super::google::protobuf::UninterpretedOption>,
}
mod field_options {
    pub enum JSType {
        JsNormal = 0,
        JsString = 1,
        JsNumber = 2,
    }
    pub enum CType {
        String = 0,
        Cord = 1,
        StringPiece = 2,
    }
}
pub struct MessageOptions {
    message_set_wire_format: bool,
    no_standard_descriptor_accessor: bool,
    deprecated: bool,
    map_entry: bool,
    uninterpreted_option: ::std::vec::Vec<super::super::google::protobuf::UninterpretedOption>,
}
pub struct FileOptions {
    java_package: String,
    java_outer_classname: String,
    java_multiple_files: bool,
    java_generate_equals_and_hash: bool,
    java_string_check_utf8: bool,
    optimize_for: ::std::result::Result<super::super::google::protobuf::file_options::OptimizeMode, i32>,
    go_package: String,
    cc_generic_services: bool,
    java_generic_services: bool,
    py_generic_services: bool,
    php_generic_services: bool,
    deprecated: bool,
    cc_enable_arenas: bool,
    objc_class_prefix: String,
    csharp_namespace: String,
    swift_prefix: String,
    php_class_prefix: String,
    php_namespace: String,
    php_metadata_namespace: String,
    ruby_package: String,
    uninterpreted_option: ::std::vec::Vec<super::super::google::protobuf::UninterpretedOption>,
}
mod file_options {
    pub enum OptimizeMode {
        Speed = 1,
        CodeSize = 2,
        LiteRuntime = 3,
    }
}
pub struct MethodDescriptorProto {
    name: String,
    input_type: String,
    output_type: String,
    options: ::std::option::Option<::std::boxed::Box<super::super::google::protobuf::MethodOptions>>,
    client_streaming: bool,
    server_streaming: bool,
}
pub struct ServiceDescriptorProto {
    name: String,
    method: ::std::vec::Vec<super::super::google::protobuf::MethodDescriptorProto>,
    options: ::std::option::Option<::std::boxed::Box<super::super::google::protobuf::ServiceOptions>>,
}
pub struct EnumValueDescriptorProto {
    name: String,
    number: i32,
    options: ::std::option::Option<::std::boxed::Box<super::super::google::protobuf::EnumValueOptions>>,
}
pub struct EnumDescriptorProto {
    name: String,
    value: ::std::vec::Vec<super::super::google::protobuf::EnumValueDescriptorProto>,
    options: ::std::option::Option<::std::boxed::Box<super::super::google::protobuf::EnumOptions>>,
    reserved_range: ::std::vec::Vec<super::super::google::protobuf::enum_descriptor_proto::EnumReservedRange>,
    reserved_name: ::std::vec::Vec<String>,
}
mod enum_descriptor_proto {
    pub struct EnumReservedRange {
        start: i32,
        end: i32,
    }
}
pub struct OneofDescriptorProto {
    name: String,
    options: ::std::option::Option<::std::boxed::Box<super::super::google::protobuf::OneofOptions>>,
}
pub struct FieldDescriptorProto {
    name: String,
    number: i32,
    label: ::std::result::Result<super::super::google::protobuf::field_descriptor_proto::Label, i32>,
    type_: ::std::result::Result<super::super::google::protobuf::field_descriptor_proto::Type, i32>,
    type_name: String,
    extendee: String,
    default_value: String,
    oneof_index: i32,
    json_name: String,
    options: ::std::option::Option<::std::boxed::Box<super::super::google::protobuf::FieldOptions>>,
    proto3_optional: bool,
}
mod field_descriptor_proto {
    pub enum Label {
        LabelOptional = 1,
        LabelRequired = 2,
        LabelRepeated = 3,
    }
    pub enum Type {
        TypeDouble = 1,
        TypeFloat = 2,
        TypeInt64 = 3,
        TypeUint64 = 4,
        TypeInt32 = 5,
        TypeFixed64 = 6,
        TypeFixed32 = 7,
        TypeBool = 8,
        TypeString = 9,
        TypeGroup = 10,
        TypeMessage = 11,
        TypeBytes = 12,
        TypeUint32 = 13,
        TypeEnum = 14,
        TypeSfixed32 = 15,
        TypeSfixed64 = 16,
        TypeSint32 = 17,
        TypeSint64 = 18,
    }
}
pub struct ExtensionRangeOptions {
    uninterpreted_option: ::std::vec::Vec<super::super::google::protobuf::UninterpretedOption>,
}
pub struct DescriptorProto {
    name: String,
    field: ::std::vec::Vec<super::super::google::protobuf::FieldDescriptorProto>,
    extension: ::std::vec::Vec<super::super::google::protobuf::FieldDescriptorProto>,
    nested_type: ::std::vec::Vec<super::super::google::protobuf::DescriptorProto>,
    enum_type: ::std::vec::Vec<super::super::google::protobuf::EnumDescriptorProto>,
    extension_range: ::std::vec::Vec<super::super::google::protobuf::descriptor_proto::ExtensionRange>,
    oneof_decl: ::std::vec::Vec<super::super::google::protobuf::OneofDescriptorProto>,
    options: ::std::option::Option<::std::boxed::Box<super::super::google::protobuf::MessageOptions>>,
    reserved_range: ::std::vec::Vec<super::super::google::protobuf::descriptor_proto::ReservedRange>,
    reserved_name: ::std::vec::Vec<String>,
}
mod descriptor_proto {
    pub struct ReservedRange {
        start: i32,
        end: i32,
    }
    pub struct ExtensionRange {
        start: i32,
        end: i32,
        options: ::std::option::Option<::std::boxed::Box<super::super::google::protobuf::ExtensionRangeOptions>>,
    }
}
pub struct FileDescriptorProto {
    name: String,
    package: String,
    dependency: ::std::vec::Vec<String>,
    public_dependency: ::std::vec::Vec<i32>,
    weak_dependency: ::std::vec::Vec<i32>,
    message_type: ::std::vec::Vec<super::super::google::protobuf::DescriptorProto>,
    enum_type: ::std::vec::Vec<super::super::google::protobuf::EnumDescriptorProto>,
    service: ::std::vec::Vec<super::super::google::protobuf::ServiceDescriptorProto>,
    extension: ::std::vec::Vec<super::super::google::protobuf::FieldDescriptorProto>,
    options: ::std::option::Option<::std::boxed::Box<super::super::google::protobuf::FileOptions>>,
    source_code_info: ::std::option::Option<::std::boxed::Box<super::super::google::protobuf::SourceCodeInfo>>,
    syntax: String,
}
pub struct FileDescriptorSet {
    file: ::std::vec::Vec<super::super::google::protobuf::FileDescriptorProto>,
}
