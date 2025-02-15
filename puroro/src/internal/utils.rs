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

use crate::{ErrorKind, Result};
use ::cached_pair::{Converter, Pair, StdConverter};
use ::once_list2::OnceList;
use ::std::alloc::Allocator;
use ::std::fmt::Debug;
use ::std::iter;
use ::std::rc::Rc;

#[derive(Clone)]
pub struct OnceList1<T, A: Allocator>(T, OnceList<T, A>);
impl<T, A: Allocator> OnceList1<T, A> {
    pub fn new_in(first: T, alloc: A) -> Self {
        Self(first, OnceList::new_in(alloc))
    }
    pub fn first(&self) -> &T {
        &self.0
    }
    pub fn last(&self) -> &T {
        match self.1.last() {
            Some(last) => last,
            None => self.first(),
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        iter::once(&self.0).chain(self.1.iter())
    }
    pub fn into_iter(self) -> impl Iterator<Item = T> {
        iter::once(self.0).chain(self.1.into_iter())
    }
    pub fn allocator(&self) -> &A {
        self.1.allocator()
    }
}
impl<T, A: Allocator + Clone> OnceList1<T, A> {
    pub fn extend(&self, other: impl IntoIterator<Item = T>) {
        self.1.extend(other);
    }
}
impl<T, A: Allocator + Clone> OnceList1<T, A> {
    pub fn push(&self, value: T) -> &T {
        self.1.push(value)
    }
}

impl<T: Debug, A: Allocator> Debug for OnceList1<T, A> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.debug_list()
            .entry(&self.0)
            .entries(self.1.iter())
            .finish()
    }
}

pub(crate) struct WithAllocator<T, A>(pub(crate) T, pub(crate) A);

pub(crate) type PairWithOnceList1<L, R, A, C = StdConverter> =
    Pair<L, OnceList1<R, A>, ConverterForOnceList1<C, A>>;

pub(crate) struct BoxedFnConverterWithContext<C, L, R, EL, ER> {
    to_left: Rc<dyn Fn(&R, &C) -> ::std::result::Result<L, EL>>,
    to_right: Rc<dyn Fn(&L, &C) -> ::std::result::Result<R, ER>>,
    context: C,
}
pub(crate) fn boxed_fn_converter_with_context<C, L, R, EL, ER>(
    context: C,
    to_left: impl Fn(&R, &C) -> ::std::result::Result<L, EL> + 'static,
    to_right: impl Fn(&L, &C) -> ::std::result::Result<R, ER> + 'static,
) -> BoxedFnConverterWithContext<C, L, R, EL, ER> {
    BoxedFnConverterWithContext {
        to_left: Rc::new(to_left),
        to_right: Rc::new(to_right),
        context,
    }
}
impl<C, L, R, EL, ER> Converter<L, R> for BoxedFnConverterWithContext<C, L, R, EL, ER> {
    type ToLeftError = EL;
    type ToRightError = ER;

    fn convert_to_left(&self, right: &R) -> ::std::result::Result<L, Self::ToLeftError> {
        (self.to_left)(right, &self.context)
    }

    fn convert_to_right(&self, left: &L) -> ::std::result::Result<R, Self::ToRightError> {
        (self.to_right)(left, &self.context)
    }
}
impl<C, L, R, EL, ER> BoxedFnConverterWithContext<C, L, R, EL, ER> {
    pub(crate) fn context(&self) -> &C {
        &self.context
    }
    pub(crate) fn context_mut(&mut self) -> &mut C {
        &mut self.context
    }
}
impl<C: Clone, L, R, EL, ER> Clone for BoxedFnConverterWithContext<C, L, R, EL, ER> {
    fn clone(&self) -> Self {
        Self {
            to_left: self.to_left.clone(),
            to_right: self.to_right.clone(),
            context: self.context.clone(),
        }
    }
}
impl<C: Debug, L, R, EL, ER> Debug for BoxedFnConverterWithContext<C, L, R, EL, ER> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.debug_struct("BoxedFnConverterWithContext")
            .field("to_left", &"<closure>")
            .field("to_right", &"<closure>")
            .field("context", &self.context)
            .finish()
    }
}

#[derive(Clone)]
pub(crate) struct ConverterForOnceList1<C, A>(C, A);
impl<C, A> ConverterForOnceList1<C, A> {
    pub(crate) fn new_in(converter: C, alloc: A) -> Self {
        Self(converter, alloc)
    }
    pub(crate) fn inner(&self) -> &C {
        &self.0
    }
    pub(crate) fn inner_mut(&mut self) -> &mut C {
        &mut self.0
    }
    pub(crate) fn allocator(&self) -> &A {
        &self.1
    }
}
impl<L, R, A, C> Converter<L, OnceList1<R, A>> for ConverterForOnceList1<C, A>
where
    C: Converter<L, R>,
    A: Allocator + Clone,
{
    type ToLeftError = C::ToLeftError;
    type ToRightError = C::ToRightError;

    fn convert_to_left(
        &self,
        right: &OnceList1<R, A>,
    ) -> ::std::result::Result<L, Self::ToLeftError> {
        self.0.convert_to_left(right.first())
    }

    fn convert_to_right(
        &self,
        left: &L,
    ) -> ::std::result::Result<OnceList1<R, A>, Self::ToRightError> {
        Ok(OnceList1::new_in(
            self.0.convert_to_right(left)?,
            self.1.clone(),
        ))
    }
}

pub(crate) trait PairWithOnceList1Ext<L, R, A, C> {
    fn try_get_or_insert_into_right(&self, pred: impl Fn(&R) -> bool) -> Result<&R>;
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
    fn test_once_list1_basic() {
        let list = OnceList1::new_in(1, Global);
        assert_eq!(*list.first(), 1);

        let items: Vec<_> = list.iter().copied().collect();
        assert_eq!(items, vec![1]);
    }

    #[test]
    fn test_once_list1_push() {
        let list = OnceList1::new_in(1, Global);
        list.push(2);
        list.push(3);

        let items: Vec<_> = list.iter().copied().collect();
        assert_eq!(items, vec![1, 2, 3]);
    }

    #[test]
    fn test_once_list1_debug() {
        let list = OnceList1::new_in(1, Global);
        list.push(2);

        assert_eq!(format!("{:?}", list), "[1, 2]");
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
