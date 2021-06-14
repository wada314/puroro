use std::fmt::Debug;

use crate::error::ErrorKind;
use crate::google::protobuf::FileDescriptorProto;
use crate::utils::{get_keyword_safe_ident, to_lower_snake_case};
use crate::Context;
use crate::Result;

use super::{EnumDescriptor, FileOrMessageRef, MessageDescriptor};
use ::once_cell::unsync::OnceCell;
use itertools::Itertools;

#[derive(Clone)]
pub struct FileDescriptor<'c> {
    proto: &'c FileDescriptorProto,
    context: &'c Context<'c>,

    lazy_messages: OnceCell<Vec<MessageDescriptor<'c>>>,
    lazy_enums: OnceCell<Vec<EnumDescriptor<'c>>>,
}
impl<'c> FileDescriptor<'c> {
    pub fn new(proto: &'c FileDescriptorProto, context: &'c Context<'c>) -> Self {
        Self {
            proto,
            context,

            lazy_messages: Default::default(),
            lazy_enums: Default::default(),
        }
    }
    pub fn output_file_path_from_root(&'c self, prefix: &str) -> String {
        if prefix.is_empty() && self.package().is_empty() {
            "mod.rs".to_string()
        } else {
            let package_items = prefix
                .split('.')
                .chain(self.package().split('.'))
                .filter(|p| !p.is_empty()) // because `"".split('.').next() == Some("")`
                ;
            Itertools::intersperse(
                package_items.map(|p| get_keyword_safe_ident(&to_lower_snake_case(p))),
                "/".to_string(),
            )
            .collect::<String>()
                + ".rs"
        }
    }
    pub fn syntax(&self) -> Result<ProtoSyntax> {
        match self.proto.syntax.as_deref() {
            Some("proto2") | Some("") | None => Ok(ProtoSyntax::Proto2),
            Some("proto3") => Ok(ProtoSyntax::Proto3),
            Some(other) => Err(ErrorKind::UnknownProtoSyntax {
                name: other.to_string(),
            })?,
        }
    }

    pub fn messages(&'c self) -> impl Iterator<Item = &MessageDescriptor<'c>> {
        self.lazy_messages
            .get_or_init(|| {
                self.proto
                    .message_type
                    .iter()
                    .map(|m| MessageDescriptor::new(m, self.context, FileOrMessageRef::File(self)))
                    .collect()
            })
            .iter()
    }
    pub fn enums(&'c self) -> impl Iterator<Item = &EnumDescriptor<'c>> {
        self.lazy_enums
            .get_or_init(|| {
                self.proto
                    .enum_type
                    .iter()
                    .map(|e| EnumDescriptor::new(e, self.context, FileOrMessageRef::File(self)))
                    .collect()
            })
            .iter()
    }
    pub fn package(&'c self) -> &'c str {
        self.proto.package.as_deref().unwrap_or("")
    }

    /// Visit all `MessageDescriptor` and `EnumDescriptor` in this
    /// file, including the nested messages and enums.
    pub fn visit_messages_and_enums_in_file<T: DescriptorVisitor<'c>>(
        &'c self,
        visitor: &mut T,
    ) -> Result<()> {
        /// Lifetime `'a` is for this descriptor's lifetime, and `'d` is for
        /// the context's lifetime.
        enum Task<'a, 'd> {
            HandleMsg(&'a MessageDescriptor<'d>),
            HandleEnum(&'a EnumDescriptor<'d>),
            EnterSubmodule(&'a str),
            ExitSubmodule(&'a str),
        }
        let mut tasks = self
            .messages()
            .map(|msg| Task::HandleMsg(msg))
            .chain(self.enums().map(|enume| Task::HandleEnum(enume)))
            .collect::<Vec<_>>();

        while let Some(task) = tasks.pop() {
            match task {
                Task::HandleMsg(msg) => {
                    visitor.handle_msg(msg)?;
                    if msg.nested_messages().next().is_some() || msg.enums().next().is_some() {
                        tasks.push(Task::ExitSubmodule(msg.name()?));
                        tasks.extend(msg.nested_messages().map(|submsg| Task::HandleMsg(submsg)));
                        tasks.extend(msg.enums().map(|enume| Task::HandleEnum(enume)));
                        tasks.push(Task::EnterSubmodule(msg.name()?));
                    }
                }
                Task::HandleEnum(enume) => {
                    visitor.handle_enum(enume)?;
                }
                Task::EnterSubmodule(name) => {
                    visitor.enter_submodule(name)?;
                }
                Task::ExitSubmodule(name) => {
                    visitor.exit_submodule(name)?;
                }
            }
        }

        Ok(())
    }
}

impl Debug for FileDescriptor<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FileDescriptor").finish()
    }
}

#[derive(Debug, Clone)]
pub enum ProtoSyntax {
    Proto2,
    Proto3,
}

pub trait DescriptorVisitor<'c> {
    fn handle_msg(&mut self, #[allow(unused)] msg: &'c MessageDescriptor<'c>) -> Result<()> {
        Ok(())
    }
    fn handle_enum(&mut self, #[allow(unused)] enume: &'c EnumDescriptor<'c>) -> Result<()> {
        Ok(())
    }
    fn enter_submodule(&mut self, #[allow(unused)] name: &str) -> Result<()> {
        Ok(())
    }
    fn exit_submodule(&mut self, #[allow(unused)] name: &str) -> Result<()> {
        Ok(())
    }
}
