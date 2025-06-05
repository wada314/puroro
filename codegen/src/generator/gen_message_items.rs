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

pub mod gen_struct;
pub mod gen_traits;

use self::gen_struct::GenStruct;
use self::gen_traits::GenTraits;
use super::CodeGeneratorOptions;
use crate::descriptor::DescriptorExt;
use crate::Result;
use ::std::rc::Rc;
use ::syn::Item;

pub struct GenMessageItems {
    gen_trait: Rc<GenTraits>,
    gen_struct: Rc<GenStruct>,
    #[allow(unused)]
    options: Rc<CodeGeneratorOptions>,
}

impl GenMessageItems {
    pub fn try_new<'a>(
        desc: &'a DescriptorExt<'a>,
        options: Rc<CodeGeneratorOptions>,
    ) -> Result<Self> {
        let gen_trait = Rc::new(GenTraits::try_new(desc, Rc::clone(&options))?);
        let gen_struct = Rc::new(GenStruct::try_new(
            desc,
            Rc::clone(&gen_trait),
            Rc::clone(&options),
        )?);
        Ok(Self {
            gen_trait,
            gen_struct,
            options,
        })
    }

    pub fn gen_items(&self) -> Result<Vec<Item>> {
        let mut items = Vec::new();
        items.extend(self.gen_struct.gen_items()?);
        items.extend(self.gen_trait.gen_items()?);
        Ok(items)
    }
}
