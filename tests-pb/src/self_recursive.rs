// A generated source code by puroro library
// package self_recursive

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub use _puroro_impls::MsgSimple as Msg;
pub mod _puroro_impls {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }

    #[derive(Clone, Default, PartialEq, Debug)]
    pub struct MsgSimple {
        pub recursive_unlabeled: ::std::option::Option<
            ::std::boxed::Box<self::_puroro_root::self_recursive::_puroro_impls::MsgSimple>,
        >,
    }
    impl ::puroro::Message for MsgSimple {}

    impl super::_puroro_traits::MsgTrait for MsgSimple {
        type Field1MessageType<'this> =
            self::_puroro_root::self_recursive::_puroro_impls::MsgSimple;
        fn recursive_unlabeled<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field1MessageType<'this>>>
        {
            self.recursive_unlabeled
                .as_ref()
                .map(|boxed| ::std::borrow::Cow::Borrowed(boxed.as_ref()))
        }
    }

    impl ::puroro::DeserFromBytesIter for MsgSimple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for MsgSimple {
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
                        self::_puroro_root::self_recursive::_puroro_impls::MsgSimple,
                    >,
                >::deser_field(&mut self.recursive_unlabeled, data),

                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl ::puroro::SerToIoWrite for MsgSimple {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Message<
                    self::_puroro_root::self_recursive::_puroro_impls::MsgSimple,
                >,
            >::ser_field(&self.recursive_unlabeled, 1, out)?;
            ::std::result::Result::Ok(())
        }
    }

    #[derive(Clone, Default, PartialEq, Debug)]
    pub struct MsgEmpty;

    impl ::puroro::Message for MsgEmpty {}

    impl super::_puroro_traits::MsgTrait for MsgEmpty {
        type Field1MessageType<'this> = self::_puroro_root::self_recursive::_puroro_impls::MsgEmpty;
        fn recursive_unlabeled<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field1MessageType<'this>>>
        {
            None
        }
    }

    impl ::puroro::SerToIoWrite for MsgEmpty {
        fn ser<W>(&self, _out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::std::result::Result::Ok(())
        }
    }

    #[derive(Clone, PartialEq, Debug)]
    struct MsgAppend1<Appendee: super::_puroro_traits::MsgTrait> {
        appendee: Appendee,
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
