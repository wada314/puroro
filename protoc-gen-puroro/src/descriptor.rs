//! Descriptor subset decoded from `CodeGeneratorRequest`.
//!
//! This is a read-only view of the protobuf descriptors protoc sends — not a
//! faithful reproduction of every `google.protobuf.*` field. Message/enum type
//! names are absolute [`ProtoFqn`] values; resolve the full graph into
//! [`crate::resolved::FileSet`] via [`crate::resolved::resolve`].

pub mod features;
mod proto_fqn;

#[cfg(test)]
pub(crate) mod test_helpers;

pub use features::FeatureSet;
pub use proto_fqn::ProtoFqn;

use ::derive_more::TryFrom;

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

/// Language mode of a `.proto` file.
///
/// For Editions, `FileDescriptorProto.syntax` is `"editions"` and the year is
/// in `FileDescriptorProto.edition` (see <https://protobuf.dev/editions/overview/>).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Syntax {
    Proto2,
    Proto3,
    Editions(Edition),
}

/// Released protobuf edition (`google.protobuf.Edition`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFrom)]
#[try_from(repr)]
#[repr(i32)]
pub enum Edition {
    /// `edition = "2023"` (`EDITION_2023 = 1000`).
    Edition2023 = 1000,
    /// `edition = "2024"` (`EDITION_2024 = 1001`).
    Edition2024 = 1001,
}

/// One `.proto` file (`FileDescriptorProto` subset).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtoFile {
    pub name: String,
    pub package: String,
    pub syntax: Syntax,
    /// File-level `options.features` (`FeatureSet`), if any fields were set.
    pub features: FeatureSet,
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
    /// `MessageOptions.map_entry` — synthetic map entry type (not user-facing).
    pub map_entry: bool,
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
    /// `FieldDescriptorProto.default_value` — textual form from protoc (see descriptor.proto).
    pub default_value: Option<String>,
    /// proto2/proto3 `FieldOptions.packed` (editions uses `features.repeated_field_encoding`).
    pub packed: Option<bool>,
    /// `(puroro.string_layout)`, if set. Singular `string` uses this to pick SSO vs heap.
    pub string_layout: Option<StringLayout>,
    /// `(puroro.bytes_layout)`, if set. Singular `bytes` uses this to pick SSO vs heap.
    pub bytes_layout: Option<BytesLayout>,
    /// `(puroro.message_layout)`, if set. Singular nested messages use this to
    /// pick boxed vs inlined storage.
    pub message_layout: Option<MessageLayout>,
    /// Field-level `options.features` (`FeatureSet`), if any fields were set.
    pub features: FeatureSet,
}

/// Field label (`FieldDescriptorProto.Label`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFrom)]
#[try_from(repr)]
#[repr(i32)]
pub enum FieldLabel {
    Optional = 1,
    Required = 2,
    Repeated = 3,
}

/// Field type (`FieldDescriptorProto.Type`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFrom)]
#[try_from(repr)]
#[repr(i32)]
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

/// Field number of `(puroro.string_layout)` on `google.protobuf.FieldOptions`.
///
/// Matches `proto/puroro/options.proto`. Numbers 50000–99999 are the internal
/// custom-option range.
pub const STRING_LAYOUT_OPTION_NUMBER: u32 = 51400;

/// `(puroro.string_layout)` — singular `string` value layout in generated Rust.
///
/// [`Unspecified`](Self::Unspecified) (and an absent option) uses the generator
/// default, currently SSO (`InlineOrHeap`). [`Sso`](Self::Sso) and
/// [`Heap`](Self::Heap) pin the layout (`Inline` + `UnmanagedString` for heap).
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFrom)]
#[try_from(repr)]
#[repr(i32)]
pub enum StringLayout {
    Unspecified = 0,
    Sso = 1,
    Heap = 2,
}

/// Field number of `(puroro.bytes_layout)` on `google.protobuf.FieldOptions`.
///
/// Matches `proto/puroro/options.proto`. Numbers 50000–99999 are the internal
/// custom-option range.
pub const BYTES_LAYOUT_OPTION_NUMBER: u32 = 51401;

/// `(puroro.bytes_layout)` — singular `bytes` value layout in generated Rust.
///
/// [`Unspecified`](Self::Unspecified) (and an absent option) uses the generator
/// default, currently SSO (`InlineOrHeap`). [`Sso`](Self::Sso) and
/// [`Heap`](Self::Heap) pin the layout (`Inline` + `UnmanagedVec` for heap).
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFrom)]
#[try_from(repr)]
#[repr(i32)]
pub enum BytesLayout {
    Unspecified = 0,
    Sso = 1,
    Heap = 2,
}

/// Field number of `(puroro.message_layout)` on `google.protobuf.FieldOptions`.
///
/// Matches `proto/puroro/options.proto`. Numbers 50000–99999 are the internal
/// custom-option range.
pub const MESSAGE_LAYOUT_OPTION_NUMBER: u32 = 51402;

/// `(puroro.message_layout)` — singular nested-message value layout in generated Rust.
///
/// [`Unspecified`](Self::Unspecified) (and an absent option) uses the generator
/// heuristic (tiny scalar-only children inlined, otherwise boxed).
/// [`Inline`](Self::Inline) requests embedding; [`Boxed`](Self::Boxed) pins a
/// heap box. Illegal cases (oneof / repeated / map / same-SCC) always box.
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFrom)]
#[try_from(repr)]
#[repr(i32)]
pub enum MessageLayout {
    Unspecified = 0,
    Inline = 1,
    Boxed = 2,
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
    /// Enum-level `options.features` (`FeatureSet`), if any fields were set.
    pub features: FeatureSet,
}

/// An enum value (`EnumValueDescriptorProto` subset).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumValueDesc {
    pub name: String,
    pub number: i32,
}
