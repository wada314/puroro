use ::puroro::{PuroroError, Result};
use ::puroro_deserializer::stream::{LengthDelimitedDeserializer, MessageHandler, Variant};

/*
// The version number of protocol compiler.
message Version {
  optional int32 major = 1;
  optional int32 minor = 2;
  optional int32 patch = 3;
  // A suffix for alpha, beta or rc release, e.g., "alpha-1", "rc2". It should
  // be empty for mainline stable releases.
  optional string suffix = 4;
}
*/
proto_readable_struct! {
    struct Version {
        major: i32 = 1,
        minor: i32 = 2,
        patch: i32 = 3,
        suffix: String = 4,
    }
}

struct Version_ {
    major: i32,
    minor: i32,
    patch: i32,
    suffix: String,
}
impl MessageHandler for Version_ {
    type Target = Version_;

    fn finish(self) -> Result<Self::Target> {
        Ok(self)
    }

    fn deserialized_variant(&mut self, field_number: usize, variant: Variant) -> Result<()> {
        match field_number {
            1 => {
                self.major = variant.to_i32()?;
            }
            2 => {
                self.minor = variant.to_i32()?;
            }
            3 => {
                self.patch = variant.to_i32()?;
            }
            _ => {
                return Err(PuroroError::UnexpectedFieldType);
            }
        }
        Ok(())
    }

    fn deserialized_32bits(&mut self, _field_number: usize, _value: [u8; 4]) -> Result<()> {
        return Err(PuroroError::UnexpectedFieldType);
    }

    fn deserialized_64bits(&mut self, _field_number: usize, _value: [u8; 8]) -> Result<()> {
        return Err(PuroroError::UnexpectedFieldType);
    }

    fn deserialize_length_delimited_field<'a, D: LengthDelimitedDeserializer<'a>>(
        &mut self,
        deserializer: D,
        field_number: usize,
    ) -> Result<()> {
        if field_number != 4 {
            return Err(PuroroError::UnexpectedFieldType);
        }
        deserializer.deserialize_as_string(|s| {
            self.suffix = s;
            Ok(())
        })?;
        Ok(())
    }
}

/*
// An encoded CodeGeneratorRequest is written to the plugin's stdin.
message CodeGeneratorRequest {
  // The .proto files that were explicitly listed on the command-line.  The
  // code generator should generate code only for these files.  Each file's
  // descriptor will be included in proto_file, below.
  repeated string file_to_generate = 1;

  // The generator parameter passed on the command-line.
  optional string parameter = 2;

  // FileDescriptorProtos for all files in files_to_generate and everything
  // they import.  The files will appear in topological order, so each file
  // appears before any file that imports it.
  //
  // protoc guarantees that all proto_files will be written after
  // the fields above, even though this is not technically guaranteed by the
  // protobuf wire format.  This theoretically could allow a plugin to stream
  // in the FileDescriptorProtos and handle them one by one rather than read
  // the entire set into memory at once.  However, as of this writing, this
  // is not similarly optimized on protoc's end -- it will store all fields in
  // memory at once before sending them to the plugin.
  //
  // Type names of fields and extensions in the FileDescriptorProto are always
  // fully qualified.
  repeated FileDescriptorProto proto_file = 15;

  // The version number of protocol compiler.
  optional Version compiler_version = 3;

}
 */
