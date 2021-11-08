// A generated source code by puroro library
// package oneofs2

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub use _puroro_simple_impl::Msg;
pub use _puroro_simple_impl::Submsg;
pub mod _puroro_simple_impl {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct Msg {
        pub group_one: ::std::option::Option<
            super::_puroro_nested::msg::_puroro_oneofs::GroupOne<
                'static,
                ::puroro::internal::bool::True,
            >,
        >,
        pub group_two: ::std::option::Option<
            super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<
                'static,
                ::puroro::internal::bool::True,
                Self,
            >,
        >,
        pub group_three:
            ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree>,
    }
    impl ::puroro::Message<Msg> for Msg {}

    impl Msg {
        pub fn new() -> Self {
            Self {
                group_one: ::std::default::Default::default(),
                group_two: ::std::default::Default::default(),
                group_three: ::std::default::Default::default(),
            }
        }
    }

    impl super::_puroro_traits::MsgTrait for Msg {
        type Field5MessageType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::oneofs2::_puroro_simple_impl::Submsg;
        fn group_one<'this>(
            &'this self,
        ) -> Option<
            super::_puroro_nested::msg::_puroro_oneofs::GroupOne<
                'this,
                ::puroro::internal::bool::False,
            >,
        > {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupOne as E;
            self.group_one.as_ref().map(|oneof| match oneof {
                E::G1Int32(v) => E::G1Int32(v.clone()),

                E::G1String(v) => E::G1String(v.as_ref()),
            })
        }
        fn group_two<'this>(
            &'this self,
        ) -> Option<
            super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<
                'this,
                ::puroro::internal::bool::False,
                Self,
            >,
        > {
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

    impl ::puroro::MessageRepresentativeImpl for Msg {
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
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "g1_string",
                                number: 2,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "g2_f32",
                                number: 3,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "g2_string",
                                number: 4,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "g2_submsg",
                                number: 5,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "g3_int32",
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
                    ::puroro::tags::OneofField, ::puroro::tags::Int32
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
                    ::puroro::tags::OneofField, ::puroro::tags::String
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
                    ::puroro::tags::OneofField, ::puroro::tags::Float
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
                    ::puroro::tags::OneofField, ::puroro::tags::String
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
                        ::puroro::tags::OneofField,
                        ::puroro::tags::Message<
                            ::std::boxed::Box<
                                self::_puroro_root::oneofs2::_puroro_simple_impl::Submsg,
                            >,
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
                    ::puroro::tags::OneofField, ::puroro::tags::Int32
                >::deser_field(field_value_mut_ref, data)
                }

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
                ::puroro::tags::OneofField,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g1_int32_opt(self),
                1,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g1_string_opt(self),
                2,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::Float,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g2_f32_opt(self),
                3,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g2_string_opt(self),
                4,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::Message<
                    <Self as super::_puroro_traits::MsgTrait>::Field5MessageType<'_>,
                >,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g2_submsg_opt(self),
                5,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g3_int32_opt(self),
                6,
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
    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct Submsg {
        pub i32_optional: ::std::option::Option<i32>,
    }
    impl ::puroro::Message<Submsg> for Submsg {}

    impl Submsg {
        pub fn new() -> Self {
            Self {
                i32_optional: ::std::default::Default::default(),
            }
        }
    }

    impl super::_puroro_traits::SubmsgTrait for Submsg {
        fn i32_optional_opt<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.i32_optional)
        }
    }

    impl ::puroro::MessageRepresentativeImpl for Submsg {
        fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
            use ::puroro::once_cell::sync::Lazy;
            static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 1]> =
                Lazy::new(|| {
                    [{
                        let init = ::puroro::internal::FieldDescriptorInitializer {
                            name: "i32_optional",
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

    impl ::puroro::internal::de::DeserMessageFromBytesIter for Submsg {
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
            >::deser_field(&mut self.i32_optional, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::internal::se::SerMessageToIoWrite for Submsg
    where
        Self: super::_puroro_traits::SubmsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::SubmsgTrait>::i32_optional_opt(self),
                1,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::default::Default for Submsg {
        fn default() -> Self {
            Self::new()
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
        type Field5MessageType<'this>
        where
            Self: 'this,
        = ();
        fn group_one<'this>(
            &'this self,
        ) -> Option<
            super::_puroro_nested::msg::_puroro_oneofs::GroupOne<
                'this,
                ::puroro::internal::bool::False,
            >,
        > {
            None
        }
        fn group_two<'this>(
            &'this self,
        ) -> Option<
            super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<
                'this,
                ::puroro::internal::bool::False,
                Self,
            >,
        > {
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
        type Field5MessageType<'this>
        where
            Self: 'this,
        = (
            ::std::option::Option<<T as MsgTrait>::Field5MessageType<'this>>,
            ::std::option::Option<<U as MsgTrait>::Field5MessageType<'this>>,
        );
        fn group_one<'this>(
            &'this self,
        ) -> Option<
            super::_puroro_nested::msg::_puroro_oneofs::GroupOne<
                'this,
                ::puroro::internal::bool::False,
            >,
        > {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupOne as E;
            Some(match (self.0.group_one(), self.1.group_one()) {
                (None, None) => {
                    return None;
                }
                (Some(E::G1Int32(_)), Some(E::G1Int32(right))) => E::G1Int32(right),
                (Some(E::G1String(_)), Some(E::G1String(right))) => E::G1String(right),
                (_, Some(E::G1Int32(right))) => E::G1Int32(right),
                (_, Some(E::G1String(right))) => E::G1String(right),
                (Some(E::G1Int32(left)), None) => E::G1Int32(left),
                (Some(E::G1String(left)), None) => E::G1String(left),
            })
        }
        fn group_two<'this>(
            &'this self,
        ) -> Option<
            super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<
                'this,
                ::puroro::internal::bool::False,
                Self,
            >,
        > {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
            Some(match (self.0.group_two(), self.1.group_two()) {
                (None, None) => {
                    return None;
                }
                (Some(E::G2F32(_)), Some(E::G2F32(right))) => E::G2F32(right),
                (Some(E::G2String(_)), Some(E::G2String(right))) => E::G2String(right),
                (Some(E::G2Submsg(left)), Some(E::G2Submsg(right))) => {
                    E::G2Submsg((Some(left), Some(right)))
                }
                (_, Some(E::G2F32(right))) => E::G2F32(right),
                (_, Some(E::G2String(right))) => E::G2String(right),
                (_, Some(E::G2Submsg(right))) => E::G2Submsg((None, Some(right))),
                (Some(E::G2F32(left)), None) => E::G2F32(left),
                (Some(E::G2String(left)), None) => E::G2String(left),
                (Some(E::G2Submsg(left)), None) => E::G2Submsg((Some(left), None)),
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
        type Field5MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as MsgTrait>::Field5MessageType<'this>,
            <U as MsgTrait>::Field5MessageType<'this>,
        >;
        fn group_one<'this>(
            &'this self,
        ) -> Option<
            super::_puroro_nested::msg::_puroro_oneofs::GroupOne<
                'this,
                ::puroro::internal::bool::False,
            >,
        > {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupOne as E;
            match self.as_ref().either(
                |t| t.group_one().map(|t| ::puroro::Either::Left(t)),
                |u| u.group_one().map(|u| ::puroro::Either::Right(u)),
            ) {
                Some(::puroro::Either::Left(E::G1Int32(v))) => Some(E::G1Int32(v)),
                Some(::puroro::Either::Right(E::G1Int32(v))) => Some(E::G1Int32(v)),
                Some(::puroro::Either::Left(E::G1String(v))) => Some(E::G1String(v)),
                Some(::puroro::Either::Right(E::G1String(v))) => Some(E::G1String(v)),
                None => None,
            }
        }
        fn group_two<'this>(
            &'this self,
        ) -> Option<
            super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<
                'this,
                ::puroro::internal::bool::False,
                Self,
            >,
        > {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
            match self.as_ref().either(
                |t| t.group_two().map(|t| ::puroro::Either::Left(t)),
                |u| u.group_two().map(|u| ::puroro::Either::Right(u)),
            ) {
                Some(::puroro::Either::Left(E::G2F32(v))) => Some(E::G2F32(v)),
                Some(::puroro::Either::Right(E::G2F32(v))) => Some(E::G2F32(v)),
                Some(::puroro::Either::Left(E::G2String(v))) => Some(E::G2String(v)),
                Some(::puroro::Either::Right(E::G2String(v))) => Some(E::G2String(v)),
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
    impl<T> MsgTrait for ::std::option::Option<T>
    where
        T: MsgTrait,
    {
        type Field5MessageType<'this>
        where
            Self: 'this,
        = T::Field5MessageType<'this>;
        fn group_one<'this>(
            &'this self,
        ) -> Option<
            super::_puroro_nested::msg::_puroro_oneofs::GroupOne<
                'this,
                ::puroro::internal::bool::False,
            >,
        > {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupOne as E;
            self.as_ref().and_then(|msg| {
                msg.group_one().map(|oneof| match oneof {
                    E::G1Int32(v) => E::G1Int32(v),
                    E::G1String(v) => E::G1String(v),
                })
            })
        }
        fn group_two<'this>(
            &'this self,
        ) -> Option<
            super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<
                'this,
                ::puroro::internal::bool::False,
                Self,
            >,
        > {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
            self.as_ref().and_then(|msg| {
                msg.group_two().map(|oneof| match oneof {
                    E::G2F32(v) => E::G2F32(v),
                    E::G2String(v) => E::G2String(v),
                    E::G2Submsg(v) => E::G2Submsg(v),
                })
            })
        }
        fn group_three<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupThree as E;
            self.as_ref().and_then(|msg| {
                msg.group_three().map(|oneof| match oneof {
                    E::G3Int32(v) => E::G3Int32(v),
                })
            })
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub g1_int32: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField1<ScalarType> where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field5MessageType<'this>
        where
            Self: 'this,
        = ();
        fn group_one<'this>(
            &'this self,
        ) -> ::std::option::Option<
            super::_puroro_nested::msg::_puroro_oneofs::GroupOne<
                'this,
                ::puroro::internal::bool::False,
            >,
        > {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupOne as E;
            ::std::option::Option::Some(E::G1Int32(::std::convert::Into::into(
                ::std::clone::Clone::clone(&self.g1_int32),
            )))
        }
        fn group_two<'this>(
            &'this self,
        ) -> ::std::option::Option<
            super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<
                'this,
                ::puroro::internal::bool::False,
                Self,
            >,
        > {
            ::std::option::Option::None
        }
        fn group_three<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            ::std::option::Option::None
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g1_int32_opt(self),
                1,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self { g1_int32: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub g1_string: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField2<ScalarType> where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field5MessageType<'this>
        where
            Self: 'this,
        = ();
        fn group_one<'this>(
            &'this self,
        ) -> ::std::option::Option<
            super::_puroro_nested::msg::_puroro_oneofs::GroupOne<
                'this,
                ::puroro::internal::bool::False,
            >,
        > {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupOne as E;
            ::std::option::Option::Some(E::G1String(self.g1_string.as_ref()))
        }
        fn group_two<'this>(
            &'this self,
        ) -> ::std::option::Option<
            super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<
                'this,
                ::puroro::internal::bool::False,
                Self,
            >,
        > {
            ::std::option::Option::None
        }
        fn group_three<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            ::std::option::Option::None
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g1_string_opt(self),
                2,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self { g1_string: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField3<ScalarType>
    where
        ScalarType: ::std::convert::Into<f32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub g2_f32: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField3<ScalarType> where
        ScalarType: ::std::convert::Into<f32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField3<ScalarType>
    where
        ScalarType: ::std::convert::Into<f32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field5MessageType<'this>
        where
            Self: 'this,
        = ();
        fn group_one<'this>(
            &'this self,
        ) -> ::std::option::Option<
            super::_puroro_nested::msg::_puroro_oneofs::GroupOne<
                'this,
                ::puroro::internal::bool::False,
            >,
        > {
            ::std::option::Option::None
        }
        fn group_two<'this>(
            &'this self,
        ) -> ::std::option::Option<
            super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<
                'this,
                ::puroro::internal::bool::False,
                Self,
            >,
        > {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
            ::std::option::Option::Some(E::G2F32(::std::convert::Into::into(
                ::std::clone::Clone::clone(&self.g2_f32),
            )))
        }
        fn group_three<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            ::std::option::Option::None
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField3<ScalarType>
    where
        ScalarType: ::std::convert::Into<f32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::Float,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g2_f32_opt(self),
                3,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField3<ScalarType>
    where
        ScalarType: ::std::convert::Into<f32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self { g2_f32: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField4<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub g2_string: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField4<ScalarType> where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField4<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field5MessageType<'this>
        where
            Self: 'this,
        = ();
        fn group_one<'this>(
            &'this self,
        ) -> ::std::option::Option<
            super::_puroro_nested::msg::_puroro_oneofs::GroupOne<
                'this,
                ::puroro::internal::bool::False,
            >,
        > {
            ::std::option::Option::None
        }
        fn group_two<'this>(
            &'this self,
        ) -> ::std::option::Option<
            super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<
                'this,
                ::puroro::internal::bool::False,
                Self,
            >,
        > {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
            ::std::option::Option::Some(E::G2String(self.g2_string.as_ref()))
        }
        fn group_three<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            ::std::option::Option::None
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField4<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g2_string_opt(self),
                4,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField4<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self { g2_string: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField5<ScalarType>
    where
        ScalarType: self::_puroro_root::oneofs2::_puroro_traits::SubmsgTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub g2_submsg: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField5<ScalarType> where
        ScalarType: self::_puroro_root::oneofs2::_puroro_traits::SubmsgTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField5<ScalarType>
    where
        ScalarType: self::_puroro_root::oneofs2::_puroro_traits::SubmsgTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field5MessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;
        fn group_one<'this>(
            &'this self,
        ) -> ::std::option::Option<
            super::_puroro_nested::msg::_puroro_oneofs::GroupOne<
                'this,
                ::puroro::internal::bool::False,
            >,
        > {
            ::std::option::Option::None
        }
        fn group_two<'this>(
            &'this self,
        ) -> ::std::option::Option<
            super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<
                'this,
                ::puroro::internal::bool::False,
                Self,
            >,
        > {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
            ::std::option::Option::Some(E::G2Submsg(&self.g2_submsg))
        }
        fn group_three<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            ::std::option::Option::None
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField5<ScalarType>
    where
        ScalarType: self::_puroro_root::oneofs2::_puroro_traits::SubmsgTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        Self: super::_puroro_traits::MsgTrait,
        for<'a> <Self as super::_puroro_traits::MsgTrait>::Field5MessageType<'a>:
            ::puroro::internal::se::SerMessageToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::Message<
                    <Self as super::_puroro_traits::MsgTrait>::Field5MessageType<'_>,
                >,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g2_submsg_opt(self),
                5,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField5<ScalarType>
    where
        ScalarType: self::_puroro_root::oneofs2::_puroro_traits::SubmsgTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self { g2_submsg: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField6<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub g3_int32: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField6<ScalarType> where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField6<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field5MessageType<'this>
        where
            Self: 'this,
        = ();
        fn group_one<'this>(
            &'this self,
        ) -> ::std::option::Option<
            super::_puroro_nested::msg::_puroro_oneofs::GroupOne<
                'this,
                ::puroro::internal::bool::False,
            >,
        > {
            ::std::option::Option::None
        }
        fn group_two<'this>(
            &'this self,
        ) -> ::std::option::Option<
            super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<
                'this,
                ::puroro::internal::bool::False,
                Self,
            >,
        > {
            ::std::option::Option::None
        }
        fn group_three<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupThree as E;
            ::std::option::Option::Some(E::G3Int32(::std::convert::Into::into(
                ::std::clone::Clone::clone(&self.g3_int32),
            )))
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField6<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g3_int32_opt(self),
                6,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField6<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self { g3_int32: value }
        }
    }
    #[derive(::std::fmt::Debug)]
    pub struct MsgBumpalo<'bump> {
        _bump: &'bump ::puroro::bumpalo::Bump,
        pub group_one: ::std::option::Option<
            super::_puroro_nested::msg::_puroro_bumpalo_oneofs::GroupOne<'bump>,
        >,
        pub group_two: ::std::option::Option<
            super::_puroro_nested::msg::_puroro_bumpalo_oneofs::GroupTwo<'bump>,
        >,
        pub group_three:
            ::std::option::Option<super::_puroro_nested::msg::_puroro_bumpalo_oneofs::GroupThree>,
    }

    pub type MsgBumpaloOwned = ::puroro::BumpaloOwned<MsgBumpalo<'static>>;

    impl<'bump> MsgBumpalo<'bump> {
        pub fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
            Self {
                _bump: bump,
                group_one: ::std::option::Option::None,
                group_two: ::std::option::Option::None,
                group_three: ::std::option::Option::None,
            }
        }
    }

    impl<'bump> ::puroro::Message<super::_puroro_simple_impl::Msg> for MsgBumpalo<'bump> {}

    impl<'bump> ::puroro::internal::impls::bumpalo::BumpaloDefault<'bump> for MsgBumpalo<'bump> {
        fn default_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
            Self::new_in(bump)
        }
    }

    impl<'bump> super::_puroro_traits::MsgTrait for MsgBumpalo<'bump> {
        type Field5MessageType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::oneofs2::_puroro_impls::SubmsgBumpalo<'bump>;
        fn group_one<'this>(
            &'this self,
        ) -> Option<
            super::_puroro_nested::msg::_puroro_oneofs::GroupOne<
                'this,
                ::puroro::internal::bool::False,
            >,
        > {
            use super::_puroro_nested::msg::_puroro_bumpalo_oneofs::GroupOne as E;
            use super::_puroro_nested::msg::_puroro_oneofs::GroupOne as F;
            self.group_one.as_ref().map(|oneof| match oneof {
                E::G1Int32(v) => F::G1Int32(v.clone()),

                E::G1String(v) => F::G1String(v.as_ref()),
            })
        }
        fn group_two<'this>(
            &'this self,
        ) -> Option<
            super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<
                'this,
                ::puroro::internal::bool::False,
                Self,
            >,
        > {
            use super::_puroro_nested::msg::_puroro_bumpalo_oneofs::GroupTwo as E;
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as F;
            self.group_two.as_ref().map(|oneof| match oneof {
                E::G2F32(v) => F::G2F32(v.clone()),

                E::G2String(v) => F::G2String(v.as_ref()),

                E::G2Submsg(v) => F::G2Submsg(v.as_ref()),
            })
        }
        fn group_three<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            use super::_puroro_nested::msg::_puroro_bumpalo_oneofs::GroupThree as E;
            use super::_puroro_nested::msg::_puroro_oneofs::GroupThree as F;
            self.group_three.as_ref().map(|oneof| match oneof {
                E::G3Int32(v) => F::G3Int32(v.clone()),
            })
        }
    }

    impl<'bump> ::puroro::internal::de::DeserMessageFromBytesIter for MsgBumpalo<'bump> {
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
            use ::puroro::internal::impls::bumpalo::de::DeserFieldFromBytesIter;
            match field_number {
                1 => {
                    use super::_puroro_nested::msg::_puroro_bumpalo_oneofs::GroupOne as E;
                    if !matches!(&self.group_one, Some(E::G1Int32(_))) {
                        self.group_one = Some(E::G1Int32(
                            ::puroro::internal::impls::bumpalo::BumpaloDefault::default_in(
                                &self._bump,
                            ),
                        ));
                    }
                    let field_value_mut_ref = match &mut self.group_one {
                        Some(E::G1Int32(v)) => v,
                        _ => unreachable!(),
                    };
                    DeserFieldFromBytesIter::<
                    ::puroro::tags::OneofField, ::puroro::tags::Int32
                >::deser_field(field_value_mut_ref, data, &self._bump)
                }
                2 => {
                    use super::_puroro_nested::msg::_puroro_bumpalo_oneofs::GroupOne as E;
                    if !matches!(&self.group_one, Some(E::G1String(_))) {
                        self.group_one = Some(E::G1String(
                            ::puroro::internal::impls::bumpalo::BumpaloDefault::default_in(
                                &self._bump,
                            ),
                        ));
                    }
                    let field_value_mut_ref = match &mut self.group_one {
                        Some(E::G1String(v)) => v,
                        _ => unreachable!(),
                    };
                    DeserFieldFromBytesIter::<
                    ::puroro::tags::OneofField, ::puroro::tags::String
                >::deser_field(field_value_mut_ref, data, &self._bump)
                }
                3 => {
                    use super::_puroro_nested::msg::_puroro_bumpalo_oneofs::GroupTwo as E;
                    if !matches!(&self.group_two, Some(E::G2F32(_))) {
                        self.group_two = Some(E::G2F32(
                            ::puroro::internal::impls::bumpalo::BumpaloDefault::default_in(
                                &self._bump,
                            ),
                        ));
                    }
                    let field_value_mut_ref = match &mut self.group_two {
                        Some(E::G2F32(v)) => v,
                        _ => unreachable!(),
                    };
                    DeserFieldFromBytesIter::<
                    ::puroro::tags::OneofField, ::puroro::tags::Float
                >::deser_field(field_value_mut_ref, data, &self._bump)
                }
                4 => {
                    use super::_puroro_nested::msg::_puroro_bumpalo_oneofs::GroupTwo as E;
                    if !matches!(&self.group_two, Some(E::G2String(_))) {
                        self.group_two = Some(E::G2String(
                            ::puroro::internal::impls::bumpalo::BumpaloDefault::default_in(
                                &self._bump,
                            ),
                        ));
                    }
                    let field_value_mut_ref = match &mut self.group_two {
                        Some(E::G2String(v)) => v,
                        _ => unreachable!(),
                    };
                    DeserFieldFromBytesIter::<
                    ::puroro::tags::OneofField, ::puroro::tags::String
                >::deser_field(field_value_mut_ref, data, &self._bump)
                }
                5 => {
                    use super::_puroro_nested::msg::_puroro_bumpalo_oneofs::GroupTwo as E;
                    if !matches!(&self.group_two, Some(E::G2Submsg(_))) {
                        self.group_two = Some(E::G2Submsg(
                            ::puroro::internal::impls::bumpalo::BumpaloDefault::default_in(
                                &self._bump,
                            ),
                        ));
                    }
                    let field_value_mut_ref = match &mut self.group_two {
                        Some(E::G2Submsg(v)) => v,
                        _ => unreachable!(),
                    };
                    DeserFieldFromBytesIter::<
                        ::puroro::tags::OneofField,
                        ::puroro::tags::Message<
                            ::puroro::bumpalo::boxed::Box<
                                'bump,
                                self::_puroro_root::oneofs2::_puroro_impls::SubmsgBumpalo<'bump>,
                            >,
                        >,
                    >::deser_field(field_value_mut_ref, data, &self._bump)
                }
                6 => {
                    use super::_puroro_nested::msg::_puroro_bumpalo_oneofs::GroupThree as E;
                    if !matches!(&self.group_three, Some(E::G3Int32(_))) {
                        self.group_three = Some(E::G3Int32(
                            ::puroro::internal::impls::bumpalo::BumpaloDefault::default_in(
                                &self._bump,
                            ),
                        ));
                    }
                    let field_value_mut_ref = match &mut self.group_three {
                        Some(E::G3Int32(v)) => v,
                        _ => unreachable!(),
                    };
                    DeserFieldFromBytesIter::<
                    ::puroro::tags::OneofField, ::puroro::tags::Int32
                >::deser_field(field_value_mut_ref, data, &self._bump)
                }

                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl<'bump> ::puroro::internal::se::SerMessageToIoWrite for MsgBumpalo<'bump>
    where
        Self: super::_puroro_traits::MsgTrait,
        for<'a> <Self as super::_puroro_traits::MsgTrait>::Field5MessageType<'a>:
            ::puroro::internal::se::SerMessageToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g1_int32_opt(self),
                1,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g1_string_opt(self),
                2,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::Float,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g2_f32_opt(self),
                3,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g2_string_opt(self),
                4,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::Message<
                    <Self as super::_puroro_traits::MsgTrait>::Field5MessageType<'_>,
                >,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g2_submsg_opt(self),
                5,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g3_int32_opt(self),
                6,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<'bump> ::std::cmp::PartialEq for MsgBumpalo<'bump> {
        fn eq(&self, rhs: &Self) -> bool {
            ::std::ptr::eq(self._bump, rhs._bump)
                && self.group_one == rhs.group_one
                && self.group_two == rhs.group_two
                && self.group_three == rhs.group_three
                && true
        }
    }

    impl<'bump> ::puroro::internal::impls::bumpalo::BumpaloClone<'bump> for MsgBumpalo<'bump> {
        fn clone_in(&self, bump: &'bump ::puroro::bumpalo::Bump) -> Self {
            Self {
                _bump: bump,
                group_one: ::puroro::internal::impls::bumpalo::BumpaloClone::clone_in(
                    &self.group_one,
                    bump,
                ),
                group_two: ::puroro::internal::impls::bumpalo::BumpaloClone::clone_in(
                    &self.group_two,
                    bump,
                ),
                group_three: ::puroro::internal::impls::bumpalo::BumpaloClone::clone_in(
                    &self.group_three,
                    bump,
                ),
            }
        }
    }
    pub struct MsgBuilder<T>(T);

    impl<T> MsgBuilder<T>
    where
        T: MsgTrait,
    {
        pub fn append_g1_int32<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField1<ScalarType>)>
        where
            ScalarType: ::std::convert::Into<i32>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            MsgBuilder((self.0, MsgSingleField1 { g1_int32: value }))
        }

        pub fn append_g1_string<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField2<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            MsgBuilder((self.0, MsgSingleField2 { g1_string: value }))
        }

        pub fn append_g2_f32<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField3<ScalarType>)>
        where
            ScalarType: ::std::convert::Into<f32>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            MsgBuilder((self.0, MsgSingleField3 { g2_f32: value }))
        }

        pub fn append_g2_string<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField4<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            MsgBuilder((self.0, MsgSingleField4 { g2_string: value }))
        }

        pub fn append_g2_submsg<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField5<ScalarType>)>
        where
            ScalarType: self::_puroro_root::oneofs2::_puroro_traits::SubmsgTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            MsgBuilder((self.0, MsgSingleField5 { g2_submsg: value }))
        }

        pub fn append_g3_int32<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField6<ScalarType>)>
        where
            ScalarType: ::std::convert::Into<i32>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            MsgBuilder((self.0, MsgSingleField6 { g3_int32: value }))
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
        fn i32_optional_opt<'this>(&'this self) -> Option<i32> {
            <U as SubmsgTrait>::i32_optional_opt(&self.1)
                .or_else(|| <T as SubmsgTrait>::i32_optional_opt(&self.0))
        }
    }
    impl<T, U> SubmsgTrait for ::puroro::Either<T, U>
    where
        T: SubmsgTrait,
        U: SubmsgTrait,
    {
        fn i32_optional_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().either(
                |t| <T as SubmsgTrait>::i32_optional_opt(t),
                |u| <U as SubmsgTrait>::i32_optional_opt(u),
            )
        }
    }
    impl<T> SubmsgTrait for ::std::option::Option<T>
    where
        T: SubmsgTrait,
    {
        fn i32_optional_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().and_then(|msg| msg.i32_optional_opt())
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct SubmsgSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub i32_optional: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Submsg> for SubmsgSingleField1<ScalarType> where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::SubmsgTrait for SubmsgSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn i32_optional_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.i32_optional,
            )))
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for SubmsgSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        Self: super::_puroro_traits::SubmsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::SubmsgTrait>::i32_optional_opt(self),
                1,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for SubmsgSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                i32_optional: value,
            }
        }
    }
    #[derive(::std::fmt::Debug)]
    pub struct SubmsgBumpalo<'bump> {
        _bump: &'bump ::puroro::bumpalo::Bump,
        pub i32_optional: ::std::option::Option<i32>,
    }

    pub type SubmsgBumpaloOwned = ::puroro::BumpaloOwned<SubmsgBumpalo<'static>>;

    impl<'bump> SubmsgBumpalo<'bump> {
        pub fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
            Self {
                _bump: bump,
                i32_optional: ::std::default::Default::default(),
            }
        }
    }

    impl<'bump> ::puroro::Message<super::_puroro_simple_impl::Submsg> for SubmsgBumpalo<'bump> {}

    impl<'bump> ::puroro::internal::impls::bumpalo::BumpaloDefault<'bump> for SubmsgBumpalo<'bump> {
        fn default_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
            Self::new_in(bump)
        }
    }

    impl<'bump> super::_puroro_traits::SubmsgTrait for SubmsgBumpalo<'bump> {
        fn i32_optional_opt<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.i32_optional)
        }
    }

    impl<'bump> ::puroro::internal::de::DeserMessageFromBytesIter for SubmsgBumpalo<'bump> {
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
            use ::puroro::internal::impls::bumpalo::de::DeserFieldFromBytesIter;
            match field_number {
            1 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Int32
            >::deser_field(&mut self.i32_optional, data, &self._bump),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl<'bump> ::puroro::internal::se::SerMessageToIoWrite for SubmsgBumpalo<'bump>
    where
        Self: super::_puroro_traits::SubmsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::SubmsgTrait>::i32_optional_opt(self),
                1,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<'bump> ::std::cmp::PartialEq for SubmsgBumpalo<'bump> {
        fn eq(&self, rhs: &Self) -> bool {
            ::std::ptr::eq(self._bump, rhs._bump) && self.i32_optional == rhs.i32_optional && true
        }
    }

    impl<'bump> ::puroro::internal::impls::bumpalo::BumpaloClone<'bump> for SubmsgBumpalo<'bump> {
        fn clone_in(&self, bump: &'bump ::puroro::bumpalo::Bump) -> Self {
            Self {
                _bump: bump,
                i32_optional: ::puroro::internal::impls::bumpalo::BumpaloClone::clone_in(
                    &self.i32_optional,
                    bump,
                ),
            }
        }
    }
    pub struct SubmsgBuilder<T>(T);

    impl<T> SubmsgBuilder<T>
    where
        T: SubmsgTrait,
    {
        pub fn append_i32_optional<ScalarType>(
            self,
            value: ScalarType,
        ) -> SubmsgBuilder<(T, SubmsgSingleField1<ScalarType>)>
        where
            ScalarType: ::std::convert::Into<i32>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            SubmsgBuilder((
                self.0,
                SubmsgSingleField1 {
                    i32_optional: value,
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
        fn g1_int32<'this>(&'this self) -> i32 {
            self.g1_int32_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_g1_int32<'this>(&'this self) -> bool {
            self.g1_int32_opt().is_some()
        }
        fn g1_string<'this>(&'this self) -> &'this str {
            self.g1_string_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_g1_string<'this>(&'this self) -> bool {
            self.g1_string_opt().is_some()
        }
        fn g2_f32<'this>(&'this self) -> f32 {
            self.g2_f32_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_g2_f32<'this>(&'this self) -> bool {
            self.g2_f32_opt().is_some()
        }
        fn g2_string<'this>(&'this self) -> &'this str {
            self.g2_string_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_g2_string<'this>(&'this self) -> bool {
            self.g2_string_opt().is_some()
        }
        type Field5MessageType<'this>: self::_puroro_root::oneofs2::_puroro_traits::SubmsgTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
        where
            Self: 'this;
        fn g2_submsg<'this>(&'this self) -> ::std::option::Option<Self::Field5MessageType<'this>> {
            self.g2_submsg_opt()
        }
        fn has_g2_submsg<'this>(&'this self) -> bool {
            self.g2_submsg_opt().is_some()
        }
        fn g3_int32<'this>(&'this self) -> i32 {
            self.g3_int32_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_g3_int32<'this>(&'this self) -> bool {
            self.g3_int32_opt().is_some()
        }
        fn group_one<'this>(
            &'this self,
        ) -> ::std::option::Option<
            super::_puroro_nested::msg::_puroro_oneofs::GroupOne<
                'this,
                ::puroro::internal::bool::False,
            >,
        > {
            ::std::option::Option::None
        }
        fn g1_int32_opt<'this>(&'this self) -> Option<i32> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupOne as E;
            self.group_one().and_then(|oneof| {
                #[allow(irrefutable_let_patterns)]
                if let E::G1Int32(v) = oneof {
                    ::std::option::Option::Some(v)
                } else {
                    ::std::option::Option::None
                }
            })
        }
        fn g1_string_opt<'this>(&'this self) -> Option<&'this str> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupOne as E;
            self.group_one().and_then(|oneof| {
                #[allow(irrefutable_let_patterns)]
                if let E::G1String(v) = oneof {
                    ::std::option::Option::Some(v)
                } else {
                    ::std::option::Option::None
                }
            })
        }
        fn group_two<'this>(
            &'this self,
        ) -> ::std::option::Option<
            super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<
                'this,
                ::puroro::internal::bool::False,
                Self,
            >,
        > {
            ::std::option::Option::None
        }
        fn g2_f32_opt<'this>(&'this self) -> Option<f32> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
            self.group_two().and_then(|oneof| {
                #[allow(irrefutable_let_patterns)]
                if let E::G2F32(v) = oneof {
                    ::std::option::Option::Some(v)
                } else {
                    ::std::option::Option::None
                }
            })
        }
        fn g2_string_opt<'this>(&'this self) -> Option<&'this str> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
            self.group_two().and_then(|oneof| {
                #[allow(irrefutable_let_patterns)]
                if let E::G2String(v) = oneof {
                    ::std::option::Option::Some(v)
                } else {
                    ::std::option::Option::None
                }
            })
        }
        fn g2_submsg_opt<'this>(
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
                    ::std::option::Option::Some(v)
                } else {
                    ::std::option::Option::None
                }
            })
        }
        fn group_three<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            ::std::option::Option::None
        }
        fn g3_int32_opt<'this>(&'this self) -> Option<i32> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupThree as E;
            self.group_three().and_then(|oneof| {
                #[allow(irrefutable_let_patterns)]
                if let E::G3Int32(v) = oneof {
                    ::std::option::Option::Some(v)
                } else {
                    ::std::option::Option::None
                }
            })
        }
    }

    macro_rules! msg_delegate {
        ($ty:ty) => {
            type Field5MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field5MessageType<'this>;
            fn group_one<'this>(
                &'this self,
            ) -> ::std::option::Option<
                super::_puroro_nested::msg::_puroro_oneofs::GroupOne<
                    'this,
                    ::puroro::internal::bool::False,
                >,
            > {
                (**self).group_one().map(|v| v.into())
            }
            fn group_two<'this>(
                &'this self,
            ) -> ::std::option::Option<
                super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<
                    'this,
                    ::puroro::internal::bool::False,
                    Self,
                >,
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
    pub trait SubmsgTrait {
        fn i32_optional<'this>(&'this self) -> i32 {
            self.i32_optional_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_i32_optional<'this>(&'this self) -> bool {
            self.i32_optional_opt().is_some()
        }
        fn i32_optional_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::None
        }
    }

    macro_rules! submsg_delegate {
        ($ty:ty) => {
            fn i32_optional_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).i32_optional_opt()
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

    impl<'bump, T> SubmsgTrait for ::puroro::bumpalo::boxed::Box<'bump, T>
    where
        T: SubmsgTrait,
    {
        submsg_delegate!(T);
    }

    impl<T> SubmsgTrait for ::puroro::BumpaloOwned<T>
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
            pub enum GroupOne<'msg, IsOwned>
            where
                IsOwned: ::puroro::internal::bool::BoolType,
            {
                G1Int32(i32),
                G1String(
                    <IsOwned as ::puroro::internal::bool::BoolType>::Choose<
                        ::std::string::String,
                        &'msg str,
                    >,
                ),
            }

            impl<'msg, IsOwned> GroupOne<'msg, IsOwned>
            where
                IsOwned: ::puroro::internal::bool::BoolType,
            {
                pub fn g1_int32(self) -> ::std::option::Option<i32> {
                    match self {
                        Self::G1Int32(v) => ::std::option::Option::Some(v),
                        #[allow(unreachable_patterns)]
                        _ => None,
                    }
                }
                pub fn g1_string(
                    self,
                ) -> ::std::option::Option<
                    <IsOwned as ::puroro::internal::bool::BoolType>::Choose<
                        ::std::string::String,
                        &'msg str,
                    >,
                > {
                    match self {
                        Self::G1String(v) => ::std::option::Option::Some(v),
                        #[allow(unreachable_patterns)]
                        _ => None,
                    }
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            pub enum GroupTwo<'msg, IsOwned, T>
            where
                IsOwned: ::puroro::internal::bool::BoolType,
                T: 'msg + ?Sized + self::_puroro_root::oneofs2::_puroro_traits::MsgTrait,
            {
                G2F32(f32),
                G2String(<IsOwned as ::puroro::internal::bool::BoolType>::Choose<::std::string::String, &'msg str>),
                G2Submsg(<IsOwned as ::puroro::internal::bool::BoolType>::Choose<::std::boxed::Box<self::_puroro_root::oneofs2::_puroro_simple_impl::Submsg>, <T as self::_puroro_root::oneofs2::_puroro_traits::MsgTrait>::Field5MessageType<'msg>>),
            }

            impl<'msg, IsOwned, T> GroupTwo<'msg, IsOwned, T>
            where
                IsOwned: ::puroro::internal::bool::BoolType,
                T: 'msg + ?Sized + self::_puroro_root::oneofs2::_puroro_traits::MsgTrait,
            {
                pub fn g2_f32(self) -> ::std::option::Option<f32> {
                    match self {
                        Self::G2F32(v) => ::std::option::Option::Some(v),
                        #[allow(unreachable_patterns)]
                        _ => None,
                    }
                }
                pub fn g2_string(
                    self,
                ) -> ::std::option::Option<
                    <IsOwned as ::puroro::internal::bool::BoolType>::Choose<
                        ::std::string::String,
                        &'msg str,
                    >,
                > {
                    match self {
                        Self::G2String(v) => ::std::option::Option::Some(v),
                        #[allow(unreachable_patterns)]
                        _ => None,
                    }
                }
                pub fn g2_submsg(self) -> ::std::option::Option<<IsOwned as ::puroro::internal::bool::BoolType>::Choose<::std::boxed::Box<self::_puroro_root::oneofs2::_puroro_simple_impl::Submsg>, <T as self::_puroro_root::oneofs2::_puroro_traits::MsgTrait>::Field5MessageType<'msg>>>{
                    match self {
                        Self::G2Submsg(v) => ::std::option::Option::Some(v),
                        #[allow(unreachable_patterns)]
                        _ => None,
                    }
                }
            }
            impl<'msg, IsOwned, T> ::std::convert::From<GroupTwo<'msg, IsOwned, T>>
                for GroupTwo<'msg, IsOwned, &'_ T>
            where
                IsOwned: ::puroro::internal::bool::BoolType,
                T: 'msg + self::_puroro_root::oneofs2::_puroro_traits::MsgTrait,
            {
                fn from(value: GroupTwo<'msg, IsOwned, T>) -> Self {
                    match value {
                        GroupTwo::G2F32(v) => GroupTwo::G2F32(v),
                        GroupTwo::G2String(v) => GroupTwo::G2String(v),
                        GroupTwo::G2Submsg(v) => GroupTwo::G2Submsg(v),
                    }
                }
            }
            impl<'msg, IsOwned, T> ::std::convert::From<GroupTwo<'msg, IsOwned, T>>
                for GroupTwo<'msg, IsOwned, &'_ mut T>
            where
                IsOwned: ::puroro::internal::bool::BoolType,
                T: 'msg + self::_puroro_root::oneofs2::_puroro_traits::MsgTrait,
            {
                fn from(value: GroupTwo<'msg, IsOwned, T>) -> Self {
                    match value {
                        GroupTwo::G2F32(v) => GroupTwo::G2F32(v),
                        GroupTwo::G2String(v) => GroupTwo::G2String(v),
                        GroupTwo::G2Submsg(v) => GroupTwo::G2Submsg(v),
                    }
                }
            }
            impl<'msg, IsOwned, T> ::std::convert::From<GroupTwo<'msg, IsOwned, T>>
                for GroupTwo<'msg, IsOwned, ::std::boxed::Box<T>>
            where
                IsOwned: ::puroro::internal::bool::BoolType,
                T: 'msg + self::_puroro_root::oneofs2::_puroro_traits::MsgTrait,
            {
                fn from(value: GroupTwo<'msg, IsOwned, T>) -> Self {
                    match value {
                        GroupTwo::G2F32(v) => GroupTwo::G2F32(v),
                        GroupTwo::G2String(v) => GroupTwo::G2String(v),
                        GroupTwo::G2Submsg(v) => GroupTwo::G2Submsg(v),
                    }
                }
            }
            impl<'msg, 'bump, IsOwned, T> ::std::convert::From<GroupTwo<'msg, IsOwned, T>>
                for GroupTwo<'msg, IsOwned, ::puroro::bumpalo::boxed::Box<'bump, T>>
            where
                IsOwned: ::puroro::internal::bool::BoolType,
                T: 'msg + self::_puroro_root::oneofs2::_puroro_traits::MsgTrait,
            {
                fn from(value: GroupTwo<'msg, IsOwned, T>) -> Self {
                    match value {
                        GroupTwo::G2F32(v) => GroupTwo::G2F32(v),
                        GroupTwo::G2String(v) => GroupTwo::G2String(v),
                        GroupTwo::G2Submsg(v) => GroupTwo::G2Submsg(v),
                    }
                }
            }
            impl<'msg, 'bump, IsOwned, T> ::std::convert::From<GroupTwo<'msg, IsOwned, T>>
                for GroupTwo<'msg, IsOwned, ::puroro::BumpaloOwned<T>>
            where
                IsOwned: ::puroro::internal::bool::BoolType,
                T: 'msg + self::_puroro_root::oneofs2::_puroro_traits::MsgTrait,
            {
                fn from(value: GroupTwo<'msg, IsOwned, T>) -> Self {
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

            impl GroupThree {
                pub fn g3_int32(self) -> ::std::option::Option<i32> {
                    match self {
                        Self::G3Int32(v) => ::std::option::Option::Some(v),
                        #[allow(unreachable_patterns)]
                        _ => None,
                    }
                }
            }
        }
        pub mod _puroro_bumpalo_oneofs {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }

            #[derive(::std::cmp::PartialEq, ::std::fmt::Debug)]
            pub enum GroupOne<'bump> {
                G1Int32(i32),
                G1String(::puroro::bumpalo::collections::String<'bump>),
            }

            impl<'bump> ::puroro::internal::impls::bumpalo::BumpaloClone<'bump> for GroupOne<'bump> {
                fn clone_in(&self, bump: &'bump ::puroro::bumpalo::Bump) -> Self {
                    match self {
                        Self::G1Int32(x) => Self::G1Int32(
                            ::puroro::internal::impls::bumpalo::BumpaloClone::clone_in(x, bump),
                        ),
                        Self::G1String(x) => Self::G1String(
                            ::puroro::internal::impls::bumpalo::BumpaloClone::clone_in(x, bump),
                        ),
                    }
                }
            }

            #[derive(::std::cmp::PartialEq, ::std::fmt::Debug)]
            pub enum GroupTwo<'bump> {
                G2F32(f32),
                G2String(::puroro::bumpalo::collections::String<'bump>),
                G2Submsg(
                    ::puroro::bumpalo::boxed::Box<
                        'bump,
                        self::_puroro_root::oneofs2::_puroro_impls::SubmsgBumpalo<'bump>,
                    >,
                ),
            }

            impl<'bump> ::puroro::internal::impls::bumpalo::BumpaloClone<'bump> for GroupTwo<'bump> {
                fn clone_in(&self, bump: &'bump ::puroro::bumpalo::Bump) -> Self {
                    match self {
                        Self::G2F32(x) => Self::G2F32(
                            ::puroro::internal::impls::bumpalo::BumpaloClone::clone_in(x, bump),
                        ),
                        Self::G2String(x) => Self::G2String(
                            ::puroro::internal::impls::bumpalo::BumpaloClone::clone_in(x, bump),
                        ),
                        Self::G2Submsg(x) => Self::G2Submsg(
                            ::puroro::internal::impls::bumpalo::BumpaloClone::clone_in(x, bump),
                        ),
                    }
                }
            }

            #[derive(::std::cmp::PartialEq, ::std::fmt::Debug)]
            pub enum GroupThree {
                G3Int32(i32),
            }

            impl<'bump> ::puroro::internal::impls::bumpalo::BumpaloClone<'bump> for GroupThree {
                fn clone_in(&self, bump: &'bump ::puroro::bumpalo::Bump) -> Self {
                    match self {
                        Self::G3Int32(x) => Self::G3Int32(
                            ::puroro::internal::impls::bumpalo::BumpaloClone::clone_in(x, bump),
                        ),
                    }
                }
            }
        }
    }
    pub mod submsg {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
    }
}
