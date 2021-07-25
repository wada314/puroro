use super::{LabelWrappedLdType, LabelWrappedMessageType, LabelWrappedType, SimpleImpl};
use crate::de::from_iter::{deser_from_scoped_iter, ScopedIter, Variants};
use crate::de::{
    DeserAnyFieldFromBytesIter, DeserEnumFromBytesIter, DeserEnumFromBytesIterProxy,
    DeserFieldFromBytesIter, DeserInternalDataFromBytesIter, DeserMsgFromBytesIter,
    DeserMsgFromBytesIterProxy, DoDefaultCheck, MessageFromBytesIter,
};
use crate::{EnumTypeGen, FieldTypeGen, MessageInternal, MsgTypeGen, StructInternalTypeGen};
use ::itertools::Itertools;
use ::puroro::fixed_bits::{Bits32TypeTag, Bits64TypeTag};
use ::puroro::types::FieldData;
use ::puroro::variant::{EnumVariantTypeForSyntax, VariantTypeTag};
use ::puroro::{tags, ErrorKind, Result};
use ::std::convert::TryFrom;

// deser from iterator
impl DeserAnyFieldFromBytesIter for SimpleImpl {}

// deser from iterator, into variant type fields
type VariantNativeType<V> = <tags::wire::Variant<V> as tags::NumericalFieldTypeTag>::NativeType;
impl<L, V, X> DeserFieldFromBytesIter<X, L, tags::wire::Variant<V>> for SimpleImpl
where
    (X, L): DoDefaultCheck,
    tags::wire::Variant<V>: tags::NumericalFieldTypeTag + VariantTypeTag,
    VariantNativeType<V>: PartialEq,
    L: LabelWrappedType,
    Self: FieldTypeGen<
        X,
        L,
        tags::wire::Variant<V>,
        Type = <L as LabelWrappedType>::Type<VariantNativeType<V>>,
    >,
{
    fn deser_from_scoped_bytes_iter<I>(
        field: &mut <Self as FieldTypeGen<X, L, tags::wire::Variant<V>>>::Type,
        data: FieldData<&mut ScopedIter<I>>,
        _internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
    {
        let do_default_check = <(X, L) as DoDefaultCheck>::VALUE;
        match data {
            FieldData::Variant(variant) => {
                let native = variant.to_native::<tags::wire::Variant<V>>()?;
                if !do_default_check || native != Default::default() {
                    *<L as LabelWrappedType>::get_or_insert_with(field, Default::default) = native;
                }
            }
            FieldData::LengthDelimited(iter) => {
                let variants_iter = Variants::new(iter);
                let values_iter = variants_iter
                    .map(|rv| rv.and_then(|v| v.to_native::<tags::wire::Variant<V>>()))
                    .filter_ok(|val| {
                        !do_default_check || *val != <VariantNativeType<V> as Default>::default()
                    });
                <L as LabelWrappedType>::extend(field, values_iter)?;
            }
            FieldData::Bits32(_) | FieldData::Bits64(_) => Err(ErrorKind::UnexpectedWireType)?,
        };
        Ok(())
    }
}

// Bits32
type Bits32NativeType<V> = <tags::wire::Bits32<V> as tags::NumericalFieldTypeTag>::NativeType;
impl<L, V, X> DeserFieldFromBytesIter<X, L, tags::wire::Bits32<V>> for SimpleImpl
where
    (X, L): DoDefaultCheck,
    tags::wire::Bits32<V>: tags::NumericalFieldTypeTag + Bits32TypeTag,
    Bits32NativeType<V>: PartialEq,
    L: LabelWrappedType,
    Self: FieldTypeGen<
        X,
        L,
        tags::wire::Bits32<V>,
        Type = <L as LabelWrappedType>::Type<Bits32NativeType<V>>,
    >,
{
    fn deser_from_scoped_bytes_iter<I>(
        field: &mut <Self as FieldTypeGen<X, L, tags::wire::Bits32<V>>>::Type,
        data: FieldData<&mut ScopedIter<I>>,
        _internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
    {
        let do_default_check = <(X, L) as DoDefaultCheck>::VALUE;
        match data {
            FieldData::Bits32(bytes) => {
                let native = <tags::wire::Bits32<V> as Bits32TypeTag>::from_array(bytes);
                if !do_default_check || native != Default::default() {
                    *<L as LabelWrappedType>::get_or_insert_with(field, Default::default) = native;
                }
            }
            _ => Err(ErrorKind::UnexpectedWireType)?,
        };
        Ok(())
    }
}

// Bits64
type Bits64NativeType<V> = <tags::wire::Bits64<V> as tags::NumericalFieldTypeTag>::NativeType;
impl<L, V, X> DeserFieldFromBytesIter<X, L, tags::wire::Bits64<V>> for SimpleImpl
where
    (X, L): DoDefaultCheck,
    tags::wire::Bits64<V>: tags::NumericalFieldTypeTag + Bits64TypeTag,
    Bits64NativeType<V>: PartialEq,
    L: LabelWrappedType,
    Self: FieldTypeGen<
        X,
        L,
        tags::wire::Bits64<V>,
        Type = <L as LabelWrappedType>::Type<Bits64NativeType<V>>,
    >,
{
    fn deser_from_scoped_bytes_iter<I>(
        field: &mut <Self as FieldTypeGen<X, L, tags::wire::Bits64<V>>>::Type,
        data: FieldData<&mut ScopedIter<I>>,
        _internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
    {
        let do_default_check = <(X, L) as DoDefaultCheck>::VALUE;
        match data {
            FieldData::Bits64(bytes) => {
                let native = <tags::wire::Bits64<V> as Bits64TypeTag>::from_array(bytes);
                if !do_default_check || native != Default::default() {
                    *<L as LabelWrappedType>::get_or_insert_with(field, Default::default) = native;
                }
            }
            _ => Err(ErrorKind::UnexpectedWireType)?,
        };
        Ok(())
    }
}

// String
impl<L, X> DeserFieldFromBytesIter<X, L, tags::String> for SimpleImpl
where
    (X, L): DoDefaultCheck + LabelWrappedLdType,
    Self: FieldTypeGen<X, L, tags::String, Type = <(X, L) as LabelWrappedLdType>::Type<str>>,
{
    fn deser_from_scoped_bytes_iter<I>(
        field: &mut <Self as FieldTypeGen<X, L, tags::String>>::Type,
        data: FieldData<&mut ScopedIter<I>>,
        _internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
    {
        let do_default_check = <(X, L) as DoDefaultCheck>::VALUE;
        match data {
            FieldData::LengthDelimited(iter) => {
                let string = String::from_utf8(iter.collect::<::std::io::Result<Vec<_>>>()?)
                    .map_err(|e| ErrorKind::InvalidUtf8(e))?;
                if !do_default_check || !string.is_empty() {
                    *<(X, L) as LabelWrappedLdType>::get_or_insert_default(field) = string;
                }
            }
            _ => Err(ErrorKind::UnexpectedWireType)?,
        }
        Ok(())
    }
}

// Bytes
impl<L, X> DeserFieldFromBytesIter<X, L, tags::Bytes> for SimpleImpl
where
    (X, L): DoDefaultCheck + LabelWrappedLdType,
    Self: FieldTypeGen<X, L, tags::Bytes, Type = <(X, L) as LabelWrappedLdType>::Type<[u8]>>,
{
    fn deser_from_scoped_bytes_iter<I>(
        field: &mut <Self as FieldTypeGen<X, L, tags::Bytes>>::Type,
        data: FieldData<&mut ScopedIter<I>>,
        _internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
    {
        let do_default_check = <(X, L) as DoDefaultCheck>::VALUE;
        match data {
            FieldData::LengthDelimited(iter) => {
                let bytes = iter.collect::<::std::io::Result<Vec<_>>>()?;
                if !do_default_check || !bytes.is_empty() {
                    *<(X, L) as LabelWrappedLdType>::get_or_insert_default(field) = bytes;
                }
            }
            _ => Err(ErrorKind::UnexpectedWireType)?,
        }
        Ok(())
    }
}

// Enum
type EnumNativeType<X, E> = <X as tags::EnumFieldTypeForSyntax>::NativeType<E>;
impl<X, L> DeserEnumFromBytesIterProxy<X, L> for SimpleImpl
where
    Self: StructInternalTypeGen,
    L: LabelWrappedType,
    X: tags::EnumFieldTypeForSyntax + EnumVariantTypeForSyntax,
    (X, L): DoDefaultCheck,
{
    type DeserEnum<E>
    where
        E: Default + TryFrom<i32> + PartialEq,
    = Self;
}

#[rustfmt::skip] // Self: EnumTypeGen<..., EnumType<E> = ...> 's <E> is removed by rustfmt 
impl<X, L, E> DeserEnumFromBytesIter<X, L, E> for SimpleImpl
where
    Self: EnumTypeGen<
        X,
        L,
        EnumType<E> = <L as LabelWrappedType>::Type<<X as tags::EnumFieldTypeForSyntax>::NativeType<E>>
    >,
    (X, L): DoDefaultCheck,
    X: tags::EnumFieldTypeForSyntax + EnumVariantTypeForSyntax,
    L: LabelWrappedType,
    E: Default + TryFrom<i32> + PartialEq,
{
    fn deser_from_scoped_bytes_iter<I>(
        field: &mut <Self as EnumTypeGen<X, L>>::EnumType<E>,
        data: FieldData<&mut ScopedIter<I>>,
        _internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
    {
        let do_default_check = <(X, L) as DoDefaultCheck>::VALUE;
        let default = <X as tags::EnumFieldTypeForSyntax>::default;
        match data {
            FieldData::Variant(variant) => {
                let native = variant.to_enum::<X, E>()?;
                if !do_default_check || native != default() {
                    *<L as LabelWrappedType>::get_or_insert_with::<_, EnumNativeType<X, E>>(
                        field, default,
                    ) = native;
                }
            }
            FieldData::LengthDelimited(iter) => {
                let variants_iter = Variants::new(iter);
                let values_iter = variants_iter
                    .map(|rv| rv.and_then(|v| v.to_enum::<X, E>()))
                    .filter_ok(|val| !do_default_check || *val != default());
                <L as LabelWrappedType>::extend(field, values_iter)?;
            }
            FieldData::Bits32(_) | FieldData::Bits64(_) => Err(ErrorKind::UnexpectedWireType)?,
        };
        Ok(())
    }
}

// Message
impl<X, L> DeserMsgFromBytesIterProxy<X, L> for SimpleImpl
where
    Self: StructInternalTypeGen,
    L: LabelWrappedMessageType,
{
    type DeserMsg<M>
    where
        M: MessageFromBytesIter + MessageInternal<ImplTypeTag = Self>,
    = Self;
}

#[rustfmt::skip] // Self: MsgTypeGen<..., MsgType<M> = ...> 's <M> is removed by rustfmt
impl<X, L, M> DeserMsgFromBytesIter<X, L, M> for SimpleImpl
where
    Self: MsgTypeGen<
        X,
        L,
        MsgType<M> = <L as LabelWrappedMessageType>::Type<M>,
    > + StructInternalTypeGen<Type = ()>,
    M: MessageFromBytesIter + MessageInternal<ImplTypeTag = SimpleImpl>,
    L: LabelWrappedMessageType,
{
    fn deser_from_scoped_bytes_iter<I>(
        field: &mut <Self as MsgTypeGen<X, L>>::MsgType<M>,
        data: FieldData<&mut ScopedIter<I>>,
        _internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
    {
        use ::std::ops::DerefMut;
        match data {
            FieldData::LengthDelimited(iter) => deser_from_scoped_iter(
                <L as LabelWrappedMessageType>::get_or_insert_with(
                    field,
                    || MessageInternal::new_with_internal_data(()),
                )
                .deref_mut(),
                iter,
            ),
            _ => Err(ErrorKind::UnexpectedWireType)?,
        }
    }
}

// Internal data
impl DeserInternalDataFromBytesIter for SimpleImpl {
    fn deser_from_scoped_bytes_iter<I>(
        _internal_data: &mut <Self as StructInternalTypeGen>::Type,
        _data: FieldData<&mut ScopedIter<I>>,
    ) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
    {
        Ok(())
    }
}
