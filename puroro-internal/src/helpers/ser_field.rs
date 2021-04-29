use crate::tags::FieldTypeAndLabelTag;

pub trait SerializableField<T>
where
    T: FieldTypeAndLabelTag,
{
}
