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

use ::syn::Item;

use self::gen_dynamic_message_impl::GenDynamicMessageImpls;
use self::gen_message_trait::GenTrait;
use crate::descriptor::DescriptorExt;
use crate::Result;

pub struct GenMessageItems {
    gen_trait: GenTrait,
    gen_gm_impls: GenDynamicMessageImpls,
}

impl GenMessageItems {
    pub fn try_new<'a>(desc: &'a DescriptorExt<'a>) -> Result<Self> {
        Ok(Self {
            gen_trait: GenTrait::try_new(desc)?,
            gen_gm_impls: GenDynamicMessageImpls::try_new(desc)?,
        })
    }

    pub fn gen_items(&self) -> Result<Vec<Item>> {
        let trait_items = self.gen_trait.gen_items()?;
        let impl_items = self.gen_gm_impls.gen_impl_message_trait()?;
        Ok(trait_items
            .into_iter()
            .chain(::std::iter::once(impl_items))
            .collect())
    }
}
