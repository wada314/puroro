# Next Steps for Puroro Development

**Last Updated**: 2025-10-17  
**Status**: Design phase complete, ready for implementation

## Priority 1: Core Runtime Implementation

### 1.1 Basic Serialization/Deserialization
- [ ] Implement `Message::parse_from_bytes()` for simple fields
  - Use `protobuf-core` for wire format I/O
  - Handle varint encoding/decoding
  - Field tag and wire type parsing
- [ ] Implement `Message::write_to_bytes()` for simple fields
  - Wire format writing
  - Field tag encoding
- [ ] Implement `Message::compute_size()` accurately
- [ ] Test with round-trip serialization

**Files to modify**:
- `sandbox/src/generated/person.rs` (implement TODOs)
- Create tests in `sandbox/tests/serialization.rs`

### 1.2 Field Types Support

#### Scalar Fields (Proto3)
- [x] String (already in sandbox)
- [x] int32 (already in sandbox)
- [ ] int64, uint32, uint64
- [ ] sint32, sint64 (zigzag encoding)
- [ ] fixed32, fixed64, sfixed32, sfixed64
- [ ] bool
- [ ] bytes
- [ ] float, double
- [ ] enum

#### Complex Fields
- [ ] Repeated fields (Vec<T>)
  - Update traits: `add_*()` methods in `PersonAppend`
  - `clear_*()` in `PersonMut`
  - `*_mut()` for direct Vec access
- [ ] Map fields (HashMap<K, V>)
  - `insert_*()` in `PersonAppend`
  - `get_*()`, `*_mut()` methods
- [ ] Nested messages
  - Recursive Message trait usage
  - Ownership considerations
- [ ] Oneof fields
  - Enum-based representation
  - Pattern matching support

## Priority 2: Code Generator

### 2.1 Basic Code Generation
- [ ] Parse FileDescriptorSet from protoc
  - Use `protoc-plugin-by-closure` (git dependency)
  - Parse descriptor.proto messages
- [ ] Generate trait definitions (6 traits per message)
- [ ] Generate struct definitions
  - Field types
  - Bitflags calculation
- [ ] Generate trait implementations
  - Infallible: Person, PersonAppend, PersonMut
  - Fallible: PersonTry, PersonAppendTry, PersonTryMut

**Files to create/modify**:
- `puroro-codegen/src/generator.rs`
- `puroro-codegen/src/descriptor.rs` (parse protobuf descriptors)
- `puroro-codegen/src/render.rs` (code rendering)

### 2.2 Advanced Features
- [ ] Conditional `_opt()` getter generation
  - Detect zero-value vs custom defaults
  - Only generate for zero-value defaults
- [ ] Module structure (packages → mod hierarchy)
- [ ] Proto2 support (custom defaults, required fields)
- [ ] Proto3 optional fields
- [ ] Editions support

## Priority 3: Advanced Runtime Features

### 3.1 Performance Optimizations
- [ ] Arena allocation support
- [ ] Zero-copy deserialization (views)
- [ ] Lazy deserialization implementation
  - Implement `PersonTry` with actual lazy loading
  - Cache deserialized fields

### 3.2 Additional Implementations
- [ ] `PersonLazy` - Lazy deserialization
- [ ] `PersonView<'a>` - Zero-copy view
- [ ] `PersonCompact` - Code size optimized
- [ ] `PersonOpen` - Public fields (for pattern matching use cases)

## Priority 4: Testing & Quality

### 4.1 Test Suite
- [ ] Round-trip serialization tests
- [ ] Compatibility tests with official protobuf
  - Test against protoc-generated test data
  - Interoperability with C++/Go/Python implementations
- [ ] Fuzzing tests
- [ ] Benchmark suite

### 4.2 Documentation
- [ ] API documentation (rustdoc)
- [ ] User guide
- [ ] Migration guide (from prost/protobuf-rust)
- [ ] Examples directory

## Priority 5: Ecosystem Integration

### 5.1 Serde Integration
- [ ] Optional Serde derive support
- [ ] Handle `_opt()` getters with `skip_serializing_if`
- [ ] JSON mapping (match ProtoJSON spec)

### 5.2 Build Integration
- [ ] Cargo build script support
- [ ] `build.rs` examples
- [ ] Documentation on build setup

## Open Questions / Future Discussions

### Design Questions
- [ ] How to handle unknown fields?
- [ ] Extension support?
- [ ] Service/RPC generation?
- [ ] Reflection API?

### Performance vs Ergonomics
- [ ] Should we provide `_mut()` methods in `PersonAppend`?
  - Pro: Performance (avoid clone)
  - Con: Can delete items from Vec/HashMap
- [ ] Builder pattern integration?
- [ ] Async I/O support?

## Dependencies

### External Crates to Add
- [ ] `protobuf-core` ✅ (already added)
- [ ] `protoc-plugin-by-closure` ✅ (git dep added)
- [ ] Serde (optional feature)
- [ ] `bytes` crate for zero-copy?

### Development Dependencies
- [ ] `quickcheck` or `proptest` for property testing
- [ ] `criterion` for benchmarking
- [ ] Official protobuf for compatibility testing

## Milestones

### Milestone 1: MVP (Minimum Viable Product)
- Basic message serialization/deserialization
- Simple fields only (string, int32, int64, bool)
- Code generator for simple messages
- 6-trait hierarchy working

**Target**: Working hello world example

### Milestone 2: Feature Complete (Proto3)
- All scalar types
- Repeated fields, maps
- Nested messages
- Full code generator

**Target**: Can generate code for real-world proto3 files

### Milestone 3: Production Ready
- Proto2 support
- Performance optimizations
- Comprehensive test suite
- Documentation

**Target**: 1.0 release

---

## Current Status

**✅ Completed**:
- Design phase (6-trait hierarchy)
- Memory layout strategy (bitflags)
- Sandbox hand-written examples
- Test framework (14 tests passing)

**🚧 In Progress**:
- Nothing (ready to start implementation!)

**📋 Next Immediate Action**:
- Implement serialization/deserialization in sandbox
- OR start code generator for simple messages

Choose based on preference:
- **Bottom-up**: Implement runtime first, ensure correctness
- **Top-down**: Implement generator first, iterate on generated code
- **Middle-out**: Do both in parallel

