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

#[derive(Debug, Hash, PartialEq, Eq)]
pub(crate) struct FullyQualifiedTypeName<'a> {
    package: Vec<&'a str>,
    name: &'a str,
}
impl<'a> FullyQualifiedTypeName<'a> {
    pub(crate) fn from_typename(input: &'a str) -> Option<Self> {
        if input.chars().next() != Some('.') {
            return None;
        }
        let mut package = input[1..].split('.').collect::<Vec<_>>();
        Some(Self {
            name: package.pop().unwrap(),
            package,
        })
    }
    pub(crate) fn new(package: Vec<&'a str>, name: &'a str) -> Self {
        Self { package, name }
    }
    pub(crate) fn push(&mut self, name: &'a str) {
        self.package.push(name);
    }
    pub(crate) fn pop(&mut self) -> bool {
        self.package.pop().is_some()
    }
    pub(crate) fn to_typename_from_root(&self) -> String {
        self.package
            .iter()
            .map(|s| to_module_name(*s))
            .chain(std::iter::once(to_type_name(self.name)))
            .fold1(|s1, s2| s1 + "::" + &s2)
            .unwrap()
    }
    pub(crate) fn to_qualified_typename(&self, path_to_package_root: &Option<String>) -> String {
        let from_root = self.to_typename_from_root();
        if let Some(to_root) = path_to_package_root {
            to_root.clone() + "::" + &from_root
        } else {
            from_root
        }
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

pub(crate) fn camel_case_to_snake_case(input: &str) -> String {
    let mut lowered = input
        .chars()
        .flat_map(|c| {
            if c.is_ascii_uppercase() {
                Some('_')
            } else {
                None
            }
            .into_iter()
            .chain(std::iter::once(c.to_ascii_lowercase()))
        })
        .peekable();
    if lowered.peek() == Some(&'_') {
        lowered.next();
    }
    lowered.collect::<String>()
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

pub(crate) fn get_keyword_safe_ident(input: &str) -> String {
    let mut s = input.to_string();
    while KEYWORDS.contains(s.as_str()) {
        s.push('_');
    }
    s
}

pub(crate) fn to_module_name(input: &str) -> String {
    get_keyword_safe_ident(&camel_case_to_snake_case(input))
}

pub(crate) fn to_type_name(input: &str) -> String {
    get_keyword_safe_ident(&snake_case_to_camel_case(input))
}

pub(crate) fn to_var_name(input: &str) -> String {
    get_keyword_safe_ident(&camel_case_to_snake_case(input))
}

pub(crate) fn to_enum_value_name(input: &str) -> String {
    get_keyword_safe_ident(&snake_case_to_camel_case(input))
}
