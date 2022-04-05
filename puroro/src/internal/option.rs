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

use crate::desc::StaticFieldDescriptor;
use crate::internal::bool::{False, True};
use crate::message::{AsMessageImplRef, MessageFieldGetter};
use crate::tags;
use crate::Result;

impl<'msg, FD, R, InnerM, LabelTag, TypeTag> MessageFieldGetter<'msg, FD, R> for Option<InnerM>
where
    FD: StaticFieldDescriptor<FieldLabelTag = LabelTag, FieldTypeTag = TypeTag>,
    LabelTag: tags::FieldLabelTag,
    TypeTag: tags::FieldTypeTag,
    Self: MessageFieldGetterImpl<'msg, FD, R, LabelTag::IsRepeated, TypeTag::IsMessage>,
{
    fn try_get_field(&'msg self) -> Result<R> {
        self.try_get_field_impl()
    }
}

trait MessageFieldGetterImpl<'msg, FD, R, IsRepeated, IsMessage> {
    fn try_get_field_impl(&'msg self) -> Result<R>;
}

macro_rules! delegate_message_field_getter {
    ($ty:ty) => {
        impl<'msg, FD, InnerM, InnerMI> MessageFieldGetterImpl<'msg, FD, $ty, False, False>
            for Option<InnerM>
        where
            FD: StaticFieldDescriptor,
            InnerM: AsMessageImplRef<MessageImplType = InnerMI>,
            InnerMI: 'msg + MessageFieldGetter<'msg, FD, $ty>,
        {
            fn try_get_field_impl(&'msg self) -> Result<$ty> {
                Ok(match self {
                    Some(m) => m.as_message_impl_ref().try_get_field()?,
                    None => FD::DEFAULT_VALUE
                        .map(|d| TryFrom::try_from(d))
                        .unwrap_or(Ok(Default::default()))?,
                })
            }
        }
    };
}

delegate_message_field_getter!(u32);
delegate_message_field_getter!(u64);
delegate_message_field_getter!(i32);
delegate_message_field_getter!(i64);
delegate_message_field_getter!(f32);
delegate_message_field_getter!(f64);
delegate_message_field_getter!(bool);
delegate_message_field_getter!(&'msg str);
delegate_message_field_getter!(&'msg [u8]);
