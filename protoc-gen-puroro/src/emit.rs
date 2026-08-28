//! Prepare `syn::Item` trees from a resolved schema (`Prepared*` + quote).
//!
//! Current scope: file-level and nested enums/messages with singular / repeated
//! scalar / string / bytes / bool / enum / message fields, real oneofs, and
//! maps with legal keys and Copy scalar / bool values. Orchestration (resolve,
//! target selection, forest install, layout) lives in [`crate::generate`].

use crate::descriptor::ProtoFqn;
use crate::error::{Error, Result};
use crate::field_kind::plan_fields;
use crate::module_tree::{ModuleForest, ModuleNode, ModuleOrigin, type_name_to_module_ident};
use crate::resolved::{Enum, File, Message};
use ::proc_macro2::Ident;
use ::quote::quote;
use ::syn::{Attribute, Item};

mod defaults;
mod enumeration;
mod ident;
mod message;
mod oneof;
mod parse;
mod type_path;

/// Crate-level inner attributes (`//!` / `/*!` header and clippy allows).
///
/// Tooling only needs the `@generated` substring near the top of the file.
/// Source `.proto` paths go in the same doc string so they stay one attribute;
/// prettyplease prints a multiline doc as `/*! … */`.
pub(crate) fn generated_file_attrs(targets: &[&File<'_>]) -> Result<Vec<Attribute>> {
    let files = targets
        .iter()
        .map(|file| format!("- `{}`", file.name()))
        .collect::<Vec<_>>()
        .join("\n");
    let doc = format!("\n@generated — do not edit\n{files}\n");
    let root_tokens = quote! {
        #![doc = #doc]
        #![allow(clippy::absolute_paths)]
        // Empty messages emit a catch-all-only `match` until field arms exist.
        #![allow(clippy::match_single_binding)]
    };
    Ok(parse::parse_file(root_tokens)?.attrs)
}

/// One message, prepared to install into a parent module.
///
/// The struct (and its impls) go on the parent. A snake_case companion holds
/// `FIELD_*` / `BIT_*`, nested types, and oneofs. Nested messages install into
/// that companion the same way. Empty companions are omitted.
struct PreparedMessage {
    /// Items appended to the parent — e.g. `[pub struct Foo, impl Foo { … }]`.
    type_items: Vec<Item>,
    /// Companion module ident — e.g. `foo` for `message Foo`. Unused when omitted.
    module_name: Ident,
    /// Protobuf FQN of this message — e.g. `.example.Foo`. Used at install to
    /// mark the companion with [`ModuleOrigin::Message`].
    proto_fqn: ProtoFqn,
    /// Items in the companion — e.g. `[pub const FIELD_TITLE, …]`.
    /// Empty together with [`Self::nested_enums`] and [`Self::nested`] means no
    /// companion.
    companion_items: Vec<Item>,
    /// Nested enums installed into this companion — e.g. `[PreparedEnum` for `Kind]`.
    nested_enums: Vec<PreparedEnum>,
    /// Nested messages installed into this companion — e.g. `[PreparedMessage` for `Bar]`.
    nested: Vec<PreparedMessage>,
}

/// One enum, prepared to install into a parent module.
struct PreparedEnum {
    /// Items appended to the parent — e.g. `[pub struct Status, impl Status { … }]`.
    items: Vec<Item>,
}

/// One `.proto` file, prepared to install into a package module.
pub(crate) struct PreparedFile {
    /// Protobuf package — e.g. `example.v1`. Empty string is the forest root.
    package: String,
    /// File-level enums — e.g. `[PreparedEnum` for `Status]`.
    enums: Vec<PreparedEnum>,
    /// File-level messages — e.g. `[PreparedMessage` for `Foo]`.
    messages: Vec<PreparedMessage>,
}

fn prepare_message(message: &Message<'_>) -> Result<PreparedMessage> {
    if !ident::is_simple_ident(message.name()) {
        return Err(Error::Codegen(format!(
            "cannot use message name `{}` as a Rust identifier",
            message.name()
        )));
    }
    let module_name = type_name_to_module_ident(message.name());
    let type_name = ident::escape_ident(message.name());
    let field_plan = plan_fields(message)?;
    let rendered = message::render_items(&field_plan, &type_name, message.name(), &module_name)?;

    let nested_enums = message
        .nested_enums()
        .map(prepare_enum)
        .collect::<Result<Vec<_>>>()?;
    let nested = message
        .nested_messages()
        .filter(|m| !m.is_map_entry())
        .map(prepare_message)
        .collect::<Result<Vec<_>>>()?;

    Ok(PreparedMessage {
        type_items: rendered.type_items,
        module_name,
        proto_fqn: message.fqn().clone(),
        companion_items: rendered.companion_items,
        nested_enums,
        nested,
    })
}

fn prepare_enum(enumeration: &Enum<'_>) -> Result<PreparedEnum> {
    Ok(PreparedEnum {
        items: enumeration::render_enum(enumeration)?,
    })
}

pub(crate) fn prepare_file(file: &File<'_>) -> Result<PreparedFile> {
    Ok(PreparedFile {
        package: file.package().to_owned(),
        enums: file.enums().map(prepare_enum).collect::<Result<Vec<_>>>()?,
        messages: file
            .messages()
            .map(prepare_message)
            .collect::<Result<Vec<_>>>()?,
    })
}

impl ModuleForest {
    pub(crate) fn install_file(&mut self, prepared: PreparedFile) {
        let package = self.ensure_package(&prepared.package);
        for e in prepared.enums {
            package.install_enum(e);
        }
        for message in prepared.messages {
            package.install_message(message);
        }
    }
}

impl ModuleNode {
    fn install_message(&mut self, prepared: PreparedMessage) {
        self.append_items(prepared.type_items);
        if prepared.companion_items.is_empty()
            && prepared.nested_enums.is_empty()
            && prepared.nested.is_empty()
        {
            return;
        }
        let companion = self.get_or_insert_child(prepared.module_name);
        companion.add_origin(ModuleOrigin::Message {
            proto_fqn: prepared.proto_fqn,
        });
        companion.append_items(prepared.companion_items);
        for nested_enum in prepared.nested_enums {
            companion.install_enum(nested_enum);
        }
        for nested in prepared.nested {
            companion.install_message(nested);
        }
    }

    fn install_enum(&mut self, prepared: PreparedEnum) {
        self.append_items(prepared.items);
    }
}
