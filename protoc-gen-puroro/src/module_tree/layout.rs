//! Physical layout of a [`ModuleForest`] into plugin files.
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
use ::proc_macro2::Span;
use ::syn::parse_quote;
use ::syn::{File, Item, LitByteStr};

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
///
/// `file_descriptor_set` is the encoded `google.protobuf.FileDescriptorSet`
/// embedded under `descriptors::FILE_DESCRIPTOR_SET` when the feature is on.
pub fn render(
    forest: &ModuleForest,
    layout: &ModuleLayout,
    file_descriptor_set: &[u8],
) -> Result<Vec<ResponseFile>> {
    reject_reserved_root_names(forest.root())?;
    match layout {
        ModuleLayout::SingleFile { path } => {
            Ok(vec![render_single_file(forest, path, file_descriptor_set)])
        }
    }
}

/// Expand `forest` into one source file with nested `pub mod` blocks.
pub fn render_single_file(
    forest: &ModuleForest,
    path: &str,
    file_descriptor_set: &[u8],
) -> ResponseFile {
    let file = render_file(forest.root(), file_descriptor_set);
    ResponseFile {
        name: path.to_owned(),
        content: ::prettyplease::unparse(&file),
    }
}

fn render_file(root: &ModuleNode, file_descriptor_set: &[u8]) -> File {
    File {
        shebang: None,
        attrs: root.inner_attrs().to_vec(),
        items: render_node_items(root, /* is_forest_root */ true, file_descriptor_set),
    }
}

fn render_node_items(
    node: &ModuleNode,
    is_forest_root: bool,
    file_descriptor_set: &[u8],
) -> Vec<Item> {
    let mut items = Vec::new();
    if is_forest_root {
        items.push(puroro_reexport());
        items.push(descriptors_module(file_descriptor_set));
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
    // Nested packages never own the crate-wide descriptor set.
    let items = render_node_items(child, /* is_forest_root */ false, &[]);
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

/// Opt-in `FileDescriptorSet` embedding (`descriptors` feature).
///
/// `file_descriptor_set` is the encoded set built from preserved
/// `CodeGeneratorRequest.proto_file` LEN bodies (see DESIGN.md §3.1). An empty
/// slice yields `b""` — only expected for hand-built requests without wire
/// payloads, not for a normal `protoc` run.
fn descriptors_module(file_descriptor_set: &[u8]) -> Item {
    let lit = LitByteStr::new(file_descriptor_set, Span::call_site());
    parse_quote! {
        /// Encoded `google.protobuf.FileDescriptorSet` for this crate (opt-in).
        ///
        /// Enable this crate's `descriptors` feature to compile this module in.
        /// The bytes are every `CodeGeneratorRequest.proto_file` entry, wrapped
        /// as `FileDescriptorSet.file`, in protoc's dependency order.
        #[cfg(feature = "descriptors")]
        pub mod descriptors {
            /// Crate-wide encoded `FileDescriptorSet`.
            pub const FILE_DESCRIPTOR_SET: &'static [u8] = #lit;
        }
    }
}

/// Root names reserved for layout-owned modules (`puroro`, `descriptors`).
const RESERVED_ROOT_NAMES: &[&str] = &["puroro", "descriptors"];

/// `puroro` is the runtime re-export; `descriptors` holds the opt-in FDS bytes.
fn reject_reserved_root_names(root: &ModuleNode) -> Result<()> {
    for name in RESERVED_ROOT_NAMES {
        if root.get_child(name).is_some()
            || root.items().iter().any(|item| item_binds_name(item, name))
        {
            return Err(Error::Codegen(format!(
                "`{name}` is reserved at the generated crate root"
            )));
        }
    }
    Ok(())
}

fn item_binds_name(item: &Item, name: &str) -> bool {
    match item {
        Item::Struct(item) => item.ident == name,
        Item::Enum(item) => item.ident == name,
        Item::Type(item) => item.ident == name,
        Item::Union(item) => item.ident == name,
        Item::Trait(item) => item.ident == name,
        Item::TraitAlias(item) => item.ident == name,
        Item::Fn(item) => item.sig.ident == name,
        Item::Const(item) => item.ident == name,
        Item::Static(item) => item.ident == name,
        Item::Mod(item) => item.ident == name,
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

        let file = render_single_file(&forest, "empty.rs", &[]);
        assert_eq!(file.name, "empty.rs");
        assert!(file.content.contains("pub struct Empty"));
        assert!(file.content.contains("allow"));
        assert!(file.content.contains("pub use ::puroro"));
        assert!(file.content.contains("mod descriptors"));
        assert!(file.content.contains("FILE_DESCRIPTOR_SET"));
        assert!(file.content.contains("mod _root"));
        assert!(file.content.contains("use super::*;"));
    }

    #[test]
    fn descriptors_module_embeds_byte_literal() {
        let forest = ModuleForest::new();
        let file = render_single_file(&forest, "lib.rs", &[0x0a, 0x01, b'x']);
        assert!(file.content.contains("FILE_DESCRIPTOR_SET"));
        assert!(file.content.contains("b\""));
        assert!(
            !file
                .content
                .contains("FILE_DESCRIPTOR_SET: &'static [u8] = b\"\";")
        );
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
            &[],
        )
        .unwrap_err();
        assert!(err.to_string().contains("reserved"));
    }

    #[test]
    fn root_module_named_descriptors_is_rejected() {
        let mut forest = ModuleForest::new();
        forest.ensure_package("descriptors");
        let err = render(
            &forest,
            &ModuleLayout::SingleFile {
                path: "lib.rs".into(),
            },
            &[],
        )
        .unwrap_err();
        assert!(err.to_string().contains("reserved"));
        assert!(err.to_string().contains("descriptors"));
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
            &[],
        )
        .unwrap();
    }

    #[test]
    fn nested_module_named_descriptors_is_allowed() {
        let mut forest = ModuleForest::new();
        forest.ensure_package("shop.descriptors");
        render(
            &forest,
            &ModuleLayout::SingleFile {
                path: "lib.rs".into(),
            },
            &[],
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

        let file = render_single_file(&forest, "lib.rs", &[]);
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

        let file = render_single_file(&forest, "lib.rs", &[]);
        assert!(file.content.contains("self::_root::example::Address"));
        let _ = file;
    }
}
