//! Compile-time default markers for generated field types.
//!
//! Fields with a proto `[default = …]` option use a message-specific ZST in the
//! generated `{message}/defaults.rs` submodule. All other fields rely on the
//! default type parameter [`ProtoDefault`] (protobuf type zero).

use crate::optional::HasDefault;

/// Protobuf type zero — the default `D` type parameter on generated field types.
///
/// Implements [`HasDefault`] for every accessor return type the runtime uses.
pub struct ProtoDefault;

impl HasDefault<i32> for ProtoDefault {
    const DEFAULT: i32 = 0;
}

impl HasDefault<i64> for ProtoDefault {
    const DEFAULT: i64 = 0;
}

impl HasDefault<u32> for ProtoDefault {
    const DEFAULT: u32 = 0;
}

impl HasDefault<u64> for ProtoDefault {
    const DEFAULT: u64 = 0;
}

impl HasDefault<bool> for ProtoDefault {
    const DEFAULT: bool = false;
}

impl<'a> HasDefault<&'a str> for ProtoDefault {
    const DEFAULT: &'a str = "";
}

impl<'a> HasDefault<&'a [u8]> for ProtoDefault {
    const DEFAULT: &'a [u8] = &[];
}
