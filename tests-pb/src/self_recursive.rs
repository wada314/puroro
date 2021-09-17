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
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct Msg {
        pub recursive_unlabeled: ::std::option::Option<
            ::std::boxed::Box<self::_puroro_root::self_recursive::_puroro_simple_impl::Msg>,
        >,
    }
    impl ::puroro::Message<Msg> for Msg {}

    impl super::_puroro_traits::MsgTrait for Msg {
        type Field1MessageType<'this> =
            &'this self::_puroro_root::self_recursive::_puroro_simple_impl::Msg;
        fn recursive_unlabeled<'this>(&'this self) -> Option<Self::Field1MessageType<'this>> {
            self.recursive_unlabeled.as_ref().map(|v| v.as_ref())
        }
    }

    impl ::puroro::MessageRepresentativeImpl for Msg {
        fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
            use ::puroro::once_cell::sync::Lazy;
            static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 1]> =
                Lazy::new(|| {
                    [{
                        let init = ::puroro::internal::FieldDescriptorInitializer {
                            name: "recursive_unlabeled",
                            number: 1,
                            lazy_containing_type: Lazy::new(|| {
                                <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                            }),
                        };
                        ::puroro::internal::init_field_descriptor(init)
                    }]
                });
            static LAZY_DESCRIPTOR: Lazy<::puroro::desc::MessageDescriptor> = Lazy::new(|| {
                let init = ::puroro::internal::MessageDescriptorInitializer {
                    name: "Msg",
                    lazy_fields: Lazy::new(|| Lazy::force(&LAZY_FIELD_DESCRIPTOR_ARRAY).as_ref()),
                };
                ::puroro::internal::init_message_descriptor(init)
            });
            Lazy::force(&LAZY_DESCRIPTOR)
        }
    }

    impl ::puroro::DeserFromBytesIter for Msg {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro::internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro::internal::de::DeserFieldsFromBytesIter for Msg {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro::internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro::internal::impls::simple::de::DeserFieldFromBytesIter;
            match field_number {
                1 => DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled,
                    ::puroro::tags::Message<
                        self::_puroro_root::self_recursive::_puroro_simple_impl::Msg,
                    >,
                >::deser_field(&mut self.recursive_unlabeled, data),

                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl ::puroro::SerToIoWrite for Msg {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Message<
                    self::_puroro_root::self_recursive::_puroro_simple_impl::Msg,
                >,
            >::ser_field(&self.recursive_unlabeled, 1, out)?;

            ::std::result::Result::Ok(())
        }
    }
}

pub use _puroro_impls::*;
pub mod _puroro_impls {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
    use super::_puroro_traits::*;
    impl MsgTrait for () {
        type Field1MessageType<'this> = ();
    }
    impl<T, U> MsgTrait for (T, U)
    where
        T: MsgTrait,
        U: MsgTrait,
    {
        type Field1MessageType<'this> = (
            ::std::option::Option<<T as MsgTrait>::Field1MessageType<'this>>,
            ::std::option::Option<<U as MsgTrait>::Field1MessageType<'this>>,
        );
        fn recursive_unlabeled<'this>(&'this self) -> Option<Self::Field1MessageType<'this>> {
            match (
                <T as MsgTrait>::recursive_unlabeled(&self.0),
                <U as MsgTrait>::recursive_unlabeled(&self.1),
            ) {
                (None, None) => None,
                (Some(t), None) => Some((Some(t), None)),
                (None, Some(u)) => Some((None, Some(u))),
                (Some(t), Some(u)) => Some((Some(t), Some(u))),
            }
        }
    }
    impl<T, U> MsgTrait for ::puroro::Either<T, U>
    where
        T: MsgTrait,
        U: MsgTrait,
    {
        type Field1MessageType<'this> = ::puroro::Either<
            <T as MsgTrait>::Field1MessageType<'this>,
            <U as MsgTrait>::Field1MessageType<'this>,
        >;
        fn recursive_unlabeled<'this>(&'this self) -> Option<Self::Field1MessageType<'this>> {
            self.as_ref().either(
                |t| <T as MsgTrait>::recursive_unlabeled(t).map(|t| ::puroro::Either::Left(t)),
                |u| <U as MsgTrait>::recursive_unlabeled(u).map(|u| ::puroro::Either::Right(u)),
            )
        }
    }
    impl<T> MsgTrait for ::std::option::Option<T>
    where
        T: MsgTrait,
    {
        type Field1MessageType<'this> = T::Field1MessageType<'this>;
        fn recursive_unlabeled<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field1MessageType<'this>> {
            self.as_ref().and_then(|msg| msg.recursive_unlabeled())
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct MsgSimpleField1 {
        pub recursive_unlabeled: ::std::option::Option<
            ::std::boxed::Box<self::_puroro_root::self_recursive::_puroro_simple_impl::Msg>,
        >,
    }

    impl ::puroro::Message<super::Msg> for MsgSimpleField1 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField1 {
        type Field1MessageType<'this> =
            &'this self::_puroro_root::self_recursive::_puroro_simple_impl::Msg;
        fn recursive_unlabeled<'this>(&'this self) -> Option<Self::Field1MessageType<'this>> {
            self.recursive_unlabeled.as_ref().map(|v| v.as_ref())
        }
    }

    impl ::puroro::SerToIoWrite for MsgSimpleField1 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Message<
                    self::_puroro_root::self_recursive::_puroro_simple_impl::Msg,
                >,
            >::ser_field(&self.recursive_unlabeled, 1, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl
        ::std::convert::From<
            ::std::option::Option<
                ::std::boxed::Box<self::_puroro_root::self_recursive::_puroro_simple_impl::Msg>,
            >,
        > for MsgSimpleField1
    {
        fn from(
            value: ::std::option::Option<
                ::std::boxed::Box<self::_puroro_root::self_recursive::_puroro_simple_impl::Msg>,
            >,
        ) -> Self {
            Self {
                recursive_unlabeled: value,
            }
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct MsgSimpleByValue {}
    impl ::puroro::Message<super::Msg> for MsgSimpleByValue {}

    impl MsgTrait for MsgSimpleByValue {
        type Field1MessageType<'this> =
            ::std::boxed::Box<self::_puroro_root::self_recursive::_puroro_simple_impl::Msg>;
        fn recursive_unlabeled<'this>(&'this self) -> Option<Self::Field1MessageType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
    }
    pub struct MsgBuilder<T>(T);

    impl<T> MsgBuilder<T>
    where
        T: MsgTrait,
    {
        pub fn append_recursive_unlabeled(
            self,
            value: ::std::option::Option<
                ::std::boxed::Box<self::_puroro_root::self_recursive::_puroro_simple_impl::Msg>,
            >,
        ) -> MsgBuilder<(T, MsgSimpleField1)> {
            MsgBuilder((self.0, ::std::convert::From::from(value)))
        }

        pub fn build(self) -> T {
            self.0
        }
    }

    impl MsgBuilder<()> {
        pub fn new() -> Self {
            Self(())
        }
    }
}
pub use _puroro_traits::*;
pub mod _puroro_traits {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }

    pub trait MsgTrait {
        type Field1MessageType<'this>: self::_puroro_root::self_recursive::_puroro_traits::MsgTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug;
        fn recursive_unlabeled<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field1MessageType<'this>> {
            ::std::default::Default::default()
        }
    }

    macro_rules! msg_delegate {
        ($ty:ty) => {
            type Field1MessageType<'this> = <$ty>::Field1MessageType<'this>;
            fn recursive_unlabeled<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field1MessageType<'this>> {
                (**self).recursive_unlabeled()
            }
        };
    }

    impl<T> MsgTrait for &'_ T
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
}
pub use _puroro_nested::*;
pub mod _puroro_nested {
    pub mod msg {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
    }
}
