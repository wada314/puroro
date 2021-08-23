// A generated source code by puroro library
// package oneofs2

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub use _puroro_impls::Msg_Simple as Msg;
pub use _puroro_impls::Submsg_Simple as Submsg;
pub mod _puroro_impls {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }

    #[allow(non_camel_case_types)]
    #[derive(Clone, Default, PartialEq, Debug)]
    pub struct Msg_Simple {
        pub g1_int32: ::std::option::Option<i32>,
        pub g1_string: ::std::option::Option<::std::string::String>,
        pub g2_f32: ::std::option::Option<f32>,
        pub g2_string: ::std::option::Option<::std::string::String>,
        pub g2_submsg: ::std::option::Option<
            ::std::boxed::Box<self::_puroro_root::oneofs2::_puroro_impls::Submsg_Simple>,
        >,
        pub g3_int32: ::std::option::Option<i32>,
        pub group_one:
            ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne_Simple>,
        pub group_two:
            ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo_Simple>,
        pub group_three:
            ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree_Simple>,
    }
    impl ::puroro::Message for Msg_Simple {}

    impl super::_puroro_traits::MsgTrait for Msg_Simple {
        type Field5MessageType<'this> = self::_puroro_root::oneofs2::_puroro_impls::Submsg_Simple;
        fn group_one<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this>>
        {
            self.group_one
                .as_ref()
                .map(|o| ::std::convert::From::from(o))
        }
        fn group_two<'this>(
            &'this self,
        ) -> ::std::option::Option<
            super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Msg_Simple>,
        > {
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
                1 => {
                    use super::_puroro_nested::msg::_puroro_oneofs::GroupOne_Simple;
                    #[allow(unused)]
                    use std::option::Option::Some;
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
                    use std::option::Option::Some;
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
                    use std::option::Option::Some;
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
                    use std::option::Option::Some;
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
                    use std::option::Option::Some;
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
                            self::_puroro_root::oneofs2::_puroro_impls::Submsg_Simple,
                        >,
                    >::deser_field(field_value_mut_ref, data)
                }
                6 => {
                    use super::_puroro_nested::msg::_puroro_oneofs::GroupThree_Simple;
                    #[allow(unused)]
                    use std::option::Option::Some;
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

    impl ::puroro::SerToIoWrite for Msg_Simple {
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
                ::puroro::tags::Message<self::_puroro_root::oneofs2::_puroro_impls::Submsg_Simple>,
            >::ser_field(&self.g2_submsg, 5, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >::ser_field(&self.g3_int32, 6, out)?;
            ::std::result::Result::Ok(())
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(Clone, Default, PartialEq, Debug)]
    pub struct Msg_Empty;

    impl ::puroro::Message for Msg_Empty {}

    impl super::_puroro_traits::MsgTrait for Msg_Empty {
        type Field5MessageType<'this> = self::_puroro_root::oneofs2::_puroro_impls::Submsg_Simple;
        fn group_one<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this>>
        {
            None
        }
        fn group_two<'this>(
            &'this self,
        ) -> ::std::option::Option<
            super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Msg_Empty>,
        > {
            None
        }
        fn group_three<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
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
}
pub use _puroro_traits::*;
pub mod _puroro_traits {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }

    pub trait MsgTrait: ::std::clone::Clone {
        type Field5MessageType<'this>: 'this
            + self::_puroro_root::oneofs2::_puroro_traits::SubmsgTrait;
        fn group_one<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this>>;
        fn group_two<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>>;
        fn group_three<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree>;
    }
    pub trait SubmsgTrait: ::std::clone::Clone {
        fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32>;
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
            pub enum GroupOne<'msg> {
                G1Int32(i32),
                G1String(::std::borrow::Cow<'msg, str>),
            }

            #[allow(non_camel_case_types)]
            #[derive(Clone, PartialEq, Debug)]
            pub enum GroupOne_Simple {
                G1Int32(i32),
                G1String(::std::string::String),
            }

            impl<'msg> ::std::convert::From<&'msg GroupOne_Simple> for GroupOne<'msg> {
                fn from(from: &'msg GroupOne_Simple) -> Self {
                    match from {
                        GroupOne_Simple::G1Int32(v) => {
                            GroupOne::G1Int32(::std::clone::Clone::clone(&v))
                        }
                        GroupOne_Simple::G1String(v) => {
                            GroupOne::G1String(::std::borrow::Cow::Borrowed(v.as_ref()))
                        }
                    }
                }
            }
            pub enum GroupTwo<'msg, T: self::_puroro_root::oneofs2::_puroro_traits::MsgTrait> {
                G2F32(f32),
                G2String(::std::borrow::Cow<'msg, str>),
                G2Submsg(::std::borrow::Cow<'msg, <T as self::_puroro_root::oneofs2::_puroro_traits::MsgTrait>::Field5MessageType<'msg>>),
            }

            #[allow(non_camel_case_types)]
            #[derive(Clone, PartialEq, Debug)]
            pub enum GroupTwo_Simple {
                G2F32(f32),
                G2String(::std::string::String),
                G2Submsg(
                    ::std::boxed::Box<self::_puroro_root::oneofs2::_puroro_impls::Submsg_Simple>,
                ),
            }

            impl<'msg> ::std::convert::From<&'msg GroupTwo_Simple>
                for GroupTwo<'msg, self::_puroro_root::oneofs2::_puroro_impls::Msg_Simple>
            {
                fn from(from: &'msg GroupTwo_Simple) -> Self {
                    match from {
                        GroupTwo_Simple::G2F32(v) => {
                            GroupTwo::G2F32(::std::clone::Clone::clone(&v))
                        }
                        GroupTwo_Simple::G2String(v) => {
                            GroupTwo::G2String(::std::borrow::Cow::Borrowed(v.as_ref()))
                        }
                        GroupTwo_Simple::G2Submsg(v) => {
                            GroupTwo::G2Submsg(::std::borrow::Cow::Borrowed(v.as_ref()))
                        }
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
        }
    }
    pub mod submsg {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
    }
}
