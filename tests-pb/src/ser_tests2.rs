// A generated source code by puroro library
// package ser_tests2

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub use _puroro_impls::MsgSimple as Msg;
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
        pub i32_optional: ::std::option::Option<i32>,
        pub i32_repeated: ::std::vec::Vec<i32>,
        pub float_optional: ::std::option::Option<f32>,
        pub float_repeated: ::std::vec::Vec<f32>,
        pub string_optional: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        pub string_repeated: ::std::vec::Vec<::std::borrow::Cow<'static, str>>,
        pub submsg_optional: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple,
            >,
        >,
        pub submsg_repeated: ::std::vec::Vec<
            self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple,
        >,
        pub enum_optional: ::std::option::Option<self::_puroro_root::ser_tests2::Enum>,
        pub enum_repeated: ::std::vec::Vec<self::_puroro_root::ser_tests2::Enum>,
        pub very_large_field_number: ::std::option::Option<i32>,
    }
    impl ::puroro::Message<MsgSimple> for MsgSimple {}

    impl MsgTrait for MsgSimple {
        fn i32_optional<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.i32_optional)
        }
        type Field2RepeatedType<'this> = ::std::iter::Cloned<::std::slice::Iter<'this, i32>>;

        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            self.i32_repeated.iter().cloned()
        }
        fn float_optional<'this>(&'this self) -> Option<f32> {
            Clone::clone(&self.float_optional)
        }
        type Field4RepeatedType<'this> = ::std::iter::Cloned<::std::slice::Iter<'this, f32>>;

        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            self.float_repeated.iter().cloned()
        }
        type Field5StringType<'this> = &'this str;
        fn string_optional<'this>(&'this self) -> Option<Self::Field5StringType<'this>> {
            self.string_optional.as_ref().map(|v| v.as_ref())
        }
        type Field6StringType<'this> = &'this str;
        type Field6RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            str,
            ::std::slice::Iter<'this, ::std::borrow::Cow<'static, str>>,
        >;

        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.string_repeated.iter())
        }
        type Field7MessageType<'this> =
            &'this self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple;
        fn submsg_optional<'this>(&'this self) -> Option<Self::Field7MessageType<'this>> {
            self.submsg_optional.as_ref().map(|v| v.as_ref())
        }
        type Field8MessageType<'this> =
            &'this self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple;
        type Field8RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple,
            >,
        >;

        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.submsg_repeated.iter())
        }
        fn enum_optional<'this>(&'this self) -> Option<self::_puroro_root::ser_tests2::Enum> {
            Clone::clone(&self.enum_optional)
        }
        type Field10RepeatedType<'this> =
            ::std::iter::Cloned<::std::slice::Iter<'this, self::_puroro_root::ser_tests2::Enum>>;

        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            self.enum_repeated.iter().cloned()
        }
        fn very_large_field_number<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.very_large_field_number)
        }
    }

    impl ::puroro::MessageRepresentativeImpl for MsgSimple {
        fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
            use ::puroro::once_cell::sync::Lazy;
            static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 11]> =
                Lazy::new(|| {
                    [
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "i32_optional",
                                number: 1,
                                lazy_containing_type: Lazy::new(|| {
                                    <MsgSimple as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "i32_repeated",
                                number: 2,
                                lazy_containing_type: Lazy::new(|| {
                                    <MsgSimple as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "float_optional",
                                number: 3,
                                lazy_containing_type: Lazy::new(|| {
                                    <MsgSimple as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "float_repeated",
                                number: 4,
                                lazy_containing_type: Lazy::new(|| {
                                    <MsgSimple as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "string_optional",
                                number: 5,
                                lazy_containing_type: Lazy::new(|| {
                                    <MsgSimple as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "string_repeated",
                                number: 6,
                                lazy_containing_type: Lazy::new(|| {
                                    <MsgSimple as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "submsg_optional",
                                number: 7,
                                lazy_containing_type: Lazy::new(|| {
                                    <MsgSimple as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "submsg_repeated",
                                number: 8,
                                lazy_containing_type: Lazy::new(|| {
                                    <MsgSimple as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "enum_optional",
                                number: 9,
                                lazy_containing_type: Lazy::new(|| {
                                    <MsgSimple as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "enum_repeated",
                                number: 10,
                                lazy_containing_type: Lazy::new(|| {
                                    <MsgSimple as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "very_large_field_number",
                                number: 536870911,
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
                ::puroro::tags::Optional, ::puroro::tags::Int32
            >::deser_field(&mut self.i32_optional, data),
            2 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Int32
            >::deser_field(&mut self.i32_repeated, data),
            3 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Float
            >::deser_field(&mut self.float_optional, data),
            4 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Float
            >::deser_field(&mut self.float_repeated, data),
            5 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.string_optional, data),
            6 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::String
            >::deser_field(&mut self.string_repeated, data),
            7 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Message<self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple>
            >::deser_field(&mut self.submsg_optional, data),
            8 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple>
            >::deser_field(&mut self.submsg_repeated, data),
            9 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Enum2<self::_puroro_root::ser_tests2::Enum>
            >::deser_field(&mut self.enum_optional, data),
            10 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Enum2<self::_puroro_root::ser_tests2::Enum>
            >::deser_field(&mut self.enum_repeated, data),
            536870911 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Int32
            >::deser_field(&mut self.very_large_field_number, data),

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
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Int32>::ser_field(
                &self.i32_optional,
                1,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::Int32>::ser_field(
                &self.i32_repeated,
                2,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Float>::ser_field(
                &self.float_optional,
                3,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::Float>::ser_field(
                &self.float_repeated,
                4,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.string_optional,
                5,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::String>::ser_field(
                &self.string_repeated,
                6,
                out,
            )?;
            SerFieldToIoWrite::<
            ::puroro::tags::Optional, ::puroro::tags::Message<self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple>
        >::ser_field(&self.submsg_optional, 7, out)?;
            SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple>
        >::ser_field(&self.submsg_repeated, 8, out)?;
            SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Enum2<self::_puroro_root::ser_tests2::Enum>,
            >::ser_field(&self.enum_optional, 9, out)?;
            SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Enum2<self::_puroro_root::ser_tests2::Enum>,
            >::ser_field(&self.enum_repeated, 10, out)?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Int32>::ser_field(
                &self.very_large_field_number,
                536870911,
                out,
            )?;

            ::std::result::Result::Ok(())
        }
    }
    impl MsgTrait for () {
        type Field2RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5StringType<'this> = &'static str;
        type Field6StringType<'this> = &'static str;
        type Field6RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field8MessageType<'this> = ();
        type Field8RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::ser_tests2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }
    impl<T, U> MsgTrait for (T, U)
    where
        T: MsgTrait,
        U: MsgTrait,
    {
        fn i32_optional<'this>(&'this self) -> Option<i32> {
            <U as MsgTrait>::i32_optional(&self.1)
                .or_else(|| <T as MsgTrait>::i32_optional(&self.0))
        }
        type Field2RepeatedType<'this> = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::Field2RepeatedType<'this>,
            <U as MsgTrait>::Field2RepeatedType<'this>,
        >;

        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as MsgTrait>::i32_repeated(&self.0),
                <U as MsgTrait>::i32_repeated(&self.1),
            )
        }
        fn float_optional<'this>(&'this self) -> Option<f32> {
            <U as MsgTrait>::float_optional(&self.1)
                .or_else(|| <T as MsgTrait>::float_optional(&self.0))
        }
        type Field4RepeatedType<'this> = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::Field4RepeatedType<'this>,
            <U as MsgTrait>::Field4RepeatedType<'this>,
        >;

        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as MsgTrait>::float_repeated(&self.0),
                <U as MsgTrait>::float_repeated(&self.1),
            )
        }
        type Field5StringType<'this> = ::puroro::Either<
            <T as MsgTrait>::Field5StringType<'this>,
            <U as MsgTrait>::Field5StringType<'this>,
        >;
        fn string_optional<'this>(&'this self) -> Option<Self::Field5StringType<'this>> {
            if let Some(right) = <U as MsgTrait>::string_optional(&self.1) {
                Some(::puroro::Either::Right(right))
            } else if let Some(left) = <T as MsgTrait>::string_optional(&self.0) {
                Some(::puroro::Either::Left(left))
            } else {
                None
            }
        }
        type Field6StringType<'this> = ::puroro::Either<
            <T as MsgTrait>::Field6StringType<'this>,
            <U as MsgTrait>::Field6StringType<'this>,
        >;
        type Field6RepeatedType<'this> = ::puroro::internal::impls::merged::MergedRepeatedLDField<
            <T as MsgTrait>::Field6RepeatedType<'this>,
            <U as MsgTrait>::Field6RepeatedType<'this>,
        >;

        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedLDField::new(
                <T as MsgTrait>::string_repeated(&self.0),
                <U as MsgTrait>::string_repeated(&self.1),
            )
        }
        type Field7MessageType<'this> = (
            ::std::option::Option<<T as MsgTrait>::Field7MessageType<'this>>,
            ::std::option::Option<<U as MsgTrait>::Field7MessageType<'this>>,
        );
        fn submsg_optional<'this>(&'this self) -> Option<Self::Field7MessageType<'this>> {
            match (
                <T as MsgTrait>::submsg_optional(&self.0),
                <U as MsgTrait>::submsg_optional(&self.1),
            ) {
                (None, None) => None,
                (Some(t), None) => Some((Some(t), None)),
                (None, Some(u)) => Some((None, Some(u))),
                (Some(t), Some(u)) => Some((Some(t), Some(u))),
            }
        }
        type Field8MessageType<'this> = ::puroro::Either<
            <T as MsgTrait>::Field8MessageType<'this>,
            <U as MsgTrait>::Field8MessageType<'this>,
        >;
        type Field8RepeatedType<'this> =
            ::puroro::internal::impls::merged::MergedRepeatedMessageField<
                <T as MsgTrait>::Field8RepeatedType<'this>,
                <U as MsgTrait>::Field8RepeatedType<'this>,
            >;

        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
                <T as MsgTrait>::submsg_repeated(&self.0),
                <U as MsgTrait>::submsg_repeated(&self.1),
            )
        }
        fn enum_optional<'this>(&'this self) -> Option<self::_puroro_root::ser_tests2::Enum> {
            <U as MsgTrait>::enum_optional(&self.1)
                .or_else(|| <T as MsgTrait>::enum_optional(&self.0))
        }
        type Field10RepeatedType<'this> = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::Field10RepeatedType<'this>,
            <U as MsgTrait>::Field10RepeatedType<'this>,
        >;

        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as MsgTrait>::enum_repeated(&self.0),
                <U as MsgTrait>::enum_repeated(&self.1),
            )
        }
        fn very_large_field_number<'this>(&'this self) -> Option<i32> {
            <U as MsgTrait>::very_large_field_number(&self.1)
                .or_else(|| <T as MsgTrait>::very_large_field_number(&self.0))
        }
    }
    impl<T, U> MsgTrait for ::puroro::Either<T, U>
    where
        T: MsgTrait,
        U: MsgTrait,
    {
        fn i32_optional<'this>(&'this self) -> Option<i32> {
            self.as_ref().either(
                |t| <T as MsgTrait>::i32_optional(t),
                |u| <U as MsgTrait>::i32_optional(u),
            )
        }
        type Field2RepeatedType<'this> = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::Field2RepeatedType<'this>,
            <U as MsgTrait>::Field2RepeatedType<'this>,
        >;

        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::i32_repeated(t))
                    .map_right(|u| <U as MsgTrait>::i32_repeated(u)),
            )
        }
        fn float_optional<'this>(&'this self) -> Option<f32> {
            self.as_ref().either(
                |t| <T as MsgTrait>::float_optional(t),
                |u| <U as MsgTrait>::float_optional(u),
            )
        }
        type Field4RepeatedType<'this> = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::Field4RepeatedType<'this>,
            <U as MsgTrait>::Field4RepeatedType<'this>,
        >;

        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::float_repeated(t))
                    .map_right(|u| <U as MsgTrait>::float_repeated(u)),
            )
        }
        type Field5StringType<'this> = ::puroro::Either<
            <T as MsgTrait>::Field5StringType<'this>,
            <U as MsgTrait>::Field5StringType<'this>,
        >;
        fn string_optional<'this>(&'this self) -> Option<Self::Field5StringType<'this>> {
            self.as_ref().either(
                |t| <T as MsgTrait>::string_optional(t).map(|t| ::puroro::Either::Left(t)),
                |u| <U as MsgTrait>::string_optional(u).map(|u| ::puroro::Either::Right(u)),
            )
        }
        type Field6StringType<'this> = ::puroro::Either<
            <T as MsgTrait>::Field6StringType<'this>,
            <U as MsgTrait>::Field6StringType<'this>,
        >;
        type Field6RepeatedType<'this> = ::puroro::internal::impls::either::EitherRepeatedLDField<
            <T as MsgTrait>::Field6RepeatedType<'this>,
            <U as MsgTrait>::Field6RepeatedType<'this>,
        >;

        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedLDField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::string_repeated(t))
                    .map_right(|u| <U as MsgTrait>::string_repeated(u)),
            )
        }
        type Field7MessageType<'this> = ::puroro::Either<
            <T as MsgTrait>::Field7MessageType<'this>,
            <U as MsgTrait>::Field7MessageType<'this>,
        >;
        fn submsg_optional<'this>(&'this self) -> Option<Self::Field7MessageType<'this>> {
            self.as_ref().either(
                |t| <T as MsgTrait>::submsg_optional(t).map(|t| ::puroro::Either::Left(t)),
                |u| <U as MsgTrait>::submsg_optional(u).map(|u| ::puroro::Either::Right(u)),
            )
        }
        type Field8MessageType<'this> = ::puroro::Either<
            <T as MsgTrait>::Field8MessageType<'this>,
            <U as MsgTrait>::Field8MessageType<'this>,
        >;
        type Field8RepeatedType<'this> =
            ::puroro::internal::impls::either::EitherRepeatedMessageField<
                <T as MsgTrait>::Field8RepeatedType<'this>,
                <U as MsgTrait>::Field8RepeatedType<'this>,
            >;

        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::submsg_repeated(t))
                    .map_right(|u| <U as MsgTrait>::submsg_repeated(u)),
            )
        }
        fn enum_optional<'this>(&'this self) -> Option<self::_puroro_root::ser_tests2::Enum> {
            self.as_ref().either(
                |t| <T as MsgTrait>::enum_optional(t),
                |u| <U as MsgTrait>::enum_optional(u),
            )
        }
        type Field10RepeatedType<'this> = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::Field10RepeatedType<'this>,
            <U as MsgTrait>::Field10RepeatedType<'this>,
        >;

        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::enum_repeated(t))
                    .map_right(|u| <U as MsgTrait>::enum_repeated(u)),
            )
        }
        fn very_large_field_number<'this>(&'this self) -> Option<i32> {
            self.as_ref().either(
                |t| <T as MsgTrait>::very_large_field_number(t),
                |u| <U as MsgTrait>::very_large_field_number(u),
            )
        }
    }
    impl<T> MsgTrait for ::std::option::Option<T>
    where
        T: MsgTrait,
    {
        fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().and_then(|msg| msg.i32_optional())
        }
        type Field2RepeatedType<'this> = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field2RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.i32_repeated().into_iter())
                .into_iter()
                .flatten()
        }
        fn float_optional<'this>(&'this self) -> ::std::option::Option<f32> {
            self.as_ref().and_then(|msg| msg.float_optional())
        }
        type Field4RepeatedType<'this> = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field4RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.float_repeated().into_iter())
                .into_iter()
                .flatten()
        }
        type Field5StringType<'this> = T::Field5StringType<'this>;
        fn string_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field5StringType<'this>> {
            self.as_ref().and_then(|msg| msg.string_optional())
        }
        type Field6StringType<'this> = T::Field6StringType<'this>;
        type Field6RepeatedType<'this> = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field6RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.string_repeated().into_iter())
                .into_iter()
                .flatten()
        }
        type Field7MessageType<'this> = T::Field7MessageType<'this>;
        fn submsg_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field7MessageType<'this>> {
            self.as_ref().and_then(|msg| msg.submsg_optional())
        }
        type Field8MessageType<'this> = T::Field8MessageType<'this>;
        type Field8RepeatedType<'this> = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field8RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.submsg_repeated().into_iter())
                .into_iter()
                .flatten()
        }
        fn enum_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::ser_tests2::Enum> {
            self.as_ref().and_then(|msg| msg.enum_optional())
        }
        type Field10RepeatedType<'this> = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field10RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.enum_repeated().into_iter())
                .into_iter()
                .flatten()
        }
        fn very_large_field_number<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().and_then(|msg| msg.very_large_field_number())
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct MsgSimpleField1 {
        pub i32_optional: ::std::option::Option<i32>,
    }

    impl ::puroro::Message<MsgSimple> for MsgSimpleField1 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField1 {
        fn i32_optional<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.i32_optional)
        }
        type Field2RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5StringType<'this> = &'static str;
        type Field6StringType<'this> = &'static str;
        type Field6RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field8MessageType<'this> = ();
        type Field8RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::ser_tests2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerToIoWrite for MsgSimpleField1 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Int32>::ser_field(
                &self.i32_optional,
                1,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<i32>> for MsgSimpleField1 {
        fn from(value: ::std::option::Option<i32>) -> Self {
            Self {
                i32_optional: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct MsgSimpleField2 {
        pub i32_repeated: ::std::vec::Vec<i32>,
    }

    impl ::puroro::Message<MsgSimple> for MsgSimpleField2 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField2 {
        type Field2RepeatedType<'this> = ::std::iter::Cloned<::std::slice::Iter<'this, i32>>;

        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            self.i32_repeated.iter().cloned()
        }
        type Field4RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5StringType<'this> = &'static str;
        type Field6StringType<'this> = &'static str;
        type Field6RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field8MessageType<'this> = ();
        type Field8RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::ser_tests2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerToIoWrite for MsgSimpleField2 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::Int32>::ser_field(
                &self.i32_repeated,
                2,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::vec::Vec<i32>> for MsgSimpleField2 {
        fn from(value: ::std::vec::Vec<i32>) -> Self {
            Self {
                i32_repeated: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct MsgSimpleField3 {
        pub float_optional: ::std::option::Option<f32>,
    }

    impl ::puroro::Message<MsgSimple> for MsgSimpleField3 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField3 {
        type Field2RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_optional<'this>(&'this self) -> Option<f32> {
            Clone::clone(&self.float_optional)
        }
        type Field4RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5StringType<'this> = &'static str;
        type Field6StringType<'this> = &'static str;
        type Field6RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field8MessageType<'this> = ();
        type Field8RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::ser_tests2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerToIoWrite for MsgSimpleField3 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Float>::ser_field(
                &self.float_optional,
                3,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<f32>> for MsgSimpleField3 {
        fn from(value: ::std::option::Option<f32>) -> Self {
            Self {
                float_optional: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct MsgSimpleField4 {
        pub float_repeated: ::std::vec::Vec<f32>,
    }

    impl ::puroro::Message<MsgSimple> for MsgSimpleField4 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField4 {
        type Field2RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4RepeatedType<'this> = ::std::iter::Cloned<::std::slice::Iter<'this, f32>>;

        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            self.float_repeated.iter().cloned()
        }
        type Field5StringType<'this> = &'static str;
        type Field6StringType<'this> = &'static str;
        type Field6RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field8MessageType<'this> = ();
        type Field8RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::ser_tests2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerToIoWrite for MsgSimpleField4 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::Float>::ser_field(
                &self.float_repeated,
                4,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::vec::Vec<f32>> for MsgSimpleField4 {
        fn from(value: ::std::vec::Vec<f32>) -> Self {
            Self {
                float_repeated: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct MsgSimpleField5 {
        pub string_optional: ::std::option::Option<::std::borrow::Cow<'static, str>>,
    }

    impl ::puroro::Message<MsgSimple> for MsgSimpleField5 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField5 {
        type Field2RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5StringType<'this> = &'this str;
        fn string_optional<'this>(&'this self) -> Option<Self::Field5StringType<'this>> {
            self.string_optional.as_ref().map(|v| v.as_ref())
        }
        type Field6StringType<'this> = &'static str;
        type Field6RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field8MessageType<'this> = ();
        type Field8RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::ser_tests2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerToIoWrite for MsgSimpleField5 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.string_optional,
                5,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<::std::borrow::Cow<'static, str>>>
        for MsgSimpleField5
    {
        fn from(value: ::std::option::Option<::std::borrow::Cow<'static, str>>) -> Self {
            Self {
                string_optional: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct MsgSimpleField6 {
        pub string_repeated: ::std::vec::Vec<::std::borrow::Cow<'static, str>>,
    }

    impl ::puroro::Message<MsgSimple> for MsgSimpleField6 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField6 {
        type Field2RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5StringType<'this> = &'static str;
        type Field6StringType<'this> = &'this str;
        type Field6RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            str,
            ::std::slice::Iter<'this, ::std::borrow::Cow<'static, str>>,
        >;

        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.string_repeated.iter())
        }
        type Field7MessageType<'this> = ();
        type Field8MessageType<'this> = ();
        type Field8RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::ser_tests2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerToIoWrite for MsgSimpleField6 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::String>::ser_field(
                &self.string_repeated,
                6,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::vec::Vec<::std::borrow::Cow<'static, str>>> for MsgSimpleField6 {
        fn from(value: ::std::vec::Vec<::std::borrow::Cow<'static, str>>) -> Self {
            Self {
                string_repeated: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct MsgSimpleField7 {
        pub submsg_optional: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple,
            >,
        >,
    }

    impl ::puroro::Message<MsgSimple> for MsgSimpleField7 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField7 {
        type Field2RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5StringType<'this> = &'static str;
        type Field6StringType<'this> = &'static str;
        type Field6RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> =
            &'this self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple;
        fn submsg_optional<'this>(&'this self) -> Option<Self::Field7MessageType<'this>> {
            self.submsg_optional.as_ref().map(|v| v.as_ref())
        }
        type Field8MessageType<'this> = ();
        type Field8RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::ser_tests2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerToIoWrite for MsgSimpleField7 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
            ::puroro::tags::Optional, ::puroro::tags::Message<self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple>
        >::ser_field(&self.submsg_optional, 7, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<::std::boxed::Box<self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple>>> for MsgSimpleField7 {
    fn from(value: ::std::option::Option<::std::boxed::Box<self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple>>) -> Self {
        Self {
            submsg_optional: value,
        }
    }
}

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct MsgSimpleField8 {
        pub submsg_repeated: ::std::vec::Vec<
            self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple,
        >,
    }

    impl ::puroro::Message<MsgSimple> for MsgSimpleField8 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField8 {
        type Field2RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5StringType<'this> = &'static str;
        type Field6StringType<'this> = &'static str;
        type Field6RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field8MessageType<'this> =
            &'this self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple;
        type Field8RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple,
            >,
        >;

        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.submsg_repeated.iter())
        }
        type Field10RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::ser_tests2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerToIoWrite for MsgSimpleField8 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple>
        >::ser_field(&self.submsg_repeated, 8, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl
        ::std::convert::From<
            ::std::vec::Vec<
                self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple,
            >,
        > for MsgSimpleField8
    {
        fn from(
            value: ::std::vec::Vec<
                self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple,
            >,
        ) -> Self {
            Self {
                submsg_repeated: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct MsgSimpleField9 {
        pub enum_optional: ::std::option::Option<self::_puroro_root::ser_tests2::Enum>,
    }

    impl ::puroro::Message<MsgSimple> for MsgSimpleField9 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField9 {
        type Field2RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5StringType<'this> = &'static str;
        type Field6StringType<'this> = &'static str;
        type Field6RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field8MessageType<'this> = ();
        type Field8RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_optional<'this>(&'this self) -> Option<self::_puroro_root::ser_tests2::Enum> {
            Clone::clone(&self.enum_optional)
        }
        type Field10RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::ser_tests2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerToIoWrite for MsgSimpleField9 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Enum2<self::_puroro_root::ser_tests2::Enum>,
            >::ser_field(&self.enum_optional, 9, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<self::_puroro_root::ser_tests2::Enum>>
        for MsgSimpleField9
    {
        fn from(value: ::std::option::Option<self::_puroro_root::ser_tests2::Enum>) -> Self {
            Self {
                enum_optional: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct MsgSimpleField10 {
        pub enum_repeated: ::std::vec::Vec<self::_puroro_root::ser_tests2::Enum>,
    }

    impl ::puroro::Message<MsgSimple> for MsgSimpleField10 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField10 {
        type Field2RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5StringType<'this> = &'static str;
        type Field6StringType<'this> = &'static str;
        type Field6RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field8MessageType<'this> = ();
        type Field8RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this> =
            ::std::iter::Cloned<::std::slice::Iter<'this, self::_puroro_root::ser_tests2::Enum>>;

        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            self.enum_repeated.iter().cloned()
        }
    }

    impl ::puroro::SerToIoWrite for MsgSimpleField10 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Enum2<self::_puroro_root::ser_tests2::Enum>,
            >::ser_field(&self.enum_repeated, 10, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::vec::Vec<self::_puroro_root::ser_tests2::Enum>>
        for MsgSimpleField10
    {
        fn from(value: ::std::vec::Vec<self::_puroro_root::ser_tests2::Enum>) -> Self {
            Self {
                enum_repeated: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct MsgSimpleField536870911 {
        pub very_large_field_number: ::std::option::Option<i32>,
    }

    impl ::puroro::Message<MsgSimple> for MsgSimpleField536870911 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField536870911 {
        type Field2RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5StringType<'this> = &'static str;
        type Field6StringType<'this> = &'static str;
        type Field6RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field8MessageType<'this> = ();
        type Field8RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::ser_tests2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn very_large_field_number<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.very_large_field_number)
        }
    }

    impl ::puroro::SerToIoWrite for MsgSimpleField536870911 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Int32>::ser_field(
                &self.very_large_field_number,
                536870911,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<i32>> for MsgSimpleField536870911 {
        fn from(value: ::std::option::Option<i32>) -> Self {
            Self {
                very_large_field_number: value,
            }
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct MsgSimpleByValue {}
    impl ::puroro::Message<MsgSimple> for MsgSimpleByValue {}

    impl MsgTrait for MsgSimpleByValue {
        fn i32_optional<'this>(&'this self) -> Option<i32> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field2RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn float_optional<'this>(&'this self) -> Option<f32> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field4RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field5StringType<'this> = ::std::borrow::Cow<'this, str>;
        fn string_optional<'this>(&'this self) -> Option<Self::Field5StringType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field6StringType<'this> = ::std::borrow::Cow<'this, str>;
        type Field6RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field7MessageType<'this> = ::std::boxed::Box<
            self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple,
        >;
        fn submsg_optional<'this>(&'this self) -> Option<Self::Field7MessageType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field8MessageType<'this> = ::std::boxed::Box<
            self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple,
        >;
        type Field8RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn enum_optional<'this>(&'this self) -> Option<self::_puroro_root::ser_tests2::Enum> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field10RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::ser_tests2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn very_large_field_number<'this>(&'this self) -> Option<i32> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
    }
    pub struct MsgBuilder<T>(T);

    impl<T> MsgBuilder<T>
    where
        T: MsgTrait,
    {
        pub fn append_i32_optional(
            self,
            value: ::std::option::Option<i32>,
        ) -> MsgBuilder<(T, MsgSimpleField1)> {
            MsgBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_i32_repeated(
            self,
            value: ::std::vec::Vec<i32>,
        ) -> MsgBuilder<(T, MsgSimpleField2)> {
            MsgBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_float_optional(
            self,
            value: ::std::option::Option<f32>,
        ) -> MsgBuilder<(T, MsgSimpleField3)> {
            MsgBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_float_repeated(
            self,
            value: ::std::vec::Vec<f32>,
        ) -> MsgBuilder<(T, MsgSimpleField4)> {
            MsgBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_string_optional(
            self,
            value: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        ) -> MsgBuilder<(T, MsgSimpleField5)> {
            MsgBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_string_repeated(
            self,
            value: ::std::vec::Vec<::std::borrow::Cow<'static, str>>,
        ) -> MsgBuilder<(T, MsgSimpleField6)> {
            MsgBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_submsg_optional(
            self,
            value: ::std::option::Option<::std::boxed::Box<self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple>>,
        ) -> MsgBuilder<(T, MsgSimpleField7)> {
            MsgBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_submsg_repeated(
            self,
            value: ::std::vec::Vec<
                self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple,
            >,
        ) -> MsgBuilder<(T, MsgSimpleField8)> {
            MsgBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_enum_optional(
            self,
            value: ::std::option::Option<self::_puroro_root::ser_tests2::Enum>,
        ) -> MsgBuilder<(T, MsgSimpleField9)> {
            MsgBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_enum_repeated(
            self,
            value: ::std::vec::Vec<self::_puroro_root::ser_tests2::Enum>,
        ) -> MsgBuilder<(T, MsgSimpleField10)> {
            MsgBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_very_large_field_number(
            self,
            value: ::std::option::Option<i32>,
        ) -> MsgBuilder<(T, MsgSimpleField536870911)> {
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
        fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::default::Default::default()
        }
        type Field2RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this>;
        fn float_optional<'this>(&'this self) -> ::std::option::Option<f32> {
            ::std::default::Default::default()
        }
        type Field4RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this>;
        type Field5StringType<'this>: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug;
        fn string_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field5StringType<'this>> {
            ::std::default::Default::default()
        }
        type Field6StringType<'this>: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug;
        type Field6RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field6StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this>;
        type Field7MessageType<'this>:
            self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_traits::SubmsgTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug;
        fn submsg_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field7MessageType<'this>> {
            ::std::default::Default::default()
        }
        type Field8MessageType<'this>:
            self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_traits::SubmsgTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug;
        type Field8RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field8MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this>;
        fn enum_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::ser_tests2::Enum> {
            ::std::default::Default::default()
        }
        type Field10RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = self::_puroro_root::ser_tests2::Enum>;
        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this>;
        fn very_large_field_number<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::default::Default::default()
        }
    }

    macro_rules! msg_delegate {
        ($ty:ty) => {
            fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).i32_optional()
            }
            type Field2RepeatedType<'this> = <$ty>::Field2RepeatedType<'this>;
            fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
                (**self).i32_repeated()
            }
            fn float_optional<'this>(&'this self) -> ::std::option::Option<f32> {
                (**self).float_optional()
            }
            type Field4RepeatedType<'this> = <$ty>::Field4RepeatedType<'this>;
            fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
                (**self).float_repeated()
            }
            type Field5StringType<'this> = <$ty>::Field5StringType<'this>;
            fn string_optional<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field5StringType<'this>> {
                (**self).string_optional()
            }
            type Field6StringType<'this> = <$ty>::Field6StringType<'this>;
            type Field6RepeatedType<'this> = <$ty>::Field6RepeatedType<'this>;
            fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
                (**self).string_repeated()
            }
            type Field7MessageType<'this> = <$ty>::Field7MessageType<'this>;
            fn submsg_optional<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field7MessageType<'this>> {
                (**self).submsg_optional()
            }
            type Field8MessageType<'this> = <$ty>::Field8MessageType<'this>;
            type Field8RepeatedType<'this> = <$ty>::Field8RepeatedType<'this>;
            fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
                (**self).submsg_repeated()
            }
            fn enum_optional<'this>(
                &'this self,
            ) -> ::std::option::Option<self::_puroro_root::ser_tests2::Enum> {
                (**self).enum_optional()
            }
            type Field10RepeatedType<'this> = <$ty>::Field10RepeatedType<'this>;
            fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
                (**self).enum_repeated()
            }
            fn very_large_field_number<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).very_large_field_number()
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
#[derive(::std::fmt::Debug, ::std::clone::Clone, ::std::cmp::PartialEq)]
pub enum Enum {
    Zeroth,
    First,
    Tenth,
}

impl ::puroro::Enum2 for Enum {}
impl ::std::convert::TryFrom<i32> for Enum {
    type Error = i32;
    fn try_from(value: i32) -> ::std::result::Result<Self, i32> {
        ::std::result::Result::Ok(match value {
            0 => Enum::Zeroth,
            1 => Enum::First,
            10 => Enum::Tenth,
            _ => Err(value)?,
        })
    }
}

impl ::std::convert::From<Enum> for i32 {
    fn from(value: Enum) -> i32 {
        match value {
            Enum::Zeroth => 0,
            Enum::First => 1,
            Enum::Tenth => 10,
        }
    }
}

impl ::std::default::Default for Enum {
    fn default() -> Self {
        Enum::Zeroth
    }
}
pub use _puroro_nested::*;
pub mod _puroro_nested {
    pub mod msg {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }

        pub use _puroro_impls::SubmsgSimple as Submsg;
        pub use _puroro_impls::*;
        pub mod _puroro_impls {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }
            use super::_puroro_traits::*;
            #[derive(
                ::std::clone::Clone,
                ::std::default::Default,
                ::std::cmp::PartialEq,
                ::std::fmt::Debug,
            )]
            pub struct SubmsgSimple {
                pub i32_optional: ::std::option::Option<i32>,
            }
            impl ::puroro::Message<SubmsgSimple> for SubmsgSimple {}

            impl SubmsgTrait for SubmsgSimple {
                fn i32_optional<'this>(&'this self) -> Option<i32> {
                    Clone::clone(&self.i32_optional)
                }
            }

            impl ::puroro::MessageRepresentativeImpl for SubmsgSimple {
                fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
                    use ::puroro::once_cell::sync::Lazy;
                    static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 1]> =
                        Lazy::new(|| {
                            [{
                                let init = ::puroro::internal::FieldDescriptorInitializer {
                                    name: "i32_optional",
                                    number: 1,
                                    lazy_containing_type: Lazy::new(|| {
                                        <SubmsgSimple as ::puroro::MessageRepresentativeImpl>::descriptor()
                                    }),
                                };
                                ::puroro::internal::init_field_descriptor(init)
                            }]
                        });
                    static LAZY_DESCRIPTOR: Lazy<::puroro::desc::MessageDescriptor> =
                        Lazy::new(|| {
                            let init = ::puroro::internal::MessageDescriptorInitializer {
                                name: "Submsg",
                                lazy_fields: Lazy::new(|| {
                                    Lazy::force(&LAZY_FIELD_DESCRIPTOR_ARRAY).as_ref()
                                }),
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
                    data: ::puroro::types::FieldData<
                        &mut ::puroro::internal::de::from_iter::ScopedIter<I>,
                    >,
                ) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    use ::puroro::internal::impls::simple::de::DeserFieldFromBytesIter;
                    match field_number {
                        1 => DeserFieldFromBytesIter::<
                            ::puroro::tags::Optional,
                            ::puroro::tags::Int32,
                        >::deser_field(&mut self.i32_optional, data),

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
                    SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::Int32
                >::ser_field(&self.i32_optional, 1, out)?;

                    ::std::result::Result::Ok(())
                }
            }
            impl SubmsgTrait for () {}
            impl<T, U> SubmsgTrait for (T, U)
            where
                T: SubmsgTrait,
                U: SubmsgTrait,
            {
                fn i32_optional<'this>(&'this self) -> Option<i32> {
                    <U as SubmsgTrait>::i32_optional(&self.1)
                        .or_else(|| <T as SubmsgTrait>::i32_optional(&self.0))
                }
            }
            impl<T, U> SubmsgTrait for ::puroro::Either<T, U>
            where
                T: SubmsgTrait,
                U: SubmsgTrait,
            {
                fn i32_optional<'this>(&'this self) -> Option<i32> {
                    self.as_ref().either(
                        |t| <T as SubmsgTrait>::i32_optional(t),
                        |u| <U as SubmsgTrait>::i32_optional(u),
                    )
                }
            }
            impl<T> SubmsgTrait for ::std::option::Option<T>
            where
                T: SubmsgTrait,
            {
                fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
                    self.as_ref().and_then(|msg| msg.i32_optional())
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            pub struct SubmsgSimpleField1 {
                pub i32_optional: ::std::option::Option<i32>,
            }

            impl ::puroro::Message<SubmsgSimple> for SubmsgSimpleField1 {}

            impl super::_puroro_traits::SubmsgTrait for SubmsgSimpleField1 {
                fn i32_optional<'this>(&'this self) -> Option<i32> {
                    Clone::clone(&self.i32_optional)
                }
            }

            impl ::puroro::SerToIoWrite for SubmsgSimpleField1 {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::Int32
                >::ser_field(&self.i32_optional, 1, out)?;
                    ::std::result::Result::Ok(())
                }
            }

            impl ::std::convert::From<::std::option::Option<i32>> for SubmsgSimpleField1 {
                fn from(value: ::std::option::Option<i32>) -> Self {
                    Self {
                        i32_optional: value,
                    }
                }
            }
            #[derive(
                ::std::clone::Clone,
                ::std::default::Default,
                ::std::cmp::PartialEq,
                ::std::fmt::Debug,
            )]
            pub struct SubmsgSimpleByValue {}
            impl ::puroro::Message<SubmsgSimple> for SubmsgSimpleByValue {}

            impl SubmsgTrait for SubmsgSimpleByValue {
                fn i32_optional<'this>(&'this self) -> Option<i32> {
                    unimplemented!("Please don't use / instantiate this struct!!")
                }
            }
            pub struct SubmsgBuilder<T>(T);

            impl<T> SubmsgBuilder<T>
            where
                T: SubmsgTrait,
            {
                pub fn append_i32_optional(
                    self,
                    value: ::std::option::Option<i32>,
                ) -> SubmsgBuilder<(T, SubmsgSimpleField1)> {
                    SubmsgBuilder((self.0, ::std::convert::From::from(value)))
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

            pub trait SubmsgTrait {
                fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
                    ::std::default::Default::default()
                }
            }

            macro_rules! submsg_delegate {
                ($ty:ty) => {
                    fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
                        (**self).i32_optional()
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
            pub mod submsg {
                mod _puroro_root {
                    pub use super::super::super::_puroro_root::*;
                }
            }
        }
    }
}
