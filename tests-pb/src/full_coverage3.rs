// A generated source code by puroro library
// package full_coverage3

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
    use super::_puroro_internal::*;
    use super::_puroro_traits::*;
}
pub mod _puroro_internal {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
    pub trait MsgTemplateFieldTypes {
        type ImplTag;
        type I32UnlabeledType;
        type I32OptionalType;
        type I32RepeatedType;
        type FloatUnlabeledType;
        type FloatOptionalType;
        type FloatRepeatedType;
        type BytesUnlabeledType;
        type BytesOptionalType;
        type BytesRepeatedType;
        type StringUnlabeledType;
        type StringOptionalType;
        type StringRepeatedType;
        type EnumUnlabeledType;
        type EnumOptionalType;
        type EnumRepeatedType;
        type SubmsgUnlabeledType;
        type SubmsgOptionalType;
        type SubmsgRepeatedType;
        type I64UnlabeledType;
        type I64OptionalType;
        type I64RepeatedType;
        type U32UnlabeledType;
        type U32OptionalType;
        type U32RepeatedType;
        type U64UnlabeledType;
        type U64OptionalType;
        type U64RepeatedType;
        type S32UnlabeledType;
        type S32OptionalType;
        type S32RepeatedType;
        type S64UnlabeledType;
        type S64OptionalType;
        type S64RepeatedType;
        type Fixed32UnlabeledType;
        type Fixed32OptionalType;
        type Fixed32RepeatedType;
        type Fixed64UnlabeledType;
        type Fixed64OptionalType;
        type Fixed64RepeatedType;
        type Sfixed32UnlabeledType;
        type Sfixed32OptionalType;
        type Sfixed32RepeatedType;
        type Sfixed64UnlabeledType;
        type Sfixed64OptionalType;
        type Sfixed64RepeatedType;
        type F64UnlabeledType;
        type F64OptionalType;
        type F64RepeatedType;
    }

    impl MsgTemplateFieldTypes for ::puroro::internal::SimpleFields {
        type ImplTag = ::puroro::tags::SimpleImpl;
        type I32UnlabeledType = i32;
        type I32OptionalType = i32;
        type I32RepeatedType = ::std::vec::Vec<i32>;
        type FloatUnlabeledType = f32;
        type FloatOptionalType = f32;
        type FloatRepeatedType = ::std::vec::Vec<f32>;
        type BytesUnlabeledType = ::std::vec::Vec<u8>;
        type BytesOptionalType = ::std::vec::Vec<u8>;
        type BytesRepeatedType = ::std::vec::Vec<::std::vec::Vec<u8>>;
        type StringUnlabeledType = ::std::string::String;
        type StringOptionalType = ::std::string::String;
        type StringRepeatedType = ::std::vec::Vec<::std::string::String>;
        type EnumUnlabeledType = self::_puroro_root::full_coverage3::Enum;
        type EnumOptionalType = self::_puroro_root::full_coverage3::Enum;
        type EnumRepeatedType = ::std::vec::Vec<self::_puroro_root::full_coverage3::Enum>;
        type SubmsgUnlabeledType = ::std::option::Option<::std::boxed::Box<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_simple_impl::Submsg>>;
        type SubmsgOptionalType = ::std::option::Option<::std::boxed::Box<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_simple_impl::Submsg>>;
        type SubmsgRepeatedType = ::std::vec::Vec<
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_simple_impl::Submsg,
        >;
        type I64UnlabeledType = i64;
        type I64OptionalType = i64;
        type I64RepeatedType = ::std::vec::Vec<i64>;
        type U32UnlabeledType = u32;
        type U32OptionalType = u32;
        type U32RepeatedType = ::std::vec::Vec<u32>;
        type U64UnlabeledType = u64;
        type U64OptionalType = u64;
        type U64RepeatedType = ::std::vec::Vec<u64>;
        type S32UnlabeledType = i32;
        type S32OptionalType = i32;
        type S32RepeatedType = ::std::vec::Vec<i32>;
        type S64UnlabeledType = i64;
        type S64OptionalType = i64;
        type S64RepeatedType = ::std::vec::Vec<i64>;
        type Fixed32UnlabeledType = u32;
        type Fixed32OptionalType = u32;
        type Fixed32RepeatedType = ::std::vec::Vec<u32>;
        type Fixed64UnlabeledType = u64;
        type Fixed64OptionalType = u64;
        type Fixed64RepeatedType = ::std::vec::Vec<u64>;
        type Sfixed32UnlabeledType = i32;
        type Sfixed32OptionalType = i32;
        type Sfixed32RepeatedType = ::std::vec::Vec<i32>;
        type Sfixed64UnlabeledType = i64;
        type Sfixed64OptionalType = i64;
        type Sfixed64RepeatedType = ::std::vec::Vec<i64>;
        type F64UnlabeledType = f64;
        type F64OptionalType = f64;
        type F64RepeatedType = ::std::vec::Vec<f64>;
    }
}
pub use _puroro_traits::*;
pub mod _puroro_traits {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
}
#[derive(::std::fmt::Debug, ::std::clone::Clone, ::std::marker::Copy, ::std::cmp::PartialEq)]
pub enum Enum {
    Zeroth,
    First,
    Tenth,
    _Unknown(i32),
}

impl ::puroro::Enum3 for Enum {}
impl ::std::convert::From<i32> for Enum {
    fn from(value: i32) -> Self {
        match value {
            0 => Enum::Zeroth,
            1 => Enum::First,
            10 => Enum::Tenth,
            _ => Enum::_Unknown(value),
        }
    }
}

impl ::std::convert::From<Enum> for i32 {
    fn from(value: Enum) -> i32 {
        match value {
            Enum::Zeroth => 0,
            Enum::First => 1,
            Enum::Tenth => 10,
            Enum::_Unknown(ivalue) => ivalue,
        }
    }
}

impl ::std::default::Default for Enum {
    fn default() -> Self {
        Enum::Zeroth
    }
}

impl<'bump> ::puroro::internal::BumpDefault<'bump> for Enum {
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

        //pub use _puroro_simple_impl::Submsg;
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
            use super::_puroro_internal::*;
            use super::_puroro_traits::*;
        }
        pub mod _puroro_internal {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }
            pub trait SubmsgTemplateFieldTypes {
                type ImplTag;
                type I32UnlabeledType;
                type I32OptionalType;
                type I64UnlabeledType;
            }

            impl SubmsgTemplateFieldTypes for ::puroro::internal::SimpleFields {
                type ImplTag = ::puroro::tags::SimpleImpl;
                type I32UnlabeledType = i32;
                type I32OptionalType = i32;
                type I64UnlabeledType = i64;
            }
        }
        pub use _puroro_traits::*;
        pub mod _puroro_traits {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }
        }
        pub use _puroro_nested::*;
        pub mod _puroro_nested {
            pub mod submsg {
                mod _puroro_root {
                    pub use super::super::super::_puroro_root::*;
                }
            }
        }
    }
}
