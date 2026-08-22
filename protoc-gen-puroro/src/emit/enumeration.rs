//! Emit a protobuf enum as a newtype-over-`i32` (open or closed).

use super::ident::{escape_ident, is_simple_ident};
use crate::case::to_upper_snake;
use crate::descriptor::features::EnumType;
use crate::error::{Error, Result};
use crate::resolved::Enum;
use ::proc_macro2::{Ident, Span, TokenStream};
use ::quote::quote;
use ::std::collections::BTreeSet;
use ::syn::Item;

/// Render an enum into its parent package or message module.
///
/// Defaults / `Default` / `HasDefault` use the **first defined** enumerator
/// (proto2 / proto3 / editions language guides). Open enums additionally require
/// that first value to be `0`.
pub(super) fn render_enum(e: &Enum<'_>) -> Result<Vec<Item>> {
    if !is_simple_ident(e.name()) {
        return Err(Error::Codegen(format!(
            "cannot use enum name `{}` as a Rust identifier",
            e.name()
        )));
    }

    let first = e
        .values()
        .next()
        .ok_or_else(|| Error::Codegen(format!("enum `{}` has no values", e.name())))?;

    for v in e.values() {
        if !is_simple_ident(v.name()) {
            return Err(Error::Codegen(format!(
                "cannot use enum value `{}` on `{}` as a Rust identifier",
                v.name(),
                e.name()
            )));
        }
    }
    if matches!(e.openness(), EnumType::Open) && first.number() != 0 {
        return Err(Error::Codegen(format!(
            "open enum `{}` must define 0 as its first value (got `{}` = {})",
            e.name(),
            first.name(),
            first.number()
        )));
    }

    let name = escape_ident(e.name());
    let default_const = variant_const_ident(e.name(), first.name())?;
    let default_self = quote! { Self::#default_const };
    let default_path = quote! { #name::#default_const };
    let variant_consts: Vec<TokenStream> = e
        .values()
        .map(|v| {
            let ident = variant_const_ident(e.name(), v.name())?;
            let number = v.number();
            Ok(quote! {
                pub const #ident: Self = Self(#number);
            })
        })
        .collect::<Result<_>>()?;

    let known_numbers: Vec<i32> = e
        .values()
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

    super::parse::parse_items(quote! {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #[repr(transparent)]
        pub struct #name(i32);

        impl #name {
            #(#variant_consts)*
        }

        impl ::core::default::Default for #name {
            fn default() -> Self {
                #default_self
            }
        }

        #convert_impls

        impl ::puroro_rt::ProtoEnumStorage for #name {
            fn to_wire(self) -> i32 {
                self.0
            }
        }

        // `CloneIn` / `DefaultIn` / `DeallocateIn`: `Copy` (+ `Default`) blankets
        // in `unmanaged`.

        impl ::puroro::HasDefault<#name> for ::puroro_rt::ProtoDefault {
            const DEFAULT: #name = #default_path;
        }
    })
}

/// Sorted unique numbers → `lo..=hi` runs and singleton `n`, joined by `|`.
fn known_value_pattern(numbers: &[i32]) -> TokenStream {
    let parts: Vec<TokenStream> = contiguous_groups(numbers)
        .into_iter()
        .map(|(lo, hi)| {
            if lo == hi {
                quote! { #lo }
            } else {
                quote! { #lo..=#hi }
            }
        })
        .collect();
    quote! { #(#parts)|* }
}

/// Inclusive `[lo, hi]` runs of consecutive values (`hi - lo + 1` members).
fn contiguous_groups(numbers: &[i32]) -> Vec<(i32, i32)> {
    let mut groups = Vec::new();
    let mut iter = numbers.iter().copied();
    let Some(mut lo) = iter.next() else {
        return groups;
    };
    let mut hi = lo;
    for n in iter {
        if hi.checked_add(1) == Some(n) {
            hi = n;
        } else {
            groups.push((lo, hi));
            lo = n;
            hi = n;
        }
    }
    groups.push((lo, hi));
    groups
}

/// `STATUS_UNSPECIFIED` on enum `Status` → `UNSPECIFIED`.
///
/// If stripping the enum-name prefix would leave a non-ident (e.g. `EDITION_2023`
/// → `2023`), keep the full value name instead.
pub(super) fn variant_const_ident(enum_name: &str, value_name: &str) -> Result<Ident> {
    let prefix = format!("{}_", to_upper_snake(enum_name));
    let rest = value_name.strip_prefix(&prefix).unwrap_or(value_name);
    let candidate = if is_simple_ident(rest) && !starts_with_digit(rest) {
        rest
    } else if is_simple_ident(value_name) {
        value_name
    } else {
        return Err(Error::Codegen(format!(
            "cannot derive a Rust constant name from enum value `{value_name}` on `{enum_name}`"
        )));
    };
    Ok(Ident::new(candidate, Span::call_site()))
}

fn starts_with_digit(name: &str) -> bool {
    name.chars().next().is_some_and(|c| c.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::quote::ToTokens;

    #[test]
    fn known_value_pattern_groups_runs() {
        let pat = |nums: &[i32]| known_value_pattern(nums).to_token_stream().to_string();
        assert_eq!(pat(&[]), "");
        assert_eq!(pat(&[7]), "7i32");
        assert_eq!(pat(&[0, 1, 2]), "0i32 ..= 2i32");
        assert_eq!(
            pat(&[0, 1, 2, 5, 10, 11]),
            "0i32 ..= 2i32 | 5i32 | 10i32 ..= 11i32"
        );
        assert_eq!(
            pat(&[i32::MAX - 1, i32::MAX]),
            format!("{}i32 ..= {}i32", i32::MAX - 1, i32::MAX)
        );
    }

    #[test]
    fn strips_enum_name_prefix() {
        let ident = variant_const_ident("Status", "STATUS_UNSPECIFIED").unwrap();
        assert_eq!(ident.to_string(), "UNSPECIFIED");
        let ident = variant_const_ident("Priority", "PRIORITY_HIGH").unwrap();
        assert_eq!(ident.to_string(), "HIGH");
    }

    #[test]
    fn keeps_full_name_when_strip_starts_with_digit() {
        let ident = variant_const_ident("Edition", "EDITION_2023").unwrap();
        assert_eq!(ident.to_string(), "EDITION_2023");
        let ident = variant_const_ident("Edition", "EDITION_PROTO2").unwrap();
        assert_eq!(ident.to_string(), "PROTO2");
    }
}
