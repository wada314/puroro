//! Logical Rust module forest produced by the core generator.
//!
//! This is **not** a physical file layout. [`ModuleNode::items`] may contain
//! nested `mod` blocks (e.g. tiny private helpers); those stay inline forever.
//! Only [`ModuleNode::children`] are layout-eligible units that a later pass may
//! turn into `mod name;` + separate files, or inline as `mod name { … }`.
//!
//! Origins are non-exclusive: a package segment and a message may contribute to
//! the same Rust path. Nodes are merged by module name; item-level clashes are
//! left to `rustc`.

pub mod layout;

use crate::descriptor::ProtoFqn;
use ::proc_macro2::{Ident, TokenStream};
use ::std::mem;

/// One codegen invocation's logical module tree.
///
/// Always has a root: empty protobuf packages hang messages/enums there, and
/// crate/module inner attributes (`#![…]`) live on the root `items`.
#[derive(Debug, Clone)]
pub struct ModuleForest {
    root: ModuleNode,
}

/// A layout-eligible module in the generated Rust namespace tree.
#[derive(Debug, Clone)]
pub struct ModuleNode {
    /// `None` for the forest root; `Some` for every nested module.
    name: Option<Ident>,
    /// Why this module exists. Multiple entries are allowed after merges.
    origins: Vec<ModuleOrigin>,
    /// Items that belong directly in this module (may include private `mod`).
    items: TokenStream,
    /// Child modules that Layout may split into files.
    children: Vec<ModuleNode>,
}

/// Provenance of a generated module (diagnostic / layout hint — not exclusive).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleOrigin {
    /// Synthetic root of the generated module forest.
    GeneratedRoot,
    /// One segment of a protobuf `package` (accumulated path, e.g. `example.v1`).
    PackageSegment { package: String },
    /// Message implementation module (`struct` + `FIELD_*` / visitors / impls).
    Message {
        /// Absolute protobuf FQN (e.g. `.example.Task`).
        proto_fqn: ProtoFqn,
    },
    /// Oneof submodule under a message module.
    Oneof {
        message_fqn: ProtoFqn,
        oneof_name: String,
    },
}

impl ModuleForest {
    /// Empty forest whose root is marked [`ModuleOrigin::GeneratedRoot`].
    pub fn new() -> Self {
        Self {
            root: ModuleNode {
                name: None,
                origins: vec![ModuleOrigin::GeneratedRoot],
                items: TokenStream::new(),
                children: Vec::new(),
            },
        }
    }

    pub fn root(&self) -> &ModuleNode {
        &self.root
    }

    pub fn root_mut(&mut self) -> &mut ModuleNode {
        &mut self.root
    }

    /// Walk / create package segment modules under the root.
    ///
    /// `package` is a dot-separated protobuf package; empty string returns the root.
    pub fn ensure_package(&mut self, package: &str) -> &mut ModuleNode {
        let mut node = &mut self.root;
        if package.is_empty() {
            return node;
        }

        let mut so_far = String::new();
        for segment in package.split('.') {
            if segment.is_empty() {
                continue;
            }
            if !so_far.is_empty() {
                so_far.push('.');
            }
            so_far.push_str(segment);
            let ident = Ident::new(segment, ::proc_macro2::Span::call_site());
            node = node.get_or_insert_child(ident);
            node.add_origin(ModuleOrigin::PackageSegment {
                package: so_far.clone(),
            });
        }
        node
    }
}

impl Default for ModuleForest {
    fn default() -> Self {
        Self::new()
    }
}

impl ModuleNode {
    /// Create a named child module with no origins / items yet.
    pub fn new_child(name: Ident) -> Self {
        Self {
            name: Some(name),
            origins: Vec::new(),
            items: TokenStream::new(),
            children: Vec::new(),
        }
    }

    pub fn name(&self) -> Option<&Ident> {
        self.name.as_ref()
    }

    pub fn origins(&self) -> &[ModuleOrigin] {
        &self.origins
    }

    pub fn items(&self) -> &TokenStream {
        &self.items
    }

    pub fn children(&self) -> &[ModuleNode] {
        &self.children
    }

    /// Record an origin unless an identical one is already present.
    pub fn add_origin(&mut self, origin: ModuleOrigin) {
        if !self.origins.contains(&origin) {
            self.origins.push(origin);
        }
    }

    /// Append tokens to this module's item list.
    pub fn append_items(&mut self, tokens: TokenStream) {
        self.items.extend(tokens);
    }

    /// Take the item token stream, leaving this node empty of items.
    pub fn take_items(&mut self) -> TokenStream {
        mem::take(&mut self.items)
    }

    /// Get or create a layout-eligible child with the given Rust module name.
    ///
    /// If a child with the same ident already exists, that node is reused (origins
    /// and items from later contributors are merged into it by the caller).
    pub fn get_or_insert_child(&mut self, name: Ident) -> &mut ModuleNode {
        let key = name.to_string();
        if let Some(index) = self
            .children
            .iter()
            .position(|child| child.name.as_ref().is_some_and(|n| n == &*key))
        {
            return &mut self.children[index];
        }
        self.children.push(Self::new_child(name));
        self.children
            .last_mut()
            .expect("child just pushed must exist")
    }

    /// Find a direct child by Rust module name.
    pub fn get_child(&self, name: &str) -> Option<&ModuleNode> {
        self.children
            .iter()
            .find(|child| child.name.as_ref().is_some_and(|n| n == name))
    }

    pub fn get_child_mut(&mut self, name: &str) -> Option<&mut ModuleNode> {
        self.children
            .iter_mut()
            .find(|child| child.name.as_ref().is_some_and(|n| n == name))
    }
}

/// Convert a protobuf message/enum type name to a Rust module ident (snake_case).
///
/// This is a minimal converter for ASCII protobuf identifiers (`Task` → `task`,
/// `FooBar` → `foo_bar`). It is not a full unicode ident mapper.
pub fn type_name_to_module_ident(type_name: &str) -> Ident {
    Ident::new(&to_snake_case(type_name), ::proc_macro2::Span::call_site())
}

fn to_snake_case(input: &str) -> String {
    let mut out = String::new();
    for (i, c) in input.chars().enumerate() {
        if c.is_uppercase() {
            if i != 0 {
                out.push('_');
            }
            for lower in c.to_lowercase() {
                out.push(lower);
            }
        } else {
            out.push(c);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::descriptor::ProtoFqn;
    use ::quote::quote;

    #[test]
    fn ensure_package_builds_segments() {
        let mut forest = ModuleForest::new();
        forest.ensure_package("example.v1");

        let example = forest.root().get_child("example").expect("example");
        assert!(example.origins().contains(&ModuleOrigin::PackageSegment {
            package: "example".into(),
        }));
        let v1 = example.get_child("v1").expect("v1");
        assert!(v1.origins().contains(&ModuleOrigin::PackageSegment {
            package: "example.v1".into(),
        }));
    }

    #[test]
    fn merge_package_and_message_same_path() {
        let mut forest = ModuleForest::new();
        // package example.task
        forest.ensure_package("example.task");
        // message Task in package example → also wants example::task
        let example = forest
            .root_mut()
            .get_or_insert_child(Ident::new("example", ::proc_macro2::Span::call_site()));
        example.add_origin(ModuleOrigin::PackageSegment {
            package: "example".into(),
        });
        let task =
            example.get_or_insert_child(Ident::new("task", ::proc_macro2::Span::call_site()));
        task.add_origin(ModuleOrigin::Message {
            proto_fqn: ProtoFqn::parse(".example.Task"),
        });
        task.append_items(quote! { pub struct Task; });

        let task = forest
            .root()
            .get_child("example")
            .unwrap()
            .get_child("task")
            .unwrap();
        assert!(task.origins().contains(&ModuleOrigin::PackageSegment {
            package: "example.task".into(),
        }));
        assert!(task.origins().contains(&ModuleOrigin::Message {
            proto_fqn: ProtoFqn::parse(".example.Task"),
        }));
        assert!(!task.items().is_empty());
    }

    #[test]
    fn empty_package_is_root() {
        let mut forest = ModuleForest::new();
        let parent = forest.ensure_package("");
        parent.append_items(quote! { #![allow(dead_code)] });
        assert!(forest.root().name().is_none());
        assert!(
            forest
                .root()
                .origins()
                .contains(&ModuleOrigin::GeneratedRoot)
        );
    }

    #[test]
    fn snake_case_helpers() {
        assert_eq!(to_snake_case("Task"), "task");
        assert_eq!(to_snake_case("FooBar"), "foo_bar");
    }
}
