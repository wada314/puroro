//! Type aliases for singular varint fields ([`SingularField`] with varint markers).

use crate::defaults::ProtoDefault;
use crate::fields::shared::field_presence::{Explicit, Implicit};
use crate::fields::singular::field::{SingularField, SingularFieldMut, SingularFieldRef};
use crate::fields::wire::varint;

/// Singular varint field — alias of [`SingularField`].
pub type SingularVarintField<T, P, const FIELD: u32, D = ProtoDefault> =
    SingularField<T, P, FIELD, D>;

/// Bound shared view for a singular varint field.
pub type SingularVarintFieldRef<'a, T, P, const FIELD: u32, D, Pb, A> =
    SingularFieldRef<'a, T, P, FIELD, D, Pb, A>;

/// Bound mutation view for a singular varint field.
pub type SingularVarintFieldMut<'f, 'c, T, P, const FIELD: u32, D, Pb, A> =
    SingularFieldMut<'f, 'c, T, P, FIELD, D, Pb, A>;

pub type SingularVarint<T, P, const FIELD: u32, D = ProtoDefault> =
    SingularVarintField<T, P, FIELD, D>;

pub type ImplicitVarintField<T, const FIELD: u32> = SingularVarintField<T, Implicit, FIELD>;
pub type ExplicitVarintField<T, const BIT: usize, const FIELD: u32, D = ProtoDefault> =
    SingularVarintField<T, Explicit<BIT>, FIELD, D>;

pub type ImplicitVarint<T, const FIELD: u32> = ImplicitVarintField<T, FIELD>;
pub type ExplicitVarint<T, const BIT: usize, const FIELD: u32, D = ProtoDefault> =
    ExplicitVarintField<T, BIT, FIELD, D>;

pub type ImplicitInt32<const FIELD: u32> = ImplicitVarintField<varint::ProtoInt32, FIELD>;
pub type ExplicitInt32<const BIT: usize, const FIELD: u32, D = ProtoDefault> =
    ExplicitVarintField<varint::ProtoInt32, BIT, FIELD, D>;
pub type ImplicitEnum<E, const FIELD: u32> = ImplicitVarintField<varint::ProtoEnum<E>, FIELD>;
pub type ExplicitEnum<E, const BIT: usize, const FIELD: u32, D = ProtoDefault> =
    ExplicitVarintField<varint::ProtoEnum<E>, BIT, FIELD, D>;
