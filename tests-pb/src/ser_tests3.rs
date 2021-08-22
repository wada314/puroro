// A generated source code by puroro library
// package ser_tests3

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub struct Msg<ImplTag = ::puroro::tags::SimpleImpl>(
    <Self as ::puroro_internal::GetImplStruct>::Type,
)
where
    Self: ::puroro_internal::GetImplStruct;

impl<ImplTag> ::std::ops::Deref for Msg<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
{
    type Target = <Self as ::puroro_internal::GetImplStruct>::Type;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<ImplTag> ::std::ops::DerefMut for Msg<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<ImplTag> ::std::clone::Clone for Msg<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: ::std::clone::Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<ImplTag> ::std::default::Default for Msg<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: ::std::default::Default,
{
    fn default() -> Self {
        Self(::std::default::Default::default())
    }
}
impl<ImplTag> ::std::fmt::Debug for Msg<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: ::std::fmt::Debug,
{
    fn fmt(
        &self,
        formatter: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        ::std::fmt::Debug::fmt(&self.0, formatter)
    }
}
impl<ImplTag> ::std::cmp::PartialEq for Msg<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: ::std::cmp::PartialEq,
{
    fn eq(&self, rhs: &Self) -> bool {
        ::std::cmp::PartialEq::eq(&self.0, &rhs.0)
    }
}

impl<ImplTag> ::puroro::Message for Msg<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::Message,
{
}

impl<ImplTag> self::_puroro_traits::MsgTrait for Msg<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type:
        self::_puroro_root::ser_tests3::_puroro_traits::MsgTrait,
{
    fn i32_unlabeled<'this>(&'this self) -> i32 {
        <<Self as ::puroro_internal::GetImplStruct>::Type
            as self::_puroro_root::ser_tests3::_puroro_traits::MsgTrait
        >::i32_unlabeled(&self.0)
    }
    type Field2RepeatedType<'this> = <
        <Self as ::puroro_internal::GetImplStruct>::Type
        as self::_puroro_root::ser_tests3::_puroro_traits::MsgTrait
    >::Field2RepeatedType::<'this>;

    fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
        <<Self as ::puroro_internal::GetImplStruct>::Type
            as self::_puroro_root::ser_tests3::_puroro_traits::MsgTrait
        >::i32_repeated(&self.0)
    }
    fn float_unlabeled<'this>(&'this self) -> f32 {
        <<Self as ::puroro_internal::GetImplStruct>::Type
            as self::_puroro_root::ser_tests3::_puroro_traits::MsgTrait
        >::float_unlabeled(&self.0)
    }
    type Field4RepeatedType<'this> = <
        <Self as ::puroro_internal::GetImplStruct>::Type
        as self::_puroro_root::ser_tests3::_puroro_traits::MsgTrait
    >::Field4RepeatedType::<'this>;

    fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
        <<Self as ::puroro_internal::GetImplStruct>::Type
            as self::_puroro_root::ser_tests3::_puroro_traits::MsgTrait
        >::float_repeated(&self.0)
    }
    fn string_unlabeled<'this>(&'this self) -> ::std::borrow::Cow<'this, str> {
        <<Self as ::puroro_internal::GetImplStruct>::Type
            as self::_puroro_root::ser_tests3::_puroro_traits::MsgTrait
        >::string_unlabeled(&self.0)
    }
    type Field6RepeatedType<'this> = <
        <Self as ::puroro_internal::GetImplStruct>::Type
        as self::_puroro_root::ser_tests3::_puroro_traits::MsgTrait
    >::Field6RepeatedType::<'this>;

    fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
        <<Self as ::puroro_internal::GetImplStruct>::Type
            as self::_puroro_root::ser_tests3::_puroro_traits::MsgTrait
        >::string_repeated(&self.0)
    }
    type Field7MessageType<'this> = <
        <Self as ::puroro_internal::GetImplStruct>::Type
        as self::_puroro_root::ser_tests3::_puroro_traits::MsgTrait
    >::Field7MessageType::<'this>;
    fn submsg_unlabeled<'this>(
        &'this self,
    ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field7MessageType<'this>>> {
        <<Self as ::puroro_internal::GetImplStruct>::Type
            as self::_puroro_root::ser_tests3::_puroro_traits::MsgTrait
        >::submsg_unlabeled(&self.0)
    }
    type Field8MessageType<'this> = <
        <Self as ::puroro_internal::GetImplStruct>::Type
        as self::_puroro_root::ser_tests3::_puroro_traits::MsgTrait
    >::Field8MessageType::<'this>;
    type Field8RepeatedType<'this> = <
        <Self as ::puroro_internal::GetImplStruct>::Type
        as self::_puroro_root::ser_tests3::_puroro_traits::MsgTrait
    >::Field8RepeatedType::<'this>;

    fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
        <<Self as ::puroro_internal::GetImplStruct>::Type
            as self::_puroro_root::ser_tests3::_puroro_traits::MsgTrait
        >::submsg_repeated(&self.0)
    }
    fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::ser_tests3::Enum {
        <<Self as ::puroro_internal::GetImplStruct>::Type
            as self::_puroro_root::ser_tests3::_puroro_traits::MsgTrait
        >::enum_unlabeled(&self.0)
    }
    type Field10RepeatedType<'this> = <
        <Self as ::puroro_internal::GetImplStruct>::Type
        as self::_puroro_root::ser_tests3::_puroro_traits::MsgTrait
    >::Field10RepeatedType::<'this>;

    fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
        <<Self as ::puroro_internal::GetImplStruct>::Type
            as self::_puroro_root::ser_tests3::_puroro_traits::MsgTrait
        >::enum_repeated(&self.0)
    }
    fn very_large_field_number<'this>(&'this self) -> i32 {
        <<Self as ::puroro_internal::GetImplStruct>::Type
            as self::_puroro_root::ser_tests3::_puroro_traits::MsgTrait
        >::very_large_field_number(&self.0)
    }
}

impl<ImplTag> ::puroro::DeserFromBytesIter for Msg<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::DeserFromBytesIter,
{
    fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
    where
        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
    {
        <<Self as ::puroro_internal::GetImplStruct>::Type as ::puroro::DeserFromBytesIter>::deser(
            self, iter,
        )
    }
}
impl<ImplTag> ::puroro_internal::de::DeserFieldsFromBytesIter for Msg<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type:
        ::puroro_internal::de::DeserFieldsFromBytesIter,
{
    fn deser_field<I>(
        &mut self,
        field_number: i32,
        data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
    ) -> ::puroro::Result<()>
    where
        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
    {
        ::puroro_internal::de::DeserFieldsFromBytesIter::deser_field(
            &mut self.0,
            field_number,
            data,
        )
    }
}

impl<ImplTag> ::puroro::SerToIoWrite for Msg<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::SerToIoWrite,
{
    fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
    where
        W: ::std::io::Write,
    {
        ::puroro::SerToIoWrite::ser(&self.0, out)
    }
}

mod _puroro_impls {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }

    impl ::puroro_internal::GetImplStruct for super::Msg<::puroro::tags::SimpleImpl> {
        type Type = Msg_Simple;
    }

    #[allow(non_camel_case_types)]
    #[derive(Clone, Default, PartialEq, Debug)]
    pub struct Msg_Simple {
        pub i32_unlabeled: i32,
        pub i32_repeated: ::std::vec::Vec<i32>,
        pub float_unlabeled: f32,
        pub float_repeated: ::std::vec::Vec<f32>,
        pub string_unlabeled: ::std::string::String,
        pub string_repeated: ::std::vec::Vec<::std::string::String>,
        pub submsg_unlabeled: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::ser_tests3::_puroro_nested::msg::Submsg<
                    ::puroro::tags::SimpleImpl,
                >,
            >,
        >,
        pub submsg_repeated: ::std::vec::Vec<
            self::_puroro_root::ser_tests3::_puroro_nested::msg::Submsg<::puroro::tags::SimpleImpl>,
        >,
        pub enum_unlabeled: self::_puroro_root::ser_tests3::Enum,
        pub enum_repeated: ::std::vec::Vec<self::_puroro_root::ser_tests3::Enum>,
        pub very_large_field_number: i32,
    }
    impl ::puroro::Message for Msg_Simple {}

    impl super::_puroro_traits::MsgTrait for Msg_Simple {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            ::std::clone::Clone::clone(&self.i32_unlabeled)
        }
        type Field2RepeatedType<'this> = ::puroro_internal::impls::simple::VecWrapper<'this, i32>;

        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecWrapper::new(&self.i32_repeated)
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            ::std::clone::Clone::clone(&self.float_unlabeled)
        }
        type Field4RepeatedType<'this> = ::puroro_internal::impls::simple::VecWrapper<'this, f32>;

        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecWrapper::new(&self.float_repeated)
        }
        fn string_unlabeled<'this>(&'this self) -> ::std::borrow::Cow<'this, str> {
            ::std::borrow::Cow::Borrowed(&self.string_unlabeled)
        }
        type Field6RepeatedType<'this> =
            ::puroro_internal::impls::simple::VecCowWrapper<'this, str>;

        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecCowWrapper::new(&self.string_repeated)
        }
        type Field7MessageType<'this> =
            self::_puroro_root::ser_tests3::_puroro_nested::msg::Submsg<::puroro::tags::SimpleImpl>;
        fn submsg_unlabeled<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field7MessageType<'this>>>
        {
            self.submsg_unlabeled
                .as_ref()
                .map(|boxed| ::std::borrow::Cow::Borrowed(boxed.as_ref()))
        }
        type Field8MessageType<'this> =
            self::_puroro_root::ser_tests3::_puroro_nested::msg::Submsg<::puroro::tags::SimpleImpl>;
        type Field8RepeatedType<'this> = ::puroro_internal::impls::simple::VecCowWrapper<
            'this,
            self::_puroro_root::ser_tests3::_puroro_nested::msg::Submsg<::puroro::tags::SimpleImpl>,
        >;

        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecCowWrapper::new(&self.submsg_repeated)
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::ser_tests3::Enum {
            ::std::clone::Clone::clone(&self.enum_unlabeled)
        }
        type Field10RepeatedType<'this> = ::puroro_internal::impls::simple::VecWrapper<
            'this,
            self::_puroro_root::ser_tests3::Enum,
        >;

        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecWrapper::new(&self.enum_repeated)
        }
        fn very_large_field_number<'this>(&'this self) -> i32 {
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
                    ::puroro::tags::Unlabeled,
                    ::puroro::tags::Int32,
                >::deser_field(&mut self.i32_unlabeled, data),
                2 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated,
                    ::puroro::tags::Int32,
                >::deser_field(&mut self.i32_repeated, data),
                3 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled,
                    ::puroro::tags::Float,
                >::deser_field(&mut self.float_unlabeled, data),
                4 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated,
                    ::puroro::tags::Float,
                >::deser_field(&mut self.float_repeated, data),
                5 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled,
                    ::puroro::tags::String,
                >::deser_field(&mut self.string_unlabeled, data),
                6 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated,
                    ::puroro::tags::String,
                >::deser_field(&mut self.string_repeated, data),
                7 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled,
                    ::puroro::tags::Message<
                        self::_puroro_root::ser_tests3::_puroro_nested::msg::Submsg<
                            ::puroro::tags::SimpleImpl,
                        >,
                    >,
                >::deser_field(&mut self.submsg_unlabeled, data),
                8 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated,
                    ::puroro::tags::Message<
                        self::_puroro_root::ser_tests3::_puroro_nested::msg::Submsg<
                            ::puroro::tags::SimpleImpl,
                        >,
                    >,
                >::deser_field(&mut self.submsg_repeated, data),
                9 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled,
                    ::puroro::tags::Enum3<self::_puroro_root::ser_tests3::Enum>,
                >::deser_field(&mut self.enum_unlabeled, data),
                10 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated,
                    ::puroro::tags::Enum3<self::_puroro_root::ser_tests3::Enum>,
                >::deser_field(&mut self.enum_repeated, data),
                536870911 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled,
                    ::puroro::tags::Int32,
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
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Int32,
            >::ser_field(&self.i32_unlabeled, 1, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Int32,
            >::ser_field(&self.i32_repeated, 2, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Float,
            >::ser_field(&self.float_unlabeled, 3, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Float,
            >::ser_field(&self.float_repeated, 4, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::String,
            >::ser_field(&self.string_unlabeled, 5, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::String,
            >::ser_field(&self.string_repeated, 6, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Message<
                    self::_puroro_root::ser_tests3::_puroro_nested::msg::Submsg<
                        ::puroro::tags::SimpleImpl,
                    >,
                >,
            >::ser_field(&self.submsg_unlabeled, 7, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::ser_tests3::_puroro_nested::msg::Submsg<
                        ::puroro::tags::SimpleImpl,
                    >,
                >,
            >::ser_field(&self.submsg_repeated, 8, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Enum3<self::_puroro_root::ser_tests3::Enum>,
            >::ser_field(&self.enum_unlabeled, 9, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Enum3<self::_puroro_root::ser_tests3::Enum>,
            >::ser_field(&self.enum_repeated, 10, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Int32,
            >::ser_field(&self.very_large_field_number, 536870911, out)?;
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
        fn i32_unlabeled<'this>(&'this self) -> i32;
        type Field2RepeatedType<'this>: ::puroro::RepeatedField<'this, i32>;

        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this>;
        fn float_unlabeled<'this>(&'this self) -> f32;
        type Field4RepeatedType<'this>: ::puroro::RepeatedField<'this, f32>;

        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this>;
        fn string_unlabeled<'this>(&'this self) -> ::std::borrow::Cow<'this, str>;
        type Field6RepeatedType<'this>: ::puroro::RepeatedField<
            'this,
            ::std::borrow::Cow<'this, str>,
        >;

        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this>;
        type Field7MessageType<'this>: 'this + self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_traits::SubmsgTrait;
        fn submsg_unlabeled<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field7MessageType<'this>>>;
        type Field8MessageType<'this>: 'this + self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_traits::SubmsgTrait;
        type Field8RepeatedType<'this>: ::puroro::RepeatedField<
            'this,
            ::std::borrow::Cow<'this, Self::Field8MessageType<'this>>,
        >;

        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this>;
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::ser_tests3::Enum;
        type Field10RepeatedType<'this>: ::puroro::RepeatedField<
            'this,
            self::_puroro_root::ser_tests3::Enum,
        >;

        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this>;
        fn very_large_field_number<'this>(&'this self) -> i32;
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

        pub struct Submsg<ImplTag = ::puroro::tags::SimpleImpl>(
            <Self as ::puroro_internal::GetImplStruct>::Type,
        )
        where
            Self: ::puroro_internal::GetImplStruct;

        impl<ImplTag> ::std::ops::Deref for Submsg<ImplTag>
        where
            Self: ::puroro_internal::GetImplStruct,
        {
            type Target = <Self as ::puroro_internal::GetImplStruct>::Type;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl<ImplTag> ::std::ops::DerefMut for Submsg<ImplTag>
        where
            Self: ::puroro_internal::GetImplStruct,
        {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
        impl<ImplTag> ::std::clone::Clone for Submsg<ImplTag>
        where
            Self: ::puroro_internal::GetImplStruct,
            <Self as ::puroro_internal::GetImplStruct>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                Self(self.0.clone())
            }
        }
        impl<ImplTag> ::std::default::Default for Submsg<ImplTag>
        where
            Self: ::puroro_internal::GetImplStruct,
            <Self as ::puroro_internal::GetImplStruct>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                Self(::std::default::Default::default())
            }
        }
        impl<ImplTag> ::std::fmt::Debug for Submsg<ImplTag>
        where
            Self: ::puroro_internal::GetImplStruct,
            <Self as ::puroro_internal::GetImplStruct>::Type: ::std::fmt::Debug,
        {
            fn fmt(
                &self,
                formatter: &mut ::std::fmt::Formatter<'_>,
            ) -> ::std::result::Result<(), ::std::fmt::Error> {
                ::std::fmt::Debug::fmt(&self.0, formatter)
            }
        }
        impl<ImplTag> ::std::cmp::PartialEq for Submsg<ImplTag>
        where
            Self: ::puroro_internal::GetImplStruct,
            <Self as ::puroro_internal::GetImplStruct>::Type: ::std::cmp::PartialEq,
        {
            fn eq(&self, rhs: &Self) -> bool {
                ::std::cmp::PartialEq::eq(&self.0, &rhs.0)
            }
        }

        impl<ImplTag> ::puroro::Message for Submsg<ImplTag>
        where
            Self: ::puroro_internal::GetImplStruct,
            <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::Message,
        {
        }

        impl<ImplTag> self::_puroro_traits::SubmsgTrait for Submsg<ImplTag>
        where
            Self: ::puroro_internal::GetImplStruct,
            <Self as ::puroro_internal::GetImplStruct>::Type:
                self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_traits::SubmsgTrait,
        {
            fn i32_unlabeled<'this>(&'this self) -> i32 {
                <<Self as ::puroro_internal::GetImplStruct>::Type
                    as self::_puroro_root::ser_tests3::_puroro_nested::msg::_puroro_traits::SubmsgTrait
                >::i32_unlabeled(&self.0)
            }
        }

        impl<ImplTag> ::puroro::DeserFromBytesIter for Submsg<ImplTag>
        where
            Self: ::puroro_internal::GetImplStruct,
            <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::DeserFromBytesIter,
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                <<Self as ::puroro_internal::GetImplStruct>::Type as ::puroro::DeserFromBytesIter>::deser(self, iter)
            }
        }
        impl<ImplTag> ::puroro_internal::de::DeserFieldsFromBytesIter for Submsg<ImplTag>
        where
            Self: ::puroro_internal::GetImplStruct,
            <Self as ::puroro_internal::GetImplStruct>::Type:
                ::puroro_internal::de::DeserFieldsFromBytesIter,
        {
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
                ::puroro_internal::de::DeserFieldsFromBytesIter::deser_field(
                    &mut self.0,
                    field_number,
                    data,
                )
            }
        }

        impl<ImplTag> ::puroro::SerToIoWrite for Submsg<ImplTag>
        where
            Self: ::puroro_internal::GetImplStruct,
            <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::SerToIoWrite,
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write,
            {
                ::puroro::SerToIoWrite::ser(&self.0, out)
            }
        }

        mod _puroro_impls {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }

            impl ::puroro_internal::GetImplStruct for super::Submsg<::puroro::tags::SimpleImpl> {
                type Type = Submsg_Simple;
            }

            #[allow(non_camel_case_types)]
            #[derive(Clone, Default, PartialEq, Debug)]
            pub struct Submsg_Simple {
                pub i32_unlabeled: i32,
            }
            impl ::puroro::Message for Submsg_Simple {}

            impl super::_puroro_traits::SubmsgTrait for Submsg_Simple {
                fn i32_unlabeled<'this>(&'this self) -> i32 {
                    ::std::clone::Clone::clone(&self.i32_unlabeled)
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
                            ::puroro::tags::Unlabeled,
                            ::puroro::tags::Int32,
                        >::deser_field(&mut self.i32_unlabeled, data),

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
                        ::puroro::tags::Unlabeled,
                        ::puroro::tags::Int32,
                    >::ser_field(&self.i32_unlabeled, 1, out)?;
                    ::std::result::Result::Ok(())
                }
            }
        }
        pub use _puroro_traits::*;
        pub mod _puroro_traits {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }

            pub trait SubmsgTrait: ::std::clone::Clone {
                fn i32_unlabeled<'this>(&'this self) -> i32;
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
