use ::puroro::tags;
use ::puroro::tags::FieldTypeTag;
use ::puroro::{Message, PuroroError, RepeatedFieldCollector, Result};
use ::puroro_serializer::deserializer::stream::{Field, LengthDelimitedDeserializer};
use ::puroro_unknown::UnknownMessage;

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

macro_rules! proto_struct {
    () => {};
    (@read) => {};
    (@write) => {};

    (mod read_module = $readmodname:ident; mod write_module = $writemodname:ident; $($tts:tt)+) => {
        proto_struct!{@read $($tts)*}
    };

    (@read struct $structname:ident { $($fname:ident: $ftype:ty = $fid:expr ,)* } $($rest:tt)*) => {
        #[allow(non_camel_case_types)]
        #[derive(Debug, Default)]
        pub(crate) struct $structname {$(
            $fname: $ftype,
        )*}
        // Impl for creating a new message instance
        impl ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for $structname {
            type Target = Self;
            fn finish(self) -> ::puroro::Result<Self::Target> { Ok(self) }

            fn met_field<T: ::puroro_serializer::deserializer::stream::LengthDelimitedDeserializer>(
                &mut self,
                field: ::puroro_serializer::deserializer::stream::Field<T>,
                field_number: usize,
            ) -> ::puroro::Result<()> {
                use $crate::macros::DeserializableFromField;
                match field_number {
                    $($fid => {
                        self.$fname.merge_from_field(field)?;
                    })*
                    _ => { /* ignore! lol */ }
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
        impl $crate::macros::DeserializableFromField for Option<$structname> {
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
        impl $crate::macros::DeserializableFromField for Vec<$structname> {
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

        proto_struct!{@read $($rest)*}
    };

    (@read enum $enumname:ident { $($ename:ident = $evalue:expr ,)* } $($rest:tt)* ) => {
        #[allow(non_camel_case_types)]
        #[derive(::num_derive::FromPrimitive)]
        #[derive(Debug)]
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
        impl $crate::macros::DeserializableFromField for $enumname {
            fn merge_from_field<D>(&mut self, field: ::puroro_serializer::deserializer::stream::Field<D>) -> ::puroro::Result<()>
            where
                D: ::puroro_serializer::deserializer::stream::LengthDelimitedDeserializer
            {
                todo!()
            }
        }
        impl $crate::macros::DeserializableFromField for Vec<$enumname> {
            fn merge_from_field<D>(&mut self, field: ::puroro_serializer::deserializer::stream::Field<D>) -> ::puroro::Result<()>
            where
                D: ::puroro_serializer::deserializer::stream::LengthDelimitedDeserializer
            {
                todo!()
            }
        }

        proto_struct! { @read $($rest)* }
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
            mod read_module = read;
            mod write_module = write;
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
            mod read_module = read;
            mod write_module = write;
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
            mod read_module = read;
            mod write_module = write;
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
            mod read_module = read;
            mod write_module = write;
            struct Test4 {
                d: Vec<i32> = 4,
            }
        }
        let t4 = Test4::from_bytes(input.bytes()).unwrap();
        assert_eq!(vec![3, 270, 86942], t4.d);
    }
}
