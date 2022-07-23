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
use ::std::borrow::Cow;
use ::std::collections::HashSet;
use ::std::iter;
use ::std::rc::{Rc, Weak};

#[derive(Debug, Clone, Copy)]
enum WordCase2 {
    CamelCase,
    LowerSnakeCase,
    UpperSnakeCase,
}

pub trait StrExt {
    fn to_word_case(&self, case: WordCase2) -> Cow<str>;
    fn word_case_matches(&self, case: WordCase2) -> bool;
}

impl<T: AsRef<str>> StrExt for T {
    fn to_word_case(&self, case: WordCase2) -> Cow<str> {
        if self.word_case_matches(case) {
            return Cow::Borrowed(self.as_ref());
        }
        let src_words = {
            let mut words = Vec::new();
            let mut word = String::new();
            let mut last_char_was_uppercase = false;
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
                WordCase2::CamelCase => s
                    .char_indices()
                    .map(|(i, c)| {
                        if i == 0 {
                            c.to_ascii_uppercase()
                        } else {
                            c.to_ascii_lowercase()
                        }
                    })
                    .collect::<String>(),
                WordCase2::LowerSnakeCase => s.to_ascii_lowercase(),
                WordCase2::UpperSnakeCase => s.to_ascii_uppercase(),
            })
            .collect::<Vec<_>>();
        Cow::Owned(match case {
            WordCase2::CamelCase => words.concat(),
            WordCase2::LowerSnakeCase => words.join("_"),
            WordCase2::UpperSnakeCase => words.join("_"),
        })
    }

    fn word_case_matches(&self, case: WordCase2) -> bool {
        let s = AsRef::<str>::as_ref(self);
        match case {
            WordCase2::CamelCase => {
                s.chars().all(|c| c.is_ascii_alphanumeric())
                    && s.chars().next().is_some_and(|c| c.is_ascii_uppercase())
            }
            WordCase2::LowerSnakeCase => {
                s.chars().next().is_some_and(|c| c.is_ascii_lowercase())
                    && s.chars()
                        .all(|c| c == '_' || c.is_ascii_lowercase() || c.is_ascii_digit())
                    && s.split('_')
                        .all(|w| w.chars().next().is_some_and(|c| c.is_ascii_lowercase()))
            }
            WordCase2::UpperSnakeCase => {
                s.chars().next().is_some_and(|c| c.is_ascii_uppercase())
                    && s.chars()
                        .all(|c| c == '_' || c.is_ascii_uppercase() || c.is_ascii_digit())
                    && s.split('_')
                        .all(|w| w.chars().next().is_some_and(|c| c.is_ascii_uppercase()))
            }
        }
    }
}

enum WordCase {
    CamelCase,
    LowerCase,
}
fn convert_cases(input: &str, generate_snake_case: bool, out_word_case: WordCase) -> String {
    let mut word_begins_next = true;
    let mut last_was_lower_case = true;
    let mut out = String::new();
    for (i, c) in input.chars().enumerate() {
        if c == '_' {
            word_begins_next = true;
            last_was_lower_case = false;
            continue;
        } else {
            let word_begins = (last_was_lower_case && c.is_ascii_uppercase()) || word_begins_next;
            last_was_lower_case = c.is_ascii_lowercase();
            word_begins_next = false;
            if word_begins {
                if generate_snake_case && i != 0 {
                    out.push('_');
                }
                if let WordCase::LowerCase = out_word_case {
                    out.push(c.to_ascii_lowercase());
                } else {
                    out.push(c.to_ascii_uppercase());
                }
            } else {
                out.push(c.to_ascii_lowercase());
            }
        }
    }
    out
}
pub fn to_lower_snake_case(input: &str) -> String {
    convert_cases(input, true, WordCase::LowerCase)
}
pub fn to_camel_case(input: &str) -> String {
    convert_cases(input, false, WordCase::CamelCase)
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

pub fn get_keyword_safe_ident(input: &str) -> Cow<'_, str> {
    if KEYWORDS.contains(input) {
        Cow::Owned(format!("r#{}", input))
    } else {
        Cow::Borrowed(input)
    }
}

pub fn make_module_path<'a, I, J>(package: I, outer_messages: J) -> String
where
    I: Iterator<Item = &'a str>,
    J: Iterator<Item = &'a str> + Clone,
{
    let package = package.map(|s| get_keyword_safe_ident(s));

    let outer_messages = outer_messages.map(|s| {
        Cow::Owned(format!(
            "_puroro_nested::{}",
            get_keyword_safe_ident(&to_lower_snake_case(s))
        ))
    });
    let mut modules_iter = iter::once("self".into())
        .chain(iter::once("_puroro_root".into()))
        .chain(package)
        .chain(outer_messages);
    modules_iter.join("::")
}

pub fn upgrade<T>(weak: &Weak<T>) -> Result<Rc<T>> {
    Ok(Weak::upgrade(weak).ok_or(ErrorKind::InternalError {
        detail: "Failed to upgrade a Weak<> pointer.".to_string(),
    })?)
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

#[cfg(test)]
mod test {
    use super::{StrExt, WordCase2};

    #[test]
    fn test_case_check() {
        use WordCase2::*;
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
}
