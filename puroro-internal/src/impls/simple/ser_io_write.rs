use crate::de::DoDefaultCheck;
use crate::se::to_io_write::write_field_number_and_wire_type;
use crate::se::{SerFieldToIoWrite, SerInternalDataToIoWrite};
use crate::{ErrorKind, FieldTypeGen, Result, SimpleImpl, StructInternalTypeGen};
use ::puroro::tags;
use ::puroro::types::WireType;
use ::puroro::variant::{Variant, VariantTypeTag};

use super::LabelWrappedType;

type NonRepeatedVariant<X, V, _1, _2> = (tags::NonRepeated<_1, _2>, (X, tags::wire::Variant<V>));
impl<X, V, _1, _2> SerFieldToIoWrite<NonRepeatedVariant<X, V, _1, _2>> for SimpleImpl
where
    X: DoDefaultCheck,
    (X, tags::wire::Variant<V>): tags::NumericalFieldTypeTag + VariantTypeTag,
    <(X, tags::wire::Variant<V>) as tags::NumericalFieldTypeTag>::NativeType:
        LabelWrappedType<tags::NonRepeated<_1, _2>> + Clone,
    Self: FieldTypeGen<
        NonRepeatedVariant<X, V, _1, _2>,
        Type = <<(X, tags::wire::Variant<V>) as tags::NumericalFieldTypeTag>::NativeType as LabelWrappedType<
            tags::NonRepeated<_1, _2>,
        >>::Type,
    >,
{
    fn ser_to_io_write<W>(
        field: &<Self as FieldTypeGen<NonRepeatedVariant<X, V, _1, _2>>>::Type,
        field_number: i32,
        out: &mut W,
        _internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> Result<()>
    where
        W: std::io::Write,
    {
        if let Some(value) = <<(X, tags::wire::Variant<V>) as tags::NumericalFieldTypeTag>::NativeType as LabelWrappedType<
            tags::NonRepeated<_1, _2>,
        >>::get_opt(field) {
            let variant = Variant::from_native::<(X, tags::wire::Variant<V>)>(value.clone())?;
            if !<X as DoDefaultCheck>::VALUE && !variant.is_zero() {
                write_field_number_and_wire_type(out, field_number, WireType::Variant)?;
                variant.encode_bytes(out)?;
            }
        }
        Ok(())
    }
}

impl SerInternalDataToIoWrite for SimpleImpl {
    fn ser_to_io_write<W>(
        _out: &mut W,
        _internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> Result<()>
    where
        W: std::io::Write,
    {
        Ok(())
    }
}
