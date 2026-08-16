//! Resolved type graph built from [`crate::descriptor`].
//!
//! Nodes live in a caller-owned [`Arena`] and refer to each other with ordinary
//! references tied to that arena's lifetime.
//!
//! Nested messages/enums are ordinary [`Vec`]s built while walking the descriptor
//! tree (each node once). Field [`TypeRef`]s may form cycles, so registration
//! queues each message and a later link pass writes fields; that storage detail
//! is not part of the public API.
//!
//! Plugin metadata ([`crate::descriptor::CodegenMeta`]) is intentionally **not**
//! stored here — pass it alongside `&FileSet` by reference.

mod arena;
mod resolve;

pub use arena::Arena;
pub use resolve::resolve;

use crate::descriptor::features::{EnumType, RepeatedFieldEncoding, Utf8Validation};
use crate::descriptor::{BytesLayout, FieldType, ProtoFqn, StringLayout, Syntax};
use ::std::cell::OnceCell;
use ::std::collections::HashMap;
use ::std::fmt;

/// Cardinality + singular presence after resolve.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldOccurrence {
    Singular(SingularPresence),
    /// Repeated field with resolved packed/expanded encoding preference.
    ///
    /// Non-packable types (string / bytes / message) still carry a value, but
    /// codegen always emits expanded wire helpers for those.
    Repeated(RepeatedFieldEncoding),
    /// `map<K, V>` — `type_ref` is the synthetic map-entry message.
    Map,
}

/// Presence policy for a singular field (matches puroro-rt markers; no BIT yet).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SingularPresence {
    Implicit,
    Explicit,
    LegacyRequired,
    Oneof,
    /// Singular message / group (pointer presence); matches puroro-rt `Message`.
    Message,
}

/// Resolved root: the set of files and the type graph within one arena.
#[derive(Debug)]
pub struct FileSet<'a> {
    files: Vec<&'a File<'a>>,
    /// All messages and enums keyed by absolute protobuf FQN.
    types_by_fqn: HashMap<ProtoFqn, TypeItem<'a>>,
}

/// One `.proto` file after resolution.
#[derive(Debug)]
pub struct File<'a> {
    name: String,
    package: String,
    syntax: Syntax,
    dependency: Vec<String>,
    messages: Vec<&'a Message<'a>>,
    enums: Vec<&'a Enum<'a>>,
}

/// A message type with resolved field type handles.
pub struct Message<'a> {
    name: String,
    /// Absolute protobuf FQN (e.g. `.example.v1.Task`).
    fqn: ProtoFqn,
    /// Enclosing message, if nested. Set once after this node is allocated
    /// (children are built first so `nested_*` can be plain [`Vec`]s).
    parent: OnceCell<&'a Message<'a>>,
    /// Filled in the link pass (may reference peer / mutually recursive types).
    fields: OnceCell<Vec<Field<'a>>>,
    nested_messages: Vec<&'a Message<'a>>,
    nested_enums: Vec<&'a Enum<'a>>,
    /// Real oneofs only; proto3 `optional` synthetic groups are dropped.
    oneofs: Vec<Oneof>,
    /// Synthetic map entry (`MessageOptions.map_entry = true`).
    map_entry: bool,
}

/// A field with a resolved [`TypeRef`].
#[derive(Debug, Clone)]
pub struct Field<'a> {
    name: String,
    number: i32,
    occurrence: FieldOccurrence,
    type_ref: TypeRef<'a>,
    /// Index into the parent [`Message`]'s real [`oneofs`](Message::oneofs).
    /// `None` for proto3 `optional` (descriptor synthetic oneof is stripped).
    oneof_index: Option<i32>,
    /// Raw `FieldDescriptorProto.default_value` from protoc, if any.
    default_value: Option<String>,
    /// Set for `string` / `bytes` fields (`VERIFY` / `NONE`).
    utf8_validation: Option<Utf8Validation>,
    /// `(puroro.string_layout)`, if set.
    string_layout: Option<StringLayout>,
    /// `(puroro.bytes_layout)`, if set.
    bytes_layout: Option<BytesLayout>,
}

/// Resolved type of a field.
#[derive(Clone)]
pub enum TypeRef<'a> {
    Double,
    Float,
    Int64,
    UInt64,
    Int32,
    Fixed64,
    Fixed32,
    Bool,
    String,
    Bytes,
    UInt32,
    SFixed32,
    SFixed64,
    SInt32,
    SInt64,
    Message(&'a Message<'a>),
    Enum(&'a Enum<'a>),
}

/// A oneof declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Oneof {
    name: String,
}

/// An enum type.
pub struct Enum<'a> {
    name: String,
    fqn: ProtoFqn,
    /// Enclosing message, if nested. Empty for file-level enums.
    parent: OnceCell<&'a Message<'a>>,
    /// Open vs closed (`features.enum_type`, or proto2/proto3 defaults).
    openness: EnumType,
    values: Vec<EnumValue>,
}

/// An enum value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumValue {
    name: String,
    number: i32,
}

/// Entry in [`FileSet`]'s type map.
#[derive(Clone, Copy)]
pub enum TypeItem<'a> {
    Message(&'a Message<'a>),
    Enum(&'a Enum<'a>),
}

impl<'a> FileSet<'a> {
    pub fn files(&self) -> impl Iterator<Item = &'a File<'a>> + '_ {
        self.files.iter().copied()
    }

    /// Look up a message or enum by absolute protobuf FQN.
    ///
    /// `fqn` must already be in canonical form (leading `.`), as stored in
    /// [`ProtoFqn`]. Use [`ProtoFqn::parse`] at the boundary if you only have a
    /// raw string. Lookup itself does not allocate — the map is keyed by
    /// [`ProtoFqn`] and queried via [`Borrow<str>`](std::borrow::Borrow).
    pub fn lookup(&self, fqn: impl AsRef<str>) -> Option<TypeItem<'a>> {
        let fqn = fqn.as_ref();
        debug_assert!(
            fqn.starts_with('.'),
            "lookup expects a canonical ProtoFqn (leading `.`), got {fqn:?}"
        );
        self.types_by_fqn.get(fqn).copied()
    }
}

impl<'a> File<'a> {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn package(&self) -> &str {
        &self.package
    }

    pub fn syntax(&self) -> Syntax {
        self.syntax
    }

    pub fn dependencies(&self) -> impl Iterator<Item = &str> + '_ {
        self.dependency.iter().map(String::as_str)
    }

    pub fn messages(&self) -> impl Iterator<Item = &'a Message<'a>> + '_ {
        self.messages.iter().copied()
    }

    pub fn enums(&self) -> impl Iterator<Item = &'a Enum<'a>> + '_ {
        self.enums.iter().copied()
    }
}

impl<'a> Message<'a> {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn fqn(&self) -> &ProtoFqn {
        &self.fqn
    }

    /// Enclosing message, if this type is nested.
    pub fn parent(&self) -> Option<&'a Message<'a>> {
        self.parent.get().copied()
    }

    /// Fields after resolve has finished.
    pub fn fields(&self) -> impl Iterator<Item = &Field<'a>> + '_ {
        self.fields
            .get()
            .expect("message fields accessed before resolve finished")
            .iter()
    }

    pub fn nested_messages(&self) -> impl Iterator<Item = &'a Message<'a>> + '_ {
        self.nested_messages.iter().copied()
    }

    pub fn nested_enums(&self) -> impl Iterator<Item = &'a Enum<'a>> + '_ {
        self.nested_enums.iter().copied()
    }

    pub fn oneofs(&self) -> impl Iterator<Item = &Oneof> + '_ {
        self.oneofs.iter()
    }

    /// Whether this is a synthetic map-entry message (not emitted as a user type).
    pub fn is_map_entry(&self) -> bool {
        self.map_entry
    }
}

impl<'a> Field<'a> {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn number(&self) -> i32 {
        self.number
    }

    pub fn occurrence(&self) -> FieldOccurrence {
        self.occurrence
    }

    pub fn type_ref(&self) -> &TypeRef<'a> {
        &self.type_ref
    }

    pub fn oneof_index(&self) -> Option<i32> {
        self.oneof_index
    }

    /// Raw proto default text from the descriptor, if `[default = …]` was set.
    pub fn default_value(&self) -> Option<&str> {
        self.default_value.as_deref()
    }

    /// UTF-8 policy for `string` / `bytes`; `None` for other types.
    pub fn utf8_validation(&self) -> Option<Utf8Validation> {
        self.utf8_validation
    }

    /// `(puroro.string_layout)`, if set on the descriptor.
    pub fn string_layout(&self) -> Option<StringLayout> {
        self.string_layout
    }

    /// `(puroro.bytes_layout)`, if set on the descriptor.
    pub fn bytes_layout(&self) -> Option<BytesLayout> {
        self.bytes_layout
    }
}

impl Oneof {
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<'a> Enum<'a> {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn fqn(&self) -> &ProtoFqn {
        &self.fqn
    }

    /// Enclosing message, if this enum is nested.
    pub fn parent(&self) -> Option<&'a Message<'a>> {
        self.parent.get().copied()
    }

    pub fn openness(&self) -> EnumType {
        self.openness
    }

    pub fn values(&self) -> impl Iterator<Item = &EnumValue> + '_ {
        self.values.iter()
    }
}

impl EnumValue {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn number(&self) -> i32 {
        self.number
    }
}

impl<'a> TypeRef<'a> {
    pub fn as_message(&self) -> Option<&'a Message<'a>> {
        match self {
            Self::Message(m) => Some(*m),
            _ => None,
        }
    }

    pub fn as_enum(&self) -> Option<&'a Enum<'a>> {
        match self {
            Self::Enum(e) => Some(*e),
            _ => None,
        }
    }

    pub fn from_scalar(type_: FieldType) -> Option<Self> {
        Some(match type_ {
            FieldType::Double => Self::Double,
            FieldType::Float => Self::Float,
            FieldType::Int64 => Self::Int64,
            FieldType::UInt64 => Self::UInt64,
            FieldType::Int32 => Self::Int32,
            FieldType::Fixed64 => Self::Fixed64,
            FieldType::Fixed32 => Self::Fixed32,
            FieldType::Bool => Self::Bool,
            FieldType::String => Self::String,
            FieldType::Bytes => Self::Bytes,
            FieldType::UInt32 => Self::UInt32,
            FieldType::SFixed32 => Self::SFixed32,
            FieldType::SFixed64 => Self::SFixed64,
            FieldType::SInt32 => Self::SInt32,
            FieldType::SInt64 => Self::SInt64,
            FieldType::Message | FieldType::Enum | FieldType::Group => return None,
        })
    }
}

impl<'a> TypeItem<'a> {
    pub fn fqn(self) -> &'a ProtoFqn {
        match self {
            Self::Message(m) => m.fqn(),
            Self::Enum(e) => e.fqn(),
        }
    }

    pub fn as_message(self) -> Option<&'a Message<'a>> {
        match self {
            Self::Message(m) => Some(m),
            Self::Enum(_) => None,
        }
    }

    pub fn as_enum(self) -> Option<&'a Enum<'a>> {
        match self {
            Self::Enum(e) => Some(e),
            Self::Message(_) => None,
        }
    }
}

impl fmt::Debug for Message<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Message")
            .field("fqn", &self.fqn)
            .field("parent", &self.parent().map(|m| m.fqn()))
            .field("fields", &self.fields.get())
            .field(
                "nested_messages",
                &self
                    .nested_messages
                    .iter()
                    .map(|m| m.fqn())
                    .collect::<Vec<_>>(),
            )
            .field(
                "nested_enums",
                &self
                    .nested_enums
                    .iter()
                    .map(|e| e.fqn())
                    .collect::<Vec<_>>(),
            )
            .field("oneofs", &self.oneofs)
            .finish()
    }
}

impl fmt::Debug for Enum<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Enum")
            .field("fqn", &self.fqn)
            .field("parent", &self.parent().map(|m| m.fqn()))
            .field("openness", &self.openness)
            .field("values", &self.values)
            .finish()
    }
}

impl fmt::Debug for TypeRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double => write!(f, "Double"),
            Self::Float => write!(f, "Float"),
            Self::Int64 => write!(f, "Int64"),
            Self::UInt64 => write!(f, "UInt64"),
            Self::Int32 => write!(f, "Int32"),
            Self::Fixed64 => write!(f, "Fixed64"),
            Self::Fixed32 => write!(f, "Fixed32"),
            Self::Bool => write!(f, "Bool"),
            Self::String => write!(f, "String"),
            Self::Bytes => write!(f, "Bytes"),
            Self::UInt32 => write!(f, "UInt32"),
            Self::SFixed32 => write!(f, "SFixed32"),
            Self::SFixed64 => write!(f, "SFixed64"),
            Self::SInt32 => write!(f, "SInt32"),
            Self::SInt64 => write!(f, "SInt64"),
            Self::Message(m) => write!(f, "Message({})", m.fqn()),
            Self::Enum(e) => write!(f, "Enum({})", e.fqn()),
        }
    }
}

impl fmt::Debug for TypeItem<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Message(m) => write!(f, "Message({})", m.fqn()),
            Self::Enum(e) => write!(f, "Enum({})", e.fqn()),
        }
    }
}
