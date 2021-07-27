use std::convert::TryInto;

use crate::de::DoDefaultCheck;
use crate::se::to_io_write::write_field_number_and_wire_type;
use crate::se::{
    SerEnumToIoWrite, SerEnumToIoWriteProxy, SerFieldToIoWrite, SerInternalDataToIoWrite,
    SerMsgToIoWrite, SerMsgToIoWriteProxy,
};
use crate::{
    ErrorKind, FieldTypeGen, Result, SerAnyFieldToIoWrite, SimpleImpl, StructInternalTypeGen,
};
use ::puroro::fixed_bits::{Bits32TypeTag, Bits64TypeTag};
use ::puroro::tags;
use ::puroro::types::WireType;
use ::puroro::variant::{EnumVariantTypeForSyntax, Variant, VariantTypeTag};
use ::puroro::SerToIoWrite;

use super::{LabelWrappedLdType, LabelWrappedMessageType, LabelWrappedType};

// All types
impl SerAnyFieldToIoWrite for SimpleImpl {}

// Non-repeated, variant
type VariantNativeType<V> = <tags::wire::Variant<V> as tags::NumericalTypeTag>::NativeType;
impl<X, V, _1, _2> SerFieldToIoWrite<X, tags::NonRepeated<_1, _2>, tags::wire::Variant<V>>
    for SimpleImpl
where
    (X, tags::NonRepeated<_1, _2>): DoDefaultCheck,
    tags::wire::Variant<V>: tags::NumericalTypeTag + VariantTypeTag,
    VariantNativeType<V>: Clone,
    tags::NonRepeated<_1, _2>: LabelWrappedType,
    Self: FieldTypeGen<
        X,
        tags::NonRepeated<_1, _2>,
        tags::wire::Variant<V>,
        Type = <tags::NonRepeated<_1, _2> as LabelWrappedType>::Type<VariantNativeType<V>>,
    >,
{
    fn ser_to_io_write<W>(
        field: &<Self as FieldTypeGen<X, tags::NonRepeated<_1, _2>, tags::wire::Variant<V>>>::Type,
        field_number: i32,
        out: &mut W,
        _internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> Result<()>
    where
        W: std::io::Write,
    {
        let do_default_check = <(X, tags::NonRepeated<_1, _2>) as DoDefaultCheck>::VALUE;
        if let Some(value) =
            <tags::NonRepeated<_1, _2> as LabelWrappedType>::iter::<VariantNativeType<V>>(field)
                .next()
        {
            let variant = Variant::from_native::<tags::wire::Variant<V>>(value.clone())?;
            if !do_default_check || !variant.is_zero() {
                write_field_number_and_wire_type(out, field_number, WireType::Variant)?;
                variant.encode_bytes(out)?;
            }
        }
        Ok(())
    }
}

// Repeated, variant
impl<X, V> SerFieldToIoWrite<X, tags::Repeated, tags::wire::Variant<V>> for SimpleImpl
where
    tags::wire::Variant<V>: tags::NumericalTypeTag + VariantTypeTag,
    VariantNativeType<V>: Clone,
    Self: FieldTypeGen<
        X,
        tags::Repeated,
        tags::wire::Variant<V>,
        Type = <tags::Repeated as LabelWrappedType>::Type<VariantNativeType<V>>,
    >,
{
    fn ser_to_io_write<W>(
        field: &<Self as FieldTypeGen<X, tags::Repeated, tags::wire::Variant<V>>>::Type,
        field_number: i32,
        out: &mut W,
        _internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> Result<()>
    where
        W: std::io::Write,
    {
        let mut iter =
            <tags::Repeated as LabelWrappedType>::iter::<VariantNativeType<V>>(field).peekable();
        if iter.peek().is_some() {
            let mut buffer: Vec<u8> = Vec::new();
            for val in iter {
                let variant = Variant::from_native::<tags::wire::Variant<V>>(val.clone())?;
                variant.encode_bytes(&mut buffer)?;
            }
            let total_len: i32 = buffer
                .len()
                .try_into()
                .map_err(|_| ErrorKind::TooLongToSerialize)?;

            write_field_number_and_wire_type(out, field_number, WireType::LengthDelimited)?;
            Variant::from_i32(total_len)?.encode_bytes(out)?;
            out.write_all(&buffer)?;
        }
        Ok(())
    }
}

// Bits32
type Bits32NativeType<V> = <tags::wire::Bits32<V> as tags::NumericalTypeTag>::NativeType;
impl<L, X, V> SerFieldToIoWrite<X, L, tags::wire::Bits32<V>> for SimpleImpl
where
    (X, L): DoDefaultCheck,
    tags::wire::Bits32<V>: tags::NumericalTypeTag + Bits32TypeTag,
    Bits32NativeType<V>: Clone,
    L: LabelWrappedType,
    Self: FieldTypeGen<
        X,
        L,
        tags::wire::Bits32<V>,
        Type = <L as LabelWrappedType>::Type<Bits32NativeType<V>>,
    >,
{
    fn ser_to_io_write<W>(
        field: &<Self as FieldTypeGen<X, L, tags::wire::Bits32<V>>>::Type,
        field_number: i32,
        out: &mut W,
        _internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> Result<()>
    where
        W: std::io::Write,
    {
        let do_default_check = <(X, L) as DoDefaultCheck>::VALUE;
        for item in <L as LabelWrappedType>::iter(field) {
            if do_default_check && item.clone() == Default::default() {
                continue;
            }
            write_field_number_and_wire_type(out, field_number, WireType::Bits32)?;
            let bytes = <tags::wire::Bits32<V> as Bits32TypeTag>::into_array(item.clone());
            out.write_all(&bytes)?;
        }
        Ok(())
    }
}

// Bits64
type Bits64NativeType<V> = <tags::wire::Bits64<V> as tags::NumericalTypeTag>::NativeType;
impl<L, X, V> SerFieldToIoWrite<X, L, tags::wire::Bits64<V>> for SimpleImpl
where
    (X, L): DoDefaultCheck,
    tags::wire::Bits64<V>: tags::NumericalTypeTag + Bits64TypeTag,
    Bits64NativeType<V>: Clone,
    L: LabelWrappedType,
    Self: FieldTypeGen<
        X,
        L,
        tags::wire::Bits64<V>,
        Type = <L as LabelWrappedType>::Type<Bits64NativeType<V>>,
    >,
{
    fn ser_to_io_write<W>(
        field: &<Self as FieldTypeGen<X, L, tags::wire::Bits64<V>>>::Type,
        field_number: i32,
        out: &mut W,
        _internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> Result<()>
    where
        W: std::io::Write,
    {
        let do_default_check = <(X, L) as DoDefaultCheck>::VALUE;
        for item in <L as LabelWrappedType>::iter(field) {
            if do_default_check && item.clone() == Default::default() {
                continue;
            }
            write_field_number_and_wire_type(out, field_number, WireType::Bits64)?;
            let bytes = <tags::wire::Bits64<V> as Bits64TypeTag>::into_array(item.clone());
            out.write_all(&bytes)?;
        }
        Ok(())
    }
}

// Bytes
impl<L, X> SerFieldToIoWrite<X, L, tags::Bytes> for SimpleImpl
where
    (X, L): DoDefaultCheck + LabelWrappedLdType,
    Self: FieldTypeGen<X, L, tags::Bytes, Type = <(X, L) as LabelWrappedLdType>::Type<[u8]>>,
{
    fn ser_to_io_write<W>(
        field: &<Self as FieldTypeGen<X, L, tags::Bytes>>::Type,
        field_number: i32,
        out: &mut W,
        _internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> Result<()>
    where
        W: std::io::Write,
    {
        let do_default_check = <(X, L) as DoDefaultCheck>::VALUE;
        for item in <(X, L) as LabelWrappedLdType>::iter::<[u8]>(field) {
            if do_default_check && item.is_empty() {
                continue;
            }
            write_field_number_and_wire_type(out, field_number, WireType::LengthDelimited)?;
            let length: i32 = item
                .len()
                .try_into()
                .map_err(|_| ErrorKind::TooLongToSerialize)?;
            Variant::from_i32(length)?.encode_bytes(out)?;
            out.write_all(item)?;
        }
        Ok(())
    }
}

// Strings
impl<L, X> SerFieldToIoWrite<X, L, tags::String> for SimpleImpl
where
    (X, L): DoDefaultCheck + LabelWrappedLdType,
    Self: FieldTypeGen<X, L, tags::String, Type = <(X, L) as LabelWrappedLdType>::Type<str>>,
{
    fn ser_to_io_write<W>(
        field: &<Self as FieldTypeGen<X, L, tags::String>>::Type,
        field_number: i32,
        out: &mut W,
        _internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> Result<()>
    where
        W: std::io::Write,
    {
        let do_default_check = <(X, L) as DoDefaultCheck>::VALUE;
        for item in <(X, L) as LabelWrappedLdType>::iter::<str>(field) {
            if do_default_check && item.is_empty() {
                continue;
            }
            write_field_number_and_wire_type(out, field_number, WireType::LengthDelimited)?;
            let length: i32 = item
                .len()
                .try_into()
                .map_err(|_| ErrorKind::TooLongToSerialize)?;
            Variant::from_i32(length)?.encode_bytes(out)?;
            out.write_all(item.as_bytes())?;
        }
        Ok(())
    }
}

// Enum
type EnumNativeType<X, E> = <X as tags::EnumTypeForSyntax>::NativeType<E>;
impl<X, _1, _2> SerEnumToIoWriteProxy<X, tags::NonRepeated<_1, _2>> for SimpleImpl
where
    Self: StructInternalTypeGen,
    tags::NonRepeated<_1, _2>: LabelWrappedType,
    X: EnumVariantTypeForSyntax,
    (X, tags::NonRepeated<_1, _2>): DoDefaultCheck,
{
    type SerEnum<E>
    where
        E: PartialEq + Clone,
        i32: From<E>,
        EnumNativeType<X, E>: Clone,
    = Self;
}
impl<X> SerEnumToIoWriteProxy<X, tags::Repeated> for SimpleImpl
where
    Self: StructInternalTypeGen,
    tags::Repeated: LabelWrappedType,
    X: EnumVariantTypeForSyntax,
    (X, tags::Repeated): DoDefaultCheck,
{
    type SerEnum<E>
    where
        E: PartialEq + Clone,
        i32: From<E>,
        EnumNativeType<X, E>: Clone,
    = Self;
}

#[rustfmt::skip]
impl<X, _1, _2, E, EnumFieldType, InternalDataType> SerEnumToIoWrite<X, tags::NonRepeated<_1, _2>, E, EnumFieldType, InternalDataType> for SimpleImpl
where
    E: PartialEq + Clone,
    i32: From<E>,
    tags::NonRepeated<_1, _2>: LabelWrappedType<Type<EnumNativeType<X, E>> = EnumFieldType>,
    X: EnumVariantTypeForSyntax,
    <X as tags::EnumTypeForSyntax>::NativeType<E>: Clone,
    (X, tags::NonRepeated<_1, _2>): DoDefaultCheck,
{
    fn ser_to_io_write<W>(
        field: &EnumFieldType,
        field_number: i32,
        out: &mut W,
        _internal_data: &InternalDataType,
    ) -> Result<()>
    where
        W: std::io::Write,
    {
        let do_default_check = <(X, tags::NonRepeated<_1, _2>) as DoDefaultCheck>::VALUE;
        if let Some(value) = <tags::NonRepeated<_1, _2> as LabelWrappedType>::iter(field).next() {
            let variant = Variant::from_enum::<X, E>(<<X as tags::EnumTypeForSyntax>::NativeType<E> as Clone>::clone(value))?;
            if !do_default_check || !variant.is_zero() {
                write_field_number_and_wire_type(out, field_number, WireType::Variant)?;
                variant.encode_bytes(out)?;
            }
        }
        Ok(())
    }
}
#[rustfmt::skip]
impl<X, E, EnumFieldType, InternalDataType> SerEnumToIoWrite<X, tags::Repeated, E, EnumFieldType, InternalDataType> for SimpleImpl
where
    E: PartialEq + Clone,
    i32: From<E>,
    tags::Repeated: LabelWrappedType<Type<EnumNativeType<X, E>> = EnumFieldType>,
    X: EnumVariantTypeForSyntax,
    <X as tags::EnumTypeForSyntax>::NativeType<E>: Clone,
    (X, tags::Repeated): DoDefaultCheck,
{
    fn ser_to_io_write<W>(
        field: &EnumFieldType,
        field_number: i32,
        out: &mut W,
        _internal_data: &InternalDataType,
    ) -> Result<()>
    where
        W: std::io::Write,
        E: PartialEq,
    {
        let iter = <tags::Repeated as LabelWrappedType>::iter(field);
        let mut buffer: Vec<u8> = Vec::new();
        for val in iter {
            let variant = Variant::from_enum::<X, E>(val.clone())?;
            variant.encode_bytes(&mut buffer)?;
        }
        let total_len: i32 = buffer
            .len()
            .try_into()
            .map_err(|_| ErrorKind::TooLongToSerialize)?;

        if total_len != 0 {
            write_field_number_and_wire_type(out, field_number, WireType::LengthDelimited)?;
            Variant::from_i32(total_len)?.encode_bytes(out)?;
            out.write_all(&buffer)?;
        }
        Ok(())
    }
}

// Message
impl<X, L> SerMsgToIoWriteProxy<X, L> for SimpleImpl
where
    Self: StructInternalTypeGen,
    L: LabelWrappedMessageType,
{
    type SerMsg<M>
    where
        M: SerToIoWrite + Clone,
    = Self;
}
#[rustfmt::skip]
impl<X, L, M, MsgFieldType, InternalDataType> SerMsgToIoWrite<X, L, M, MsgFieldType, InternalDataType> for SimpleImpl
where
    L: LabelWrappedMessageType<Type<M> = MsgFieldType>,
    M: SerToIoWrite + Clone,
{
    fn ser_to_io_write<W>(
        field: &MsgFieldType,
        field_number: i32,
        out: &mut W,
        _internal_data: &InternalDataType,
    ) -> Result<()>
    where
        W: std::io::Write,
    {
        use std::ops::Deref as _;
        for boxed in <L as LabelWrappedMessageType>::iter(field) {
            write_field_number_and_wire_type(out, field_number, WireType::LengthDelimited)?;
            let mut buffer: Vec<u8> = Vec::new();
            <M as SerToIoWrite>::ser(boxed.deref(), &mut buffer)?;
            let length: i32 = buffer
                .len()
                .try_into()
                .map_err(|_| ErrorKind::TooLongToSerialize)?;
            Variant::from_i32(length)?.encode_bytes(out)?;
            out.write_all(&buffer)?;
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
