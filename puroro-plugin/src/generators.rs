mod enums;
mod message_frags;
mod message_impls;
mod message_traits;
mod writer;

use itertools::Itertools;

use crate::context::{AllocatorType, Context, ImplType};
use crate::utils::{get_keyword_safe_ident, to_lower_snake_case, Indentor};
use crate::wrappers::{DescriptorVisitor, EnumDescriptor, MessageDescriptor};
use crate::Result;
use std::collections::HashMap;
use std::fmt::Write;

use self::message_impls::MessageImplCodeGenerator;
use self::message_traits::MessageTraitCodeGenerator;

struct Visitor<'c> {
    output: Indentor<String>,
    context: Context<'c>,
    bumpalo_context: Context<'c>,
    slice_view_context: Context<'c>,
}
impl<'c> DescriptorVisitor<'c> for Visitor<'c> {
    fn handle_msg(&mut self, msg: &'c MessageDescriptor<'c>) -> Result<()> {
        let normal_impl_gen = MessageImplCodeGenerator::new(&self.context, msg);
        let bumpalo_impl_gen = MessageImplCodeGenerator::new(&self.bumpalo_context, msg);
        let slice_view_impl_gen = MessageImplCodeGenerator::new(&self.slice_view_context, msg);
        let trait_gen = MessageTraitCodeGenerator::new(&self.context, msg);

        trait_gen.print_msg_traits(&mut self.output)?;
        //normal_impl_gen.print_msg(&mut self.output)?;
        //bumpalo_impl_gen.print_msg(&mut self.output)?;
        slice_view_impl_gen.print_msg(&mut self.output)?;
        Ok(())
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
#![allow(dead_code)]
\n"
        .to_string();
        let mut visitor = Visitor {
            output: Indentor::new(output),
            context: context.with_alloc_type(AllocatorType::Default),
            bumpalo_context: context.with_alloc_type(AllocatorType::Bumpalo),
            slice_view_context: context.with_impl_type(ImplType::SliceView),
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
