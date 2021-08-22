// A generated source code by puroro library
// package proto3_defaults

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
        self::_puroro_root::proto3_defaults::_puroro_traits::MsgTrait,
{
    fn i32_unlabeled<'this>(&'this self) -> i32 {
        <<Self as ::puroro_internal::GetImplStruct>::Type
            as self::_puroro_root::proto3_defaults::_puroro_traits::MsgTrait
        >::i32_unlabeled(&self.0)
    }
    fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
        <<Self as ::puroro_internal::GetImplStruct>::Type
            as self::_puroro_root::proto3_defaults::_puroro_traits::MsgTrait
        >::i32_optional(&self.0)
    }
    type Field3RepeatedType<'this> = <
        <Self as ::puroro_internal::GetImplStruct>::Type
        as self::_puroro_root::proto3_defaults::_puroro_traits::MsgTrait
    >::Field3RepeatedType::<'this>;

    fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
        <<Self as ::puroro_internal::GetImplStruct>::Type
            as self::_puroro_root::proto3_defaults::_puroro_traits::MsgTrait
        >::i32_repeated(&self.0)
    }
    fn f32_unlabeled<'this>(&'this self) -> f32 {
        <<Self as ::puroro_internal::GetImplStruct>::Type
            as self::_puroro_root::proto3_defaults::_puroro_traits::MsgTrait
        >::f32_unlabeled(&self.0)
    }
    fn string_unlabeled<'this>(&'this self) -> ::std::borrow::Cow<'this, str> {
        <<Self as ::puroro_internal::GetImplStruct>::Type
            as self::_puroro_root::proto3_defaults::_puroro_traits::MsgTrait
        >::string_unlabeled(&self.0)
    }
    type Field6MessageType<'this> = <
        <Self as ::puroro_internal::GetImplStruct>::Type
        as self::_puroro_root::proto3_defaults::_puroro_traits::MsgTrait
    >::Field6MessageType::<'this>;
    fn submsg_unlabeled<'this>(
        &'this self,
    ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field6MessageType<'this>>> {
        <<Self as ::puroro_internal::GetImplStruct>::Type
            as self::_puroro_root::proto3_defaults::_puroro_traits::MsgTrait
        >::submsg_unlabeled(&self.0)
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
        self::_puroro_root::proto3_defaults::_puroro_traits::SubmsgTrait,
{
    fn i32_unlabeled<'this>(&'this self) -> i32 {
        <<Self as ::puroro_internal::GetImplStruct>::Type
            as self::_puroro_root::proto3_defaults::_puroro_traits::SubmsgTrait
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
        <<Self as ::puroro_internal::GetImplStruct>::Type as ::puroro::DeserFromBytesIter>::deser(
            self, iter,
        )
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

    impl ::puroro_internal::GetImplStruct for super::Msg<::puroro::tags::SimpleImpl> {
        type Type = Msg_Simple;
    }

    #[allow(non_camel_case_types)]
    #[derive(Clone, Default, PartialEq, Debug)]
    pub struct Msg_Simple {
        pub i32_unlabeled: i32,
        pub i32_optional: ::std::option::Option<i32>,
        pub i32_repeated: ::std::vec::Vec<i32>,
        pub f32_unlabeled: f32,
        pub string_unlabeled: ::std::string::String,
        pub submsg_unlabeled: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::proto3_defaults::Submsg<::puroro::tags::SimpleImpl>,
            >,
        >,
    }
    impl ::puroro::Message for Msg_Simple {}

    impl super::_puroro_traits::MsgTrait for Msg_Simple {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            ::std::clone::Clone::clone(&self.i32_unlabeled)
        }
        fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::clone::Clone::clone(&self.i32_optional)
        }
        type Field3RepeatedType<'this> = ::puroro_internal::impls::simple::VecWrapper<'this, i32>;

        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecWrapper::new(&self.i32_repeated)
        }
        fn f32_unlabeled<'this>(&'this self) -> f32 {
            ::std::clone::Clone::clone(&self.f32_unlabeled)
        }
        fn string_unlabeled<'this>(&'this self) -> ::std::borrow::Cow<'this, str> {
            ::std::borrow::Cow::Borrowed(&self.string_unlabeled)
        }
        type Field6MessageType<'this> =
            self::_puroro_root::proto3_defaults::Submsg<::puroro::tags::SimpleImpl>;
        fn submsg_unlabeled<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field6MessageType<'this>>>
        {
            self.submsg_unlabeled
                .as_ref()
                .map(|boxed| ::std::borrow::Cow::Borrowed(boxed.as_ref()))
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
                    ::puroro::tags::Optional,
                    ::puroro::tags::Int32,
                >::deser_field(&mut self.i32_optional, data),
                3 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated,
                    ::puroro::tags::Int32,
                >::deser_field(&mut self.i32_repeated, data),
                4 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled,
                    ::puroro::tags::Float,
                >::deser_field(&mut self.f32_unlabeled, data),
                5 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled,
                    ::puroro::tags::String,
                >::deser_field(&mut self.string_unlabeled, data),
                6 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled,
                    ::puroro::tags::Message<
                        self::_puroro_root::proto3_defaults::Submsg<::puroro::tags::SimpleImpl>,
                    >,
                >::deser_field(&mut self.submsg_unlabeled, data),

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
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >::ser_field(&self.i32_optional, 2, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Int32,
            >::ser_field(&self.i32_repeated, 3, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Float,
            >::ser_field(&self.f32_unlabeled, 4, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::String,
            >::ser_field(&self.string_unlabeled, 5, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Message<
                    self::_puroro_root::proto3_defaults::Submsg<::puroro::tags::SimpleImpl>,
                >,
            >::ser_field(&self.submsg_unlabeled, 6, out)?;
            ::std::result::Result::Ok(())
        }
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

    pub trait MsgTrait: ::std::clone::Clone {
        fn i32_unlabeled<'this>(&'this self) -> i32;
        fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32>;
        type Field3RepeatedType<'this>: ::puroro::RepeatedField<'this, i32>;

        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this>;
        fn f32_unlabeled<'this>(&'this self) -> f32;
        fn string_unlabeled<'this>(&'this self) -> ::std::borrow::Cow<'this, str>;
        type Field6MessageType<'this>: 'this + self::_puroro_root::proto3_defaults::_puroro_traits::SubmsgTrait;
        fn submsg_unlabeled<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field6MessageType<'this>>>;
    }
    pub trait SubmsgTrait: ::std::clone::Clone {
        fn i32_unlabeled<'this>(&'this self) -> i32;
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
