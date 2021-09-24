// A generated source code by puroro library
// package ser_tests3

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
        pub i32_unlabeled: i32,
        pub i32_repeated: ::std::vec::Vec<i32>,
        pub float_unlabeled: f32,
        pub float_repeated: ::std::vec::Vec<f32>,
        pub string_unlabeled: ::std::string::String,
        pub string_repeated: ::std::vec::Vec<::std::string::String>,
        pub submsg_unlabeled: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_simple_impl::Submsg,
            >,
        >,
        pub submsg_repeated: ::std::vec::Vec<
            self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_simple_impl::Submsg,
        >,
        pub enum_unlabeled: self::_puroro_root::ser_tests3::Enum,
        pub enum_repeated: ::std::vec::Vec<self::_puroro_root::ser_tests3::Enum>,
        pub very_large_field_number: i32,
    }
    impl ::puroro::Message<Msg> for Msg {}

    impl super::_puroro_traits::MsgTrait for Msg {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Clone::clone(&self.i32_unlabeled)
        }
        type Field2RepeatedType<'this> = ::std::iter::Cloned<::std::slice::Iter<'this, i32>>;

        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            self.i32_repeated.iter().cloned()
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            Clone::clone(&self.float_unlabeled)
        }
        type Field4RepeatedType<'this> = ::std::iter::Cloned<::std::slice::Iter<'this, f32>>;

        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            self.float_repeated.iter().cloned()
        }
        type Field5StringType<'this>
        where
            Self: 'this,
        = &'this str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field5StringType<'this> {
            self.string_unlabeled.as_ref()
        }
        type Field6StringType<'this>
        where
            Self: 'this,
        = &'this str;
        type Field6RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            str,
            ::std::slice::Iter<'this, ::std::string::String>,
        >;

        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.string_repeated.iter())
        }
        type Field7MessageType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_simple_impl::Submsg;
        fn submsg_unlabeled<'this>(&'this self) -> Option<Self::Field7MessageType<'this>> {
            self.submsg_unlabeled.as_ref().map(|v| v.as_ref())
        }
        type Field8MessageType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_simple_impl::Submsg;
        type Field8RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_simple_impl::Submsg,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_simple_impl::Submsg,
            >,
        >;

        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.submsg_repeated.iter())
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::ser_tests3::Enum {
            Clone::clone(&self.enum_unlabeled)
        }
        type Field10RepeatedType<'this> =
            ::std::iter::Cloned<::std::slice::Iter<'this, self::_puroro_root::ser_tests3::Enum>>;

        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            self.enum_repeated.iter().cloned()
        }
        fn very_large_field_number<'this>(&'this self) -> i32 {
            Clone::clone(&self.very_large_field_number)
        }
    }

    impl ::puroro::MessageRepresentativeImpl for Msg {
        fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
            use ::puroro::once_cell::sync::Lazy;
            static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 11]> =
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
                                name: "i32_repeated",
                                number: 2,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "float_unlabeled",
                                number: 3,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "float_repeated",
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
                                name: "string_repeated",
                                number: 6,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "submsg_unlabeled",
                                number: 7,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "submsg_repeated",
                                number: 8,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "enum_unlabeled",
                                number: 9,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "enum_repeated",
                                number: 10,
                                lazy_containing_type: Lazy::new(|| {
                                    <Msg as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "very_large_field_number",
                                number: 536870911,
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

    impl ::puroro::DeserializableMessageFromBytesIterator for Msg {
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
                ::puroro::tags::Repeated, ::puroro::tags::Int32
            >::deser_field(&mut self.i32_repeated, data),
            3 => DeserFieldFromBytesIter::<
                ::puroro::tags::Unlabeled, ::puroro::tags::Float
            >::deser_field(&mut self.float_unlabeled, data),
            4 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Float
            >::deser_field(&mut self.float_repeated, data),
            5 => DeserFieldFromBytesIter::<
                ::puroro::tags::Unlabeled, ::puroro::tags::String
            >::deser_field(&mut self.string_unlabeled, data),
            6 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::String
            >::deser_field(&mut self.string_repeated, data),
            7 => DeserFieldFromBytesIter::<
                ::puroro::tags::Unlabeled, ::puroro::tags::Message<::std::boxed::Box<self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_simple_impl::Submsg>>
            >::deser_field(&mut self.submsg_unlabeled, data),
            8 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_simple_impl::Submsg>
            >::deser_field(&mut self.submsg_repeated, data),
            9 => DeserFieldFromBytesIter::<
                ::puroro::tags::Unlabeled, ::puroro::tags::Enum3<self::_puroro_root::ser_tests3::Enum>
            >::deser_field(&mut self.enum_unlabeled, data),
            10 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Enum3<self::_puroro_root::ser_tests3::Enum>
            >::deser_field(&mut self.enum_repeated, data),
            536870911 => DeserFieldFromBytesIter::<
                ::puroro::tags::Unlabeled, ::puroro::tags::Int32
            >::deser_field(&mut self.very_large_field_number, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for Msg {
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
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::Int32>::ser_field(
                &self.i32_repeated,
                2,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Unlabeled, ::puroro::tags::Float>::ser_field(
                &self.float_unlabeled,
                3,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::Float>::ser_field(
                &self.float_repeated,
                4,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Unlabeled, ::puroro::tags::String>::ser_field(
                &self.string_unlabeled,
                5,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::String>::ser_field(
                &self.string_repeated,
                6,
                out,
            )?;
            SerFieldToIoWrite::<
            ::puroro::tags::Unlabeled, ::puroro::tags::Message<::std::boxed::Box<self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_simple_impl::Submsg>>
        >::ser_field(&self.submsg_unlabeled, 7, out)?;
            SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_simple_impl::Submsg>
        >::ser_field(&self.submsg_repeated, 8, out)?;
            SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Enum3<self::_puroro_root::ser_tests3::Enum>,
            >::ser_field(&self.enum_unlabeled, 9, out)?;
            SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Enum3<self::_puroro_root::ser_tests3::Enum>,
            >::ser_field(&self.enum_repeated, 10, out)?;
            SerFieldToIoWrite::<::puroro::tags::Unlabeled, ::puroro::tags::Int32>::ser_field(
                &self.very_large_field_number,
                536870911,
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
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            Default::default()
        }
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5StringType<'this>
        where
            Self: 'this,
        = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field5StringType<'this> {
            ""
        }
        type Field6StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field6RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field8MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field8RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::ser_tests3::Enum {
            Default::default()
        }
        type Field10RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::ser_tests3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn very_large_field_number<'this>(&'this self) -> i32 {
            Default::default()
        }
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
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::Field2RepeatedType<'this>,
            <U as MsgTrait>::Field2RepeatedType<'this>,
        >;

        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as MsgTrait>::i32_repeated(&self.0),
                <U as MsgTrait>::i32_repeated(&self.1),
            )
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            let right = <U as MsgTrait>::float_unlabeled(&self.1);
            if right != ::std::default::Default::default() {
                right
            } else {
                <T as MsgTrait>::float_unlabeled(&self.0)
            }
        }
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::Field4RepeatedType<'this>,
            <U as MsgTrait>::Field4RepeatedType<'this>,
        >;

        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as MsgTrait>::float_repeated(&self.0),
                <U as MsgTrait>::float_repeated(&self.1),
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
            let right = <U as MsgTrait>::string_unlabeled(&self.1);
            if !right.is_empty() {
                ::puroro::Either::Right(right)
            } else {
                ::puroro::Either::Left(<T as MsgTrait>::string_unlabeled(&self.0))
            }
        }
        type Field6StringType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as MsgTrait>::Field6StringType<'this>,
            <U as MsgTrait>::Field6StringType<'this>,
        >;
        type Field6RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedLDField<
            <T as MsgTrait>::Field6RepeatedType<'this>,
            <U as MsgTrait>::Field6RepeatedType<'this>,
        >;

        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedLDField::new(
                <T as MsgTrait>::string_repeated(&self.0),
                <U as MsgTrait>::string_repeated(&self.1),
            )
        }
        type Field7MessageType<'this>
        where
            Self: 'this,
        = (
            ::std::option::Option<<T as MsgTrait>::Field7MessageType<'this>>,
            ::std::option::Option<<U as MsgTrait>::Field7MessageType<'this>>,
        );
        fn submsg_unlabeled<'this>(&'this self) -> Option<Self::Field7MessageType<'this>> {
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
        type Field8MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as MsgTrait>::Field8MessageType<'this>,
            <U as MsgTrait>::Field8MessageType<'this>,
        >;
        type Field8RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedMessageField<
            <T as MsgTrait>::Field8RepeatedType<'this>,
            <U as MsgTrait>::Field8RepeatedType<'this>,
        >;

        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
                <T as MsgTrait>::submsg_repeated(&self.0),
                <U as MsgTrait>::submsg_repeated(&self.1),
            )
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::ser_tests3::Enum {
            let right = <U as MsgTrait>::enum_unlabeled(&self.1);
            if right != ::std::default::Default::default() {
                right
            } else {
                <T as MsgTrait>::enum_unlabeled(&self.0)
            }
        }
        type Field10RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::Field10RepeatedType<'this>,
            <U as MsgTrait>::Field10RepeatedType<'this>,
        >;

        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as MsgTrait>::enum_repeated(&self.0),
                <U as MsgTrait>::enum_repeated(&self.1),
            )
        }
        fn very_large_field_number<'this>(&'this self) -> i32 {
            let right = <U as MsgTrait>::very_large_field_number(&self.1);
            if right != ::std::default::Default::default() {
                right
            } else {
                <T as MsgTrait>::very_large_field_number(&self.0)
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
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
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
        fn float_unlabeled<'this>(&'this self) -> f32 {
            self.as_ref().either(
                |t| <T as MsgTrait>::float_unlabeled(t),
                |u| <U as MsgTrait>::float_unlabeled(u),
            )
        }
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
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
        type Field6StringType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as MsgTrait>::Field6StringType<'this>,
            <U as MsgTrait>::Field6StringType<'this>,
        >;
        type Field6RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedLDField<
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
        type Field7MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as MsgTrait>::Field7MessageType<'this>,
            <U as MsgTrait>::Field7MessageType<'this>,
        >;
        fn submsg_unlabeled<'this>(&'this self) -> Option<Self::Field7MessageType<'this>> {
            self.as_ref().either(
                |t| <T as MsgTrait>::submsg_unlabeled(t).map(|t| ::puroro::Either::Left(t)),
                |u| <U as MsgTrait>::submsg_unlabeled(u).map(|u| ::puroro::Either::Right(u)),
            )
        }
        type Field8MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as MsgTrait>::Field8MessageType<'this>,
            <U as MsgTrait>::Field8MessageType<'this>,
        >;
        type Field8RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedMessageField<
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
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::ser_tests3::Enum {
            self.as_ref().either(
                |t| <T as MsgTrait>::enum_unlabeled(t),
                |u| <U as MsgTrait>::enum_unlabeled(u),
            )
        }
        type Field10RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
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
        fn very_large_field_number<'this>(&'this self) -> i32 {
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
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            self.as_ref()
                .map_or_else(::std::default::Default::default, |msg| msg.i32_unlabeled())
        }
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
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
        fn float_unlabeled<'this>(&'this self) -> f32 {
            self.as_ref()
                .map_or_else(::std::default::Default::default, |msg| {
                    msg.float_unlabeled()
                })
        }
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
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
        type Field5StringType<'this>
        where
            Self: 'this,
        = ::puroro::Either<T::Field5StringType<'this>, &'static str>;
        fn string_unlabeled<'this>(&'this self) -> Self::Field5StringType<'this> {
            self.as_ref().map_or(::puroro::Either::Right(""), |msg| {
                ::puroro::Either::Left(msg.string_unlabeled())
            })
        }
        type Field6StringType<'this>
        where
            Self: 'this,
        = T::Field6StringType<'this>;
        type Field6RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
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
        type Field7MessageType<'this>
        where
            Self: 'this,
        = T::Field7MessageType<'this>;
        fn submsg_unlabeled<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field7MessageType<'this>> {
            self.as_ref().and_then(|msg| msg.submsg_unlabeled())
        }
        type Field8MessageType<'this>
        where
            Self: 'this,
        = T::Field8MessageType<'this>;
        type Field8RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
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
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::ser_tests3::Enum {
            self.as_ref()
                .map_or_else(::std::default::Default::default, |msg| msg.enum_unlabeled())
        }
        type Field10RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
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
        fn very_large_field_number<'this>(&'this self) -> i32 {
            self.as_ref()
                .map_or_else(::std::default::Default::default, |msg| {
                    msg.very_large_field_number()
                })
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
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            Default::default()
        }
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5StringType<'this>
        where
            Self: 'this,
        = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field5StringType<'this> {
            ""
        }
        type Field6StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field6RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field8MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field8RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::ser_tests3::Enum {
            Default::default()
        }
        type Field10RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::ser_tests3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn very_large_field_number<'this>(&'this self) -> i32 {
            Default::default()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for MsgSingleField1 {
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
        pub i32_repeated: ::std::vec::Vec<i32>,
    }

    impl ::puroro::Message<super::Msg> for MsgSingleField2 {}

    impl super::_puroro_traits::MsgTrait for MsgSingleField2 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Default::default()
        }
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Cloned<::std::slice::Iter<'this, i32>>;

        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            self.i32_repeated.iter().cloned()
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            Default::default()
        }
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5StringType<'this>
        where
            Self: 'this,
        = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field5StringType<'this> {
            ""
        }
        type Field6StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field6RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field8MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field8RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::ser_tests3::Enum {
            Default::default()
        }
        type Field10RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::ser_tests3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn very_large_field_number<'this>(&'this self) -> i32 {
            Default::default()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for MsgSingleField2 {
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

    impl ::std::convert::From<::std::vec::Vec<i32>> for MsgSingleField2 {
        fn from(value: ::std::vec::Vec<i32>) -> Self {
            Self {
                i32_repeated: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField3 {
        pub float_unlabeled: f32,
    }

    impl ::puroro::Message<super::Msg> for MsgSingleField3 {}

    impl super::_puroro_traits::MsgTrait for MsgSingleField3 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Default::default()
        }
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            Clone::clone(&self.float_unlabeled)
        }
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5StringType<'this>
        where
            Self: 'this,
        = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field5StringType<'this> {
            ""
        }
        type Field6StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field6RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field8MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field8RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::ser_tests3::Enum {
            Default::default()
        }
        type Field10RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::ser_tests3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn very_large_field_number<'this>(&'this self) -> i32 {
            Default::default()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for MsgSingleField3 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Unlabeled, ::puroro::tags::Float>::ser_field(
                &self.float_unlabeled,
                3,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<f32> for MsgSingleField3 {
        fn from(value: f32) -> Self {
            Self {
                float_unlabeled: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField4 {
        pub float_repeated: ::std::vec::Vec<f32>,
    }

    impl ::puroro::Message<super::Msg> for MsgSingleField4 {}

    impl super::_puroro_traits::MsgTrait for MsgSingleField4 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Default::default()
        }
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            Default::default()
        }
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Cloned<::std::slice::Iter<'this, f32>>;

        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            self.float_repeated.iter().cloned()
        }
        type Field5StringType<'this>
        where
            Self: 'this,
        = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field5StringType<'this> {
            ""
        }
        type Field6StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field6RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field8MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field8RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::ser_tests3::Enum {
            Default::default()
        }
        type Field10RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::ser_tests3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn very_large_field_number<'this>(&'this self) -> i32 {
            Default::default()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for MsgSingleField4 {
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

    impl ::std::convert::From<::std::vec::Vec<f32>> for MsgSingleField4 {
        fn from(value: ::std::vec::Vec<f32>) -> Self {
            Self {
                float_repeated: value,
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
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            Default::default()
        }
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5StringType<'this>
        where
            Self: 'this,
        = &'this str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field5StringType<'this> {
            self.string_unlabeled.deref()
        }
        type Field6StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field6RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field8MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field8RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::ser_tests3::Enum {
            Default::default()
        }
        type Field10RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::ser_tests3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn very_large_field_number<'this>(&'this self) -> i32 {
            Default::default()
        }
    }

    impl<T> ::puroro::SerializableMessageToIoWrite for MsgSingleField5<T>
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
        T: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub string_repeated: ::std::vec::Vec<T>,
    }

    impl<T> ::puroro::Message<super::Msg> for MsgSingleField6<T> where
        T: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<T> super::_puroro_traits::MsgTrait for MsgSingleField6<T>
    where
        T: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Default::default()
        }
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            Default::default()
        }
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5StringType<'this>
        where
            Self: 'this,
        = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field5StringType<'this> {
            ""
        }
        type Field6StringType<'this>
        where
            Self: 'this,
        = &'this str;
        type Field6RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::single_field::DerefIter<::std::slice::Iter<'this, T>>;

        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::single_field::DerefIter::new(self.string_repeated.iter())
        }
        type Field7MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field8MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field8RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::ser_tests3::Enum {
            Default::default()
        }
        type Field10RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::ser_tests3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn very_large_field_number<'this>(&'this self) -> i32 {
            Default::default()
        }
    }

    impl<T> ::puroro::SerializableMessageToIoWrite for MsgSingleField6<T>
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
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::String>::ser_field::<
                T,
                _,
                _,
            >(&self.string_repeated, 6, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<T> ::std::convert::From<::std::vec::Vec<T>> for MsgSingleField6<T>
    where
        T: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ::std::vec::Vec<T>) -> Self {
            Self {
                string_repeated: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField7<T>
    where
        T: self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_traits::SubmsgTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub submsg_unlabeled: ::std::option::Option<T>,
    }

    impl<T> ::puroro::Message<super::Msg> for MsgSingleField7<T> where
        T: self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_traits::SubmsgTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<T> super::_puroro_traits::MsgTrait for MsgSingleField7<T>
    where
        T: self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_traits::SubmsgTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Default::default()
        }
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            Default::default()
        }
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5StringType<'this>
        where
            Self: 'this,
        = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field5StringType<'this> {
            ""
        }
        type Field6StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field6RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this>
        where
            Self: 'this,
        = &'this T;
        fn submsg_unlabeled<'this>(&'this self) -> Option<Self::Field7MessageType<'this>> {
            self.submsg_unlabeled.as_ref()
        }
        type Field8MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field8RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::ser_tests3::Enum {
            Default::default()
        }
        type Field10RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::ser_tests3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn very_large_field_number<'this>(&'this self) -> i32 {
            Default::default()
        }
    }

    impl<T> ::puroro::SerializableMessageToIoWrite for MsgSingleField7<T>
    where
        T: self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_traits::SubmsgTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        T: ::puroro::SerializableMessageToIoWrite,
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
            >(&self.submsg_unlabeled, 7, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<T> ::std::convert::From<::std::option::Option<T>> for MsgSingleField7<T>
    where
        T: self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_traits::SubmsgTrait
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

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField8<T>
    where
        T: self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_traits::SubmsgTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub submsg_repeated: ::std::vec::Vec<T>,
    }

    impl<T> ::puroro::Message<super::Msg> for MsgSingleField8<T> where
        T: self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_traits::SubmsgTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<T> super::_puroro_traits::MsgTrait for MsgSingleField8<T>
    where
        T: self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_traits::SubmsgTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Default::default()
        }
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            Default::default()
        }
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5StringType<'this>
        where
            Self: 'this,
        = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field5StringType<'this> {
            ""
        }
        type Field6StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field6RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field8MessageType<'this>
        where
            Self: 'this,
        = &'this T;
        type Field8RepeatedType<'this>
        where
            Self: 'this,
        = ::std::slice::Iter<'this, T>;

        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            self.submsg_repeated.iter()
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::ser_tests3::Enum {
            Default::default()
        }
        type Field10RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::ser_tests3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn very_large_field_number<'this>(&'this self) -> i32 {
            Default::default()
        }
    }

    impl<T> ::puroro::SerializableMessageToIoWrite for MsgSingleField8<T>
    where
        T: self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_traits::SubmsgTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        T: ::puroro::SerializableMessageToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::Message<T>>::ser_field::<
                T,
                _,
                _,
            >(&self.submsg_repeated, 8, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<T> ::std::convert::From<::std::vec::Vec<T>> for MsgSingleField8<T>
    where
        T: self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_traits::SubmsgTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ::std::vec::Vec<T>) -> Self {
            Self {
                submsg_repeated: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField9 {
        pub enum_unlabeled: self::_puroro_root::ser_tests3::Enum,
    }

    impl ::puroro::Message<super::Msg> for MsgSingleField9 {}

    impl super::_puroro_traits::MsgTrait for MsgSingleField9 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Default::default()
        }
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            Default::default()
        }
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5StringType<'this>
        where
            Self: 'this,
        = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field5StringType<'this> {
            ""
        }
        type Field6StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field6RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field8MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field8RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::ser_tests3::Enum {
            Clone::clone(&self.enum_unlabeled)
        }
        type Field10RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::ser_tests3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn very_large_field_number<'this>(&'this self) -> i32 {
            Default::default()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for MsgSingleField9 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Enum3<self::_puroro_root::ser_tests3::Enum>,
            >::ser_field(&self.enum_unlabeled, 9, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<self::_puroro_root::ser_tests3::Enum> for MsgSingleField9 {
        fn from(value: self::_puroro_root::ser_tests3::Enum) -> Self {
            Self {
                enum_unlabeled: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField10 {
        pub enum_repeated: ::std::vec::Vec<self::_puroro_root::ser_tests3::Enum>,
    }

    impl ::puroro::Message<super::Msg> for MsgSingleField10 {}

    impl super::_puroro_traits::MsgTrait for MsgSingleField10 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Default::default()
        }
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            Default::default()
        }
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5StringType<'this>
        where
            Self: 'this,
        = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field5StringType<'this> {
            ""
        }
        type Field6StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field6RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field8MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field8RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::ser_tests3::Enum {
            Default::default()
        }
        type Field10RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Cloned<::std::slice::Iter<'this, self::_puroro_root::ser_tests3::Enum>>;

        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            self.enum_repeated.iter().cloned()
        }
        fn very_large_field_number<'this>(&'this self) -> i32 {
            Default::default()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for MsgSingleField10 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Enum3<self::_puroro_root::ser_tests3::Enum>,
            >::ser_field(&self.enum_repeated, 10, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::vec::Vec<self::_puroro_root::ser_tests3::Enum>>
        for MsgSingleField10
    {
        fn from(value: ::std::vec::Vec<self::_puroro_root::ser_tests3::Enum>) -> Self {
            Self {
                enum_repeated: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField536870911 {
        pub very_large_field_number: i32,
    }

    impl ::puroro::Message<super::Msg> for MsgSingleField536870911 {}

    impl super::_puroro_traits::MsgTrait for MsgSingleField536870911 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Default::default()
        }
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            Default::default()
        }
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5StringType<'this>
        where
            Self: 'this,
        = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field5StringType<'this> {
            ""
        }
        type Field6StringType<'this>
        where
            Self: 'this,
        = &'static str;
        type Field6RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field8MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field8RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::ser_tests3::Enum {
            Default::default()
        }
        type Field10RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::ser_tests3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        fn very_large_field_number<'this>(&'this self) -> i32 {
            Clone::clone(&self.very_large_field_number)
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for MsgSingleField536870911 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Unlabeled, ::puroro::tags::Int32>::ser_field(
                &self.very_large_field_number,
                536870911,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<i32> for MsgSingleField536870911 {
        fn from(value: i32) -> Self {
            Self {
                very_large_field_number: value,
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
        type Field2RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field4RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field5StringType<'this> = ::std::string::String;
        fn string_unlabeled<'this>(&'this self) -> Self::Field5StringType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field6StringType<'this> = ::std::string::String;
        type Field6RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field7MessageType<'this> = ::std::boxed::Box<
            self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_simple_impl::Submsg,
        >;
        fn submsg_unlabeled<'this>(&'this self) -> Option<Self::Field7MessageType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field8MessageType<'this> = ::std::boxed::Box<
            self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_simple_impl::Submsg,
        >;
        type Field8RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::ser_tests3::Enum {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field10RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::ser_tests3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn very_large_field_number<'this>(&'this self) -> i32 {
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
        pub fn append_i32_repeated(
            self,
            value: ::std::vec::Vec<i32>,
        ) -> MsgBuilder<(T, MsgSingleField2)> {
            MsgBuilder((
                self.0,
                MsgSingleField2 {
                    i32_repeated: value,
                },
            ))
        }
        pub fn append_float_unlabeled(self, value: f32) -> MsgBuilder<(T, MsgSingleField3)> {
            MsgBuilder((
                self.0,
                MsgSingleField3 {
                    float_unlabeled: value,
                },
            ))
        }
        pub fn append_float_repeated(
            self,
            value: ::std::vec::Vec<f32>,
        ) -> MsgBuilder<(T, MsgSingleField4)> {
            MsgBuilder((
                self.0,
                MsgSingleField4 {
                    float_repeated: value,
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
        pub fn append_string_repeated<U>(
            self,
            value: ::std::vec::Vec<U>,
        ) -> MsgBuilder<(T, MsgSingleField6<U>)>
        where
            U: ::std::ops::Deref<Target = str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            MsgBuilder((
                self.0,
                MsgSingleField6 {
                    string_repeated: value,
                },
            ))
        }
        pub fn append_submsg_unlabeled<U>(
            self,
            value: ::std::option::Option<U>,
        ) -> MsgBuilder<(T, MsgSingleField7<U>)>
        where
            U: self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_traits::SubmsgTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            MsgBuilder((
                self.0,
                MsgSingleField7 {
                    submsg_unlabeled: value,
                },
            ))
        }
        pub fn append_submsg_repeated<U>(
            self,
            value: ::std::vec::Vec<U>,
        ) -> MsgBuilder<(T, MsgSingleField8<U>)>
        where
            U: self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_traits::SubmsgTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            MsgBuilder((
                self.0,
                MsgSingleField8 {
                    submsg_repeated: value,
                },
            ))
        }
        pub fn append_enum_unlabeled(
            self,
            value: self::_puroro_root::ser_tests3::Enum,
        ) -> MsgBuilder<(T, MsgSingleField9)> {
            MsgBuilder((
                self.0,
                MsgSingleField9 {
                    enum_unlabeled: value,
                },
            ))
        }
        pub fn append_enum_repeated(
            self,
            value: ::std::vec::Vec<self::_puroro_root::ser_tests3::Enum>,
        ) -> MsgBuilder<(T, MsgSingleField10)> {
            MsgBuilder((
                self.0,
                MsgSingleField10 {
                    enum_repeated: value,
                },
            ))
        }
        pub fn append_very_large_field_number(
            self,
            value: i32,
        ) -> MsgBuilder<(T, MsgSingleField536870911)> {
            MsgBuilder((
                self.0,
                MsgSingleField536870911 {
                    very_large_field_number: value,
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
}
pub use _puroro_traits::*;
pub mod _puroro_traits {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }

    pub trait MsgTrait {
        fn i32_unlabeled<'this>(&'this self) -> i32;
        type Field2RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = i32>
        where
            Self: 'this;
        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this>;
        fn float_unlabeled<'this>(&'this self) -> f32;
        type Field4RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = f32>
        where
            Self: 'this;
        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this>;
        type Field5StringType<'this>: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
        where
            Self: 'this;
        fn string_unlabeled<'this>(&'this self) -> Self::Field5StringType<'this>;
        type Field6StringType<'this>: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
        where
            Self: 'this;
        type Field6RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field6StringType<'this>>
        where
            Self: 'this;
        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this>;
        type Field7MessageType<'this>:
            self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_traits::SubmsgTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
            where Self: 'this;
        fn submsg_unlabeled<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field7MessageType<'this>> {
            ::std::default::Default::default()
        }
        type Field8MessageType<'this>:
            self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_traits::SubmsgTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
            where Self: 'this;
        type Field8RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field8MessageType<'this>>
        where
            Self: 'this;
        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this>;
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::ser_tests3::Enum;
        type Field10RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = self::_puroro_root::ser_tests3::Enum>
        where
            Self: 'this;
        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this>;
        fn very_large_field_number<'this>(&'this self) -> i32;
    }

    macro_rules! msg_delegate {
        ($ty:ty) => {
            fn i32_unlabeled<'this>(&'this self) -> i32 {
                (**self).i32_unlabeled()
            }
            type Field2RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field2RepeatedType<'this>;
            fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
                (**self).i32_repeated()
            }
            fn float_unlabeled<'this>(&'this self) -> f32 {
                (**self).float_unlabeled()
            }
            type Field4RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field4RepeatedType<'this>;
            fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
                (**self).float_repeated()
            }
            type Field5StringType<'this>
            where
                Self: 'this,
            = <$ty>::Field5StringType<'this>;
            fn string_unlabeled<'this>(&'this self) -> Self::Field5StringType<'this> {
                (**self).string_unlabeled()
            }
            type Field6StringType<'this>
            where
                Self: 'this,
            = <$ty>::Field6StringType<'this>;
            type Field6RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field6RepeatedType<'this>;
            fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
                (**self).string_repeated()
            }
            type Field7MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field7MessageType<'this>;
            fn submsg_unlabeled<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field7MessageType<'this>> {
                (**self).submsg_unlabeled()
            }
            type Field8MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field8MessageType<'this>;
            type Field8RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field8RepeatedType<'this>;
            fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
                (**self).submsg_repeated()
            }
            fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::ser_tests3::Enum {
                (**self).enum_unlabeled()
            }
            type Field10RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field10RepeatedType<'this>;
            fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
                (**self).enum_repeated()
            }
            fn very_large_field_number<'this>(&'this self) -> i32 {
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
}
#[derive(::std::fmt::Debug, ::std::clone::Clone, ::std::cmp::PartialEq)]
pub enum Enum {
    Zeroth,
    First,
    Tenth,
    _Unknown(i32),
}

impl ::puroro::Enum3 for Enum {}
impl ::std::convert::From<i32> for Enum {
    fn from(value: i32) -> Self {
        match value {
            0 => Enum::Zeroth,
            1 => Enum::First,
            10 => Enum::Tenth,
            _ => Enum::_Unknown(value),
        }
    }
}

impl ::std::convert::From<Enum> for i32 {
    fn from(value: Enum) -> i32 {
        match value {
            Enum::Zeroth => 0,
            Enum::First => 1,
            Enum::Tenth => 10,
            Enum::_Unknown(ivalue) => ivalue,
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

        pub use _puroro_simple_impl::Submsg;
        pub mod _puroro_simple_impl {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }
            #[derive(
                ::std::clone::Clone,
                ::std::default::Default,
                ::std::cmp::PartialEq,
                ::std::fmt::Debug,
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
                                        <Submsg as ::puroro::MessageRepresentativeImpl>::descriptor(
                                        )
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

            impl ::puroro::DeserializableMessageFromBytesIterator for Submsg {
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
                            ::puroro::tags::Unlabeled,
                            ::puroro::tags::Int32,
                        >::deser_field(&mut self.i32_unlabeled, data),

                        _ => unimplemented!("TODO: This case should be handled properly..."),
                    }
                }
            }

            impl ::puroro::SerializableMessageToIoWrite for Submsg {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::Int32
                >::ser_field(&self.i32_unlabeled, 1, out)?;

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

            impl ::puroro::SerializableMessageToIoWrite for SubmsgSingleField1 {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::Int32
                >::ser_field(&self.i32_unlabeled, 1, out)?;
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
                ::std::clone::Clone,
                ::std::default::Default,
                ::std::cmp::PartialEq,
                ::std::fmt::Debug,
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
                pub fn append_i32_unlabeled(
                    self,
                    value: i32,
                ) -> SubmsgBuilder<(T, SubmsgSingleField1)> {
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
            pub mod submsg {
                mod _puroro_root {
                    pub use super::super::super::_puroro_root::*;
                }
            }
        }
    }
}
