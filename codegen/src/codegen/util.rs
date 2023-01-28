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

use crate::{FatalErrorKind, Result};
use ::lazy_static::lazy_static;
use ::once_cell::unsync::OnceCell;
use ::std::any::Any;
use ::std::borrow::Cow;
use ::std::collections::HashSet;
use ::std::rc::{Rc, Weak};

pub(crate) trait WeakExt<T: ?Sized> {
    fn try_upgrade(&self) -> Result<Rc<T>>;
}
impl<T: ?Sized> WeakExt<T> for Weak<T> {
    fn try_upgrade(&self) -> Result<Rc<T>> {
        Ok(Weak::upgrade(self).ok_or(FatalErrorKind::InternalError {
            detail: "Weak ptr upgrade failed".to_string(),
        })?)
    }
}

#[derive(Debug, Default)]
pub(crate) struct AnonymousCache {
    list: OnceCell<Cell>,
}
impl AnonymousCache {
    pub(crate) fn get<T: 'static + Default>(&self) -> Result<&T> {
        self.list
            .get_or_try_init(|| -> Result<_> {
                Ok(Cell {
                    car: Box::new(T::default()),
                    cdr: OnceCell::default(),
                })
            })?
            .get_or_insert()
    }
}
#[derive(Debug)]
struct Cell {
    car: Box<dyn Any>,
    cdr: OnceCell<Box<Cell>>,
}
impl Cell {
    fn get_or_insert<T: 'static + Default>(&self) -> Result<&T> {
        if let Some(t) = self.car.downcast_ref::<T>() {
            Ok(t)
        } else {
            self.cdr
                .get_or_try_init(|| -> Result<_> {
                    Ok(Box::new(Cell {
                        car: Box::new(T::default()),
                        cdr: OnceCell::default(),
                    }))
                })?
                .get_or_insert::<T>()
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum WordCase {
    CamelCase,
    LowerSnakeCase,
    #[allow(unused)]
    UpperSnakeCase,
}

pub(crate) trait StrExt {
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
        let assume_input_is_camel_case = !self.as_ref().contains('_')
            && (self.as_ref().contains(|c: char| c.is_lowercase())
                && self.as_ref().contains(|c: char| c.is_uppercase()));
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
                    if !last_char_was_uppercase || assume_input_is_camel_case {
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
            words = words
                .into_iter()
                .filter(|word| !word.is_empty())
                .collect::<Vec<_>>();
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
                    && s.chars().any(|c| c.is_ascii_lowercase())
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
        } else if KEYWORDS_NO_RAW.contains(self.as_ref()) {
            Cow::Owned(format!("_{}", self.as_ref()))
        } else {
            Cow::Borrowed(self.as_ref())
        }
    }
}

lazy_static! {
    static ref KEYWORDS: HashSet<&'static str> = [
        "as", "break", "const", "continue", "else", "enum", "extern", "false", "fn", "for", "if",
        "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
        "static", "struct", "trait", "true", "type", "unsafe", "use", "where", "while", "async",
        "await", "dyn", "abstract", "become", "box", "do", "final", "macro", "override", "priv",
        "typeof", "unsized", "virtual", "yield", "try", "union", "'static", "async", "await",
        "dyn", "abstract", "become", "box", "do", "final", "macro", "override", "priv", "typeof",
        "unsized", "virtual", "yield", "try",
    ]
    .iter()
    .copied()
    .collect::<HashSet<&'static str>>();
    static ref KEYWORDS_NO_RAW: HashSet<&'static str> = ["crate", "self", "Self", "super",]
        .iter()
        .copied()
        .collect::<HashSet<&'static str>>();
}

#[allow(unused)]
pub(crate) fn convert_octal_escapes_to_bytes(input: &str) -> Result<Vec<u8>> {
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
                                    .map_err(|e| FatalErrorKind::ParseIntError { source: e })?;
                                    decoded.push(u8_value);
                                }
                                _ => Err(FatalErrorKind::InvalidString {
                                    string: input.to_string(),
                                })?,
                            }
                        }
                    }
                } else {
                    Err(FatalErrorKind::InvalidString {
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
    Ok(decoded)
}
