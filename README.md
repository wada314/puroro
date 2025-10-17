# Puroro

A Rust-idiomatic implementation of Google Protocol Buffers.

## Design Principles

1. **Precise Protobuf Spec Support**: Implement Protocol Buffers specification accurately and completely
2. **Rust-idiomatic Interface**: Design APIs that feel natural in Rust, not restricted by C/C++ interfaces
   - Leverage Rust's type system
   - Use Rust idioms (Result, Option, iterators, etc.)
   - Modern Rust patterns (builder pattern, trait-based design)
   - Memory safety without sacrificing performance

## Project Structure

- **`puroro/`**: Runtime library used by generated code
- **`puroro-codegen/`**: Code generator (protoc plugin)

## Status

🚧 **Work in Progress** - This project is being rebuilt from scratch with AI assistance.

See [Design Discussion](doc/design-discussion.md) for detailed design decisions and progress.

## License

Apache-2.0

