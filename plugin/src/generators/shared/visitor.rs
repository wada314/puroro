use crate::protos::*;
use crate::Result;

pub(crate) trait DescriptorVisitor<'p> {
    fn handle_msg(&mut self, msg: &'p DescriptorProto) -> Result<()>;
    fn handle_enum(&mut self, enume: &'p EnumDescriptorProto) -> Result<()>;
    fn enter_submodule(&mut self, name: &'p str) -> Result<()>;
    fn exit_submodule(&mut self, name: &'p str) -> Result<()>;
}

pub(crate) fn visit_in_file<'p, T: DescriptorVisitor<'p>>(
    file: &'p FileDescriptorProto,
    visitor: &mut T,
) -> Result<()> {
    enum Task<'q> {
        HandleMsg(&'q DescriptorProto),
        HandleEnum(&'q EnumDescriptorProto),
        EnterSubmodule(&'q str),
        ExitSubmodule(&'q str),
    }
    let mut tasks = file
        .message_type
        .iter()
        .map(|msg| Task::HandleMsg(msg))
        .chain(file.enum_type.iter().map(|enume| Task::HandleEnum(enume)))
        .collect::<Vec<_>>();

    while let Some(task) = tasks.pop() {
        match task {
            Task::HandleMsg(msg) => {
                visitor.handle_msg(msg)?;
                if !msg.nested_type.is_empty() || !msg.enum_type.is_empty() {
                    tasks.push(Task::ExitSubmodule(&msg.name));
                    tasks.extend(msg.nested_type.iter().map(|submsg| Task::HandleMsg(submsg)));
                    tasks.extend(msg.enum_type.iter().map(|enume| Task::HandleEnum(enume)));
                    tasks.push(Task::EnterSubmodule(&msg.name));
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
