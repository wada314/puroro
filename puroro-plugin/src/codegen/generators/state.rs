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

use super::super::descriptor_resolver::DescriptorResolver;
pub use super::super::restructure::Syntax;
use super::super::utils::Fqtn;
use super::{Enum, Message};
use crate::{ErrorKind, Result};
use ::std::collections::HashMap;
use ::std::fmt::Debug;
use ::std::rc::Rc;

#[derive(Debug, Default)]
pub struct State {
    fqtn_to_generated_message_map: HashMap<Fqtn, Rc<Message>>,
    fqtn_to_generated_enum_map: HashMap<Fqtn, Rc<Enum>>,
    fqtn_to_bit_slice_allocation_map: HashMap<Fqtn, BitSliceAllocation>,
}
impl State {
    pub fn fqtn_to_bit_slice_allocation_mut(&mut self, fqtn: &Fqtn) -> &mut BitSliceAllocation {
        self.fqtn_to_bit_slice_allocation_map
            .entry(fqtn.to_owned())
            .or_default()
    }

    pub fn fqtn_to_generated_message<'a>(
        &mut self,
        fqtn: &Fqtn,
        resolver: &'a DescriptorResolver<'a>,
    ) -> Result<Rc<Message>> {
        if let Some(found) = self.fqtn_to_generated_message_map.get(fqtn) {
            Ok(Rc::clone(found))
        } else {
            let message_desc = resolver.fqtn_to_message(fqtn)?;
            let generated = Rc::new(Message::try_new(&message_desc, resolver, self)?);
            self.fqtn_to_generated_message_map
                .insert(fqtn.clone(), Rc::clone(&generated));
            Ok(generated)
        }
    }

    pub fn fqtn_to_generated_enum<'a>(
        &mut self,
        fqtn: &Fqtn,
        resolver: &'a DescriptorResolver<'a>,
    ) -> Result<Rc<Enum>> {
        if let Some(found) = self.fqtn_to_generated_enum_map.get(fqtn) {
            Ok(Rc::clone(found))
        } else {
            let enum_desc = resolver.fqtn_to_enum(fqtn)?;
            let generated = Rc::new(Enum::try_new(&enum_desc, resolver)?);
            self.fqtn_to_generated_enum_map
                .insert(fqtn.clone(), Rc::clone(&generated));
            Ok(generated)
        }
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
