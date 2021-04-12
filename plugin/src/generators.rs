pub mod shared;
//pub mod simple;

use std::fmt::Write;

use crate::utils::{get_keyword_safe_ident, to_lower_snake_case, Indentor};
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
            format!(
                "\
#[derive(Debug, Clone)]
pub enum {name} {{\n",
                name = enume.native_bare_typename()
            ),
            indent((iter(enume.values().map(|value| {
                Ok(format!(
                    "{name} = {value},\n",
                    name = value.native_name(),
                    value = value.number()
                ))
            })),)),
            "\
}}\n",
            format!(
                "\
impl std::convert::TryFrom<i32> for {name} {{
    type Error = i32;
    fn try_from(val: i32) -> ::std::result::Result<Self, i32> {{
        match val {{\n",
                name = enume.native_bare_typename()
            ),
            indent_n(
                3,
                (
                    iter(enume.values().map(|value| {
                        Ok(format!(
                            "{number} => Ok(Self::{name}),\n",
                            number = value.number(),
                            name = value.native_name(),
                        ))
                    })),
                    "x => Err(x),\n",
                ),
            ),
            "        \
        }}
    }}
}}\n",
        )
            .write_into(&mut self.output)
    }

    fn enter_submodule(&mut self, name: &str) -> crate::Result<()> {
        let mod_name = get_keyword_safe_ident(&to_lower_snake_case(name));
        self.output
            .write_fmt(format_args!("pub mod {name} {{\n", name = mod_name))?;
        Ok(())
    }

    fn exit_submodule(&mut self, _name: &str) -> crate::Result<()> {
        self.output.write_str("}\n")?;
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
