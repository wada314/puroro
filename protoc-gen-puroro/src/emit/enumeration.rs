//! Emit a protobuf enum as a newtype-over-`i32` (open or closed).

use crate::descriptor::features::EnumType;
use crate::error::{Error, Result};
use crate::resolved::Enum;
use ::proc_macro2::{Ident, Span, TokenStream};
use ::quote::quote;
use ::std::collections::BTreeSet;

/// Render a file-level enum type into the package (or forest-root) module.
pub(super) fn render_enum(e: &Enum<'_>) -> Result<TokenStream> {
    if !is_simple_ident(e.name()) {
        return Err(Error::Codegen(format!(
            "enum name `{}` is not a simple Rust identifier",
            e.name()
        )));
    }
    if e.parent().is_some() {
        return Err(Error::Codegen(format!(
            "nested enum `{}` is not supported yet",
            e.name()
        )));
    }

    let values: Vec<_> = e.values().collect();
    if values.is_empty() {
        return Err(Error::Codegen(format!("enum `{}` has no values", e.name())));
    }
    let zero = values.iter().find(|v| v.number() == 0).ok_or_else(|| {
        Error::Codegen(format!(
            "enum `{}` has no zero value (required for generated defaults)",
            e.name()
        ))
    })?;

    for v in &values {
        if !is_simple_ident(v.name()) {
            return Err(Error::Codegen(format!(
                "enum value `{}` on `{}` is not a simple Rust identifier",
                v.name(),
                e.name()
            )));
        }
    }

    let name = Ident::new(e.name(), Span::call_site());
    let zero_const = variant_const_ident(e.name(), zero.name())?;
    let variant_consts: Vec<TokenStream> = values
        .iter()
        .map(|v| {
            let ident = variant_const_ident(e.name(), v.name())?;
            let number = v.number();
            Ok(quote! {
                pub const #ident: Self = Self(#number);
            })
        })
        .collect::<Result<_>>()?;

    let known_numbers: Vec<i32> = values
        .iter()
        .map(|v| v.number())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let known_pat = known_value_pattern(&known_numbers);

    let convert_impls = match e.openness() {
        EnumType::Open => quote! {
            impl ::core::convert::From<i32> for #name {
                fn from(value: i32) -> Self {
                    Self(value)
                }
            }

            impl ::core::convert::TryFrom<#name> for i32 {
                type Error = i32;

                fn try_from(value: #name) -> ::core::result::Result<Self, Self::Error> {
                    match value.0 {
                        #known_pat => ::core::result::Result::Ok(value.0),
                        other => ::core::result::Result::Err(other),
                    }
                }
            }

            impl ::puroro_rt::OpenEnum for #name {}
        },
        EnumType::Closed => quote! {
            impl ::core::convert::TryFrom<i32> for #name {
                type Error = i32;

                fn try_from(value: i32) -> ::core::result::Result<Self, Self::Error> {
                    match value {
                        #known_pat => ::core::result::Result::Ok(Self(value)),
                        other => ::core::result::Result::Err(other),
                    }
                }
            }

            impl ::core::convert::From<#name> for i32 {
                fn from(value: #name) -> Self {
                    value.0
                }
            }

            impl ::puroro_rt::ClosedEnum for #name {}
        },
    };

    Ok(quote! {
        #[derive(::core::clone::Clone, ::core::marker::Copy, ::core::fmt::Debug, ::core::cmp::PartialEq, ::core::cmp::Eq, ::core::hash::Hash)]
        #[repr(transparent)]
        pub struct #name(i32);

        impl #name {
            #(#variant_consts)*
        }

        impl ::core::default::Default for #name {
            fn default() -> Self {
                Self::#zero_const
            }
        }

        #convert_impls

        impl ::puroro_rt::ProtoEnumStorage for #name {
            fn proto_zero() -> Self {
                Self::#zero_const
            }

            fn to_wire(self) -> i32 {
                self.0
            }
        }

        impl<A: ::allocator_api2::alloc::Allocator> ::puroro_rt::CloneIn<A> for #name {
            #[inline]
            fn clone_in(&self, _alloc: A) -> Self {
                *self
            }
        }

        impl<A: ::allocator_api2::alloc::Allocator> ::puroro_rt::DeallocateIn<A> for #name {
            #[inline]
            unsafe fn deallocate_in(self, _alloc: A) {}
        }

        impl ::puroro::HasDefault<#name> for ::puroro_rt::ProtoDefault {
            const DEFAULT: #name = #name::#zero_const;
        }
    })
}

/// Contiguous known values → `lo..=hi`; otherwise an `|` pattern.
fn known_value_pattern(numbers: &[i32]) -> TokenStream {
    if let Some((lo, hi)) = contiguous_range(numbers) {
        quote! { #lo..=#hi }
    } else {
        quote! { #(#numbers)|* }
    }
}

fn contiguous_range(numbers: &[i32]) -> Option<(i32, i32)> {
    let (&lo, &hi) = (numbers.first()?, numbers.last()?);
    for (i, &n) in numbers.iter().enumerate() {
        let expected = lo.checked_add(i as i32)?;
        if n != expected {
            return None;
        }
    }
    Some((lo, hi))
}

/// `STATUS_UNSPECIFIED` on enum `Status` → `UNSPECIFIED`.
fn variant_const_ident(enum_name: &str, value_name: &str) -> Result<Ident> {
    let prefix = format!("{}_", camel_to_screaming_snake(enum_name));
    let rest = value_name.strip_prefix(&prefix).unwrap_or(value_name);
    if !is_simple_ident(rest) {
        return Err(Error::Codegen(format!(
            "enum value `{value_name}` on `{enum_name}` does not yield a simple Rust const \
             identifier after prefix strip (got `{rest}`)"
        )));
    }
    Ok(Ident::new(rest, Span::call_site()))
}

fn camel_to_screaming_snake(name: &str) -> String {
    let chars: Vec<char> = name.chars().collect();
    let mut out = String::new();
    for (i, &c) in chars.iter().enumerate() {
        if c.is_ascii_uppercase() && i > 0 {
            let prev_lower = chars[i - 1].is_ascii_lowercase();
            let next_lower = chars.get(i + 1).is_some_and(|n| n.is_ascii_lowercase());
            if prev_lower || next_lower {
                out.push('_');
            }
        }
        out.push(c.to_ascii_uppercase());
    }
    out
}

fn is_simple_ident(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {
            chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_enum_name_prefix() {
        let ident = variant_const_ident("Status", "STATUS_UNSPECIFIED").unwrap();
        assert_eq!(ident.to_string(), "UNSPECIFIED");
        let ident = variant_const_ident("Priority", "PRIORITY_HIGH").unwrap();
        assert_eq!(ident.to_string(), "HIGH");
    }

    #[test]
    fn camel_to_screaming_handles_multi_word() {
        assert_eq!(camel_to_screaming_snake("Status"), "STATUS");
        assert_eq!(camel_to_screaming_snake("FooBar"), "FOO_BAR");
    }
}
