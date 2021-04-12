use crate::generators::shared::utils::{
    FullyQualifiedTypeName, MaybeFullyQualifiedTypeName, PackagePath,
};

use crate::google::protobuf::compiler::CodeGeneratorRequest;
use crate::wrappers::FileDescriptor;
use crate::{ErrorKind, Result};
use ::once_cell::unsync::OnceCell;

pub struct Context<'c> {
    proto: CodeGeneratorRequest,
    lazy_file_descriptors: OnceCell<Vec<FileDescriptor<'c>>>,
}

impl<'c> Context<'c> {
    pub fn new(cgreq: CodeGeneratorRequest) -> Result<Self> {
        Ok(Self {
            proto: cgreq,
            lazy_file_descriptors: Default::default(),
        })
    }
    pub fn file_descriptors(&'c self) -> impl Iterator<Item = &FileDescriptor<'c>> + 'c {
        self.lazy_file_descriptors
            .get_or_init(|| {
                self.proto
                    .proto_file
                    .iter()
                    .map(|f| FileDescriptor::new(f, self))
                    .collect()
            })
            .iter()
    }
}
