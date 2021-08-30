// A generated source code by puroro library
// package official_samples

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub use _puroro_impls::Test1Simple as Test1;
pub use _puroro_impls::Test2Simple as Test2;
pub use _puroro_impls::Test3Simple as Test3;
pub use _puroro_impls::Test4Simple as Test4;
pub mod _puroro_impls {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct Test1Simple {
        pub a: ::std::option::Option<i32>,
    }
    impl ::puroro::Message for Test1Simple {}

    impl super::_puroro_traits::Test1Trait for Test1Simple {
        fn a<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::clone::Clone::clone(&self.a)
        }
    }

    impl ::puroro::DeserFromBytesIter for Test1Simple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for Test1Simple {
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

    impl ::puroro::SerToIoWrite for Test1Simple {
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
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct Test1Empty;

    impl ::puroro::Message for Test1Empty {}

    impl super::_puroro_traits::Test1Trait for Test1Empty {}

    impl ::puroro::SerToIoWrite for Test1Empty {
        fn ser<W>(&self, _out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::std::result::Result::Ok(())
        }
    }
    pub struct Test1Merged<T, U> {
        t: T,
        u: U,
    }

    impl<T, U> Test1Merged<T, U> {
        pub fn new(t: T, u: U) -> Self {
            Self { t, u }
        }
    }

    impl<T, U> ::puroro::Message for Test1Merged<T, U> {}

    /*
    impl<T, U> super::_puroro_traits::Test1Trait for Test1Merged<T, U>
    where
        T: super::_puroro_traits::Test1Trait,
        U: super::_puroro_traits::Test1Trait,
    {
    }
    */
    impl<T, U> super::_puroro_traits::Test1Trait for ::puroro::Either<T, U>
    where
        T: ::std::ops::Deref,
        U: ::std::ops::Deref,
        <T as ::std::ops::Deref>::Target: super::_puroro_traits::Test1Trait,
        <U as ::std::ops::Deref>::Target: super::_puroro_traits::Test1Trait,
    {
        fn a<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().either(
                |t| <<T as ::std::ops::Deref>::Target as super::_puroro_traits::Test1Trait>::a(t),
                |u| <<U as ::std::ops::Deref>::Target as super::_puroro_traits::Test1Trait>::a(u),
            )
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct Test1SimpleField1 {
        a: ::std::option::Option<i32>,
    }

    impl ::puroro::Message for Test1SimpleField1 {}

    impl super::_puroro_traits::Test1Trait for Test1SimpleField1 {
        fn a<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::clone::Clone::clone(&self.a)
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct Test2Simple {
        pub b: ::std::option::Option<::std::string::String>,
    }
    impl ::puroro::Message for Test2Simple {}

    impl super::_puroro_traits::Test2Trait for Test2Simple {
        type Field2ScalarGetterType<'this> = &'this str;
        fn b<'this>(&'this self) -> ::std::option::Option<Self::Field2ScalarGetterType<'this>> {
            self.b.as_ref().map(|v| v.as_ref())
        }
    }

    impl ::puroro::DeserFromBytesIter for Test2Simple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for Test2Simple {
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

    impl ::puroro::SerToIoWrite for Test2Simple {
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
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct Test2Empty;

    impl ::puroro::Message for Test2Empty {}

    impl super::_puroro_traits::Test2Trait for Test2Empty {
        type Field2ScalarGetterType<'this> = &'static str;
    }

    impl ::puroro::SerToIoWrite for Test2Empty {
        fn ser<W>(&self, _out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::std::result::Result::Ok(())
        }
    }
    pub struct Test2Merged<T, U> {
        t: T,
        u: U,
    }

    impl<T, U> Test2Merged<T, U> {
        pub fn new(t: T, u: U) -> Self {
            Self { t, u }
        }
    }

    impl<T, U> ::puroro::Message for Test2Merged<T, U> {}

    /*
    impl<T, U> super::_puroro_traits::Test2Trait for Test2Merged<T, U>
    where
        T: super::_puroro_traits::Test2Trait,
        U: super::_puroro_traits::Test2Trait,
    {
    }
    */
    impl<T, U> super::_puroro_traits::Test2Trait for ::puroro::Either<T, U>
    where
        T: ::std::ops::Deref,
        U: ::std::ops::Deref,
        <T as ::std::ops::Deref>::Target: super::_puroro_traits::Test2Trait,
        <U as ::std::ops::Deref>::Target: super::_puroro_traits::Test2Trait,
    {
        type Field2ScalarGetterType<'this> = ::puroro::Either<
    <<T as ::std::ops::Deref>::Target as super::_puroro_traits::Test2Trait>::Field2ScalarGetterType<'this>,
    <<U as ::std::ops::Deref>::Target as super::_puroro_traits::Test2Trait>::Field2ScalarGetterType<'this>,
>;
        fn b<'this>(&'this self) -> ::std::option::Option<Self::Field2ScalarGetterType<'this>> {
            todo!()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct Test2SimpleField2 {
        b: ::std::option::Option<::std::string::String>,
    }

    impl ::puroro::Message for Test2SimpleField2 {}

    impl super::_puroro_traits::Test2Trait for Test2SimpleField2 {
        type Field2ScalarGetterType<'this> = &'this str;
        fn b<'this>(&'this self) -> ::std::option::Option<Self::Field2ScalarGetterType<'this>> {
            self.b.as_ref().map(|v| v.as_ref())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct Test3Simple {
        pub c: ::std::option::Option<
            ::std::boxed::Box<self::_puroro_root::official_samples::_puroro_impls::Test1Simple>,
        >,
    }
    impl ::puroro::Message for Test3Simple {}

    impl super::_puroro_traits::Test3Trait for Test3Simple {
        type Field3MessageType<'this> =
            self::_puroro_root::official_samples::_puroro_impls::Test1Simple;
        type Field3ScalarGetterType<'this> = &'this Self::Field3MessageType<'this>;
        fn c<'this>(&'this self) -> ::std::option::Option<Self::Field3ScalarGetterType<'this>> {
            self.c.as_ref().map(|v| v.as_ref())
        }
    }

    impl ::puroro::DeserFromBytesIter for Test3Simple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for Test3Simple {
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
                        self::_puroro_root::official_samples::_puroro_impls::Test1Simple,
                    >,
                >::deser_field(&mut self.c, data),

                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl ::puroro::SerToIoWrite for Test3Simple {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    self::_puroro_root::official_samples::_puroro_impls::Test1Simple,
                >,
            >::ser_field(&self.c, 3, out)?;
            ::std::result::Result::Ok(())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct Test3Empty;

    impl ::puroro::Message for Test3Empty {}

    impl super::_puroro_traits::Test3Trait for Test3Empty {
        type Field3MessageType<'this> =
            self::_puroro_root::official_samples::_puroro_impls::Test1Empty;
        type Field3ScalarGetterType<'this> = &'static Self::Field3MessageType<'this>;
    }

    impl ::puroro::SerToIoWrite for Test3Empty {
        fn ser<W>(&self, _out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::std::result::Result::Ok(())
        }
    }
    pub struct Test3Merged<T, U> {
        t: T,
        u: U,
    }

    impl<T, U> Test3Merged<T, U> {
        pub fn new(t: T, u: U) -> Self {
            Self { t, u }
        }
    }

    impl<T, U> ::puroro::Message for Test3Merged<T, U> {}

    /*
    impl<T, U> super::_puroro_traits::Test3Trait for Test3Merged<T, U>
    where
        T: super::_puroro_traits::Test3Trait,
        U: super::_puroro_traits::Test3Trait,
    {
    }
    */
    impl<T, U> super::_puroro_traits::Test3Trait for ::puroro::Either<T, U>
    where
        T: ::std::ops::Deref,
        U: ::std::ops::Deref,
        <T as ::std::ops::Deref>::Target: super::_puroro_traits::Test3Trait,
        <U as ::std::ops::Deref>::Target: super::_puroro_traits::Test3Trait,
    {
        type Field3MessageType<'this> = ::puroro::Either<
    <<T as ::std::ops::Deref>::Target as super::_puroro_traits::Test3Trait>::Field3ScalarGetterType<'this>,
    <<U as ::std::ops::Deref>::Target as super::_puroro_traits::Test3Trait>::Field3ScalarGetterType<'this>,
>;
        type Field3ScalarGetterType<'this> =
            ::puroro_internal::Derefable<Self::Field3MessageType<'this>>;
        fn c<'this>(&'this self) -> ::std::option::Option<Self::Field3ScalarGetterType<'this>> {
            todo!()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct Test3SimpleField3 {
        c: ::std::option::Option<
            ::std::boxed::Box<self::_puroro_root::official_samples::_puroro_impls::Test1Simple>,
        >,
    }

    impl ::puroro::Message for Test3SimpleField3 {}

    impl super::_puroro_traits::Test3Trait for Test3SimpleField3 {
        type Field3MessageType<'this> =
            self::_puroro_root::official_samples::_puroro_impls::Test1Simple;
        type Field3ScalarGetterType<'this> = &'this Self::Field3MessageType<'this>;
        fn c<'this>(&'this self) -> ::std::option::Option<Self::Field3ScalarGetterType<'this>> {
            self.c.as_ref().map(|v| v.as_ref())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct Test4Simple {
        pub d: ::std::vec::Vec<i32>,
    }
    impl ::puroro::Message for Test4Simple {}

    impl super::_puroro_traits::Test4Trait for Test4Simple {
        type Field4RepeatedType<'this> = ::std::iter::Cloned<::std::slice::Iter<'this, i32>>;

        fn d<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            self.d.iter().cloned()
        }
    }

    impl ::puroro::DeserFromBytesIter for Test4Simple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for Test4Simple {
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

    impl ::puroro::SerToIoWrite for Test4Simple {
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
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct Test4Empty;

    impl ::puroro::Message for Test4Empty {}

    impl super::_puroro_traits::Test4Trait for Test4Empty {
        type Field4RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn d<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerToIoWrite for Test4Empty {
        fn ser<W>(&self, _out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::std::result::Result::Ok(())
        }
    }
    pub struct Test4Merged<T, U> {
        t: T,
        u: U,
    }

    impl<T, U> Test4Merged<T, U> {
        pub fn new(t: T, u: U) -> Self {
            Self { t, u }
        }
    }

    impl<T, U> ::puroro::Message for Test4Merged<T, U> {}

    /*
    impl<T, U> super::_puroro_traits::Test4Trait for Test4Merged<T, U>
    where
        T: super::_puroro_traits::Test4Trait,
        U: super::_puroro_traits::Test4Trait,
    {
    }
    */
    impl<T, U> super::_puroro_traits::Test4Trait for ::puroro::Either<T, U>
    where
        T: ::std::ops::Deref,
        U: ::std::ops::Deref,
        <T as ::std::ops::Deref>::Target: super::_puroro_traits::Test4Trait,
        <U as ::std::ops::Deref>::Target: super::_puroro_traits::Test4Trait,
    {
        type Field4RepeatedType<'this> = ::puroro_internal::impls::either::EitherRepeatedField<
    <<T as ::std::ops::Deref>::Target as super::_puroro_traits::Test4Trait>::Field4RepeatedType<'this>,
    <<U as ::std::ops::Deref>::Target as super::_puroro_traits::Test4Trait>::Field4RepeatedType<'this>,
>;

        fn d<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| {
                        <<T as ::std::ops::Deref>::Target as super::_puroro_traits::Test4Trait>::d(
                            t,
                        )
                    })
                    .map_right(|u| {
                        <<U as ::std::ops::Deref>::Target as super::_puroro_traits::Test4Trait>::d(
                            u,
                        )
                    }),
            )
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct Test4SimpleField4 {
        d: ::std::vec::Vec<i32>,
    }

    impl ::puroro::Message for Test4SimpleField4 {}

    impl super::_puroro_traits::Test4Trait for Test4SimpleField4 {
        type Field4RepeatedType<'this> = ::std::iter::Cloned<::std::slice::Iter<'this, i32>>;

        fn d<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            self.d.iter().cloned()
        }
    }
}
pub use _puroro_traits::*;
pub mod _puroro_traits {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }

    pub trait Test1Trait {
        fn a<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::default::Default::default()
        }
    }
    pub trait Test2Trait {
        type Field2ScalarGetterType<'this>: ::std::ops::Deref<Target = str>;
        fn b<'this>(&'this self) -> ::std::option::Option<Self::Field2ScalarGetterType<'this>> {
            ::std::default::Default::default()
        }
    }
    pub trait Test3Trait {
        type Field3MessageType<'this>: self::_puroro_root::official_samples::_puroro_traits::Test1Trait;
        type Field3ScalarGetterType<'this>: ::std::ops::Deref<
            Target = Self::Field3MessageType<'this>,
        >;
        fn c<'this>(&'this self) -> ::std::option::Option<Self::Field3ScalarGetterType<'this>> {
            ::std::default::Default::default()
        }
    }
    pub trait Test4Trait {
        type Field4RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = i32>;
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
