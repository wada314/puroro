// Copyright 2021 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod blanket_both;
mod blanket_either;
mod blanket_either_or_both;
mod blanket_option;
mod blanket_ref;
mod dynamic_message;

use super::field::Field;
use crate::cases::{Case, convert_into_case};
use crate::descriptor::DescriptorExt;
use crate::generator::CodeGeneratorOptions;
use crate::{Result, ResultExt};
use ::culpa::throws;
use ::puroro::Either;
use ::quote::{format_ident, quote};
use ::std::iter::once;
use ::std::rc::Rc;
use ::syn::{Block, Ident, ImplItemFn, Item, Path, parse2};
use blanket_both::GenBlanketBothImpls;
use blanket_either::GenBlanketEitherImpls;
use blanket_either_or_both::GenBlanketEitherOrBothImpls;
use blanket_option::GenBlanketOptionImpls;
use blanket_ref::GenBlanketRefImpls;
use dynamic_message::DynamicMessageImplsGenerator;

type Error = crate::ErrorKind;

pub struct GenTraits {
    view_trait_name: Ident,
    try_view_trait_name: Ident,
    fields: Vec<Field>,
    options: Rc<CodeGeneratorOptions>,
}

pub trait ImplsGenerator {
    #[throws]
    fn generate<'a>(
        &self,
        view_trait_path: &Path,
        try_view_trait_path: &Path,
        fields: Box<dyn 'a + Iterator<Item = &'a Field>>,
    ) -> Vec<Item>;
}

#[throws]
fn impls_helper<'a, F, G>(
    fields: impl Iterator<Item = &'a Field>,
    gen_getter: F,
    gen_has_method: G,
    is_try_trait: bool,
) -> Vec<ImplItemFn>
where
    F: Fn(&Field) -> Result<Block>,
    G: Fn(&Field) -> Result<Block>,
{
    fields
        .map(|f| {
            let get_method: ImplItemFn = {
                let signature = if is_try_trait {
                    f.try_getter_signature()
                } else {
                    f.getter_signature()
                };
                let body = gen_getter(f)?;
                parse2(quote! {
                    #signature #body
                })?
            };
            let has_method: Option<ImplItemFn> = match (is_try_trait, f) {
                (false, Field::Explicit { has_method_signature: signature, .. })
                | (false, Field::Implicit { has_method_signature: signature, .. })
                | (true, Field::Explicit { try_has_method_signature: signature, .. })
                | (true, Field::Implicit { try_has_method_signature: signature, .. }) => {
                    let body = gen_has_method(f)?;
                    Some(parse2(quote! {
                        #signature #body
                    })?)
                }
                _ => None,
            };
            Ok(once(get_method).chain(has_method.into_iter()))
        })
        .flat_map(ResultExt::transpose_iter)
        .collect::<Result<Vec<_>>>()?
}

impl GenTraits {
    #[throws]
    pub fn try_new<'a>(desc: &'a DescriptorExt<'a>, options: Rc<CodeGeneratorOptions>) -> Self {
        let current_path = Rc::new(desc.current_path().to_owned());
        Self {
            view_trait_name: Self::gen_view_trait_name(desc.name())?,
            try_view_trait_name: Self::gen_try_view_trait_name(desc.name())?,
            fields: desc
                .non_oneof_fields()?
                .into_iter()
                .map(|f| Field::try_new(f, Rc::clone(&current_path), Rc::clone(&options)))
                .collect::<Result<Vec<_>>>()?,
            options,
        }
    }

    pub fn view_trait_name(&self) -> &Ident {
        &self.view_trait_name
    }

    pub fn try_view_trait_name(&self) -> &Ident {
        &self.try_view_trait_name
    }

    #[throws]
    pub fn gen_view_trait_name(message_name: &str) -> Ident {
        format_ident!("{}View", convert_into_case(message_name, Case::CamelCase))
    }

    #[throws]
    fn gen_try_view_trait_name(message_name: &str) -> Ident {
        format_ident!(
            "Try{}View",
            convert_into_case(message_name, Case::CamelCase)
        )
    }

    #[throws]
    pub fn gen_items(&self) -> Vec<Item> {
        let view_trait_def = self.gen_view_trait()?;
        let view_trait_name = &self.view_trait_name;
        let view_trait_path: Path = parse2(quote! { self::#view_trait_name })?;
        let try_trait_def = self.gen_try_view_trait()?;
        let try_trait_name = &self.try_view_trait_name;
        let try_trait_path: Path = parse2(quote! { self::#try_trait_name })?;

        let blanket_impl_generators: Vec<Rc<dyn ImplsGenerator>> = vec![
            Rc::new(GenBlanketRefImpls::new(Rc::clone(&self.options))),
            Rc::new(GenBlanketOptionImpls::new(Rc::clone(&self.options))),
            Rc::new(GenBlanketBothImpls::new(Rc::clone(&self.options))),
            Rc::new(GenBlanketEitherImpls::new(Rc::clone(&self.options))),
            Rc::new(GenBlanketEitherOrBothImpls::new(Rc::clone(&self.options))),
            Rc::new(DynamicMessageImplsGenerator::new(Rc::clone(&self.options))),
        ];
        let blanket_impls = blanket_impl_generators
            .iter()
            .map(|g| {
                g.generate(
                    &view_trait_path,
                    &try_trait_path,
                    Box::new(self.fields.iter()),
                )
            })
            .map(|r| match r {
                Ok(vec) => Either::Left(vec.into_iter().map(Ok)),
                Err(e) => Either::Right(once(Err(e))),
            })
            .flatten()
            .collect::<Result<Vec<_>>>()?;

        [view_trait_def, try_trait_def]
            .into_iter()
            .chain(blanket_impls)
            .collect()
    }

    #[throws]
    fn gen_view_trait(&self) -> Item {
        let trait_name = &self.view_trait_name;
        let try_trait_name = &self.try_view_trait_name;
        let getters = self
            .fields
            .iter()
            .map(|f| f.getter_signature())
            .collect::<Vec<_>>();
        let has_methods = self
            .fields
            .iter()
            .filter_map(|f| match f {
                Field::Implicit { has_method_signature, .. }
                | Field::Explicit { has_method_signature, .. } => Some(has_method_signature),
                _ => None,
            })
            .collect::<Vec<_>>();
        parse2(quote! {
            pub trait #trait_name: self::#try_trait_name {
                #(#getters;)*
                #(#has_methods;)*
            }
        })?
    }

    #[throws]
    fn gen_try_view_trait(&self) -> Item {
        let trait_name = &self.try_view_trait_name;
        let try_getters = self
            .fields
            .iter()
            .map(|f| f.try_getter_signature())
            .collect::<Vec<_>>();
        let try_has_methods = self
            .fields
            .iter()
            .filter_map(|f| match f {
                Field::Implicit { try_has_method_signature, .. }
                | Field::Explicit { try_has_method_signature, .. } => {
                    Some(try_has_method_signature)
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        parse2(quote! {
            pub trait #trait_name {
                #(#try_getters;)*
                #(#try_has_methods;)*
            }
        })?
    }
}
