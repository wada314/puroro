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

#![feature(once_cell_try)]
#![feature(error_generic_member_access)]
#![feature(assert_matches)]

pub mod cases;
pub mod descriptor;
pub mod generator;
pub mod proto_path;

pub use crate::generator::compile;
use ::puroro::dynamic::DynamicMessage;
use ::puroro::google::protobuf::compiler::CodeGeneratorRequest;
use ::puroro::message::{Message, MessageMut};
use ::std::backtrace::Backtrace;
use ::thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorKind {
    #[error("Compile error: {0}\n{1}")]
    CompileError(String, Backtrace),
    #[error("::puroro error: {0}\n{1}")]
    PuroroError(#[from] ::puroro::ErrorKind, Backtrace),
    #[error("::std::num::TryFromIntError: {0}\n{1}")]
    StdTryFromIntError(#[from] ::std::num::TryFromIntError, Backtrace),
    #[error("::syn error: {0}\n{1}")]
    SynParseError(#[from] ::syn::Error, Backtrace),
}

impl From<String> for ErrorKind {
    fn from(s: String) -> Self {
        ErrorKind::CompileError(s, Backtrace::capture())
    }
}
pub type Result<T> = ::std::result::Result<T, ErrorKind>;

pub fn compile_binary(input: impl AsRef<[u8]>) -> Result<Vec<u8>> {
    let request: CodeGeneratorRequest = DynamicMessage::deser_from_read(input.as_ref())
        .unwrap()
        .into();
    let response = compile(&request)?;
    let mut output_buffer = Vec::new();
    response.write(&mut output_buffer)?;
    Ok(output_buffer)
}

// ```protobuf
// message Person {
//     string name = 1;
//     uint32 age = 2;
//     repeated Person children = 3;
// }
// ```

#[repr(transparent)]
pub struct Person<T = PersonInner>(T);
pub struct PersonInner {
    name: String,
    age: u32,
    children: Vec<PersonInner>,
}
pub struct Field<T>(T, i32);

pub trait ScalarStringField<const N: i32> {
    fn get(&self) -> &str;
}
pub trait ScalarU32Field<const N: i32> {
    fn get(&self) -> u32;
}
pub trait RepeatedMessageField<const N: i32> {
    type Message;
    fn get(&self) -> impl Iterator<Item = &Self::Message>;
}

impl ScalarStringField<1> for PersonInner {
    fn get(&self) -> &str {
        &self.name
    }
}
impl ScalarU32Field<2> for PersonInner {
    fn get(&self) -> u32 {
        self.age
    }
}
impl RepeatedMessageField<3> for PersonInner {
    type Message = PersonInner;
    fn get(&self) -> impl Iterator<Item = &Self::Message> {
        self.children.iter()
    }
}

impl<T, const N: i32> ScalarStringField<N> for &T
where
    T: ScalarStringField<N>,
{
    fn get(&self) -> &str {
        <T as ScalarStringField<N>>::get(self)
    }
}

impl<T, const N: i32> ScalarU32Field<N> for &T
where
    T: ScalarU32Field<N>,
{
    fn get(&self) -> u32 {
        <T as ScalarU32Field<N>>::get(self)
    }
}

impl<T, const N: i32> RepeatedMessageField<N> for &T
where
    T: RepeatedMessageField<N>,
{
    type Message = <T as RepeatedMessageField<N>>::Message;
    fn get(&self) -> impl Iterator<Item = &Self::Message> {
        <T as RepeatedMessageField<N>>::get(self)
    }
}

pub trait PersonTrait: ScalarStringField<1> + ScalarU32Field<2> + RepeatedMessageField<3> {
    fn name(&self) -> &str {
        <Self as ScalarStringField<1>>::get(self)
    }
    fn age(&self) -> u32 {
        <Self as ScalarU32Field<2>>::get(self)
    }
    fn children(&self) -> impl Iterator<Item = &<Self as RepeatedMessageField<3>>::Message> {
        <Self as RepeatedMessageField<3>>::get(self)
    }
}
impl<T> PersonTrait for T
where
    T: ScalarStringField<1>,
    T: ScalarU32Field<2>,
    T: RepeatedMessageField<3>,
{
}

pub trait PersonTrait2: PersonTrait {
    fn children2(&self) -> impl Iterator<Item = &impl PersonTrait2>;
}
impl PersonTrait2 for PersonInner {
    fn children2(&self) -> impl Iterator<Item = &impl PersonTrait2> {
        <Self as PersonTrait>::children(self)
    }
}
impl<T: PersonTrait2> PersonTrait2 for &T {
    fn children2(&self) -> impl Iterator<Item = &impl PersonTrait2> {
        <T as PersonTrait2>::children2(self)
    }
}

impl<T> Person<T>
where
    T: PersonTrait2,
{
    pub fn name(&self) -> &str {
        self.0.name()
    }
    pub fn age(&self) -> u32 {
        self.0.age()
    }
    pub fn children(&self) -> impl Iterator<Item = Person<impl PersonTrait2>> {
        self.0.children2().map(|x| Person(x))
    }
}

fn foo<T: PersonTrait2>(p: Person<T>) {
    println!("{}", p.name());
    println!("{}", p.age());
    for child in p.children() {
        println!("{}", child.name());
        for grandchild in child.children() {
            println!("{}", grandchild.name());
        }
    }
}
