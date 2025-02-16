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

mod converter;
mod once_list1;

use crate::{ErrorKind, Result};
use ::cached_pair::{Converter, Pair, StdConverter};
use ::std::alloc::Allocator;

pub(crate) use converter::{
    boxed_fn_converter_with_context, BoxedFnConverterWithContext, ConverterForOnceList1,
};
pub(crate) use once_list1::OnceList1;

pub(crate) type PairWithOnceList1<L, R, A, C = StdConverter> =
    Pair<L, OnceList1<R, A>, ConverterForOnceList1<C, A>>;

pub(crate) trait PairWithOnceList1Ext<L, R, A, C> {
    fn try_get_or_insert_into_right(&self, pred: impl Fn(&R) -> bool) -> Result<&R>;
    fn try_get_or_insert_into_right_mut(&mut self, pred: impl Fn(&R) -> bool) -> Result<&mut R>;
}

impl<L, R, A, C> PairWithOnceList1Ext<L, R, A, C>
    for Pair<L, OnceList1<R, A>, ConverterForOnceList1<C, A>>
where
    A: Allocator + Clone,
    C: Converter<L, R>,
    ErrorKind: From<C::ToLeftError> + From<C::ToRightError>,
{
    fn try_get_or_insert_into_right(&self, pred: impl Fn(&R) -> bool) -> Result<&R> {
        // First try to find in existing list if available
        if let Some(list) = self.right_opt() {
            if let Some(item) = list.iter().find(|x| pred(*x)) {
                return Ok(item);
            }

            // The value not exists, but the list exists.
            list.push(self.converter().0.convert_to_right(self.try_left()?)?);
        } else {
            // The value not exists, and the list not exists.
            // No need to do anything, because the list will be generated in the next step.
        }
        Ok(self.try_right()?.last())
    }

    fn try_get_or_insert_into_right_mut(&mut self, pred: impl Fn(&R) -> bool) -> Result<&mut R> {
        'unify_or_clear_list: {
            if let Some(list1) = self.right_opt_mut() {
                if pred(&list1.0) {
                    list1.1.clear();
                    break 'unify_or_clear_list;
                } else if let Some(item) = list1.1.remove(pred) {
                    *list1 = OnceList1::new_in(item, list1.1.allocator().clone());
                    break 'unify_or_clear_list;
                }
            }
            let _ = self.try_extract_right()?;
        }
        // The right side is either empty or contains only the target value.
        // In both cases, try_extract_left() will work.
        let _ = self.try_extract_left()?;
        Ok(&mut self.try_right_mut()?.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::std::alloc::Global;

    #[derive(::derive_more::TryInto, ::derive_more::From, PartialEq, Debug, Clone)]
    #[try_into(owned, ref)]
    enum Int32Compatible {
        String(String),
        Array([u8; 4]),
    }

    impl TryFrom<&Int32Compatible> for i32 {
        type Error = ErrorKind;

        fn try_from(value: &Int32Compatible) -> Result<Self> {
            match value {
                Int32Compatible::String(s) => s
                    .parse()
                    .map_err(|_| "Invalid number format".to_string().into()),
                Int32Compatible::Array(a) => Ok(i32::from_le_bytes(*a)),
            }
        }
    }

    impl From<i32> for Int32Compatible {
        fn from(value: i32) -> Self {
            Int32Compatible::Array(value.to_le_bytes())
        }
    }

    #[derive(Default)]
    struct TestConverter;

    impl Converter<i32, Int32Compatible> for TestConverter {
        type ToLeftError = ErrorKind;
        type ToRightError = ErrorKind;

        fn convert_to_left(
            &self,
            right: &Int32Compatible,
        ) -> ::std::result::Result<i32, Self::ToLeftError> {
            right.try_into()
        }

        fn convert_to_right(
            &self,
            left: &i32,
        ) -> ::std::result::Result<Int32Compatible, Self::ToRightError> {
            Ok((*left).into())
        }
    }

    #[test]
    fn test_pair_with_once_list1_ext_find_existing() -> Result<()> {
        let list = OnceList1::new_in(Int32Compatible::String("42".to_string()), Global);
        list.push(Int32Compatible::Array(123i32.to_le_bytes()));
        let pair =
            Pair::from_right_conv(list, ConverterForOnceList1::new_in(TestConverter, Global));

        let result: &String = pair
            .try_get_or_insert_into_right(|n| TryInto::<&String>::try_into(n).is_ok())?
            .try_into()
            .unwrap();
        assert_eq!(*result, "42".to_string());
        Ok(())
    }

    #[test]
    fn test_pair_with_once_list1_ext_create_list_from_left() -> Result<()> {
        let pair = Pair::from_left_conv(42, ConverterForOnceList1::new_in(TestConverter, Global));

        let result: &String = pair
            .try_get_or_insert_into_right(|n| TryInto::<&String>::try_into(n).is_ok())?
            .try_into()
            .unwrap();
        assert_eq!(*result, "42".to_string());
        Ok(())
    }

    #[test]
    fn test_pair_with_once_list1_ext_push_new_value() -> Result<()> {
        let list = OnceList1::new_in(Int32Compatible::Array(42i32.to_le_bytes()), Global);
        let pair =
            Pair::from_right_conv(list, ConverterForOnceList1::new_in(TestConverter, Global));

        let result: &String = pair
            .try_get_or_insert_into_right(|n| TryInto::<&String>::try_into(n).is_ok())?
            .try_into()
            .unwrap();
        assert_eq!(*result, "42".to_string());
        Ok(())
    }

    #[test]
    fn test_pair_with_once_list1_ext_both_sides_present_but_no_string() -> Result<()> {
        let list = OnceList1::new_in(Int32Compatible::Array(42i32.to_le_bytes()), Global);
        let pair =
            Pair::from_right_conv(list, ConverterForOnceList1::new_in(TestConverter, Global));
        let _ = pair.try_left();

        let result: &String = pair
            .try_get_or_insert_into_right(|n| TryInto::<&String>::try_into(n).is_ok())?
            .try_into()
            .unwrap();
        assert_eq!(*result, "42".to_string());
        Ok(())
    }
}
