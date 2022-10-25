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

use crate::Result;
use ::proc_macro2::TokenStream;
use ::quote::quote;
use ::syn;

use super::{descriptor_resolver::DescriptorResolver, utils::Package};

pub struct ModuleGen {}

impl ModuleGen {
    pub fn try_from_package<'a, S: AsRef<str>>(
        p: Package<S>,
        resolver: &'a DescriptorResolver,
    ) -> Result<TokenStream> {
        Ok(quote! {
            mod foo {}
        })
    }
}
