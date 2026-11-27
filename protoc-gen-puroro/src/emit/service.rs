//! Emit tonic client/server stubs from resolved [`Service`] values.
//!
//! Uses [`tonic_build::CodeGenBuilder`] with [`::puroro::tonic::PuroroCodec`].
//! Stub modules are sealed tonic style; we inject `mod _root` so request /
//! response paths can use `self::_root::…` like the rest of the forest.

use super::ident::is_simple_ident;
use super::type_path::fqn_to_message_root_path;
use crate::case::to_snake_case;
use crate::error::{Error, Result};
use crate::module_tree::layout::nested_root_alias;
use crate::resolved::{Method, Service};
use ::proc_macro2::TokenStream;
use ::quote::quote;
use ::syn::{Item, ItemMod, parse_quote, parse2};
use ::tonic_build::{CodeGenBuilder, Method as TonicMethodTrait, Service as TonicServiceTrait};

const CODEC_PATH: &str = "::puroro::tonic::PuroroCodec";

/// Adapter that feeds one resolved service into tonic-build.
struct TonicServiceSpec {
    name: String,
    package: String,
    identifier: String,
    methods: Vec<TonicMethodSpec>,
    comments: Vec<String>,
}

struct TonicMethodSpec {
    /// Rust fn name (`say_hello`).
    name: String,
    /// Proto RPC name (`SayHello`).
    identifier: String,
    client_streaming: bool,
    server_streaming: bool,
    request: TokenStream,
    response: TokenStream,
    comments: Vec<String>,
}

impl TonicServiceSpec {
    fn from_resolved(service: &Service<'_>, package: &str) -> Result<Self> {
        if !is_simple_ident(service.name()) {
            return Err(Error::Codegen(format!(
                "cannot use service name `{}` as a Rust identifier",
                service.name()
            )));
        }
        let methods = service
            .methods()
            .map(|m| TonicMethodSpec::from_resolved(m))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self {
            name: service.name().to_owned(),
            package: package.to_owned(),
            identifier: service.name().to_owned(),
            methods,
            comments: comment_lines(service.comments().doc_text()),
        })
    }
}

impl TonicMethodSpec {
    fn from_resolved(method: &Method<'_>) -> Result<Self> {
        if !is_simple_ident(method.name()) {
            return Err(Error::Codegen(format!(
                "cannot use method name `{}` as a Rust identifier",
                method.name()
            )));
        }
        let request = {
            let path = fqn_to_message_root_path(method.input())?;
            quote! { #path }
        };
        let response = {
            let path = fqn_to_message_root_path(method.output())?;
            quote! { #path }
        };
        Ok(Self {
            name: to_snake_case(method.name()),
            identifier: method.name().to_owned(),
            client_streaming: method.client_streaming(),
            server_streaming: method.server_streaming(),
            request,
            response,
            comments: comment_lines(method.comments().doc_text()),
        })
    }
}

impl TonicServiceTrait for TonicServiceSpec {
    type Comment = String;
    type Method = TonicMethodSpec;

    fn name(&self) -> &str {
        &self.name
    }

    fn package(&self) -> &str {
        &self.package
    }

    fn identifier(&self) -> &str {
        &self.identifier
    }

    fn methods(&self) -> &[Self::Method] {
        &self.methods
    }

    fn comment(&self) -> &[Self::Comment] {
        &self.comments
    }
}

impl TonicMethodTrait for TonicMethodSpec {
    type Comment = String;

    fn name(&self) -> &str {
        &self.name
    }

    fn identifier(&self) -> &str {
        &self.identifier
    }

    fn codec_path(&self) -> &str {
        CODEC_PATH
    }

    fn client_streaming(&self) -> bool {
        self.client_streaming
    }

    fn server_streaming(&self) -> bool {
        self.server_streaming
    }

    fn comment(&self) -> &[Self::Comment] {
        &self.comments
    }

    fn request_response_name(
        &self,
        _proto_path: &str,
        _compile_well_known_types: bool,
    ) -> (TokenStream, TokenStream) {
        (self.request.clone(), self.response.clone())
    }
}

/// Render cfg-gated `{service}_client` / `{service}_server` modules for `service`.
pub(super) fn render_service_items(
    service: &Service<'_>,
    package: &str,
    build_client: bool,
    build_server: bool,
) -> Result<Vec<Item>> {
    if !build_client && !build_server {
        return Ok(Vec::new());
    }
    let spec = TonicServiceSpec::from_resolved(service, package)?;
    let mut builder = CodeGenBuilder::new();
    builder.emit_package(true).build_transport(true);

    let mut items = Vec::new();
    if build_client {
        items.push(wrap_tonic_module(
            builder.generate_client(&spec, ""),
            "client",
            service.name(),
        )?);
    }
    if build_server {
        items.push(wrap_tonic_module(
            builder.generate_server(&spec, ""),
            "server",
            service.name(),
        )?);
    }
    Ok(items)
}

fn wrap_tonic_module(tokens: TokenStream, kind: &str, service_name: &str) -> Result<Item> {
    let file: syn::File = parse2(tokens).map_err(|e| {
        Error::Codegen(format!(
            "tonic-build {kind} output for service `{service_name}` did not parse: {e}"
        ))
    })?;
    let mut mods: Vec<ItemMod> = file
        .items
        .into_iter()
        .filter_map(|item| match item {
            Item::Mod(m) => Some(m),
            _ => None,
        })
        .collect();
    if mods.len() != 1 {
        return Err(Error::Codegen(format!(
            "tonic-build {kind} output for service `{service_name}` expected one module, got {}",
            mods.len()
        )));
    }
    let mut module = mods.pop().expect("len checked");
    module
        .attrs
        .insert(0, parse_quote!(#[cfg(feature = "tonic")]));
    match &mut module.content {
        Some((_, items)) => {
            items.insert(0, nested_root_alias());
        }
        None => {
            return Err(Error::Codegen(format!(
                "tonic-build {kind} module for service `{service_name}` has no body"
            )));
        }
    }
    Ok(Item::Mod(module))
}

fn comment_lines(text: Option<&str>) -> Vec<String> {
    match text {
        Some(text) if !text.trim().is_empty() => text.lines().map(str::to_owned).collect(),
        _ => Vec::new(),
    }
}
