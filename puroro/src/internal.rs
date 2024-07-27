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

pub mod deser;
pub mod types;

use crate::{ErrorKind, Result};

#[derive(Debug)]
pub(crate) enum WireType {
    Variant = 0,
    I64 = 1,
    Len = 2,
    I32 = 5,
}
impl TryFrom<u32> for WireType {
    type Error = ErrorKind;
    fn try_from(value: u32) -> Result<WireType> {
        Ok(match value {
            0 => WireType::Variant,
            1 => WireType::I64,
            2 => WireType::Len,
            5 => WireType::I32,
            _ => Err(ErrorKind::UnknownWireType)?,
        })
    }
}
impl From<WireType> for u32 {
    fn from(value: WireType) -> u32 {
        value as u32
    }
}

pub trait Any {}
impl<T> Any for T {}

#[macro_export]
macro_rules! impl_message_trait_for_trivial_types {
    ($(
        $vis:vis trait $tname:ident $(: $supertt:tt)? {
            $(fn $mname:ident(&self $(, $pname:ident : $pty:ty)* ) $( -> $rty:ty)? ;)*
        }
    )*) => {
        $(
            $vis trait $tname $(: $supertt)? {
                $(fn $mname(&self $(, $pname : $pty)* ) $(-> $rty)?;)*
            }
            impl<'a, T: $tname> $tname for &'a T {
                $(fn $mname(&self $(, $pname : $pty)* ) $(-> $rty)? {
                    <T as $tname>::$mname(self $(, $pname)* )
                })*
            }
            impl<'a, T: $tname> $tname for &'a mut T {
                $(fn $mname(&self $(, $pname : $pty)* ) $(-> $rty)? {
                    <T as $tname>::$mname(self $(, $pname)* )
                })*
            }
        )*
    };
}
pub use impl_message_trait_for_trivial_types;

#[macro_export]
macro_rules! impl_message_mut_trait_for_trivial_types {
    ($(
        $vis:vis trait $tname:ident $(: $supertt:tt)? {
            $(fn $mname:ident(&mut self $(, $pname:ident : $pty:ty)* ) $( -> $rty:ty)? ;)*
        }
    )*) => {
        $(
            $vis trait $tname $(: $supertt)? {
                $(fn $mname(&mut self $(, $pname : $pty)* ) $( -> $rty)?;)*
            }
            impl<'a, T: $tname> $tname for &'a mut T {
                $(fn $mname(&mut self $(, $pname : $pty)* ) $(-> $rty)? {
                    <T as $tname>::$mname(self $(, $pname)* )
                })*
            }
        )*
    };
}
pub use impl_message_mut_trait_for_trivial_types;
