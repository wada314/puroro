//! Composable field types for generated protobuf messages.
//!
//! Each proto field (except oneof variants) maps to a self-contained runtime
//! type in this module. Field getters/setters/encode/decode only touch:
//!
//! 1. The field's own struct member, and
//! 2. [`MessageParts`] / [`MessagePartsMut`] (presence bitfield, allocator,
//!    unknown-field buffer).
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
//! | `scalar` | IMPLICIT / EXPLICIT singular scalars |
//! | `string` | IMPLICIT / EXPLICIT `string` |
//! | `bytes` | IMPLICIT / EXPLICIT `bytes` |
//! | `repeated` | packed / expanded repeated fields |
//! | `message` | singular nested message |
//! | `enum_` | open / closed enum |
//! | [`oneof::OneofSlot`] | `oneof` group (mutual exclusion) |
//! | [`common::MessageCommon`] | presence + unknown + allocator |

pub mod common;
pub mod oneof;
pub mod presence;

pub use common::{MessageCommon, MessageParts, MessagePartsMut};
pub use oneof::OneofSlot;
pub use presence::PresenceBits;
