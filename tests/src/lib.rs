use protobuf_pb as protos;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        use protos::google::protobuf::DescriptorProto;
        assert_eq!(2 + 2, 4);
    }
}
