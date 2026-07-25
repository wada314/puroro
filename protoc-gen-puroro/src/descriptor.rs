//! Descriptor subset decoded from `CodeGeneratorRequest`.
//!
//! This is a read-only view of the protobuf descriptors protoc sends — not a
//! faithful reproduction of every `google.protobuf.*` field. Message/enum type
//! names are absolute [`ProtoFqn`] values; resolve the full graph into
//! [`crate::resolved::FileSet`] via [`crate::resolved::resolve`].

mod proto_fqn;

pub use proto_fqn::ProtoFqn;

/// Plugin / generate-time metadata (not part of the type graph).
///
/// Passed by reference through the generator; kept separate from
/// [`crate::resolved::FileSet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodegenMeta {
    /// Proto file names that should be generated (`file_to_generate`).
    pub file_to_generate: Vec<String>,
    /// Raw plugin parameter string (`parameter`), if any.
    pub parameter: Option<String>,
}

/// Top-level request derived from `CodeGeneratorRequest`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodegenRequest {
    pub meta: CodegenMeta,
    /// Descriptor set (`proto_file`), in dependency order as provided by protoc.
    pub proto_files: Vec<ProtoFile>,
}

/// Protobuf syntax of a `.proto` file (`FileDescriptorProto.syntax`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Syntax {
    Proto2,
    Proto3,
}

/// One `.proto` file (`FileDescriptorProto` subset).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtoFile {
    pub name: String,
    pub package: String,
    pub syntax: Syntax,
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
    /// Set for `TYPE_MESSAGE` / `TYPE_ENUM` / `TYPE_GROUP` (absolute FQN).
    pub type_name: Option<ProtoFqn>,
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
    UInt64 = 4,
    Int32 = 5,
    Fixed64 = 6,
    Fixed32 = 7,
    Bool = 8,
    String = 9,
    Group = 10,
    Message = 11,
    Bytes = 12,
    UInt32 = 13,
    Enum = 14,
    SFixed32 = 15,
    SFixed64 = 16,
    SInt32 = 17,
    SInt64 = 18,
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
            4 => Some(Self::UInt64),
            5 => Some(Self::Int32),
            6 => Some(Self::Fixed64),
            7 => Some(Self::Fixed32),
            8 => Some(Self::Bool),
            9 => Some(Self::String),
            10 => Some(Self::Group),
            11 => Some(Self::Message),
            12 => Some(Self::Bytes),
            13 => Some(Self::UInt32),
            14 => Some(Self::Enum),
            15 => Some(Self::SFixed32),
            16 => Some(Self::SFixed64),
            17 => Some(Self::SInt32),
            18 => Some(Self::SInt64),
            _ => None,
        }
    }
}
