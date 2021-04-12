use crate::google::protobuf::FileDescriptorProto;
use crate::Context;
use crate::Result;

use super::{message, r#enum};
use super::{EnumDescriptor, MessageDescriptor};
use ::once_cell::unsync::OnceCell;

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
    pub fn path_from_root(&self) -> &str {
        &self.proto.name
    }
    pub fn messages(&'c self) -> impl Iterator<Item = &MessageDescriptor<'c>> {
        self.lazy_messages
            .get_or_init(|| {
                self.proto
                    .message_type
                    .iter()
                    .map(|m| MessageDescriptor::new(m, self.context, message::Parent::File(self)))
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
                    .map(|e| EnumDescriptor::new(e, self.context, r#enum::Parent::File(self)))
                    .collect()
            })
            .iter()
    }

    /// Visit all `MessageDescriptor` and `EnumDescriptor` containted in this
    /// file, including the nested messages and enums.
    pub fn visit_messages_and_enums_in_file<T: DescriptorVisitor>(
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
                        tasks.push(Task::ExitSubmodule(msg.name()));
                        tasks.extend(msg.nested_messages().map(|submsg| Task::HandleMsg(submsg)));
                        tasks.extend(msg.enums().map(|enume| Task::HandleEnum(enume)));
                        tasks.push(Task::EnterSubmodule(msg.name()));
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
pub trait DescriptorVisitor {
    fn handle_msg(&mut self, msg: &MessageDescriptor) -> Result<()>;
    fn handle_enum(&mut self, enume: &EnumDescriptor) -> Result<()>;
    fn enter_submodule(&mut self, name: &str) -> Result<()>;
    fn exit_submodule(&mut self, name: &str) -> Result<()>;
}
