//! Physical layout of a [`ModuleForest`](super::ModuleForest) into plugin files.
//!
//! Layout never splits `mod` blocks that appear inside [`ModuleNode::items`](super::ModuleNode::items);
//! only forest `children` are eligible for file splitting.
//!
//! Every emitted module also gets a private `_root` alias so generated code can
//! name peers as `self::_root::…` regardless of nesting depth or whether the
//! forest is the crate root or an embedded submodule.

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
    let tokens = render_node_body(forest.root(), /* is_forest_root */ true);
    Ok(ResponseFile {
        name: path.to_owned(),
        content: tokens_to_source(tokens)?,
    })
}

fn render_node_body(node: &ModuleNode, is_forest_root: bool) -> TokenStream {
    let root_alias = root_alias_mod(is_forest_root);
    let items = node.items().clone();
    let child_mods = node.children().iter().map(render_child_mod);
    if is_forest_root {
        // Inner attributes (`#![…]`) in `items` must stay at the top of the file.
        quote! {
            #items
            #root_alias
            #(#child_mods)*
        }
    } else {
        quote! {
            #root_alias
            #items
            #(#child_mods)*
        }
    }
}

fn render_child_mod(child: &ModuleNode) -> TokenStream {
    let name = child
        .name()
        .expect("layout-eligible child modules must be named");
    let body = render_node_body(child, /* is_forest_root */ false);
    quote! {
        pub mod #name {
            #body
        }
    }
}

/// Private alias of the generated forest root, visible as `self::_root` everywhere.
///
/// - Forest root: re-exports the root module's public items.
/// - Nested module: chains to the parent's `_root` via `super::super` (the extra
///   `super` accounts for this `_root` submodule itself).
fn root_alias_mod(is_forest_root: bool) -> TokenStream {
    if is_forest_root {
        quote! {
            #[allow(unused_imports)]
            mod _root {
                pub(super) use super::*;
            }
        }
    } else {
        quote! {
            #[allow(unused_imports)]
            mod _root {
                pub(super) use super::super::_root::*;
            }
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
    use crate::descriptor::ProtoFqn;
    use crate::module_tree::{ModuleForest, ModuleOrigin, type_name_to_module_ident};
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
            proto_fqn: ProtoFqn::parse(".Empty"),
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
        assert!(file.content.contains("mod _root"));
        assert!(file.content.contains("use super::*;"));
        assert!(file.content.contains("use super::super::_root::*;"));
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
        // Nested modules chain `_root` upward.
        assert!(file.content.contains("use super::super::_root::*;"));
    }

    #[test]
    fn nested_module_can_name_peer_via_self_root() {
        let mut forest = ModuleForest::new();
        let parent = forest.ensure_package("example");
        let address = parent.get_or_insert_child(type_name_to_module_ident("Address"));
        address.append_items(quote! {
            pub struct Address;
        });
        let task = parent.get_or_insert_child(type_name_to_module_ident("Task"));
        // Simulate a cross-type reference the way real codegen will emit it.
        task.append_items(quote! {
            pub type Assignee = self::_root::example::address::Address;
        });

        let file = render_single_file(&forest, "lib.rs").unwrap();
        assert!(
            file.content
                .contains("self::_root::example::address::Address")
        );
        // Compiles as a snippet when parsed: already validated by tokens_to_source.
        let _ = file;
    }
}
