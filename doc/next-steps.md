# Next Steps for Puroro Development

**Last Updated**: 2026-01-22  
**Status**: Lazy parsing infrastructure implemented; serialization/deserialization remains the main milestone

## Current Status Summary

### ✅ Major Milestones Completed

**Core Field Operations System** (2025-10-21):
- ✅ Field label types (ImplicitOptional, ExplicitOptional, Repeated, Map)
- ✅ FieldType struct with type-level metadata encoding
- ✅ Field trait with comprehensive operations (set, get, clear, is_present)
- ✅ String field implementations (both ImplicitOptional and ExplicitOptional)
- ✅ Scalar field implementations (i32, i64, u32, u64, f32, f64, bool)
- ✅ SharedFields with BitArray for presence tracking
- ✅ Reference generated-style code in sandbox (tests passing, including lazy parsing examples)

**Memory Layout Optimization**:
- ✅ Stack-allocated SharedFields with BitArray
- ✅ Optimized bit indices for ExplicitOptional fields
- ✅ Size-descending field ordering
- ✅ Zero heap overhead for presence tracking

**Type Safety**:
- ✅ Compile-time field number validation
- ✅ Presence bit index encoding at type level
- ✅ Field type information available at compile time
- ✅ Trait-based operations with type safety

## Priority 1: Serialization/Deserialization (Current Focus)

### 1.1 Basic Serialization/Deserialization
- [ ] Implement `Message::parse_from_bytes()` for simple fields
  - Use `protobuf-core` for wire format I/O
  - Handle varint encoding/decoding
  - Field tag and wire type parsing
  - Support for ImplicitOptional and ExplicitOptional fields
- [ ] Implement `Message::write_to_bytes()` for simple fields
  - Wire format writing
  - Field tag encoding
  - Presence bit handling for ExplicitOptional fields
- [ ] Implement `Message::compute_size()` accurately
- [ ] Test with round-trip serialization

**Files to modify**:
- `sandbox/src/generated/person.rs` (implement Message trait)
- Create tests in `sandbox/tests/serialization.rs`

### 1.2 Field Types Support (Expanded)

#### Scalar Fields (Proto3) - ✅ COMPLETED
- ✅ String (both ImplicitOptional and ExplicitOptional)
- ✅ int32, int64, uint32, uint64 (both variants)
- ✅ sint32, sint64 (zigzag encoding) - ✅ COMPLETED
- ✅ fixed32, fixed64, sfixed32, sfixed64 - ✅ COMPLETED
- ✅ bool (both variants) - ✅ COMPLETED
- ✅ float, double (both variants) - ✅ COMPLETED
- [ ] bytes (both variants)

#### Complex Fields - 🚧 NEXT PRIORITY
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
- [ ] Enum fields
  - Enum type generation
  - Wire format handling
- [ ] Oneof fields
  - Enum-based representation
  - Pattern matching support

## Priority 2: Code Generator (Parallel Development)

### 2.1 Basic Code Generation
- [ ] Parse FileDescriptorSet from protoc
  - Use `protoc-plugin-by-closure` (git dependency)
  - Parse descriptor.proto messages
- [ ] Generate trait definitions (6 traits per message)
- [ ] Generate struct definitions with FieldType
  - Field types with type-level metadata
  - Bitflags calculation for SharedFields
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
- ✅ Lazy parsing infrastructure (incremental parsing)
  - `MessageParserStateRef` + `FieldIterator` in `puroro/src/lazy_parser.rs`
  - `LazyRepeated` in `puroro/src/repeated_lazy.rs`
  - Reference lazy implementations in `sandbox/src/generated/*.rs`

- [ ] Lazy deserialization ergonomics / expansion
  - Extend lazy support beyond the reference sandbox code
  - Decide whether `Repeated` should keep `get()`/`len()` (random access) or move to a streaming-only API

### 3.2 Additional Implementations
- [ ] `PersonLazy` - Lazy deserialization (beyond sandbox reference)
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

### Milestone 1: MVP (Minimum Viable Product) - ✅ COMPLETED
- ✅ Basic field operations (set, get, clear, is_present)
- ✅ Simple fields (string, int32, int64, bool, float, double)
- ✅ Trait-based field operations with type safety
- ✅ 6-trait hierarchy working
- ✅ Generated code integration in sandbox

**Status**: ✅ **COMPLETED** (2025-10-21)

### Milestone 2: Serialization Support (Current Target)
- [ ] Basic message serialization/deserialization
- [ ] Support for ImplicitOptional and ExplicitOptional fields
- [ ] Round-trip serialization tests
- [ ] Wire format compatibility

**Target**: Working serialization for simple fields

### Milestone 3: Feature Complete (Proto3)
- [ ] All scalar types with serialization
- [ ] Repeated fields, maps
- [ ] Nested messages
- [ ] Full code generator
- [ ] Enum support

**Target**: Can generate code for real-world proto3 files

### Milestone 4: Production Ready
- [ ] Proto2 support
- [ ] Performance optimizations
- [ ] Comprehensive test suite
- [ ] Documentation
- [ ] Advanced features (oneof, extensions)

**Target**: 1.0 release

---

## Current Status

**✅ Completed**:
- Core field operations system with trait-based approach
- FieldType struct with type-level metadata encoding
- String and scalar field implementations (ImplicitOptional and ExplicitOptional)
- SharedFields with BitArray for presence tracking
- Generated code integration in sandbox (17 tests passing)
- Memory layout optimization (stack-allocated, size-descending ordering)
- Type safety with compile-time validation

**🚧 In Progress**:
- Serialization/deserialization implementation
- Code generator development

**📋 Next Immediate Actions**:
1. **Implement Message trait methods** in sandbox for serialization/deserialization
2. **Start code generator** for trait and struct generation
3. **Add support for repeated fields** and maps

**Development Strategy**:
- **Bottom-up**: Implement runtime first, ensure correctness
- **Top-down**: Implement generator first, iterate on generated code
- **Middle-out**: Do both in parallel (recommended)

**Key Achievements**:
- ✅ **Type Safety**: 100% compile-time validation of field operations
- ✅ **Memory Efficiency**: Stack-allocated SharedFields with BitArray (1 byte for ≤8 fields)
- ✅ **Performance**: Zero runtime overhead for field operations
- ✅ **Scalability**: Supports unlimited fields with minimal overhead
- ✅ **Code Simplicity**: Single-line operations in generated code

