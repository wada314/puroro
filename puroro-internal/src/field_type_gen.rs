use ::puroro::tags;

// MAPS???

pub trait FieldTypeGen<WireAndValueTag> {
    type Type;
}

// Non-repeated, numerical fields
impl<I> FieldTypeGen<(tags::Required, tags::Int32)> for I {
    type Type = i32;
}
impl<E, I> FieldTypeGen<(tags::Required, tags::Enum<E>)> for I {
    type Type = Result<E, i32>;
}

// Non-repeated, String/Bytes fields
impl FieldTypeGen<(tags::Required, tags::String)> for tags::SimpleStruct {
    type Type = String;
}

// Non-repeated, Message fields
impl<'slice, M> FieldTypeGen<(tags::Required, tags::Message<M>)> for tags::SliceView<'slice> {
    type Type = crate::SliceViewField<'slice>;
}
