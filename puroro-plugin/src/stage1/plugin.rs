pub(crate) mod code_generator_response {
    proto_struct! {
        struct File {
            name: String = 1,
            insertion_point: String = 2,
            content: String = 15,
            generated_code_info: Option<super::GeneratedCodeInfo> = 16,
        }
    }
}
pub(crate) mod field_descriptor_proto {
    proto_struct! {
    enum Type {
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
    enum Label {
        LabelOptional = 1,
        LabelRequired = 2,
        LabelRepeated = 3,
    }
    }
}
proto_struct! {
    // From plugin.proto
    struct Version {
        major: i32 = 1,
        minor: i32 = 2,
        patch: i32 = 3,
        suffix: String = 4,
    }
    struct CodeGeneratorRequest {
        file_to_generate: Vec<String> = 1,
        parameter: String = 2,
        proto_file: Vec<FileDescriptorProto> = 15,
        compiler_version: Option<Version> = 3,
    }
    struct CodeGeneratorResponse {
        error: String = 1,
        supported_features: u64 = 2,
        file: Vec<code_generator_response::File> = 15,
    }
    enum Feature {
        FEATURE_NONE = 0,
        FEATURE_PROTO3_OPTIONAL = 1,
    }

    // From descriptor.proto
    struct FileDescriptorProto {
        name: String = 1,
        package: String = 2,
        dependency: Vec<String> = 3,
        public_dependency: Vec<i32> = 10,
        weak_dependency: Vec<i32> = 11,
        message_type: Vec<DescriptorProto> = 4,
        enum_type: Vec<EnumDescriptorProto> = 5,
        extension: Vec<FieldDescriptorProto> = 7,
        options: Option<FileOptions> = 8,
        source_code_info: Option<SourceCodeInfo> = 9,
        syntax: String = 12,
    }
    struct DescriptorProto {
        name: String = 1,
        field: Vec<FieldDescriptorProto> = 2,
        extension: Vec<FieldDescriptorProto> = 6,
        nested_type: Vec<DescriptorProto> = 3,
        enum_type: Vec<EnumDescriptorProto> = 4,
        oneof_decl: Vec<OneofDescriptorProto> = 8,
        options: Option<MessageOptions> = 7,
        reserved_name: Vec<String> = 10,
    }
    struct ExtensionRangeOptions {
        uninterpreted_option: Vec<UninterpretedOption> = 999,
        // extensions 1000 to max;
    }
    struct FieldDescriptorProto {
        name: String = 1,
        number: i32 = 3,
        label: Result<field_descriptor_proto::Label, i32> = 4,
        type_: Result<field_descriptor_proto::Type, i32> = 5,
        type_name: String = 6,
        extendee: String = 2,
        default_value: String = 7,
        oneof_index: i32 = 9,
        json_name: String = 10,
        options: Option<FieldOptions> = 8,
        proto3_optional: bool = 17,
    }
    struct OneofDescriptorProto {
        name: String = 1,
        options: Option<OneofOptions> = 2,
    }
    struct EnumDescriptorProto {
        name: String = 1,
        value: Vec<EnumValueDescriptorProto> = 2,
        options: Option<EnumOptions> = 3,
    }
    struct EnumDescriptorProto_EnumReservedRange {
        start: i32 = 1,
        end: i32 = 2,
    }
    struct EnumValueDescriptorProto {
        name: String = 1,
        number: i32 = 2,
        options: Option<EnumValueOptions> = 3,
    }
    struct FileOptions {
        uninterpreted_option: Vec<UninterpretedOption> = 999,
        // extensions 1000 to max;
    }
    struct MessageOptions {
        map_entry: bool = 7,
        uninterpreted_option: Vec<UninterpretedOption> = 999,
        // extensions 1000 to max;
    }
    struct FieldOptions {
        packed: bool = 2,
        uninterpreted_option: Vec<UninterpretedOption> = 999,
        // extensions 1000 to max;
    }
    struct OneofOptions {
        uninterpreted_option: Vec<UninterpretedOption> = 999,
        // extensions 1000 to max;
    }
    struct EnumOptions {
        uninterpreted_option: Vec<UninterpretedOption> = 999,
        // extensions 1000 to max;
    }
    struct EnumValueOptions {
        uninterpreted_option: Vec<UninterpretedOption> = 999,
        // extensions 1000 to max;
    }
    struct UninterpretedOption {
    }
    struct SourceCodeInfo {
        location: Vec<SourceCodeInfo_Location> = 1,
    }
    struct SourceCodeInfo_Location {
        path: Vec<i32> = 1,
        span: Vec<i32> = 2,
        leading_comments: String = 3,
        trailing_comments: String = 4,
        leading_detached_comments: Vec<String> = 6,
    }
    struct GeneratedCodeInfo {
        annotation: Vec<GeneratedCodeInfo_Annotation> = 1,
    }
    struct GeneratedCodeInfo_Annotation {
        path: Vec<i32> = 1,
        source_file: String = 2,
        begin: i32 = 3,
        end: i32 = 4,
    }
}
