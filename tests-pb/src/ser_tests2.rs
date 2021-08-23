// A generated source code by puroro library
// package ser_tests2

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub use _puroro_impls::Msg_Simple as Msg;
pub mod _puroro_impls {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }

    #[allow(non_camel_case_types)]
    #[derive(Clone, Default, PartialEq, Debug)]
    pub struct Msg_Simple {
        pub i32_optional: ::std::option::Option<i32>,
        pub i32_repeated: ::std::vec::Vec<i32>,
        pub float_optional: ::std::option::Option<f32>,
        pub float_repeated: ::std::vec::Vec<f32>,
        pub string_optional: ::std::option::Option<::std::string::String>,
        pub string_repeated: ::std::vec::Vec<::std::string::String>,
        pub submsg_optional: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::Submsg_Simple,
            >,
        >,
        pub submsg_repeated: ::std::vec::Vec<
            self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::Submsg_Simple,
        >,
        pub enum_optional: ::std::option::Option<self::_puroro_root::ser_tests2::Enum>,
        pub enum_repeated: ::std::vec::Vec<self::_puroro_root::ser_tests2::Enum>,
        pub very_large_field_number: ::std::option::Option<i32>,
    }
    impl ::puroro::Message for Msg_Simple {}

    impl super::_puroro_traits::MsgTrait for Msg_Simple {
        fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::clone::Clone::clone(&self.i32_optional)
        }
        type Field2RepeatedType<'this> = ::puroro_internal::impls::simple::VecWrapper<'this, i32>;

        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecWrapper::new(&self.i32_repeated)
        }
        fn float_optional<'this>(&'this self) -> ::std::option::Option<f32> {
            ::std::clone::Clone::clone(&self.float_optional)
        }
        type Field4RepeatedType<'this> = ::puroro_internal::impls::simple::VecWrapper<'this, f32>;

        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecWrapper::new(&self.float_repeated)
        }
        fn string_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            self.string_optional
                .as_ref()
                .map(|v| ::std::borrow::Cow::Borrowed(v.as_ref()))
        }
        type Field6RepeatedType<'this> =
            ::puroro_internal::impls::simple::VecCowWrapper<'this, str>;

        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecCowWrapper::new(&self.string_repeated)
        }
        type Field7MessageType<'this> =
            self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::Submsg_Simple;
        fn submsg_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field7MessageType<'this>>>
        {
            self.submsg_optional
                .as_ref()
                .map(|boxed| ::std::borrow::Cow::Borrowed(boxed.as_ref()))
        }
        type Field8MessageType<'this> =
            self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::Submsg_Simple;
        type Field8RepeatedType<'this> = ::puroro_internal::impls::simple::VecCowWrapper<
            'this,
            self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::Submsg_Simple,
        >;

        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecCowWrapper::new(&self.submsg_repeated)
        }
        fn enum_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::ser_tests2::Enum> {
            ::std::clone::Clone::clone(&self.enum_optional)
        }
        type Field10RepeatedType<'this> = ::puroro_internal::impls::simple::VecWrapper<
            'this,
            self::_puroro_root::ser_tests2::Enum,
        >;

        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecWrapper::new(&self.enum_repeated)
        }
        fn very_large_field_number<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::clone::Clone::clone(&self.very_large_field_number)
        }
    }

    impl ::puroro::DeserFromBytesIter for Msg_Simple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for Msg_Simple {
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
                ::puroro::tags::Optional, ::puroro::tags::Int32
            >::deser_field(&mut self.i32_optional, data),
            2 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Int32
            >::deser_field(&mut self.i32_repeated, data),
            3 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Float
            >::deser_field(&mut self.float_optional, data),
            4 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Float
            >::deser_field(&mut self.float_repeated, data),
            5 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.string_optional, data),
            6 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::String
            >::deser_field(&mut self.string_repeated, data),
            7 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Message<self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::Submsg_Simple>
            >::deser_field(&mut self.submsg_optional, data),
            8 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::Submsg_Simple>
            >::deser_field(&mut self.submsg_repeated, data),
            9 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Enum2<self::_puroro_root::ser_tests2::Enum>
            >::deser_field(&mut self.enum_optional, data),
            10 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Enum2<self::_puroro_root::ser_tests2::Enum>
            >::deser_field(&mut self.enum_repeated, data),
            536870911 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Int32
            >::deser_field(&mut self.very_large_field_number, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerToIoWrite for Msg_Simple {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >::ser_field(&self.i32_optional, 1, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Int32,
            >::ser_field(&self.i32_repeated, 2, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Float,
            >::ser_field(&self.float_optional, 3, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Float,
            >::ser_field(&self.float_repeated, 4, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.string_optional, 5, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::String,
            >::ser_field(&self.string_repeated, 6, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
            ::puroro::tags::Optional, ::puroro::tags::Message<self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::Submsg_Simple>
        >::ser_field(&self.submsg_optional, 7, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::Submsg_Simple>
        >::ser_field(&self.submsg_repeated, 8, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Enum2<self::_puroro_root::ser_tests2::Enum>,
            >::ser_field(&self.enum_optional, 9, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Enum2<self::_puroro_root::ser_tests2::Enum>,
            >::ser_field(&self.enum_repeated, 10, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >::ser_field(&self.very_large_field_number, 536870911, out)?;
            ::std::result::Result::Ok(())
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(Clone, Default, PartialEq, Debug)]
    pub struct Msg_Empty;

    impl ::puroro::Message for Msg_Empty {}

    impl super::_puroro_traits::MsgTrait for Msg_Empty {
        fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
            None
        }
        type Field2RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_optional<'this>(&'this self) -> ::std::option::Option<f32> {
            None
        }
        type Field4RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn string_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            None
        }
        type Field6RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<::std::borrow::Cow<'this, str>>;
        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> =
            self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::Submsg_Empty;
        fn submsg_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field7MessageType<'this>>>
        {
            None
        }
        type Field8MessageType<'this> =
            self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::Submsg_Empty;
        type Field8RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            ::std::borrow::Cow<'this, Self::Field8MessageType<'this>>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::ser_tests2::Enum> {
            None
        }
        type Field10RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::ser_tests2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn very_large_field_number<'this>(&'this self) -> ::std::option::Option<i32> {
            None
        }
    }

    impl ::puroro::SerToIoWrite for Msg_Empty {
        fn ser<W>(&self, _out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::std::result::Result::Ok(())
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(Clone, PartialEq, Debug)]
    struct Msg_Append1<Appendee: super::_puroro_traits::MsgTrait> {
        appendee: Appendee,
    }

    #[allow(non_camel_case_types)]
    #[derive(Clone, PartialEq, Debug)]
    struct Msg_Append2<Appendee: super::_puroro_traits::MsgTrait> {
        appendee: Appendee,
    }

    #[allow(non_camel_case_types)]
    #[derive(Clone, PartialEq, Debug)]
    struct Msg_Append3<Appendee: super::_puroro_traits::MsgTrait> {
        appendee: Appendee,
    }

    #[allow(non_camel_case_types)]
    #[derive(Clone, PartialEq, Debug)]
    struct Msg_Append4<Appendee: super::_puroro_traits::MsgTrait> {
        appendee: Appendee,
    }

    #[allow(non_camel_case_types)]
    #[derive(Clone, PartialEq, Debug)]
    struct Msg_Append5<Appendee: super::_puroro_traits::MsgTrait> {
        appendee: Appendee,
    }

    #[allow(non_camel_case_types)]
    #[derive(Clone, PartialEq, Debug)]
    struct Msg_Append6<Appendee: super::_puroro_traits::MsgTrait> {
        appendee: Appendee,
    }

    #[allow(non_camel_case_types)]
    #[derive(Clone, PartialEq, Debug)]
    struct Msg_Append7<Appendee: super::_puroro_traits::MsgTrait> {
        appendee: Appendee,
    }

    #[allow(non_camel_case_types)]
    #[derive(Clone, PartialEq, Debug)]
    struct Msg_Append8<Appendee: super::_puroro_traits::MsgTrait> {
        appendee: Appendee,
    }

    #[allow(non_camel_case_types)]
    #[derive(Clone, PartialEq, Debug)]
    struct Msg_Append9<Appendee: super::_puroro_traits::MsgTrait> {
        appendee: Appendee,
    }

    #[allow(non_camel_case_types)]
    #[derive(Clone, PartialEq, Debug)]
    struct Msg_Append10<Appendee: super::_puroro_traits::MsgTrait> {
        appendee: Appendee,
    }

    #[allow(non_camel_case_types)]
    #[derive(Clone, PartialEq, Debug)]
    struct Msg_Append536870911<Appendee: super::_puroro_traits::MsgTrait> {
        appendee: Appendee,
    }
}
pub use _puroro_traits::*;
pub mod _puroro_traits {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }

    pub trait MsgTrait: ::std::clone::Clone {
        fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32>;
        type Field2RepeatedType<'this>: ::puroro::RepeatedField<'this, i32>;

        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this>;
        fn float_optional<'this>(&'this self) -> ::std::option::Option<f32>;
        type Field4RepeatedType<'this>: ::puroro::RepeatedField<'this, f32>;

        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this>;
        fn string_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>>;
        type Field6RepeatedType<'this>: ::puroro::RepeatedField<
            'this,
            ::std::borrow::Cow<'this, str>,
        >;

        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this>;
        type Field7MessageType<'this>: 'this + self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_traits::SubmsgTrait;
        fn submsg_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field7MessageType<'this>>>;
        type Field8MessageType<'this>: 'this + self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_traits::SubmsgTrait;
        type Field8RepeatedType<'this>: ::puroro::RepeatedField<
            'this,
            ::std::borrow::Cow<'this, Self::Field8MessageType<'this>>,
        >;

        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this>;
        fn enum_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::ser_tests2::Enum>;
        type Field10RepeatedType<'this>: ::puroro::RepeatedField<
            'this,
            self::_puroro_root::ser_tests2::Enum,
        >;

        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this>;
        fn very_large_field_number<'this>(&'this self) -> ::std::option::Option<i32>;
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

        pub use _puroro_impls::Submsg_Simple as Submsg;
        pub mod _puroro_impls {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }

            #[allow(non_camel_case_types)]
            #[derive(Clone, Default, PartialEq, Debug)]
            pub struct Submsg_Simple {
                pub i32_optional: ::std::option::Option<i32>,
            }
            impl ::puroro::Message for Submsg_Simple {}

            impl super::_puroro_traits::SubmsgTrait for Submsg_Simple {
                fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
                    ::std::clone::Clone::clone(&self.i32_optional)
                }
            }

            impl ::puroro::DeserFromBytesIter for Submsg_Simple {
                fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
                }
            }

            impl ::puroro_internal::de::DeserFieldsFromBytesIter for Submsg_Simple {
                fn deser_field<I>(
                    &mut self,
                    field_number: i32,
                    data: ::puroro::types::FieldData<
                        &mut ::puroro_internal::de::from_iter::ScopedIter<I>,
                    >,
                ) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    match field_number {
                        1 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                            ::puroro::tags::Optional,
                            ::puroro::tags::Int32,
                        >::deser_field(&mut self.i32_optional, data),

                        _ => unimplemented!("TODO: This case should be handled properly..."),
                    }
                }
            }

            impl ::puroro::SerToIoWrite for Submsg_Simple {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int32,
                    >::ser_field(&self.i32_optional, 1, out)?;
                    ::std::result::Result::Ok(())
                }
            }

            #[allow(non_camel_case_types)]
            #[derive(Clone, Default, PartialEq, Debug)]
            pub struct Submsg_Empty;

            impl ::puroro::Message for Submsg_Empty {}

            impl super::_puroro_traits::SubmsgTrait for Submsg_Empty {
                fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
                    None
                }
            }

            impl ::puroro::SerToIoWrite for Submsg_Empty {
                fn ser<W>(&self, _out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    ::std::result::Result::Ok(())
                }
            }

            #[allow(non_camel_case_types)]
            #[derive(Clone, PartialEq, Debug)]
            struct Submsg_Append1<Appendee: super::_puroro_traits::SubmsgTrait> {
                appendee: Appendee,
            }
        }
        pub use _puroro_traits::*;
        pub mod _puroro_traits {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }

            pub trait SubmsgTrait: ::std::clone::Clone {
                fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32>;
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
