use itertools::Itertools;
use std::fmt::Write;

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
