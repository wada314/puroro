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
        file: Vec<CodeGeneratorResponse_File> = 15,
    }
    struct CodeGeneratorResponse_File {
        name: String = 1,
        insertion_point: String = 2,
        content: String = 15,
        generated_code_info: Option<GeneratedCodeInfo> = 16,
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
        extension_range: Vec<DescriptorProto_ExtensionRange> = 5,
        oneof_decl: Vec<OneofDescriptorProto> = 8,
        options: Option<MessageOptions> = 7,
        reserved_range: Vec<DescriptorProto_ReservedRange> = 9,
        reserved_name: Vec<String> = 10,
    }
    struct DescriptorProto_ExtensionRange {
        start: i32 = 1,
        end: i32 = 2,
        options: Option<ExtensionRangeOptions> = 3,
    }
    struct DescriptorProto_ReservedRange {
        start: i32 = 1,
        end: i32 = 2,
    }
    struct ExtensionRangeOptions {
        uninterpreted_option: Vec<UninterpretedOption> = 999,
        // extensions 1000 to max;
    }
    struct FieldDescriptorProto {
        name: String = 1,
        number: i32 = 3,
        label: Result<FieldDescriptorProto_Label, i32> = 4,
        type_: Result<FieldDescriptorProto_Type, i32> = 5,
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

    enum FieldDescriptorProto_Type {
        TYPE_DOUBLE = 1,
        TYPE_FLOAT = 2,
        TYPE_INT64 = 3,
        TYPE_UINT64 = 4,
        TYPE_INT32 = 5,
        TYPE_FIXED64 = 6,
        TYPE_FIXED32 = 7,
        TYPE_BOOL = 8,
        TYPE_STRING = 9,
        TYPE_GROUP = 10,
        TYPE_MESSAGE = 11,
        TYPE_BYTES = 12,
        TYPE_UINT32 = 13,
        TYPE_ENUM = 14,
        TYPE_SFIXED32 = 15,
        TYPE_SFIXED64 = 16,
        TYPE_SINT32 = 17,
        TYPE_SINT64 = 18,
    }
    enum FieldDescriptorProto_Label {
        LABEL_OPTIONAL = 1,
        LABEL_REQUIRED = 2,
        LABEL_REPEATED = 3,
    }
}
