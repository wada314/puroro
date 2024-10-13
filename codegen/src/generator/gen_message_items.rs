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

pub mod gen_dynamic_message_impl;
pub mod gen_message_trait;

use self::gen_dynamic_message_impl::GenDynamicMessageImpls;
use self::gen_message_trait::GenTrait;
use super::CodeGeneratorOptions;
use crate::descriptor::DescriptorExt;
use crate::Result;
use ::std::rc::Rc;
use ::syn::Item;

pub struct GenMessageItems {
    gen_trait: GenTrait,
    gen_gm_impls: GenDynamicMessageImpls,
    #[allow(unused)]
    options: Rc<CodeGeneratorOptions>,
}

impl GenMessageItems {
    pub fn try_new<'a>(
        desc: &'a DescriptorExt<'a>,
        options: Rc<CodeGeneratorOptions>,
    ) -> Result<Self> {
        Ok(Self {
            gen_trait: GenTrait::try_new(desc, Rc::clone(&options))?,
            gen_gm_impls: GenDynamicMessageImpls::try_new(desc, Rc::clone(&options))?,
            options,
        })
    }

    pub fn gen_items(&self) -> Result<Vec<Item>> {
        let mut items = Vec::new();
        items.extend(self.gen_trait.gen_items()?);
        items.push(self.gen_gm_impls.gen_impl_message_trait()?);
        items.push(self.gen_gm_impls.gen_impl_message_app_trait()?);
        Ok(items)
    }
}
