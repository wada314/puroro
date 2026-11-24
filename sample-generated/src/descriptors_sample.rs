//! Sample of the opt-in `descriptors` feature surface.
//!
//! Production plugin output puts `FILE_DESCRIPTOR_SET` in a crate-root
//! `descriptors` module. This hand-written crate mirrors that shape.

/// Crate-wide descriptor set.
///
/// Empty stub: the sample is not produced by `protoc`, so there is no real
/// `FileDescriptorSet` to embed. Generated crates will replace this with bytes
/// preserved from `CodeGeneratorRequest.proto_file`.
pub const FILE_DESCRIPTOR_SET: &[u8] = &[];

#[cfg(test)]
mod tests {
    use super::*;
    use ::puroro::Message;

    use crate::{Task, TaskLazy};

    #[test]
    fn full_name_is_always_available() {
        assert_eq!(
            Task::<::allocator_api2::alloc::Global>::FULL_NAME,
            "example.Task"
        );
        assert_eq!(
            TaskLazy::<::allocator_api2::alloc::Global>::FULL_NAME,
            "example.Task"
        );
    }

    #[test]
    fn file_descriptor_set_is_empty_stub() {
        assert!(FILE_DESCRIPTOR_SET.is_empty());
    }
}
