# Puroro Documentation

This directory contains the current documentation for Puroro, a Rust-idiomatic Protocol Buffers implementation.

## Current Documents

### Design Documents
- **`comprehensive-field-descriptor-design.md`** - Current field descriptor design using trait-based approach
- **`design-discussion.md`** - Current design decisions, implementation status, and discussion history
- **`enum-implementation-design.md`** - Comprehensive design decisions for protobuf enum implementation
- **`field-context-design.md`** - Current field operations design with SharedFields approach
- **`field-operations-design.md`** - Current field operations implementation details
- **`lazy-implementation-notes.md`** - Critical properties for lazy deserialization: field overwriting behavior and parsing strategies
- **`lazy-parsing-state-design.md`** - Current design + invariants for lazy parser state and nested message handling
- **`lazy-parsing-state-design-discussion.md`** - Condensed discussion history for the lazy parser state design
- **`lazy-parsing-next-steps.md`** - Notes and follow-ups specifically for lazy repeated parsing

### Development Planning
- **`next-steps.md`** - Current development priorities and milestones

## Historical Documents

- **`historical/`** - Contains superseded exploration documents (see `historical/README.md`)
- **`session-reports/`** - Contains detailed session reports (see `session-reports/README.md`)

## Current Status

**Last Updated**: 2026-01-22  
**Implementation Status**: Multiple runtime pieces are in progress. Lazy parsing is implemented as a current discussion focus, and serialization/deserialization + code generator are also in progress.

### ✅ Completed
- Field operations system with trait-based approach
- FieldType struct with type-level metadata encoding
- String and scalar field implementations (ImplicitOptional and ExplicitOptional)
- SharedFields with BitArray for presence tracking
- Reference generated-style code in sandbox (including lazy parser + lazy repeated)

### 🚧 Current Focus
- Serialization/deserialization implementation
- Code generator development (puroro-codegen is still largely a stub)

## Quick Reference

For the most up-to-date information, start with:
1. `design-discussion.md` - Overview of current design decisions
2. `comprehensive-field-descriptor-design.md` - Detailed field descriptor design
3. `enum-implementation-design.md` - Protobuf enum implementation design decisions
4. `next-steps.md` - Current development priorities
