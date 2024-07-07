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

use crate::cases::{convert_into_case, Case};
use crate::generator::avoid_reserved_keywords;
use crate::Result;
use ::itertools::Itertools;
use ::quote::quote;
use ::std::borrow::Borrow;
use ::std::fmt::Display;
use ::std::ops::Deref;
use ::syn::{parse2, parse_str, Path, PathSegment};

#[derive(Debug, Eq, Ord, Hash)]
pub struct ProtoPath(str);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Default)]
pub struct ProtoPathBuf(String);

impl ProtoPath {
    pub fn new<S: AsRef<str> + ?Sized>(path: &S) -> &Self {
        unsafe { &*(path.as_ref() as *const str as *const ProtoPath) }
    }

    pub fn is_absolute(&self) -> bool {
        self.0.starts_with('.')
    }
    pub fn is_relative(&self) -> bool {
        !self.is_absolute()
    }
    pub fn parent(&self) -> Option<&Self> {
        match self.0.rsplit_once('.') {
            None if self.0.is_empty() => None,
            // parent of "a" is "".
            None => Some(ProtoPath::new("")),
            // rsplit(".") returns ("", "").
            Some(("", "")) => None,
            // rsplit(".a") returns ("", "a").
            Some(("", _)) => Some(ProtoPath::new(".")),
            Some((parent, _)) => Some(ProtoPath::new(parent)),
        }
    }
    pub fn last_component(&self) -> Option<&str> {
        match self.0.rsplit_once('.') {
            None if self.0.is_empty() => None,
            None => Some(&self.0),
            // rsplit(".") returns ("", "").
            Some(("", "")) => None,
            Some((_, last)) => Some(last),
        }
    }
    pub fn components(&self) -> impl Iterator<Item = &str> {
        let relative = self.0.strip_prefix('.').unwrap_or(&self.0);
        (!relative.is_empty())
            .then(|| relative.split('.'))
            .into_iter()
            .flatten()
    }
    pub fn ancestors(&self) -> impl Iterator<Item = &Self> {
        ::std::iter::successors(Some(self), |path| path.parent())
    }
    pub fn strip_prefix(&self, prefix: &Self) -> Result<&Self> {
        if prefix.0.ends_with('.') {
            if self.0.starts_with(&prefix.0) {
                Ok(ProtoPath::new(&self.0[prefix.0.len()..]))
            } else {
                Err(format!(
                    "Failed to strip prefix: {} from {}.",
                    self.0.to_string(),
                    prefix.0.to_string(),
                ))?
            }
        } else {
            if self.0.starts_with(&prefix.0) && self.0[prefix.0.len()..].starts_with('.') {
                Ok(ProtoPath::new(&self.0[prefix.0.len() + 1..]))
            } else {
                Err(format!(
                    "Failed to strip prefix: {} from {}.",
                    self.0.to_string(),
                    prefix.0.to_string(),
                ))?
            }
        }
    }

    pub fn to_rust_file_path(&self) -> String {
        let mut result = self
            .0
            .split('.')
            .filter(|s| !s.is_empty())
            .map(|s| convert_into_case(s, Case::LowerSnakeCase))
            .map(|s| avoid_reserved_keywords(&s).to_string())
            .join("/");
        if result.is_empty() {
            return "mod.rs".to_string();
        } else {
            result += ".rs";
            return result;
        }
    }

    pub fn to_rust_path(&self) -> Result<Path> {
        self.to_rust_path_with(|item| {
            Ok(parse_str(&avoid_reserved_keywords(&convert_into_case(
                item,
                Case::CamelCase,
            )))?)
        })
    }
    pub fn to_rust_path_with(
        &self,
        last_item_naming: impl FnOnce(&str) -> Result<PathSegment>,
    ) -> Result<Path> {
        let first_component: PathSegment =
            parse_str(if self.is_absolute() { "crate" } else { "self" })?;
        if let (components_iter, Some(item)) = (
            self.parent().into_iter().flat_map(|p| p.components()),
            self.last_component(),
        ) {
            let modules = components_iter
                .map(|s| {
                    Ok(parse_str(&avoid_reserved_keywords(&convert_into_case(
                        s,
                        Case::LowerSnakeCase,
                    )))?)
                })
                .collect::<Result<Vec<PathSegment>>>()?;
            let item = last_item_naming(item)?;
            Ok(parse2(quote! { #first_component :: #(#modules::)* #item})?)
        } else {
            Err(format!(
                "The proto path {} cannot be converted to a rust path.",
                self.as_str()
            ))?
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl ProtoPathBuf {
    pub fn new() -> Self {
        Self("".to_string())
    }
    pub fn push(&mut self, path: impl AsRef<ProtoPath>) {
        if !self.0.ends_with('.') {
            self.0.push('.');
        }
        self.0.push_str(&path.as_ref().0);
    }
}

impl AsRef<str> for ProtoPath {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl AsRef<ProtoPath> for ProtoPath {
    fn as_ref(&self) -> &ProtoPath {
        self
    }
}
impl AsRef<ProtoPath> for str {
    fn as_ref(&self) -> &ProtoPath {
        ProtoPath::new(self)
    }
}
impl AsRef<ProtoPath> for String {
    fn as_ref(&self) -> &ProtoPath {
        ProtoPath::new(self)
    }
}
impl From<&ProtoPath> for String {
    fn from(v: &ProtoPath) -> String {
        v.0.to_string()
    }
}
impl Display for ProtoPath {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl From<&ProtoPath> for ProtoPathBuf {
    fn from(v: &ProtoPath) -> ProtoPathBuf {
        ProtoPathBuf(v.0.to_string())
    }
}
impl ToOwned for ProtoPath {
    type Owned = ProtoPathBuf;
    fn to_owned(&self) -> Self::Owned {
        ProtoPathBuf(self.0.to_string())
    }
}
impl Deref for ProtoPathBuf {
    type Target = ProtoPath;
    fn deref(&self) -> &Self::Target {
        ProtoPath::new(&self.0)
    }
}

impl AsRef<str> for ProtoPathBuf {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl AsRef<ProtoPath> for ProtoPathBuf {
    fn as_ref(&self) -> &ProtoPath {
        ProtoPath::new(&self.0)
    }
}
impl Borrow<ProtoPath> for ProtoPathBuf {
    fn borrow(&self) -> &ProtoPath {
        ProtoPath::new(&self.0)
    }
}
impl From<ProtoPathBuf> for String {
    fn from(v: ProtoPathBuf) -> String {
        v.0
    }
}
impl From<String> for ProtoPathBuf {
    fn from(v: String) -> ProtoPathBuf {
        ProtoPathBuf(v)
    }
}
impl From<&str> for ProtoPathBuf {
    fn from(v: &str) -> ProtoPathBuf {
        ProtoPathBuf(v.to_string())
    }
}
impl Display for ProtoPathBuf {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl PartialEq for ProtoPath {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_bytes() == other.0.as_bytes()
    }
}
impl PartialOrd for ProtoPath {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.as_bytes().partial_cmp(&other.0.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::itertools::Itertools;

    #[test]
    fn test_parent() {
        assert_eq!(ProtoPath::new("a.b.c").parent().unwrap().as_str(), "a.b");
        assert_eq!(ProtoPath::new("a.b").parent().unwrap().as_str(), "a");
        assert_eq!(ProtoPath::new("a").parent().unwrap().as_str(), "");
        assert_eq!(ProtoPath::new("").parent(), None);

        assert_eq!(ProtoPath::new(".a.b.c").parent().unwrap().as_str(), ".a.b");
        assert_eq!(ProtoPath::new(".a.b").parent().unwrap().as_str(), ".a");
        assert_eq!(ProtoPath::new(".a").parent().unwrap().as_str(), ".");
        assert_eq!(ProtoPath::new(".").parent(), None);
    }

    #[test]
    fn test_last_component() {
        assert_eq!(ProtoPath::new("a.b.c").last_component().unwrap(), "c");
        assert_eq!(ProtoPath::new("a.b").last_component().unwrap(), "b");
        assert_eq!(ProtoPath::new("a").last_component().unwrap(), "a");
        assert_eq!(ProtoPath::new("").last_component(), None);

        assert_eq!(ProtoPath::new(".a.b.c").last_component().unwrap(), "c");
        assert_eq!(ProtoPath::new(".a.b").last_component().unwrap(), "b");
        assert_eq!(ProtoPath::new(".a").last_component().unwrap(), "a");
        assert_eq!(ProtoPath::new(".").last_component(), None);
    }

    #[test]
    fn test_compenents() {
        assert_eq!(
            ProtoPath::new("a.b.c").components().collect_vec(),
            vec!["a", "b", "c",]
        );
        assert_eq!(
            ProtoPath::new("a.b").components().collect_vec(),
            vec!["a", "b",]
        );
        assert_eq!(ProtoPath::new("a").components().collect_vec(), vec!["a",]);
        assert_eq!(ProtoPath::new("").components().next(), None);

        assert_eq!(
            ProtoPath::new(".a.b.c").components().collect_vec(),
            vec!["a", "b", "c",]
        );
        assert_eq!(
            ProtoPath::new(".a.b").components().collect_vec(),
            vec!["a", "b",]
        );
        assert_eq!(ProtoPath::new(".a").components().collect_vec(), vec!["a",]);
        assert_eq!(ProtoPath::new(".").components().next(), None);
    }

    #[test]
    fn test_ancestors() {
        assert_eq!(
            ProtoPath::new("a.b.c")
                .ancestors()
                .map(|p| p.as_str())
                .collect_vec(),
            vec!["a.b.c", "a.b", "a", ""]
        );
        assert_eq!(
            ProtoPath::new("a.b")
                .ancestors()
                .map(|p| p.as_str())
                .collect_vec(),
            vec!["a.b", "a", ""]
        );
        assert_eq!(
            ProtoPath::new("a")
                .ancestors()
                .map(|p| p.as_str())
                .collect_vec(),
            vec!["a", ""]
        );
        assert_eq!(
            ProtoPath::new("")
                .ancestors()
                .map(|p| p.as_str())
                .collect_vec(),
            vec![""]
        );

        assert_eq!(
            ProtoPath::new(".a.b.c")
                .ancestors()
                .map(|p| p.as_str())
                .collect_vec(),
            vec![".a.b.c", ".a.b", ".a", "."]
        );
        assert_eq!(
            ProtoPath::new(".a.b")
                .ancestors()
                .map(|p| p.as_str())
                .collect_vec(),
            vec![".a.b", ".a", "."]
        );
        assert_eq!(
            ProtoPath::new(".a")
                .ancestors()
                .map(|p| p.as_str())
                .collect_vec(),
            vec![".a", "."]
        );
        assert_eq!(
            ProtoPath::new(".")
                .ancestors()
                .map(|p| p.as_str())
                .collect_vec(),
            vec!["."]
        );
    }
}
