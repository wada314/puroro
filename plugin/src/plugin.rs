use ::puroro::{PuroroError, Result};
use ::puroro_deserializer::stream::{LengthDelimitedDeserializer, MessageHandler, Variant};

// From plugin.proto

proto_struct! {
    struct Version {
        major: i32 = 1,
        minor: i32 = 2,
        patch: i32 = 3,
        suffix: String = 4,
    }
    struct CodeGeneratorRequest {
        file_to_generate: Vec<String> = 1,
        parameter: String = 2,
        // proto_file: Vec<FileDescriptorProto> = 15,
        compiler_version: Option<Version> = 3,
    }
    struct CodeGeneratorResponse {
        error: String = 1,
        supported_features: u64 = 2,
        file: Vec<CodeGeneratorResponse_File> = 15,
    }
    struct CodeGeneratorResponse_File {
        name: String = 1,
        insertion_point: String = 2,
        content: String = 15,
        // generated_code_info: GeneratedCodeInfo = 16,
    }
}
proto_enum! {
    enum Feature {
        FEATURE_NONE = 0,
        FEATURE_PROTO3_OPTIONAL = 1,
    }
}
/*
  enum Feature {
    FEATURE_NONE = 0;
    FEATURE_PROTO3_OPTIONAL = 1;
  }
} */
