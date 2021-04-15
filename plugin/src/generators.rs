mod enums;
mod msgs;
mod writer;

use super::Context;
use crate::utils::{get_keyword_safe_ident, to_lower_snake_case, Indentor};
use crate::wrappers::{DescriptorVisitor, EnumDescriptor, MessageDescriptor};
use crate::Result;
use std::fmt::Write;

struct Visitor {
    output: Indentor<String>,
}
impl<'c> DescriptorVisitor<'c> for Visitor {
    fn handle_msg(&mut self, msg: &'c MessageDescriptor<'c>) -> Result<()> {
        msgs::print_msg(&mut self.output, msg)
    }

    fn handle_enum(&mut self, enume: &'c EnumDescriptor<'c>) -> Result<()> {
        enums::print_enum(&mut self.output, enume)
    }

    fn enter_submodule(&mut self, name: &str) -> Result<()> {
        let mod_name = get_keyword_safe_ident(&to_lower_snake_case(name));
        self.output
            .write_fmt(format_args!("pub mod {name} {{\n", name = mod_name))?;
        Ok(())
    }

    fn exit_submodule(&mut self, name: &str) -> Result<()> {
        let mod_name = get_keyword_safe_ident(&to_lower_snake_case(name));
        self.output
            .write_fmt(format_args!("}} // mod {name}\n", name = mod_name))?;
        Ok(())
    }
}
pub fn do_generate<'c>(context: &'c Context<'c>) -> Result<Vec<(String, String)>> {
    let mut filenames_and_contents = Vec::new();
    for file_desc in context.file_descriptors() {
        let file_name = file_desc.output_file_path_from_root().to_string();
        let mut visitor = Visitor {
            output: Indentor::new(String::new()),
        };
        file_desc.visit_messages_and_enums_in_file(&mut visitor)?;
        filenames_and_contents.push((file_name.clone(), visitor.output.into_inner()));
    }
    Ok(filenames_and_contents)
}
