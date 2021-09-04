// A generated source code by puroro library
// package oneofs2

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub use _puroro_impls::MsgSimple as Msg;
pub use _puroro_impls::SubmsgSimple as Submsg;
pub mod _puroro_impls {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct MsgSimple {
        pub g1_int32: ::std::option::Option<i32>,
        pub g1_string: ::std::option::Option<::std::string::String>,
        pub g2_f32: ::std::option::Option<f32>,
        pub g2_string: ::std::option::Option<::std::string::String>,
        pub g2_submsg: ::std::option::Option<
            ::std::boxed::Box<self::_puroro_root::oneofs2::_puroro_impls::SubmsgSimple>,
        >,
        pub g3_int32: ::std::option::Option<i32>,
        pub group_one:
            ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne_Simple>,
        pub group_two:
            ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo_Simple>,
        pub group_three:
            ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree_Simple>,
    }
    impl ::puroro::Message for MsgSimple {}

    impl super::_puroro_traits::MsgTrait for MsgSimple {
        type Field2ScalarGetterType<'this> = &'this str;
        type Field4ScalarGetterType<'this> = &'this str;
        type Field5MessageType<'this> = self::_puroro_root::oneofs2::_puroro_impls::SubmsgSimple;
        type Field5ScalarGetterType<'this> = &'this Self::Field5MessageType<'this>;
        fn group_one<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this, Self>>
        {
            self.group_one
                .as_ref()
                .map(|o| ::std::convert::From::from(o))
        }
        fn group_two<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>>
        {
            self.group_two
                .as_ref()
                .map(|o| ::std::convert::From::from(o))
        }
        fn group_three<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            self.group_three
                .as_ref()
                .map(|o| ::std::convert::From::from(o))
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
                1 => {
                    use super::_puroro_nested::msg::_puroro_oneofs::GroupOne_Simple;
                    #[allow(unused)]
                    use ::std::option::Option::Some;
                    if !matches!(&self.group_one, Some(GroupOne_Simple::G1Int32(_))) {
                        self.group_one =
                            Some(GroupOne_Simple::G1Int32(::std::default::Default::default()));
                    }
                    let field_value_mut_ref = match &mut self.group_one {
                        Some(GroupOne_Simple::G1Int32(v)) => v,
                        _ => unreachable!(),
                    };
                    ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                        ::puroro::tags::OneofItem,
                        ::puroro::tags::Int32,
                    >::deser_field(field_value_mut_ref, data)
                }
                2 => {
                    use super::_puroro_nested::msg::_puroro_oneofs::GroupOne_Simple;
                    #[allow(unused)]
                    use ::std::option::Option::Some;
                    if !matches!(&self.group_one, Some(GroupOne_Simple::G1String(_))) {
                        self.group_one =
                            Some(GroupOne_Simple::G1String(::std::default::Default::default()));
                    }
                    let field_value_mut_ref = match &mut self.group_one {
                        Some(GroupOne_Simple::G1String(v)) => v,
                        _ => unreachable!(),
                    };
                    ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                        ::puroro::tags::OneofItem,
                        ::puroro::tags::String,
                    >::deser_field(field_value_mut_ref, data)
                }
                3 => {
                    use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo_Simple;
                    #[allow(unused)]
                    use ::std::option::Option::Some;
                    if !matches!(&self.group_two, Some(GroupTwo_Simple::G2F32(_))) {
                        self.group_two =
                            Some(GroupTwo_Simple::G2F32(::std::default::Default::default()));
                    }
                    let field_value_mut_ref = match &mut self.group_two {
                        Some(GroupTwo_Simple::G2F32(v)) => v,
                        _ => unreachable!(),
                    };
                    ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                        ::puroro::tags::OneofItem,
                        ::puroro::tags::Float,
                    >::deser_field(field_value_mut_ref, data)
                }
                4 => {
                    use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo_Simple;
                    #[allow(unused)]
                    use ::std::option::Option::Some;
                    if !matches!(&self.group_two, Some(GroupTwo_Simple::G2String(_))) {
                        self.group_two =
                            Some(GroupTwo_Simple::G2String(::std::default::Default::default()));
                    }
                    let field_value_mut_ref = match &mut self.group_two {
                        Some(GroupTwo_Simple::G2String(v)) => v,
                        _ => unreachable!(),
                    };
                    ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                        ::puroro::tags::OneofItem,
                        ::puroro::tags::String,
                    >::deser_field(field_value_mut_ref, data)
                }
                5 => {
                    use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo_Simple;
                    #[allow(unused)]
                    use ::std::option::Option::Some;
                    if !matches!(&self.group_two, Some(GroupTwo_Simple::G2Submsg(_))) {
                        self.group_two =
                            Some(GroupTwo_Simple::G2Submsg(::std::default::Default::default()));
                    }
                    let field_value_mut_ref = match &mut self.group_two {
                        Some(GroupTwo_Simple::G2Submsg(v)) => v,
                        _ => unreachable!(),
                    };
                    ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                        ::puroro::tags::OneofItem,
                        ::puroro::tags::Message<
                            self::_puroro_root::oneofs2::_puroro_impls::SubmsgSimple,
                        >,
                    >::deser_field(field_value_mut_ref, data)
                }
                6 => {
                    use super::_puroro_nested::msg::_puroro_oneofs::GroupThree_Simple;
                    #[allow(unused)]
                    use ::std::option::Option::Some;
                    if !matches!(&self.group_three, Some(GroupThree_Simple::G3Int32(_))) {
                        self.group_three = Some(GroupThree_Simple::G3Int32(
                            ::std::default::Default::default(),
                        ));
                    }
                    let field_value_mut_ref = match &mut self.group_three {
                        Some(GroupThree_Simple::G3Int32(v)) => v,
                        _ => unreachable!(),
                    };
                    ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                        ::puroro::tags::OneofItem,
                        ::puroro::tags::Int32,
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
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >::ser_field(&self.g1_int32, 1, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.g1_string, 2, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Float,
            >::ser_field(&self.g2_f32, 3, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.g2_string, 4, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<self::_puroro_root::oneofs2::_puroro_impls::SubmsgSimple>,
            >::ser_field(&self.g2_submsg, 5, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >::ser_field(&self.g3_int32, 6, out)?;
            ::std::result::Result::Ok(())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct MsgEmpty;

    impl ::puroro::Message for MsgEmpty {}

    impl super::_puroro_traits::MsgTrait for MsgEmpty {
        type Field2ScalarGetterType<'this> = &'static str;
        type Field4ScalarGetterType<'this> = &'static str;
        type Field5MessageType<'this> = self::_puroro_root::oneofs2::_puroro_impls::SubmsgEmpty;
        type Field5ScalarGetterType<'this> = &'static Self::Field5MessageType<'this>;
        fn group_one<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this, Self>>
        {
            None
        }
        fn group_two<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>>
        {
            None
        }
        fn group_three<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            None
        }
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
        type Field2ScalarGetterType<'this> = ::puroro::Either<
            <T::Target as super::_puroro_traits::MsgTrait>::Field2ScalarGetterType<'this>,
            <U::Target as super::_puroro_traits::MsgTrait>::Field2ScalarGetterType<'this>,
        >;
        type Field4ScalarGetterType<'this> = ::puroro::Either<
            <T::Target as super::_puroro_traits::MsgTrait>::Field4ScalarGetterType<'this>,
            <U::Target as super::_puroro_traits::MsgTrait>::Field4ScalarGetterType<'this>,
        >;
        type Field5MessageType<'this> = ::puroro::EitherOrBoth<
            <T::Target as super::_puroro_traits::MsgTrait>::Field5ScalarGetterType<'this>,
            <U::Target as super::_puroro_traits::MsgTrait>::Field5ScalarGetterType<'this>,
        >;
        type Field5ScalarGetterType<'this> =
            ::puroro_internal::Derefable<Self::Field5MessageType<'this>>;
        fn group_one<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this, Self>>
        {
            todo!()
        }
        fn group_two<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>>
        {
            todo!()
        }
        fn group_three<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
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
        type Field2ScalarGetterType<'this> = ::puroro::Either<
            <T::Target as super::_puroro_traits::MsgTrait>::Field2ScalarGetterType<'this>,
            <U::Target as super::_puroro_traits::MsgTrait>::Field2ScalarGetterType<'this>,
        >;
        type Field4ScalarGetterType<'this> = ::puroro::Either<
            <T::Target as super::_puroro_traits::MsgTrait>::Field4ScalarGetterType<'this>,
            <U::Target as super::_puroro_traits::MsgTrait>::Field4ScalarGetterType<'this>,
        >;
        type Field5MessageType<'this> = ::puroro::Either<
            <T::Target as super::_puroro_traits::MsgTrait>::Field5ScalarGetterType<'this>,
            <U::Target as super::_puroro_traits::MsgTrait>::Field5ScalarGetterType<'this>,
        >;
        type Field5ScalarGetterType<'this> =
            ::puroro_internal::Derefable<Self::Field5MessageType<'this>>;
        fn group_one<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this, Self>>
        {
            self.as_ref()
                .either(
                    |t| t.group_one().map(|t| ::puroro::Either::Left(t)),
                    |u| u.group_one().map(|u| ::puroro::Either::Right(u)),
                )
                .map(|tu| tu.into())
        }
        fn group_two<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>>
        {
            self.as_ref()
                .either(
                    |t| t.group_two().map(|t| ::puroro::Either::Left(t)),
                    |u| u.group_two().map(|u| ::puroro::Either::Right(u)),
                )
                .map(|tu| tu.into())
        }
        fn group_three<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            self.as_ref()
                .either(
                    |t| t.group_three().map(|t| ::puroro::Either::Left(t)),
                    |u| u.group_three().map(|u| ::puroro::Either::Right(u)),
                )
                .map(|tu| tu.into())
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField1 {
        g1_int32: ::std::option::Option<i32>,
    }

    impl ::puroro::Message for MsgSimpleField1 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField1 {
        type Field2ScalarGetterType<'this> = &'static str;
        type Field4ScalarGetterType<'this> = &'static str;
        type Field5MessageType<'this> = self::_puroro_root::oneofs2::_puroro_impls::SubmsgEmpty;
        type Field5ScalarGetterType<'this> = &'static Self::Field5MessageType<'this>;
        fn group_one<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this, Self>>
        {
            todo!()
        }
        fn group_two<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>>
        {
            todo!()
        }
        fn group_three<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            todo!()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField2 {
        g1_string: ::std::option::Option<::std::string::String>,
    }

    impl ::puroro::Message for MsgSimpleField2 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField2 {
        type Field2ScalarGetterType<'this> = &'this str;
        type Field4ScalarGetterType<'this> = &'static str;
        type Field5MessageType<'this> = self::_puroro_root::oneofs2::_puroro_impls::SubmsgEmpty;
        type Field5ScalarGetterType<'this> = &'static Self::Field5MessageType<'this>;
        fn group_one<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this, Self>>
        {
            todo!()
        }
        fn group_two<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>>
        {
            todo!()
        }
        fn group_three<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            todo!()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField3 {
        g2_f32: ::std::option::Option<f32>,
    }

    impl ::puroro::Message for MsgSimpleField3 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField3 {
        type Field2ScalarGetterType<'this> = &'static str;
        type Field4ScalarGetterType<'this> = &'static str;
        type Field5MessageType<'this> = self::_puroro_root::oneofs2::_puroro_impls::SubmsgEmpty;
        type Field5ScalarGetterType<'this> = &'static Self::Field5MessageType<'this>;
        fn group_one<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this, Self>>
        {
            todo!()
        }
        fn group_two<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>>
        {
            todo!()
        }
        fn group_three<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            todo!()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField4 {
        g2_string: ::std::option::Option<::std::string::String>,
    }

    impl ::puroro::Message for MsgSimpleField4 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField4 {
        type Field2ScalarGetterType<'this> = &'static str;
        type Field4ScalarGetterType<'this> = &'this str;
        type Field5MessageType<'this> = self::_puroro_root::oneofs2::_puroro_impls::SubmsgEmpty;
        type Field5ScalarGetterType<'this> = &'static Self::Field5MessageType<'this>;
        fn group_one<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this, Self>>
        {
            todo!()
        }
        fn group_two<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>>
        {
            todo!()
        }
        fn group_three<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            todo!()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField5 {
        g2_submsg: ::std::option::Option<
            ::std::boxed::Box<self::_puroro_root::oneofs2::_puroro_impls::SubmsgSimple>,
        >,
    }

    impl ::puroro::Message for MsgSimpleField5 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField5 {
        type Field2ScalarGetterType<'this> = &'static str;
        type Field4ScalarGetterType<'this> = &'static str;
        type Field5MessageType<'this> = self::_puroro_root::oneofs2::_puroro_impls::SubmsgSimple;
        type Field5ScalarGetterType<'this> = &'this Self::Field5MessageType<'this>;
        fn group_one<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this, Self>>
        {
            todo!()
        }
        fn group_two<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>>
        {
            todo!()
        }
        fn group_three<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            todo!()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField6 {
        g3_int32: ::std::option::Option<i32>,
    }

    impl ::puroro::Message for MsgSimpleField6 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField6 {
        type Field2ScalarGetterType<'this> = &'static str;
        type Field4ScalarGetterType<'this> = &'static str;
        type Field5MessageType<'this> = self::_puroro_root::oneofs2::_puroro_impls::SubmsgEmpty;
        type Field5ScalarGetterType<'this> = &'static Self::Field5MessageType<'this>;
        fn group_one<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this, Self>>
        {
            todo!()
        }
        fn group_two<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>>
        {
            todo!()
        }
        fn group_three<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            todo!()
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct SubmsgSimple {
        pub i32_optional: ::std::option::Option<i32>,
    }
    impl ::puroro::Message for SubmsgSimple {}

    impl super::_puroro_traits::SubmsgTrait for SubmsgSimple {
        fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::clone::Clone::clone(&self.i32_optional)
        }
    }

    impl ::puroro::DeserFromBytesIter for SubmsgSimple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for SubmsgSimple {
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
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >::ser_field(&self.i32_optional, 1, out)?;
            ::std::result::Result::Ok(())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct SubmsgEmpty;

    impl ::puroro::Message for SubmsgEmpty {}

    impl super::_puroro_traits::SubmsgTrait for SubmsgEmpty {}

    impl ::puroro::SerToIoWrite for SubmsgEmpty {
        fn ser<W>(&self, _out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::std::result::Result::Ok(())
        }
    }
    impl<T, U> super::_puroro_traits::SubmsgTrait for ::puroro::EitherOrBoth<T, U>
    where
        T: ::std::ops::Deref,
        U: ::std::ops::Deref,
        T::Target: super::_puroro_traits::SubmsgTrait,
        U::Target: super::_puroro_traits::SubmsgTrait,
    {
        fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
            let left_opt = self
                .as_ref()
                .left()
                .and_then(|t| <T::Target as super::_puroro_traits::SubmsgTrait>::i32_optional(t));
            left_opt.or_else(|| {
                self.as_ref().right().and_then(|u| {
                    <U::Target as super::_puroro_traits::SubmsgTrait>::i32_optional(u)
                })
            })
        }
    }
    impl<T, U> super::_puroro_traits::SubmsgTrait for ::puroro::Either<T, U>
    where
        T: ::std::ops::Deref,
        U: ::std::ops::Deref,
        T::Target: super::_puroro_traits::SubmsgTrait,
        U::Target: super::_puroro_traits::SubmsgTrait,
    {
        fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().either(
                |t| <T::Target as super::_puroro_traits::SubmsgTrait>::i32_optional(t),
                |u| <U::Target as super::_puroro_traits::SubmsgTrait>::i32_optional(u),
            )
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct SubmsgSimpleField1 {
        i32_optional: ::std::option::Option<i32>,
    }

    impl ::puroro::Message for SubmsgSimpleField1 {}

    impl super::_puroro_traits::SubmsgTrait for SubmsgSimpleField1 {
        fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::clone::Clone::clone(&self.i32_optional)
        }
    }
}
pub use _puroro_traits::*;
pub mod _puroro_traits {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }

    pub trait MsgTrait {
        type Field2ScalarGetterType<'this>: ::std::ops::Deref<Target = str>;
        type Field4ScalarGetterType<'this>: ::std::ops::Deref<Target = str>;
        type Field5MessageType<'this>: self::_puroro_root::oneofs2::_puroro_traits::SubmsgTrait;
        type Field5ScalarGetterType<'this>: ::std::ops::Deref<
            Target = Self::Field5MessageType<'this>,
        >;
        fn group_one<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this, Self>>
        {
            ::std::option::Option::None
        }
        fn group_two<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>>
        {
            ::std::option::Option::None
        }
        fn group_three<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            ::std::option::Option::None
        }
    }
    pub trait SubmsgTrait {
        fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
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

        pub use _puroro_oneofs::*;
        pub mod _puroro_oneofs {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }
            pub enum GroupOne<
                'msg,
                T: ?Sized + self::_puroro_root::oneofs2::_puroro_traits::MsgTrait,
            > {
                G1Int32(i32),
                G1String(<T as self::_puroro_root::oneofs2::_puroro_traits::MsgTrait>::Field2ScalarGetterType<'msg>),
            }

            #[allow(non_camel_case_types)]
            #[derive(Clone, PartialEq, Debug)]
            pub enum GroupOne_Simple {
                G1Int32(i32),
                G1String(::std::string::String),
            }

            impl<'msg> ::std::convert::From<&'msg GroupOne_Simple>
                for GroupOne<'msg, self::_puroro_root::oneofs2::_puroro_impls::MsgSimple>
            {
                fn from(from: &'msg GroupOne_Simple) -> Self {
                    match from {
                        GroupOne_Simple::G1Int32(v) => {
                            GroupOne::G1Int32(::std::clone::Clone::clone(&v))
                        }
                        GroupOne_Simple::G1String(v) => GroupOne::G1String(v.as_ref()),
                    }
                }
            }
            impl<'msg, T, U>
                ::std::convert::From<
                    ::puroro::Either<GroupOne<'msg, T::Target>, GroupOne<'msg, U::Target>>,
                > for GroupOne<'msg, ::puroro::Either<T, U>>
            where
                T: ::std::ops::Deref,
                U: ::std::ops::Deref,
                T::Target: self::_puroro_root::oneofs2::_puroro_traits::MsgTrait,
                U::Target: self::_puroro_root::oneofs2::_puroro_traits::MsgTrait,
            {
                fn from(
                    value: ::puroro::Either<GroupOne<'msg, T::Target>, GroupOne<'msg, U::Target>>,
                ) -> Self {
                    match value {
                        ::puroro::Either::Left(GroupOne::G1Int32(v)) => GroupOne::G1Int32(v),
                        ::puroro::Either::Right(GroupOne::G1Int32(v)) => GroupOne::G1Int32(v),
                        ::puroro::Either::Left(GroupOne::G1String(v)) => {
                            GroupOne::G1String(::puroro::Either::Left(v))
                        }
                        ::puroro::Either::Right(GroupOne::G1String(v)) => {
                            GroupOne::G1String(::puroro::Either::Right(v))
                        }
                    }
                }
            }
            pub enum GroupTwo<
                'msg,
                T: ?Sized + self::_puroro_root::oneofs2::_puroro_traits::MsgTrait,
            > {
                G2F32(f32),
                G2String(<T as self::_puroro_root::oneofs2::_puroro_traits::MsgTrait>::Field4ScalarGetterType<'msg>),
                G2Submsg(<T as self::_puroro_root::oneofs2::_puroro_traits::MsgTrait>::Field5ScalarGetterType<'msg>),
            }

            #[allow(non_camel_case_types)]
            #[derive(Clone, PartialEq, Debug)]
            pub enum GroupTwo_Simple {
                G2F32(f32),
                G2String(::std::string::String),
                G2Submsg(
                    ::std::boxed::Box<self::_puroro_root::oneofs2::_puroro_impls::SubmsgSimple>,
                ),
            }

            impl<'msg> ::std::convert::From<&'msg GroupTwo_Simple>
                for GroupTwo<'msg, self::_puroro_root::oneofs2::_puroro_impls::MsgSimple>
            {
                fn from(from: &'msg GroupTwo_Simple) -> Self {
                    match from {
                        GroupTwo_Simple::G2F32(v) => {
                            GroupTwo::G2F32(::std::clone::Clone::clone(&v))
                        }
                        GroupTwo_Simple::G2String(v) => GroupTwo::G2String(v.as_ref()),
                        GroupTwo_Simple::G2Submsg(v) => GroupTwo::G2Submsg(v.as_ref()),
                    }
                }
            }
            impl<'msg, T, U>
                ::std::convert::From<
                    ::puroro::Either<GroupTwo<'msg, T::Target>, GroupTwo<'msg, U::Target>>,
                > for GroupTwo<'msg, ::puroro::Either<T, U>>
            where
                T: ::std::ops::Deref,
                U: ::std::ops::Deref,
                T::Target: self::_puroro_root::oneofs2::_puroro_traits::MsgTrait,
                U::Target: self::_puroro_root::oneofs2::_puroro_traits::MsgTrait,
            {
                fn from(
                    value: ::puroro::Either<GroupTwo<'msg, T::Target>, GroupTwo<'msg, U::Target>>,
                ) -> Self {
                    match value {
                        ::puroro::Either::Left(GroupTwo::G2F32(v)) => GroupTwo::G2F32(v),
                        ::puroro::Either::Right(GroupTwo::G2F32(v)) => GroupTwo::G2F32(v),
                        ::puroro::Either::Left(GroupTwo::G2String(v)) => {
                            GroupTwo::G2String(::puroro::Either::Left(v))
                        }
                        ::puroro::Either::Right(GroupTwo::G2String(v)) => {
                            GroupTwo::G2String(::puroro::Either::Right(v))
                        }
                        ::puroro::Either::Left(GroupTwo::G2Submsg(v)) => GroupTwo::G2Submsg(
                            ::puroro_internal::Derefable::new(::puroro::Either::Left(v)),
                        ),
                        ::puroro::Either::Right(GroupTwo::G2Submsg(v)) => GroupTwo::G2Submsg(
                            ::puroro_internal::Derefable::new(::puroro::Either::Right(v)),
                        ),
                    }
                }
            }
            pub enum GroupThree {
                G3Int32(i32),
            }

            #[allow(non_camel_case_types)]
            #[derive(Clone, PartialEq, Debug)]
            pub enum GroupThree_Simple {
                G3Int32(i32),
            }

            impl<'msg> ::std::convert::From<&'msg GroupThree_Simple> for GroupThree {
                fn from(from: &'msg GroupThree_Simple) -> Self {
                    match from {
                        GroupThree_Simple::G3Int32(v) => {
                            GroupThree::G3Int32(::std::clone::Clone::clone(&v))
                        }
                    }
                }
            }
            impl ::std::convert::From<::puroro::Either<GroupThree, GroupThree>> for GroupThree {
                fn from(value: ::puroro::Either<GroupThree, GroupThree>) -> Self {
                    value.into_inner()
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
