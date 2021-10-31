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

pub mod to_io_write;
pub use to_io_write::SerFieldToIoWrite;

use crate::{Either, Result};
use ::std::io::Write;

pub trait SerMessageToIoWrite {
    fn ser<W>(&self, out: &mut W) -> Result<()>
    where
        W: Write;
}
impl SerMessageToIoWrite for () {
    fn ser<W>(&self, _: &mut W) -> Result<()>
    where
        W: Write,
    {
        Ok(())
    }
}
impl<T, U> SerMessageToIoWrite for (T, U)
where
    T: SerMessageToIoWrite,
    U: SerMessageToIoWrite,
{
    fn ser<W>(&self, out: &mut W) -> Result<()>
    where
        W: Write,
    {
        self.0.ser(out)?;
        self.1.ser(out)?;
        Ok(())
    }
}
impl<T, U> SerMessageToIoWrite for Either<T, U>
where
    T: SerMessageToIoWrite,
    U: SerMessageToIoWrite,
{
    fn ser<W>(&self, out: &mut W) -> Result<()>
    where
        W: Write,
    {
        match self {
            Either::Left(v) => v.ser(out)?,
            Either::Right(v) => v.ser(out)?,
        }
        Ok(())
    }
}
impl<T> SerMessageToIoWrite for Option<T>
where
    T: SerMessageToIoWrite,
{
    fn ser<W>(&self, out: &mut W) -> Result<()>
    where
        W: Write,
    {
        if let Some(ref msg) = self {
            msg.ser(out)?;
        }
        Ok(())
    }
}
impl<T> SerMessageToIoWrite for Box<T>
where
    T: SerMessageToIoWrite,
{
    fn ser<W>(&self, out: &mut W) -> Result<()>
    where
        W: Write,
    {
        use ::std::ops::Deref;
        <T as SerMessageToIoWrite>::ser(self.deref(), out)?;
        Ok(())
    }
}
impl<'a, T> SerMessageToIoWrite for &'a T
where
    T: SerMessageToIoWrite,
{
    fn ser<W>(&self, out: &mut W) -> Result<()>
    where
        W: Write,
    {
        use ::std::ops::Deref;
        <T as SerMessageToIoWrite>::ser(self.deref(), out)?;
        Ok(())
    }
}
impl<'a, T> SerMessageToIoWrite for &'a mut T
where
    T: SerMessageToIoWrite,
{
    fn ser<W>(&self, out: &mut W) -> Result<()>
    where
        W: Write,
    {
        use ::std::ops::Deref;
        <T as SerMessageToIoWrite>::ser(self.deref(), out)?;
        Ok(())
    }
}
