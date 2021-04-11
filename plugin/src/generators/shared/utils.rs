use ::lazy_static::lazy_static;
use itertools::Itertools;
use std::{borrow::Cow, collections::HashSet, fmt::Write, rc::Rc};
pub(crate) struct Indentor<W> {
    writer: W,
    indent_next: bool,
    level: usize,
}
impl<W> Indentor<W> {
    pub(crate) fn new(writer: W) -> Self {
        Self {
            writer,
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
    pub(crate) fn into_inner(self) -> W {
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

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) struct PackagePath<'p>(Rc<Vec<&'p str>>);
impl<'p> PackagePath<'p> {
    pub(crate) fn new(package_str: &'p str) -> Self {
        Self(Rc::new(package_str.split('.').collect::<Vec<_>>()))
    }
    pub(crate) fn push(&mut self, subpackage: &'p str) {
        Rc::make_mut(&mut self.0).push(subpackage);
    }
    pub(crate) fn pop(&mut self) -> Option<&'p str> {
        Rc::make_mut(&mut self.0).pop()
    }
    pub(crate) fn from_origin(&self, origin_package: &PackagePath<'p>) -> PackagePath<'p> {
        let mut new_package = origin_package.0.clone();
        Rc::make_mut(&mut new_package).append(Rc::make_mut(&mut self.0.clone()));
        PackagePath(new_package)
    }
    pub(crate) fn iter(&self) -> impl '_ + Iterator<Item = &'p str> {
        self.0.iter().cloned()
    }
    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub(crate) struct MaybeFullyQualifiedTypeName<'p> {
    is_absolute: bool,
    package: PackagePath<'p>,
    name: &'p str,
}
impl<'p> MaybeFullyQualifiedTypeName<'p> {
    pub(crate) fn from_maybe_fq_typename(mut input: &'p str) -> Option<Self> {
        let mut is_absolute = false;
        if let Some(input_body) = input.strip_prefix('.') {
            input = input_body;
            is_absolute = true;
        }
        let mut package_and_name = PackagePath::new(input);
        if let Some(name) = package_and_name.pop() {
            Some(Self {
                is_absolute,
                package: package_and_name,
                name,
            })
        } else {
            None
        }
    }
    pub(crate) fn try_to_absolute(&self) -> Option<FullyQualifiedTypeName<'p>> {
        if self.is_absolute {
            Some(FullyQualifiedTypeName::new(self.package.clone(), self.name))
        } else {
            None
        }
    }
    pub(crate) fn with_package(
        &self,
        given_package: &PackagePath<'p>,
    ) -> FullyQualifiedTypeName<'p> {
        if self.is_absolute {
            FullyQualifiedTypeName::new(self.package.clone(), self.name)
        } else {
            FullyQualifiedTypeName::new(self.package.from_origin(given_package), self.name)
        }
    }
    pub(crate) fn to_native_maybe_qualified_typename(&self, path_to_package_root: &str) -> String {
        if self.is_absolute {
            let from_root = self
                .package
                .iter()
                .map(|s| to_module_name(s))
                .chain(std::iter::once(to_type_name(self.name)))
                .fold1(|s1, s2| s1 + "::" + &s2)
                .unwrap();
            path_to_package_root.to_string() + "::" + &from_root
        } else {
            self.name.to_string()
        }
    }
}
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub(crate) struct FullyQualifiedTypeName<'p> {
    package: PackagePath<'p>,
    name: &'p str,
}
impl<'p> FullyQualifiedTypeName<'p> {
    pub(crate) fn new(package: PackagePath<'p>, name: &'p str) -> Self {
        Self { package, name }
    }
    pub(crate) fn to_native_typename_from_root(&self) -> String {
        self.package
            .iter()
            .map(|s| to_module_name(s))
            .chain(std::iter::once(to_type_name(self.name)))
            .fold1(|s1, s2| s1 + "::" + &s2)
            .unwrap()
    }
    pub(crate) fn to_native_qualified_typename(&self, path_to_package_root: &str) -> String {
        let from_root = self.to_native_typename_from_root();
        path_to_package_root.to_string() + "::" + &from_root
    }
}
impl<'a> std::fmt::Display for FullyQualifiedTypeName<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for p in self.package.iter() {
            f.write_str(p)?;
            f.write_str(".")?;
        }
        f.write_str(self.name)?;
        Ok(())
    }
}

pub(crate) fn snake_case_to_camel_case(input: &str) -> String {
    let mut capitalize_next = true;
    input
        .chars()
        .filter_map(|c| {
            if c == '_' {
                capitalize_next = true;
                None
            } else {
                if capitalize_next {
                    capitalize_next = false;
                    Some(c.to_ascii_uppercase())
                } else {
                    Some(c.to_ascii_lowercase())
                }
            }
        })
        .collect()
}

pub(crate) fn camel_case_to_lower_snake_case(input: &str) -> String {
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
    get_keyword_safe_ident(&camel_case_to_lower_snake_case(input))
}

pub(crate) fn to_type_name(input: &str) -> String {
    get_keyword_safe_ident(&input)
}

pub(crate) fn to_var_name(input: &str) -> String {
    get_keyword_safe_ident(&camel_case_to_lower_snake_case(input))
}

pub(crate) fn to_enum_value_name(input: &str) -> String {
    get_keyword_safe_ident(&snake_case_to_camel_case(input))
}

// Note that we cannot use `std::path::Path` because protobuf compiler requires
// a slash-delimited path regardless of the OS they are running on.
pub(crate) fn package_to_file_path<'a>(package: &Vec<&'a str>) -> Vec<String> {
    package.iter().map(|p| to_module_name(*p)).collect()
}

pub(crate) fn to_string_literal(input: &str) -> String {
    let mut max_consecutive_hash = 0usize;
    let mut cur_consecutive_hash = 0usize;
    for char in input.chars() {
        if char == '#' {
            cur_consecutive_hash += 1;
            max_consecutive_hash = max_consecutive_hash.max(cur_consecutive_hash);
        } else {
            cur_consecutive_hash = 0;
        }
    }
    let hashes = std::iter::repeat('#')
        .take(max_consecutive_hash)
        .collect::<String>();
    format!(
        "r{hashes}\"{content}\"{hashes}",
        hashes = hashes,
        content = input
    )
}
