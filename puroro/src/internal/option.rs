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
use crate::message::{AsMessageImplRef, MessageScalarFieldGetter};
use crate::tags;
use crate::Result;

impl<'msg, FD, InnerM, ReturnType> MessageScalarFieldGetter<'msg, FD> for Option<InnerM>
where
    FD: StaticFieldDescriptor,
    InnerM: MessageScalarFieldGetter<'msg, FD, ReturnType = ReturnType>,
{
    type ReturnType = ReturnType;
    fn try_get_opt_field(&'msg self) -> Result<Option<Self::ReturnType>> {
        Ok(self.map(|m| m.try_get_field()).transpose()?)
    }
}

trait MessageFieldGetterImpl<'msg, FD, TypeTag, IsRepeated, IsMessage> {
    type ReturnTypeImpl;
    fn try_get_field_impl(&'msg self) -> Result<Self::ReturnTypeImpl>;
}

macro_rules! delegate_message_field_getter {
    ($ty:ty, $tag:ty) => {
        impl<'msg, FD, InnerM, InnerMI> MessageFieldGetterImpl<'msg, FD, $tag, False, False>
            for Option<InnerM>
        where
            FD: StaticFieldDescriptor,
            InnerM: AsMessageImplRef<MessageImplType = InnerMI>,
            InnerMI: 'msg + MessageScalarFieldGetter<'msg, FD, ReturnType = $ty>,
        {
            type ReturnTypeImpl = $ty;
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

delegate_message_field_getter!(u32, tags::UInt32);
delegate_message_field_getter!(u64, tags::UInt64);
delegate_message_field_getter!(i32, tags::Int32);
delegate_message_field_getter!(i64, tags::Int64);
delegate_message_field_getter!(f32, tags::Float);
delegate_message_field_getter!(f64, tags::Double);
delegate_message_field_getter!(bool, tags::Bool);
delegate_message_field_getter!(&'msg str, tags::String);
delegate_message_field_getter!(&'msg [u8], tags::Bytes);

impl<'msg, FD, FieldMD, OptM, OptMI, FieldM> MessageFieldGetterImpl<'msg, FD, FieldMD, False, True>
    for Option<OptM>
where
    FD: StaticFieldDescriptor,
    OptM: AsMessageImplRef<MessageImplType = OptMI>,
    OptMI: 'msg + MessageScalarFieldGetter<'msg, FD, ReturnType = FieldM>,
    FieldM: 'msg,
{
    type ReturnTypeImpl = Option<FieldM>;
    fn try_get_field_impl(&'msg self) -> Result<Option<FieldM>> {
        Ok(self
            .as_ref()
            .map(|inner_m| inner_m.as_message_impl_ref().try_get_field())
            .transpose()?)
    }
}
