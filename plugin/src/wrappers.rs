//! Wrappers are, literally, wrappers around the `***DescriptorProto`s.
//!
//! # Implementation strategy
//! The wrappers implement the methods to get the values of protos,
//! and maybe do a little more work like converting the proto type name to
//! Rust-styled type name.
//! Those little-more-work results may be cached into `OnceCell<T>` field for
//! performance, and for allowing us to forget about the field initialization
//! order problem. So most of the field of the wrappers should be wrapped into
//! the `OnceCell<T>` wrapper. The exceptions are, the reference to the source
//! proto structure, the parent and the context class (i.e. root).
//! Because we do not need to modify the data, we can make these wrappers
//! connected tightly: The parents own the children, and the children refers
//! their parent (this is normally not possible if anything of the structure
//! is needed to be mutable).

mod r#enum;
mod field;
mod file;
mod message;
pub use field::{FieldDescriptor, FieldLabel, FieldType, NonvariantFieldType, NonnumericalFieldType};
pub use file::{DescriptorVisitor, FileDescriptor};
pub use message::MessageDescriptor;
pub use r#enum::{EnumDescriptor, EnumValueDescriptor};

// Used for the `parent` type for messages and enums.
#[derive(Debug, Clone)]
pub enum FileOrMessageRef<'c> {
    File(&'c FileDescriptor<'c>),
    Message(&'c MessageDescriptor<'c>),
}
impl<'c> FileOrMessageRef<'c> {
    pub fn package_for_child(&self) -> String {
        match self {
            &FileOrMessageRef::File(file) => file.package().to_string(),
            &FileOrMessageRef::Message(message) => format!(
                "{package}.{name}",
                package = message.package().to_string(),
                name = message.name()
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub enum EnumOrMessageRef<'c> {
    Enum(&'c EnumDescriptor<'c>),
    Message(&'c MessageDescriptor<'c>),
}
