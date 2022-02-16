// A generated source code by puroro library
// package library

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

//pub use _puroro_simple_impl::Book;
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
    pub trait BookTemplateFieldTypes {
        type ImplTag;
        type TitleType;
        type NumPagesType;
    }

    impl BookTemplateFieldTypes for ::puroro::internal::OwnedFields {
        type ImplTag = ::puroro::tags::OwnedImpl;
        type TitleType = ::std::string::String;
        type NumPagesType = u32;
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
    pub mod book {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
    }
}
