use crate::se::{SerFieldToIoWrite, SerInternalDataToIoWrite};
use crate::{ErrorKind, FieldTypeGen, Result, SimpleImpl, StructInternalTypeGen};
use ::puroro::tags;

use super::LabelWrappedType;

// shorthand names
type Variant<X, V> = (X, tags::wire::Variant<V>);
type NonRepeatedVariant<X, V, _1, _2> = (tags::NonRepeated<_1, _2>, Variant<X, V>);

impl<X, V, _1, _2> SerFieldToIoWrite<NonRepeatedVariant<X, V, _1, _2>> for SimpleImpl
where
    Variant<X, V>: tags::NumericalFieldTypeTag,
    <Variant<X, V> as tags::NumericalFieldTypeTag>::NativeType:
        LabelWrappedType<tags::NonRepeated<_1, _2>>,
{
    fn ser_to_io_write<W>(
        field: &<Self as FieldTypeGen<NonRepeatedVariant<X, V, _1, _2>>>::Type,
        out: &mut W,
        internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> Result<()>
    where
        W: std::io::Write,
    {
        todo!()
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
