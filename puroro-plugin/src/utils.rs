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

enum WordCase2 {
    CamelCase,
    LowerSnakeCase,
    UpperSnakeCase,
}

pub trait StrExt {
    fn convert_word_case(&self, case: WordCase2) -> Cow<str>;
    fn matches_word_case(&self, case: WordCase2) -> bool;
}

impl<T: AsRef<str>> StrExt for T {
    fn convert_word_case(&self, case: WordCase2) -> Cow<str> {
        todo!()
    }

    fn matches_word_case(&self, case: WordCase2) -> bool {
        todo!()
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
