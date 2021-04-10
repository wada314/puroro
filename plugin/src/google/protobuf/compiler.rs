pub struct CodeGeneratorResponse {
    error: String,
    supported_features: u64,
    file: ::std::vec::Vec<super::super::super::google::protobuf::compiler::code_generator_response::File>,
}
impl ::std::default::Default for CodeGeneratorResponse {
    fn default() -> Self {
        Self {
            error: std::default::Default::default(),
            supported_features: std::default::Default::default(),
            file: std::default::Default::default(),
        }
    }
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
    impl ::std::default::Default for File {
        fn default() -> Self {
            Self {
                name: std::default::Default::default(),
                insertion_point: std::default::Default::default(),
                content: std::default::Default::default(),
                generated_code_info: std::default::Default::default(),
            }
        }
    }
}
pub struct CodeGeneratorRequest {
    file_to_generate: ::std::vec::Vec<String>,
    parameter: String,
    proto_file: ::std::vec::Vec<super::super::super::google::protobuf::FileDescriptorProto>,
    compiler_version: ::std::option::Option<::std::boxed::Box<super::super::super::google::protobuf::compiler::Version>>,
}
impl ::std::default::Default for CodeGeneratorRequest {
    fn default() -> Self {
        Self {
            file_to_generate: std::default::Default::default(),
            parameter: std::default::Default::default(),
            proto_file: std::default::Default::default(),
            compiler_version: std::default::Default::default(),
        }
    }
}
pub struct Version {
    major: i32,
    minor: i32,
    patch: i32,
    suffix: String,
}
impl ::std::default::Default for Version {
    fn default() -> Self {
        Self {
            major: std::default::Default::default(),
            minor: std::default::Default::default(),
            patch: std::default::Default::default(),
            suffix: std::default::Default::default(),
        }
    }
}
