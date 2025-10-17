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
   - `protobuf-core` 0.1 for wire format I/O
   - `.cargo/config.toml` created with bindeps enabled
   - Note: `protoc-plugin-by-closure` 0.2.0 has bindeps issues, temporarily disabled
4. **Build status**: ✓ Compiles successfully

**Next steps**: 
- Design the core runtime API (Message trait methods, field types)
- Implement basic serialization/deserialization
- Design code generator architecture

---

## Discussion Topics

### Topic: Understanding Utility Crates
**Date**: 2025-10-17

**Questions**:
- What functionality does `protobuf-core` provide?
- What functionality does `protoc-plugin-by-closure` provide?
- Are these external crates or part of this repository?
- How should we integrate them into our new design?


