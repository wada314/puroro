pub struct Version {
    major: i32,
    minor: i32,
    patch: i32,
    suffix: String,
} // pub struct Version {
pub struct CodeGeneratorRequest {
    file_to_generate: ::std::vec::Vec<String>,
    parameter: String,
    proto_file: ::std::vec::Vec<super::super::super::google::protobuf::FileDescriptorProto>,
    compiler_version: ::std::option::Option<::std::boxed::Box<super::super::super::google::protobuf::compiler::Version>>,
} // pub struct CodeGeneratorRequest {
mod code_generator_response {
    pub enum Feature {
        FeatureNone = 0,
        FeatureProto3Optional = 1,
    } // pub enum Feature {
    pub struct File {
        name: String,
        insertion_point: String,
        content: String,
        generated_code_info: ::std::option::Option<::std::boxed::Box<super::super::super::super::google::protobuf::GeneratedCodeInfo>>,
    } // pub struct File {
} // mod code_generator_response {
pub struct CodeGeneratorResponse {
    error: String,
    supported_features: u64,
    file: ::std::vec::Vec<super::super::super::google::protobuf::compiler::code_generator_response::File>,
} // pub struct CodeGeneratorResponse {
