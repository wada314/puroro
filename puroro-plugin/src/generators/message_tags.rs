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
        let ident = format!("{}Tag", self.msg.native_ident()?);
        (
            format!("pub struct {ident}();\n\n", ident = ident),
            format!(
                "\
impl ::puroro::MessageTag for {ident} {{
}}
\n",
                ident = ident,
            ),
        )
            .write_into(output)
    }

    pub fn tag_ident(&self, msg: &'c MessageDescriptor<'c>) -> Result<Cow<'static, str>> {
        Ok(format!("{}Tag", msg.native_ident()?).into())
    }

    pub fn tag_path_from_struct(&self, cur_package: &str) -> Result<Cow<'static, str>> {
        Ok(format!(
            "{module}::{ident}",
            module = relative_path_over_namespaces(cur_package, self.msg.package()?, "tags")?,
            ident = self.tag_ident(self.msg)?,
        )
        .into())
    }
}
