use ::lazy_static::lazy_static;
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
