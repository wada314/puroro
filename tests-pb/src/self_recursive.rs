// A generated source code by puroro library
// package self_recursive

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub use _puroro_impls::Msg_Simple as Msg;
pub mod _puroro_impls {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }

    #[allow(non_camel_case_types)]
    #[derive(Clone, Default, PartialEq, Debug)]
    pub struct Msg_Simple {
        pub recursive_unlabeled: ::std::option::Option<
            ::std::boxed::Box<self::_puroro_root::self_recursive::_puroro_impls::Msg_Simple>,
        >,
    }
    impl ::puroro::Message for Msg_Simple {}

    impl super::_puroro_traits::MsgTrait for Msg_Simple {
        type Field1MessageType<'this> =
            self::_puroro_root::self_recursive::_puroro_impls::Msg_Simple;
        fn recursive_unlabeled<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field1MessageType<'this>>>
        {
            self.recursive_unlabeled
                .as_ref()
                .map(|boxed| ::std::borrow::Cow::Borrowed(boxed.as_ref()))
        }
    }

    impl ::puroro::DeserFromBytesIter for Msg_Simple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for Msg_Simple {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
                1 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled,
                    ::puroro::tags::Message<
                        self::_puroro_root::self_recursive::_puroro_impls::Msg_Simple,
                    >,
                >::deser_field(&mut self.recursive_unlabeled, data),

                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl ::puroro::SerToIoWrite for Msg_Simple {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Message<
                    self::_puroro_root::self_recursive::_puroro_impls::Msg_Simple,
                >,
            >::ser_field(&self.recursive_unlabeled, 1, out)?;
            ::std::result::Result::Ok(())
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(Clone, Default, PartialEq, Debug)]
    pub struct Msg_Empty;

    impl ::puroro::Message for Msg_Empty {}

    impl super::_puroro_traits::MsgTrait for Msg_Empty {
        type Field1MessageType<'this> =
            self::_puroro_root::self_recursive::_puroro_impls::Msg_Simple;
        fn recursive_unlabeled<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field1MessageType<'this>>>
        {
            None
        }
    }

    impl ::puroro::SerToIoWrite for Msg_Empty {
        fn ser<W>(&self, _out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::std::result::Result::Ok(())
        }
    }
}
pub use _puroro_traits::*;
pub mod _puroro_traits {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }

    pub trait MsgTrait: ::std::clone::Clone {
        type Field1MessageType<'this>: 'this
            + self::_puroro_root::self_recursive::_puroro_traits::MsgTrait;
        fn recursive_unlabeled<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field1MessageType<'this>>>;
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
