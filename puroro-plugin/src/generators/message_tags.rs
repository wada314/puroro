use std::borrow::Cow;

use super::writer::IntoFragment;
use crate::utils::{relative_path_over_namespaces, Indentor};
use crate::wrappers::MessageDescriptor;
use crate::Result;

pub struct MessageTagCodeGenerator<'c> {
    msg: &'c MessageDescriptor<'c>,
}

impl<'c> MessageTagCodeGenerator<'c> {
    pub fn new(msg: &'c MessageDescriptor<'c>) -> Self {
        Self { msg }
    }

    pub fn print_msg_tag<W: std::fmt::Write>(&self, output: &mut Indentor<W>) -> Result<()> {
        (
            format!(
                "pub struct {ident}();\n\n",
                ident = self.msg.native_tag_ident()?
            ),
            format!(
                "\
impl ::puroro::MessageTag for {ident} {{
}}
\n",
                ident = self.msg.native_tag_ident()?,
            ),
        )
            .write_into(output)
    }

    pub fn tag_path_from_struct(&self, cur_package: &str) -> Result<Cow<'static, str>> {
        Ok(format!(
            "{module}::{ident}",
            module = relative_path_over_namespaces(self.msg.package()?, "tags")?,
            ident = self.msg.native_tag_ident()?,
        )
        .into())
    }
}
