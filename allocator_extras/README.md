`allocator_extras` is an experimental crate for `puroro` that offers allocator-aware types and utilities targeting nightly Rust.

- Depends on the `allocator_api2` crate to prototype minimal implementations of `Allocator`-friendly types.
- Focuses on the smallest feature set necessary to validate design ideas rather than full drop-in replacements for standard library types.
- Intended for internal experimentation only (`publish = false`) and may change without notice.
