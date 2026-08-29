//! Sparse `*Desc` constructors for unit tests.
//!
//! Arguments are the identifying fields. Everything else is a typical empty /
//! proto3 value. Override with struct update syntax:
//!
//! ```ignore
//! FieldDesc { proto3_optional: true, ..field("title", 2, FieldType::String) }
//! ```

use super::{
    CodegenMeta, CodegenRequest, EnumDesc, EnumValueDesc, FeatureSet, FieldDesc, FieldLabel,
    FieldType, MessageDesc, OneofDesc, ProtoFile, Syntax,
};

/// Optional field with empty options (`label = Optional`, no type name / oneof).
pub(crate) fn field(name: impl Into<String>, number: i32, type_: FieldType) -> FieldDesc {
    FieldDesc {
        name: name.into(),
        number,
        label: FieldLabel::Optional,
        type_,
        type_name: None,
        oneof_index: None,
        proto3_optional: false,
        default_value: None,
        packed: None,
        string_layout: None,
        bytes_layout: None,
        message_layout: None,
        features: FeatureSet::default(),
    }
}

/// Empty message (`map_entry = false`, no fields / nested types / oneofs).
pub(crate) fn message(name: impl Into<String>) -> MessageDesc {
    MessageDesc {
        name: name.into(),
        fields: vec![],
        nested_messages: vec![],
        nested_enums: vec![],
        oneofs: vec![],
        map_entry: false,
    }
}

/// Enum with the given values and default (unset) features.
pub(crate) fn enumeration(name: impl Into<String>, values: Vec<EnumValueDesc>) -> EnumDesc {
    EnumDesc {
        name: name.into(),
        values,
        features: FeatureSet::default(),
    }
}

pub(crate) fn enum_value(name: impl Into<String>, number: i32) -> EnumValueDesc {
    EnumValueDesc {
        name: name.into(),
        number,
    }
}

pub(crate) fn oneof(name: impl Into<String>) -> OneofDesc {
    OneofDesc { name: name.into() }
}

/// Synthetic `map_entry` message with `key` (field 1) and `value` (field 2).
pub(crate) fn map_entry(name: impl Into<String>, key: FieldDesc, value: FieldDesc) -> MessageDesc {
    MessageDesc {
        fields: vec![key, value],
        map_entry: true,
        ..message(name)
    }
}

/// Proto3 file with no messages, enums, or dependencies.
pub(crate) fn proto_file(name: impl Into<String>, package: impl Into<String>) -> ProtoFile {
    ProtoFile {
        name: name.into(),
        package: package.into(),
        syntax: Syntax::Proto3,
        features: FeatureSet::default(),
        dependency: vec![],
        messages: vec![],
        enums: vec![],
    }
}

/// Request that generates every file in `proto_files` (`parameter` unset).
pub(crate) fn request(proto_files: Vec<ProtoFile>) -> CodegenRequest {
    CodegenRequest {
        meta: CodegenMeta {
            file_to_generate: proto_files.iter().map(|f| f.name.clone()).collect(),
            parameter: None,
        },
        proto_files,
    }
}
