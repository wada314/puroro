// A generated source code by puroro library
// package official_samples

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub struct Test1<ImplTag = ::puroro::tags::SimpleImpl>(
    <Self as ::puroro_internal::GetImplStruct>::Type,
)
where
    Self: ::puroro_internal::GetImplStruct;

impl<ImplTag> ::std::ops::Deref for Test1<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
{
    type Target = <Self as ::puroro_internal::GetImplStruct>::Type;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<ImplTag> ::std::ops::DerefMut for Test1<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<ImplTag> ::std::clone::Clone for Test1<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: ::std::clone::Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<ImplTag> ::std::default::Default for Test1<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: ::std::default::Default,
{
    fn default() -> Self {
        Self(::std::default::Default::default())
    }
}
impl<ImplTag> ::std::fmt::Debug for Test1<ImplTag>
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
impl<ImplTag> ::std::cmp::PartialEq for Test1<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: ::std::cmp::PartialEq,
{
    fn eq(&self, rhs: &Self) -> bool {
        ::std::cmp::PartialEq::eq(&self.0, &rhs.0)
    }
}

impl<ImplTag> ::puroro::Message for Test1<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::Message,
{
}

impl<ImplTag> self::_puroro_traits::Test1Trait for Test1<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type:
        self::_puroro_root::official_samples::_puroro_traits::Test1Trait,
{
    fn a<'this>(&'this self) -> ::std::option::Option<i32> {
        <<Self as ::puroro_internal::GetImplStruct>::Type
            as self::_puroro_root::official_samples::_puroro_traits::Test1Trait
        >::a(&self.0)
    }
}

impl<ImplTag> ::puroro::DeserFromBytesIter for Test1<ImplTag>
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
impl<ImplTag> ::puroro_internal::de::DeserFieldsFromBytesIter for Test1<ImplTag>
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

impl<ImplTag> ::puroro::SerToIoWrite for Test1<ImplTag>
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

pub struct Test2<ImplTag = ::puroro::tags::SimpleImpl>(
    <Self as ::puroro_internal::GetImplStruct>::Type,
)
where
    Self: ::puroro_internal::GetImplStruct;

impl<ImplTag> ::std::ops::Deref for Test2<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
{
    type Target = <Self as ::puroro_internal::GetImplStruct>::Type;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<ImplTag> ::std::ops::DerefMut for Test2<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<ImplTag> ::std::clone::Clone for Test2<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: ::std::clone::Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<ImplTag> ::std::default::Default for Test2<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: ::std::default::Default,
{
    fn default() -> Self {
        Self(::std::default::Default::default())
    }
}
impl<ImplTag> ::std::fmt::Debug for Test2<ImplTag>
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
impl<ImplTag> ::std::cmp::PartialEq for Test2<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: ::std::cmp::PartialEq,
{
    fn eq(&self, rhs: &Self) -> bool {
        ::std::cmp::PartialEq::eq(&self.0, &rhs.0)
    }
}

impl<ImplTag> ::puroro::Message for Test2<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::Message,
{
}

impl<ImplTag> self::_puroro_traits::Test2Trait for Test2<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type:
        self::_puroro_root::official_samples::_puroro_traits::Test2Trait,
{
    fn b<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
        <<Self as ::puroro_internal::GetImplStruct>::Type
            as self::_puroro_root::official_samples::_puroro_traits::Test2Trait
        >::b(&self.0)
    }
}

impl<ImplTag> ::puroro::DeserFromBytesIter for Test2<ImplTag>
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
impl<ImplTag> ::puroro_internal::de::DeserFieldsFromBytesIter for Test2<ImplTag>
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

impl<ImplTag> ::puroro::SerToIoWrite for Test2<ImplTag>
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

pub struct Test3<ImplTag = ::puroro::tags::SimpleImpl>(
    <Self as ::puroro_internal::GetImplStruct>::Type,
)
where
    Self: ::puroro_internal::GetImplStruct;

impl<ImplTag> ::std::ops::Deref for Test3<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
{
    type Target = <Self as ::puroro_internal::GetImplStruct>::Type;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<ImplTag> ::std::ops::DerefMut for Test3<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<ImplTag> ::std::clone::Clone for Test3<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: ::std::clone::Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<ImplTag> ::std::default::Default for Test3<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: ::std::default::Default,
{
    fn default() -> Self {
        Self(::std::default::Default::default())
    }
}
impl<ImplTag> ::std::fmt::Debug for Test3<ImplTag>
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
impl<ImplTag> ::std::cmp::PartialEq for Test3<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: ::std::cmp::PartialEq,
{
    fn eq(&self, rhs: &Self) -> bool {
        ::std::cmp::PartialEq::eq(&self.0, &rhs.0)
    }
}

impl<ImplTag> ::puroro::Message for Test3<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::Message,
{
}

impl<ImplTag> self::_puroro_traits::Test3Trait for Test3<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type:
        self::_puroro_root::official_samples::_puroro_traits::Test3Trait,
{
    type Field3MessageType<'this> = <
        <Self as ::puroro_internal::GetImplStruct>::Type
        as self::_puroro_root::official_samples::_puroro_traits::Test3Trait
    >::Field3MessageType::<'this>;
    fn c<'this>(
        &'this self,
    ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field3MessageType<'this>>> {
        <<Self as ::puroro_internal::GetImplStruct>::Type
            as self::_puroro_root::official_samples::_puroro_traits::Test3Trait
        >::c(&self.0)
    }
}

impl<ImplTag> ::puroro::DeserFromBytesIter for Test3<ImplTag>
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
impl<ImplTag> ::puroro_internal::de::DeserFieldsFromBytesIter for Test3<ImplTag>
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

impl<ImplTag> ::puroro::SerToIoWrite for Test3<ImplTag>
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

pub struct Test4<ImplTag = ::puroro::tags::SimpleImpl>(
    <Self as ::puroro_internal::GetImplStruct>::Type,
)
where
    Self: ::puroro_internal::GetImplStruct;

impl<ImplTag> ::std::ops::Deref for Test4<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
{
    type Target = <Self as ::puroro_internal::GetImplStruct>::Type;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<ImplTag> ::std::ops::DerefMut for Test4<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<ImplTag> ::std::clone::Clone for Test4<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: ::std::clone::Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<ImplTag> ::std::default::Default for Test4<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: ::std::default::Default,
{
    fn default() -> Self {
        Self(::std::default::Default::default())
    }
}
impl<ImplTag> ::std::fmt::Debug for Test4<ImplTag>
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
impl<ImplTag> ::std::cmp::PartialEq for Test4<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: ::std::cmp::PartialEq,
{
    fn eq(&self, rhs: &Self) -> bool {
        ::std::cmp::PartialEq::eq(&self.0, &rhs.0)
    }
}

impl<ImplTag> ::puroro::Message for Test4<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::Message,
{
}

impl<ImplTag> self::_puroro_traits::Test4Trait for Test4<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type:
        self::_puroro_root::official_samples::_puroro_traits::Test4Trait,
{
    type Field4RepeatedType<'this> = <
        <Self as ::puroro_internal::GetImplStruct>::Type
        as self::_puroro_root::official_samples::_puroro_traits::Test4Trait
    >::Field4RepeatedType::<'this>;

    fn d<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
        <<Self as ::puroro_internal::GetImplStruct>::Type
            as self::_puroro_root::official_samples::_puroro_traits::Test4Trait
        >::d(&self.0)
    }
}

impl<ImplTag> ::puroro::DeserFromBytesIter for Test4<ImplTag>
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
impl<ImplTag> ::puroro_internal::de::DeserFieldsFromBytesIter for Test4<ImplTag>
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

impl<ImplTag> ::puroro::SerToIoWrite for Test4<ImplTag>
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

    impl ::puroro_internal::GetImplStruct for super::Test1<::puroro::tags::SimpleImpl> {
        type Type = Test1_Simple;
    }

    #[allow(non_camel_case_types)]
    #[derive(Clone, Default, PartialEq, Debug)]
    pub struct Test1_Simple {
        pub a: ::std::option::Option<i32>,
    }
    impl ::puroro::Message for Test1_Simple {}

    impl super::_puroro_traits::Test1Trait for Test1_Simple {
        fn a<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::clone::Clone::clone(&self.a)
        }
    }

    impl ::puroro::DeserFromBytesIter for Test1_Simple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for Test1_Simple {
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
                >::deser_field(&mut self.a, data),

                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl ::puroro::SerToIoWrite for Test1_Simple {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >::ser_field(&self.a, 1, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::puroro_internal::GetImplStruct for super::Test2<::puroro::tags::SimpleImpl> {
        type Type = Test2_Simple;
    }

    #[allow(non_camel_case_types)]
    #[derive(Clone, Default, PartialEq, Debug)]
    pub struct Test2_Simple {
        pub b: ::std::option::Option<::std::string::String>,
    }
    impl ::puroro::Message for Test2_Simple {}

    impl super::_puroro_traits::Test2Trait for Test2_Simple {
        fn b<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            self.b
                .as_ref()
                .map(|v| ::std::borrow::Cow::Borrowed(v.as_ref()))
        }
    }

    impl ::puroro::DeserFromBytesIter for Test2_Simple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for Test2_Simple {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
                2 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::String,
                >::deser_field(&mut self.b, data),

                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl ::puroro::SerToIoWrite for Test2_Simple {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.b, 2, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::puroro_internal::GetImplStruct for super::Test3<::puroro::tags::SimpleImpl> {
        type Type = Test3_Simple;
    }

    #[allow(non_camel_case_types)]
    #[derive(Clone, Default, PartialEq, Debug)]
    pub struct Test3_Simple {
        pub c: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::official_samples::Test1<::puroro::tags::SimpleImpl>,
            >,
        >,
    }
    impl ::puroro::Message for Test3_Simple {}

    impl super::_puroro_traits::Test3Trait for Test3_Simple {
        type Field3MessageType<'this> =
            self::_puroro_root::official_samples::Test1<::puroro::tags::SimpleImpl>;
        fn c<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field3MessageType<'this>>>
        {
            self.c
                .as_ref()
                .map(|boxed| ::std::borrow::Cow::Borrowed(boxed.as_ref()))
        }
    }

    impl ::puroro::DeserFromBytesIter for Test3_Simple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for Test3_Simple {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
                3 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::Message<
                        self::_puroro_root::official_samples::Test1<::puroro::tags::SimpleImpl>,
                    >,
                >::deser_field(&mut self.c, data),

                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl ::puroro::SerToIoWrite for Test3_Simple {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    self::_puroro_root::official_samples::Test1<::puroro::tags::SimpleImpl>,
                >,
            >::ser_field(&self.c, 3, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::puroro_internal::GetImplStruct for super::Test4<::puroro::tags::SimpleImpl> {
        type Type = Test4_Simple;
    }

    #[allow(non_camel_case_types)]
    #[derive(Clone, Default, PartialEq, Debug)]
    pub struct Test4_Simple {
        pub d: ::std::vec::Vec<i32>,
    }
    impl ::puroro::Message for Test4_Simple {}

    impl super::_puroro_traits::Test4Trait for Test4_Simple {
        type Field4RepeatedType<'this> = ::puroro_internal::impls::simple::VecWrapper<'this, i32>;

        fn d<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecWrapper::new(&self.d)
        }
    }

    impl ::puroro::DeserFromBytesIter for Test4_Simple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for Test4_Simple {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
                4 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated,
                    ::puroro::tags::Int32,
                >::deser_field(&mut self.d, data),

                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl ::puroro::SerToIoWrite for Test4_Simple {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Int32,
            >::ser_field(&self.d, 4, out)?;
            ::std::result::Result::Ok(())
        }
    }
}
pub use _puroro_traits::*;
pub mod _puroro_traits {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }

    pub trait Test1Trait: ::std::clone::Clone {
        fn a<'this>(&'this self) -> ::std::option::Option<i32>;
    }
    pub trait Test2Trait: ::std::clone::Clone {
        fn b<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>>;
    }
    pub trait Test3Trait: ::std::clone::Clone {
        type Field3MessageType<'this>: 'this + self::_puroro_root::official_samples::_puroro_traits::Test1Trait;
        fn c<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field3MessageType<'this>>>;
    }
    pub trait Test4Trait: ::std::clone::Clone {
        type Field4RepeatedType<'this>: ::puroro::RepeatedField<'this, i32>;

        fn d<'this>(&'this self) -> Self::Field4RepeatedType<'this>;
    }
}
pub use _puroro_nested::*;
pub mod _puroro_nested {
    pub mod test1 {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
    }
    pub mod test2 {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
    }
    pub mod test3 {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
    }
    pub mod test4 {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
    }
}
