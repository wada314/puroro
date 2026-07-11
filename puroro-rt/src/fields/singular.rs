//! Singular (non-repeated) field wrappers — scalars and nested messages.
//!
//! “Singular” covers both presence-tracked (`EXPLICIT`) and non-presence-tracked
//! (`IMPLICIT`) fields; see [`field`](crate::fields::singular::field).

pub(crate) mod access;
pub(crate) mod field;
pub(crate) mod len;
pub(crate) mod message;
pub(crate) mod varint;

pub use access::SingularAccess;
pub use field::{SingularField, SingularFieldMut, SingularFieldRef};
pub use len::{
    ExplicitBytes, ExplicitLenField, ExplicitString, ImplicitBytes, ImplicitLenField,
    ImplicitString, SingularLen, SingularLenField, SingularLenFieldMut, SingularLenFieldRef,
};
pub use message::{
    MessagePresence, NestedMessage, NestedMessageField, NestedMessageFieldMut,
    NestedMessageFieldRef, Singular,
};
pub use varint::{
    ExplicitEnum, ExplicitInt32, ExplicitVarint, ExplicitVarintField, ImplicitEnum, ImplicitInt32,
    ImplicitVarint, ImplicitVarintField, SingularVarint, SingularVarintField,
    SingularVarintFieldMut, SingularVarintFieldRef,
};
