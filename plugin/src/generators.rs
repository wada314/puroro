pub mod shared;
//pub mod simple;

use crate::utils::Indentor;
use crate::wrappers;
use crate::Result;
use shared::writers::{indent, indent_n, iter, TupleOfIntoFragments};

use super::Context;
struct Visitor {
    output: Indentor<String>,
}
impl<'c> wrappers::DescriptorVisitor<'c> for Visitor {
    fn handle_msg(&mut self, msg: &'c wrappers::MessageDescriptor<'c>) -> crate::Result<()> {
        Ok(())
    }

    fn handle_enum(&mut self, enume: &'c wrappers::EnumDescriptor<'c>) -> crate::Result<()> {
        (
            format!("pub enum {name} {{\n", name = enume.name()),
            indent((iter(enume.values().map(|value| {
                Ok(format!(
                    "{name} = {value},\n",
                    name = value.native_name(),
                    value = value.number()
                ))
            })),)),
            "}}\n",
        )
            .write_into(&mut self.output)
    }

    fn enter_submodule(&mut self, name: &str) -> crate::Result<()> {
        Ok(())
    }

    fn exit_submodule(&mut self, name: &str) -> crate::Result<()> {
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
