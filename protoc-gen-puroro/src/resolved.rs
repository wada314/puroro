//! Resolved type graph built from [`crate::descriptor`].
//!
//! Nodes live in a caller-owned [`Arena`] and refer to each other with ordinary
//! references tied to that arena's lifetime. Fields are filled once via
//! [`OnceCell`](std::cell::OnceCell) during [`resolve`]. Plugin metadata
//! ([`crate::descriptor::CodegenMeta`]) is intentionally **not** stored here —
//! pass it alongside `&FileSet` by reference.

mod arena;
mod resolve;

pub use arena::Arena;
pub use resolve::resolve;

use crate::descriptor::{FieldLabel, FieldType, ProtoFqn};
use ::std::cell::OnceCell;
use ::std::collections::HashMap;
use ::std::fmt;

/// Resolved root: the set of files and the type graph within one arena.
#[derive(Debug)]
pub struct FileSet<'a> {
    pub files: Vec<&'a File<'a>>,
    /// All messages and enums keyed by absolute protobuf FQN.
    pub types_by_fqn: HashMap<ProtoFqn, TypeItem<'a>>,
}

/// One `.proto` file after resolution.
#[derive(Debug)]
pub struct File<'a> {
    pub name: String,
    pub package: String,
    pub dependency: Vec<String>,
    pub messages: Vec<&'a Message<'a>>,
    pub enums: Vec<&'a Enum<'a>>,
}

/// A message type with resolved field type handles.
pub struct Message<'a> {
    pub name: String,
    /// Absolute protobuf FQN (e.g. `.example.v1.Task`).
    pub fqn: ProtoFqn,
    /// Enclosing message, if nested.
    pub parent: Option<&'a Message<'a>>,
    fields: OnceCell<Vec<Field<'a>>>,
    nested_messages: OnceCell<Vec<&'a Message<'a>>>,
    nested_enums: OnceCell<Vec<&'a Enum<'a>>>,
    pub oneofs: Vec<Oneof>,
}

/// A field with a resolved [`TypeRef`].
#[derive(Debug, Clone)]
pub struct Field<'a> {
    pub name: String,
    pub number: i32,
    pub label: FieldLabel,
    pub type_ref: TypeRef<'a>,
    pub oneof_index: Option<i32>,
    pub proto3_optional: bool,
}

/// Resolved type of a field.
#[derive(Clone)]
pub enum TypeRef<'a> {
    Double,
    Float,
    Int64,
    Uint64,
    Int32,
    Fixed64,
    Fixed32,
    Bool,
    String,
    Bytes,
    Uint32,
    Sfixed32,
    Sfixed64,
    Sint32,
    Sint64,
    Message(&'a Message<'a>),
    Enum(&'a Enum<'a>),
}

/// A oneof declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Oneof {
    pub name: String,
}

/// An enum type.
#[derive(Debug)]
pub struct Enum<'a> {
    pub name: String,
    pub fqn: ProtoFqn,
    pub parent: Option<&'a Message<'a>>,
    pub values: Vec<EnumValue>,
}

/// An enum value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumValue {
    pub name: String,
    pub number: i32,
}

/// Entry in [`FileSet::types_by_fqn`].
#[derive(Clone, Copy)]
pub enum TypeItem<'a> {
    Message(&'a Message<'a>),
    Enum(&'a Enum<'a>),
}

impl<'a> FileSet<'a> {
    pub fn message(&self, fqn: impl AsRef<str>) -> Option<&'a Message<'a>> {
        let fqn = ProtoFqn::parse(fqn.as_ref());
        match self.types_by_fqn.get(&fqn)? {
            TypeItem::Message(m) => Some(*m),
            TypeItem::Enum(_) => None,
        }
    }

    pub fn enum_ty(&self, fqn: impl AsRef<str>) -> Option<&'a Enum<'a>> {
        let fqn = ProtoFqn::parse(fqn.as_ref());
        match self.types_by_fqn.get(&fqn)? {
            TypeItem::Enum(e) => Some(*e),
            TypeItem::Message(_) => None,
        }
    }
}

impl<'a> Message<'a> {
    pub fn fields(&self) -> &[Field<'a>] {
        self.fields
            .get()
            .map(Vec::as_slice)
            .expect("message fields accessed before resolve finished")
    }

    pub fn nested_messages(&self) -> &[&'a Message<'a>] {
        self.nested_messages
            .get()
            .map(Vec::as_slice)
            .expect("nested messages accessed before resolve finished")
    }

    pub fn nested_enums(&self) -> &[&'a Enum<'a>] {
        self.nested_enums
            .get()
            .map(Vec::as_slice)
            .expect("nested enums accessed before resolve finished")
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
            FieldType::Uint64 => Self::Uint64,
            FieldType::Int32 => Self::Int32,
            FieldType::Fixed64 => Self::Fixed64,
            FieldType::Fixed32 => Self::Fixed32,
            FieldType::Bool => Self::Bool,
            FieldType::String => Self::String,
            FieldType::Bytes => Self::Bytes,
            FieldType::Uint32 => Self::Uint32,
            FieldType::Sfixed32 => Self::Sfixed32,
            FieldType::Sfixed64 => Self::Sfixed64,
            FieldType::Sint32 => Self::Sint32,
            FieldType::Sint64 => Self::Sint64,
            FieldType::Message | FieldType::Enum | FieldType::Group => return None,
        })
    }
}

impl<'a> TypeItem<'a> {
    pub fn fqn(self) -> &'a ProtoFqn {
        match self {
            Self::Message(m) => &m.fqn,
            Self::Enum(e) => &e.fqn,
        }
    }
}

impl fmt::Debug for Message<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Message")
            .field("fqn", &self.fqn)
            .field("parent", &self.parent.map(|m| &m.fqn))
            .field("fields", &self.fields.get())
            .field(
                "nested_messages",
                &self
                    .nested_messages
                    .get()
                    .map(|v| v.iter().map(|m| &m.fqn).collect::<Vec<_>>()),
            )
            .field(
                "nested_enums",
                &self
                    .nested_enums
                    .get()
                    .map(|v| v.iter().map(|e| &e.fqn).collect::<Vec<_>>()),
            )
            .field("oneofs", &self.oneofs)
            .finish()
    }
}

impl fmt::Debug for TypeRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double => write!(f, "Double"),
            Self::Float => write!(f, "Float"),
            Self::Int64 => write!(f, "Int64"),
            Self::Uint64 => write!(f, "Uint64"),
            Self::Int32 => write!(f, "Int32"),
            Self::Fixed64 => write!(f, "Fixed64"),
            Self::Fixed32 => write!(f, "Fixed32"),
            Self::Bool => write!(f, "Bool"),
            Self::String => write!(f, "String"),
            Self::Bytes => write!(f, "Bytes"),
            Self::Uint32 => write!(f, "Uint32"),
            Self::Sfixed32 => write!(f, "Sfixed32"),
            Self::Sfixed64 => write!(f, "Sfixed64"),
            Self::Sint32 => write!(f, "Sint32"),
            Self::Sint64 => write!(f, "Sint64"),
            Self::Message(m) => write!(f, "Message({})", m.fqn),
            Self::Enum(e) => write!(f, "Enum({})", e.fqn),
        }
    }
}

impl fmt::Debug for TypeItem<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Message(m) => write!(f, "Message({})", m.fqn),
            Self::Enum(e) => write!(f, "Enum({})", e.fqn),
        }
    }
}
