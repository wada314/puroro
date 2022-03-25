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

//////////////////////////////////////////////////////////////

use crate::internal::Bitfield;
use crate::tags::FieldTypeTag;
use crate::{ErrorKind, PuroroError, Result};
use ::once_cell::sync::Lazy;
use ::std::marker::PhantomData;
use ::std::ops::Deref;

struct Nil;
struct Cons<T, U>(T, U);
struct Field<T, FD>(T, PhantomData<FD>);

trait TryFromField<MD, FD, F>: Sized {
    fn try_from_field(_field: &F) -> Result<Self> {
        Err(ErrorKind::ReflectionError)?
    }
}
trait TryFromFieldOpt<MD, FD, F>: Sized {
    fn try_from_field_opt(_field: &F) -> Result<Option<Self>> {
        Err(ErrorKind::ReflectionError)?
    }
}
impl<MD, FD, F> TryFromField<MD, FD, F> for u32
where
    FD: StaticFieldDescriptor,
    u32: TryFromFieldOpt<MD, FD, F>,
{
    fn try_from_field(field: &F) -> Result<Self> {
        match Self::try_from_field_opt(field)? {
            Some(val) => Ok(val),
            None => match FD::DEFAULT_VALUE {
                FieldDefaultValue::U32(val) => Ok(val),
                _ => Err(ErrorKind::ReflectionError)?,
            },
        }
    }
}
impl<MD, FD> TryFromFieldOpt<MD, FD, u32> for u32 {
    fn try_from_field_opt(field: &u32) -> Result<Option<Self>> {
        Ok(Some(*field))
    }
}

trait GetFieldByNumber<MD, R> {
    fn field_by_number(&self, _: i32) -> Result<R>;
}
impl<MD, FD, R, F, U> GetFieldByNumber<MD, R> for Cons<Field<F, FD>, U>
where
    R: TryFromField<MD, FD, F>,
    U: GetFieldByNumber<MD, R>,
    FD: StaticFieldDescriptor,
{
    fn field_by_number(&self, number: i32) -> Result<R> {
        if FD::NUMBER == number {
            R::try_from_field(&self.0.0)
        } else {
            self.1.field_by_number(number)
        }
    }
}
impl<MD, R> GetFieldByNumber<MD, R> for Nil {
    fn field_by_number(&self, _: i32) -> Result<R> {
        Err(ErrorKind::ReflectionError)?
    }
}

pub trait StaticMessageDescriptor {
    /// Only existing field implements `StaticFieldDescriptor`
    type Fields<const NUMBER: i32>;
}
pub trait StaticFieldDescriptor {
    const NUMBER: i32;
    const DEFAULT_VALUE: FieldDefaultValue;
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum FieldTypeEnum {
    Int32,
    String,
    Message(&'static MessageDescriptor),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum FieldLabelEnum {
    Optional,
    Required,
    Repeated,
    Unlabeled,
}

#[derive(Debug, PartialEq)]
pub struct MessageDescriptor {
    fields: &'static [FieldDescriptor],
}

impl<'a> MessageDescriptor {
    pub fn field_by_number(&self, number: i32) -> Result<&FieldDescriptor> {
        debug_assert!(self.fields.is_sorted_by_key(|f| f.number));
        Ok(self
            .fields
            .binary_search_by_key(&number, |f| f.number)
            .map(|index| &self.fields[index])
            .map_err(|_| ErrorKind::ReflectionError)?)
    }
}

#[derive(Debug, PartialEq)]
pub enum FieldDefaultValue {
    None,
    U32(u32),
    String(&'static str),
}

#[derive(Debug, PartialEq)]
pub struct FieldDescriptor {
    number: i32,
    default_value: FieldDefaultValue,
    field_type: FieldTypeEnum,
    field_label: FieldLabelEnum,
}

impl FieldDescriptor {
    pub fn number(&self) -> i32 {
        self.number
    }
    pub fn field_type(&self) -> FieldTypeEnum {
        self.field_type
    }
    pub fn field_label(&self) -> FieldLabelEnum {
        self.field_label
    }
    pub fn default_value_u32(&self) -> Result<u32> {
        match &self.default_value {
            FieldDefaultValue::U32(v) => Ok(*v),
            _ => Err(ErrorKind::ReflectionError)?,
        }
    }
    pub fn default_value_str(&self) -> Result<&'static str> {
        match &self.default_value {
            FieldDefaultValue::String(v) => Ok(*v),
            _ => Err(ErrorKind::ReflectionError)?,
        }
    }
}

pub trait Message {
    fn try_get_u32<'a>(&'a self, _: &'a FieldDescriptor) -> Result<u32> {
        Err(ErrorKind::ReflectionError)?
    }
    fn try_get_repeated_u32_boxed<'a>(
        &'a self,
        _: &'a FieldDescriptor,
    ) -> Result<Box<dyn 'a + Iterator<Item = u32>>> {
        Err(ErrorKind::ReflectionError)?
    }
    fn try_get_str<'a>(&'a self, _: &'a FieldDescriptor) -> Result<&'a str> {
        Err(ErrorKind::ReflectionError)?
    }
    fn try_get_message<'a>(&'a self, _: &'a FieldDescriptor) -> Result<&'a dyn Message> {
        Err(ErrorKind::ReflectionError)?
    }
}

pub trait MessageMut {}

/// assume a proto like this as input:
/// message Person {
///     optional string name = 1;
///     optional uint32 age = 2;
///     optional Person partner = 4;
///     repeated string nicknames = 5;
///     repeated uint32 scores = 6;
///     repeated Person children = 3;
/// }
#[derive(Default)]
pub struct Person {
    _bitvec: ::bitvec::BitArr!(for 2),
    name: String,
    age: u32,
    partner: Option<Box<Person>>,
}

static PERSON_DEFAULT_INSTANCE: Lazy<Person> = Lazy::new(Default::default);

impl Message for Person {
    fn try_get_u32<'a>(&'a self, fd: &'a FieldDescriptor) -> Result<u32> {
        Ok(match fd.number {
            2 => {
                if self._bitvec.get(0) {
                    self.age
                } else {
                    fd.default_value_u32()?
                }
            }
            _ => Err(ErrorKind::ReflectionError)?,
        })
    }

    fn try_get_str<'a>(&'a self, fd: &'a FieldDescriptor) -> Result<&'a str> {
        Ok(match fd.number {
            1 => {
                if self._bitvec.get(0) {
                    &self.name
                } else {
                    &fd.default_value_str()?
                }
            }
            _ => Err(ErrorKind::ReflectionError)?,
        })
    }

    fn try_get_message<'a>(&'a self, fd: &'a FieldDescriptor) -> Result<&'a dyn Message> {
        Ok(match fd.number {
            4 => match &self.partner {
                Some(boxed) => AsRef::as_ref(boxed) as &dyn Message,
                None => Lazy::force(&PERSON_DEFAULT_INSTANCE) as &dyn Message,
            },
            _ => Err(ErrorKind::ReflectionError)?,
        })
    }
}

struct PersonStaticMessageDescriptor;
struct PersonStaticFieldDescriptor<const NUMBER: i32>;

impl StaticMessageDescriptor for PersonStaticMessageDescriptor {
    type Fields<const NUMBER: i32> = PersonStaticFieldDescriptor<NUMBER>;
}
impl StaticFieldDescriptor for PersonStaticFieldDescriptor<1> {
    const NUMBER: i32 = 1;
    const DEFAULT_VALUE: FieldDefaultValue = FieldDefaultValue::String("John Doe");
}
impl StaticFieldDescriptor for PersonStaticFieldDescriptor<2> {
    const NUMBER: i32 = 2;
    const DEFAULT_VALUE: FieldDefaultValue = FieldDefaultValue::U32(14);
}
impl StaticFieldDescriptor for PersonStaticFieldDescriptor<3> {
    const NUMBER: i32 = 3;
    const DEFAULT_VALUE: FieldDefaultValue = FieldDefaultValue::None;
}
