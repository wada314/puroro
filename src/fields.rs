//! Composable field types for generated protobuf messages.
//!
//! Each proto field (except oneof variants) maps to a self-contained runtime
//! type in this module. Field getters/setters/encode/decode only touch:
//!
//! 1. The field's own struct member, and
//! 2. [`MessageCommon`] (presence bitfield, allocator, unknown-field buffer).
//!
//! The `protoc` plugin picks a catalog type per field, instantiates it with
//! `const` field number / presence bit / wire-encoding parameters, and emits
//! thin delegating methods on the message struct. See
//! [`IMPLEMENTATION.md`](../IMPLEMENTATION.md) §10.
//!
//! # Module layout (field catalog — implementation in progress)
//!
//! | Submodule / type | Proto pattern |
//! |---|---|
//! | [`varint`] | `VarintProtoType` markers (`ProtoInt32`, `ProtoBool`, …) |
//! | [`scalar`] | `ImplicitVarintField` / `ExplicitVarintField` |
//! | [`fixed32`] / [`fixed64`] | fixed-width scalar markers (planned) |
//! | [`len`] / [`len_field`] | `LenProtoType`, `ImplicitLenField`, `ExplicitLenField` |
//! | [`message`] | `NestedMessageField` |
//! | [`oneof::OneofSlot`] | `oneof` group (mutual exclusion) |
//! | [`common::MessageCommon`] | presence + unknown + allocator |

pub mod common;
pub mod fixed32;
pub mod fixed64;
pub mod len;
pub mod len_field;
pub mod message;
pub mod oneof;
pub mod presence;
pub mod scalar;
pub mod varint;

pub use common::MessageCommon;
pub use len::{LenProtoType, ProtoBytes, ProtoString};
pub use len_field::{
    ExplicitBytes, ExplicitLenField, ExplicitString, ImplicitBytes, ImplicitLenField,
    ImplicitString,
};
pub use message::{NestedMessage, NestedMessageField};
pub use oneof::OneofSlot;
pub use presence::PresenceBits;
pub use scalar::{ExplicitVarint, ExplicitVarintField, ImplicitEnum, ImplicitInt32, ImplicitVarint, ImplicitVarintField};
pub use varint::{ProtoBool, ProtoEnum, ProtoInt32, ProtoInt64, ProtoSint32, ProtoSint64, ProtoUInt32, ProtoUInt64, VarintProtoType};
