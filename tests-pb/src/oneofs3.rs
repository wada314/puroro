// A generated source code by puroro library
// package oneofs3

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

//pub use _puroro_simple_impl::Msg;
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
    pub trait MsgTemplateFieldTypes {
        type ImplTag;
        type G1Int32Type;
        type G1StringType;
        type G2F32Type;
        type G2StringType;
        type G2SubmsgType;
        type G3Int32Type;
    }

    impl MsgTemplateFieldTypes for ::puroro::internal::SimpleFields {
        type ImplTag = ::puroro::tags::SimpleImpl;
        type G1Int32Type = i32;
        type G1StringType = ::std::string::String;
        type G2F32Type = f32;
        type G2StringType = ::std::string::String;
        type G2SubmsgType =
            ::std::boxed::Box<self::_puroro_root::oneofs3::_puroro_simple_impl::Submsg>;
        type G3Int32Type = i32;
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
    pub mod msg {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }

        pub use _puroro_oneofs::*;
        pub mod _puroro_oneofs {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }

            pub enum GroupOne<G1Int32, G1String> {
                G1Int32(G1Int32),
                G1String(G1String),
            }
            impl<G1Int32, G1String> GroupOne<G1Int32, G1String> {
                pub fn g1_int32(self) -> ::std::option::Option<G1Int32> {
                    match self {
                        Self::G1Int32(v) => ::std::option::Option::Some(v),
                        #[allow(unreachable_patterns)]
                        _ => None,
                    }
                }
                pub fn g1_string(self) -> ::std::option::Option<G1String> {
                    match self {
                        Self::G1String(v) => ::std::option::Option::Some(v),
                        #[allow(unreachable_patterns)]
                        _ => None,
                    }
                }
            }

            impl<G1Int32, G1String> ::std::fmt::Debug for GroupOne<G1Int32, G1String>
            where
                G1Int32: ::std::fmt::Debug,
                G1String: ::std::fmt::Debug,
            {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    match self {
                        Self::G1Int32(v) => f.debug_tuple("GroupOne::G1Int32").field(&v).finish(),
                        Self::G1String(v) => f.debug_tuple("GroupOne::G1String").field(&v).finish(),
                    }
                }
            }

            impl<G1Int32, G1String> ::std::clone::Clone for GroupOne<G1Int32, G1String>
            where
                G1Int32: ::std::clone::Clone,
                G1String: ::std::clone::Clone,
            {
                fn clone(&self) -> Self {
                    match self {
                        Self::G1Int32(ref v) => {
                            Self::G1Int32(<G1Int32 as ::std::clone::Clone>::clone(v))
                        }
                        Self::G1String(ref v) => {
                            Self::G1String(<G1String as ::std::clone::Clone>::clone(v))
                        }
                    }
                }
            }

            impl<G1Int32, G1String> ::std::cmp::PartialEq for GroupOne<G1Int32, G1String>
            where
                G1Int32: ::std::cmp::PartialEq,
                G1String: ::std::cmp::PartialEq,
            {
                fn eq(&self, rhs: &Self) -> bool {
                    match (self, rhs) {
                        (Self::G1Int32(left), Self::G1Int32(right)) => left == right,
                        (Self::G1String(left), Self::G1String(right)) => left == right,
                        #[allow(unreachable_patterns)]
                        _ => false,
                    }
                }
            }

            pub enum GroupTwo<G2F32, G2String, G2Submsg> {
                G2F32(G2F32),
                G2String(G2String),
                G2Submsg(G2Submsg),
            }
            impl<G2F32, G2String, G2Submsg> GroupTwo<G2F32, G2String, G2Submsg> {
                pub fn g2_f32(self) -> ::std::option::Option<G2F32> {
                    match self {
                        Self::G2F32(v) => ::std::option::Option::Some(v),
                        #[allow(unreachable_patterns)]
                        _ => None,
                    }
                }
                pub fn g2_string(self) -> ::std::option::Option<G2String> {
                    match self {
                        Self::G2String(v) => ::std::option::Option::Some(v),
                        #[allow(unreachable_patterns)]
                        _ => None,
                    }
                }
                pub fn g2_submsg(self) -> ::std::option::Option<G2Submsg> {
                    match self {
                        Self::G2Submsg(v) => ::std::option::Option::Some(v),
                        #[allow(unreachable_patterns)]
                        _ => None,
                    }
                }
            }

            impl<G2F32, G2String, G2Submsg> ::std::fmt::Debug for GroupTwo<G2F32, G2String, G2Submsg>
            where
                G2F32: ::std::fmt::Debug,
                G2String: ::std::fmt::Debug,
                G2Submsg: ::std::fmt::Debug,
            {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    match self {
                        Self::G2F32(v) => f.debug_tuple("GroupTwo::G2F32").field(&v).finish(),
                        Self::G2String(v) => f.debug_tuple("GroupTwo::G2String").field(&v).finish(),
                        Self::G2Submsg(v) => f.debug_tuple("GroupTwo::G2Submsg").field(&v).finish(),
                    }
                }
            }

            impl<G2F32, G2String, G2Submsg> ::std::clone::Clone for GroupTwo<G2F32, G2String, G2Submsg>
            where
                G2F32: ::std::clone::Clone,
                G2String: ::std::clone::Clone,
                G2Submsg: ::std::clone::Clone,
            {
                fn clone(&self) -> Self {
                    match self {
                        Self::G2F32(ref v) => Self::G2F32(<G2F32 as ::std::clone::Clone>::clone(v)),
                        Self::G2String(ref v) => {
                            Self::G2String(<G2String as ::std::clone::Clone>::clone(v))
                        }
                        Self::G2Submsg(ref v) => {
                            Self::G2Submsg(<G2Submsg as ::std::clone::Clone>::clone(v))
                        }
                    }
                }
            }

            impl<G2F32, G2String, G2Submsg> ::std::cmp::PartialEq for GroupTwo<G2F32, G2String, G2Submsg>
            where
                G2F32: ::std::cmp::PartialEq,
                G2String: ::std::cmp::PartialEq,
                G2Submsg: ::std::cmp::PartialEq,
            {
                fn eq(&self, rhs: &Self) -> bool {
                    match (self, rhs) {
                        (Self::G2F32(left), Self::G2F32(right)) => left == right,
                        (Self::G2String(left), Self::G2String(right)) => left == right,
                        (Self::G2Submsg(left), Self::G2Submsg(right)) => left == right,
                        #[allow(unreachable_patterns)]
                        _ => false,
                    }
                }
            }

            pub enum GroupThree<G3Int32> {
                G3Int32(G3Int32),
            }
            impl<G3Int32> GroupThree<G3Int32> {
                pub fn g3_int32(self) -> ::std::option::Option<G3Int32> {
                    match self {
                        Self::G3Int32(v) => ::std::option::Option::Some(v),
                        #[allow(unreachable_patterns)]
                        _ => None,
                    }
                }
            }

            impl<G3Int32> ::std::fmt::Debug for GroupThree<G3Int32>
            where
                G3Int32: ::std::fmt::Debug,
            {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    match self {
                        Self::G3Int32(v) => f.debug_tuple("GroupThree::G3Int32").field(&v).finish(),
                    }
                }
            }

            impl<G3Int32> ::std::clone::Clone for GroupThree<G3Int32>
            where
                G3Int32: ::std::clone::Clone,
            {
                fn clone(&self) -> Self {
                    match self {
                        Self::G3Int32(ref v) => {
                            Self::G3Int32(<G3Int32 as ::std::clone::Clone>::clone(v))
                        }
                    }
                }
            }

            impl<G3Int32> ::std::cmp::PartialEq for GroupThree<G3Int32>
            where
                G3Int32: ::std::cmp::PartialEq,
            {
                fn eq(&self, rhs: &Self) -> bool {
                    match (self, rhs) {
                        (Self::G3Int32(left), Self::G3Int32(right)) => left == right,
                        #[allow(unreachable_patterns)]
                        _ => false,
                    }
                }
            }
        }
    }
    pub mod submsg {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
    }
}
