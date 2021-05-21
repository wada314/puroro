#[allow(unused)]
use protobuf_pb as protos;
#[allow(unused)]
use sample_pb as samples;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        #[allow(unused)]
        use protos::google::protobuf::DescriptorProto;
        assert_eq!(2 + 2, 4);
    }
}
