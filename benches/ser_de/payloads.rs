//! Fixed wire payloads for benchmarks.
//!
//! Bytes are produced once from sample messages so decode benches do not
//! re-encode on every iteration. Sample messages themselves are not stored in
//! `static`s (they are not `Sync`).

use ::std::sync::LazyLock;

use ::puroro::Message;

use crate::messages::{FlatScalars, Nest, PackedInts, StringHeavy};

/// Nesting depth exercised by the nested-message benches.
pub const NEST_DEPTH: usize = 16;

/// Element count for the packed-int32 benches.
pub const PACKED_LEN: usize = 512;

pub static FLAT_BYTES: LazyLock<Vec<u8>> = LazyLock::new(|| FlatScalars::sample().encode_to_vec());

pub static NEST_BYTES: LazyLock<Vec<u8>> =
    LazyLock::new(|| Nest::sample(NEST_DEPTH).encode_to_vec());

pub static PACKED_BYTES: LazyLock<Vec<u8>> =
    LazyLock::new(|| PackedInts::sample(PACKED_LEN).encode_to_vec());

pub static STRINGS_BYTES: LazyLock<Vec<u8>> =
    LazyLock::new(|| StringHeavy::sample().encode_to_vec());

/// Sanity check used by unit tests and as a bench warm-up aid.
pub fn assert_roundtrips() {
    assert!(!FLAT_BYTES.is_empty());
    assert!(!NEST_BYTES.is_empty());
    assert!(!PACKED_BYTES.is_empty());
    assert!(!STRINGS_BYTES.is_empty());

    let flat = FlatScalars::decode(FLAT_BYTES.as_slice()).expect("flat decode");
    assert_eq!(flat.encode_to_vec(), *FLAT_BYTES);

    let nest = Nest::decode(NEST_BYTES.as_slice()).expect("nest decode");
    assert_eq!(nest.encode_to_vec(), *NEST_BYTES);

    let packed = PackedInts::decode(PACKED_BYTES.as_slice()).expect("packed decode");
    assert_eq!(packed.encode_to_vec(), *PACKED_BYTES);

    let strings = StringHeavy::decode(STRINGS_BYTES.as_slice()).expect("strings decode");
    assert_eq!(strings.encode_to_vec(), *STRINGS_BYTES);
}
