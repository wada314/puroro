//! Emit message-local `[default = …]` marker ZSTs and `SingularField` `D` args.

use super::enumeration;
use super::type_path::fqn_to_enum_root_path;
use crate::default_value::{CustomDefault, DefaultLit};
use crate::error::{Error, Result};
use crate::field_kind::WireTypeKind;
use ::proc_macro2::{Ident, Span, TokenStream};
use ::quote::quote;
use ::syn::{Item, LitByteStr, LitStr, Type, parse_quote};

/// `, L, D` type-argument tail for `SingularField<…, A, …>` (may be empty).
pub(super) fn layout_and_default_args(
    layout_ty: &Option<Type>,
    default_marker: Option<&Ident>,
) -> TokenStream {
    match (layout_ty, default_marker) {
        (None, None) => TokenStream::new(),
        (Some(layout), None) => quote! { , #layout },
        (None, Some(default_ty)) => quote! { , ::puroro_rt::Inline, #default_ty },
        (Some(layout), Some(default_ty)) => quote! { , #layout, #default_ty },
    }
}

pub(super) fn marker_ident(custom: &CustomDefault) -> Ident {
    Ident::new(&custom.marker_name, Span::call_site())
}

/// `pub struct …; impl HasDefault<…> for … { … }`
pub(super) fn render_marker_item(
    custom: &CustomDefault,
    wire: &WireTypeKind<'_>,
) -> Result<Vec<Item>> {
    let marker = marker_ident(custom);
    let (ty, expr, lifetime) = has_default_ty_and_expr(&custom.lit, wire)?;
    if lifetime {
        super::parse::parse_items(quote! {
            pub struct #marker;
            impl<'a> ::puroro::HasDefault<#ty> for #marker {
                const DEFAULT: #ty = #expr;
            }
        })
    } else {
        super::parse::parse_items(quote! {
            pub struct #marker;
            impl ::puroro::HasDefault<#ty> for #marker {
                const DEFAULT: #ty = #expr;
            }
        })
    }
}

fn has_default_ty_and_expr(
    lit: &DefaultLit,
    wire: &WireTypeKind<'_>,
) -> Result<(Type, TokenStream, bool)> {
    Ok(match lit {
        DefaultLit::Bool(v) => (parse_quote! { bool }, quote! { #v }, false),
        DefaultLit::I32(v) => (parse_quote! { i32 }, quote! { #v }, false),
        DefaultLit::I64(v) => (parse_quote! { i64 }, quote! { #v }, false),
        DefaultLit::U32(v) => (parse_quote! { u32 }, quote! { #v }, false),
        DefaultLit::U64(v) => (parse_quote! { u64 }, quote! { #v }, false),
        DefaultLit::F32Bits(bits) => {
            let bits = *bits;
            (
                parse_quote! { f32 },
                quote! { f32::from_bits(#bits) },
                false,
            )
        }
        DefaultLit::F64Bits(bits) => {
            let bits = *bits;
            (
                parse_quote! { f64 },
                quote! { f64::from_bits(#bits) },
                false,
            )
        }
        DefaultLit::Str(s) => {
            let lit = LitStr::new(s, Span::call_site());
            (parse_quote! { &'a str }, quote! { #lit }, true)
        }
        DefaultLit::Bytes(b) => {
            let lit = LitByteStr::new(b, Span::call_site());
            (parse_quote! { &'a [u8] }, quote! { #lit }, true)
        }
        DefaultLit::Enum { value_name, .. } => {
            let WireTypeKind::Enum { ty, .. } = wire else {
                return Err(Error::Codegen(
                    "internal error: enum default lit without enum wire type".into(),
                ));
            };
            let path = fqn_to_enum_root_path(ty)?;
            let variant = enumeration::variant_const_ident(ty.name(), value_name)?;
            (parse_quote! { #path }, quote! { #path::#variant }, false)
        }
    })
}
