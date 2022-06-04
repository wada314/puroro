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

use super::super::desc::MessageDescriptor;
use super::{MdFdIntoOwnedType, OwnedMessage};
use ::metako::*;
use ::std::marker::PhantomData;
use ::std::mem::{transmute, ManuallyDrop};
use ::std::ops::{Deref, DerefMut};
use ::std::ptr::null_mut;

/// A wrapper of Box<OwnedMessage<MD>>.
///
/// The important thing is that this type declaration does not
/// require any bounds for `MD` param.
/// We need this property to avoid recursive message's infinite
/// type recursion problem.
/// Instead of that property, this type internally erases the type info,
/// and is required to be manually dropped using `take` method.
pub struct BoxedMessage<MD>(ManuallyDrop<*mut ()>, PhantomData<MD>);
impl<MD> BoxedMessage<MD>
where
    MD: MessageDescriptor,
    list::Map<MdFdIntoOwnedType>: Func<MD::GetFieldListAsMdFd>,
{
    pub fn take(mut self) -> Box<OwnedMessage<MD>> {
        let ptr = self.0;
        self.0 = ManuallyDrop::new(null_mut());
        unsafe { Box::from_raw(transmute(ManuallyDrop::into_inner(ptr))) }
    }
}

impl<MD> Default for BoxedMessage<MD>
where
    MD: MessageDescriptor,
    list::Map<MdFdIntoOwnedType>: Func<MD::GetFieldListAsMdFd>,
    OwnedMessage<MD>: Default,
{
    fn default() -> Self {
        let boxed = Box::new(OwnedMessage::<MD>::default());
        Self(
            ManuallyDrop::new(unsafe { transmute(Box::into_raw(boxed)) }),
            PhantomData,
        )
    }
}

impl<MD> Drop for BoxedMessage<MD> {
    fn drop(&mut self) {
        debug_assert_eq!(
            *self.0,
            null_mut(),
            "This type should be explicitly cleaned up before automatic drop!"
        );
    }
}

impl<MD> Deref for BoxedMessage<MD>
where
    MD: MessageDescriptor,
    list::Map<MdFdIntoOwnedType>: Func<MD::GetFieldListAsMdFd>,
{
    type Target = OwnedMessage<MD>;
    fn deref(&self) -> &Self::Target {
        unsafe { transmute::<_, &OwnedMessage<MD>>(&self.0) }
    }
}

impl<MD> DerefMut for BoxedMessage<MD>
where
    MD: MessageDescriptor,
    list::Map<MdFdIntoOwnedType>: Func<MD::GetFieldListAsMdFd>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { transmute::<_, &mut OwnedMessage<MD>>(&mut self.0) }
    }
}
