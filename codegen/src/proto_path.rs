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
use crate::generator::{avoid_reserved_keywords, to_ident, CodeGeneratorOptions};
use crate::Result;
use ::itertools::Itertools;
use ::quote::{format_ident, quote};
use ::std::borrow::Borrow;
use ::std::fmt::Display;
use ::std::ops::Deref;
use ::syn::{parse2, Path, PathSegment};

#[derive(Debug, Eq, Ord, Hash)]
pub struct ProtoPath(str);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Default)]
pub struct ProtoPathBuf(String);

impl ProtoPath {
    /// Creates a new `ProtoPath` from a string slice.
    /// The path is not validated.
    pub fn new<S: AsRef<str> + ?Sized>(path: &S) -> &Self {
        unsafe { &*(path.as_ref() as *const str as *const ProtoPath) }
    }

    /// Returns `true` if the path is absolute (starts with a dot).
    ///
    /// # Examples
    ///
    /// ```
    /// use puroro_codegen::proto_path::ProtoPath;
    /// assert!(ProtoPath::new(".foo.bar").is_absolute());
    /// assert!(!ProtoPath::new("foo.bar").is_absolute());
    /// ```
    pub fn is_absolute(&self) -> bool {
        self.0.starts_with('.')
    }

    /// Returns `true` if the path is relative (does not start with a dot).
    ///
    /// # Examples
    ///
    /// ```
    /// # use puroro_codegen::proto_path::ProtoPath;
    /// assert!(ProtoPath::new("foo.bar").is_relative());
    /// assert!(!ProtoPath::new(".foo.bar").is_relative());
    /// ```
    pub fn is_relative(&self) -> bool {
        !self.is_absolute()
    }

    /// Returns the parent path, or `None` if this path has no parent.
    ///
    /// # Examples
    ///
    /// ```
    /// # use puroro_codegen::proto_path::ProtoPath;
    /// let path = ProtoPath::new("foo.bar.baz");
    /// assert_eq!(path.parent().unwrap().as_str(), "foo.bar");
    ///
    /// let path = ProtoPath::new("foo");
    /// assert_eq!(path.parent().unwrap().as_str(), "");
    ///
    /// let path = ProtoPath::new(".foo");
    /// assert_eq!(path.parent().unwrap().as_str(), ".");
    ///
    /// let path = ProtoPath::new(".");
    /// assert_eq!(path.parent(), None);
    /// ```
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

    /// Returns the last component of the path, or `None` if the path is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// # use puroro_codegen::proto_path::ProtoPath;
    /// let path = ProtoPath::new("foo.bar.baz");
    /// assert_eq!(path.last_component().unwrap(), "baz");
    ///
    /// let path = ProtoPath::new("foo");
    /// assert_eq!(path.last_component().unwrap(), "foo");
    ///
    /// let path = ProtoPath::new(".");
    /// assert_eq!(path.last_component(), None);
    /// ```
    pub fn last_component(&self) -> Option<&str> {
        match self.0.rsplit_once('.') {
            None if self.0.is_empty() => None,
            None => Some(&self.0),
            // rsplit(".") returns ("", "").
            Some(("", "")) => None,
            Some((_, last)) => Some(last),
        }
    }

    /// Returns an iterator over the components of the path.
    ///
    /// # Examples
    ///
    /// ```
    /// # use puroro_codegen::proto_path::ProtoPath;
    /// let path = ProtoPath::new("foo.bar.baz");
    /// let components: Vec<_> = path.components().collect();
    /// assert_eq!(components, vec!["foo", "bar", "baz"]);
    /// ```
    pub fn components(&self) -> impl Iterator<Item = &str> {
        let relative = self.0.strip_prefix('.').unwrap_or(&self.0);
        (!relative.is_empty())
            .then(|| relative.split('.'))
            .into_iter()
            .flatten()
    }

    /// Returns an iterator over the ancestor paths of the current path.
    ///
    /// # Examples
    ///
    /// ```
    /// # use puroro_codegen::proto_path::ProtoPath;
    /// let path = ProtoPath::new("a.b.c");
    /// assert_eq!(
    ///     path.ancestors().map(|p| p.as_str()).collect::<Vec<_>>(),
    ///     vec!["a.b.c", "a.b", "a", ""]
    /// );
    /// ```
    pub fn ancestors(&self) -> impl Iterator<Item = &Self> {
        ::std::iter::successors(Some(self), |path| path.parent())
    }

    /// Strips the specified prefix from this path, returning `None` if the prefix does not match.
    ///
    /// # Examples
    ///
    /// ```
    /// # use puroro_codegen::proto_path::ProtoPath;
    /// let path = ProtoPath::new("foo.bar.baz");
    /// let prefix = ProtoPath::new("foo.bar");
    /// assert_eq!(path.strip_prefix(prefix).unwrap().as_str(), "baz");
    /// ```
    pub fn strip_prefix(&self, prefix: &Self) -> Option<&Self> {
        if prefix.0.ends_with('.') {
            if self.0.starts_with(&prefix.0) {
                Some(ProtoPath::new(&self.0[prefix.0.len()..]))
            } else {
                None
            }
        } else {
            if self.0.starts_with(&prefix.0) && self.0[prefix.0.len()..].starts_with('.') {
                Some(ProtoPath::new(&self.0[prefix.0.len() + 1..]))
            } else {
                None
            }
        }
    }

    /// Converts this path to a relative path using the specified base path.
    /// Returns `None` if the conversion is not possible.
    ///
    /// # Examples
    ///
    /// ```
    /// # use puroro_codegen::proto_path::ProtoPath;
    /// let path = ProtoPath::new(".foo.bar.baz");
    /// let base = ProtoPath::new(".foo.bar");
    /// assert_eq!(path.to_relative_path(base).unwrap().as_str(), "baz");
    /// ```
    pub fn to_relative_path(&self, base: &Self) -> Option<&Self> {
        if self.is_absolute() && base.is_absolute() {
            for ancestors in base.ancestors() {
                if let Some(relative) = self.strip_prefix(ancestors) {
                    return Some(relative);
                }
            }
        }
        None
    }

    /// Consider the path is a proto package path or a proto message path,
    /// and convert it to a rust file path.
    ///
    /// # Examples
    ///
    /// ```
    /// # use puroro_codegen::proto_path::ProtoPath;
    /// let path = ProtoPath::new("foo.bar.baz");
    /// assert_eq!(path.to_rust_file_path(), "foo/bar/baz.rs");
    ///
    /// let path = ProtoPath::new(".");
    /// assert_eq!(path.to_rust_file_path(), "mod.rs");
    /// ```
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

    pub fn to_rust_path(&self, options: &CodeGeneratorOptions) -> Result<Path> {
        self.to_rust_path_with(options, |item| {
            Ok(to_ident(&convert_into_case(item, Case::CamelCase)).into())
        })
    }

    /// Convert the proto path to a rust path.
    /// The path items except the last one are converted to rust modules,
    /// and the user can specify the naming of the last item using the `last_item_naming` closure.
    ///
    /// # Examples
    ///
    /// ```
    /// # use puroro_codegen::proto_path::ProtoPath;
    /// use syn::parse_str;
    /// let path = ProtoPath::new("foo.bar.baz");
    /// let rust_path = path.to_rust_path_with(&Default::default(), |item| {
    ///     let capitalized = item.chars().enumerate().map(|(i, c)| {
    ///         if i == 0 { c.to_ascii_uppercase() } else { c }
    ///     }).collect::<String>();
    ///     Ok(parse_str(&capitalized)?)
    /// }).unwrap();
    /// assert_eq!(rust_path, parse_str("self::foo::bar::Baz").unwrap());
    /// ```

    pub fn to_rust_path_with(
        &self,
        options: &CodeGeneratorOptions,
        last_item_naming: impl FnOnce(&str) -> Result<PathSegment>,
    ) -> Result<Path> {
        let first_component = self
            .is_absolute()
            .then(|| format_ident!("_puroro_root"))
            .into_iter();
        if let (components_iter, Some(item)) = (
            self.parent().into_iter().flat_map(|p| p.components()),
            self.last_component(),
        ) {
            let modules = components_iter
                .map(|s| to_ident(&convert_into_case(s, Case::LowerSnakeCase)).into())
                .collect::<Vec<PathSegment>>();
            let item = last_item_naming(item)?;
            let path_from_self = parse2(quote! { #(#first_component ::)* #(#modules::)* #item})?;
            Ok(options.path_in_self_module(&path_from_self)?)
        } else {
            Err(format!(
                "The proto path {} cannot be converted to a rust path.",
                self.as_str()
            ))?
        }
    }

    /// Returns a string slice of the underlying path.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns an owned string of the underlying path.
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl ProtoPathBuf {
    /// Creates a new empty `ProtoPathBuf`.
    ///
    /// # Examples
    ///
    /// ```
    /// use puroro_codegen::proto_path::ProtoPathBuf;
    /// let path = ProtoPathBuf::new();
    /// assert_eq!(path.as_str(), "");
    /// ```
    pub fn new() -> Self {
        Self("".to_string())
    }

    /// Pushes a path component onto this path.
    ///
    /// # Examples
    ///
    /// ```
    /// use puroro_codegen::proto_path::ProtoPathBuf;
    /// let mut path = ProtoPathBuf::new();
    /// path.push("foo");
    /// path.push("bar");
    /// assert_eq!(path.as_str(), ".foo.bar");
    /// ```
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
