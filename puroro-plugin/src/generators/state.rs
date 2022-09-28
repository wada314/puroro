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

use super::Message;
use crate::descriptor_resolver::DescriptorResolver;
use crate::restructure::MessageOrEnumRef;
pub use crate::restructure::Syntax;
use crate::utils::Fqtn;
use crate::{ErrorKind, Result};
use ::std::collections::HashMap;
use ::std::rc::Rc;

#[derive(Debug, Default)]
pub struct State {
    fqtn_to_generated_message_map: HashMap<Fqtn<String>, Rc<Message>>,
    fqtn_to_bit_slice_allocation_map: HashMap<Fqtn<String>, BitSliceAllocation>,
}
impl State {
    pub fn fqtn_to_bit_slice_allocation_mut<S: AsRef<str>>(
        &mut self,
        fqtn: &Fqtn<S>,
    ) -> &mut BitSliceAllocation {
        self.fqtn_to_bit_slice_allocation_map
            .entry(fqtn.to_owned())
            .or_default()
    }

    pub fn fqtn_to_generated_message<S: AsRef<str>>(
        &mut self,
        fqtn: &Fqtn<S>,
        resolver: &DescriptorResolver,
    ) -> Result<Rc<Message>> {
        todo!()
    }
}

#[derive(Debug, Default)]
pub struct BitSliceAllocation {
    bit_slice_len: usize,
    finalized: bool,
}
impl BitSliceAllocation {
    pub fn bit_slice_len_mut(&mut self) -> Result<&mut usize> {
        if self.finalized {
            Err(ErrorKind::InternalError {
                detail: "Bad allocation order for bitslice.".to_string(),
            })?
        } else {
            Ok(&mut self.bit_slice_len)
        }
    }
    pub fn finalize(&mut self) -> Result<usize> {
        if self.finalized {
            Err(ErrorKind::InternalError {
                detail: "Multiple .finalize() call for bitslice allocation".to_string(),
            })?
        }
        self.finalized = true;
        Ok(self.bit_slice_len)
    }
}
