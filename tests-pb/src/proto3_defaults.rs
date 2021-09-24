// A generated source code by puroro library
// package proto3_defaults

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
        pub i32_unlabeled: i32,
        pub i32_optional: ::std::option::Option<i32>,
        pub i32_repeated: ::std::vec::Vec<i32>,
        pub f32_unlabeled: f32,
        pub string_unlabeled: ::std::string::String,
        pub submsg_unlabeled: ::std::option::Option<
            ::std::boxed::Box<self::_puroro_root::proto3_defaults::_puroro_simple_impl::Submsg>,
        >,
    }
    impl ::puroro::Message<Msg> for Msg {}

    impl super::_puroro_traits::MsgTrait for Msg {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Clone::clone(&self.i32_unlabeled)
        }
        fn i32_optional<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.i32_optional)
        }
        type Field3RepeatedType<'this> = ::std::iter::Cloned<::std::slice::Iter<'this, i32>>;

        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            self.i32_repeated.iter().cloned()
        }
        fn f32_unlabeled<'this>(&'this self) -> f32 {
            Clone::clone(&self.f32_unlabeled)
        }
        type Field5StringType<'this>
        where
            Self: 'this,
        = &'this str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field5StringType<'this> {
            self.string_unlabeled.as_ref()
        }
        type Field6MessageType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::proto3_defaults::_puroro_simple_impl::Submsg;
        fn submsg_unlabeled<'this>(&'this self) -> Option<Self::Field6MessageType<'this>> {
            self.submsg_unlabeled.as_ref().map(|v| v.as_ref())
        }
    }

    impl ::puroro::MessageRepresentativeImpl for Msg {
        fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
            use ::puroro::once_cell::sync::Lazy;
            static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 6]> =
                Lazy::new(|| {
                    [
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "i32_unlabeled",
                                number: 1,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "i32_optional",
                                number: 2,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "i32_repeated",
                                number: 3,
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
            data: ::puroro::types::FieldData<&mut ::puroro::internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro::internal::impls::simple::de::DeserFieldFromBytesIter;
            match field_number {
            1 => DeserFieldFromBytesIter::<
                ::puroro::tags::Unlabeled, ::puroro::tags::Int32
            >::deser_field(&mut self.i32_unlabeled, data),
            2 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Int32
            >::deser_field(&mut self.i32_optional, data),
            3 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Int32
            >::deser_field(&mut self.i32_repeated, data),
            4 => DeserFieldFromBytesIter::<
                ::puroro::tags::Unlabeled, ::puroro::tags::Float
            >::deser_field(&mut self.f32_unlabeled, data),
            5 => DeserFieldFromBytesIter::<
                ::puroro::tags::Unlabeled, ::puroro::tags::String
            >::deser_field(&mut self.string_unlabeled, data),
            6 => DeserFieldFromBytesIter::<
                ::puroro::tags::Unlabeled, ::puroro::tags::Message<::std::boxed::Box<self::_puroro_root::proto3_defaults::_puroro_simple_impl::Submsg>>
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
            SerFieldToIoWrite::<::puroro::tags::Unlabeled, ::puroro::tags::Int32>::ser_field(
                &self.i32_unlabeled,
                1,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Int32>::ser_field(
                &self.i32_optional,
                2,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::Int32>::ser_field(
                &self.i32_repeated,
                3,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Unlabeled, ::puroro::tags::Float>::ser_field(
                &self.f32_unlabeled,
                4,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Unlabeled, ::puroro::tags::String>::ser_field(
                &self.string_unlabeled,
                5,
                out,
            )?;
            SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Message<
                    ::std::boxed::Box<
                        self::_puroro_root::proto3_defaults::_puroro_simple_impl::Submsg,
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
        pub i32_unlabeled: i32,
    }
    impl ::puroro::Message<Submsg> for Submsg {}

    impl super::_puroro_traits::SubmsgTrait for Submsg {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Clone::clone(&self.i32_unlabeled)
        }
    }

    impl ::puroro::MessageRepresentativeImpl for Submsg {
        fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
            use ::puroro::once_cell::sync::Lazy;
            static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 1]> =
                Lazy::new(|| {
                    [{
                        let init = ::puroro::internal::FieldDescriptorInitializer {
                            name: "i32_unlabeled",
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
            data: ::puroro::types::FieldData<&mut ::puroro::internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro::internal::impls::simple::de::DeserFieldFromBytesIter;
            match field_number {
            1 => DeserFieldFromBytesIter::<
                ::puroro::tags::Unlabeled, ::puroro::tags::Int32
            >::deser_field(&mut self.i32_unlabeled, data),

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
            SerFieldToIoWrite::<::puroro::tags::Unlabeled, ::puroro::tags::Int32>::ser_field(
                &self.i32_unlabeled,
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
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Default::default()
        }
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn f32_unlabeled<'this>(&'this self) -> f32 {
            Default::default()
        }
        type Field5StringType<'this>
        where
            Self: 'this,
        = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field5StringType<'this> {
            ""
        }
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
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            let right = <U as MsgTrait>::i32_unlabeled(&self.1);
            if right != ::std::default::Default::default() {
                right
            } else {
                <T as MsgTrait>::i32_unlabeled(&self.0)
            }
        }
        fn i32_optional<'this>(&'this self) -> Option<i32> {
            <U as MsgTrait>::i32_optional(&self.1)
                .or_else(|| <T as MsgTrait>::i32_optional(&self.0))
        }
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::Field3RepeatedType<'this>,
            <U as MsgTrait>::Field3RepeatedType<'this>,
        >;

        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as MsgTrait>::i32_repeated(&self.0),
                <U as MsgTrait>::i32_repeated(&self.1),
            )
        }
        fn f32_unlabeled<'this>(&'this self) -> f32 {
            let right = <U as MsgTrait>::f32_unlabeled(&self.1);
            if right != ::std::default::Default::default() {
                right
            } else {
                <T as MsgTrait>::f32_unlabeled(&self.0)
            }
        }
        type Field5StringType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as MsgTrait>::Field5StringType<'this>,
            <U as MsgTrait>::Field5StringType<'this>,
        >;
        fn string_unlabeled<'this>(&'this self) -> Self::Field5StringType<'this> {
            let right = <U as MsgTrait>::string_unlabeled(&self.1);
            if !right.is_empty() {
                ::puroro::Either::Right(right)
            } else {
                ::puroro::Either::Left(<T as MsgTrait>::string_unlabeled(&self.0))
            }
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
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            self.as_ref().either(
                |t| <T as MsgTrait>::i32_unlabeled(t),
                |u| <U as MsgTrait>::i32_unlabeled(u),
            )
        }
        fn i32_optional<'this>(&'this self) -> Option<i32> {
            self.as_ref().either(
                |t| <T as MsgTrait>::i32_optional(t),
                |u| <U as MsgTrait>::i32_optional(u),
            )
        }
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::Field3RepeatedType<'this>,
            <U as MsgTrait>::Field3RepeatedType<'this>,
        >;

        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::i32_repeated(t))
                    .map_right(|u| <U as MsgTrait>::i32_repeated(u)),
            )
        }
        fn f32_unlabeled<'this>(&'this self) -> f32 {
            self.as_ref().either(
                |t| <T as MsgTrait>::f32_unlabeled(t),
                |u| <U as MsgTrait>::f32_unlabeled(u),
            )
        }
        type Field5StringType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as MsgTrait>::Field5StringType<'this>,
            <U as MsgTrait>::Field5StringType<'this>,
        >;
        fn string_unlabeled<'this>(&'this self) -> Self::Field5StringType<'this> {
            self.as_ref().either(
                |t| ::puroro::Either::Left(<T as MsgTrait>::string_unlabeled(t)),
                |u| ::puroro::Either::Right(<U as MsgTrait>::string_unlabeled(u)),
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
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            self.as_ref()
                .map_or_else(::std::default::Default::default, |msg| msg.i32_unlabeled())
        }
        fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().and_then(|msg| msg.i32_optional())
        }
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field3RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.i32_repeated().into_iter())
                .into_iter()
                .flatten()
        }
        fn f32_unlabeled<'this>(&'this self) -> f32 {
            self.as_ref()
                .map_or_else(::std::default::Default::default, |msg| msg.f32_unlabeled())
        }
        type Field5StringType<'this>
        where
            Self: 'this,
        = ::puroro::Either<T::Field5StringType<'this>, &'static str>;
        fn string_unlabeled<'this>(&'this self) -> Self::Field5StringType<'this> {
            self.as_ref().map_or(::puroro::Either::Right(""), |msg| {
                ::puroro::Either::Left(msg.string_unlabeled())
            })
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
        pub i32_unlabeled: i32,
    }

    impl ::puroro::Message<super::Msg> for MsgSingleField1 {}

    impl super::_puroro_traits::MsgTrait for MsgSingleField1 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Clone::clone(&self.i32_unlabeled)
        }
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn f32_unlabeled<'this>(&'this self) -> f32 {
            Default::default()
        }
        type Field5StringType<'this>
        where
            Self: 'this,
        = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field5StringType<'this> {
            ""
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
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Unlabeled, ::puroro::tags::Int32>::ser_field(
                &self.i32_unlabeled,
                1,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<i32> for MsgSingleField1 {
        fn from(value: i32) -> Self {
            Self {
                i32_unlabeled: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField2 {
        pub i32_optional: ::std::option::Option<i32>,
    }

    impl ::puroro::Message<super::Msg> for MsgSingleField2 {}

    impl super::_puroro_traits::MsgTrait for MsgSingleField2 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Default::default()
        }
        fn i32_optional<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.i32_optional)
        }
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn f32_unlabeled<'this>(&'this self) -> f32 {
            Default::default()
        }
        type Field5StringType<'this>
        where
            Self: 'this,
        = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field5StringType<'this> {
            ""
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
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Int32>::ser_field(
                &self.i32_optional,
                2,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<i32>> for MsgSingleField2 {
        fn from(value: ::std::option::Option<i32>) -> Self {
            Self {
                i32_optional: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField3 {
        pub i32_repeated: ::std::vec::Vec<i32>,
    }

    impl ::puroro::Message<super::Msg> for MsgSingleField3 {}

    impl super::_puroro_traits::MsgTrait for MsgSingleField3 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Default::default()
        }
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Cloned<::std::slice::Iter<'this, i32>>;

        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            self.i32_repeated.iter().cloned()
        }
        fn f32_unlabeled<'this>(&'this self) -> f32 {
            Default::default()
        }
        type Field5StringType<'this>
        where
            Self: 'this,
        = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field5StringType<'this> {
            ""
        }
        type Field6MessageType<'this>
        where
            Self: 'this,
        = ();
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for MsgSingleField3 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::Int32>::ser_field(
                &self.i32_repeated,
                3,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::vec::Vec<i32>> for MsgSingleField3 {
        fn from(value: ::std::vec::Vec<i32>) -> Self {
            Self {
                i32_repeated: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField4 {
        pub f32_unlabeled: f32,
    }

    impl ::puroro::Message<super::Msg> for MsgSingleField4 {}

    impl super::_puroro_traits::MsgTrait for MsgSingleField4 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Default::default()
        }
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn f32_unlabeled<'this>(&'this self) -> f32 {
            Clone::clone(&self.f32_unlabeled)
        }
        type Field5StringType<'this>
        where
            Self: 'this,
        = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field5StringType<'this> {
            ""
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
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Unlabeled, ::puroro::tags::Float>::ser_field(
                &self.f32_unlabeled,
                4,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<f32> for MsgSingleField4 {
        fn from(value: f32) -> Self {
            Self {
                f32_unlabeled: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField5<T>
    where
        T: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub string_unlabeled: T,
    }

    impl<T> ::puroro::Message<super::Msg> for MsgSingleField5<T> where
        T: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<T> super::_puroro_traits::MsgTrait for MsgSingleField5<T>
    where
        T: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Default::default()
        }
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn f32_unlabeled<'this>(&'this self) -> f32 {
            Default::default()
        }
        type Field5StringType<'this>
        where
            Self: 'this,
        = &'this str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field5StringType<'this> {
            self.string_unlabeled.deref()
        }
        type Field6MessageType<'this>
        where
            Self: 'this,
        = ();
    }

    impl<T> ::puroro::internal::SerializableMessageToIoWrite for MsgSingleField5<T>
    where
        T: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Unlabeled, ::puroro::tags::String>::ser_field::<
                T,
                _,
                _,
            >(&self.string_unlabeled, 5, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<T> ::std::convert::From<T> for MsgSingleField5<T>
    where
        T: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: T) -> Self {
            Self {
                string_unlabeled: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField6<T>
    where
        T: self::_puroro_root::proto3_defaults::_puroro_traits::SubmsgTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub submsg_unlabeled: ::std::option::Option<T>,
    }

    impl<T> ::puroro::Message<super::Msg> for MsgSingleField6<T> where
        T: self::_puroro_root::proto3_defaults::_puroro_traits::SubmsgTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<T> super::_puroro_traits::MsgTrait for MsgSingleField6<T>
    where
        T: self::_puroro_root::proto3_defaults::_puroro_traits::SubmsgTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Default::default()
        }
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn f32_unlabeled<'this>(&'this self) -> f32 {
            Default::default()
        }
        type Field5StringType<'this>
        where
            Self: 'this,
        = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field5StringType<'this> {
            ""
        }
        type Field6MessageType<'this>
        where
            Self: 'this,
        = &'this T;
        fn submsg_unlabeled<'this>(&'this self) -> Option<Self::Field6MessageType<'this>> {
            self.submsg_unlabeled.as_ref()
        }
    }

    impl<T> ::puroro::internal::SerializableMessageToIoWrite for MsgSingleField6<T>
    where
        T: self::_puroro_root::proto3_defaults::_puroro_traits::SubmsgTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        T: ::puroro::internal::SerializableMessageToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Unlabeled, ::puroro::tags::Message<T>>::ser_field::<
                T,
                _,
                _,
            >(&self.submsg_unlabeled, 6, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<T> ::std::convert::From<::std::option::Option<T>> for MsgSingleField6<T>
    where
        T: self::_puroro_root::proto3_defaults::_puroro_traits::SubmsgTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ::std::option::Option<T>) -> Self {
            Self {
                submsg_unlabeled: value,
            }
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct MsgSimpleByValue {}
    impl ::puroro::Message<super::Msg> for MsgSimpleByValue {}

    impl MsgTrait for MsgSimpleByValue {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn i32_optional<'this>(&'this self) -> Option<i32> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field3RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn f32_unlabeled<'this>(&'this self) -> f32 {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field5StringType<'this> = ::std::string::String;
        fn string_unlabeled<'this>(&'this self) -> Self::Field5StringType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field6MessageType<'this> =
            ::std::boxed::Box<self::_puroro_root::proto3_defaults::_puroro_simple_impl::Submsg>;
        fn submsg_unlabeled<'this>(&'this self) -> Option<Self::Field6MessageType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
    }
    pub struct MsgBuilder<T>(T);

    impl<T> MsgBuilder<T>
    where
        T: MsgTrait,
    {
        pub fn append_i32_unlabeled(self, value: i32) -> MsgBuilder<(T, MsgSingleField1)> {
            MsgBuilder((
                self.0,
                MsgSingleField1 {
                    i32_unlabeled: value,
                },
            ))
        }
        pub fn append_i32_optional(
            self,
            value: ::std::option::Option<i32>,
        ) -> MsgBuilder<(T, MsgSingleField2)> {
            MsgBuilder((
                self.0,
                MsgSingleField2 {
                    i32_optional: value,
                },
            ))
        }
        pub fn append_i32_repeated(
            self,
            value: ::std::vec::Vec<i32>,
        ) -> MsgBuilder<(T, MsgSingleField3)> {
            MsgBuilder((
                self.0,
                MsgSingleField3 {
                    i32_repeated: value,
                },
            ))
        }
        pub fn append_f32_unlabeled(self, value: f32) -> MsgBuilder<(T, MsgSingleField4)> {
            MsgBuilder((
                self.0,
                MsgSingleField4 {
                    f32_unlabeled: value,
                },
            ))
        }
        pub fn append_string_unlabeled<U>(self, value: U) -> MsgBuilder<(T, MsgSingleField5<U>)>
        where
            U: ::std::ops::Deref<Target = str>
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
        pub fn append_submsg_unlabeled<U>(
            self,
            value: ::std::option::Option<U>,
        ) -> MsgBuilder<(T, MsgSingleField6<U>)>
        where
            U: self::_puroro_root::proto3_defaults::_puroro_traits::SubmsgTrait
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
    impl SubmsgTrait for () {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Default::default()
        }
    }
    impl<T, U> SubmsgTrait for (T, U)
    where
        T: SubmsgTrait,
        U: SubmsgTrait,
    {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            let right = <U as SubmsgTrait>::i32_unlabeled(&self.1);
            if right != ::std::default::Default::default() {
                right
            } else {
                <T as SubmsgTrait>::i32_unlabeled(&self.0)
            }
        }
    }
    impl<T, U> SubmsgTrait for ::puroro::Either<T, U>
    where
        T: SubmsgTrait,
        U: SubmsgTrait,
    {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            self.as_ref().either(
                |t| <T as SubmsgTrait>::i32_unlabeled(t),
                |u| <U as SubmsgTrait>::i32_unlabeled(u),
            )
        }
    }
    impl<T> SubmsgTrait for ::std::option::Option<T>
    where
        T: SubmsgTrait,
    {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            self.as_ref()
                .map_or_else(::std::default::Default::default, |msg| msg.i32_unlabeled())
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct SubmsgSingleField1 {
        pub i32_unlabeled: i32,
    }

    impl ::puroro::Message<super::Submsg> for SubmsgSingleField1 {}

    impl super::_puroro_traits::SubmsgTrait for SubmsgSingleField1 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Clone::clone(&self.i32_unlabeled)
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for SubmsgSingleField1 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Unlabeled, ::puroro::tags::Int32>::ser_field(
                &self.i32_unlabeled,
                1,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<i32> for SubmsgSingleField1 {
        fn from(value: i32) -> Self {
            Self {
                i32_unlabeled: value,
            }
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct SubmsgSimpleByValue {}
    impl ::puroro::Message<super::Submsg> for SubmsgSimpleByValue {}

    impl SubmsgTrait for SubmsgSimpleByValue {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
    }
    pub struct SubmsgBuilder<T>(T);

    impl<T> SubmsgBuilder<T>
    where
        T: SubmsgTrait,
    {
        pub fn append_i32_unlabeled(self, value: i32) -> SubmsgBuilder<(T, SubmsgSingleField1)> {
            SubmsgBuilder((
                self.0,
                SubmsgSingleField1 {
                    i32_unlabeled: value,
                },
            ))
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
        fn i32_unlabeled<'this>(&'this self) -> i32;
        fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::default::Default::default()
        }
        type Field3RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = i32>
        where
            Self: 'this;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this>;
        fn f32_unlabeled<'this>(&'this self) -> f32;
        type Field5StringType<'this>: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
        where
            Self: 'this;
        fn string_unlabeled<'this>(&'this self) -> Self::Field5StringType<'this>;
        type Field6MessageType<'this>:
            self::_puroro_root::proto3_defaults::_puroro_traits::SubmsgTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
            where Self: 'this;
        fn submsg_unlabeled<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field6MessageType<'this>> {
            ::std::default::Default::default()
        }
    }

    macro_rules! msg_delegate {
        ($ty:ty) => {
            fn i32_unlabeled<'this>(&'this self) -> i32 {
                (**self).i32_unlabeled()
            }
            fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).i32_optional()
            }
            type Field3RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field3RepeatedType<'this>;
            fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
                (**self).i32_repeated()
            }
            fn f32_unlabeled<'this>(&'this self) -> f32 {
                (**self).f32_unlabeled()
            }
            type Field5StringType<'this>
            where
                Self: 'this,
            = <$ty>::Field5StringType<'this>;
            fn string_unlabeled<'this>(&'this self) -> Self::Field5StringType<'this> {
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
        fn i32_unlabeled<'this>(&'this self) -> i32;
    }

    macro_rules! submsg_delegate {
        ($ty:ty) => {
            fn i32_unlabeled<'this>(&'this self) -> i32 {
                (**self).i32_unlabeled()
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
