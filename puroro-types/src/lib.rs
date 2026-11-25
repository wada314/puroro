//! Generated `google.protobuf` types for puroro (descriptors + WKTs).
//!
//! Enable Cargo features `descriptors` and/or `wkt` (both are on by default).
//! See the crate README for the feature table and dependency notes.
//!
//! **Not an official Google product.** Prefer
//! [`google-protobuf`](https://crates.io/crates/google-protobuf) for production.

#![allow(clippy::absolute_paths)]
#![allow(clippy::match_single_binding)]

include!(concat!(env!("OUT_DIR"), "/generated_lib.rs"));

#[cfg(test)]
mod tests {
    #[cfg(feature = "descriptors")]
    mod descriptors {
        use crate::google::protobuf::{DescriptorProto, FileDescriptorProto, FileDescriptorSet};
        use ::puroro::{BytesMut, Message};

        #[test]
        fn file_descriptor_set_round_trip() {
            let mut file = FileDescriptorProto::new();
            file.name_mut().set(b"demo.proto");
            file.package_mut().set(b"demo");
            let mut msg = DescriptorProto::new();
            msg.name_mut().set(b"Task");
            file.message_type_mut().push(msg);

            let mut set = FileDescriptorSet::new();
            set.file_mut().push(file);

            let decoded: FileDescriptorSet =
                FileDescriptorSet::decode(&set.encode_to_vec()[..]).expect("decode");
            assert_eq!(decoded.file().len(), 1);
            assert_eq!(decoded.file()[0].name().get(), b"demo.proto");
            assert_eq!(decoded.file()[0].package().get(), b"demo");
            assert_eq!(decoded.file()[0].message_type()[0].name().get(), b"Task");
            assert_eq!(
                FileDescriptorSet::<::allocator_api2::alloc::Global>::FULL_NAME,
                "google.protobuf.FileDescriptorSet"
            );
        }

        #[test]
        fn embedded_file_descriptor_set_is_non_empty() {
            assert!(
                !crate::descriptors::FILE_DESCRIPTOR_SET.is_empty(),
                "plugin should embed proto_file bodies when descriptors feature is on"
            );
        }
    }

    #[cfg(feature = "wkt")]
    mod wkt {
        use crate::google::protobuf::{Duration, Empty, Timestamp};
        use ::puroro::Message;

        #[test]
        fn timestamp_round_trip() {
            let mut ts = Timestamp::new();
            *ts.seconds_mut() = 1_700_000_000;
            *ts.nanos_mut() = 123;
            let decoded: Timestamp = Timestamp::decode(&ts.encode_to_vec()[..]).expect("decode");
            assert_eq!(decoded.seconds(), 1_700_000_000);
            assert_eq!(decoded.nanos(), 123);
            assert_eq!(
                Timestamp::<::allocator_api2::alloc::Global>::FULL_NAME,
                "google.protobuf.Timestamp"
            );
        }

        #[test]
        fn duration_and_empty_construct() {
            let mut d = Duration::new();
            *d.seconds_mut() = -1;
            *d.nanos_mut() = 500_000_000;
            let _ = Empty::new();
            let decoded: Duration = Duration::decode(&d.encode_to_vec()[..]).expect("decode");
            assert_eq!(decoded.seconds(), -1);
            assert_eq!(decoded.nanos(), 500_000_000);
        }
    }
}
