#![feature(allocator_api)]
#![feature(generic_associated_types)]
#![feature(min_type_alias_impl_trait)]

//#[path = r"D:\Documents\projects\proto-rust\target\debug\build\tests-71fe3c39c5dbebf6\out\mod.rs"]
//mod protos;

include!(r"D:\Documents\projects\proto-rust\target\debug\build\tests-71fe3c39c5dbebf6\out\mod.rs");

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        use google::protobuf::DescriptorProto;
        assert_eq!(2 + 2, 4);
    }
}
