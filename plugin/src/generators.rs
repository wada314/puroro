pub mod shared;
//pub mod simple;

use crate::wrappers;

use super::Context;
struct Visitor {}
impl<'c> wrappers::DescriptorVisitor<'c> for Visitor {
    fn handle_msg(&mut self, msg: &'c wrappers::MessageDescriptor<'c>) -> crate::Result<()> {
        Ok(())
    }

    fn handle_enum(&mut self, enume: &'c wrappers::EnumDescriptor<'c>) -> crate::Result<()> {
        Ok(())
    }
}
pub fn do_generate<'c>(context: &'c Context<'c>) -> Vec<(String, String)> {
    let mut visitor = Visitor {};
    context.file_descriptors().for_each(|f| {
        f.visit_messages_and_enums_in_file(&mut visitor);
    });
    Vec::new()
}
