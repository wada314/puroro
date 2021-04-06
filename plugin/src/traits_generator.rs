use crate::plugin::*;
use crate::Result;
use ::itertools::Itertools;

fn generate_traits_for_descriptor(desc: &DescriptorProto) -> Result<String> {
    Ok(desc.name.clone())
}

pub(crate) fn generate_traits(cgreq: &CodeGeneratorRequest) -> Result<String> {
    cgreq
        .proto_file
        .iter()
        .map(|fdp| &fdp.message_type)
        .flatten()
        .map(|desc| generate_traits_for_descriptor(desc))
        .fold_ok(String::new(), |l, r| l + &r)
}
