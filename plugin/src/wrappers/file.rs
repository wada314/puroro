use crate::google::protobuf::FileDescriptorProto;
use crate::Context;
use crate::Result;

use super::EnumDescriptor;
use super::MessageDescriptor;

pub struct FileDescriptor<'c> {
    proto: &'c FileDescriptorProto,
    context: &'c Context<'c>,
    messages: Vec<MessageDescriptor<'c>>,
    enums: Vec<EnumDescriptor<'c>>,
}
impl<'c> FileDescriptor<'c> {
    pub fn new(proto: &'c FileDescriptorProto, context: &'c Context<'c>) -> Self {
        Self {
            proto,
            context,
            messages: proto
                .message_type
                .iter()
                .map(|m| MessageDescriptor::new(m, context))
                .collect(),
            enums: proto
                .enum_type
                .iter()
                .map(|e| EnumDescriptor::new(e, context))
                .collect(),
        }
    }
    pub fn path_from_root(&self) -> &str {
        &self.proto.name
    }
    pub fn messages(&self) -> impl Iterator<Item = &MessageDescriptor<'c>> {
        self.messages.iter()
    }
    pub fn enums(&self) -> impl Iterator<Item = &EnumDescriptor<'c>> {
        self.enums.iter()
    }

    /// Visit all `MessageDescriptor` and `EnumDescriptor` containted in this
    /// file, including the nested messages and enums.
    pub fn visit_messages_and_enums_in_file<T: DescriptorVisitor>(
        &self,
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
