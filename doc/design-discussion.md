# Puroro Design Discussion History

This document records the design discussions and decisions for the Puroro project - a Rust implementation of Google Protocol Buffers.

## 2025-10-17: Project Restart from Scratch

### Background
- Starting completely from scratch on `scratch3` branch
- This is a hobby project for learning purposes
- Official Rust protobuf implementations already exist (official & prost)
- We have AI assistant (Claude) to help with the implementation

### Project Structure Goals
The project should consist of at least 2 main libraries:

1. **Code Generator Library**: Generates Rust code from .proto files
2. **Runtime Library**: Utility library used by the generated code

### External Dependencies
We plan to use two utility crates to reduce the amount of code we need to write:

#### `protobuf-core`
- **Purpose**: Low-level wire format I/O utilities for Protocol Buffers
- **Provides**:
  - Varint encoding/decoding
  - Field-level reading/writing (Tag, FieldNumber, WireType)
  - Basic wire format constants and utilities
  - Primitive building blocks for parsers/serializers
- **Link**: https://docs.rs/protobuf-core/latest/protobuf_core/

#### `protoc-plugin-by-closure`
- **Purpose**: Convenient wrapper for running protoc with plugin code as a closure
- **Provides**:
  - `Protoc`: Run protoc with actual filesystem
  - `ProtocOnMemory`: Run protoc without touching filesystem (useful for testing/macros)
  - Takes `CodeGeneratorRequest` bytes and returns `CodeGeneratorResponse` bytes
- **Link**: https://wada314.github.io/protoc-plugin-by-closure/

**Benefits**: These crates handle the tedious low-level details, allowing us to focus on the higher-level logic of code generation and runtime message handling.

### Current Workspace Structure
```
puroro/           - Runtime library
codegen/          - Code generator
inline/           - Inline macro related
tests/            - Test suite
```

### Design Principles (Decided)

1. **Precise Protobuf Spec Support**: Implement Protocol Buffers specification accurately and completely
2. **Rust-idiomatic Interface**: Design APIs that feel natural in Rust, not restricted by C/C++ interfaces like the official implementation
   - Leverage Rust's type system
   - Use Rust idioms (Result, Option, iterators, etc.)
   - Modern Rust patterns (builder pattern, trait-based design)
   - Memory safety without sacrificing performance

### Crate Structure (Decided)

```
puroro/              - Runtime library (used by generated code)
puroro-codegen/      - Code generator (protoc plugin)
```

**Minimum viable crates**: These two are the essential building blocks.

### Progress Log

#### 2025-10-17: Initial Setup Complete ✓

1. **Cleaned up**: Removed all existing code to start fresh
2. **Created basic structure**:
   - Workspace with `puroro` and `puroro-codegen` crates
   - Basic `Message` trait in runtime library
   - Error types for both crates
   - Skeleton for protoc plugin binary
3. **Dependencies configured**:
   - `protobuf-core` 0.1 for wire format I/O (from crates.io)
   - `protoc-plugin-by-closure` (from git repository)
   - `.cargo/config.toml` created with bindeps enabled
4. **Build status**: ✓ Compiles successfully

**Important discovery about bindeps**:
- ✅ Bindeps work with local path dependencies
- ✅ Bindeps work with git dependencies
- ❌ Bindeps appear NOT to work with crates.io dependencies
- This may be related to [Cargo Issue #12555](https://github.com/rust-lang/cargo/issues/12555), which tracks crates.io support for bindeps
- Current workaround: Use git or local path dependencies for crates that require bindeps

**Next steps**: 
- Design the core runtime API (Message trait methods, field types)
- Implement basic serialization/deserialization
- Design code generator architecture

---

## API Design Decisions

### Field Access Pattern: Open Struct vs Closed Struct

We need to decide how generated message structs should expose their fields.

#### Option 1: Open Struct (Public Fields)

```rust
pub struct Person {
    pub name: String,
    pub age: i32,
    pub email: String,
}

// Usage
let mut person = Person::new();
person.name = "Alice".to_string();
person.age = 30;
```

**Pros:**
- ✅ **Simpler, more idiomatic Rust**: Direct field access is the Rust way
- ✅ **Less generated code**: No need to generate getter/setter methods
- ✅ **Better performance**: No function call overhead
- ✅ **Pattern matching**: Can use struct patterns in `match`, `if let`, etc.
- ✅ **Struct update syntax**: Can use `Person { name: "Bob".into(), ..person }`
- ✅ **Cleaner API**: No `get_name()`, `set_name()`, `mut_name()` noise
- ✅ **Zero-cost abstraction**: Direct memory access

**Cons:**
- ❌ **No validation**: Can't validate field values on write
- ❌ **No hooks**: Can't add side effects when fields change
- ❌ **Breaking changes**: Changing field types requires breaking API changes
- ❌ **Less compatible with C++ API**: Different from official protobuf libraries
- ❌ **No lazy initialization**: Can't defer expensive field computation
- ❌ **Cannot hide internal implementation**: Exposed fields must match internal representation exactly
- ❌ **Inefficient optional fields**: Optional fields would be `Option<T>`, which is memory inefficient
  - Example: `Option<i32>` is 8 bytes (4 for value + 4 for discriminant), but could be 4 bytes + 1 bit in a bitflag
  - For messages with many optional fields, this overhead compounds significantly
  - Cannot use bitflags or other space-efficient representations for presence tracking

#### Option 2: Closed Struct (Private Fields with Getters/Setters)

```rust
pub struct Person {
    name: String,
    age: i32,
    email: String,
}

impl Person {
    pub fn name(&self) -> &str { &self.name }
    pub fn set_name(&mut self, v: String) { self.name = v; }
    pub fn mut_name(&mut self) -> &mut String { &mut self.name }
    
    pub fn age(&self) -> i32 { self.age }
    pub fn set_age(&mut self, v: i32) { self.age = v; }
    
    pub fn email(&self) -> &str { &self.email }
    pub fn set_email(&mut self, v: String) { self.email = v; }
    pub fn mut_email(&mut self) -> &mut String { &mut self.email }
}

// Usage
let mut person = Person::new();
person.set_name("Alice".to_string());
person.set_age(30);
```

**Pros:**
- ✅ **Validation opportunity**: Can validate values in setters
- ✅ **Hook opportunity**: Can add side effects (dirty flags, observers, etc.)
- ✅ **Future flexibility**: Can change internal representation without breaking API
- ✅ **Compatible with C++ style**: Similar to official protobuf libraries
- ✅ **Lazy initialization**: Can defer expensive computations
- ✅ **Efficient memory layout**: Can use optimized internal representations
  - Example: Use bitflags for optional field presence tracking instead of `Option<T>`
  - Can pack multiple boolean fields into a single byte
  - Internal representation can be optimized independently of public API

**Cons:**
- ❌ **More verbose**: Requires more generated code (getter/setter for each field)
- ❌ **Less idiomatic Rust**: Not the Rust way for simple data structures
- ❌ **Performance overhead**: Function call overhead (though likely inlined)
- ❌ **No pattern matching**: Can't use struct patterns
- ❌ **No struct update syntax**: Can't use `..` spread operator
- ❌ **Cluttered API**: Many methods (`get_*`, `set_*`, `mut_*`, `has_*`, `clear_*`)
- ❌ **Not zero-cost**: Extra abstraction layer

#### Option 3: Hybrid Approach

Could also consider:
- Public fields for scalar types (i32, String, etc.)
- Methods for complex operations (repeated fields, oneofs, etc.)
- Builder pattern for construction

#### Decision: TBD

**TODO**: Discuss and decide which approach aligns best with our "Rust-idiomatic" design principle.

---

## Discussion Topics

### Topic: Understanding Utility Crates
**Date**: 2025-10-17

**Questions**:
- What functionality does `protobuf-core` provide?
- What functionality does `protoc-plugin-by-closure` provide?
- Are these external crates or part of this repository?
- How should we integrate them into our new design?


