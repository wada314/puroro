//! Opt-in schema identity for generated messages (`descriptors` feature).
//!
//! Enable the same-named feature on a generated crate (and thus on `puroro`) to
//! get an embedded [`FileDescriptorSetBytes`] plus [`DescribedMessage`]. Light
//! identity ([`crate::Message::FULL_NAME`]) stays available without this feature.
//! See `DESIGN.md` §3.1.

/// Encoded `google.protobuf.FileDescriptorSet` bytes.
///
/// Generated crates expose one crate-wide constant of this type behind the
/// `descriptors` feature. Callers that speak gRPC server reflection (or similar)
/// pass [`as_bytes`](Self::as_bytes) through to the wire.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FileDescriptorSetBytes(&'static [u8]);

impl FileDescriptorSetBytes {
    /// Wrap a static byte slice as a descriptor set.
    pub const fn new(bytes: &'static [u8]) -> Self {
        Self(bytes)
    }

    /// Raw `FileDescriptorSet` payload.
    pub const fn as_bytes(self) -> &'static [u8] {
        self.0
    }
}

/// Generated message that can name its declaring `.proto` file.
///
/// The crate-wide [`FileDescriptorSetBytes`] holds the encoded set. This trait
/// only records which `FileDescriptorProto.name` declared the message so a
/// consumer can find that file inside the set. Nested messages use the outer
/// file's name.
pub trait DescribedMessage: crate::Message {
    /// `FileDescriptorProto.name` of the file that declared this message.
    ///
    /// This is a protobuf logical path relative to the source-tree root
    /// (e.g. `foo/bar/baz.proto`), not a host OS path. Separators are always
    /// `/`; protoc normalizes Windows `\` to `/` before it reaches the plugin.
    const FILE_NAME: &'static str;
}
