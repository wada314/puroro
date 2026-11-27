//! Wire field numbers for the `plugin.proto` / `descriptor.proto` subset.
//!
//! Each nested module mirrors one `google.protobuf.*` (or plugin) message and
//! names the fields this plugin reads or writes. Prefer these constants over
//! bare integers in `match` / encode paths so the wire layer stays aligned with
//! the official descriptors.
//!
//! The plugin intentionally does **not** depend on `puroro-types` generated
//! message types: that crate is produced by this plugin (`build.rs` + bindeps),
//! so a live dependency would be a bootstrap cycle. Custom options
//! (`51400`–`51405`) also stay numeric until typed extensions exist. The
//! structured codegen IR remains [`crate::descriptor`].

#![allow(dead_code)] // Some official numbers are documented for completeness.

/// `google.protobuf.compiler.CodeGeneratorRequest`
pub mod code_generator_request {
    /// `repeated string file_to_generate = 1;`
    pub const FILE_TO_GENERATE: u32 = 1;
    /// `optional string parameter = 2;`
    pub const PARAMETER: u32 = 2;
    /// `optional Version compiler_version = 3;` (ignored)
    pub const COMPILER_VERSION: u32 = 3;
    /// `repeated FileDescriptorProto proto_file = 15;`
    pub const PROTO_FILE: u32 = 15;
    /// `repeated FileDescriptorProto source_file_descriptors = 17;` (ignored)
    pub const SOURCE_FILE_DESCRIPTORS: u32 = 17;
}

/// `google.protobuf.compiler.CodeGeneratorResponse`
pub mod code_generator_response {
    /// `optional string error = 1;`
    pub const ERROR: u32 = 1;
    /// `optional uint64 supported_features = 2;`
    pub const SUPPORTED_FEATURES: u32 = 2;
    /// `optional int32 minimum_edition = 3;`
    pub const MINIMUM_EDITION: u32 = 3;
    /// `optional int32 maximum_edition = 4;`
    pub const MAXIMUM_EDITION: u32 = 4;
    /// `repeated File file = 15;`
    pub const FILE: u32 = 15;

    /// `google.protobuf.compiler.CodeGeneratorResponse.File`
    pub mod file {
        /// `optional string name = 1;`
        pub const NAME: u32 = 1;
        /// `optional string content = 15;`
        pub const CONTENT: u32 = 15;
    }
}

/// `google.protobuf.FileDescriptorSet`
pub mod file_descriptor_set {
    /// `repeated FileDescriptorProto file = 1;`
    pub const FILE: u32 = 1;
}

/// `google.protobuf.FileDescriptorProto`
pub mod file_descriptor_proto {
    /// `optional string name = 1;`
    pub const NAME: u32 = 1;
    /// `optional string package = 2;`
    pub const PACKAGE: u32 = 2;
    /// `repeated string dependency = 3;`
    pub const DEPENDENCY: u32 = 3;
    /// `repeated DescriptorProto message_type = 4;`
    pub const MESSAGE_TYPE: u32 = 4;
    /// `repeated EnumDescriptorProto enum_type = 5;`
    pub const ENUM_TYPE: u32 = 5;
    /// `optional FileOptions options = 8;`
    pub const OPTIONS: u32 = 8;
    /// `optional SourceCodeInfo source_code_info = 9;`
    pub const SOURCE_CODE_INFO: u32 = 9;
    /// `optional string syntax = 12;`
    pub const SYNTAX: u32 = 12;
    /// `optional Edition edition = 14;`
    pub const EDITION: u32 = 14;
}

/// `google.protobuf.DescriptorProto`
pub mod descriptor_proto {
    /// `optional string name = 1;`
    pub const NAME: u32 = 1;
    /// `repeated FieldDescriptorProto field = 2;`
    pub const FIELD: u32 = 2;
    /// `repeated DescriptorProto nested_type = 3;`
    pub const NESTED_TYPE: u32 = 3;
    /// `repeated EnumDescriptorProto enum_type = 4;`
    pub const ENUM_TYPE: u32 = 4;
    /// `optional MessageOptions options = 7;`
    pub const OPTIONS: u32 = 7;
    /// `repeated OneofDescriptorProto oneof_decl = 8;`
    pub const ONEOF_DECL: u32 = 8;
}

/// `google.protobuf.FieldDescriptorProto`
pub mod field_descriptor_proto {
    /// `optional string name = 1;`
    pub const NAME: u32 = 1;
    /// `optional int32 number = 3;`
    pub const NUMBER: u32 = 3;
    /// `optional Label label = 4;`
    pub const LABEL: u32 = 4;
    /// `optional Type type = 5;`
    pub const TYPE: u32 = 5;
    /// `optional string type_name = 6;`
    pub const TYPE_NAME: u32 = 6;
    /// `optional string default_value = 7;`
    pub const DEFAULT_VALUE: u32 = 7;
    /// `optional FieldOptions options = 8;`
    pub const OPTIONS: u32 = 8;
    /// `optional int32 oneof_index = 9;`
    pub const ONEOF_INDEX: u32 = 9;
    /// `optional bool proto3_optional = 17;`
    pub const PROTO3_OPTIONAL: u32 = 17;
}

/// `google.protobuf.OneofDescriptorProto`
pub mod oneof_descriptor_proto {
    /// `optional string name = 1;`
    pub const NAME: u32 = 1;
}

/// `google.protobuf.EnumDescriptorProto`
pub mod enum_descriptor_proto {
    /// `optional string name = 1;`
    pub const NAME: u32 = 1;
    /// `repeated EnumValueDescriptorProto value = 2;`
    pub const VALUE: u32 = 2;
    /// `optional EnumOptions options = 3;`
    pub const OPTIONS: u32 = 3;
}

/// `google.protobuf.EnumValueDescriptorProto`
pub mod enum_value_descriptor_proto {
    /// `optional string name = 1;`
    pub const NAME: u32 = 1;
    /// `optional int32 number = 2;`
    pub const NUMBER: u32 = 2;
}

/// `google.protobuf.FileOptions`
pub mod file_options {
    /// `optional FeatureSet features = 50;`
    pub const FEATURES: u32 = 50;
}

/// `google.protobuf.MessageOptions`
pub mod message_options {
    /// `optional bool map_entry = 7;`
    pub const MAP_ENTRY: u32 = 7;
    /// `optional FeatureSet features = 12;`
    pub const FEATURES: u32 = 12;
}

/// `google.protobuf.FieldOptions`
pub mod field_options {
    /// `optional bool packed = 2;`
    pub const PACKED: u32 = 2;
    /// `optional FeatureSet features = 21;`
    pub const FEATURES: u32 = 21;
}

/// `google.protobuf.EnumOptions`
pub mod enum_options {
    /// `optional FeatureSet features = 7;`
    pub const FEATURES: u32 = 7;
}

/// `google.protobuf.FeatureSet`
pub mod feature_set {
    /// `optional FieldPresence field_presence = 1;`
    pub const FIELD_PRESENCE: u32 = 1;
    /// `optional EnumType enum_type = 2;`
    pub const ENUM_TYPE: u32 = 2;
    /// `optional RepeatedFieldEncoding repeated_field_encoding = 3;`
    pub const REPEATED_FIELD_ENCODING: u32 = 3;
    /// `optional Utf8Validation utf8_validation = 4;`
    pub const UTF8_VALIDATION: u32 = 4;
    /// `optional MessageEncoding message_encoding = 5;`
    pub const MESSAGE_ENCODING: u32 = 5;
    /// `optional JsonFormat json_format = 6;`
    pub const JSON_FORMAT: u32 = 6;
    /// `optional EnforceNamingStyle enforce_naming_style = 7;`
    pub const ENFORCE_NAMING_STYLE: u32 = 7;
    /// `optional DefaultSymbolVisibility default_symbol_visibility = 8;`
    pub const DEFAULT_SYMBOL_VISIBILITY: u32 = 8;
}

/// `google.protobuf.SourceCodeInfo`
pub mod source_code_info {
    /// `repeated Location location = 1;`
    pub const LOCATION: u32 = 1;

    /// `google.protobuf.SourceCodeInfo.Location`
    pub mod location {
        /// `repeated int32 path = 1 [packed = true];`
        pub const PATH: u32 = 1;
        /// `repeated int32 span = 2 [packed = true];` (ignored)
        pub const SPAN: u32 = 2;
        /// `optional string leading_comments = 3;`
        pub const LEADING_COMMENTS: u32 = 3;
        /// `optional string trailing_comments = 4;`
        pub const TRAILING_COMMENTS: u32 = 4;
        /// `repeated string leading_detached_comments = 6;` (ignored)
        pub const LEADING_DETACHED_COMMENTS: u32 = 6;
    }
}
