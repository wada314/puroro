//! Singular field wrappers (scalar + nested message).

pub mod field;
pub mod len;
pub mod message;
pub mod varint;

pub use field::{SingularField, SingularFieldMut};
pub use len::{
    ExplicitBytes, ExplicitLenField, ExplicitString, ImplicitBytes, ImplicitLenField,
    ImplicitString, LegacyRequiredLenField, SingularLen, SingularLenField, SingularLenFieldMut,
};
pub use message::{
    MessagePresence, NestedMessage, NestedMessageField, NestedMessageFieldMut, Singular,
};
pub use varint::{
    ExplicitEnum, ExplicitInt32, ExplicitVarint, ExplicitVarintField, ImplicitEnum, ImplicitInt32,
    ImplicitVarint, ImplicitVarintField, SingularVarint, SingularVarintField,
    SingularVarintFieldMut,
};
