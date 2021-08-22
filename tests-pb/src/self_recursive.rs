// A generated source code by puroro library
// package self_recursive

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
        self::_puroro_root::self_recursive::_puroro_traits::MsgTrait,
{
    type Field1MessageType<'this> = <
        <Self as ::puroro_internal::GetImplStruct>::Type
        as self::_puroro_root::self_recursive::_puroro_traits::MsgTrait
    >::Field1MessageType::<'this>;
    fn recursive_unlabeled<'this>(
        &'this self,
    ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field1MessageType<'this>>> {
        <<Self as ::puroro_internal::GetImplStruct>::Type
            as self::_puroro_root::self_recursive::_puroro_traits::MsgTrait
        >::recursive_unlabeled(&self.0)
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
        pub recursive_unlabeled: ::std::option::Option<
            ::std::boxed::Box<self::_puroro_root::self_recursive::Msg<::puroro::tags::SimpleImpl>>,
        >,
    }
    impl ::puroro::Message for Msg_Simple {}

    impl super::_puroro_traits::MsgTrait for Msg_Simple {
        type Field1MessageType<'this> =
            self::_puroro_root::self_recursive::Msg<::puroro::tags::SimpleImpl>;
        fn recursive_unlabeled<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field1MessageType<'this>>>
        {
            self.recursive_unlabeled
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
                    ::puroro::tags::Message<
                        self::_puroro_root::self_recursive::Msg<::puroro::tags::SimpleImpl>,
                    >,
                >::deser_field(&mut self.recursive_unlabeled, data),

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
                ::puroro::tags::Message<
                    self::_puroro_root::self_recursive::Msg<::puroro::tags::SimpleImpl>,
                >,
            >::ser_field(&self.recursive_unlabeled, 1, out)?;
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
        type Field1MessageType<'this>: 'this
            + self::_puroro_root::self_recursive::_puroro_traits::MsgTrait;
        fn recursive_unlabeled<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field1MessageType<'this>>>;
    }
}
pub use _puroro_nested::*;
pub mod _puroro_nested {
    pub mod msg {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
    }
}
