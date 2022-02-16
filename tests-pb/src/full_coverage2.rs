// A generated source code by puroro library
// package full_coverage2

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
        type I32RequiredType;
        type I32OptionalType;
        type I32RepeatedType;
        type FloatRequiredType;
        type FloatOptionalType;
        type FloatRepeatedType;
        type BytesRequiredType;
        type BytesOptionalType;
        type BytesRepeatedType;
        type StringRequiredType;
        type StringOptionalType;
        type StringRepeatedType;
        type EnumRequiredType;
        type EnumOptionalType;
        type EnumRepeatedType;
        type SubmsgRequiredType;
        type SubmsgOptionalType;
        type SubmsgRepeatedType;
        type I64RequiredType;
        type I64OptionalType;
        type I64RepeatedType;
        type U32RequiredType;
        type U32OptionalType;
        type U32RepeatedType;
        type U64RequiredType;
        type U64OptionalType;
        type U64RepeatedType;
        type S32RequiredType;
        type S32OptionalType;
        type S32RepeatedType;
        type S64RequiredType;
        type S64OptionalType;
        type S64RepeatedType;
        type Fixed32RequiredType;
        type Fixed32OptionalType;
        type Fixed32RepeatedType;
        type Fixed64RequiredType;
        type Fixed64OptionalType;
        type Fixed64RepeatedType;
        type Sfixed32RequiredType;
        type Sfixed32OptionalType;
        type Sfixed32RepeatedType;
        type Sfixed64RequiredType;
        type Sfixed64OptionalType;
        type Sfixed64RepeatedType;
        type F64RequiredType;
        type F64OptionalType;
        type F64RepeatedType;
    }

    impl MsgTemplateFieldTypes for ::puroro::internal::OwnedFields {
        type ImplTag = ::puroro::tags::OwnedImpl;
        type I32RequiredType = i32;
        type I32OptionalType = i32;
        type I32RepeatedType = ::std::vec::Vec<i32>;
        type FloatRequiredType = f32;
        type FloatOptionalType = f32;
        type FloatRepeatedType = ::std::vec::Vec<f32>;
        type BytesRequiredType = ::std::vec::Vec<u8>;
        type BytesOptionalType = ::std::vec::Vec<u8>;
        type BytesRepeatedType = ::std::vec::Vec<::std::vec::Vec<u8>>;
        type StringRequiredType = ::std::string::String;
        type StringOptionalType = ::std::string::String;
        type StringRepeatedType = ::std::vec::Vec<::std::string::String>;
        type EnumRequiredType = self::_puroro_root::full_coverage2::Enum;
        type EnumOptionalType = self::_puroro_root::full_coverage2::Enum;
        type EnumRepeatedType = ::std::vec::Vec<self::_puroro_root::full_coverage2::Enum>;
        type SubmsgRequiredType = ::std::option::Option<::std::boxed::Box<self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_simple_impl::Submsg>>;
        type SubmsgOptionalType = ::std::option::Option<::std::boxed::Box<self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_simple_impl::Submsg>>;
        type SubmsgRepeatedType = ::std::vec::Vec<
            self::_puroro_root::full_coverage2::_puroro_nested::msg::_puroro_simple_impl::Submsg,
        >;
        type I64RequiredType = i64;
        type I64OptionalType = i64;
        type I64RepeatedType = ::std::vec::Vec<i64>;
        type U32RequiredType = u32;
        type U32OptionalType = u32;
        type U32RepeatedType = ::std::vec::Vec<u32>;
        type U64RequiredType = u64;
        type U64OptionalType = u64;
        type U64RepeatedType = ::std::vec::Vec<u64>;
        type S32RequiredType = i32;
        type S32OptionalType = i32;
        type S32RepeatedType = ::std::vec::Vec<i32>;
        type S64RequiredType = i64;
        type S64OptionalType = i64;
        type S64RepeatedType = ::std::vec::Vec<i64>;
        type Fixed32RequiredType = u32;
        type Fixed32OptionalType = u32;
        type Fixed32RepeatedType = ::std::vec::Vec<u32>;
        type Fixed64RequiredType = u64;
        type Fixed64OptionalType = u64;
        type Fixed64RepeatedType = ::std::vec::Vec<u64>;
        type Sfixed32RequiredType = i32;
        type Sfixed32OptionalType = i32;
        type Sfixed32RepeatedType = ::std::vec::Vec<i32>;
        type Sfixed64RequiredType = i64;
        type Sfixed64OptionalType = i64;
        type Sfixed64RepeatedType = ::std::vec::Vec<i64>;
        type F64RequiredType = f64;
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
}

impl ::puroro::Enum2 for Enum {}
impl ::std::convert::TryFrom<i32> for Enum {
    type Error = i32;
    fn try_from(value: i32) -> ::std::result::Result<Self, i32> {
        ::std::result::Result::Ok(match value {
            0 => Enum::Zeroth,
            1 => Enum::First,
            10 => Enum::Tenth,
            _ => Err(value)?,
        })
    }
}

impl ::std::convert::From<Enum> for i32 {
    fn from(value: Enum) -> i32 {
        match value {
            Enum::Zeroth => 0,
            Enum::First => 1,
            Enum::Tenth => 10,
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
            #[allow(unused_imports)]
            use super::_puroro_internal::*;
            #[allow(unused_imports)]
            use super::_puroro_traits::*;
        }
        pub mod _puroro_internal {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }
            pub trait SubmsgTemplateFieldTypes {
                type ImplTag;
                type I32RequiredType;
                type I64RequiredType;
            }

            impl SubmsgTemplateFieldTypes for ::puroro::internal::OwnedFields {
                type ImplTag = ::puroro::tags::OwnedImpl;
                type I32RequiredType = i32;
                type I64RequiredType = i64;
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
