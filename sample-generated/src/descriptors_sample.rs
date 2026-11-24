//! Sample of the opt-in `descriptors` feature surface.
//!
//! Production plugin output puts `FILE_DESCRIPTOR_SET` in a crate-root
//! `descriptors` module and implements [`DescribedMessage`] next to each
//! message type. This hand-written crate mirrors that shape.

use ::puroro::descriptors::{DescribedMessage, FileDescriptorSetBytes};
use ::puroro_rt::decode::LazyProfile;

use crate::{Address, AddressLazy, Marker, Point, PointLazy, School, Student, Task, TaskLazy};

/// Crate-wide descriptor set.
///
/// Empty stub: the sample is not produced by `protoc`, so there is no real
/// `FileDescriptorSet` to embed. Generated crates will replace this with bytes
/// preserved from `CodeGeneratorRequest.proto_file`.
pub const FILE_DESCRIPTOR_SET: FileDescriptorSetBytes = FileDescriptorSetBytes::new(&[]);

macro_rules! described {
    ($ty:ident, $file:expr) => {
        impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone> DescribedMessage
            for $ty<A>
        {
            const FILE_NAME: &'static str = $file;
        }
    };
}

macro_rules! described_lazy {
    ($ty:ident, $file:expr) => {
        impl<A: ::allocator_api2::alloc::Allocator + ::core::clone::Clone, P: LazyProfile>
            DescribedMessage for $ty<A, P>
        where
            P::Buf: ::puroro_rt::decode::WireAppend,
        {
            const FILE_NAME: &'static str = $file;
        }
    };
}

// DESIGN.md reference schema lives in a fictional `example.proto`.
described!(Task, "example.proto");
described_lazy!(TaskLazy, "example.proto");
described!(Address, "example.proto");
described_lazy!(AddressLazy, "example.proto");
described!(Point, "example.proto");
described_lazy!(PointLazy, "example.proto");
described!(School, "example.proto");
described!(Student, "example.proto");
described!(Marker, "example.proto");

#[cfg(test)]
mod tests {
    use super::*;
    use ::puroro::Message;

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
    fn described_message_names_source_file() {
        assert_eq!(
            <Task<::allocator_api2::alloc::Global> as DescribedMessage>::FILE_NAME,
            "example.proto"
        );
        assert!(FILE_DESCRIPTOR_SET.as_bytes().is_empty());
    }
}
