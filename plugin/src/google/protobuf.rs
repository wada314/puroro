pub struct GeneratedCodeInfo {
    annotation: ::std::vec::Vec<super::super::google::protobuf::generated_code_info::Annotation>,
}
impl ::std::default::Default for GeneratedCodeInfo {
    fn default() -> Self {
        Self {
            annotation: std::default::Default::default(),
        }
    }
}
mod generated_code_info {
    pub struct Annotation {
        path: ::std::vec::Vec<i32>,
        source_file: String,
        begin: i32,
        end: i32,
    }
    impl ::std::default::Default for Annotation {
        fn default() -> Self {
            Self {
                path: std::default::Default::default(),
                source_file: std::default::Default::default(),
                begin: std::default::Default::default(),
                end: std::default::Default::default(),
            }
        }
    }
}
pub struct SourceCodeInfo {
    location: ::std::vec::Vec<super::super::google::protobuf::source_code_info::Location>,
}
impl ::std::default::Default for SourceCodeInfo {
    fn default() -> Self {
        Self {
            location: std::default::Default::default(),
        }
    }
}
mod source_code_info {
    pub struct Location {
        path: ::std::vec::Vec<i32>,
        span: ::std::vec::Vec<i32>,
        leading_comments: String,
        trailing_comments: String,
        leading_detached_comments: ::std::vec::Vec<String>,
    }
    impl ::std::default::Default for Location {
        fn default() -> Self {
            Self {
                path: std::default::Default::default(),
                span: std::default::Default::default(),
                leading_comments: std::default::Default::default(),
                trailing_comments: std::default::Default::default(),
                leading_detached_comments: std::default::Default::default(),
            }
        }
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
impl ::std::default::Default for UninterpretedOption {
    fn default() -> Self {
        Self {
            name: std::default::Default::default(),
            identifier_value: std::default::Default::default(),
            positive_int_value: std::default::Default::default(),
            negative_int_value: std::default::Default::default(),
            double_value: std::default::Default::default(),
            string_value: std::default::Default::default(),
            aggregate_value: std::default::Default::default(),
        }
    }
}
mod uninterpreted_option {
    pub struct NamePart {
        name_part: String,
        is_extension: bool,
    }
    impl ::std::default::Default for NamePart {
        fn default() -> Self {
            Self {
                name_part: std::default::Default::default(),
                is_extension: std::default::Default::default(),
            }
        }
    }
}
pub struct MethodOptions {
    deprecated: bool,
    idempotency_level: ::std::result::Result<super::super::google::protobuf::method_options::IdempotencyLevel, i32>,
    uninterpreted_option: ::std::vec::Vec<super::super::google::protobuf::UninterpretedOption>,
}
impl ::std::default::Default for MethodOptions {
    fn default() -> Self {
        Self {
            deprecated: std::default::Default::default(),
            idempotency_level: 0i32.try_into(),
            uninterpreted_option: std::default::Default::default(),
        }
    }
}
mod method_options {
    pub enum IdempotencyLevel {
        IdempotencyUnknown = 0,
        NoSideEffects = 1,
        Idempotent = 2,
    }
    impl std::convert::TryFrom<i32> for IdempotencyLevel {
        type Error = i32; fn try_from(val: i32) -> std::result::Result<Self, i32> {
            match val {
                0 => Ok(Self::IdempotencyUnknown),
                1 => Ok(Self::NoSideEffects),
                2 => Ok(Self::Idempotent),
                x => Err(x),
            }
        }
    }
}
pub struct ServiceOptions {
    deprecated: bool,
    uninterpreted_option: ::std::vec::Vec<super::super::google::protobuf::UninterpretedOption>,
}
impl ::std::default::Default for ServiceOptions {
    fn default() -> Self {
        Self {
            deprecated: std::default::Default::default(),
            uninterpreted_option: std::default::Default::default(),
        }
    }
}
pub struct EnumValueOptions {
    deprecated: bool,
    uninterpreted_option: ::std::vec::Vec<super::super::google::protobuf::UninterpretedOption>,
}
impl ::std::default::Default for EnumValueOptions {
    fn default() -> Self {
        Self {
            deprecated: std::default::Default::default(),
            uninterpreted_option: std::default::Default::default(),
        }
    }
}
pub struct EnumOptions {
    allow_alias: bool,
    deprecated: bool,
    uninterpreted_option: ::std::vec::Vec<super::super::google::protobuf::UninterpretedOption>,
}
impl ::std::default::Default for EnumOptions {
    fn default() -> Self {
        Self {
            allow_alias: std::default::Default::default(),
            deprecated: std::default::Default::default(),
            uninterpreted_option: std::default::Default::default(),
        }
    }
}
pub struct OneofOptions {
    uninterpreted_option: ::std::vec::Vec<super::super::google::protobuf::UninterpretedOption>,
}
impl ::std::default::Default for OneofOptions {
    fn default() -> Self {
        Self {
            uninterpreted_option: std::default::Default::default(),
        }
    }
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
impl ::std::default::Default for FieldOptions {
    fn default() -> Self {
        Self {
            ctype: 0i32.try_into(),
            packed: std::default::Default::default(),
            jstype: 0i32.try_into(),
            lazy: std::default::Default::default(),
            deprecated: std::default::Default::default(),
            weak: std::default::Default::default(),
            uninterpreted_option: std::default::Default::default(),
        }
    }
}
mod field_options {
    pub enum JSType {
        JsNormal = 0,
        JsString = 1,
        JsNumber = 2,
    }
    impl std::convert::TryFrom<i32> for JSType {
        type Error = i32; fn try_from(val: i32) -> std::result::Result<Self, i32> {
            match val {
                0 => Ok(Self::JsNormal),
                1 => Ok(Self::JsString),
                2 => Ok(Self::JsNumber),
                x => Err(x),
            }
        }
    }
    pub enum CType {
        String = 0,
        Cord = 1,
        StringPiece = 2,
    }
    impl std::convert::TryFrom<i32> for CType {
        type Error = i32; fn try_from(val: i32) -> std::result::Result<Self, i32> {
            match val {
                0 => Ok(Self::String),
                1 => Ok(Self::Cord),
                2 => Ok(Self::StringPiece),
                x => Err(x),
            }
        }
    }
}
pub struct MessageOptions {
    message_set_wire_format: bool,
    no_standard_descriptor_accessor: bool,
    deprecated: bool,
    map_entry: bool,
    uninterpreted_option: ::std::vec::Vec<super::super::google::protobuf::UninterpretedOption>,
}
impl ::std::default::Default for MessageOptions {
    fn default() -> Self {
        Self {
            message_set_wire_format: std::default::Default::default(),
            no_standard_descriptor_accessor: std::default::Default::default(),
            deprecated: std::default::Default::default(),
            map_entry: std::default::Default::default(),
            uninterpreted_option: std::default::Default::default(),
        }
    }
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
impl ::std::default::Default for FileOptions {
    fn default() -> Self {
        Self {
            java_package: std::default::Default::default(),
            java_outer_classname: std::default::Default::default(),
            java_multiple_files: std::default::Default::default(),
            java_generate_equals_and_hash: std::default::Default::default(),
            java_string_check_utf8: std::default::Default::default(),
            optimize_for: 0i32.try_into(),
            go_package: std::default::Default::default(),
            cc_generic_services: std::default::Default::default(),
            java_generic_services: std::default::Default::default(),
            py_generic_services: std::default::Default::default(),
            php_generic_services: std::default::Default::default(),
            deprecated: std::default::Default::default(),
            cc_enable_arenas: std::default::Default::default(),
            objc_class_prefix: std::default::Default::default(),
            csharp_namespace: std::default::Default::default(),
            swift_prefix: std::default::Default::default(),
            php_class_prefix: std::default::Default::default(),
            php_namespace: std::default::Default::default(),
            php_metadata_namespace: std::default::Default::default(),
            ruby_package: std::default::Default::default(),
            uninterpreted_option: std::default::Default::default(),
        }
    }
}
mod file_options {
    pub enum OptimizeMode {
        Speed = 1,
        CodeSize = 2,
        LiteRuntime = 3,
    }
    impl std::convert::TryFrom<i32> for OptimizeMode {
        type Error = i32; fn try_from(val: i32) -> std::result::Result<Self, i32> {
            match val {
                1 => Ok(Self::Speed),
                2 => Ok(Self::CodeSize),
                3 => Ok(Self::LiteRuntime),
                x => Err(x),
            }
        }
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
impl ::std::default::Default for MethodDescriptorProto {
    fn default() -> Self {
        Self {
            name: std::default::Default::default(),
            input_type: std::default::Default::default(),
            output_type: std::default::Default::default(),
            options: std::default::Default::default(),
            client_streaming: std::default::Default::default(),
            server_streaming: std::default::Default::default(),
        }
    }
}
pub struct ServiceDescriptorProto {
    name: String,
    method: ::std::vec::Vec<super::super::google::protobuf::MethodDescriptorProto>,
    options: ::std::option::Option<::std::boxed::Box<super::super::google::protobuf::ServiceOptions>>,
}
impl ::std::default::Default for ServiceDescriptorProto {
    fn default() -> Self {
        Self {
            name: std::default::Default::default(),
            method: std::default::Default::default(),
            options: std::default::Default::default(),
        }
    }
}
pub struct EnumValueDescriptorProto {
    name: String,
    number: i32,
    options: ::std::option::Option<::std::boxed::Box<super::super::google::protobuf::EnumValueOptions>>,
}
impl ::std::default::Default for EnumValueDescriptorProto {
    fn default() -> Self {
        Self {
            name: std::default::Default::default(),
            number: std::default::Default::default(),
            options: std::default::Default::default(),
        }
    }
}
pub struct EnumDescriptorProto {
    name: String,
    value: ::std::vec::Vec<super::super::google::protobuf::EnumValueDescriptorProto>,
    options: ::std::option::Option<::std::boxed::Box<super::super::google::protobuf::EnumOptions>>,
    reserved_range: ::std::vec::Vec<super::super::google::protobuf::enum_descriptor_proto::EnumReservedRange>,
    reserved_name: ::std::vec::Vec<String>,
}
impl ::std::default::Default for EnumDescriptorProto {
    fn default() -> Self {
        Self {
            name: std::default::Default::default(),
            value: std::default::Default::default(),
            options: std::default::Default::default(),
            reserved_range: std::default::Default::default(),
            reserved_name: std::default::Default::default(),
        }
    }
}
mod enum_descriptor_proto {
    pub struct EnumReservedRange {
        start: i32,
        end: i32,
    }
    impl ::std::default::Default for EnumReservedRange {
        fn default() -> Self {
            Self {
                start: std::default::Default::default(),
                end: std::default::Default::default(),
            }
        }
    }
}
pub struct OneofDescriptorProto {
    name: String,
    options: ::std::option::Option<::std::boxed::Box<super::super::google::protobuf::OneofOptions>>,
}
impl ::std::default::Default for OneofDescriptorProto {
    fn default() -> Self {
        Self {
            name: std::default::Default::default(),
            options: std::default::Default::default(),
        }
    }
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
impl ::std::default::Default for FieldDescriptorProto {
    fn default() -> Self {
        Self {
            name: std::default::Default::default(),
            number: std::default::Default::default(),
            label: 0i32.try_into(),
            type_: 0i32.try_into(),
            type_name: std::default::Default::default(),
            extendee: std::default::Default::default(),
            default_value: std::default::Default::default(),
            oneof_index: std::default::Default::default(),
            json_name: std::default::Default::default(),
            options: std::default::Default::default(),
            proto3_optional: std::default::Default::default(),
        }
    }
}
mod field_descriptor_proto {
    pub enum Label {
        LabelOptional = 1,
        LabelRequired = 2,
        LabelRepeated = 3,
    }
    impl std::convert::TryFrom<i32> for Label {
        type Error = i32; fn try_from(val: i32) -> std::result::Result<Self, i32> {
            match val {
                1 => Ok(Self::LabelOptional),
                2 => Ok(Self::LabelRequired),
                3 => Ok(Self::LabelRepeated),
                x => Err(x),
            }
        }
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
    impl std::convert::TryFrom<i32> for Type {
        type Error = i32; fn try_from(val: i32) -> std::result::Result<Self, i32> {
            match val {
                1 => Ok(Self::TypeDouble),
                2 => Ok(Self::TypeFloat),
                3 => Ok(Self::TypeInt64),
                4 => Ok(Self::TypeUint64),
                5 => Ok(Self::TypeInt32),
                6 => Ok(Self::TypeFixed64),
                7 => Ok(Self::TypeFixed32),
                8 => Ok(Self::TypeBool),
                9 => Ok(Self::TypeString),
                10 => Ok(Self::TypeGroup),
                11 => Ok(Self::TypeMessage),
                12 => Ok(Self::TypeBytes),
                13 => Ok(Self::TypeUint32),
                14 => Ok(Self::TypeEnum),
                15 => Ok(Self::TypeSfixed32),
                16 => Ok(Self::TypeSfixed64),
                17 => Ok(Self::TypeSint32),
                18 => Ok(Self::TypeSint64),
                x => Err(x),
            }
        }
    }
}
pub struct ExtensionRangeOptions {
    uninterpreted_option: ::std::vec::Vec<super::super::google::protobuf::UninterpretedOption>,
}
impl ::std::default::Default for ExtensionRangeOptions {
    fn default() -> Self {
        Self {
            uninterpreted_option: std::default::Default::default(),
        }
    }
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
impl ::std::default::Default for DescriptorProto {
    fn default() -> Self {
        Self {
            name: std::default::Default::default(),
            field: std::default::Default::default(),
            extension: std::default::Default::default(),
            nested_type: std::default::Default::default(),
            enum_type: std::default::Default::default(),
            extension_range: std::default::Default::default(),
            oneof_decl: std::default::Default::default(),
            options: std::default::Default::default(),
            reserved_range: std::default::Default::default(),
            reserved_name: std::default::Default::default(),
        }
    }
}
mod descriptor_proto {
    pub struct ReservedRange {
        start: i32,
        end: i32,
    }
    impl ::std::default::Default for ReservedRange {
        fn default() -> Self {
            Self {
                start: std::default::Default::default(),
                end: std::default::Default::default(),
            }
        }
    }
    pub struct ExtensionRange {
        start: i32,
        end: i32,
        options: ::std::option::Option<::std::boxed::Box<super::super::google::protobuf::ExtensionRangeOptions>>,
    }
    impl ::std::default::Default for ExtensionRange {
        fn default() -> Self {
            Self {
                start: std::default::Default::default(),
                end: std::default::Default::default(),
                options: std::default::Default::default(),
            }
        }
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
impl ::std::default::Default for FileDescriptorProto {
    fn default() -> Self {
        Self {
            name: std::default::Default::default(),
            package: std::default::Default::default(),
            dependency: std::default::Default::default(),
            public_dependency: std::default::Default::default(),
            weak_dependency: std::default::Default::default(),
            message_type: std::default::Default::default(),
            enum_type: std::default::Default::default(),
            service: std::default::Default::default(),
            extension: std::default::Default::default(),
            options: std::default::Default::default(),
            source_code_info: std::default::Default::default(),
            syntax: std::default::Default::default(),
        }
    }
}
pub struct FileDescriptorSet {
    file: ::std::vec::Vec<super::super::google::protobuf::FileDescriptorProto>,
}
impl ::std::default::Default for FileDescriptorSet {
    fn default() -> Self {
        Self {
            file: std::default::Default::default(),
        }
    }
}
