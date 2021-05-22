use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

use crate::google::protobuf::compiler::CodeGeneratorRequest;
use crate::utils::iter_package_to_root;
use crate::wrappers::{
    DescriptorVisitor, EnumDescriptor, EnumOrMessageRef, FileDescriptor, MessageDescriptor,
};
use crate::{ErrorKind, Result};
use ::once_cell::unsync::OnceCell;

#[derive(Debug, Clone)]
pub enum ImplType {
    Default,
    SliceView {
        check_utf8: bool,
        check_wire_types: bool,
    },
}

#[derive(Debug, Clone)]
pub enum AllocatorType {
    Default,
    Bumpalo,
}

#[derive(Clone)]
pub struct Context<'proto> {
    proto: &'proto CodeGeneratorRequest,
    impl_type: ImplType,
    alloc_type: AllocatorType,

    lazy_file_descriptors: OnceCell<Vec<FileDescriptor<'proto>>>,
    lazy_fq_name_to_desc_map: OnceCell<HashMap<&'proto str, EnumOrMessageRef<'proto>>>,
    lazy_packages_with_subpackages_map: OnceCell<HashMap<&'proto str, HashSet<&'proto str>>>,
}

impl<'proto> Context<'proto> {
    pub fn new(
        cgreq: &'proto CodeGeneratorRequest,
        impl_type: ImplType,
        alloc_type: AllocatorType,
    ) -> Self {
        Self {
            proto: cgreq,
            impl_type,
            alloc_type,
            lazy_file_descriptors: Default::default(),
            lazy_fq_name_to_desc_map: Default::default(),
            lazy_packages_with_subpackages_map: Default::default(),
        }
    }
    pub fn with_impl_type(&self, impl_type: ImplType) -> Self {
        Self::new(self.proto, impl_type, self.alloc_type.clone())
    }
    pub fn with_alloc_type(&self, alloc_type: AllocatorType) -> Self {
        Self::new(self.proto, self.impl_type.clone(), alloc_type)
    }
    pub fn impl_type(&self) -> ImplType {
        self.impl_type.clone()
    }
    pub fn alloc_type(&self) -> AllocatorType {
        self.alloc_type.clone()
    }

    pub fn file_descriptors(&'proto self) -> impl Iterator<Item = &FileDescriptor<'proto>> + 'proto {
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

    pub fn fq_name_to_desc(&'proto self, fq_name: &str) -> Result<Option<EnumOrMessageRef<'proto>>> {
        let map = self
            .lazy_fq_name_to_desc_map
            .get_or_try_init(|| -> Result<_> {
                struct Visitor<'a, 'd>(&'a mut HashMap<&'d str, EnumOrMessageRef<'d>>);
                impl<'a, 'd> DescriptorVisitor<'d> for Visitor<'a, 'd> {
                    fn handle_msg(&mut self, msg: &'d MessageDescriptor<'d>) -> Result<()> {
                        if let Some(_) = self
                            .0
                            .insert(msg.fully_qualified_name()?, EnumOrMessageRef::Message(msg))
                        {
                            Err(ErrorKind::ConflictedName {
                                name: msg.fully_qualified_name()?.to_string(),
                            })?
                        }
                        Ok(())
                    }

                    fn handle_enum(&mut self, enume: &'d EnumDescriptor<'d>) -> Result<()> {
                        if let Some(_) = self
                            .0
                            .insert(enume.fully_qualified_name()?, EnumOrMessageRef::Enum(enume))
                        {
                            Err(ErrorKind::ConflictedName {
                                name: enume.fully_qualified_name()?.to_string(),
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

    pub fn packages_with_subpackages(
        &'proto self,
    ) -> impl Iterator<Item = (&'proto str, impl Iterator<Item = &'proto str>)> {
        let map = self.lazy_packages_with_subpackages_map.get_or_init(|| {
            let mut map: HashMap<&'proto str, HashSet<&'proto str>> = HashMap::new();
            for file in self.file_descriptors() {
                for cur_package in iter_package_to_root(file.package()) {
                    if cur_package.is_empty() {
                        // Do nothing?
                    } else if let Some((parent_package, subpackage)) = cur_package.rsplit_once('.')
                    {
                        map.entry(parent_package).or_default().insert(subpackage);
                    } else {
                        map.entry("").or_default().insert(cur_package);
                    }
                }
            }
            map
        });
        map.iter().map(|(k, v)| (*k, v.iter().cloned()))
    }
}

impl Debug for Context<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Context").finish()
    }
}
