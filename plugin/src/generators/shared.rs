pub(crate) mod context;
pub(crate) mod utils;
pub(crate) mod visitor;
pub(crate) mod writers;

use crate::protos::*;
use crate::Result;
use std::fmt::Write;
use utils::{Indentor, PackagePath};

use context::Context;
use visitor::{visit_in_file, DescriptorVisitor};

pub(crate) trait FileGeneratorHandler {
    fn handle_msg<'p, W: std::fmt::Write>(
        &mut self,
        out: &mut Indentor<W>,
        context: &Context,
        msg: &'p DescriptorProto,
    ) -> Result<()>;
    fn handle_enum<'p, W: std::fmt::Write>(
        &mut self,
        out: &mut Indentor<W>,
        context: &Context,
        enume: &'p EnumDescriptorProto,
    ) -> Result<()>;
    fn generate_filename<'p>(
        &mut self,
        context: &Context,
        file: &'p FileDescriptorProto,
    ) -> Result<String>;
}

pub(crate) fn generate_file_with_handler<'p, H>(
    context: &mut Context<'p>,
    input_file: &'p FileDescriptorProto,
    handler: &mut H,
) -> Result<(String, String)>
where
    H: FileGeneratorHandler,
{
    context.set_package(&PackagePath::new(&input_file.package));
    let filename = handler.generate_filename(context, input_file)?;

    struct InnerVisitor<'a, 'q, H: FileGeneratorHandler> {
        output: Indentor<String>,
        context: &'a mut Context<'q>,
        handler: &'a mut H,
    }
    impl<'a, 'q, H> DescriptorVisitor<'q> for InnerVisitor<'a, 'q, H>
    where
        H: FileGeneratorHandler,
    {
        fn handle_msg(&mut self, msg: &'q DescriptorProto) -> Result<()> {
            self.handler.handle_msg(&mut self.output, self.context, msg)
        }
        fn handle_enum(&mut self, enume: &'q EnumDescriptorProto) -> Result<()> {
            self.handler
                .handle_enum(&mut self.output, self.context, enume)
        }
        fn enter_submodule(&mut self, name: &'q str) -> Result<()> {
            let module_name = utils::to_module_name(name);
            writeln!(&mut self.output, "pub mod {name} {{", name = module_name)?;
            self.context.enter_submessage_namespace(name);
            self.output.indent();
            Ok(())
        }
        fn exit_submodule(&mut self, name: &'q str) -> Result<()> {
            self.context.leave_submessage_namespace(name);
            self.output.unindent();
            writeln!(self.output, "}}")?;
            Ok(())
        }
    }

    let mut inner_visitor = InnerVisitor {
        output: Indentor::new(String::new()),
        context,
        handler,
    };
    visit_in_file(&input_file, &mut inner_visitor)?;

    Ok((filename, inner_visitor.output.into_inner()))
}
