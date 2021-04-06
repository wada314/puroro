use ::puroro::{Message, RepeatedFieldCollector, Result};
use ::puroro_unknown::UnknownMessage;

pub(crate) trait DeserializableFieldTypeTag {
    type Output;
    fn get_from_unknown_message(msg: &UnknownMessage, field_number: usize) -> Result<Self::Output>;
}
pub(crate) trait SerializableFieldTypeTag {
    type Native;
    fn serialize<W: std::io::Write>(self_: &Self::Native, write: &mut W) -> Result<Self::Native>;
}
trait RepeatableFieldTypeTag: DeserializableFieldTypeTag {}

macro_rules! define_native_field_type_tag {
    ($type:ty, $get_method:ident, $handle_repeated_method:ident) => {
        impl DeserializableFieldTypeTag for $type {
            type Output = $type;
            fn get_from_unknown_message(
                msg: &UnknownMessage,
                field_number: usize,
            ) -> Result<Self::Output> {
                msg.$get_method(field_number, <$type as Default>::default())
            }
        }
        impl DeserializableFieldTypeTag for Vec<$type> {
            type Output = Vec<$type>;
            fn get_from_unknown_message(
                msg: &UnknownMessage,
                field_number: usize,
            ) -> Result<Self::Output> {
                msg.$handle_repeated_method(
                    field_number,
                    &RepeatedFieldCollector::<$type, Vec<$type>>::new(),
                )
            }
        }
    };
}
define_native_field_type_tag!(i32, get_field_as_i32_or, handle_field_as_repeated_i32);
define_native_field_type_tag!(i64, get_field_as_i64_or, handle_field_as_repeated_i64);
define_native_field_type_tag!(u32, get_field_as_u32_or, handle_field_as_repeated_u32);
define_native_field_type_tag!(u64, get_field_as_u64_or, handle_field_as_repeated_u64);
define_native_field_type_tag!(bool, get_field_as_bool_or, handle_field_as_repeated_bool);
impl DeserializableFieldTypeTag for String {
    type Output = String;
    fn get_from_unknown_message(msg: &UnknownMessage, field_number: usize) -> Result<Self::Output> {
        msg.handle_field_as_str(field_number, &RepeatedFieldCollector::<char, String>::new())
    }
}
impl DeserializableFieldTypeTag for Vec<String> {
    type Output = Vec<String>;
    fn get_from_unknown_message(msg: &UnknownMessage, field_number: usize) -> Result<Self::Output> {
        msg.handle_field_as_repeated_str(
            field_number,
            &RepeatedFieldCollector::<char, String>::new(),
            &RepeatedFieldCollector::<String, Vec<String>>::new(),
        )
    }
}

macro_rules! proto_struct {
    () => {};
    (@read) => {};
    (@write) => {};

    (mod read_module = $readmodname:ident; mod write_module = $writemodname:ident; $($tts:tt)+) => {
        pub(crate) mod $readmodname {
            proto_struct!{@read $($tts)*}
        }
        pub(crate) mod $writemodname {
            proto_struct!{@write $($tts)*}
        }
    };

    (@read struct $structname:ident { $($fname:ident: $ftype:ty = $fid:expr ,)* } $($rest:tt)*) => {
        #[allow(non_camel_case_types)]
        #[derive(Debug)]
        pub(crate) struct $structname(::puroro_unknown::UnknownMessage);
        #[allow(dead_code)]
        impl $structname {
            $(
                pub(crate) fn $fname(&self) -> ::puroro::Result<<$ftype as $crate::macros::DeserializableFieldTypeTag>::Output> {
                    <$ftype as $crate::macros::DeserializableFieldTypeTag>::get_from_unknown_message(&self.0, $fid)
                }
            )*
        }
        impl std::ops::Deref for $structname {
            type Target = ::puroro_unknown::UnknownMessage;
            fn deref(&self) -> &Self::Target { &self.0 }
        }
        impl ::puroro::Deserializable for $structname {
            fn from_bytes<I: Iterator<Item = std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
                Ok(Self(::puroro_unknown::UnknownMessage::from_bytes(iter)?))
            }
        }
        impl ::puroro::Mergeable for $structname {
            fn merge(&self, latter: &Self) -> ::puroro::Result<Self> {
                Ok(Self(self.0.merge(&latter.0)?))
            }
        }
        impl $crate::macros::DeserializableFieldTypeTag for Option<$structname> {
            type Output = Option<$structname>;
            fn get_from_unknown_message(msg: &::puroro_unknown::UnknownMessage, field_number: usize) -> ::puroro::Result<Self::Output> {
                use ::puroro::Message;
                msg.get_field_as_message::<$structname>(field_number)
            }
        }
        impl $crate::macros::DeserializableFieldTypeTag for Vec<$structname> {
            type Output = Vec<$structname>;
            fn get_from_unknown_message(msg: &::puroro_unknown::UnknownMessage, field_number: usize) -> ::puroro::Result<Self::Output> {
                use ::puroro::Message;
                msg.collect_field_as_repeated_message::<$structname, Vec<$structname>>(
                    field_number)
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
        impl $crate::macros::DeserializableFieldTypeTag for std::result::Result<$enumname, i32> {
            type Output = std::result::Result<$enumname, i32>;
            fn get_from_unknown_message(msg: &::puroro_unknown::UnknownMessage, field_number: usize) -> ::puroro::Result<Self::Output> {
                use ::puroro::Message;
                use ::num_traits::FromPrimitive;
                let raw = msg.get_field_as_i32_or(field_number, <i32 as Default>::default())?;
                match $enumname::from_i32(raw) {
                    Some(enumified) => Ok(Ok(enumified)),
                    None => Ok(Err(raw)),
                }
            }
        }

        proto_struct! { @read $($rest)* }
    };

    (@write struct $structname:ident { $($fname:ident: $ftype:ty = $fid:expr ,)* } $($rest:tt)*) => {
        #[allow(non_camel_case_types)]
        #[derive(Debug)]
        pub(crate) struct $structname {
            $(pub(crate) $fname: $ftype, )*
        }
        impl ::puroro::Serializable for $structname {
            fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<Self> {
                todo!()
            }
        }
        proto_struct!{@write $($rest)*}
    };

    (@write enum $enumname:ident { $($ename:ident = $evalue:expr ,)* } $($rest:tt)* ) => {
        proto_struct! { @write $($rest)* }
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
        use read::Test1;
        let t1 = Test1::from_bytes(input.bytes()).unwrap();
        assert_eq!(150, t1.a().unwrap());
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
        use read::Test2;
        let t2 = Test2::from_bytes(input.bytes()).unwrap();
        assert_eq!("testing", t2.b().unwrap());
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
        use read::Test3;
        let t3 = Test3::from_bytes(input.bytes()).unwrap();
        let t1 = t3.c().unwrap().unwrap();
        assert_eq!(150, t1.a().unwrap());
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
        use read::Test4;
        let t4 = Test4::from_bytes(input.bytes()).unwrap();
        assert_eq!(vec![3, 270, 86942], t4.d().unwrap());
    }
}
