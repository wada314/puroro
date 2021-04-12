use std::collections::HashMap;

use crate::generators::shared::utils::{
    FullyQualifiedTypeName, MaybeFullyQualifiedTypeName, PackagePath,
};

use crate::google::protobuf::compiler::CodeGeneratorRequest;
use crate::wrappers::{DescriptorVisitor, EnumDescriptor, FileDescriptor, MessageDescriptor};
use crate::{ErrorKind, Result};
use ::once_cell::unsync::OnceCell;

#[derive(Clone)]
enum EnumOrMessageRef<'c> {
    Enum(&'c EnumDescriptor<'c>),
    Message(&'c MessageDescriptor<'c>),
}

pub struct Context<'c> {
    proto: CodeGeneratorRequest,
    lazy_file_descriptors: OnceCell<Vec<FileDescriptor<'c>>>,
    lazy_fq_name_to_desc_map: OnceCell<HashMap<&'c str, EnumOrMessageRef<'c>>>,
}

impl<'c> Context<'c> {
    pub fn new(cgreq: CodeGeneratorRequest) -> Result<Self> {
        Ok(Self {
            proto: cgreq,
            lazy_file_descriptors: Default::default(),
            lazy_fq_name_to_desc_map: Default::default(),
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
    fn fq_name_to_desc(&'c self, fq_name: &str) -> Option<EnumOrMessageRef<'c>> {
        let map = self.lazy_fq_name_to_desc_map.get_or_init(|| {
            struct Visitor();
            impl DescriptorVisitor for Visitor {
                fn handle_msg(&mut self, msg: &MessageDescriptor) -> Result<()> {
                    Ok(())
                }

                fn handle_enum(&mut self, enume: &EnumDescriptor) -> Result<()> {
                    Ok(())
                }
            }
            for file in self.file_descriptors() {}
        });
    }
}
