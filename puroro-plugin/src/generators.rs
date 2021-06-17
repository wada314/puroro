mod enums;
mod message_frags;
mod message_impls;
mod message_tags;
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
use self::message_tags::MessageTagCodeGenerator;
use self::message_traits::MessageTraitCodeGenerator;

static FILE_HEADER: &str = "\
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
\n";

pub fn do_generate<'c>(context: &'c Context<'c>) -> Result<HashMap<String, String>> {
    let mut filenames_and_contents = HashMap::new();
    for file_desc in context.file_descriptors() {
        {
            // Simple struct impl
            let file_name = package_to_file_path("simple", file_desc.package());
            let mut visitor = MessageGeneratingVisitor {
                output: Indentor::new(FILE_HEADER.to_string()),
                context: context.clone(),
            };
            file_desc.visit_messages_and_enums_in_file(&mut visitor)?;
            filenames_and_contents.insert(file_name, visitor.output.into_inner());
        }

        {
            // Bumpalo struct impl
            let file_name = package_to_file_path("bumpalo", file_desc.package());
            let mut visitor = MessageGeneratingVisitor {
                output: Indentor::new(FILE_HEADER.to_string()),
                context: context.with_alloc_type(AllocatorType::Bumpalo),
            };
            file_desc.visit_messages_and_enums_in_file(&mut visitor)?;
            filenames_and_contents.insert(file_name, visitor.output.into_inner());
        }

        {
            // SliceView struct impl
            let file_name = package_to_file_path("slice_view", file_desc.package());
            let mut visitor = MessageGeneratingVisitor {
                output: Indentor::new(FILE_HEADER.to_string()),
                context: context.with_impl_type(ImplType::SliceView),
            };
            file_desc.visit_messages_and_enums_in_file(&mut visitor)?;
            filenames_and_contents.insert(file_name, visitor.output.into_inner());
        }

        {
            // enums
            let file_name = package_to_file_path("enums", file_desc.package());
            let mut visitor = EnumGeneratingVisitor {
                output: Indentor::new(FILE_HEADER.to_string()),
            };
            file_desc.visit_messages_and_enums_in_file(&mut visitor)?;
            filenames_and_contents.insert(file_name, visitor.output.into_inner());
        }

        {
            // traits
            let file_name = package_to_file_path("traits", file_desc.package());
            let mut visitor = TraitGeneratingVisitor {
                output: Indentor::new(FILE_HEADER.to_string()),
            };
            file_desc.visit_messages_and_enums_in_file(&mut visitor)?;
            filenames_and_contents.insert(file_name, visitor.output.into_inner());
        }

        {
            // tags
            let file_name = package_to_file_path("tags", file_desc.package());
            let mut visitor = TagGeneratingVisitor {
                output: Indentor::new(FILE_HEADER.to_string()),
            };
            file_desc.visit_messages_and_enums_in_file(&mut visitor)?;
            filenames_and_contents.insert(file_name, visitor.output.into_inner());
        }
    }

    // Generate the submodule declarations. i.e. `pub mod xxx;`
    let prefixes = ["simple", "bumpalo", "slice_view", "enums", "traits", "tags"];
    for (package, subpackages_iter) in context.packages_with_subpackages() {
        for prefix in std::array::IntoIter::new(prefixes) {
            let file_name = package_to_file_path(prefix, package);
            let submodule_decls = subpackages_iter
                .clone()
                .map(|p| {
                    format!(
                        "pub mod {name};\n",
                        name = get_keyword_safe_ident(&to_lower_snake_case(p))
                    )
                })
                .collect::<String>();
            let content = filenames_and_contents
                .entry(file_name)
                .or_insert(FILE_HEADER.to_string());
            content.push_str(&format!("\n{}", submodule_decls));
        }
    }

    // Generate the root `mod.rs` file.
    let root_file_name = package_to_file_path("", "");
    let prefix_mods = prefixes
        .iter()
        .map(|p| {
            format!(
                "{maybe_cfg}pub mod {name};\n",
                maybe_cfg = if *p == "bumpalo" {
                    "#[cfg(feature = \"puroro-bumpalo\")]\n"
                } else {
                    ""
                },
                name = p
            )
        })
        .collect::<String>();
    filenames_and_contents.insert(root_file_name, prefix_mods);

    Ok(filenames_and_contents)
}

fn package_to_file_path(prefix: &str, package: &str) -> String {
    if prefix.is_empty() && package.is_empty() {
        "mod.rs".to_string()
    } else {
        let package_items = prefix
                .split('.')
                .chain(package.split('.'))
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

fn enter_submodule(output: &mut Indentor<String>, name: &str) -> Result<()> {
    let mod_name = get_keyword_safe_ident(&to_lower_snake_case(name));
    output.write_fmt(format_args!("pub mod {name} {{\n", name = mod_name))?;
    Ok(())
}
fn exit_submodule(output: &mut Indentor<String>, name: &str) -> Result<()> {
    let mod_name = get_keyword_safe_ident(&to_lower_snake_case(name));
    output.write_fmt(format_args!("}} // mod {name}\n", name = mod_name))?;
    Ok(())
}

struct MessageGeneratingVisitor<'c> {
    output: Indentor<String>,
    context: Context<'c>,
}
impl<'c> DescriptorVisitor<'c> for MessageGeneratingVisitor<'c> {
    fn handle_msg(&mut self, msg: &'c MessageDescriptor<'c>) -> Result<()> {
        let impl_gen = MessageImplCodeGenerator::new(&self.context, msg);
        impl_gen.print_msg(&mut self.output)?;
        Ok(())
    }
    fn handle_enum(&mut self, _: &'c EnumDescriptor<'c>) -> Result<()> {
        Ok(())
    }
    fn enter_submodule(&mut self, name: &str) -> Result<()> {
        enter_submodule(&mut self.output, name)
    }
    fn exit_submodule(&mut self, name: &str) -> Result<()> {
        exit_submodule(&mut self.output, name)
    }
}

struct TraitGeneratingVisitor {
    output: Indentor<String>,
}
impl<'c> DescriptorVisitor<'c> for TraitGeneratingVisitor {
    fn handle_msg(&mut self, msg: &'c MessageDescriptor<'c>) -> Result<()> {
        let trait_gen = MessageTraitCodeGenerator::new(msg);
        trait_gen.print_msg_traits(&mut self.output)?;
        Ok(())
    }
    fn handle_enum(&mut self, _: &'c EnumDescriptor<'c>) -> Result<()> {
        Ok(())
    }
    fn enter_submodule(&mut self, name: &str) -> Result<()> {
        enter_submodule(&mut self.output, name)
    }
    fn exit_submodule(&mut self, name: &str) -> Result<()> {
        exit_submodule(&mut self.output, name)
    }
}

struct EnumGeneratingVisitor {
    output: Indentor<String>,
}
impl<'c> DescriptorVisitor<'c> for EnumGeneratingVisitor {
    fn handle_msg(&mut self, _: &'c MessageDescriptor<'c>) -> Result<()> {
        Ok(())
    }
    fn handle_enum(&mut self, enume: &'c EnumDescriptor<'c>) -> Result<()> {
        enums::print_enum(&mut self.output, enume)
    }
    fn enter_submodule(&mut self, name: &str) -> Result<()> {
        enter_submodule(&mut self.output, name)
    }
    fn exit_submodule(&mut self, name: &str) -> Result<()> {
        exit_submodule(&mut self.output, name)
    }
}

struct TagGeneratingVisitor {
    output: Indentor<String>,
}
impl<'c> DescriptorVisitor<'c> for TagGeneratingVisitor {
    fn handle_msg(&mut self, msg: &'c MessageDescriptor<'c>) -> Result<()> {
        let tags_gen = MessageTagCodeGenerator::new(&msg);
        tags_gen.print_msg_tag(&mut self.output)?;
        Ok(())
    }
    fn handle_enum(&mut self, _: &'c EnumDescriptor<'c>) -> Result<()> {
        Ok(())
    }
    fn enter_submodule(&mut self, name: &str) -> Result<()> {
        enter_submodule(&mut self.output, name)
    }
    fn exit_submodule(&mut self, name: &str) -> Result<()> {
        exit_submodule(&mut self.output, name)
    }
}
