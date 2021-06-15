use crate::tags;

pub trait ApplyToFieldMut<LabelTag, WireAndValueTag, FieldType, ReturnType, ErrorType> {
    fn apply(&mut self, field: &mut FieldType) -> Result<ReturnType, ErrorType>;
}

macro_rules! define_apply_to_field_for_simple_message {
    ($(($ltag:ty, $vtag:ty, $actual:ty)),* $(,)?) => {
        pub trait ApplyToFieldMutForSimpleMessage<ReturnType, ErrorType>:
            $( ApplyToFieldMut <$ltag, $vtag, $actual, ReturnType, ErrorType> +)*
        {
        }
    };
}

define_apply_to_field_for_simple_message!(
    (tags::Required, tags::Int32, i32),
    (tags::Required, tags::UInt32, u32),
    (tags::Required, tags::SInt32, i32),
    (tags::Required, tags::Int64, i64),
    (tags::Required, tags::UInt64, u64),
    (tags::Required, tags::SInt64, i64),
    (tags::Required, tags::Bool, bool),
    (tags::Required, tags::String, String),
);
