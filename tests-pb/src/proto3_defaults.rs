// A generated source code by puroro library
// package proto3_defaults

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
        type I32UnlabeledType;
        type I32OptionalType;
        type I32RepeatedType;
        type F32UnlabeledType;
        type StringUnlabeledType;
        type SubmsgUnlabeledType;
    }

    impl MsgTemplateFieldTypes for ::puroro::internal::SimpleFields {
        type ImplTag = ::puroro::tags::SimpleImpl;
        type I32UnlabeledType = i32;
        type I32OptionalType = i32;
        type I32RepeatedType = ::std::vec::Vec<i32>;
        type F32UnlabeledType = f32;
        type StringUnlabeledType = ::std::string::String;
        type SubmsgUnlabeledType = ::std::option::Option<
            ::std::boxed::Box<self::_puroro_root::proto3_defaults::_puroro_simple_impl::Submsg>,
        >;
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
    }
    pub mod submsg {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
    }
}
