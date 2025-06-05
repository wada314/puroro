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

use ::std::rc::Rc;

use super::gen_traits::GenTraits;
use crate::cases::{convert_into_case, Case};
use crate::descriptor::{DescriptorExt, FieldDescriptorExt};
use crate::generator::{to_ident, CodeGeneratorOptions};
use crate::ErrorKind;
use ::culpa::throws;
use ::quote::quote;
use ::syn::{parse2, Ident, Item};

type Error = ErrorKind;

pub struct GenStruct {
    struct_name: Ident,
    options: Rc<CodeGeneratorOptions>,
    gen_traits: Rc<GenTraits>,
}

impl GenStruct {
    #[throws]
    pub fn try_new<'a>(
        desc: &'a DescriptorExt<'a>,
        gen_traits: Rc<GenTraits>,
        options: Rc<CodeGeneratorOptions>,
    ) -> Self {
        Self {
            struct_name: Self::struct_name(desc.name())?,
            options,
            gen_traits,
        }
    }

    #[throws]
    pub fn gen_items(&self) -> Vec<Item> {
        vec![self.gen_struct()?]
    }

    #[throws]
    fn struct_name(message_name: &str) -> Ident {
        to_ident(&convert_into_case(message_name, Case::CamelCase))
    }

    #[throws]
    fn gen_struct(&self) -> Item {
        let struct_name = &self.struct_name;
        parse2(quote! {
            pub struct #struct_name {
                foo: i32,
            }
        })?
    }
}
