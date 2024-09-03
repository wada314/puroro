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
use ::derive_more::Debug;
use ::replace_with::replace_with_or_abort_and_return;
use ::std::alloc::Allocator;
use ::std::alloc::Global;
use ::std::cell::OnceCell;

pub trait EnumOfDeriveds<T> {
    type Error;
    fn as_ref(&self) -> &dyn Derived<T, Error = Self::Error>;
}

pub trait EnumVariant<E>: Sized {
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

#[derive(Clone, Debug)]
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
impl<T, E, A: Allocator + Clone> BaseAndDerived<T, E, A> {
    pub fn from_derived<D: Derived<T> + EnumVariant<E>>(derived: D, alloc: A) -> Self {
        BaseAndDerived::StartFromDerived {
            derived: derived.into_enum(),
            base_cell: OnceCell::new(),
            derived_cells: OnceList::new_in(alloc),
        }
    }
}

impl<T, E, A: Allocator + Clone> BaseAndDerived<T, E, A>
where
    E: EnumOfDeriveds<T>,
{
    pub fn try_as_base(&self) -> Result<&T, E::Error> {
        match self {
            BaseAndDerived::StartFromBase { base, .. } => Ok(base),
            BaseAndDerived::StartFromDerived {
                base_cell, derived, ..
            } => base_cell.get_or_try_init(|| derived.as_ref().to_base()),
        }
    }

    pub fn try_as_derived<D: Derived<T, Error = E::Error> + EnumVariant<E>>(
        &self,
    ) -> Result<&D, E::Error> {
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

    fn push_and_get<D: Derived<T, Error = E::Error> + EnumVariant<E>>(
        &self,
        base: &T,
    ) -> Result<&D, E::Error> {
        let derived_cells = match self {
            BaseAndDerived::StartFromBase { derived_cells, .. } => derived_cells,
            BaseAndDerived::StartFromDerived { derived_cells, .. } => derived_cells,
        };
        let pushed_enum = derived_cells.push(D::from_base(base)?.into_enum());
        Ok(D::from_enum_ref(pushed_enum).unwrap())
    }

    pub fn try_as_base_mut(&mut self) -> Result<&mut T, E::Error> {
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
                *self = BaseAndDerived::from_base(base, derived_cells.allocator().clone());
                let BaseAndDerived::StartFromBase { base: base_mut, .. } = self else {
                    unreachable!();
                };
                Ok(base_mut)
            }
        }
    }

    pub fn try_as_derived_mut<D: Derived<T, Error = E::Error> + EnumVariant<E>>(
        &mut self,
    ) -> Result<&mut D, E::Error> {
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
                    derived_cells: OnceList::new_in(derived_cells.allocator().clone()),
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

#[cfg(test)]
mod tests {
    #![allow(unused)]
    use super::*;
    use ::derive_more::Debug;
    use ::std::assert_matches::assert_matches;
    use ::std::result::Result;

    #[derive(Debug)]
    struct Mul2(u32);
    #[derive(Debug)]
    struct Mul3(u32);

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
}
