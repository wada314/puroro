// A generated source code by puroro library
// package proto3_defaults

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub use _puroro_impls::MsgSimple as Msg;
pub use _puroro_impls::SubmsgSimple as Submsg;
pub use _puroro_impls::*;
pub use _puroro_impls::*;
pub mod _puroro_impls {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
    use super::_puroro_traits::*;
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct MsgSimple {
        pub i32_unlabeled: i32,
        pub i32_optional: ::std::option::Option<i32>,
        pub i32_repeated: ::std::vec::Vec<i32>,
        pub f32_unlabeled: f32,
        pub string_unlabeled: ::std::borrow::Cow<'static, str>,
        pub submsg_unlabeled: ::std::option::Option<
            ::std::boxed::Box<self::_puroro_root::proto3_defaults::_puroro_impls::SubmsgSimple>,
        >,
    }
    impl ::puroro::Message<MsgSimple> for MsgSimple {}

    impl MsgTrait for MsgSimple {
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
        type Field5StringType<'this> = &'this str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field5StringType<'this> {
            &self.string_unlabeled
        }
        type Field6MessageType<'this> =
            &'this self::_puroro_root::proto3_defaults::_puroro_impls::SubmsgSimple;
        fn submsg_unlabeled<'this>(&'this self) -> Option<Self::Field6MessageType<'this>> {
            self.submsg_unlabeled.as_ref().map(|v| v.as_ref())
        }
    }

    impl ::puroro::MessageRepresentativeImpl for MsgSimple {
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
                                    <MsgSimple as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "i32_optional",
                                number: 2,
                                lazy_containing_type: Lazy::new(|| {
                                    <MsgSimple as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "i32_repeated",
                                number: 3,
                                lazy_containing_type: Lazy::new(|| {
                                    <MsgSimple as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "f32_unlabeled",
                                number: 4,
                                lazy_containing_type: Lazy::new(|| {
                                    <MsgSimple as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "string_unlabeled",
                                number: 5,
                                lazy_containing_type: Lazy::new(|| {
                                    <MsgSimple as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "submsg_unlabeled",
                                number: 6,
                                lazy_containing_type: Lazy::new(|| {
                                    <MsgSimple as ::puroro::MessageRepresentativeImpl>::descriptor()
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

    impl ::puroro::DeserFromBytesIter for MsgSimple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro::internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro::internal::de::DeserFieldsFromBytesIter for MsgSimple {
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
                ::puroro::tags::Unlabeled, ::puroro::tags::Message<self::_puroro_root::proto3_defaults::_puroro_impls::SubmsgSimple>
            >::deser_field(&mut self.submsg_unlabeled, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerToIoWrite for MsgSimple {
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
                    self::_puroro_root::proto3_defaults::_puroro_impls::SubmsgSimple,
                >,
            >::ser_field(&self.submsg_unlabeled, 6, out)?;

            ::std::result::Result::Ok(())
        }
    }
    impl MsgTrait for () {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Default::default()
        }
        type Field3RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn f32_unlabeled<'this>(&'this self) -> f32 {
            Default::default()
        }
        type Field5StringType<'this> = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field5StringType<'this> {
            ""
        }
        type Field6MessageType<'this> = ();
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
        type Field3RepeatedType<'this> = ::puroro::internal::impls::merged::MergedRepeatedField<
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
        type Field5StringType<'this> = ::puroro::Either<
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
        type Field6MessageType<'this> = ::puroro::Either<
            ::puroro::Either<
                <T as MsgTrait>::Field6MessageType<'this>,
                <U as MsgTrait>::Field6MessageType<'this>,
            >,
            (
                <T as MsgTrait>::Field6MessageType<'this>,
                <U as MsgTrait>::Field6MessageType<'this>,
            ),
        >;
        fn submsg_unlabeled<'this>(&'this self) -> Option<Self::Field6MessageType<'this>> {
            match (
                <T as MsgTrait>::submsg_unlabeled(&self.0),
                <U as MsgTrait>::submsg_unlabeled(&self.1),
            ) {
                (None, None) => None,
                (Some(t), None) => Some(::puroro::Either::Left(::puroro::Either::Left(t))),
                (None, Some(u)) => Some(::puroro::Either::Left(::puroro::Either::Right(u))),
                (Some(t), Some(u)) => Some(::puroro::Either::Right((t, u))),
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
        type Field3RepeatedType<'this> = ::puroro::internal::impls::either::EitherRepeatedField<
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
        type Field5StringType<'this> = ::puroro::Either<
            <T as MsgTrait>::Field5StringType<'this>,
            <U as MsgTrait>::Field5StringType<'this>,
        >;
        fn string_unlabeled<'this>(&'this self) -> Self::Field5StringType<'this> {
            self.as_ref().either(
                |t| ::puroro::Either::Left(<T as MsgTrait>::string_unlabeled(t)),
                |u| ::puroro::Either::Right(<U as MsgTrait>::string_unlabeled(u)),
            )
        }
        type Field6MessageType<'this> = ::puroro::Either<
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
        type Field3RepeatedType<'this> = ::std::iter::Flatten<
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
        type Field5StringType<'this> = ::puroro::Either<T::Field5StringType<'this>, &'static str>;
        fn string_unlabeled<'this>(&'this self) -> Self::Field5StringType<'this> {
            self.as_ref().map_or(::puroro::Either::Right(""), |msg| {
                ::puroro::Either::Left(msg.string_unlabeled())
            })
        }
        type Field6MessageType<'this> = T::Field6MessageType<'this>;
        fn submsg_unlabeled<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field6MessageType<'this>> {
            self.as_ref().and_then(|msg| msg.submsg_unlabeled())
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct MsgSimpleField1 {
        pub i32_unlabeled: i32,
    }

    impl ::puroro::Message<MsgSimple> for MsgSimpleField1 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField1 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Clone::clone(&self.i32_unlabeled)
        }
        type Field3RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn f32_unlabeled<'this>(&'this self) -> f32 {
            Default::default()
        }
        type Field5StringType<'this> = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field5StringType<'this> {
            ""
        }
        type Field6MessageType<'this> = ();
    }

    impl ::puroro::SerToIoWrite for MsgSimpleField1 {
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

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct MsgSimpleField2 {
        pub i32_optional: ::std::option::Option<i32>,
    }

    impl ::puroro::Message<MsgSimple> for MsgSimpleField2 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField2 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Default::default()
        }
        fn i32_optional<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.i32_optional)
        }
        type Field3RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn f32_unlabeled<'this>(&'this self) -> f32 {
            Default::default()
        }
        type Field5StringType<'this> = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field5StringType<'this> {
            ""
        }
        type Field6MessageType<'this> = ();
    }

    impl ::puroro::SerToIoWrite for MsgSimpleField2 {
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

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct MsgSimpleField3 {
        pub i32_repeated: ::std::vec::Vec<i32>,
    }

    impl ::puroro::Message<MsgSimple> for MsgSimpleField3 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField3 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Default::default()
        }
        type Field3RepeatedType<'this> = ::std::iter::Cloned<::std::slice::Iter<'this, i32>>;

        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            self.i32_repeated.iter().cloned()
        }
        fn f32_unlabeled<'this>(&'this self) -> f32 {
            Default::default()
        }
        type Field5StringType<'this> = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field5StringType<'this> {
            ""
        }
        type Field6MessageType<'this> = ();
    }

    impl ::puroro::SerToIoWrite for MsgSimpleField3 {
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

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct MsgSimpleField4 {
        pub f32_unlabeled: f32,
    }

    impl ::puroro::Message<MsgSimple> for MsgSimpleField4 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField4 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Default::default()
        }
        type Field3RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn f32_unlabeled<'this>(&'this self) -> f32 {
            Clone::clone(&self.f32_unlabeled)
        }
        type Field5StringType<'this> = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field5StringType<'this> {
            ""
        }
        type Field6MessageType<'this> = ();
    }

    impl ::puroro::SerToIoWrite for MsgSimpleField4 {
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

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct MsgSimpleField5 {
        pub string_unlabeled: ::std::borrow::Cow<'static, str>,
    }

    impl ::puroro::Message<MsgSimple> for MsgSimpleField5 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField5 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Default::default()
        }
        type Field3RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn f32_unlabeled<'this>(&'this self) -> f32 {
            Default::default()
        }
        type Field5StringType<'this> = &'this str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field5StringType<'this> {
            &self.string_unlabeled
        }
        type Field6MessageType<'this> = ();
    }

    impl ::puroro::SerToIoWrite for MsgSimpleField5 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Unlabeled, ::puroro::tags::String>::ser_field(
                &self.string_unlabeled,
                5,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct MsgSimpleField6 {
        pub submsg_unlabeled: ::std::option::Option<
            ::std::boxed::Box<self::_puroro_root::proto3_defaults::_puroro_impls::SubmsgSimple>,
        >,
    }

    impl ::puroro::Message<MsgSimple> for MsgSimpleField6 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField6 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Default::default()
        }
        type Field3RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn f32_unlabeled<'this>(&'this self) -> f32 {
            Default::default()
        }
        type Field5StringType<'this> = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field5StringType<'this> {
            ""
        }
        type Field6MessageType<'this> =
            &'this self::_puroro_root::proto3_defaults::_puroro_impls::SubmsgSimple;
        fn submsg_unlabeled<'this>(&'this self) -> Option<Self::Field6MessageType<'this>> {
            self.submsg_unlabeled.as_ref().map(|v| v.as_ref())
        }
    }

    impl ::puroro::SerToIoWrite for MsgSimpleField6 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Message<
                    self::_puroro_root::proto3_defaults::_puroro_impls::SubmsgSimple,
                >,
            >::ser_field(&self.submsg_unlabeled, 6, out)?;
            ::std::result::Result::Ok(())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct MsgSimpleByValue {}
    impl ::puroro::Message<MsgSimple> for MsgSimpleByValue {}

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
        type Field5StringType<'this> = ::std::borrow::Cow<'this, str>;
        fn string_unlabeled<'this>(&'this self) -> Self::Field5StringType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field6MessageType<'this> =
            ::std::boxed::Box<self::_puroro_root::proto3_defaults::_puroro_impls::SubmsgSimple>;
        fn submsg_unlabeled<'this>(&'this self) -> Option<Self::Field6MessageType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct SubmsgSimple {
        pub i32_unlabeled: i32,
    }
    impl ::puroro::Message<SubmsgSimple> for SubmsgSimple {}

    impl SubmsgTrait for SubmsgSimple {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Clone::clone(&self.i32_unlabeled)
        }
    }

    impl ::puroro::MessageRepresentativeImpl for SubmsgSimple {
        fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
            use ::puroro::once_cell::sync::Lazy;
            static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 1]> =
                Lazy::new(|| {
                    [{
                        let init = ::puroro::internal::FieldDescriptorInitializer {
                            name: "i32_unlabeled",
                            number: 1,
                            lazy_containing_type: Lazy::new(|| {
                                <SubmsgSimple as ::puroro::MessageRepresentativeImpl>::descriptor()
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

    impl ::puroro::DeserFromBytesIter for SubmsgSimple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro::internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro::internal::de::DeserFieldsFromBytesIter for SubmsgSimple {
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

    impl ::puroro::SerToIoWrite for SubmsgSimple {
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
    pub struct SubmsgSimpleField1 {
        pub i32_unlabeled: i32,
    }

    impl ::puroro::Message<SubmsgSimple> for SubmsgSimpleField1 {}

    impl super::_puroro_traits::SubmsgTrait for SubmsgSimpleField1 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Clone::clone(&self.i32_unlabeled)
        }
    }

    impl ::puroro::SerToIoWrite for SubmsgSimpleField1 {
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
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct SubmsgSimpleByValue {}
    impl ::puroro::Message<SubmsgSimple> for SubmsgSimpleByValue {}

    impl SubmsgTrait for SubmsgSimpleByValue {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            unimplemented!("Please don't use / instantiate this struct!!")
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
            + ::std::iter::IntoIterator<Item = i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this>;
        fn f32_unlabeled<'this>(&'this self) -> f32;
        type Field5StringType<'this>: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug;
        fn string_unlabeled<'this>(&'this self) -> Self::Field5StringType<'this>;
        type Field6MessageType<'this>:
            self::_puroro_root::proto3_defaults::_puroro_traits::SubmsgTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug;
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
            type Field3RepeatedType<'this> = <$ty>::Field3RepeatedType<'this>;
            fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
                (**self).i32_repeated()
            }
            fn f32_unlabeled<'this>(&'this self) -> f32 {
                (**self).f32_unlabeled()
            }
            type Field5StringType<'this> = <$ty>::Field5StringType<'this>;
            fn string_unlabeled<'this>(&'this self) -> Self::Field5StringType<'this> {
                (**self).string_unlabeled()
            }
            type Field6MessageType<'this> = <$ty>::Field6MessageType<'this>;
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
