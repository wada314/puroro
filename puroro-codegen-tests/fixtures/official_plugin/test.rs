//! Compile + smoke tests for official `descriptor.proto` / `plugin.proto`.

use crate::official_plugin::google::protobuf::compiler::code_generator_response::File as ResponseFile;
use crate::official_plugin::google::protobuf::compiler::{
    CodeGeneratorRequest, CodeGeneratorResponse,
};
use crate::official_plugin::google::protobuf::field_descriptor_proto::{Label, Type};
use crate::official_plugin::google::protobuf::{
    DescriptorProto, FieldDescriptorProto, FileDescriptorProto,
};
use ::puroro::Message;

#[test]
fn constructs_core_types() {
    let _ = FileDescriptorProto::new();
    let _ = DescriptorProto::new();
    let _ = FieldDescriptorProto::new();
    let _ = CodeGeneratorRequest::new();
    let _ = CodeGeneratorResponse::new();
    assert_eq!(Type::DOUBLE, Type::try_from(1).unwrap());
    assert_eq!(Label::OPTIONAL, Label::try_from(1).unwrap());
    // proto2: default is the first defined enumerator (not wire 0).
    assert_eq!(Type::default(), Type::DOUBLE);
    assert_eq!(Label::default(), Label::OPTIONAL);
    assert!(Type::try_from(0).is_err());
}

#[test]
fn code_generator_response_round_trip() {
    let mut resp = CodeGeneratorResponse::new();
    resp.error_mut().push_str("boom");
    *resp.supported_features_mut() = 3;
    let mut file = ResponseFile::new();
    file.name_mut().push_str("out.rs");
    file.content_mut().push_str("fn main() {}");
    resp.file_mut().push(file);

    let decoded: CodeGeneratorResponse =
        CodeGeneratorResponse::decode(&resp.encode_to_vec()[..]).expect("decode");
    assert_eq!(decoded.error().get(), "boom");
    assert_eq!(decoded.supported_features().get(), 3);
    assert_eq!(decoded.file().len(), 1);
    assert_eq!(decoded.file()[0].name().get(), "out.rs");
    assert_eq!(decoded.file()[0].content().get(), "fn main() {}");
}

#[test]
fn file_descriptor_proto_name_round_trip() {
    let mut file = FileDescriptorProto::new();
    file.name_mut().push_str("demo.proto");
    file.package_mut().push_str("demo");
    let mut msg = DescriptorProto::new();
    msg.name_mut().push_str("Task");
    file.message_type_mut().push(msg);

    let decoded: FileDescriptorProto =
        FileDescriptorProto::decode(&file.encode_to_vec()[..]).expect("decode");
    assert_eq!(decoded.name().get(), "demo.proto");
    assert_eq!(decoded.package().get(), "demo");
    assert_eq!(decoded.message_type().len(), 1);
    assert_eq!(decoded.message_type()[0].name().get(), "Task");
}
