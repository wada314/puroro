// A generated source code by puroro library
// package self_recursive

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub use _puroro_simple_impl::Msg;
pub mod _puroro_simple_impl {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
    pub struct Msg {
        recursive_unlabeled: ::std::option::Option<
            ::std::boxed::Box<self::_puroro_root::self_recursive::_puroro_simple_impl::Msg>,
        >,
    }
    impl ::puroro::Message<Msg> for Msg {}

    impl Msg {
        pub fn new() -> Self {
            Self {
                recursive_unlabeled: ::std::default::Default::default(),
            }
        }
        pub fn recursive_unlabeled_mut(
            &mut self,
        ) -> &mut ::std::option::Option<
            ::std::boxed::Box<self::_puroro_root::self_recursive::_puroro_simple_impl::Msg>,
        > {
            &mut self.recursive_unlabeled
        }
    }

    impl super::_puroro_traits::MsgTrait for Msg {
        type Field1ScalarGetterType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::self_recursive::_puroro_simple_impl::Msg;
        fn recursive_unlabeled_opt<'this>(
            &'this self,
        ) -> Option<Self::Field1ScalarGetterType<'this>> {
            self.recursive_unlabeled.as_ref().map(|v| v.as_ref())
        }
    }

    impl ::puroro::MessageRepresentativeImpl for Msg {}

    impl ::puroro::internal::de::DeserMessageFromBytesIter for Msg {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::internal::types::FieldData<
                &mut ::puroro::internal::de::from_iter::ScopedIter<I>,
            >,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro::internal::impls::simple::de::DeserFieldFromBytesIter;
            match field_number {
                1 => DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled,
                    ::puroro::tags::Message<
                        ::std::boxed::Box<
                            self::_puroro_root::self_recursive::_puroro_simple_impl::Msg,
                        >,
                    >,
                >::deser_field(&mut self.recursive_unlabeled, data),

                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl ::puroro::internal::se::SerMessageToIoWrite for Msg
    where
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Message<
                    <Self as super::_puroro_traits::MsgTrait>::Field1ScalarGetterType<'_>,
                >,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::recursive_unlabeled_opt(self),
                1,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::default::Default for Msg {
        fn default() -> Self {
            Self::new()
        }
    }

    impl ::std::fmt::Debug for Msg
    where
        Self: super::_puroro_traits::MsgTrait,
    {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.debug_struct("Msg")
                .field(
                    "recursive_unlabeled",
                    &<Self as super::_puroro_traits::MsgTrait>::recursive_unlabeled(self),
                )
                .finish()
        }
    }

    impl ::std::clone::Clone for Msg {
        fn clone(&self) -> Self {
            Self {
                recursive_unlabeled: ::std::clone::Clone::clone(&self.recursive_unlabeled),
            }
        }
    }

    impl ::std::cmp::PartialEq for Msg {
        fn eq(&self, rhs: &Self) -> bool {
            self.recursive_unlabeled == rhs.recursive_unlabeled && true
        }
    }
}

pub use _puroro_impls::*;
pub mod _puroro_impls {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
    use super::_puroro_traits::*;
}
pub use _puroro_traits::*;
pub mod _puroro_traits {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }

    pub trait MsgTrait {
        type Field1ScalarGetterType<'this>: self::_puroro_root::self_recursive::_puroro_traits::MsgTrait
            where Self: 'this;

        fn recursive_unlabeled<'this>(&'this self) -> Self::Field1ScalarGetterType<'this> {
            self.recursive_unlabeled_opt()
                .unwrap_or(Self::recursive_unlabeled_default_value())
        }
        fn recursive_unlabeled_default_value() -> Self::Field1ScalarGetterType<'static>;

        fn has_recursive_unlabeled<'this>(&'this self) -> bool {
            self.recursive_unlabeled_opt().is_some()
        }
        fn recursive_unlabeled_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field1ScalarGetterType<'this>> {
            ::std::option::Option::None
        }
    }

    macro_rules! msg_delegate {
        ($ty:ty) => {
            type Field1ScalarGetterType<'this>
            where
                Self: 'this,
            = <$ty as MsgTrait>::Field1ScalarGetterType<'this>;
            fn recursive_unlabeled_opt<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field1ScalarGetterType<'this>> {
                (**self).recursive_unlabeled_opt()
            }
            fn recursive_unlabeled_default_value()
            -> <$ty as MsgTrait>::Field1ScalarGetterType<'static> {
                <$ty as MsgTrait>::recursive_unlabeled_default_value()
            }
        };
    }

    impl<T> MsgTrait for &'_ T
    where
        T: MsgTrait,
    {
        msg_delegate!(T);
    }

    impl<T> MsgTrait for &'_ mut T
    where
        T: MsgTrait,
    {
        msg_delegate!(T);
    }

    impl<T> MsgTrait for ::std::boxed::Box<T>
    where
        T: MsgTrait,
    {
        msg_delegate!(T);
    }

    impl<'bump, T> MsgTrait for ::puroro::bumpalo::boxed::Box<'bump, T>
    where
        T: MsgTrait,
    {
        msg_delegate!(T);
    }

    impl<T> MsgTrait for ::puroro::BumpaloOwned<T>
    where
        T: MsgTrait,
    {
        msg_delegate!(T);
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
