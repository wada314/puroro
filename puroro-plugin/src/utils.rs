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

use crate::{ErrorKind, Result};
use ::itertools::Itertools;
use ::lazy_static::lazy_static;
use ::std::borrow::Borrow;
use ::std::borrow::Cow;
use ::std::collections::HashSet;
use ::std::fmt::Display;

#[derive(Debug, Clone, Copy)]
#[allow(unused)]
pub enum WordCase {
    CamelCase,
    LowerSnakeCase,
    UpperSnakeCase,
}

pub trait StrExt {
    fn to_word_case(&self, case: WordCase) -> Cow<str>;
    fn word_case_matches(&self, case: WordCase) -> bool;
    fn escape_rust_keywords(&self) -> Cow<str>;

    fn to_lower_snake_case(&self) -> Cow<str> {
        self.to_word_case(WordCase::LowerSnakeCase)
    }
    fn to_upper_snake_case(&self) -> Cow<str> {
        self.to_word_case(WordCase::UpperSnakeCase)
    }
    fn to_camel_case(&self) -> Cow<str> {
        self.to_word_case(WordCase::CamelCase)
    }
}

impl<T: AsRef<str>> StrExt for T {
    fn to_word_case(&self, case: WordCase) -> Cow<str> {
        if self.word_case_matches(case) {
            return Cow::Borrowed(self.as_ref());
        }
        let src_words = {
            let mut words = Vec::new();
            let mut word = String::new();
            let mut last_char_was_uppercase = true;
            for c in self.as_ref().chars() {
                if c == '_' {
                    words.push(word);
                    word = String::new();
                    last_char_was_uppercase = false;
                } else if c.is_ascii_uppercase() {
                    if !last_char_was_uppercase {
                        words.push(word);
                        word = String::new();
                    }
                    word.push(c);
                    last_char_was_uppercase = true;
                } else {
                    word.push(c);
                    last_char_was_uppercase = false;
                }
            }
            words.push(word);
            words
        };
        let words = src_words
            .into_iter()
            .map(|s| match case {
                WordCase::CamelCase => s
                    .char_indices()
                    .map(|(i, c)| {
                        if i == 0 {
                            c.to_ascii_uppercase()
                        } else {
                            c.to_ascii_lowercase()
                        }
                    })
                    .collect::<String>(),
                WordCase::LowerSnakeCase => s.to_ascii_lowercase(),
                WordCase::UpperSnakeCase => s.to_ascii_uppercase(),
            })
            .collect::<Vec<_>>();
        Cow::Owned(match case {
            WordCase::CamelCase => words.concat(),
            WordCase::LowerSnakeCase => words.join("_"),
            WordCase::UpperSnakeCase => words.join("_"),
        })
    }

    fn word_case_matches(&self, case: WordCase) -> bool {
        let s = AsRef::<str>::as_ref(self);
        match case {
            WordCase::CamelCase => {
                s.chars().all(|c| c.is_ascii_alphanumeric())
                    && s.chars().next().is_some_and(|c| c.is_ascii_uppercase())
            }
            WordCase::LowerSnakeCase => {
                s.chars().next().is_some_and(|c| c.is_ascii_lowercase())
                    && s.chars()
                        .all(|c| c == '_' || c.is_ascii_lowercase() || c.is_ascii_digit())
                    && s.split('_')
                        .all(|w| w.chars().next().is_some_and(|c| c.is_ascii_lowercase()))
            }
            WordCase::UpperSnakeCase => {
                s.chars().next().is_some_and(|c| c.is_ascii_uppercase())
                    && s.chars()
                        .all(|c| c == '_' || c.is_ascii_uppercase() || c.is_ascii_digit())
                    && s.split('_')
                        .all(|w| w.chars().next().is_some_and(|c| c.is_ascii_uppercase()))
            }
        }
    }

    fn escape_rust_keywords(&self) -> Cow<str> {
        if KEYWORDS.contains(self.as_ref()) {
            Cow::Owned(format!("r#{}", self.as_ref()))
        } else {
            Cow::Borrowed(self.as_ref())
        }
    }
}

lazy_static! {
    static ref KEYWORDS: HashSet<&'static str> = [
        "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn",
        "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
        "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe",
        "use", "where", "while", "async", "await", "dyn", "abstract", "become", "box", "do",
        "final", "macro", "override", "priv", "typeof", "unsized", "virtual", "yield", "try",
        "union", "'static",
    ]
    .iter()
    .copied()
    .collect::<HashSet<&'static str>>();
}

#[allow(unused)]
pub fn convert_octal_escape_to_rust_style_escape(input: &str) -> Result<String> {
    // protoc escapes 0x7F~0xFF character as octal escape "\234".
    // Rust does not support that style so we need to re-encode it.
    let mut decoded = Vec::new();
    let mut bytes = input.bytes();
    loop {
        if let Some(c) = bytes.next() {
            if c == b'\\' {
                if let Some(d) = bytes.next() {
                    match d {
                        b'\\' | b'\"' | b'\'' => {
                            decoded.push(d);
                        }
                        b'r' => decoded.push(b'\r'),
                        b'n' => decoded.push(b'\n'),
                        b't' => decoded.push(b'\t'),
                        _ => {
                            let e_opt = bytes.next();
                            let f_opt = bytes.next();
                            match (d, e_opt, f_opt) {
                                (
                                    (b'0'..=b'9'),
                                    Some(e @ (b'0'..=b'9')),
                                    Some(f @ (b'0'..=b'9')),
                                ) => {
                                    let u8_value = u8::from_str_radix(
                                        &format!("{}{}{}", d - b'0', e - b'0', f - b'0'),
                                        8,
                                    )
                                    .map_err(|e| ErrorKind::ParseIntError { source: e })?;
                                    decoded.push(u8_value);
                                }
                                _ => Err(ErrorKind::InvalidString {
                                    string: input.to_string(),
                                })?,
                            }
                        }
                    }
                } else {
                    Err(ErrorKind::InvalidString {
                        string: input.to_string(),
                    })?
                }
            } else {
                decoded.push(c);
            }
        } else {
            break;
        }
    }
    Ok(decoded
        .into_iter()
        .map(|b| format!(r"\x{:02x}", b))
        .collect::<String>())
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Package<S>(S);

impl<S> Package<S> {
    pub fn new(package_str: S) -> Self {
        Self(package_str)
    }
}

impl<S: AsRef<str>> Package<S> {
    pub fn packages_and_subpackages(&self) -> impl Iterator<Item = (Package<&str>, &str)> {
        self.0.as_ref().split('.').scan(0, |path_end, package| {
            let path = &self.0.as_ref()[0..*path_end];

            if *path_end != 0 {
                *path_end += 1;
            }
            *path_end += package.len();

            Some((Package::new(path), package))
        })
    }

    pub fn leaf_package_name(&self) -> Option<&str> {
        let s = self.0.as_ref();
        match (s.is_empty(), s.rsplit_once('.')) {
            (false, Some((_, p))) => Some(p),
            (false, None) => Some(s),
            (true, _) => None,
        }
    }

    pub fn full_package_path(&self) -> &str {
        self.0.as_ref()
    }

    pub fn to_owned(&self) -> Package<String> {
        Package::new(self.0.as_ref().to_string())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl<S: Borrow<str>> Borrow<str> for Package<S> {
    fn borrow(&self) -> &str {
        self.0.borrow()
    }
}

impl<S: AsRef<str>> Display for Package<S> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.0.as_ref())
    }
}

impl<S, T> ::std::ops::Add<T> for &Package<S>
where
    S: AsRef<str>,
    T: AsRef<str>,
{
    type Output = Package<String>;
    fn add(self, rhs: T) -> Self::Output {
        let mut new_string = self.0.as_ref().to_string();
        if !new_string.is_empty() {
            new_string.push('.');
        }
        new_string.push_str(rhs.as_ref());
        Package::new(new_string)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Fqtn(String);
impl Fqtn {
    pub fn new<S: ToString>(s: S) -> Self {
        Self(s.to_string())
    }
}
impl Fqtn {
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }

    pub fn to_rust_path(&self) -> String {
        assert!(self.0.starts_with('.'));
        let absl_path = &self.0[1..];
        let (path_nodes, leaf) = match absl_path.rsplit_once('.') {
            Some((path, leaf)) => (Some(path.split('.')).into_iter().flatten(), leaf),
            None => (None.into_iter().flatten(), self.0.as_ref()),
        };
        let escaped_path_nodes =
            path_nodes.map(|s| s.to_lower_snake_case().escape_rust_keywords().into_owned());
        let escaped_leaf = leaf.to_camel_case().escape_rust_keywords().into_owned();
        let mut result = String::new();
        result.push_str("_puroro_root");
        for node in escaped_path_nodes {
            result.push_str("::");
            result.push_str(&node);
        }
        result.push_str("::");
        result.push_str(&escaped_leaf);
        result
    }

    pub fn to_rust_module_path(&self) -> String {
        assert!(self.0.starts_with('.'));
        let absl_path = &self.0[1..];
        let path_nodes = absl_path
            .split('.')
            .map(|s| s.to_lower_snake_case().escape_rust_keywords().into_owned());
        let mut result = String::new();
        result.push_str("_puroro_root");
        for node in path_nodes {
            result.push_str("::");
            result.push_str(&node);
        }
        result
    }

    pub fn to_rust_module_file_path(&self) -> String {
        assert!(self.0.starts_with('.'));
        let absl_path = &self.0[1..];
        absl_path
            .split('.')
            .map(|s| s.to_lower_snake_case().into_owned())
            .join("/")
            + ".rs"
    }
}
impl Display for Fqtn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        String::fmt(&self.0, f)
    }
}

#[cfg(test)]
mod test {
    use super::{Package, StrExt, WordCase};

    #[test]
    fn test_case_check() {
        use WordCase::*;
        const STR1: &'static str = "ThisIsAPen";
        const STR2: &'static str = "this_is_a_pen";
        const STR3: &'static str = "THIS_IS_A_PEN";
        assert_eq!(true, STR1.word_case_matches(CamelCase));
        assert_eq!(false, STR1.word_case_matches(LowerSnakeCase));
        assert_eq!(false, STR1.word_case_matches(UpperSnakeCase));
        assert_eq!(false, STR2.word_case_matches(CamelCase));
        assert_eq!(true, STR2.word_case_matches(LowerSnakeCase));
        assert_eq!(false, STR2.word_case_matches(UpperSnakeCase));
        assert_eq!(false, STR3.word_case_matches(CamelCase));
        assert_eq!(false, STR3.word_case_matches(LowerSnakeCase));
        assert_eq!(true, STR3.word_case_matches(UpperSnakeCase));
    }

    #[test]
    fn test_subpackages() {
        let package = Package::new("foo.bar.baz");
        let mut iter = package.packages_and_subpackages();
        assert_eq!(Some((Package::new(""), "foo")), iter.next());
        assert_eq!(Some((Package::new("foo"), "bar")), iter.next());
        assert_eq!(Some((Package::new("foo.bar"), "baz")), iter.next());
        assert_eq!(None, iter.next());
    }
}
