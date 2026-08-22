//! Emit message-local `[default = …]` marker ZSTs.

use super::enumeration;
use super::type_path::fqn_to_enum_root_path;
use crate::default_value::{CustomDefault, DefaultLit};
use crate::error::{Error, Result};
use crate::field_kind::WireTypeKind;
use ::proc_macro2::{Ident, Span, TokenStream};
use ::quote::quote;
use ::syn::{Item, Lifetime, LitByteStr, LitStr, Type, parse_quote};

/// `pub struct …; impl HasDefault<…> for … { … }`
pub(super) fn render_marker_item(
    custom: &CustomDefault,
    wire: &WireTypeKind<'_>,
) -> Result<Vec<Item>> {
    let marker = Ident::new(&custom.marker_name, Span::call_site());
    let (ty, expr, lifetime) = has_default_ty_and_expr(&custom.lit, wire)?;
    let lifetime = lifetime.iter();
    super::parse::parse_items(quote! {
        pub struct #marker;
        impl #(<#lifetime>)* ::puroro::HasDefault<#ty> for #marker {
            const DEFAULT: #ty = #expr;
        }
    })
}

fn has_default_ty_and_expr(
    lit: &DefaultLit,
    wire: &WireTypeKind<'_>,
) -> Result<(Type, TokenStream, Option<Lifetime>)> {
    Ok(match lit {
        DefaultLit::Bool(v) => (parse_quote! { bool }, quote! { #v }, None),
        DefaultLit::I32(v) => (parse_quote! { i32 }, quote! { #v }, None),
        DefaultLit::I64(v) => (parse_quote! { i64 }, quote! { #v }, None),
        DefaultLit::U32(v) => (parse_quote! { u32 }, quote! { #v }, None),
        DefaultLit::U64(v) => (parse_quote! { u64 }, quote! { #v }, None),
        DefaultLit::F32Bits(bits) => {
            let bits = *bits;
            (parse_quote! { f32 }, quote! { f32::from_bits(#bits) }, None)
        }
        DefaultLit::F64Bits(bits) => {
            let bits = *bits;
            (parse_quote! { f64 }, quote! { f64::from_bits(#bits) }, None)
        }
        DefaultLit::Str(s) => {
            let lit = LitStr::new(s, Span::call_site());
            (
                parse_quote! { &'a str },
                quote! { #lit },
                Some(parse_quote! { 'a }),
            )
        }
        DefaultLit::Bytes(b) => {
            let lit = LitByteStr::new(b, Span::call_site());
            (
                parse_quote! { &'a [u8] },
                quote! { #lit },
                Some(parse_quote! { 'a }),
            )
        }
        DefaultLit::Enum { value_name, .. } => {
            let WireTypeKind::Enum { ty, .. } = wire else {
                return Err(Error::internal("enum default lit without enum wire type"));
            };
            let path = fqn_to_enum_root_path(ty)?;
            let variant = enumeration::variant_const_ident(ty.name(), value_name)?;
            (parse_quote! { #path }, quote! { #path::#variant }, None)
        }
    })
}
