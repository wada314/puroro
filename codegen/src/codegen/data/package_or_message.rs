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

use super::{DataTypeBase, Enum, Message, MessageOrEnumCase, Oneof, Package, PackageOrMessageCase};
use crate::{FatalErrorKind, Result};
use ::std::fmt::Debug;
use ::std::ops::Deref;
use ::std::rc::Rc;

pub(crate) trait PackageOrMessage: DataTypeBase + Debug {
    fn either(&self) -> PackageOrMessageCase<&Package, &Message>;

    fn messages(&self) -> Result<Box<dyn '_ + Iterator<Item = &Rc<Message>>>>;
    fn enums(&self) -> Result<Box<dyn '_ + Iterator<Item = &Rc<Enum>>>>;
    fn oneofs(&self) -> Result<Box<dyn '_ + Iterator<Item = &Rc<Oneof>>>>;
    fn subpackages(&self) -> Result<Box<dyn '_ + Iterator<Item = &Rc<Package>>>>;
    fn root_package(&self) -> Result<Rc<Package>>;
    fn parent(&self) -> Result<Option<Rc<dyn PackageOrMessage>>>;

    fn is_root(&self) -> Result<bool> {
        Ok(self.parent()?.is_none())
    }

    fn all_child_packages(&self) -> Result<Vec<Rc<Package>>> {
        let mut ret = Vec::new();
        let mut stack = self.subpackages()?.collect::<Vec<_>>();
        while let Some(p) = stack.pop() {
            stack.extend(p.subpackages()?);
            ret.push(Rc::clone(p));
        }
        Ok(ret)
    }

    fn all_child_messages(&self) -> Result<Vec<Rc<Message>>> {
        let mut ret = self.messages()?.cloned().collect::<Vec<_>>();
        let messages_iter = self
            .messages()?
            .map(|m| Rc::deref(m) as &dyn PackageOrMessage);
        let packages_iter = self
            .subpackages()?
            .map(|p| Rc::deref(p) as &dyn PackageOrMessage);
        let mut stack = messages_iter.chain(packages_iter).collect::<Vec<_>>();
        while let Some(p) = stack.pop() {
            stack.extend(p.messages()?.map(|m| Rc::deref(m) as &dyn PackageOrMessage));
            stack.extend(
                p.subpackages()?
                    .map(|p| Rc::deref(p) as &dyn PackageOrMessage),
            );
            ret.extend(p.messages()?.cloned());
        }
        Ok(ret)
    }

    fn resolve_type_name(
        &self,
        type_name: &str,
    ) -> Result<MessageOrEnumCase<Rc<Message>, Rc<Enum>>> {
        if let Some(absolute_path) = type_name.strip_prefix('.') {
            return self.root_package()?.resolve_type_name(absolute_path);
        }

        if let Some((subcomponent, rest)) = type_name.split_once('.') {
            // 2 or more remaining components. The next component is either a message or a package.
            if let Some(m) = self
                .messages()?
                .try_find(|m| -> Result<_> { Ok(m.name()? == subcomponent) })?
            {
                return m.resolve_type_name(rest);
            } else if let Some(p) = self
                .subpackages()?
                .try_find(|p| -> Result<_> { Ok(p.name()? == subcomponent) })?
            {
                return p.resolve_type_name(rest);
            }
        } else {
            // Exactly 1 remaining component. Message or Enum. Return that item.
            if let Some(m) = self
                .messages()?
                .try_find(|m| -> Result<_> { Ok(m.name()? == type_name) })?
            {
                return Ok(MessageOrEnumCase::Message(Rc::clone(m)));
            } else if let Some(e) = self
                .enums()?
                .try_find(|e| -> Result<_> { Ok(e.name()? == type_name) })?
            {
                return Ok(MessageOrEnumCase::Enum(Rc::clone(e)));
            }
        }
        Err(FatalErrorKind::UnknownTypeName {
            name: type_name.to_string(),
        })?
    }
}
