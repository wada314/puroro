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

use super::{EnumTrait, MessageOrEnum, MessageTrait, PackageTrait, RootPackage};
use crate::{ErrorKind, Result};
use ::std::borrow::Cow;
use ::std::rc::Rc;

#[derive(Debug, Clone, Copy)]
pub(super) enum PackageOrMessage<P, M> {
    Package(P),
    Message(M),
}
impl PackageOrMessage<Rc<dyn PackageTrait>, Rc<dyn MessageTrait>> {
    pub(super) fn messages(&self) -> Result<&[Rc<dyn MessageTrait>]> {
        match self {
            PackageOrMessage::Package(p) => p.messages(),
            PackageOrMessage::Message(m) => m.messages(),
        }
    }
    pub(super) fn enums(&self) -> Result<&[Rc<dyn EnumTrait>]> {
        match self {
            PackageOrMessage::Package(p) => p.enums(),
            PackageOrMessage::Message(m) => m.enums(),
        }
    }
    pub(super) fn subpackages(&self) -> Result<&[Rc<dyn PackageTrait>]> {
        match self {
            PackageOrMessage::Package(p) => p.subpackages(),
            PackageOrMessage::Message(_) => Ok(&[]),
        }
    }
    pub(super) fn root_package(&self) -> Result<Rc<RootPackage>> {
        match self {
            PackageOrMessage::Package(p) => p.root(),
            PackageOrMessage::Message(m) => m.input_file()?.package()?.root(),
        }
    }
    pub(super) fn parent(&self) -> Result<Option<Self>> {
        match self {
            PackageOrMessage::Package(p) => {
                Ok(p.parent()?.map(|parent| PackageOrMessage::Package(parent)))
            }
            PackageOrMessage::Message(m) => Ok(Some(m.parent()?)),
        }
    }
    pub(super) fn rust_module_path(&self) -> Result<Cow<'_, str>> {
        match self {
            PackageOrMessage::Package(p) => p.rust_module_path(),
            PackageOrMessage::Message(m) => m.rust_module_path().map(|s| s.into()),
        }
    }

    pub(super) fn resolve_type_name(
        &self,
        type_name: &str,
    ) -> Result<MessageOrEnum<Rc<dyn MessageTrait>, Rc<dyn EnumTrait>>> {
        if type_name.is_empty() {
            match self {
                PackageOrMessage::Package(_) => Err(ErrorKind::UnknownTypeName {
                    name: type_name.to_string(),
                })?,
                PackageOrMessage::Message(m) => {
                    return Ok(MessageOrEnum::Message(Rc::clone(m)));
                }
            }
        }

        if let Some(abs_type_name) = type_name.strip_prefix('.') {
            return self.root_package()?.resolve_type_name(abs_type_name);
        }

        let (subcomponent_name, rest) = type_name.split_once('.').unwrap_or((type_name, ""));

        if let Some(subpackage) = self
            .subpackages()?
            .into_iter()
            .try_find(|p| -> Result<_> { Ok(p.name()? == subcomponent_name) })?
        {
            return PackageOrMessage::Package(Rc::clone(subpackage)).resolve_type_name(rest);
        }
        if let Some(message) = self
            .messages()?
            .into_iter()
            .try_find(|m| -> Result<_> { Ok(m.name() == subcomponent_name) })?
        {
            return PackageOrMessage::Message(Rc::clone(message)).resolve_type_name(rest);
        }
        if let Some(enume) = self
            .enums()?
            .into_iter()
            .try_find(|e| -> Result<_> { Ok(e.name() == subcomponent_name) })?
        {
            if !rest.is_empty() {
                Err(ErrorKind::UnknownTypeName {
                    name: type_name.to_string(),
                })?
            }
            return Ok(MessageOrEnum::Enum(Rc::clone(enume)));
        }

        Err(ErrorKind::UnknownTypeName {
            name: type_name.to_string(),
        })?
    }
}
