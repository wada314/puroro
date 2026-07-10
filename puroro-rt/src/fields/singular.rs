//! Singular (non-repeated) field wrappers — scalars and nested messages.
//!
//! “Singular” covers both presence-tracked (`EXPLICIT`) and non-presence-tracked
//! (`IMPLICIT`) fields; see [`field`](crate::fields::singular::field).

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
