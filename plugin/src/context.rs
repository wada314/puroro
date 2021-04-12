use std::collections::HashMap;

use crate::google::protobuf::compiler::CodeGeneratorRequest;
use crate::wrappers::{
    DescriptorVisitor, EnumDescriptor, EnumOrMessageRef, FileDescriptor, MessageDescriptor,
};
use crate::{ErrorKind, Result};
use ::once_cell::unsync::OnceCell;

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
    pub fn fq_name_to_desc(&'c self, fq_name: &str) -> Result<Option<EnumOrMessageRef<'c>>> {
        let map = self
            .lazy_fq_name_to_desc_map
            .get_or_try_init(|| -> Result<_> {
                struct Visitor<'a, 'd>(&'a mut HashMap<&'d str, EnumOrMessageRef<'d>>);
                impl<'a, 'd> DescriptorVisitor<'d> for Visitor<'a, 'd> {
                    fn handle_msg(&mut self, msg: &'d MessageDescriptor<'d>) -> Result<()> {
                        if let Some(_) =
                            self.0.insert(msg.fq_name(), EnumOrMessageRef::Message(msg))
                        {
                            Err(ErrorKind::ConflictedName {
                                name: msg.fq_name().to_string(),
                            })?
                        }
                        Ok(())
                    }

                    fn handle_enum(&mut self, enume: &'d EnumDescriptor<'d>) -> Result<()> {
                        if let Some(_) = self
                            .0
                            .insert(enume.fq_name(), EnumOrMessageRef::Enum(enume))
                        {
                            Err(ErrorKind::ConflictedName {
                                name: enume.fq_name().to_string(),
                            })?
                        }
                        Ok(())
                    }
                }
                let mut map = HashMap::new();
                let mut visitor = Visitor(&mut map);
                for file in self.file_descriptors() {
                    file.visit_messages_and_enums_in_file(&mut visitor)?;
                }
                Ok(map)
            })?;
        Ok(map.get(fq_name).cloned())
    }
}
