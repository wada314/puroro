#![cfg_attr(feature = "puroro-nightly", feature(backtrace))]
#![cfg_attr(feature = "puroro-nightly", feature(generic_associated_types))]
#![cfg_attr(feature = "puroro-nightly", feature(min_type_alias_impl_trait))]
#![allow(incomplete_features)]

#[allow(unused)]
use protobuf_pb as protos;
#[allow(unused)]
use sample_pb as samples;

trait SliceSource {
    type Iter<'a>: Iterator<Item = &'a [u8]>;
    fn slices(&self) -> Self::Iter<'_>;
}

trait MsgTrait {}
struct Msg<S: SliceSource> {
    source: S,
}
impl<S: SliceSource> Msg<S> {
    fn get_child(&self) -> Msg<&'_ Self> {
        Msg { source: self }
    }
}

impl<'a> SliceSource for &'a [u8] {
    type Iter<'b> = impl Iterator<Item = &'b [u8]>;
    fn slices(&self) -> Self::Iter<'_> {
        std::iter::once(*self)
    }
}

impl<'a, S: SliceSource> SliceSource for &'a Msg<S> {
    type Iter<'b> = impl Iterator<Item = &'b [u8]>;
    fn slices(&self) -> Self::Iter<'_> {
        self.source.slices()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        #[allow(unused)]
        use protos::google::protobuf::DescriptorProto;
        let bytes: &[u8] = &[0u8, 1, 2];

        let m = Msg { source: bytes };

        assert_eq!(2 + 2, 4);
    }
}
