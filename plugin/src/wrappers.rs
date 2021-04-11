//! Wrappers are, literally, wrappers around the `***DescriptorProto`s.
//!
//! # Implementation strategy
//! The wrappers implement the methods to get the values of protos,
//! and maybe do a little more work like converting the proto type name to
//! Rust-styled type name. We do not consider about modifying the data.
//! Those little-more-work results may be cached into `OnceCell<T>` field for
//! performance, and for allowing us to forget about the field initialization
//! order problem. So most of the field of the wrappers should be wrapped into
//! the `OnceCell<T>` wrapper. The exceptions are, the reference to the source
//! proto structure, the reference to the context class, and the children
//! wrappers.
//!
mod r#enum;
mod field;
mod file;
mod message;
pub use field::FieldDescriptor;
pub use file::FileDescriptor;
pub use message::MessageDescriptor;
pub use r#enum::{EnumDescriptor, EnumValueDescriptor};
