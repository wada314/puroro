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

### Next Steps
1. Understand the utility crates (`protobuf-core` and `protoc-plugin-by-closure`)
2. Define the basic architecture and design principles
3. Decide what to keep/delete from the existing code
4. Start implementing from the foundation

---

## Discussion Topics

### Topic: Understanding Utility Crates
**Date**: 2025-10-17

**Questions**:
- What functionality does `protobuf-core` provide?
- What functionality does `protoc-plugin-by-closure` provide?
- Are these external crates or part of this repository?
- How should we integrate them into our new design?


