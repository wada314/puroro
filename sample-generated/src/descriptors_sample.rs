//! Sample of the opt-in `descriptors` feature surface.
//!
//! Production plugin output puts `FILE_DESCRIPTOR_SET` in a crate-root
//! `descriptors` module as a Rust byte literal of the request's
//! `FileDescriptorSet`. This hand-written crate mirrors that shape.
//!
//! # Empty sketch
//!
//! The constant is `&[]` here because this crate is not produced by `protoc`.
//! Generated crates embed the preserved `CodeGeneratorRequest.proto_file`
//! bodies. Do not treat emptiness in *this* sample as the generated-crate
//! behaviour.

/// Crate-wide encoded `FileDescriptorSet`.
///
/// Always empty in this hand-written sample (`&[]`). Plugin output replaces
/// this with a `b"…"` literal of the request's descriptor set.
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
    fn file_descriptor_set_is_empty_in_hand_written_sample() {
        assert!(FILE_DESCRIPTOR_SET.is_empty());
    }
}
