//! Type aliases for singular LEN fields ([`SingularField`] with LEN markers).
//!
//! The allocator is no longer a type parameter on the field: storage is
//! allocator-less and `A` is taken from [`MessageCommon`](crate::fields::shared::MessageCommon)
//! at method call sites (same as varint).

use crate::defaults::ProtoDefault;
use crate::fields::shared::field_presence::{Explicit, Implicit};
use crate::fields::singular::field::{SingularField, SingularFieldMut, SingularFieldRef};
use crate::fields::wire::len;

/// Singular LEN field — alias of [`SingularField`] (no allocator type parameter).
pub type SingularLenField<T, P, const FIELD: u32, D = ProtoDefault> = SingularField<T, P, FIELD, D>;

/// Bound shared view for a singular LEN field.
pub type SingularLenFieldRef<'a, T, P, const FIELD: u32, D, Pb, A> =
    SingularFieldRef<'a, T, P, FIELD, D, Pb, A>;

/// Bound mutation view for a singular LEN field.
pub type SingularLenFieldMut<'f, 'c, T, P, const FIELD: u32, D, Pb, A> =
    SingularFieldMut<'f, 'c, T, P, FIELD, D, Pb, A>;

pub type SingularLen<T, P, const FIELD: u32, D = ProtoDefault> = SingularLenField<T, P, FIELD, D>;

pub type ImplicitLenField<T, const FIELD: u32> = SingularLenField<T, Implicit, FIELD>;
pub type ExplicitLenField<T, const BIT: usize, const FIELD: u32, D = ProtoDefault> =
    SingularLenField<T, Explicit<BIT>, FIELD, D>;

pub type ImplicitString<A, const FIELD: u32> = ImplicitLenField<len::ProtoString<A>, FIELD>;
pub type ExplicitString<A, const BIT: usize, const FIELD: u32, D = ProtoDefault> =
    ExplicitLenField<len::ProtoString<A>, BIT, FIELD, D>;
pub type ImplicitBytes<A, const FIELD: u32> = ImplicitLenField<len::ProtoBytes<A>, FIELD>;
pub type ExplicitBytes<A, const BIT: usize, const FIELD: u32, D = ProtoDefault> =
    ExplicitLenField<len::ProtoBytes<A>, BIT, FIELD, D>;
