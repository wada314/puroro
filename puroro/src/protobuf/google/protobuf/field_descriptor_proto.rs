mod _root {
    #[allow(unused)]
    pub(crate) use super::super::_root::*;
}
mod _puroro {
    #[allow(unused)]
    pub(crate) use super::_root::_puroro::*;
}
mod _pinternal {
    #[allow(unused)]
    pub(crate) use super::_root::_pinternal::*;
}
#[derive(
    ::std::clone::Clone,
    ::std::marker::Copy,
    ::std::cmp::PartialEq,
    ::std::cmp::Eq,
    ::std::cmp::PartialOrd,
    ::std::cmp::Ord,
    ::std::hash::Hash,
    ::std::fmt::Debug,
)]
pub enum Type {
    /** 0 is reserved for errors.
 Order is weird for historical reasons.
*/
    TypeDouble,
    TypeFloat,
    /** Not ZigZag encoded.  Negative numbers take 10 bytes.  Use TYPE_SINT64 if
 negative values are likely.
*/
    TypeInt64,
    TypeUint64,
    /** Not ZigZag encoded.  Negative numbers take 10 bytes.  Use TYPE_SINT32 if
 negative values are likely.
*/
    TypeInt32,
    TypeFixed64,
    TypeFixed32,
    TypeBool,
    TypeString,
    /** Tag-delimited aggregate.
 Group type is deprecated and not supported in proto3. However, Proto3
 implementations should still be able to parse the group wire format and
 treat group fields as unknown fields.
*/
    TypeGroup,
    /** Length-delimited aggregate.
*/
    TypeMessage,
    /** New in version 2.
*/
    TypeBytes,
    TypeUint32,
    TypeEnum,
    TypeSfixed32,
    TypeSfixed64,
    /** Uses ZigZag encoding.
*/
    TypeSint32,
    /** Uses ZigZag encoding.
*/
    TypeSint64,
}
impl ::std::default::Default for Type {
    fn default() -> Self {
        Self::TypeDouble
    }
}
impl ::std::convert::From::<Type> for i32 {
    fn from(val: Type) -> i32 {
        match val {
            Type::TypeDouble => 1i32,
            Type::TypeFloat => 2i32,
            Type::TypeInt64 => 3i32,
            Type::TypeUint64 => 4i32,
            Type::TypeInt32 => 5i32,
            Type::TypeFixed64 => 6i32,
            Type::TypeFixed32 => 7i32,
            Type::TypeBool => 8i32,
            Type::TypeString => 9i32,
            Type::TypeGroup => 10i32,
            Type::TypeMessage => 11i32,
            Type::TypeBytes => 12i32,
            Type::TypeUint32 => 13i32,
            Type::TypeEnum => 14i32,
            Type::TypeSfixed32 => 15i32,
            Type::TypeSfixed64 => 16i32,
            Type::TypeSint32 => 17i32,
            Type::TypeSint64 => 18i32,
        }
    }
}
impl ::std::convert::TryFrom::<i32> for Type {
    type Error = self::_puroro::PuroroError;
    fn try_from(val: i32) -> ::std::result::Result<Self, Self::Error> {
        match val {
            1i32 => ::std::result::Result::Ok(self::Type::TypeDouble),
            2i32 => ::std::result::Result::Ok(self::Type::TypeFloat),
            3i32 => ::std::result::Result::Ok(self::Type::TypeInt64),
            4i32 => ::std::result::Result::Ok(self::Type::TypeUint64),
            5i32 => ::std::result::Result::Ok(self::Type::TypeInt32),
            6i32 => ::std::result::Result::Ok(self::Type::TypeFixed64),
            7i32 => ::std::result::Result::Ok(self::Type::TypeFixed32),
            8i32 => ::std::result::Result::Ok(self::Type::TypeBool),
            9i32 => ::std::result::Result::Ok(self::Type::TypeString),
            10i32 => ::std::result::Result::Ok(self::Type::TypeGroup),
            11i32 => ::std::result::Result::Ok(self::Type::TypeMessage),
            12i32 => ::std::result::Result::Ok(self::Type::TypeBytes),
            13i32 => ::std::result::Result::Ok(self::Type::TypeUint32),
            14i32 => ::std::result::Result::Ok(self::Type::TypeEnum),
            15i32 => ::std::result::Result::Ok(self::Type::TypeSfixed32),
            16i32 => ::std::result::Result::Ok(self::Type::TypeSfixed64),
            17i32 => ::std::result::Result::Ok(self::Type::TypeSint32),
            18i32 => ::std::result::Result::Ok(self::Type::TypeSint64),
            _ => {
                ::std::result::Result::Err(
                    self::_puroro::PuroroError::UnknownEnumVariant(val),
                )?
            }
        }
    }
}
#[derive(
    ::std::clone::Clone,
    ::std::marker::Copy,
    ::std::cmp::PartialEq,
    ::std::cmp::Eq,
    ::std::cmp::PartialOrd,
    ::std::cmp::Ord,
    ::std::hash::Hash,
    ::std::fmt::Debug,
)]
pub enum Label {
    /** 0 is reserved for errors
*/
    LabelOptional,
    LabelRequired,
    LabelRepeated,
}
impl ::std::default::Default for Label {
    fn default() -> Self {
        Self::LabelOptional
    }
}
impl ::std::convert::From::<Label> for i32 {
    fn from(val: Label) -> i32 {
        match val {
            Label::LabelOptional => 1i32,
            Label::LabelRequired => 2i32,
            Label::LabelRepeated => 3i32,
        }
    }
}
impl ::std::convert::TryFrom::<i32> for Label {
    type Error = self::_puroro::PuroroError;
    fn try_from(val: i32) -> ::std::result::Result<Self, Self::Error> {
        match val {
            1i32 => ::std::result::Result::Ok(self::Label::LabelOptional),
            2i32 => ::std::result::Result::Ok(self::Label::LabelRequired),
            3i32 => ::std::result::Result::Ok(self::Label::LabelRepeated),
            _ => {
                ::std::result::Result::Err(
                    self::_puroro::PuroroError::UnknownEnumVariant(val),
                )?
            }
        }
    }
}
