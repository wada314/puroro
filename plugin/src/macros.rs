use ::puroro::{Message, RepeatedFieldCollector, RepeatedFieldHandler, Result};
use ::puroro_unknown::UnknownMessage;

pub(crate) trait FieldTypeTag {
    type Output;
    fn get_from_unknown_message(msg: &UnknownMessage, field_number: usize) -> Result<Self::Output>;
}
trait RepeatableFieldTypeTag: FieldTypeTag {}

macro_rules! define_native_field_type_tag {
    ($type:ty, $get_method:ident, $handle_repeated_method:ident) => {
        impl FieldTypeTag for $type {
            type Output = $type;
            fn get_from_unknown_message(
                msg: &UnknownMessage,
                field_number: usize,
            ) -> Result<Self::Output> {
                msg.$get_method(field_number)
            }
        }
        impl FieldTypeTag for Vec<$type> {
            type Output = Vec<$type>;
            fn get_from_unknown_message(
                msg: &UnknownMessage,
                field_number: usize,
            ) -> Result<Self::Output> {
                msg.$handle_repeated_method(
                    field_number,
                    RepeatedFieldCollector::<$type, Vec<$type>>::new(),
                )
            }
        }
    };
}
define_native_field_type_tag!(i32, get_field_as_i32, handle_field_as_repeated_i32);
define_native_field_type_tag!(i64, get_field_as_i64, handle_field_as_repeated_i64);
define_native_field_type_tag!(u32, get_field_as_u32, handle_field_as_repeated_u32);
define_native_field_type_tag!(u64, get_field_as_u64, handle_field_as_repeated_u64);
define_native_field_type_tag!(bool, get_field_as_bool, handle_field_as_repeated_bool);
impl FieldTypeTag for String {
    type Output = String;
    fn get_from_unknown_message(msg: &UnknownMessage, field_number: usize) -> Result<Self::Output> {
        msg.collect_field_as_str(field_number)
    }
}
impl FieldTypeTag for Vec<String> {
    type Output = Vec<String>;
    fn get_from_unknown_message(msg: &UnknownMessage, field_number: usize) -> Result<Self::Output> {
        msg.collect_field_as_repeated_str(field_number)
    }
}

macro_rules! proto_readable_struct {
    (struct $strname:ident { $($fname:ident: $ftype:ty = $fid:expr),*, }) => {
        pub(crate) struct $strname(::puroro_unknown::UnknownMessage);
        impl $strname {
            $(
                fn $fname(&self) -> Result<<$ftype as $crate::macros::FieldTypeTag>::Output> {
                    <$ftype as $crate::macros::FieldTypeTag>::get_from_unknown_message(&self.0, $fid)
                }
            )*
        }
        impl std::ops::Deref for $strname {
            type Target = ::puroro_unknown::UnknownMessage;
            fn deref(&self) -> &Self::Target { &self.0 }
        }
        impl ::puroro::Deserializable for $strname {
            fn from_bytes<I: Iterator<Item = std::io::Result<u8>>>(iter: I) -> Result<Self> {
                Ok(Self(::puroro_unknown::UnknownMessage::from_bytes(iter)?))
            }
        }
        impl ::puroro::Mergeable for $strname {
            type MergedType = Self;
            fn merge(&self, latter: &Self) -> Result<Self::MergedType> {
                Ok(Self(self.0.merge(&latter.0)?))
            }
        }
        impl $crate::macros::FieldTypeTag for Option<$strname> {
            type Output = Option<$strname>;
            fn get_from_unknown_message(msg: &::puroro_unknown::UnknownMessage, field_number: usize) -> Result<Self::Output> {
                use ::puroro::Message;
                msg.get_field_as_message::<$strname>(field_number)
            }
        }
        impl $crate::macros::FieldTypeTag for Vec<$strname> {
            type Output = Vec<$strname>;
            fn get_from_unknown_message(msg: &::puroro_unknown::UnknownMessage, field_number: usize) -> Result<Self::Output> {
                use ::puroro::Message;
                msg.collect_field_as_repeated_message::<$strname, Vec<$strname>>(
                    field_number)
            }
        }
    };
}
