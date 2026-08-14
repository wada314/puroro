//! Singular (non-repeated) field wrappers.
//!
//! “Singular” covers both presence-tracked (`EXPLICIT`) and non-presence-tracked
//! (`IMPLICIT`) fields; see [`field`](crate::fields::singular::field).

pub(crate) mod field;

pub use field::{SingularField, SingularFieldAccess, SingularFieldMut, SingularFieldRef};
