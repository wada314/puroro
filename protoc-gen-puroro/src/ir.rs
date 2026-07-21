//! Codegen intermediate representation.
//!
//! This is a **read-only schema view** for the generator — not a faithful
//! reproduction of `google.protobuf.*` message types. Only fields the emitter
//! needs are retained.

/// Top-level request derived from `CodeGeneratorRequest`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodegenRequest {
    /// Proto file names that should be generated (`file_to_generate`).
    pub file_to_generate: Vec<String>,
    /// Raw plugin parameter string (`parameter`), if any.
    pub parameter: Option<String>,
    /// Descriptor set (`proto_file`), in dependency order as provided by protoc.
    pub proto_files: Vec<ProtoFile>,
}

/// One `.proto` file (`FileDescriptorProto` subset).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtoFile {
    pub name: String,
    pub package: String,
    pub dependency: Vec<String>,
    pub messages: Vec<MessageDesc>,
    pub enums: Vec<EnumDesc>,
}

/// A message type (`DescriptorProto` subset).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageDesc {
    pub name: String,
    pub fields: Vec<FieldDesc>,
    pub nested_messages: Vec<MessageDesc>,
    pub nested_enums: Vec<EnumDesc>,
    pub oneofs: Vec<OneofDesc>,
}

/// A field (`FieldDescriptorProto` subset).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldDesc {
    pub name: String,
    pub number: i32,
    pub label: FieldLabel,
    pub type_: FieldType,
    /// Set for `TYPE_MESSAGE` / `TYPE_ENUM` / `TYPE_GROUP` (protobuf FQN, often with leading `.`).
    pub type_name: Option<String>,
    /// Index into the parent message's `oneofs`, when this field is a oneof member.
    pub oneof_index: Option<i32>,
    pub proto3_optional: bool,
}

/// Field label (`FieldDescriptorProto.Label`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldLabel {
    Optional = 1,
    Required = 2,
    Repeated = 3,
}

/// Field type (`FieldDescriptorProto.Type`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldType {
    Double = 1,
    Float = 2,
    Int64 = 3,
    Uint64 = 4,
    Int32 = 5,
    Fixed64 = 6,
    Fixed32 = 7,
    Bool = 8,
    String = 9,
    Group = 10,
    Message = 11,
    Bytes = 12,
    Uint32 = 13,
    Enum = 14,
    Sfixed32 = 15,
    Sfixed64 = 16,
    Sint32 = 17,
    Sint64 = 18,
}

/// A oneof declaration (`OneofDescriptorProto` subset).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OneofDesc {
    pub name: String,
}

/// An enum type (`EnumDescriptorProto` subset).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumDesc {
    pub name: String,
    pub values: Vec<EnumValueDesc>,
}

/// An enum value (`EnumValueDescriptorProto` subset).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumValueDesc {
    pub name: String,
    pub number: i32,
}

impl FieldLabel {
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::Optional),
            2 => Some(Self::Required),
            3 => Some(Self::Repeated),
            _ => None,
        }
    }
}

impl FieldType {
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::Double),
            2 => Some(Self::Float),
            3 => Some(Self::Int64),
            4 => Some(Self::Uint64),
            5 => Some(Self::Int32),
            6 => Some(Self::Fixed64),
            7 => Some(Self::Fixed32),
            8 => Some(Self::Bool),
            9 => Some(Self::String),
            10 => Some(Self::Group),
            11 => Some(Self::Message),
            12 => Some(Self::Bytes),
            13 => Some(Self::Uint32),
            14 => Some(Self::Enum),
            15 => Some(Self::Sfixed32),
            16 => Some(Self::Sfixed64),
            17 => Some(Self::Sint32),
            18 => Some(Self::Sint64),
            _ => None,
        }
    }
}
