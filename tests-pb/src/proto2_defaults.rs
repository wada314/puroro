// A generated source code by puroro library
// package proto2_defaults

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub use _puroro_simple_impl::Msg;
pub use _puroro_simple_impl::Submsg;
pub mod _puroro_simple_impl {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct Msg {
        pub i32_default: ::std::option::Option<i32>,
        pub i32_0: ::std::option::Option<i32>,
        pub f32_unlabeled: ::std::option::Option<f32>,
        pub string_unlabeled: ::std::option::Option<::std::string::String>,
        pub submsg_unlabeled: ::std::option::Option<
            ::std::boxed::Box<self::_puroro_root::proto2_defaults::_puroro_simple_impl::Submsg>,
        >,
    }
    impl ::puroro::Message<Msg> for Msg {}

    impl super::_puroro_traits::MsgTrait for Msg {
        fn i32_default<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.i32_default)
        }
        fn i32_0<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.i32_0)
        }
        fn f32_unlabeled<'this>(&'this self) -> Option<f32> {
            Clone::clone(&self.f32_unlabeled)
        }
        fn string_unlabeled<'this>(&'this self) -> Option<&'this str> {
            self.string_unlabeled.as_ref().map(|v| v.as_ref())
        }
        type Field6MessageType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::proto2_defaults::_puroro_simple_impl::Submsg;
        fn submsg_unlabeled<'this>(&'this self) -> Option<Self::Field6MessageType<'this>> {
            self.submsg_unlabeled.as_ref().map(|v| v.as_ref())
        }
    }

    impl ::puroro::MessageRepresentativeImpl for Msg {
        fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
            use ::puroro::once_cell::sync::Lazy;
            static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 5]> =
                Lazy::new(|| {
                    [
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "i32_default",
                                number: 1,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "i32_0",
                                number: 2,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "f32_unlabeled",
                                number: 4,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "string_unlabeled",
                                number: 5,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "submsg_unlabeled",
                                number: 6,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                    ]
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

    impl ::puroro::internal::DeserializableMessageFromBytesIterator for Msg {
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
                ::puroro::tags::Optional, ::puroro::tags::Int32
            >::deser_field(&mut self.i32_default, data),
            2 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Int32
            >::deser_field(&mut self.i32_0, data),
            4 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Float
            >::deser_field(&mut self.f32_unlabeled, data),
            5 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.string_unlabeled, data),
            6 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Message<::std::boxed::Box<self::_puroro_root::proto2_defaults::_puroro_simple_impl::Submsg>>
            >::deser_field(&mut self.submsg_unlabeled, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for Msg {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Int32>::ser_field(
                &self.i32_default,
                1,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Int32>::ser_field(
                &self.i32_0,
                2,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Float>::ser_field(
                &self.f32_unlabeled,
                4,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.string_unlabeled,
                5,
                out,
            )?;
            SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    ::std::boxed::Box<
                        self::_puroro_root::proto2_defaults::_puroro_simple_impl::Submsg,
                    >,
                >,
            >::ser_field(&self.submsg_unlabeled, 6, out)?;

            ::std::result::Result::Ok(())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct Submsg {
        pub i32_default: ::std::option::Option<i32>,
    }
    impl ::puroro::Message<Submsg> for Submsg {}

    impl super::_puroro_traits::SubmsgTrait for Submsg {
        fn i32_default<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.i32_default)
        }
    }

    impl ::puroro::MessageRepresentativeImpl for Submsg {
        fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
            use ::puroro::once_cell::sync::Lazy;
            static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 1]> =
                Lazy::new(|| {
                    [{
                        let init = ::puroro::internal::FieldDescriptorInitializer {
                            name: "i32_default",
                            number: 1,
                            lazy_containing_type: Lazy::new(|| {
                                <Submsg as ::puroro::MessageRepresentativeImpl>::descriptor()
                            }),
                        };
                        ::puroro::internal::init_field_descriptor(init)
                    }]
                });
            static LAZY_DESCRIPTOR: Lazy<::puroro::desc::MessageDescriptor> = Lazy::new(|| {
                let init = ::puroro::internal::MessageDescriptorInitializer {
                    name: "Submsg",
                    lazy_fields: Lazy::new(|| Lazy::force(&LAZY_FIELD_DESCRIPTOR_ARRAY).as_ref()),
                };
                ::puroro::internal::init_message_descriptor(init)
            });
            Lazy::force(&LAZY_DESCRIPTOR)
        }
    }

    impl ::puroro::internal::DeserializableMessageFromBytesIterator for Submsg {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro::internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro::internal::de::DeserFieldsFromBytesIter for Submsg {
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
                ::puroro::tags::Optional, ::puroro::tags::Int32
            >::deser_field(&mut self.i32_default, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for Submsg {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Int32>::ser_field(
                &self.i32_default,
                1,
                out,
            )?;

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
        type Field6MessageType<'this>
        where
            Self: 'this,
        = ();
    }
    impl<T, U> MsgTrait for (T, U)
    where
        T: MsgTrait,
        U: MsgTrait,
    {
        fn i32_default<'this>(&'this self) -> Option<i32> {
            <U as MsgTrait>::i32_default(&self.1).or_else(|| <T as MsgTrait>::i32_default(&self.0))
        }
        fn i32_0<'this>(&'this self) -> Option<i32> {
            <U as MsgTrait>::i32_0(&self.1).or_else(|| <T as MsgTrait>::i32_0(&self.0))
        }
        fn f32_unlabeled<'this>(&'this self) -> Option<f32> {
            <U as MsgTrait>::f32_unlabeled(&self.1)
                .or_else(|| <T as MsgTrait>::f32_unlabeled(&self.0))
        }
        fn string_unlabeled<'this>(&'this self) -> Option<&'this str> {
            <U as MsgTrait>::string_unlabeled(&self.1)
                .or_else(|| <T as MsgTrait>::string_unlabeled(&self.0))
        }
        type Field6MessageType<'this>
        where
            Self: 'this,
        = (
            ::std::option::Option<<T as MsgTrait>::Field6MessageType<'this>>,
            ::std::option::Option<<U as MsgTrait>::Field6MessageType<'this>>,
        );
        fn submsg_unlabeled<'this>(&'this self) -> Option<Self::Field6MessageType<'this>> {
            match (
                <T as MsgTrait>::submsg_unlabeled(&self.0),
                <U as MsgTrait>::submsg_unlabeled(&self.1),
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
        fn i32_default<'this>(&'this self) -> Option<i32> {
            self.as_ref().either(
                |t| <T as MsgTrait>::i32_default(t),
                |u| <U as MsgTrait>::i32_default(u),
            )
        }
        fn i32_0<'this>(&'this self) -> Option<i32> {
            self.as_ref()
                .either(|t| <T as MsgTrait>::i32_0(t), |u| <U as MsgTrait>::i32_0(u))
        }
        fn f32_unlabeled<'this>(&'this self) -> Option<f32> {
            self.as_ref().either(
                |t| <T as MsgTrait>::f32_unlabeled(t),
                |u| <U as MsgTrait>::f32_unlabeled(u),
            )
        }
        fn string_unlabeled<'this>(&'this self) -> Option<&'this str> {
            self.as_ref().either(
                |t| <T as MsgTrait>::string_unlabeled(t),
                |u| <U as MsgTrait>::string_unlabeled(u),
            )
        }
        type Field6MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as MsgTrait>::Field6MessageType<'this>,
            <U as MsgTrait>::Field6MessageType<'this>,
        >;
        fn submsg_unlabeled<'this>(&'this self) -> Option<Self::Field6MessageType<'this>> {
            self.as_ref().either(
                |t| <T as MsgTrait>::submsg_unlabeled(t).map(|t| ::puroro::Either::Left(t)),
                |u| <U as MsgTrait>::submsg_unlabeled(u).map(|u| ::puroro::Either::Right(u)),
            )
        }
    }
    impl<T> MsgTrait for ::std::option::Option<T>
    where
        T: MsgTrait,
    {
        fn i32_default<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().and_then(|msg| msg.i32_default())
        }
        fn i32_0<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().and_then(|msg| msg.i32_0())
        }
        fn f32_unlabeled<'this>(&'this self) -> ::std::option::Option<f32> {
            self.as_ref().and_then(|msg| msg.f32_unlabeled())
        }
        fn string_unlabeled<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().and_then(|msg| msg.string_unlabeled())
        }
        type Field6MessageType<'this>
        where
            Self: 'this,
        = T::Field6MessageType<'this>;
        fn submsg_unlabeled<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field6MessageType<'this>> {
            self.as_ref().and_then(|msg| msg.submsg_unlabeled())
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField1 {
        pub i32_default: ::std::option::Option<i32>,
    }

    impl ::puroro::Message<super::Msg> for MsgSingleField1 {}

    impl super::_puroro_traits::MsgTrait for MsgSingleField1 {
        fn i32_default<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.i32_default)
        }
        type Field6MessageType<'this>
        where
            Self: 'this,
        = ();
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for MsgSingleField1 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Int32>::ser_field::<
                (),
                _,
                _,
            >(&self.i32_default, 1, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<i32>> for MsgSingleField1 {
        fn from(value: ::std::option::Option<i32>) -> Self {
            Self { i32_default: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField2 {
        pub i32_0: ::std::option::Option<i32>,
    }

    impl ::puroro::Message<super::Msg> for MsgSingleField2 {}

    impl super::_puroro_traits::MsgTrait for MsgSingleField2 {
        fn i32_0<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.i32_0)
        }
        type Field6MessageType<'this>
        where
            Self: 'this,
        = ();
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for MsgSingleField2 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Int32>::ser_field::<
                (),
                _,
                _,
            >(&self.i32_0, 2, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<i32>> for MsgSingleField2 {
        fn from(value: ::std::option::Option<i32>) -> Self {
            Self { i32_0: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField4 {
        pub f32_unlabeled: ::std::option::Option<f32>,
    }

    impl ::puroro::Message<super::Msg> for MsgSingleField4 {}

    impl super::_puroro_traits::MsgTrait for MsgSingleField4 {
        fn f32_unlabeled<'this>(&'this self) -> Option<f32> {
            Clone::clone(&self.f32_unlabeled)
        }
        type Field6MessageType<'this>
        where
            Self: 'this,
        = ();
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for MsgSingleField4 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Float>::ser_field::<
                (),
                _,
                _,
            >(&self.f32_unlabeled, 4, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<f32>> for MsgSingleField4 {
        fn from(value: ::std::option::Option<f32>) -> Self {
            Self {
                f32_unlabeled: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField5<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub string_unlabeled: ::std::option::Option<ScalarType>,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField5<ScalarType> where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField5<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn string_unlabeled<'this>(&'this self) -> Option<&'this str> {
            self.string_unlabeled.as_ref().map(|r| r.as_ref())
        }
        type Field6MessageType<'this>
        where
            Self: 'this,
        = ();
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite for MsgSingleField5<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field::<
                ScalarType,
                _,
                _,
            >(&self.string_unlabeled, 5, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<::std::option::Option<ScalarType>>
        for MsgSingleField5<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ::std::option::Option<ScalarType>) -> Self {
            Self {
                string_unlabeled: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField6<ScalarType>
    where
        ScalarType: self::_puroro_root::proto2_defaults::_puroro_traits::SubmsgTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub submsg_unlabeled: ::std::option::Option<ScalarType>,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField6<ScalarType> where
        ScalarType: self::_puroro_root::proto2_defaults::_puroro_traits::SubmsgTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField6<ScalarType>
    where
        ScalarType: self::_puroro_root::proto2_defaults::_puroro_traits::SubmsgTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field6MessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;
        fn submsg_unlabeled<'this>(&'this self) -> Option<Self::Field6MessageType<'this>> {
            self.submsg_unlabeled.as_ref()
        }
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite for MsgSingleField6<ScalarType>
    where
        ScalarType: self::_puroro_root::proto2_defaults::_puroro_traits::SubmsgTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        ScalarType: ::puroro::internal::SerializableMessageToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
            ::puroro::tags::Optional, ::puroro::tags::Message<ScalarType>
        >::ser_field::
        <ScalarType, _, _>
        (
            &self.submsg_unlabeled,
            6,
            out
        )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<::std::option::Option<ScalarType>>
        for MsgSingleField6<ScalarType>
    where
        ScalarType: self::_puroro_root::proto2_defaults::_puroro_traits::SubmsgTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ::std::option::Option<ScalarType>) -> Self {
            Self {
                submsg_unlabeled: value,
            }
        }
    }
    pub struct MsgBuilder<T>(T);

    impl<T> MsgBuilder<T>
    where
        T: MsgTrait,
    {
        pub fn append_i32_default(
            self,
            value: ::std::option::Option<i32>,
        ) -> MsgBuilder<(T, MsgSingleField1)> {
            MsgBuilder((self.0, MsgSingleField1 { i32_default: value }))
        }

        pub fn append_i32_0(
            self,
            value: ::std::option::Option<i32>,
        ) -> MsgBuilder<(T, MsgSingleField2)> {
            MsgBuilder((self.0, MsgSingleField2 { i32_0: value }))
        }

        pub fn append_f32_unlabeled(
            self,
            value: ::std::option::Option<f32>,
        ) -> MsgBuilder<(T, MsgSingleField4)> {
            MsgBuilder((
                self.0,
                MsgSingleField4 {
                    f32_unlabeled: value,
                },
            ))
        }

        pub fn append_string_unlabeled<ScalarType>(
            self,
            value: ::std::option::Option<ScalarType>,
        ) -> MsgBuilder<(T, MsgSingleField5<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            MsgBuilder((
                self.0,
                MsgSingleField5 {
                    string_unlabeled: value,
                },
            ))
        }

        pub fn append_submsg_unlabeled<ScalarType>(
            self,
            value: ::std::option::Option<ScalarType>,
        ) -> MsgBuilder<(T, MsgSingleField6<ScalarType>)>
        where
            ScalarType: self::_puroro_root::proto2_defaults::_puroro_traits::SubmsgTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            MsgBuilder((
                self.0,
                MsgSingleField6 {
                    submsg_unlabeled: value,
                },
            ))
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
    impl SubmsgTrait for () {}
    impl<T, U> SubmsgTrait for (T, U)
    where
        T: SubmsgTrait,
        U: SubmsgTrait,
    {
        fn i32_default<'this>(&'this self) -> Option<i32> {
            <U as SubmsgTrait>::i32_default(&self.1)
                .or_else(|| <T as SubmsgTrait>::i32_default(&self.0))
        }
    }
    impl<T, U> SubmsgTrait for ::puroro::Either<T, U>
    where
        T: SubmsgTrait,
        U: SubmsgTrait,
    {
        fn i32_default<'this>(&'this self) -> Option<i32> {
            self.as_ref().either(
                |t| <T as SubmsgTrait>::i32_default(t),
                |u| <U as SubmsgTrait>::i32_default(u),
            )
        }
    }
    impl<T> SubmsgTrait for ::std::option::Option<T>
    where
        T: SubmsgTrait,
    {
        fn i32_default<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().and_then(|msg| msg.i32_default())
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct SubmsgSingleField1 {
        pub i32_default: ::std::option::Option<i32>,
    }

    impl ::puroro::Message<super::Submsg> for SubmsgSingleField1 {}

    impl super::_puroro_traits::SubmsgTrait for SubmsgSingleField1 {
        fn i32_default<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.i32_default)
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for SubmsgSingleField1 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Int32>::ser_field::<
                (),
                _,
                _,
            >(&self.i32_default, 1, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<i32>> for SubmsgSingleField1 {
        fn from(value: ::std::option::Option<i32>) -> Self {
            Self { i32_default: value }
        }
    }
    pub struct SubmsgBuilder<T>(T);

    impl<T> SubmsgBuilder<T>
    where
        T: SubmsgTrait,
    {
        pub fn append_i32_default(
            self,
            value: ::std::option::Option<i32>,
        ) -> SubmsgBuilder<(T, SubmsgSingleField1)> {
            SubmsgBuilder((self.0, SubmsgSingleField1 { i32_default: value }))
        }

        pub fn build(self) -> T {
            self.0
        }
    }

    impl SubmsgBuilder<()> {
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
        fn i32_default<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::default::Default::default()
        }
        fn i32_0<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::default::Default::default()
        }
        fn f32_unlabeled<'this>(&'this self) -> ::std::option::Option<f32> {
            ::std::default::Default::default()
        }
        fn string_unlabeled<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::default::Default::default()
        }
        type Field6MessageType<'this>:
            self::_puroro_root::proto2_defaults::_puroro_traits::SubmsgTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
            where Self: 'this;
        fn submsg_unlabeled<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field6MessageType<'this>> {
            ::std::default::Default::default()
        }
    }

    macro_rules! msg_delegate {
        ($ty:ty) => {
            fn i32_default<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).i32_default()
            }
            fn i32_0<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).i32_0()
            }
            fn f32_unlabeled<'this>(&'this self) -> ::std::option::Option<f32> {
                (**self).f32_unlabeled()
            }
            fn string_unlabeled<'this>(&'this self) -> ::std::option::Option<&'this str> {
                (**self).string_unlabeled()
            }
            type Field6MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field6MessageType<'this>;
            fn submsg_unlabeled<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field6MessageType<'this>> {
                (**self).submsg_unlabeled()
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
    pub trait SubmsgTrait {
        fn i32_default<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::default::Default::default()
        }
    }

    macro_rules! submsg_delegate {
        ($ty:ty) => {
            fn i32_default<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).i32_default()
            }
        };
    }

    impl<T> SubmsgTrait for &'_ T
    where
        T: SubmsgTrait,
    {
        submsg_delegate!(T);
    }

    impl<T> SubmsgTrait for &'_ mut T
    where
        T: SubmsgTrait,
    {
        submsg_delegate!(T);
    }

    impl<T> SubmsgTrait for ::std::boxed::Box<T>
    where
        T: SubmsgTrait,
    {
        submsg_delegate!(T);
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
