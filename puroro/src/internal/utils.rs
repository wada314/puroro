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

use crate::once_list::OnceList;
use ::replace_with::replace_with_or_abort_and_return;
use ::std::alloc::Allocator;
use ::std::alloc::Global;
use ::std::any::Any;
use ::std::cell::OnceCell;
use ::std::marker::PhantomData;

pub trait Enum: Sized {
    type VariantsDynTrait: ?Sized;
    fn as_ref(&self) -> &Self::VariantsDynTrait;
    fn as_mut(&self) -> &mut Self::VariantsDynTrait;
    fn into_boxed(self) -> Box<Self::VariantsDynTrait>;
}
pub trait EnumVariant<E: Enum>: Sized {
    fn from_enum(e: E) -> Result<Self, E>;
    fn into_enum(self) -> E;
    fn from_enum_ref(e: &E) -> Option<&Self>;
    fn from_enum_mut(e: &mut E) -> Option<&mut Self>;
}

pub trait Derived<T> {
    type Error;
    fn from_base(base: &T) -> Result<Self, Self::Error>
    where
        Self: Sized;
    fn to_base(&self) -> Result<T, Self::Error>;
}

pub enum BaseAndDerived<T, E, A: Allocator = Global> {
    StartFromBase {
        base: T,
        derived_cells: OnceList<E, A>,
    },
    StartFromDerived {
        derived: E,
        base_cell: OnceCell<T>,
        derived_cells: OnceList<E, A>,
    },
}

impl<T, E, A: Allocator> BaseAndDerived<T, E, A> {
    pub fn from_base(base: T, alloc: A) -> Self {
        BaseAndDerived::StartFromBase {
            base,
            derived_cells: OnceList::new_in(alloc),
        }
    }
}
impl<T, E: Enum, A: Allocator + Clone> BaseAndDerived<T, E, A> {
    pub fn from_derived<D: Derived<T> + EnumVariant<E>>(derived: D, alloc: A) -> Self {
        BaseAndDerived::StartFromDerived {
            derived: derived.into_enum(),
            base_cell: OnceCell::new(),
            derived_cells: OnceList::new_in(alloc),
        }
    }
}

impl<T, E, Er, A: Allocator + Clone> BaseAndDerived<T, E, A>
where
    E: Enum<VariantsDynTrait = dyn Derived<T, Error = Er>>,
{
    pub fn try_as_base(&self) -> Result<&T, Er> {
        match self {
            BaseAndDerived::StartFromBase { base, .. } => Ok(base),
            BaseAndDerived::StartFromDerived {
                base_cell, derived, ..
            } => base_cell.get_or_try_init(|| derived.as_ref().to_base()),
        }
    }

    pub fn try_as_derived<D: Derived<T, Error = Er> + EnumVariant<E>>(&self) -> Result<&D, Er> {
        match self {
            BaseAndDerived::StartFromBase {
                base,
                derived_cells,
            } => {
                if let Some(d) = derived_cells.find_map(|d| D::from_enum_ref(d)) {
                    return Ok(d);
                }
                Ok(self.push_and_get(base)?)
            }
            BaseAndDerived::StartFromDerived {
                derived,
                base_cell,
                derived_cells,
            } => {
                if let Some(d) = D::from_enum_ref(derived) {
                    return Ok(d);
                }
                if let Some(d) = derived_cells.find_map(D::from_enum_ref) {
                    return Ok(d);
                }
                let base = base_cell.get_or_try_init(|| derived.as_ref().to_base())?;
                Ok(self.push_and_get(base)?)
            }
        }
    }

    fn derived_cells(&self) -> &OnceList<E, A> {
        match self {
            BaseAndDerived::StartFromBase { derived_cells, .. } => derived_cells,
            BaseAndDerived::StartFromDerived { derived_cells, .. } => derived_cells,
        }
    }

    fn push_and_get<D: Derived<T, Error = Er> + EnumVariant<E>>(&self, base: &T) -> Result<&D, Er> {
        let derived_cells = self.derived_cells();
        let pushed_enum = derived_cells.push(D::from_base(base)?.into_enum());
        Ok(D::from_enum_ref(pushed_enum).unwrap())
    }

    pub fn try_as_base_mut(&mut self) -> Result<&mut T, Er> {
        match self {
            BaseAndDerived::StartFromBase {
                base,
                derived_cells,
            } => {
                derived_cells.clear();
                Ok(base)
            }
            BaseAndDerived::StartFromDerived {
                base_cell,
                derived,
                derived_cells,
            } => {
                let base = base_cell
                    .take()
                    .map(Ok)
                    .unwrap_or_else(|| derived.as_ref().to_base())?;
                *self = BaseAndDerived::from_base(base, derived_cells.alloc().clone());
                let BaseAndDerived::StartFromBase { base: base_mut, .. } = self else {
                    unreachable!();
                };
                Ok(base_mut)
            }
        }
    }

    pub fn try_as_derived_mut<D: Derived<T, Error = Er> + EnumVariant<E>>(
        &mut self,
    ) -> Result<&mut D, Er> {
        match self {
            BaseAndDerived::StartFromBase {
                base,
                derived_cells,
            } => {
                let new_derived = derived_cells
                    .take_map(|d| D::from_enum(d))
                    .map(Ok)
                    .unwrap_or_else(|| Ok(D::from_base(base)?))?;
                *self = BaseAndDerived::StartFromDerived {
                    derived: new_derived.into_enum(),
                    base_cell: OnceCell::new(),
                    derived_cells: OnceList::new_in(derived_cells.alloc().clone()),
                };
                if let BaseAndDerived::StartFromDerived { derived, .. } = self {
                    Ok(D::from_enum_mut(derived).unwrap())
                } else {
                    unreachable!();
                }
            }
            BaseAndDerived::StartFromDerived {
                base_cell,
                derived,
                derived_cells,
            } => {
                replace_with_or_abort_and_return(derived, |derived_taken| {
                    match D::from_enum(derived_taken) {
                        Ok(d) => return (Ok(()), d.into_enum()),
                        Err(derived_taken) => {
                            let base = match base_cell
                                .get_or_try_init(|| derived_taken.as_ref().to_base())
                            {
                                Ok(base) => base,
                                Err(e) => return (Err(e), derived_taken),
                            };
                            let new_derived = match D::from_base(&base) {
                                Ok(d) => d,
                                Err(e) => return (Err(e), derived_taken),
                            };
                            (Ok(()), new_derived.into_enum())
                        }
                    }
                })?;
                base_cell.take();
                derived_cells.clear();
                Ok(D::from_enum_mut(derived).unwrap())
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum InterconvertiblePair<T, U, I> {
    Left(T, OnceCell<U>, PhantomData<I>),
    Right(U, OnceCell<T>, PhantomData<I>),
    Both(T, U, PhantomData<I>),
}

pub trait Interconverter<L, R> {
    type Error;
    type Context;
    fn try_into_left(right: &R) -> Result<L, Self::Error>
    where
        Self: Interconverter<L, R, Context = ()>,
    {
        Self::try_into_left_with_context(right, &())
    }
    fn try_into_right(left: &L) -> Result<R, Self::Error>
    where
        Self: Interconverter<L, R, Context = ()>,
    {
        Self::try_into_right_with_context(left, &())
    }
    fn try_into_left_with_context(right: &R, context: &Self::Context) -> Result<L, Self::Error>;
    fn try_into_right_with_context(left: &L, context: &Self::Context) -> Result<R, Self::Error>;
}

impl<T, U, I> InterconvertiblePair<T, U, I>
where
    I: Interconverter<T, U>,
{
    pub fn from_left(left: T) -> Self {
        InterconvertiblePair::Left(left, OnceCell::new(), PhantomData)
    }
    pub fn from_right(right: U) -> Self {
        InterconvertiblePair::Right(right, OnceCell::new(), PhantomData)
    }
    pub fn from_both(left: T, right: U) -> Self {
        InterconvertiblePair::Both(left, right, PhantomData)
    }

    pub fn try_left_with_context(&self, context: &I::Context) -> Result<&T, I::Error> {
        match self {
            InterconvertiblePair::Left(left, _, _) => Ok(left),
            InterconvertiblePair::Right(right, left_cell, _) => {
                left_cell.get_or_try_init(|| I::try_into_left_with_context(right, context))
            }
            InterconvertiblePair::Both(left, _, _) => Ok(left),
        }
    }
    pub fn try_right_with_context(&self, context: &I::Context) -> Result<&U, I::Error> {
        match self {
            InterconvertiblePair::Left(left, right_cell, _) => {
                right_cell.get_or_try_init(|| I::try_into_right_with_context(left, context))
            }
            InterconvertiblePair::Right(right, _, _) => Ok(right),
            InterconvertiblePair::Both(_, right, _) => Ok(right),
        }
    }

    pub fn try_into_left_with_context(self, context: &I::Context) -> Result<T, (I::Error, U)> {
        Ok(match self {
            InterconvertiblePair::Left(left, _, _) => left,
            InterconvertiblePair::Right(right, mut left_cell, _) => match left_cell.take() {
                Some(left) => left,
                None => I::try_into_left_with_context(&right, context).map_err(|e| (e, right))?,
            },
            InterconvertiblePair::Both(left, _, _) => left,
        })
    }
    pub fn try_into_right_with_context(self, context: &I::Context) -> Result<U, (I::Error, T)> {
        Ok(match self {
            InterconvertiblePair::Left(left, mut right_cell, _) => match right_cell.take() {
                Some(right) => right,
                None => I::try_into_right_with_context(&left, context).map_err(|e| (e, left))?,
            },
            InterconvertiblePair::Right(right, _, _) => right,
            InterconvertiblePair::Both(_, right, _) => right,
        })
    }

    pub fn try_left_mut_with_context(&mut self, context: &I::Context) -> Result<&mut T, I::Error> {
        unsafe {
            let taken_self = ::std::ptr::read(self);
            let (new_self, result) = match taken_self.try_into_left_with_context(context) {
                Ok(left) => (
                    InterconvertiblePair::Left(left, OnceCell::new(), PhantomData),
                    Ok(()),
                ),
                Err((e, right)) => (
                    InterconvertiblePair::Right(right, OnceCell::new(), PhantomData),
                    Err(e),
                ),
            };
            ::std::ptr::write(self, new_self);
            result
        }?;
        let InterconvertiblePair::Left(left, _, _) = self else {
            unreachable!();
        };
        Ok(left)
    }
    pub fn try_right_mut_with_context(&mut self, context: &I::Context) -> Result<&mut U, I::Error> {
        unsafe {
            let taken_self = ::std::ptr::read(self);
            let (new_self, result) = match taken_self.try_into_right_with_context(context) {
                Ok(right) => (
                    InterconvertiblePair::Right(right, OnceCell::new(), PhantomData),
                    Ok(()),
                ),
                Err((e, left)) => (
                    InterconvertiblePair::Left(left, OnceCell::new(), PhantomData),
                    Err(e),
                ),
            };
            ::std::ptr::write(self, new_self);
            result
        }?;
        let InterconvertiblePair::Right(right, _, _) = self else {
            unreachable!();
        };
        Ok(right)
    }
}

impl<T, U, I> InterconvertiblePair<T, U, I>
where
    I: Interconverter<T, U, Context = ()>,
{
    pub fn try_left(&self) -> Result<&T, I::Error> {
        self.try_left_with_context(&())
    }
    pub fn try_right(&self) -> Result<&U, I::Error> {
        self.try_right_with_context(&())
    }
    pub fn try_into_left(self) -> Result<T, (I::Error, U)> {
        self.try_into_left_with_context(&())
    }
    pub fn try_into_right(self) -> Result<U, (I::Error, T)> {
        self.try_into_right_with_context(&())
    }
    pub fn try_left_mut(&mut self) -> Result<&mut T, I::Error> {
        self.try_left_mut_with_context(&())
    }
    pub fn try_right_mut(&mut self) -> Result<&mut U, I::Error> {
        self.try_right_mut_with_context(&())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::std::assert_matches::assert_matches;
    use ::std::result::Result;

    #[derive(Debug)]
    struct Mul2(u32);
    #[derive(Debug)]
    struct Mul3(u32);

    struct InterconvertMul2Mul3;
    impl Interconverter<Mul2, Mul3> for InterconvertMul2Mul3 {
        type Error = u32;
        type Context = ();
        fn try_into_left_with_context(right: &Mul3, _context: &()) -> Result<Mul2, u32> {
            if right.0 % 2 == 0 {
                Ok(Mul2(right.0))
            } else {
                Err(right.0)
            }
        }
        fn try_into_right_with_context(left: &Mul2, _context: &()) -> Result<Mul3, u32> {
            if left.0 % 3 == 0 {
                Ok(Mul3(left.0))
            } else {
                Err(left.0)
            }
        }
    }
    type Mul2Mul3Pair = InterconvertiblePair<Mul2, Mul3, InterconvertMul2Mul3>;

    impl PartialEq<u32> for Mul2 {
        fn eq(&self, other: &u32) -> bool {
            self.0 == *other
        }
    }
    impl PartialEq<u32> for Mul3 {
        fn eq(&self, other: &u32) -> bool {
            self.0 == *other
        }
    }

    #[test]
    fn test_from_left_ok_right() {
        let eob = Mul2Mul3Pair::from_left(Mul2(6));
        assert_matches!(eob.try_left(), Ok(&Mul2(6)));
        assert_matches!(eob.try_right(), Ok(&Mul3(6)));
    }

    #[test]
    fn test_from_right_ok_left() {
        let eob = Mul2Mul3Pair::from_right(Mul3(6));
        assert_matches!(eob.try_left(), Ok(&Mul2(6)));
        assert_matches!(eob.try_right(), Ok(&Mul3(6)));
    }

    #[test]
    fn test_from_both() {
        let eob = Mul2Mul3Pair::from_both(Mul2(6), Mul3(6));
        assert_matches!(eob.try_left(), Ok(&Mul2(6)));
        assert_matches!(eob.try_right(), Ok(&Mul3(6)));
    }

    #[test]
    fn test_from_left_err_right() {
        let eob = Mul2Mul3Pair::from_left(Mul2(4));
        assert_matches!(eob.try_left(), Ok(&Mul2(4)));
        assert_matches!(eob.try_right(), Err(4));
    }

    #[test]
    fn test_from_right_err_left() {
        let eob = Mul2Mul3Pair::from_right(Mul3(9));
        assert_matches!(eob.try_left(), Err(9));
        assert_matches!(eob.try_right(), Ok(&Mul3(9)));
    }

    #[test]
    fn test_mut_left() {
        let mut eob = Mul2Mul3Pair::from_right(Mul3(6));
        *(eob.try_left_mut().unwrap()) = Mul2(12);
        assert_matches!(eob.try_left(), Ok(&Mul2(12)));
        assert_matches!(eob.try_right(), Ok(&Mul3(12)));
    }

    #[test]
    fn test_mut_right() {
        let mut eob = Mul2Mul3Pair::from_left(Mul2(6));
        *(eob.try_right_mut().unwrap()) = Mul3(12);
        assert_matches!(eob.try_right(), Ok(&Mul3(12)));
        assert_matches!(eob.try_left(), Ok(&Mul2(12)));
    }
}
