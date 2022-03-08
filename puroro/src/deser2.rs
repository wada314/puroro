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

use crate::internal::types::WireType;
use crate::internal::variant::Variant;
use crate::internal::MatchFieldNumber;
use crate::tags;
use crate::{ErrorKind, MessageImpl, Result};
use ::std::io::Result as IoResult;
use ::std::io::{BufReader, Read};

pub trait DeserFromRead<R: Read> {
    fn deser_from_read<'a, 'b>(
        &'a mut self,
        read: &'b mut R,
    ) -> (DeserFromReadContext<'a, 'b, R>, Result<()>);
}

impl<R: Read, MP, FieldsType, SharedType> DeserFromRead<R>
    for MessageImpl<MP, tags::OwnedImpl, FieldsType, SharedType>
{
    fn deser_from_read<'a, 'b>(
        &'a mut self,
        read: &'b mut R,
    ) -> (DeserFromReadContext<'a, 'b, R>, Result<()>) {
        let mut context = DeserFromReadContext {
            message_stack: vec![self],
            read: BufReader::new(read),
        };
        let result = context.continue_deser();
        (context, result)
    }
}

pub struct DeserFromReadContext<'a, 'b, R> {
    message_stack: Vec<&'a mut dyn DeserFromRead<R>>,
    read: BufReader<&'b mut R>,
}

impl<'a, 'b, R: Read> DeserFromReadContext<'a, 'b, R> {
    pub fn continue_deser(&mut self) -> Result<()> {
        todo!()
    }
}
