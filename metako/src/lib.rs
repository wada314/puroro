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

#![feature(generic_associated_types)]

pub mod bool;
pub mod func;
pub mod list;
pub mod map;
pub mod number;

pub use crate::bool::{AllOf2, And, AnyOf2, Bool, Not, Or, Switch, SwitchFunctor, B0, B1};
pub use crate::func::{Const, Functor, Pred};
pub use crate::number::{IsNumberEqualFunctor, Number};
