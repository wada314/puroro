<!-- Generated to track allocator adoption targets -->
# Allocator-Aware Refactor Targets

This note captures the current `puroro` runtime structures that allocate and therefore
need to absorb the `allocator_extras` types and traits.

## Core Runtime (`puroro/src`)

- `field_ops::StringFieldWrapper` wraps `std::string::String` and should swap to
  `allocator_extras::String<A>` plus the allocator-aware trait implementations.
- `field_ops::BytesFieldWrapper` currently holds `Vec<u8>`; we need a `Vec<u8, A>` alias
  (likely via a helper in `allocator_extras::util`).
- `field_ops::MessageFieldWrapper` stores `Option<Box<M>>`; allocator propagation
  requires `Option<Box<M, A>>` and corresponding `CloneIn`/`DefaultIn` bounds.
- `field_ops::FieldStorage` relies on `T: Default`; allocator-aware variants will
  need `DefaultIn` + constructor paths that accept an allocator.
- `shared::SharedFields` hints at future allocator storage (`Vec<u8, A>` for unknown
  fields); when we add the allocator parameter this struct should own the allocator
  instance so that field ops can stay signature-compatible.
- `lib::Message` trait methods use `Vec<u8>` and `std::string::FromUtf8Error`; we
  should migrate to allocator-aware return types (likely `Vec<u8, A>` and
  `allocator_extras::FromUtf8Error<A>`).

## View Layer (`puroro/src/view.rs`)

- `view::ViewCow` uses `Box<T>` for owned data and should generalise to `Box<T, A>`
  (or a thin wrapper around it). The clone-on-write semantics will depend on
  `CloneIn`.

## Error Types

- `error::Error::InvalidUtf8` wraps `std::string::FromUtf8Error`; we will need to
  convert to the allocator-aware error once `String<A>` is integrated.

## Follow-up Questions

- Do we store an allocator per message instance (likely in `SharedFields`) or pass
  it through every API boundary?
- How do we expose defaults for `A = Global` while allowing callers to override on
  a per-constructor basis?

This map will be updated as we refactor each module; it serves as the checklist for
the next steps in the allocator integration plan.

