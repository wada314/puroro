//! Type aliases for singular LEN fields ([`SingularField`] with LEN markers).
//!
//! The allocator is no longer a type parameter on the field: storage is
//! allocator-less and `A` is taken from [`MessageCommon`](crate::fields::shared::MessageCommon)
//! at method call sites (same as varint).

use crate::defaults::ProtoDefault;
use crate::fields::singular::field::{SingularField, SingularFieldMut};
use crate::fields::wire::len;

/// Singular LEN field — alias of [`SingularField`] (no allocator type parameter).
pub type SingularLenField<T, P, const FIELD: u32, D = ProtoDefault> =
    SingularField<T, P, FIELD, D>;

/// Bound mutation view for a singular LEN field.
pub type SingularLenFieldMut<'f, 'c, T, P, const FIELD: u32, D, Pb, A> =
    SingularFieldMut<'f, 'c, T, P, FIELD, D, Pb, A>;

pub type SingularLen<T, P, const FIELD: u32, D = ProtoDefault> =
    SingularLenField<T, P, FIELD, D>;

pub type ImplicitLenField<T, const FIELD: u32> =
    SingularLenField<T, crate::fields::shared::field_presence::Implicit, FIELD>;
pub type OneofLenField<T, const FIELD: u32> =
    SingularLenField<T, crate::fields::shared::field_presence::Oneof, FIELD>;
pub type ExplicitLenField<T, const BIT: usize, const FIELD: u32, D = ProtoDefault> =
    SingularLenField<T, crate::fields::shared::field_presence::Explicit<BIT>, FIELD, D>;
pub type LegacyRequiredLenField<T, const BIT: usize, const FIELD: u32, D = ProtoDefault> =
    SingularLenField<T, crate::fields::shared::field_presence::LegacyRequired<BIT>, FIELD, D>;

pub type ImplicitString<const FIELD: u32> = ImplicitLenField<len::ProtoString, FIELD>;
pub type ExplicitString<const BIT: usize, const FIELD: u32, D = ProtoDefault> =
    ExplicitLenField<len::ProtoString, BIT, FIELD, D>;
pub type ImplicitBytes<const FIELD: u32> = ImplicitLenField<len::ProtoBytes, FIELD>;
pub type ExplicitBytes<const BIT: usize, const FIELD: u32, D = ProtoDefault> =
    ExplicitLenField<len::ProtoBytes, BIT, FIELD, D>;
