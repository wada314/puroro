mod enums;
mod messages;
mod writer;

use itertools::Itertools;

use super::Context;
use crate::utils::{get_keyword_safe_ident, to_lower_snake_case, Indentor};
use crate::wrappers::{DescriptorVisitor, EnumDescriptor, MessageDescriptor};
use crate::Result;
use std::collections::HashMap;
use std::fmt::Write;

struct Visitor {
    output: Indentor<String>,
}
impl<'c> DescriptorVisitor<'c> for Visitor {
    fn handle_msg(&mut self, msg: &'c MessageDescriptor<'c>) -> Result<()> {
        messages::print_msg(&mut self.output, msg)
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
pub fn do_generate<'c>(context: &'c Context<'c>) -> Result<HashMap<String, String>> {
    let mut filenames_and_contents = HashMap::new();
    for file_desc in context.file_descriptors() {
        let file_name = file_desc.output_file_path_from_root().to_string();
        let output = "\
#![allow(unused_variables)]
#![allow(unused_imports)]
\n"
        .to_string();
        let mut visitor = Visitor {
            output: Indentor::new(output),
        };
        file_desc.visit_messages_and_enums_in_file(&mut visitor)?;
        filenames_and_contents.insert(file_name, visitor.output.into_inner());
    }

    for (package, subpackages_iter) in context.packages_with_subpackages() {
        let file_name = if package.is_empty() {
            "mod.rs".to_string()
        } else {
            Itertools::intersperse(
                package
                    .split('.')
                    .map(|p| get_keyword_safe_ident(&to_lower_snake_case(p))),
                "/".to_string(),
            )
            .collect::<String>()
                + ".rs"
        };
        let extra_content = subpackages_iter
            .map(|p| {
                format!(
                    "pub mod {name};\n",
                    name = get_keyword_safe_ident(&to_lower_snake_case(p))
                )
            })
            .collect::<String>();
        let content = filenames_and_contents.entry(file_name).or_default();
        *content = format!("{}\n{}", content, extra_content);
    }

    Ok(filenames_and_contents)
}
