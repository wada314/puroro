// A generated source code by puroro library
// package official_samples2

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

//pub use _puroro_simple_impl::Test1;
//pub use _puroro_simple_impl::Test2;
//pub use _puroro_simple_impl::Test3;
//pub use _puroro_simple_impl::Test4;
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
    pub trait Test1TemplateFieldTypes {
        type ImplTag;
        type AType;
    }

    impl Test1TemplateFieldTypes for ::puroro::internal::OwnedFields {
        type ImplTag = ::puroro::tags::OwnedImpl;
        type AType = i32;
    }
    pub trait Test2TemplateFieldTypes {
        type ImplTag;
        type BType;
    }

    impl Test2TemplateFieldTypes for ::puroro::internal::OwnedFields {
        type ImplTag = ::puroro::tags::OwnedImpl;
        type BType = ::std::string::String;
    }
    pub trait Test3TemplateFieldTypes {
        type ImplTag;
        type CType;
    }

    impl Test3TemplateFieldTypes for ::puroro::internal::OwnedFields {
        type ImplTag = ::puroro::tags::OwnedImpl;
        type CType = ::std::option::Option<
            ::std::boxed::Box<self::_puroro_root::official_samples2::_puroro_simple_impl::Test1>,
        >;
    }
    pub trait Test4TemplateFieldTypes {
        type ImplTag;
        type DType;
    }

    impl Test4TemplateFieldTypes for ::puroro::internal::OwnedFields {
        type ImplTag = ::puroro::tags::OwnedImpl;
        type DType = ::std::vec::Vec<i32>;
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
    pub mod test1 {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
    }
    pub mod test2 {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
    }
    pub mod test3 {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
    }
    pub mod test4 {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
    }
}
