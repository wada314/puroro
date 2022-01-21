// A generated source code by puroro library
// package ser_tests3

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
        type I32RepeatedType;
        type FloatUnlabeledType;
        type FloatRepeatedType;
        type StringUnlabeledType;
        type StringRepeatedType;
        type SubmsgUnlabeledType;
        type SubmsgRepeatedType;
        type EnumUnlabeledType;
        type EnumRepeatedType;
        type VeryLargeFieldNumberType;
    }

    impl MsgTemplateFieldTypes for ::puroro::internal::SimpleFields {
        type ImplTag = ::puroro::tags::SimpleImpl;
        type I32UnlabeledType = i32;
        type I32RepeatedType = ::std::vec::Vec<i32>;
        type FloatUnlabeledType = f32;
        type FloatRepeatedType = ::std::vec::Vec<f32>;
        type StringUnlabeledType = ::std::string::String;
        type StringRepeatedType = ::std::vec::Vec<::std::string::String>;
        type SubmsgUnlabeledType = ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_simple_impl::Submsg,
            >,
        >;
        type SubmsgRepeatedType = ::std::vec::Vec<
            self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_simple_impl::Submsg,
        >;
        type EnumUnlabeledType = self::_puroro_root::ser_tests3::Enum;
        type EnumRepeatedType = ::std::vec::Vec<self::_puroro_root::ser_tests3::Enum>;
        type VeryLargeFieldNumberType = i32;
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
            }

            impl SubmsgTemplateFieldTypes for ::puroro::internal::SimpleFields {
                type ImplTag = ::puroro::tags::SimpleImpl;
                type I32UnlabeledType = i32;
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
