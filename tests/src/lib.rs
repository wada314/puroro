#![cfg_attr(feature = "puroro-nightly", feature(backtrace))]
#![cfg_attr(feature = "puroro-nightly", feature(generic_associated_types))]
#![cfg_attr(feature = "puroro-nightly", feature(min_type_alias_impl_trait))]
#![allow(incomplete_features)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use ::puroro_internal::tags;

#[derive(Debug)]
pub struct CodeGeneratorResponse {
    pub error: ::std::option::Option<::std::string::String>,
    pub supported_features: ::std::option::Option<u64>,
    pub file: ::std::vec::Vec<::std::string::String>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

trait ApplyFieldMutFor<LabelTag, WireAndValueTag> {
    type FieldType;
    fn apply(&self, field: &mut Self::FieldType);
}

trait ApplyFieldMutForDefaultStruct:
    ApplyFieldMutFor<tags::Optional2, tags::String, FieldType = Option<String>>
{
}

trait MessageDefaultStruct {
    fn apply_to_field_mut<T>(&mut self, field_number: usize, f: T)
    where
        T: ApplyFieldMutForDefaultStruct;
}

impl MessageDefaultStruct for CodeGeneratorResponse {
    fn apply_to_field_mut<T>(&mut self, field_number: usize, f: T)
    where
        T: ApplyFieldMutForDefaultStruct,
    {
        match field_number {
            0 => {
                <T as ApplyFieldMutFor<tags::Optional2, tags::String>>::apply(&f, &mut self.error);
            }
            _ => (),
        }
    }
}

struct DoSomethingForOptional2String {}
impl ApplyFieldMutFor<tags::Optional2, tags::String> for DoSomethingForOptional2String {
    type FieldType = Option<String>;
    fn apply(&self, field: &mut Self::FieldType) {
        todo!()
    }
}
impl ApplyFieldMutForDefaultStruct for DoSomethingForOptional2String {}

#[cfg(test)]
mod tests {
    #[allow(unused)]
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    fn test(mut message: CodeGeneratorResponse, doer: DoSomethingForOptional2String) {
        <CodeGeneratorResponse as MessageDefaultStruct>::apply_to_field_mut(&mut message, 0, doer);
    }
}
