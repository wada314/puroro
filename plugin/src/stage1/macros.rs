use ::puroro::tags;
use ::puroro::{PuroroError, Result};
use ::puroro_serializer::deserializer::stream::{Field, LengthDelimitedDeserializer};

// An (incomplete) transformation from a native type written in the macro to our `FieldTypeTag`.
pub(crate) trait NativeTypeToFieldTypeTag {
    type Tag: ::puroro::tags::FieldTypeTag;
}
impl NativeTypeToFieldTypeTag for String {
    type Tag = ::puroro::tags::String;
}
impl<T> NativeTypeToFieldTypeTag for Vec<T>
where
    T: NativeTypeToFieldTypeTag,
    T::Tag: tags::SingularFieldTypeTag,
{
    type Tag = ::puroro::tags::Repeated<T::Tag>;
}
pub(crate) trait DeserializableFromField {
    fn merge_from_field<D: LengthDelimitedDeserializer>(&mut self, field: Field<D>) -> Result<()>;
}
pub(crate) trait SerializableField {
    fn serialize<T: ::puroro_serializer::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
        field_number: usize,
    ) -> Result<()>;
}

// I need this for supporting `Default` for `Result` type...
pub(crate) trait MyDefault {
    fn default() -> Self;
}
impl<T> MyDefault for Option<T> {
    fn default() -> Self {
        None
    }
}
impl<T> MyDefault for Vec<T> {
    fn default() -> Self {
        Vec::new()
    }
}
impl<T> MyDefault for std::result::Result<T, i32>
where
    T: ::num_traits::FromPrimitive,
{
    fn default() -> Self {
        T::from_i32(<i32 as Default>::default()).ok_or(<i32 as Default>::default())
    }
}

macro_rules! define_variant_fields {
    ($native:ty, $tag:ty) => {
        impl DeserializableFromField for $native {
            fn merge_from_field<D: LengthDelimitedDeserializer>(
                &mut self,
                field: Field<D>,
            ) -> Result<()> {
                match field {
                    Field::Variant(variant) => {
                        *self = variant.to_native::<$tag>()?;
                    }
                    Field::LengthDelimited(ldd) => {
                        *self = ldd
                            .deserialize_as_variants()
                            .last()
                            .unwrap_or(Err(PuroroError::ZeroLengthPackedField))?
                            .to_native::<$tag>()?;
                    }
                    _ => {
                        Err(PuroroError::UnexpectedWireType)?;
                    }
                }
                Ok(())
            }
        }

        impl DeserializableFromField for Vec<$native> {
            fn merge_from_field<D: LengthDelimitedDeserializer>(
                &mut self,
                field: Field<D>,
            ) -> Result<()> {
                if let Field::LengthDelimited(ldd) = field {
                    let mut vec = ldd
                        .deserialize_as_variants()
                        .map(|rv| rv.and_then(|v| v.to_native::<$tag>()))
                        .collect::<Result<Vec<_>>>()?;
                    self.append(&mut vec);
                    Ok(())
                } else {
                    Err(PuroroError::UnexpectedWireType)
                }
            }
        }

        impl MyDefault for $native {
            fn default() -> Self {
                Default::default()
            }
        }

        impl SerializableField for $native {
            fn serialize<T>(&self, serializer: &mut T, field_number: usize) -> ::puroro::Result<()>
            where
                T: ::puroro_serializer::serializer::MessageSerializer,
            {
                if *self != <$native as Default>::default() {
                    serializer.serialize_variant::<$tag>(field_number, *self)?;
                }
                Ok(())
            }
        }
        impl SerializableField for Vec<$native> {
            fn serialize<T>(&self, serializer: &mut T, field_number: usize) -> ::puroro::Result<()>
            where
                T: ::puroro_serializer::serializer::MessageSerializer,
            {
                serializer
                    .serialize_variants_twice::<$tag, _>(field_number, self.iter().map(|&v| Ok(v)))
            }
        }
    };
}
define_variant_fields!(i32, tags::Int32);
define_variant_fields!(i64, tags::Int64);
define_variant_fields!(u32, tags::UInt32);
define_variant_fields!(u64, tags::UInt64);
define_variant_fields!(bool, tags::Bool);
impl DeserializableFromField for String {
    fn merge_from_field<D: LengthDelimitedDeserializer>(&mut self, field: Field<D>) -> Result<()> {
        if let Field::LengthDelimited(ldd) = field {
            *self = ldd.deserialize_as_chars().collect::<Result<String>>()?;
            Ok(())
        } else {
            Err(PuroroError::UnexpectedWireType)
        }
    }
}
impl DeserializableFromField for Vec<String> {
    fn merge_from_field<D: LengthDelimitedDeserializer>(&mut self, field: Field<D>) -> Result<()> {
        if let Field::LengthDelimited(ldd) = field {
            self.push(ldd.deserialize_as_chars().collect::<Result<String>>()?);
            Ok(())
        } else {
            Err(PuroroError::UnexpectedWireType)
        }
    }
}
impl MyDefault for String {
    fn default() -> Self {
        Default::default()
    }
}
impl SerializableField for String {
    fn serialize<T: puroro_serializer::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
        field_number: usize,
    ) -> Result<()> {
        if *self != <String as Default>::default() {
            serializer.serialize_bytes_twice(field_number, self.bytes().map(|b| Ok(b)))?;
        }
        Ok(())
    }
}
impl SerializableField for Vec<String> {
    fn serialize<T: puroro_serializer::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
        field_number: usize,
    ) -> Result<()> {
        for string in self {
            serializer.serialize_bytes_twice(field_number, string.bytes().map(|b| Ok(b)))?;
        }
        Ok(())
    }
}

macro_rules! proto_struct {
    () => {};

    (struct $structname:ident { $($fname:ident: $ftype:ty = $fid:expr ,)* } $($rest:tt)*) => {
        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone)]
        pub(crate) struct $structname {$(
            pub(crate) $fname: $ftype,
        )*}
        // Impl for creating a new message instance
        impl ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for $structname {
            type Target = Self;
            fn finish(self) -> ::puroro::Result<Self::Target> { Ok(self) }

            #[allow(unused_variables)]
            fn met_field<T: ::puroro_serializer::deserializer::stream::LengthDelimitedDeserializer>(
                &mut self,
                field: ::puroro_serializer::deserializer::stream::Field<T>,
                field_number: usize,
            ) -> ::puroro::Result<()> {
                #[allow(unused)]
                use $crate::stage1::macros::DeserializableFromField;
                match field_number {
                    $($fid => {
                        self.$fname.merge_from_field(field)?;
                    })*
                    _ => {
                        /* ignore! lol */
                        if let ::puroro_serializer::deserializer::stream::Field::LengthDelimited(ldd) = field {
                            // Eat the input iterator anyway
                            ldd.leave_as_unknown()?;
                        }
                    }
                };
                Ok(())
            }
        }
        // impl for merging a message into existing instance
        impl ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for &mut $structname {
            type Target = Self;
            fn finish(self) -> ::puroro::Result<Self::Target> { Ok(self) }
            fn met_field<T: ::puroro_serializer::deserializer::stream::LengthDelimitedDeserializer>(
                &mut self,
                field: ::puroro_serializer::deserializer::stream::Field<T>,
                field_number: usize,
            ) -> ::puroro::Result<()> {
                <$structname>::met_field(*self, field, field_number)
            }
        }
        impl ::puroro::Deserializable for $structname {
            fn from_bytes<I: Iterator<Item = std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
                use ::puroro_serializer::deserializer::stream::Deserializer;
                let deserializer = ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter);
                let value = $structname::default();
                deserializer.deserialize(value)
            }
        }
        impl $crate::stage1::macros::DeserializableFromField for Option<$structname> {
            fn merge_from_field<D>(&mut self, field: ::puroro_serializer::deserializer::stream::Field<D>) -> ::puroro::Result<()>
            where
                D: ::puroro_serializer::deserializer::stream::LengthDelimitedDeserializer
            {
                if let ::puroro_serializer::deserializer::stream::Field::LengthDelimited(ldd) = field {
                    let msg = self.get_or_insert_with($structname::default);
                    ldd.deserialize_as_message(msg)?;
                    Ok(())
                } else {
                    Err(::puroro::PuroroError::UnexpectedWireType)
                }
            }
        }
        impl $crate::stage1::macros::DeserializableFromField for Vec<$structname> {
            fn merge_from_field<D>(&mut self, field: ::puroro_serializer::deserializer::stream::Field<D>) -> ::puroro::Result<()>
            where
                D: ::puroro_serializer::deserializer::stream::LengthDelimitedDeserializer
            {
                if let ::puroro_serializer::deserializer::stream::Field::LengthDelimited(ldd) = field {
                    self.push(ldd.deserialize_as_message($structname::default())?);
                    Ok(())
                } else {
                    Err(::puroro::PuroroError::UnexpectedWireType)
                }
            }
        }
        impl ::puroro_serializer::serializer::Serializable for $structname {
            #[allow(unused_variables)]
            fn serialize<T: puroro_serializer::serializer::MessageSerializer>(
                &self,
                serializer: &mut T,
            ) -> ::puroro::Result<()> {
                $(
                    $crate::stage1::macros::SerializableField::serialize(
                        &self.$fname, serializer, $fid
                    )?;
                )*
                Ok(())
            }
        }
        impl ::puroro::Serializable for $structname {
            fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
                ::puroro_serializer::serializer::Serializable::serialize(
                    self,
                    &mut ::puroro_serializer::serializer::default_serializer(write)
                )
            }
        }
        impl $crate::stage1::macros::SerializableField for Option<$structname> {
            fn serialize<T: puroro_serializer::serializer::MessageSerializer>(
                &self,
                serializer: &mut T,
                field_number: usize,
            ) -> ::puroro::Result<()> {
                if let Some(msg)= self {
                    serializer.serialize_message_twice(field_number, msg)?;
                }
                Ok(())
            }
        }
        impl $crate::stage1::macros::SerializableField for Vec<$structname> {
            fn serialize<T: puroro_serializer::serializer::MessageSerializer>(
                &self,
                serializer: &mut T,
                field_number: usize,
            ) -> ::puroro::Result<()> {
                for msg in self {
                    serializer.serialize_message_twice(field_number, msg)?;
                }
                Ok(())
            }
        }
        impl Default for $structname {
            fn default() -> Self {
                Self {$(
                    $fname: $crate::stage1::macros::MyDefault::default(),
                )*}
            }
        }

        proto_struct!{$($rest)*}
    };

    (enum $enumname:ident { $($ename:ident = $evalue:expr ,)* } $($rest:tt)* ) => {
        #[allow(non_camel_case_types)]
        #[derive(::num_derive::FromPrimitive)]
        #[derive(Debug, Clone, Copy)]
        pub(crate) enum $enumname {
            $(
                #[allow(non_camel_case_types)]
                $ename = $evalue
            ),*,
        }
        impl Default for $enumname {
            fn default() -> Self {
                // 0th of the all-element tuple
                ( $($enumname::$ename),* ).0
            }
        }
        impl $crate::stage1::macros::DeserializableFromField for Result<$enumname, i32> {
            fn merge_from_field<D>(&mut self, field: ::puroro_serializer::deserializer::stream::Field<D>) -> ::puroro::Result<()>
            where
                D: ::puroro_serializer::deserializer::stream::LengthDelimitedDeserializer
            {
                use ::num_traits::FromPrimitive;
                let mut integer: i32 = 0;
                integer.merge_from_field(field)?;
                *self = $enumname::from_i32(integer).ok_or(integer);
                Ok(())
            }
        }
        impl $crate::stage1::macros::DeserializableFromField for Vec<Result<$enumname, i32>> {
            fn merge_from_field<D>(&mut self, field: ::puroro_serializer::deserializer::stream::Field<D>) -> ::puroro::Result<()>
            where
                D: ::puroro_serializer::deserializer::stream::LengthDelimitedDeserializer
            {
                use ::num_traits::FromPrimitive;
                let mut integers: Vec<i32> = Vec::new();
                integers.merge_from_field(field)?;
                self.append(&mut integers.into_iter().map(|i| $enumname::from_i32(i).ok_or(i)).collect::<Vec<_>>());
                Ok(())
            }
        }
        impl $crate::stage1::macros::SerializableField for Result<$enumname, i32> {
            fn serialize<T: puroro_serializer::serializer::MessageSerializer>(
                &self,
                serializer: &mut T,
                field_number: usize,
            ) -> ::puroro::Result<()> {
                let integer = match self {
                    Ok(e) => (*e as i32),
                    Err(i) => *i,
                };
                serializer.serialize_variant::<::puroro::tags::Int32>(field_number, integer)?;
                Ok(())
            }
        }
        impl $crate::stage1::macros::SerializableField for Vec<Result<$enumname, i32>> {
            fn serialize<T: puroro_serializer::serializer::MessageSerializer>(
                &self,
                serializer: &mut T,
                field_number: usize,
            ) -> ::puroro::Result<()> {
                let iter = self.iter().map(|rresult|{
                    let integer = match rresult {
                        Ok(e) => (*e as i32),
                        Err(i) => *i,
                    };
                    Ok(integer)
                });
                serializer.serialize_variants_twice::<::puroro::tags::Int32, _>(field_number, iter)?;
                Ok(())
            }
        }
        proto_struct! { $($rest)* }
    };

}

#[cfg(test)]
mod tests {
    use ::puroro::Deserializable;
    use std::io::Read;
    #[test]
    fn test_encoding_sample1() {
        // https://developers.google.com/protocol-buffers/docs/encoding#simple
        // message Test1 {
        //   optional int32 a = 1;
        // }
        // a = 150
        let input: &[u8] = &[0x08, 0x96, 0x01];
        proto_struct! {
            struct Test1 {
                a: i32 = 1,
            }
        }
        let t1 = Test1::from_bytes(input.bytes()).unwrap();
        assert_eq!(150, t1.a);
    }

    #[test]
    fn test_encoding_sample2() {
        // https://developers.google.com/protocol-buffers/docs/encoding#strings
        // message Test2 {
        //   optional string b = 2;
        // }
        // b = "testing"
        let input: &[u8] = &[0x12, 0x07, 0x74, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67];
        proto_struct! {
            struct Test2 {
                b: String = 2,
            }
        }
        let t2 = Test2::from_bytes(input.bytes()).unwrap();
        assert_eq!("testing", t2.b);
    }

    #[test]
    fn test_encoding_sample3() {
        // https://developers.google.com/protocol-buffers/docs/encoding#embedded
        // message Test1 {
        //   optional int32 a = 1;
        // }
        // message Test3 {
        //   optional Test1 c = 3;
        // }
        // a = 150
        let input: &[u8] = &[0x1a, 0x03, 0x08, 0x96, 0x01];
        proto_struct! {
            struct Test1 {
                a: i32 = 1,
            }
            struct Test3 {
                c: Option<Test1> = 3,
            }
        }
        let t3 = Test3::from_bytes(input.bytes()).unwrap();
        let t1 = t3.c.unwrap();
        assert_eq!(150, t1.a);
    }

    #[test]
    fn test_encoding_sample4() {
        // https://developers.google.com/protocol-buffers/docs/encoding#packed
        // message Test4 {
        //   repeated int32 d = 4 [packed=true];
        // }
        // d = [3, 270, 86942]
        let input: &[u8] = &[0x22, 0x06, 0x03, 0x8E, 0x02, 0x9E, 0xA7, 0x05];
        proto_struct! {
            struct Test4 {
                d: Vec<i32> = 4,
            }
        }
        let t4 = Test4::from_bytes(input.bytes()).unwrap();
        assert_eq!(vec![3, 270, 86942], t4.d);
    }
}
