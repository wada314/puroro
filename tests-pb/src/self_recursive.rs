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
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct MsgSimple {
        pub recursive_unlabeled: ::std::option::Option<
            ::std::boxed::Box<self::_puroro_root::self_recursive::_puroro_impls::MsgSimple>,
        >,
    }
    impl ::puroro::Message for MsgSimple {}

    impl super::_puroro_traits::MsgTrait for MsgSimple {
        type Field1MessageType<'this> =
            self::_puroro_root::self_recursive::_puroro_impls::MsgSimple;
        type Field1ScalarGetterType<'this> = &'this Self::Field1MessageType<'this>;
        fn recursive_unlabeled<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field1ScalarGetterType<'this>> {
            self.recursive_unlabeled.as_ref().map(|v| v.as_ref())
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
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct MsgEmpty;

    impl ::puroro::Message for MsgEmpty {}

    impl super::_puroro_traits::MsgTrait for MsgEmpty {
        type Field1MessageType<'this> = self::_puroro_root::self_recursive::_puroro_impls::MsgEmpty;
        type Field1ScalarGetterType<'this> = &'static Self::Field1MessageType<'this>;
    }

    impl ::puroro::SerToIoWrite for MsgEmpty {
        fn ser<W>(&self, _out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::std::result::Result::Ok(())
        }
    }
    impl<T, U> super::_puroro_traits::MsgTrait for ::puroro::EitherOrBoth<T, U>
    where
        T: ::std::ops::Deref,
        U: ::std::ops::Deref,
        T::Target: super::_puroro_traits::MsgTrait,
        U::Target: super::_puroro_traits::MsgTrait,
    {
        type Field1MessageType<'this> = ::puroro::EitherOrBoth<
            <T::Target as super::_puroro_traits::MsgTrait>::Field1ScalarGetterType<'this>,
            <U::Target as super::_puroro_traits::MsgTrait>::Field1ScalarGetterType<'this>,
        >;
        type Field1ScalarGetterType<'this> =
            ::puroro_internal::Derefable<Self::Field1MessageType<'this>>;
        fn recursive_unlabeled<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field1ScalarGetterType<'this>> {
            todo!()
        }
    }
    impl<T, U> super::_puroro_traits::MsgTrait for ::puroro::Either<T, U>
    where
        T: ::std::ops::Deref,
        U: ::std::ops::Deref,
        T::Target: super::_puroro_traits::MsgTrait,
        U::Target: super::_puroro_traits::MsgTrait,
    {
        type Field1MessageType<'this> = ::puroro::Either<
            <T::Target as super::_puroro_traits::MsgTrait>::Field1ScalarGetterType<'this>,
            <U::Target as super::_puroro_traits::MsgTrait>::Field1ScalarGetterType<'this>,
        >;
        type Field1ScalarGetterType<'this> =
            ::puroro_internal::Derefable<Self::Field1MessageType<'this>>;
        fn recursive_unlabeled<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field1ScalarGetterType<'this>> {
            self.as_ref().either(
                |t| {
                    <T::Target as super::_puroro_traits::MsgTrait>::recursive_unlabeled(t)
                        .map(|t| ::puroro_internal::Derefable::new(::puroro::Either::Left(t)))
                },
                |u| {
                    <U::Target as super::_puroro_traits::MsgTrait>::recursive_unlabeled(u)
                        .map(|u| ::puroro_internal::Derefable::new(::puroro::Either::Right(u)))
                },
            )
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField1 {
        recursive_unlabeled: ::std::option::Option<
            ::std::boxed::Box<self::_puroro_root::self_recursive::_puroro_impls::MsgSimple>,
        >,
    }

    impl ::puroro::Message for MsgSimpleField1 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField1 {
        type Field1MessageType<'this> =
            self::_puroro_root::self_recursive::_puroro_impls::MsgSimple;
        type Field1ScalarGetterType<'this> = &'this Self::Field1MessageType<'this>;
        fn recursive_unlabeled<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field1ScalarGetterType<'this>> {
            self.recursive_unlabeled.as_ref().map(|v| v.as_ref())
        }
    }
}
pub use _puroro_traits::*;
pub mod _puroro_traits {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }

    pub trait MsgTrait {
        type Field1MessageType<'this>: self::_puroro_root::self_recursive::_puroro_traits::MsgTrait;
        type Field1ScalarGetterType<'this>: ::std::ops::Deref<
            Target = Self::Field1MessageType<'this>,
        >;
        fn recursive_unlabeled<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field1ScalarGetterType<'this>> {
            ::std::default::Default::default()
        }
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
