// A generated source code by puroro library
// package .google.protobuf.FieldDescriptorProto

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub mod _puroro {
    pub use ::puroro::*;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    TypeDouble,
    TypeFloat,
    TypeInt64,
    TypeUint64,
    TypeInt32,
    TypeFixed64,
    TypeFixed32,
    TypeBool,
    TypeString,
    TypeGroup,
    TypeMessage,
    TypeBytes,
    TypeUint32,
    TypeEnum,
    TypeSfixed32,
    TypeSfixed64,
    TypeSint32,
    TypeSint64,
}

impl ::std::default::Default for Type {
    fn default() -> Self {
        Type::TypeDouble
    }
}

impl ::std::convert::TryFrom<i32> for Type {
    type Error = self::_puroro::PuroroError;
    fn try_from(x: i32) -> ::std::result::Result<Self, Self::Error> {
        #[allow(unused)]
        use ::std::result::Result::{Err, Ok};
        match x {
            1 => Ok(self::Type::TypeDouble),
            2 => Ok(self::Type::TypeFloat),
            3 => Ok(self::Type::TypeInt64),
            4 => Ok(self::Type::TypeUint64),
            5 => Ok(self::Type::TypeInt32),
            6 => Ok(self::Type::TypeFixed64),
            7 => Ok(self::Type::TypeFixed32),
            8 => Ok(self::Type::TypeBool),
            9 => Ok(self::Type::TypeString),
            10 => Ok(self::Type::TypeGroup),
            11 => Ok(self::Type::TypeMessage),
            12 => Ok(self::Type::TypeBytes),
            13 => Ok(self::Type::TypeUint32),
            14 => Ok(self::Type::TypeEnum),
            15 => Ok(self::Type::TypeSfixed32),
            16 => Ok(self::Type::TypeSfixed64),
            17 => Ok(self::Type::TypeSint32),
            18 => Ok(self::Type::TypeSint64),
            e => Err(self::_puroro::ErrorKind::UnknownEnumVariant(e))?,
        }
    }
}

impl ::std::convert::From<Type> for i32 {
    fn from(x: Type) -> i32 {
        match x {
            self::Type::TypeDouble => 1,
            self::Type::TypeFloat => 2,
            self::Type::TypeInt64 => 3,
            self::Type::TypeUint64 => 4,
            self::Type::TypeInt32 => 5,
            self::Type::TypeFixed64 => 6,
            self::Type::TypeFixed32 => 7,
            self::Type::TypeBool => 8,
            self::Type::TypeString => 9,
            self::Type::TypeGroup => 10,
            self::Type::TypeMessage => 11,
            self::Type::TypeBytes => 12,
            self::Type::TypeUint32 => 13,
            self::Type::TypeEnum => 14,
            self::Type::TypeSfixed32 => 15,
            self::Type::TypeSfixed64 => 16,
            self::Type::TypeSint32 => 17,
            self::Type::TypeSint64 => 18,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Label {
    LabelOptional,
    LabelRequired,
    LabelRepeated,
}

impl ::std::default::Default for Label {
    fn default() -> Self {
        Label::LabelOptional
    }
}

impl ::std::convert::TryFrom<i32> for Label {
    type Error = self::_puroro::PuroroError;
    fn try_from(x: i32) -> ::std::result::Result<Self, Self::Error> {
        #[allow(unused)]
        use ::std::result::Result::{Err, Ok};
        match x {
            1 => Ok(self::Label::LabelOptional),
            2 => Ok(self::Label::LabelRequired),
            3 => Ok(self::Label::LabelRepeated),
            e => Err(self::_puroro::ErrorKind::UnknownEnumVariant(e))?,
        }
    }
}

impl ::std::convert::From<Label> for i32 {
    fn from(x: Label) -> i32 {
        match x {
            self::Label::LabelOptional => 1,
            self::Label::LabelRequired => 2,
            self::Label::LabelRepeated => 3,
        }
    }
}
