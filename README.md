# Puroro

A Rust-idiomatic implementation of Google Protocol Buffers.

## Design Principles

1. **Precise Protobuf Spec Support**: Implement Protocol Buffers specification accurately and completely
2. **Rust-idiomatic Interface**: Design APIs that feel natural in Rust, not restricted by C/C++ interfaces
   - Leverage Rust's type system and trait-based design
   - Use Rust idioms (Result, Option, iterators)
   - Memory safety without sacrificing performance
   - Type-level safety (make illegal states unrepresentable)

## Key Features

### Trait-Based Design (6 Traits Per Message)

```rust
// Read-only access
trait Person {
    fn name(&self) -> &str;
    fn age(&self) -> i32;
    fn has_age(&self) -> bool;
}

// Append-only access (most common use case)
trait PersonAppend: Person {
    fn set_name(&mut self, v: impl Into<String>);
    fn set_age(&mut self, v: i32);
}

// Full mutable access (destructive operations)
trait PersonMut: PersonAppend {
    fn clear_name(&mut self);
}
```

- **Three levels of mutability**: Read, Append, Full-Mut
- **Fallible variants**: `PersonTry`, `PersonAppendTry`, `PersonTryMut` for lazy/validated implementations
- **Type-safe API contracts**: Functions can require only the access level they need

### Memory-Efficient Implementation

- Uses bitflags for presence tracking instead of `Option<T>`
- Avoids memory overhead for optional fields
- Example: 32 optional i32 fields = 4 bytes (bitflags) + 128 bytes (values) = 132 bytes
- vs. naive `Option<i32>` = 32 × 8 bytes = 256 bytes

### Conditional Optional Getters

- `_opt()` getters **only for zero-default fields**
- Type system prevents misuse with custom defaults
- Compile-time error if used incorrectly

```rust
// Zero-default field
let age: Option<i32> = person.age_opt();  // ✅ Compiles

// Custom default field
let score: Option<i32> = person.score_opt();  // ❌ Compile error
// help: field `score` has a non-zero default value (72)
```

## Project Structure

- **`puroro/`**: Runtime library used by generated code
- **`puroro-codegen/`**: Code generator (protoc plugin)
- **`sandbox/`**: Design exploration and hand-written examples
- **`doc/`**: Design discussions and documentation

## Current Status

🚧 **Work in Progress** - Design phase complete, implementation in progress

**Completed**:
- ✅ Trait hierarchy design (6 traits)
- ✅ Memory layout strategy (bitflags)
- ✅ API design (closed struct approach)
- ✅ Hand-written example implementation

**Next Steps**:
- [ ] Implement serialization/deserialization
- [ ] Code generator implementation
- [ ] Support for repeated fields, maps, oneofs
- [ ] Support for nested messages

See [Design Discussion](doc/design-discussion.md) for detailed design decisions and rationale.

## Experimental Allocator Support

- Nightly-only experimental crate `allocator_extras` hosts allocator-aware utilities such as `String<A>`
- Activate the experiment with `cargo +nightly test -p allocator_extras`
- Feature flag `allocator-extras` on `puroro` pulls the allocator-aware types into the main crate for further experiments

## Documentation

- [Design Discussion History](doc/design-discussion.md) - Complete record of design decisions
- [Sandbox Examples](sandbox/) - Hand-written examples demonstrating the API

## License

Apache-2.0

