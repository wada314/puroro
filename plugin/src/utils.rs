use ::lazy_static::lazy_static;
use itertools::Itertools;
use std::{collections::HashSet, fmt::Write};
pub(crate) struct Indentor<'a, W: Write> {
    write: &'a mut W,
    indent_next: bool,
    level: usize,
}
impl<'a, W: Write> Indentor<'a, W> {
    pub(crate) fn new(write: &'a mut W) -> Self {
        Self {
            write,
            indent_next: false,
            level: 0,
        }
    }
    pub(crate) fn indent(&mut self) {
        self.level += 1;
    }
    pub(crate) fn unindent(&mut self) {
        assert_ne!(0, self.level, "unindenting too much");
        self.level -= 1;
    }
}
impl<'a, W: Write> Write for Indentor<'a, W> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        for c in s.chars() {
            self.write_char(c)?;
        }
        Ok(())
    }
    fn write_char(&mut self, c: char) -> std::fmt::Result {
        if self.indent_next {
            self.indent_next = false;
            self.write.write_str(
                &std::iter::repeat(' ')
                    .take(4 * self.level)
                    .collect::<String>(),
            )?;
        }
        if c == '\n' {
            self.indent_next = true;
        }
        self.write.write_char(c)
    }
}

pub(crate) fn snake_case_to_camel_case(input: &str) -> String {
    input
        .chars()
        .scan(true, |capitalize_next, c| {
            if c == '_' {
                *capitalize_next = true;
                Some(c)
            } else {
                if *capitalize_next {
                    *capitalize_next = false;
                    Some(c.to_ascii_uppercase())
                } else {
                    Some(c)
                }
            }
        })
        .collect()
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

pub(crate) fn get_keywoard_safe_ident(input: &str) -> String {
    let mut s = input.to_string();
    while KEYWORDS.contains(s.as_str()) {
        s.push('_');
    }
    s
}
