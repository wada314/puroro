pub struct CodeGeneratorResponse {
    error: String,
    supported_features: u64,
    file: ::std::vec::Vec<super::super::super::google::protobuf::compiler::code_generator_response::File>,
}
mod code_generator_response {
    pub enum Feature {
        FeatureNone = 0,
        FeatureProto3Optional = 1,
    }
    impl std::convert::TryFrom<i32> for Feature {
        type Error = i32; 
        fn try_from(val: i32) -> std::result::Result<Self, i32> {
            match val {
                0 => Ok(Self::FeatureNone),
                1 => Ok(Self::FeatureProto3Optional),
                x => Err(x),
            }
        }
    }
    pub struct File {
        name: String,
        insertion_point: String,
        content: String,
        generated_code_info: ::std::option::Option<::std::boxed::Box<super::super::super::google::protobuf::GeneratedCodeInfo>>,
    }
}
pub struct CodeGeneratorRequest {
    file_to_generate: ::std::vec::Vec<String>,
    parameter: String,
    proto_file: ::std::vec::Vec<super::super::super::google::protobuf::FileDescriptorProto>,
    compiler_version: ::std::option::Option<::std::boxed::Box<super::super::super::google::protobuf::compiler::Version>>,
}
pub struct Version {
    major: i32,
    minor: i32,
    patch: i32,
    suffix: String,
}
