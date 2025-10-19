# Sandbox - API Design Exploration

This directory contains hand-written code that represents our ideal API design for Puroro.

## Purpose

Before implementing the code generator, we:
1. Hand-write the code we want the generator to produce
2. Verify it compiles and works correctly
3. Use it as a specification for the code generator

## Structure

```
sandbox/
├── protos/
│   └── person.proto        # Sample .proto file
├── src/
│   ├── lib.rs
│   ├── generated.rs
│   └── generated/
│       └── person.rs       # Hand-written "generated" code
└── tests/
    └── basic.rs            # Integration tests (14 tests, all passing)
```

## Example: Person Message

### Proto Definition

```proto
syntax = "proto3";

package example;

message Person {
  string name = 1;
  int32 age = 2;
  string email = 3;
}
```

### Generated Traits (6 Total)

#### 1. `Person` - Read-only Access
```rust
pub trait Person {
    fn name(&self) -> &str;
    fn age(&self) -> i32;
    fn email(&self) -> &str;
    fn has_name(&self) -> bool;
    fn has_age(&self) -> bool;
    fn has_email(&self) -> bool;
}
```

#### 2. `PersonAppend` - Append Operations
```rust
pub trait PersonAppend: Person {
    fn set_name(&mut self, v: impl Into<String>);
    fn set_age(&mut self, v: i32);
    fn set_email(&mut self, v: impl Into<String>);
}
```

#### 3. `PersonMut` - Full Mutation
```rust
pub trait PersonMut: PersonAppend {
    fn clear_name(&mut self);
    fn clear_age(&mut self);
    fn clear_email(&mut self);
}
```

#### 4-6. Fallible Variants
- `PersonTry` - All getters return `Result<T, Error>`
- `PersonAppendTry` - Fallible setters
- `PersonTryMut` - Fallible clear operations

### Implementation: PersonImpl

```rust
pub struct PersonImpl {
    _has_bits: u32,  // Efficient presence tracking
    name: String,
    age: i32,
    email: String,
}
```

**Memory Layout**:
- 4 bytes for presence bits (up to 32 fields)
- Fields stored directly (no `Option<T>` overhead)
- Total: ~36 bytes vs ~72 bytes with `Option<T>` for all fields

## Usage Examples

### Read-Only Access
```rust
fn print_person(p: &impl Person) {
    println!("{} is {} years old", p.name(), p.age());
}
```

### Append-Only (Most Common)
```rust
fn populate_profile(p: &mut impl PersonAppend) {
    p.set_name("Alice");
    p.set_age(30);
    // Cannot accidentally clear data
}
```

### Full Mutation (Rare)
```rust
fn reset_profile(p: &mut impl PersonMut) {
    p.clear_name();
    p.clear_age();
}
```

### Fallible Operations
```rust
fn try_load_person(p: &impl PersonTry) -> Result<String, Error> {
    Ok(format!("{} (age: {})", p.try_name()?, p.try_age()?))
}
```

## Tests

Run tests with:
```bash
cargo test -p sandbox
```

Current test coverage (14 tests):
- ✅ Message creation and defaults
- ✅ Getters and setters
- ✅ Presence tracking (`has_*()`)
- ✅ Clear operations
- ✅ Clone and equality
- ✅ Trait-based generic code
- ✅ Fallible operations
- ✅ Trait hierarchy (Person → PersonAppend → PersonMut)

## Design Validation

This sandbox validates:
1. **Trait hierarchy works** - 6 traits compile and compose correctly
2. **Memory layout is efficient** - Bitflags instead of `Option<T>`
3. **API is ergonomic** - Tests show natural usage patterns
4. **Type safety works** - Functions can specify exact access level needed
5. **Fallible traits work** - Error handling patterns are clear

## Next Steps

Once we're satisfied with this API:
1. Implement serialization/deserialization in `PersonImpl`
2. Build code generator to produce this code from `.proto` files
3. Expand to support repeated fields, maps, oneofs, nested messages

