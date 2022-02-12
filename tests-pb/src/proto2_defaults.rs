// A generated source code by puroro library
// package proto2_defaults

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

//pub use _puroro_simple_impl::Msg;
pub mod _puroro_simple_impl {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
}

pub use _puroro_impls::*;
pub mod _puroro_impls {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
    #[allow(unused_imports)]
    use super::_puroro_internal::*;
    #[allow(unused_imports)]
    use super::_puroro_traits::*;
}
pub mod _puroro_internal {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
    pub trait MsgTemplateFieldTypes {
        type ImplTag;
        type I32DefaultType;
        type I320Type;
        type I3242Type;
        type I32M42Type;
        type I322147483647Type;
        type I32M2147483648Type;
        type I320123Type;
        type I320x123Type;
        type U32DefaultType;
        type U320Type;
        type U3242Type;
        type U324294967295Type;
        type U320123Type;
        type U320x123Type;
        type I64DefaultType;
        type I640Type;
        type I6442Type;
        type I64M42Type;
        type I649223372036854775807Type;
        type I64M9223372036854775808Type;
        type I640123Type;
        type I640x123Type;
        type U64DefaultType;
        type U640Type;
        type U6442Type;
        type U6418446744073709551615Type;
        type U640123Type;
        type U640x123Type;
        type F32DefaultType;
        type F320Type;
        type F32M0Type;
        type F320pType;
        type F32P0Type;
        type F320p0Type;
        type F3242Type;
        type F32M42Type;
        type F320p25Type;
        type F321p5e2Type;
        type F32InfType;
        type F32MinfType;
        type F32NanType;
        type F32MnanType;
        type BoolDefaultType;
        type BoolTrueType;
        type BoolFalseType;
        type StringDefaultType;
        type StringEmptyType;
        type StringAbcType;
        type StringAiuType;
        type StringBackslashType;
        type StringTabType;
        type StringCrlfType;
        type BytesDefaultType;
        type BytesEmptyType;
        type BytesAbcType;
        type BytesAiuType;
        type BytesBackslashType;
        type BytesTabType;
        type BytesCrlfType;
        type EnumDefaultType;
        type EnumOneType;
        type EnumFourtyTwoType;
    }

    impl MsgTemplateFieldTypes for ::puroro::internal::SimpleFields {
        type ImplTag = ::puroro::tags::SimpleImpl;
        type I32DefaultType = i32;
        type I320Type = i32;
        type I3242Type = i32;
        type I32M42Type = i32;
        type I322147483647Type = i32;
        type I32M2147483648Type = i32;
        type I320123Type = i32;
        type I320x123Type = i32;
        type U32DefaultType = u32;
        type U320Type = u32;
        type U3242Type = u32;
        type U324294967295Type = u32;
        type U320123Type = u32;
        type U320x123Type = u32;
        type I64DefaultType = i64;
        type I640Type = i64;
        type I6442Type = i64;
        type I64M42Type = i64;
        type I649223372036854775807Type = i64;
        type I64M9223372036854775808Type = i64;
        type I640123Type = i64;
        type I640x123Type = i64;
        type U64DefaultType = u64;
        type U640Type = u64;
        type U6442Type = u64;
        type U6418446744073709551615Type = u64;
        type U640123Type = u64;
        type U640x123Type = u64;
        type F32DefaultType = f32;
        type F320Type = f32;
        type F32M0Type = f32;
        type F320pType = f32;
        type F32P0Type = f32;
        type F320p0Type = f32;
        type F3242Type = f32;
        type F32M42Type = f32;
        type F320p25Type = f32;
        type F321p5e2Type = f32;
        type F32InfType = f32;
        type F32MinfType = f32;
        type F32NanType = f32;
        type F32MnanType = f32;
        type BoolDefaultType = bool;
        type BoolTrueType = bool;
        type BoolFalseType = bool;
        type StringDefaultType = ::std::string::String;
        type StringEmptyType = ::std::string::String;
        type StringAbcType = ::std::string::String;
        type StringAiuType = ::std::string::String;
        type StringBackslashType = ::std::string::String;
        type StringTabType = ::std::string::String;
        type StringCrlfType = ::std::string::String;
        type BytesDefaultType = ::std::vec::Vec<u8>;
        type BytesEmptyType = ::std::vec::Vec<u8>;
        type BytesAbcType = ::std::vec::Vec<u8>;
        type BytesAiuType = ::std::vec::Vec<u8>;
        type BytesBackslashType = ::std::vec::Vec<u8>;
        type BytesTabType = ::std::vec::Vec<u8>;
        type BytesCrlfType = ::std::vec::Vec<u8>;
        type EnumDefaultType = self::_puroro_root::proto2_defaults::MyEnum;
        type EnumOneType = self::_puroro_root::proto2_defaults::MyEnum;
        type EnumFourtyTwoType = self::_puroro_root::proto2_defaults::MyEnum;
    }
}
pub use _puroro_traits::*;
pub mod _puroro_traits {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
}
#[derive(::std::fmt::Debug, ::std::clone::Clone, ::std::marker::Copy, ::std::cmp::PartialEq)]
pub enum MyEnum {
    One,
    FourtyTwo,
}

impl ::puroro::Enum2 for MyEnum {}
impl ::std::convert::TryFrom<i32> for MyEnum {
    type Error = i32;
    fn try_from(value: i32) -> ::std::result::Result<Self, i32> {
        ::std::result::Result::Ok(match value {
            1 => MyEnum::One,
            42 => MyEnum::FourtyTwo,
            _ => Err(value)?,
        })
    }
}

impl ::std::convert::From<MyEnum> for i32 {
    fn from(value: MyEnum) -> i32 {
        match value {
            MyEnum::One => 1,
            MyEnum::FourtyTwo => 42,
        }
    }
}

impl ::std::default::Default for MyEnum {
    fn default() -> Self {
        MyEnum::One
    }
}

impl<'bump> ::puroro::internal::BumpDefault<'bump> for MyEnum {
    fn default_in(_: &'bump ::puroro::bumpalo::Bump) -> Self {
        ::std::default::Default::default()
    }
}
pub use _puroro_nested::*;
pub mod _puroro_nested {
    pub mod msg {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
    }
}
