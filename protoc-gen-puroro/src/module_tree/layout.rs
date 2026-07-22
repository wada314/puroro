//! Physical layout of a [`ModuleForest`](super::ModuleForest) into plugin files.
//!
//! Layout never splits `mod` blocks that appear inside [`ModuleNode::items`](super::ModuleNode::items);
//! only forest `children` are eligible for file splitting.

use super::ModuleForest;
use super::ModuleNode;
use crate::error::{Error, Result};
use crate::plugin_io::ResponseFile;
use ::proc_macro2::TokenStream;
use ::quote::quote;

/// How to materialise a [`ModuleForest`] as `CodeGeneratorResponse` files.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleLayout {
    /// Expand the whole forest into one `.rs` file (`mod child { … }` inline).
    SingleFile {
        /// Path relative to the plugin output directory (e.g. `lib.rs`, `empty.rs`).
        path: String,
    },
    // FileTree { … } — deferred until split policies are needed.
}

/// Render a forest according to `layout`.
pub fn render(forest: &ModuleForest, layout: &ModuleLayout) -> Result<Vec<ResponseFile>> {
    match layout {
        ModuleLayout::SingleFile { path } => Ok(vec![render_single_file(forest, path)?]),
    }
}

/// Expand `forest` into one source file with nested `pub mod` blocks.
pub fn render_single_file(forest: &ModuleForest, path: &str) -> Result<ResponseFile> {
    let tokens = render_node_body(forest.root());
    Ok(ResponseFile {
        name: path.to_owned(),
        content: tokens_to_source(tokens)?,
    })
}

fn render_node_body(node: &ModuleNode) -> TokenStream {
    let items = node.items().clone();
    let child_mods = node.children().iter().map(render_child_mod);
    quote! {
        #items
        #(#child_mods)*
    }
}

fn render_child_mod(child: &ModuleNode) -> TokenStream {
    let name = child
        .name()
        .expect("layout-eligible child modules must be named");
    let body = render_node_body(child);
    quote! {
        pub mod #name {
            #body
        }
    }
}

fn tokens_to_source(tokens: TokenStream) -> Result<String> {
    let file = ::syn::parse2::<::syn::File>(tokens)
        .map_err(|e| Error::Codegen(format!("generated token stream is not valid Rust: {e}")))?;
    Ok(::prettyplease::unparse(&file))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::module_tree::{ModuleForest, ModuleOrigin, proto_fqn, type_name_to_module_ident};
    use ::quote::quote;

    #[test]
    fn single_file_inlines_message_under_root() {
        let mut forest = ModuleForest::new();
        forest.root_mut().append_items(quote! {
            #![allow(clippy::absolute_paths)]
        });

        let parent = forest.ensure_package("");
        let mod_name = type_name_to_module_ident("Empty");
        let type_name = ::proc_macro2::Ident::new("Empty", ::proc_macro2::Span::call_site());
        parent.append_items(quote! {
            pub use #mod_name::#type_name;
        });

        let child = parent.get_or_insert_child(mod_name.clone());
        child.add_origin(ModuleOrigin::Message {
            proto_fqn: proto_fqn("", "Empty"),
        });
        child.append_items(quote! {
            pub struct Empty;
        });

        let file = render_single_file(&forest, "empty.rs").unwrap();
        assert_eq!(file.name, "empty.rs");
        assert!(file.content.contains("pub use empty::Empty"));
        assert!(file.content.contains("pub mod empty"));
        assert!(file.content.contains("pub struct Empty"));
        assert!(file.content.contains("allow"));
    }

    #[test]
    fn single_file_nests_package_modules() {
        let mut forest = ModuleForest::new();
        let parent = forest.ensure_package("example");
        parent.append_items(quote! {
            pub struct Status(pub i32);
        });
        let mod_name = type_name_to_module_ident("Task");
        let type_name = ::proc_macro2::Ident::new("Task", ::proc_macro2::Span::call_site());
        parent.append_items(quote! {
            pub use #mod_name::#type_name;
        });
        let task = parent.get_or_insert_child(mod_name);
        task.append_items(quote! {
            pub struct Task;
        });

        let file = render_single_file(&forest, "lib.rs").unwrap();
        assert!(file.content.contains("pub mod example"));
        assert!(file.content.contains("pub struct Status"));
        assert!(file.content.contains("pub mod task"));
        assert!(file.content.contains("pub struct Task"));
    }
}
