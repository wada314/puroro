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
use ::syn::parse_quote;
use ::syn::{File, Item};

/// How to materialise a [`ModuleForest`] as `CodeGeneratorResponse` files.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleLayout {
    /// Expand the whole forest into one `.rs` file (`mod child { … }` inline).
    SingleFile {
        /// Path relative to the plugin output directory (e.g. `lib.rs`).
        path: String,
    },
    // FileTree { … } — deferred until split policies are needed.
}

/// Render a forest according to `layout`.
pub fn render(forest: &ModuleForest, layout: &ModuleLayout) -> Result<Vec<ResponseFile>> {
    reject_reserved_puroro(forest.root())?;
    match layout {
        ModuleLayout::SingleFile { path } => Ok(vec![render_single_file(forest, path)]),
    }
}

/// Expand `forest` into one source file with nested `pub mod` blocks.
pub fn render_single_file(forest: &ModuleForest, path: &str) -> ResponseFile {
    let file = render_file(forest.root());
    ResponseFile {
        name: path.to_owned(),
        content: ::prettyplease::unparse(&file),
    }
}

fn render_file(root: &ModuleNode) -> File {
    File {
        shebang: None,
        attrs: root.inner_attrs().to_vec(),
        items: render_node_items(root, /* is_forest_root */ true),
    }
}

fn render_node_items(node: &ModuleNode, is_forest_root: bool) -> Vec<Item> {
    let mut items = Vec::new();
    if is_forest_root {
        items.push(puroro_reexport());
        items.extend(node.items().iter().cloned());
        items.push(root_alias_item(true));
        items.extend(node.children().iter().map(render_child_mod));
    } else {
        items.push(nested_root_alias());
        items.extend(node.items().iter().cloned());
        items.extend(node.children().iter().map(render_child_mod));
    }
    items
}

fn render_child_mod(child: &ModuleNode) -> Item {
    let name = child
        .name()
        .expect("layout-eligible child modules must be named")
        .clone();
    let inner_attrs = child.inner_attrs();
    let items = render_node_items(child, /* is_forest_root */ false);
    parse_quote! {
        pub mod #name {
            #(#inner_attrs)*
            #(#items)*
        }
    }
}

/// Re-export the runtime crate at the generated crate root (`generated::puroro`).
fn puroro_reexport() -> Item {
    parse_quote! {
        pub use ::puroro;
    }
}

/// `puroro` at the forest root is the runtime re-export, not a proto module.
fn reject_reserved_puroro(root: &ModuleNode) -> Result<()> {
    if root.get_child("puroro").is_some() || root.items().iter().any(item_binds_puroro) {
        return Err(Error::Codegen(
            "`puroro` is reserved at the generated crate root; the runtime crate is re-exported as that module"
                .to_owned(),
        ));
    }
    Ok(())
}

fn item_binds_puroro(item: &Item) -> bool {
    match item {
        Item::Struct(item) => item.ident == "puroro",
        Item::Enum(item) => item.ident == "puroro",
        Item::Type(item) => item.ident == "puroro",
        Item::Union(item) => item.ident == "puroro",
        Item::Trait(item) => item.ident == "puroro",
        Item::TraitAlias(item) => item.ident == "puroro",
        Item::Fn(item) => item.sig.ident == "puroro",
        Item::Const(item) => item.ident == "puroro",
        Item::Static(item) => item.ident == "puroro",
        Item::Mod(item) => item.ident == "puroro",
        _ => false,
    }
}

/// `_root` alias for a module nested inside the forest (`mod foo { mod _root { … } }`).
///
/// Also used by quote-time nested modules that are not forest children (oneof
/// bodies, `defaults`) so `self::_root::…` paths work at any depth.
pub(crate) fn nested_root_alias() -> Item {
    root_alias_item(false)
}

/// Private alias of the generated forest root, visible as `self::_root` everywhere.
///
/// - Forest root: re-exports the root module's public items.
/// - Nested module: chains to the parent's `_root` via `super::super` (the extra
///   `super` accounts for this `_root` submodule itself).
fn root_alias_item(is_forest_root: bool) -> Item {
    if is_forest_root {
        parse_quote! {
            #[allow(unused_imports)]
            mod _root {
                pub(super) use super::*;
            }
        }
    } else {
        parse_quote! {
            #[allow(unused_imports)]
            mod _root {
                pub(super) use super::super::_root::*;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::module_tree::{ModuleForest, type_name_to_module_ident};
    use ::syn::parse_quote;

    #[test]
    fn single_file_inlines_message_under_root() {
        let mut forest = ModuleForest::new();
        let file: File = parse_quote! {
            #![allow(clippy::absolute_paths)]
        };
        forest.root_mut().append_inner_attrs(file.attrs);

        let parent = forest.ensure_package("");
        parent.append_items([parse_quote! {
            pub struct Empty;
        }]);

        let file = render_single_file(&forest, "empty.rs");
        assert_eq!(file.name, "empty.rs");
        assert!(file.content.contains("pub struct Empty"));
        assert!(file.content.contains("allow"));
        assert!(file.content.contains("pub use ::puroro"));
        assert!(file.content.contains("mod _root"));
        assert!(file.content.contains("use super::*;"));
    }

    #[test]
    fn root_module_named_puroro_is_rejected() {
        let mut forest = ModuleForest::new();
        forest.ensure_package("puroro");
        let err = render(
            &forest,
            &ModuleLayout::SingleFile {
                path: "lib.rs".into(),
            },
        )
        .unwrap_err();
        assert!(err.to_string().contains("reserved"));
    }

    #[test]
    fn nested_module_named_puroro_is_allowed() {
        let mut forest = ModuleForest::new();
        forest.ensure_package("shop.puroro");
        render(
            &forest,
            &ModuleLayout::SingleFile {
                path: "lib.rs".into(),
            },
        )
        .unwrap();
    }

    #[test]
    fn single_file_nests_package_modules() {
        let mut forest = ModuleForest::new();
        let parent = forest.ensure_package("example");
        parent.append_items([parse_quote! {
            pub struct Status(pub i32);
        }]);
        let mod_name = type_name_to_module_ident("Task");
        parent.append_items([parse_quote! {
            pub struct Task;
        }]);
        let task = parent.get_or_insert_child(mod_name);
        task.append_items([parse_quote! {
            pub const FIELD_TITLE: u32 = 1;
        }]);

        let file = render_single_file(&forest, "lib.rs");
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
        parent.append_items([parse_quote! {
            pub struct Address;
        }]);
        let task = parent.get_or_insert_child(type_name_to_module_ident("Task"));
        // Simulate a cross-type reference the way real codegen will emit it.
        task.append_items([parse_quote! {
            pub type Assignee = self::_root::example::Address;
        }]);

        let file = render_single_file(&forest, "lib.rs");
        assert!(file.content.contains("self::_root::example::Address"));
        let _ = file;
    }
}
