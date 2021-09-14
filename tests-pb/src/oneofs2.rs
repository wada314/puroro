// A generated source code by puroro library
// package oneofs2

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
        pub group_one: ::std::option::Option<
            super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'static, MsgSimpleByValue>,
        >,
        pub group_two: ::std::option::Option<
            super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'static, MsgSimpleByValue>,
        >,
        pub group_three:
            ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree>,
    }
    impl ::puroro::Message<MsgSimple> for MsgSimple {}

    impl MsgTrait for MsgSimple {
        type Field2StringType<'this> = &'this str;
        type Field4StringType<'this> = &'this str;
        type Field5MessageType<'this> =
            &'this self::_puroro_root::oneofs2::_puroro_impls::SubmsgSimple;
        fn group_one<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this, Self>> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupOne as E;
            self.group_one.as_ref().map(|oneof| match oneof {
                E::G1Int32(v) => E::G1Int32(v.clone()),

                E::G1String(v) => E::G1String(v.as_ref()),
            })
        }
        fn group_two<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
            self.group_two.as_ref().map(|oneof| match oneof {
                E::G2F32(v) => E::G2F32(v.clone()),

                E::G2String(v) => E::G2String(v.as_ref()),

                E::G2Submsg(v) => E::G2Submsg(v.as_ref()),
            })
        }
        fn group_three<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupThree as E;
            self.group_three.as_ref().map(|oneof| match oneof {
                E::G3Int32(v) => E::G3Int32(v.clone()),
            })
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
                                name: "g1_int32",
                                number: 1,
                                lazy_containing_type: Lazy::new(|| {
                                    <MsgSimple as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "g1_string",
                                number: 2,
                                lazy_containing_type: Lazy::new(|| {
                                    <MsgSimple as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "g2_f32",
                                number: 3,
                                lazy_containing_type: Lazy::new(|| {
                                    <MsgSimple as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "g2_string",
                                number: 4,
                                lazy_containing_type: Lazy::new(|| {
                                    <MsgSimple as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "g2_submsg",
                                number: 5,
                                lazy_containing_type: Lazy::new(|| {
                                    <MsgSimple as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "g3_int32",
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
                1 => {
                    use super::_puroro_nested::msg::_puroro_oneofs::GroupOne as E;
                    if !matches!(&self.group_one, Some(E::G1Int32(_))) {
                        self.group_one = Some(E::G1Int32(::std::default::Default::default()));
                    }
                    let field_value_mut_ref = match &mut self.group_one {
                        Some(E::G1Int32(v)) => v,
                        _ => unreachable!(),
                    };
                    DeserFieldFromBytesIter::<
                    ::puroro::tags::OneofItem, ::puroro::tags::Int32
                >::deser_field(field_value_mut_ref, data)
                }
                2 => {
                    use super::_puroro_nested::msg::_puroro_oneofs::GroupOne as E;
                    if !matches!(&self.group_one, Some(E::G1String(_))) {
                        self.group_one = Some(E::G1String(::std::default::Default::default()));
                    }
                    let field_value_mut_ref = match &mut self.group_one {
                        Some(E::G1String(v)) => v,
                        _ => unreachable!(),
                    };
                    DeserFieldFromBytesIter::<
                    ::puroro::tags::OneofItem, ::puroro::tags::String
                >::deser_field(field_value_mut_ref, data)
                }
                3 => {
                    use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
                    if !matches!(&self.group_two, Some(E::G2F32(_))) {
                        self.group_two = Some(E::G2F32(::std::default::Default::default()));
                    }
                    let field_value_mut_ref = match &mut self.group_two {
                        Some(E::G2F32(v)) => v,
                        _ => unreachable!(),
                    };
                    DeserFieldFromBytesIter::<
                    ::puroro::tags::OneofItem, ::puroro::tags::Float
                >::deser_field(field_value_mut_ref, data)
                }
                4 => {
                    use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
                    if !matches!(&self.group_two, Some(E::G2String(_))) {
                        self.group_two = Some(E::G2String(::std::default::Default::default()));
                    }
                    let field_value_mut_ref = match &mut self.group_two {
                        Some(E::G2String(v)) => v,
                        _ => unreachable!(),
                    };
                    DeserFieldFromBytesIter::<
                    ::puroro::tags::OneofItem, ::puroro::tags::String
                >::deser_field(field_value_mut_ref, data)
                }
                5 => {
                    use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
                    if !matches!(&self.group_two, Some(E::G2Submsg(_))) {
                        self.group_two = Some(E::G2Submsg(::std::default::Default::default()));
                    }
                    let field_value_mut_ref = match &mut self.group_two {
                        Some(E::G2Submsg(v)) => v,
                        _ => unreachable!(),
                    };
                    DeserFieldFromBytesIter::<
                        ::puroro::tags::OneofItem,
                        ::puroro::tags::Message<
                            self::_puroro_root::oneofs2::_puroro_impls::SubmsgSimple,
                        >,
                    >::deser_field(field_value_mut_ref, data)
                }
                6 => {
                    use super::_puroro_nested::msg::_puroro_oneofs::GroupThree as E;
                    if !matches!(&self.group_three, Some(E::G3Int32(_))) {
                        self.group_three = Some(E::G3Int32(::std::default::Default::default()));
                    }
                    let field_value_mut_ref = match &mut self.group_three {
                        Some(E::G3Int32(v)) => v,
                        _ => unreachable!(),
                    };
                    DeserFieldFromBytesIter::<
                    ::puroro::tags::OneofItem, ::puroro::tags::Int32
                >::deser_field(field_value_mut_ref, data)
                }

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
            if let Some(oneof) = self.group_one.as_ref() {
                use super::_puroro_nested::msg::_puroro_oneofs::GroupOne as E;
                match oneof {
                    E::G1Int32(v) => SerFieldToIoWrite::<
                        ::puroro::tags::OneofItem,
                        ::puroro::tags::Int32,
                    >::ser_field(v, 1, out)?,
                    E::G1String(v) => SerFieldToIoWrite::<
                        ::puroro::tags::OneofItem,
                        ::puroro::tags::String,
                    >::ser_field(v, 2, out)?,
                }
            }
            if let Some(oneof) = self.group_two.as_ref() {
                use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
                match oneof {
                    E::G2F32(v) => SerFieldToIoWrite::<
                        ::puroro::tags::OneofItem,
                        ::puroro::tags::Float,
                    >::ser_field(v, 3, out)?,
                    E::G2String(v) => SerFieldToIoWrite::<
                        ::puroro::tags::OneofItem,
                        ::puroro::tags::String,
                    >::ser_field(v, 4, out)?,
                    E::G2Submsg(v) => SerFieldToIoWrite::<
                        ::puroro::tags::OneofItem,
                        ::puroro::tags::Message<
                            self::_puroro_root::oneofs2::_puroro_impls::SubmsgSimple,
                        >,
                    >::ser_field(v, 5, out)?,
                }
            }
            if let Some(oneof) = self.group_three.as_ref() {
                use super::_puroro_nested::msg::_puroro_oneofs::GroupThree as E;
                match oneof {
                    E::G3Int32(v) => SerFieldToIoWrite::<
                        ::puroro::tags::OneofItem,
                        ::puroro::tags::Int32,
                    >::ser_field(v, 6, out)?,
                }
            }

            ::std::result::Result::Ok(())
        }
    }
    impl MsgTrait for () {
        type Field2StringType<'this> = &'static str;
        type Field4StringType<'this> = &'static str;
        type Field5MessageType<'this> = ();
        fn group_one<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this, Self>> {
            None
        }
        fn group_two<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>> {
            None
        }
        fn group_three<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            None
        }
    }
    impl<T, U> MsgTrait for (T, U)
    where
        T: MsgTrait,
        U: MsgTrait,
    {
        type Field2StringType<'this> = ::puroro::Either<
            <T as MsgTrait>::Field2StringType<'this>,
            <U as MsgTrait>::Field2StringType<'this>,
        >;
        type Field4StringType<'this> = ::puroro::Either<
            <T as MsgTrait>::Field4StringType<'this>,
            <U as MsgTrait>::Field4StringType<'this>,
        >;
        type Field5MessageType<'this> = ::puroro::Either<
            ::puroro::Either<
                <T as MsgTrait>::Field5MessageType<'this>,
                <U as MsgTrait>::Field5MessageType<'this>,
            >,
            (
                <T as MsgTrait>::Field5MessageType<'this>,
                <U as MsgTrait>::Field5MessageType<'this>,
            ),
        >;
        fn group_one<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this, Self>> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupOne as E;
            Some(match (self.0.group_one(), self.1.group_one()) {
                (None, None) => {
                    return None;
                }
                (Some(E::G1Int32(_)), Some(E::G1Int32(right))) => E::G1Int32(right),
                (Some(E::G1String(_)), Some(E::G1String(right))) => {
                    E::G1String(::puroro::Either::Right(right))
                }
                (_, Some(E::G1Int32(right))) => E::G1Int32(right),
                (_, Some(E::G1String(right))) => E::G1String(::puroro::Either::Right(right)),
                (Some(E::G1Int32(left)), None) => E::G1Int32(left),
                (Some(E::G1String(left)), None) => E::G1String(::puroro::Either::Left(left)),
            })
        }
        fn group_two<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
            Some(match (self.0.group_two(), self.1.group_two()) {
                (None, None) => {
                    return None;
                }
                (Some(E::G2F32(_)), Some(E::G2F32(right))) => E::G2F32(right),
                (Some(E::G2String(_)), Some(E::G2String(right))) => {
                    E::G2String(::puroro::Either::Right(right))
                }
                (Some(E::G2Submsg(left)), Some(E::G2Submsg(right))) => {
                    E::G2Submsg(::puroro::Either::Right((left, right)))
                }
                (_, Some(E::G2F32(right))) => E::G2F32(right),
                (_, Some(E::G2String(right))) => E::G2String(::puroro::Either::Right(right)),
                (_, Some(E::G2Submsg(right))) => {
                    E::G2Submsg(::puroro::Either::Left(::puroro::Either::Right(right)))
                }
                (Some(E::G2F32(left)), None) => E::G2F32(left),
                (Some(E::G2String(left)), None) => E::G2String(::puroro::Either::Left(left)),
                (Some(E::G2Submsg(left)), None) => {
                    E::G2Submsg(::puroro::Either::Left(::puroro::Either::Left(left)))
                }
            })
        }
        fn group_three<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupThree as E;
            Some(match (self.0.group_three(), self.1.group_three()) {
                (None, None) => {
                    return None;
                }
                (Some(E::G3Int32(_)), Some(E::G3Int32(right))) => E::G3Int32(right),
                (_, Some(E::G3Int32(right))) => E::G3Int32(right),
                (Some(E::G3Int32(left)), None) => E::G3Int32(left),
            })
        }
    }
    impl<T, U> MsgTrait for ::puroro::Either<T, U>
    where
        T: MsgTrait,
        U: MsgTrait,
    {
        type Field2StringType<'this> = ::puroro::Either<
            <T as MsgTrait>::Field2StringType<'this>,
            <U as MsgTrait>::Field2StringType<'this>,
        >;
        type Field4StringType<'this> = ::puroro::Either<
            <T as MsgTrait>::Field4StringType<'this>,
            <U as MsgTrait>::Field4StringType<'this>,
        >;
        type Field5MessageType<'this> = ::puroro::Either<
            <T as MsgTrait>::Field5MessageType<'this>,
            <U as MsgTrait>::Field5MessageType<'this>,
        >;
        fn group_one<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this, Self>> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupOne as E;
            match self.as_ref().either(
                |t| t.group_one().map(|t| ::puroro::Either::Left(t)),
                |u| u.group_one().map(|u| ::puroro::Either::Right(u)),
            ) {
                Some(::puroro::Either::Left(E::G1Int32(v))) => Some(E::G1Int32(v)),
                Some(::puroro::Either::Right(E::G1Int32(v))) => Some(E::G1Int32(v)),
                Some(::puroro::Either::Left(E::G1String(v))) => {
                    Some(E::G1String(::puroro::Either::Left(v)))
                }
                Some(::puroro::Either::Right(E::G1String(v))) => {
                    Some(E::G1String(::puroro::Either::Right(v)))
                }
                None => None,
            }
        }
        fn group_two<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
            match self.as_ref().either(
                |t| t.group_two().map(|t| ::puroro::Either::Left(t)),
                |u| u.group_two().map(|u| ::puroro::Either::Right(u)),
            ) {
                Some(::puroro::Either::Left(E::G2F32(v))) => Some(E::G2F32(v)),
                Some(::puroro::Either::Right(E::G2F32(v))) => Some(E::G2F32(v)),
                Some(::puroro::Either::Left(E::G2String(v))) => {
                    Some(E::G2String(::puroro::Either::Left(v)))
                }
                Some(::puroro::Either::Right(E::G2String(v))) => {
                    Some(E::G2String(::puroro::Either::Right(v)))
                }
                Some(::puroro::Either::Left(E::G2Submsg(v))) => {
                    Some(E::G2Submsg(::puroro::Either::Left(v)))
                }
                Some(::puroro::Either::Right(E::G2Submsg(v))) => {
                    Some(E::G2Submsg(::puroro::Either::Right(v)))
                }
                None => None,
            }
        }
        fn group_three<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupThree as E;
            match self.as_ref().either(
                |t| t.group_three().map(|t| ::puroro::Either::Left(t)),
                |u| u.group_three().map(|u| ::puroro::Either::Right(u)),
            ) {
                Some(::puroro::Either::Left(E::G3Int32(v))) => Some(E::G3Int32(v)),
                Some(::puroro::Either::Right(E::G3Int32(v))) => Some(E::G3Int32(v)),
                None => None,
            }
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct MsgSimpleByValue {}
    impl ::puroro::Message<MsgSimple> for MsgSimpleByValue {}

    impl MsgTrait for MsgSimpleByValue {
        type Field2StringType<'this> = ::std::borrow::Cow<'this, str>;
        type Field4StringType<'this> = ::std::borrow::Cow<'this, str>;
        type Field5MessageType<'this> =
            ::std::boxed::Box<self::_puroro_root::oneofs2::_puroro_impls::SubmsgSimple>;
        fn group_one<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this, Self>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn group_two<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn group_three<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
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
                ::puroro::tags::Optional, ::puroro::tags::Int32
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
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Int32>::ser_field(
                &self.i32_optional,
                1,
                out,
            )?;

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
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Int32>::ser_field(
                &self.i32_optional,
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
        fn i32_optional<'this>(&'this self) -> Option<i32> {
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
        type Field2StringType<'this>: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug;
        type Field4StringType<'this>: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug;
        type Field5MessageType<'this>: self::_puroro_root::oneofs2::_puroro_traits::SubmsgTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug;
        fn group_one<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this, Self>>
        {
            ::std::option::Option::None
        }
        fn g1_int32<'this>(&'this self) -> Option<i32> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupOne as E;
            self.group_one().and_then(|oneof| {
                #[allow(irrefutable_let_patterns)]
                if let E::G1Int32(v) = oneof {
                    Some(v)
                } else {
                    None
                }
            })
        }
        fn g1_string<'this>(
            &'this self,
        ) -> Option<
            <Self as self::_puroro_root::oneofs2::_puroro_traits::MsgTrait>::Field2StringType<
                'this,
            >,
        > {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupOne as E;
            self.group_one().and_then(|oneof| {
                #[allow(irrefutable_let_patterns)]
                if let E::G1String(v) = oneof {
                    Some(v)
                } else {
                    None
                }
            })
        }
        fn group_two<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>>
        {
            ::std::option::Option::None
        }
        fn g2_f32<'this>(&'this self) -> Option<f32> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
            self.group_two().and_then(|oneof| {
                #[allow(irrefutable_let_patterns)]
                if let E::G2F32(v) = oneof {
                    Some(v)
                } else {
                    None
                }
            })
        }
        fn g2_string<'this>(
            &'this self,
        ) -> Option<
            <Self as self::_puroro_root::oneofs2::_puroro_traits::MsgTrait>::Field4StringType<
                'this,
            >,
        > {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
            self.group_two().and_then(|oneof| {
                #[allow(irrefutable_let_patterns)]
                if let E::G2String(v) = oneof {
                    Some(v)
                } else {
                    None
                }
            })
        }
        fn g2_submsg<'this>(
            &'this self,
        ) -> Option<
            <Self as self::_puroro_root::oneofs2::_puroro_traits::MsgTrait>::Field5MessageType<
                'this,
            >,
        > {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
            self.group_two().and_then(|oneof| {
                #[allow(irrefutable_let_patterns)]
                if let E::G2Submsg(v) = oneof {
                    Some(v)
                } else {
                    None
                }
            })
        }
        fn group_three<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            ::std::option::Option::None
        }
        fn g3_int32<'this>(&'this self) -> Option<i32> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupThree as E;
            self.group_three().and_then(|oneof| {
                #[allow(irrefutable_let_patterns)]
                if let E::G3Int32(v) = oneof {
                    Some(v)
                } else {
                    None
                }
            })
        }
    }

    macro_rules! msg_delegate {
        ($ty:ty) => {
            type Field2StringType<'this> = <$ty>::Field2StringType<'this>;
            type Field4StringType<'this> = <$ty>::Field4StringType<'this>;
            type Field5MessageType<'this> = <$ty>::Field5MessageType<'this>;
            fn group_one<'this>(
                &'this self,
            ) -> ::std::option::Option<
                super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this, Self>,
            > {
                (**self).group_one().map(|v| v.into())
            }
            fn group_two<'this>(
                &'this self,
            ) -> ::std::option::Option<
                super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>,
            > {
                (**self).group_two().map(|v| v.into())
            }
            fn group_three<'this>(
                &'this self,
            ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
                (**self).group_three().map(|v| v.into())
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
    pub mod msg {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }

        pub use _puroro_oneofs::*;
        pub mod _puroro_oneofs {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }
            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            pub enum GroupOne<
                'msg,
                T: ?Sized + self::_puroro_root::oneofs2::_puroro_traits::MsgTrait,
            > {
                G1Int32(i32),
                G1String(
                    <T as self::_puroro_root::oneofs2::_puroro_traits::MsgTrait>::Field2StringType<
                        'msg,
                    >,
                ),
            }
            impl<'msg, T> ::std::convert::From<GroupOne<'msg, T>> for GroupOne<'msg, &'_ T>
            where
                T: self::_puroro_root::oneofs2::_puroro_traits::MsgTrait,
            {
                fn from(value: GroupOne<'msg, T>) -> Self {
                    match value {
                        GroupOne::G1Int32(v) => GroupOne::G1Int32(v),
                        GroupOne::G1String(v) => GroupOne::G1String(v),
                    }
                }
            }
            impl<'msg, T> ::std::convert::From<GroupOne<'msg, T>> for GroupOne<'msg, ::std::boxed::Box<T>>
            where
                T: self::_puroro_root::oneofs2::_puroro_traits::MsgTrait,
            {
                fn from(value: GroupOne<'msg, T>) -> Self {
                    match value {
                        GroupOne::G1Int32(v) => GroupOne::G1Int32(v),
                        GroupOne::G1String(v) => GroupOne::G1String(v),
                    }
                }
            }
            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            pub enum GroupTwo<
                'msg,
                T: ?Sized + self::_puroro_root::oneofs2::_puroro_traits::MsgTrait,
            > {
                G2F32(f32),
                G2String(<T as self::_puroro_root::oneofs2::_puroro_traits::MsgTrait>::Field4StringType<'msg>),
                G2Submsg(<T as self::_puroro_root::oneofs2::_puroro_traits::MsgTrait>::Field5MessageType<'msg>),
            }
            impl<'msg, T> ::std::convert::From<GroupTwo<'msg, T>> for GroupTwo<'msg, &'_ T>
            where
                T: self::_puroro_root::oneofs2::_puroro_traits::MsgTrait,
            {
                fn from(value: GroupTwo<'msg, T>) -> Self {
                    match value {
                        GroupTwo::G2F32(v) => GroupTwo::G2F32(v),
                        GroupTwo::G2String(v) => GroupTwo::G2String(v),
                        GroupTwo::G2Submsg(v) => GroupTwo::G2Submsg(v),
                    }
                }
            }
            impl<'msg, T> ::std::convert::From<GroupTwo<'msg, T>> for GroupTwo<'msg, ::std::boxed::Box<T>>
            where
                T: self::_puroro_root::oneofs2::_puroro_traits::MsgTrait,
            {
                fn from(value: GroupTwo<'msg, T>) -> Self {
                    match value {
                        GroupTwo::G2F32(v) => GroupTwo::G2F32(v),
                        GroupTwo::G2String(v) => GroupTwo::G2String(v),
                        GroupTwo::G2Submsg(v) => GroupTwo::G2Submsg(v),
                    }
                }
            }
            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            pub enum GroupThree {
                G3Int32(i32),
            }
        }
    }
    pub mod submsg {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
    }
}
