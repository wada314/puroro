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

use super::super::util::{AnonymousCache, StrExt, WeakExt};
use super::super::{
    Enum, EnumImpl, Field, FieldExt, FieldImpl, InputFile, Package, PackageOrMessage,
    PackageOrMessageExt,
};
use crate::Result;
use ::once_cell::unsync::OnceCell;
use ::proc_macro2::TokenStream;
use ::puroro_protobuf_compiled::google::protobuf::{
    DescriptorProto, EnumDescriptorProto, FieldDescriptorProto,
};
use ::quote::{format_ident, quote};
use ::std::borrow::Cow;
use ::std::fmt::Debug;
use ::std::iter;
use ::std::rc::{Rc, Weak};
