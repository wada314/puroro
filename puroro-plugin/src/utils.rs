use crate::Result;
use ::lazy_static::lazy_static;
use itertools::Itertools;
use std::borrow::Cow;
use std::fmt::Display;
use std::iter::FromIterator;
use std::{collections::HashSet, fmt::Write};

pub struct Indentor<W> {
    writer: W,
    indent_next: bool,
    level: usize,
}
impl<W> Indentor<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            indent_next: false,
            level: 0,
        }
    }
    pub fn indent(&mut self) {
        self.level += 1;
    }
    pub fn unindent(&mut self) {
        assert_ne!(0, self.level, "unindenting too much");
        self.level -= 1;
    }
    pub fn into_inner(self) -> W {
        self.writer
    }
}
impl<W: Write> Write for Indentor<W> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        for c in s.chars() {
            self.write_char(c)?;
        }
        Ok(())
    }
    fn write_char(&mut self, c: char) -> std::fmt::Result {
        if self.indent_next {
            self.indent_next = false;
            self.writer.write_str(
                &std::iter::repeat(' ')
                    .take(4 * self.level)
                    .collect::<String>(),
            )?;
        }
        if c == '\n' {
            self.indent_next = true;
        }
        self.writer.write_char(c)
    }
}

enum WordCase {
    CamelCase,
    #[allow(unused)]
    UpperCase,
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
                if let WordCase::UpperCase = out_word_case {
                    out.push(c.to_ascii_uppercase());
                } else {
                    out.push(c.to_ascii_lowercase());
                }
            }
        }
    }
    out
}
pub fn to_lower_snake_case(input: &str) -> String {
    convert_cases(input, true, WordCase::LowerCase)
}
#[allow(unused)]
pub fn to_upper_snake_case(input: &str) -> String {
    convert_cases(input, true, WordCase::UpperCase)
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

pub fn get_keyword_safe_ident(input: &str) -> String {
    let mut s = input.to_string();
    while KEYWORDS.contains(s.as_str()) {
        s.push('_');
    }
    s
}

pub fn iter_package_to_root(package: &str) -> impl Iterator<Item = &str> {
    std::iter::successors(Some(package), |package| {
        if package.is_empty() {
            None
        } else if let Some((remain, _)) = package.rsplit_once('.') {
            Some(remain)
        } else {
            Some("")
        }
    })
}

pub fn relative_path(cur_package: &str, dst_package: &str) -> Result<String> {
    let mut dst_iter = dst_package.split('.').filter(|p| !p.is_empty()).peekable();
    let mut cur_iter = cur_package.split('.').filter(|p| !p.is_empty()).peekable();
    while let (Some(p1), Some(p2)) = (dst_iter.peek(), cur_iter.peek()) {
        if *p1 == *p2 {
            dst_iter.next();
            cur_iter.next();
        } else {
            break;
        }
    }
    let super_count = cur_iter.count();
    let maybe_self = if super_count == 0 {
        Some(Cow::Borrowed("self"))
    } else {
        None
    }
    .into_iter();
    let supers = std::iter::repeat("super".into()).take(super_count);
    let mods = dst_iter.map(|s| get_keyword_safe_ident(&to_lower_snake_case(s)).into());
    Ok(
        Itertools::intersperse(maybe_self.chain(supers).chain(mods), "::".into())
            .collect::<String>(),
    )
}

// e.g. From <root mod>::simple::package::* to <root mod>::traits::package::*
pub fn relative_path_over_namespaces(dst_package: &str, dst_prefix: &str) -> Result<String> {
    let prefix = get_keyword_safe_ident(&to_lower_snake_case(dst_prefix));
    let dst_module = dst_package
        .split('.')
        .filter(|p| !p.is_empty())
        .map(|p| format!("::{}", get_keyword_safe_ident(&to_lower_snake_case(p))))
        .collect::<String>();
    Ok(format!(
        "puroro_root::{prefix}{dst_module}",
        prefix = prefix,
        dst_module = dst_module,
    ))
}

pub struct GenericParams(Vec<&'static str>);
impl GenericParams {
    pub fn push(mut self, lt: &'static str) -> Self {
        if !self.0.contains(&lt) {
            if lt.starts_with('\'') {
                if let Some((insert_pos, _)) =
                    self.0.iter().find_position(|&&s| !s.starts_with('\''))
                {
                    self.0.insert(insert_pos, lt);
                } else {
                    self.0.push(lt);
                }
            } else {
                self.0.push(lt);
            }
        }
        self
    }
    pub fn remove(self, lt: &'static str) -> Self {
        Self(self.0.into_iter().filter(move |s| *s != lt).collect())
    }
    pub fn replace(self, from: &'static str, to: &'static str) -> Self {
        Self(
            self.0
                .into_iter()
                .map(move |s| if s == from { to } else { s })
                .collect(),
        )
    }
}
impl FromIterator<&'static str> for GenericParams {
    fn from_iter<T: IntoIterator<Item = &'static str>>(iter: T) -> Self {
        Self(Vec::from_iter(iter))
    }
}
impl Display for GenericParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.0.iter();
        if let Some(first) = iter.next() {
            f.write_char('<')?;
            f.write_str(*first)?;
            for item in iter {
                f.write_str(", ")?;
                f.write_str(*item)?;
            }
            f.write_char('>')?;
        }
        Ok(())
    }
}
