use crate::generators::utils::*;
use crate::Result;
use itertools::Itertools;
use std::fmt::Write;

pub(crate) struct FileGeneratorContext<'a, 'b, W: Write> {
    writer: Indentor<'a, W>,
    package: Vec<&'b str>,
    path_to_package_root: String,
}
impl<'a, 'b, W: Write> FileGeneratorContext<'a, 'b, W> {
    pub(crate) fn new(writer: &'a mut W, package: Vec<&'b str>) -> Self {
        let path_to_package_root = Self::generate_path_to_package_root(&package);
        Self {
            writer: Indentor::new(writer),
            package,
            path_to_package_root,
        }
    }
    pub(crate) fn writer(&mut self) -> &mut Indentor<'a, W> {
        &mut self.writer
    }
    fn generate_path_to_package_root(package: &Vec<&str>) -> String {
        if package.is_empty() {
            "self".into()
        } else {
            let supers = std::iter::repeat("super").take(package.len());
            Itertools::intersperse(supers, "::").collect::<String>()
        }
    }

    pub(crate) fn indent<F: FnOnce(&mut FileGeneratorContext<'a, 'b, W>) -> Result<()>>(
        &mut self,
        f: F,
    ) -> Result<()> {
        self.writer.indent();
        let ret = (f)(self);
        self.writer.unindent();
        ret
    }

    pub(crate) fn enter_submessage_namespace<
        F: FnOnce(&mut FileGeneratorContext<'a, 'b, W>) -> Result<()>,
    >(
        &mut self,
        message_name: &'b str,
        f: F,
    ) -> Result<()> {
        self.package.push(message_name);
        self.path_to_package_root = Self::generate_path_to_package_root(&self.package);
        let ret = (f)(self);
        self.package.pop();
        self.path_to_package_root = Self::generate_path_to_package_root(&self.package);
        ret
    }
}
