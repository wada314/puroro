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

pub mod getter;
pub mod getter_opt;

use crate::internal::{EmptyFields, ImplProperties};
use crate::{tags, MessageImpl};
use ::std::marker::PhantomData;

pub struct MergedShared<T, U> {
    left: T,
    right: U,
}
impl<T, U> From<(T, U)> for MergedShared<T, U> {
    fn from((left, right): (T, U)) -> Self {
        Self { left, right }
    }
}
impl<MP, T, U> From<(T, U)> for MessageImpl<MP, tags::MergedImpl, EmptyFields, MergedShared<T, U>> {
    fn from(val: (T, U)) -> Self {
        MessageImpl::from_raw_parts(Default::default(), Into::into(val))
    }
}

pub trait IntoMergedMessage<MP> {
    type MergedMessage;
    fn into_message(self) -> Self::MergedMessage;
}

pub struct MergedImplProperties<T, U>(PhantomData<(T, U)>);
impl<T, U> ImplProperties for MergedImplProperties<T, U> {
    type ImplTag = tags::MergedImpl;
    type FieldsType = EmptyFields;
    type SharedType = MergedShared<T, U>;
}
