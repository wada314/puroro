// A generated source code by puroro library
// package official_samples2

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub use _puroro_simple_impl::Test1;
pub use _puroro_simple_impl::Test2;
pub use _puroro_simple_impl::Test3;
pub use _puroro_simple_impl::Test4;
pub mod _puroro_simple_impl {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct Test1 {
        pub a: ::std::option::Option<i32>,
    }
    impl ::puroro::Message<Test1> for Test1 {}

    impl super::_puroro_traits::Test1Trait for Test1 {
        fn a<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.a)
        }
    }

    impl ::puroro::MessageRepresentativeImpl for Test1 {
        fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
            use ::puroro::once_cell::sync::Lazy;
            static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 1]> =
                Lazy::new(|| {
                    [{
                        let init = ::puroro::internal::FieldDescriptorInitializer {
                            name: "a",
                            number: 1,
                            lazy_containing_type: Lazy::new(|| {
                                <Test1 as ::puroro::MessageRepresentativeImpl>::descriptor()
                            }),
                        };
                        ::puroro::internal::init_field_descriptor(init)
                    }]
                });
            static LAZY_DESCRIPTOR: Lazy<::puroro::desc::MessageDescriptor> = Lazy::new(|| {
                let init = ::puroro::internal::MessageDescriptorInitializer {
                    name: "Test1",
                    lazy_fields: Lazy::new(|| Lazy::force(&LAZY_FIELD_DESCRIPTOR_ARRAY).as_ref()),
                };
                ::puroro::internal::init_message_descriptor(init)
            });
            Lazy::force(&LAZY_DESCRIPTOR)
        }
    }

    impl ::puroro::DeserFromBytesIter for Test1 {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro::internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro::internal::de::DeserFieldsFromBytesIter for Test1 {
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
                ::puroro::tags::Optional, ::puroro::tags::Int32
            >::deser_field(&mut self.a, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerToIoWrite for Test1 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Int32>::ser_field(
                &self.a, 1, out,
            )?;

            ::std::result::Result::Ok(())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct Test2 {
        pub b: ::std::option::Option<::std::borrow::Cow<'static, str>>,
    }
    impl ::puroro::Message<Test2> for Test2 {}

    impl super::_puroro_traits::Test2Trait for Test2 {
        type Field2StringType<'this> = &'this str;
        fn b<'this>(&'this self) -> Option<Self::Field2StringType<'this>> {
            self.b.as_ref().map(|v| v.as_ref())
        }
    }

    impl ::puroro::MessageRepresentativeImpl for Test2 {
        fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
            use ::puroro::once_cell::sync::Lazy;
            static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 1]> =
                Lazy::new(|| {
                    [{
                        let init = ::puroro::internal::FieldDescriptorInitializer {
                            name: "b",
                            number: 2,
                            lazy_containing_type: Lazy::new(|| {
                                <Test2 as ::puroro::MessageRepresentativeImpl>::descriptor()
                            }),
                        };
                        ::puroro::internal::init_field_descriptor(init)
                    }]
                });
            static LAZY_DESCRIPTOR: Lazy<::puroro::desc::MessageDescriptor> = Lazy::new(|| {
                let init = ::puroro::internal::MessageDescriptorInitializer {
                    name: "Test2",
                    lazy_fields: Lazy::new(|| Lazy::force(&LAZY_FIELD_DESCRIPTOR_ARRAY).as_ref()),
                };
                ::puroro::internal::init_message_descriptor(init)
            });
            Lazy::force(&LAZY_DESCRIPTOR)
        }
    }

    impl ::puroro::DeserFromBytesIter for Test2 {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro::internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro::internal::de::DeserFieldsFromBytesIter for Test2 {
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
            2 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.b, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerToIoWrite for Test2 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.b, 2, out,
            )?;

            ::std::result::Result::Ok(())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct Test3 {
        pub c: ::std::option::Option<
            ::std::boxed::Box<self::_puroro_root::official_samples2::_puroro_simple_impl::Test1>,
        >,
    }
    impl ::puroro::Message<Test3> for Test3 {}

    impl super::_puroro_traits::Test3Trait for Test3 {
        type Field3MessageType<'this> =
            &'this self::_puroro_root::official_samples2::_puroro_simple_impl::Test1;
        fn c<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            self.c.as_ref().map(|v| v.as_ref())
        }
    }

    impl ::puroro::MessageRepresentativeImpl for Test3 {
        fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
            use ::puroro::once_cell::sync::Lazy;
            static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 1]> =
                Lazy::new(|| {
                    [{
                        let init = ::puroro::internal::FieldDescriptorInitializer {
                            name: "c",
                            number: 3,
                            lazy_containing_type: Lazy::new(|| {
                                <Test3 as ::puroro::MessageRepresentativeImpl>::descriptor()
                            }),
                        };
                        ::puroro::internal::init_field_descriptor(init)
                    }]
                });
            static LAZY_DESCRIPTOR: Lazy<::puroro::desc::MessageDescriptor> = Lazy::new(|| {
                let init = ::puroro::internal::MessageDescriptorInitializer {
                    name: "Test3",
                    lazy_fields: Lazy::new(|| Lazy::force(&LAZY_FIELD_DESCRIPTOR_ARRAY).as_ref()),
                };
                ::puroro::internal::init_message_descriptor(init)
            });
            Lazy::force(&LAZY_DESCRIPTOR)
        }
    }

    impl ::puroro::DeserFromBytesIter for Test3 {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro::internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro::internal::de::DeserFieldsFromBytesIter for Test3 {
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
                3 => DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::Message<
                        self::_puroro_root::official_samples2::_puroro_simple_impl::Test1,
                    >,
                >::deser_field(&mut self.c, data),

                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl ::puroro::SerToIoWrite for Test3 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    self::_puroro_root::official_samples2::_puroro_simple_impl::Test1,
                >,
            >::ser_field(&self.c, 3, out)?;

            ::std::result::Result::Ok(())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct Test4 {
        pub d: ::std::vec::Vec<i32>,
    }
    impl ::puroro::Message<Test4> for Test4 {}

    impl super::_puroro_traits::Test4Trait for Test4 {
        type Field4RepeatedType<'this> = ::std::iter::Cloned<::std::slice::Iter<'this, i32>>;

        fn d<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            self.d.iter().cloned()
        }
    }

    impl ::puroro::MessageRepresentativeImpl for Test4 {
        fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
            use ::puroro::once_cell::sync::Lazy;
            static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 1]> =
                Lazy::new(|| {
                    [{
                        let init = ::puroro::internal::FieldDescriptorInitializer {
                            name: "d",
                            number: 4,
                            lazy_containing_type: Lazy::new(|| {
                                <Test4 as ::puroro::MessageRepresentativeImpl>::descriptor()
                            }),
                        };
                        ::puroro::internal::init_field_descriptor(init)
                    }]
                });
            static LAZY_DESCRIPTOR: Lazy<::puroro::desc::MessageDescriptor> = Lazy::new(|| {
                let init = ::puroro::internal::MessageDescriptorInitializer {
                    name: "Test4",
                    lazy_fields: Lazy::new(|| Lazy::force(&LAZY_FIELD_DESCRIPTOR_ARRAY).as_ref()),
                };
                ::puroro::internal::init_message_descriptor(init)
            });
            Lazy::force(&LAZY_DESCRIPTOR)
        }
    }

    impl ::puroro::DeserFromBytesIter for Test4 {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro::internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro::internal::de::DeserFieldsFromBytesIter for Test4 {
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
            4 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Int32
            >::deser_field(&mut self.d, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerToIoWrite for Test4 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::Int32>::ser_field(
                &self.d, 4, out,
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
    impl Test1Trait for () {}
    impl<T, U> Test1Trait for (T, U)
    where
        T: Test1Trait,
        U: Test1Trait,
    {
        fn a<'this>(&'this self) -> Option<i32> {
            <U as Test1Trait>::a(&self.1).or_else(|| <T as Test1Trait>::a(&self.0))
        }
    }
    impl<T, U> Test1Trait for ::puroro::Either<T, U>
    where
        T: Test1Trait,
        U: Test1Trait,
    {
        fn a<'this>(&'this self) -> Option<i32> {
            self.as_ref()
                .either(|t| <T as Test1Trait>::a(t), |u| <U as Test1Trait>::a(u))
        }
    }
    impl<T> Test1Trait for ::std::option::Option<T>
    where
        T: Test1Trait,
    {
        fn a<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().and_then(|msg| msg.a())
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct Test1SimpleField1 {
        pub a: ::std::option::Option<i32>,
    }

    impl ::puroro::Message<super::Test1> for Test1SimpleField1 {}

    impl super::_puroro_traits::Test1Trait for Test1SimpleField1 {
        fn a<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.a)
        }
    }

    impl ::puroro::SerToIoWrite for Test1SimpleField1 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Int32>::ser_field(
                &self.a, 1, out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<i32>> for Test1SimpleField1 {
        fn from(value: ::std::option::Option<i32>) -> Self {
            Self { a: value }
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct Test1SimpleByValue {}
    impl ::puroro::Message<super::Test1> for Test1SimpleByValue {}

    impl Test1Trait for Test1SimpleByValue {
        fn a<'this>(&'this self) -> Option<i32> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
    }
    pub struct Test1Builder<T>(T);

    impl<T> Test1Builder<T>
    where
        T: Test1Trait,
    {
        pub fn append_a(
            self,
            value: ::std::option::Option<i32>,
        ) -> Test1Builder<(T, Test1SimpleField1)> {
            Test1Builder((self.0, ::std::convert::From::from(value)))
        }

        pub fn build(self) -> T {
            self.0
        }
    }

    impl Test1Builder<()> {
        pub fn new() -> Self {
            Self(())
        }
    }
    impl Test2Trait for () {
        type Field2StringType<'this> = &'static str;
    }
    impl<T, U> Test2Trait for (T, U)
    where
        T: Test2Trait,
        U: Test2Trait,
    {
        type Field2StringType<'this> = ::puroro::Either<
            <T as Test2Trait>::Field2StringType<'this>,
            <U as Test2Trait>::Field2StringType<'this>,
        >;
        fn b<'this>(&'this self) -> Option<Self::Field2StringType<'this>> {
            if let Some(right) = <U as Test2Trait>::b(&self.1) {
                Some(::puroro::Either::Right(right))
            } else if let Some(left) = <T as Test2Trait>::b(&self.0) {
                Some(::puroro::Either::Left(left))
            } else {
                None
            }
        }
    }
    impl<T, U> Test2Trait for ::puroro::Either<T, U>
    where
        T: Test2Trait,
        U: Test2Trait,
    {
        type Field2StringType<'this> = ::puroro::Either<
            <T as Test2Trait>::Field2StringType<'this>,
            <U as Test2Trait>::Field2StringType<'this>,
        >;
        fn b<'this>(&'this self) -> Option<Self::Field2StringType<'this>> {
            self.as_ref().either(
                |t| <T as Test2Trait>::b(t).map(|t| ::puroro::Either::Left(t)),
                |u| <U as Test2Trait>::b(u).map(|u| ::puroro::Either::Right(u)),
            )
        }
    }
    impl<T> Test2Trait for ::std::option::Option<T>
    where
        T: Test2Trait,
    {
        type Field2StringType<'this> = T::Field2StringType<'this>;
        fn b<'this>(&'this self) -> ::std::option::Option<Self::Field2StringType<'this>> {
            self.as_ref().and_then(|msg| msg.b())
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct Test2SimpleField2 {
        pub b: ::std::option::Option<::std::borrow::Cow<'static, str>>,
    }

    impl ::puroro::Message<super::Test2> for Test2SimpleField2 {}

    impl super::_puroro_traits::Test2Trait for Test2SimpleField2 {
        type Field2StringType<'this> = &'this str;
        fn b<'this>(&'this self) -> Option<Self::Field2StringType<'this>> {
            self.b.as_ref().map(|v| v.as_ref())
        }
    }

    impl ::puroro::SerToIoWrite for Test2SimpleField2 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.b, 2, out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<::std::borrow::Cow<'static, str>>>
        for Test2SimpleField2
    {
        fn from(value: ::std::option::Option<::std::borrow::Cow<'static, str>>) -> Self {
            Self { b: value }
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct Test2SimpleByValue {}
    impl ::puroro::Message<super::Test2> for Test2SimpleByValue {}

    impl Test2Trait for Test2SimpleByValue {
        type Field2StringType<'this> = ::std::borrow::Cow<'this, str>;
        fn b<'this>(&'this self) -> Option<Self::Field2StringType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
    }
    pub struct Test2Builder<T>(T);

    impl<T> Test2Builder<T>
    where
        T: Test2Trait,
    {
        pub fn append_b(
            self,
            value: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        ) -> Test2Builder<(T, Test2SimpleField2)> {
            Test2Builder((self.0, ::std::convert::From::from(value)))
        }

        pub fn build(self) -> T {
            self.0
        }
    }

    impl Test2Builder<()> {
        pub fn new() -> Self {
            Self(())
        }
    }
    impl Test3Trait for () {
        type Field3MessageType<'this> = ();
    }
    impl<T, U> Test3Trait for (T, U)
    where
        T: Test3Trait,
        U: Test3Trait,
    {
        type Field3MessageType<'this> = (
            ::std::option::Option<<T as Test3Trait>::Field3MessageType<'this>>,
            ::std::option::Option<<U as Test3Trait>::Field3MessageType<'this>>,
        );
        fn c<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            match (<T as Test3Trait>::c(&self.0), <U as Test3Trait>::c(&self.1)) {
                (None, None) => None,
                (Some(t), None) => Some((Some(t), None)),
                (None, Some(u)) => Some((None, Some(u))),
                (Some(t), Some(u)) => Some((Some(t), Some(u))),
            }
        }
    }
    impl<T, U> Test3Trait for ::puroro::Either<T, U>
    where
        T: Test3Trait,
        U: Test3Trait,
    {
        type Field3MessageType<'this> = ::puroro::Either<
            <T as Test3Trait>::Field3MessageType<'this>,
            <U as Test3Trait>::Field3MessageType<'this>,
        >;
        fn c<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            self.as_ref().either(
                |t| <T as Test3Trait>::c(t).map(|t| ::puroro::Either::Left(t)),
                |u| <U as Test3Trait>::c(u).map(|u| ::puroro::Either::Right(u)),
            )
        }
    }
    impl<T> Test3Trait for ::std::option::Option<T>
    where
        T: Test3Trait,
    {
        type Field3MessageType<'this> = T::Field3MessageType<'this>;
        fn c<'this>(&'this self) -> ::std::option::Option<Self::Field3MessageType<'this>> {
            self.as_ref().and_then(|msg| msg.c())
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct Test3SimpleField3 {
        pub c: ::std::option::Option<
            ::std::boxed::Box<self::_puroro_root::official_samples2::_puroro_simple_impl::Test1>,
        >,
    }

    impl ::puroro::Message<super::Test3> for Test3SimpleField3 {}

    impl super::_puroro_traits::Test3Trait for Test3SimpleField3 {
        type Field3MessageType<'this> =
            &'this self::_puroro_root::official_samples2::_puroro_simple_impl::Test1;
        fn c<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            self.c.as_ref().map(|v| v.as_ref())
        }
    }

    impl ::puroro::SerToIoWrite for Test3SimpleField3 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    self::_puroro_root::official_samples2::_puroro_simple_impl::Test1,
                >,
            >::ser_field(&self.c, 3, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl
        ::std::convert::From<
            ::std::option::Option<
                ::std::boxed::Box<
                    self::_puroro_root::official_samples2::_puroro_simple_impl::Test1,
                >,
            >,
        > for Test3SimpleField3
    {
        fn from(
            value: ::std::option::Option<
                ::std::boxed::Box<
                    self::_puroro_root::official_samples2::_puroro_simple_impl::Test1,
                >,
            >,
        ) -> Self {
            Self { c: value }
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct Test3SimpleByValue {}
    impl ::puroro::Message<super::Test3> for Test3SimpleByValue {}

    impl Test3Trait for Test3SimpleByValue {
        type Field3MessageType<'this> =
            ::std::boxed::Box<self::_puroro_root::official_samples2::_puroro_simple_impl::Test1>;
        fn c<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
    }
    pub struct Test3Builder<T>(T);

    impl<T> Test3Builder<T>
    where
        T: Test3Trait,
    {
        pub fn append_c(
            self,
            value: ::std::option::Option<
                ::std::boxed::Box<
                    self::_puroro_root::official_samples2::_puroro_simple_impl::Test1,
                >,
            >,
        ) -> Test3Builder<(T, Test3SimpleField3)> {
            Test3Builder((self.0, ::std::convert::From::from(value)))
        }

        pub fn build(self) -> T {
            self.0
        }
    }

    impl Test3Builder<()> {
        pub fn new() -> Self {
            Self(())
        }
    }
    impl Test4Trait for () {
        type Field4RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn d<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }
    impl<T, U> Test4Trait for (T, U)
    where
        T: Test4Trait,
        U: Test4Trait,
    {
        type Field4RepeatedType<'this> = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as Test4Trait>::Field4RepeatedType<'this>,
            <U as Test4Trait>::Field4RepeatedType<'this>,
        >;

        fn d<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as Test4Trait>::d(&self.0),
                <U as Test4Trait>::d(&self.1),
            )
        }
    }
    impl<T, U> Test4Trait for ::puroro::Either<T, U>
    where
        T: Test4Trait,
        U: Test4Trait,
    {
        type Field4RepeatedType<'this> = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as Test4Trait>::Field4RepeatedType<'this>,
            <U as Test4Trait>::Field4RepeatedType<'this>,
        >;

        fn d<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as Test4Trait>::d(t))
                    .map_right(|u| <U as Test4Trait>::d(u)),
            )
        }
    }
    impl<T> Test4Trait for ::std::option::Option<T>
    where
        T: Test4Trait,
    {
        type Field4RepeatedType<'this> = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field4RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn d<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.d().into_iter())
                .into_iter()
                .flatten()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct Test4SimpleField4 {
        pub d: ::std::vec::Vec<i32>,
    }

    impl ::puroro::Message<super::Test4> for Test4SimpleField4 {}

    impl super::_puroro_traits::Test4Trait for Test4SimpleField4 {
        type Field4RepeatedType<'this> = ::std::iter::Cloned<::std::slice::Iter<'this, i32>>;

        fn d<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            self.d.iter().cloned()
        }
    }

    impl ::puroro::SerToIoWrite for Test4SimpleField4 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::Int32>::ser_field(
                &self.d, 4, out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::vec::Vec<i32>> for Test4SimpleField4 {
        fn from(value: ::std::vec::Vec<i32>) -> Self {
            Self { d: value }
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct Test4SimpleByValue {}
    impl ::puroro::Message<super::Test4> for Test4SimpleByValue {}

    impl Test4Trait for Test4SimpleByValue {
        type Field4RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn d<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
    }
    pub struct Test4Builder<T>(T);

    impl<T> Test4Builder<T>
    where
        T: Test4Trait,
    {
        pub fn append_d(self, value: ::std::vec::Vec<i32>) -> Test4Builder<(T, Test4SimpleField4)> {
            Test4Builder((self.0, ::std::convert::From::from(value)))
        }

        pub fn build(self) -> T {
            self.0
        }
    }

    impl Test4Builder<()> {
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

    pub trait Test1Trait {
        fn a<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::default::Default::default()
        }
    }

    macro_rules! test1_delegate {
        ($ty:ty) => {
            fn a<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).a()
            }
        };
    }

    impl<T> Test1Trait for &'_ T
    where
        T: Test1Trait,
    {
        test1_delegate!(T);
    }

    impl<T> Test1Trait for ::std::boxed::Box<T>
    where
        T: Test1Trait,
    {
        test1_delegate!(T);
    }
    pub trait Test2Trait {
        type Field2StringType<'this>: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug;
        fn b<'this>(&'this self) -> ::std::option::Option<Self::Field2StringType<'this>> {
            ::std::default::Default::default()
        }
    }

    macro_rules! test2_delegate {
        ($ty:ty) => {
            type Field2StringType<'this> = <$ty>::Field2StringType<'this>;
            fn b<'this>(&'this self) -> ::std::option::Option<Self::Field2StringType<'this>> {
                (**self).b()
            }
        };
    }

    impl<T> Test2Trait for &'_ T
    where
        T: Test2Trait,
    {
        test2_delegate!(T);
    }

    impl<T> Test2Trait for ::std::boxed::Box<T>
    where
        T: Test2Trait,
    {
        test2_delegate!(T);
    }
    pub trait Test3Trait {
        type Field3MessageType<'this>:
            self::_puroro_root::official_samples2::_puroro_traits::Test1Trait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug;
        fn c<'this>(&'this self) -> ::std::option::Option<Self::Field3MessageType<'this>> {
            ::std::default::Default::default()
        }
    }

    macro_rules! test3_delegate {
        ($ty:ty) => {
            type Field3MessageType<'this> = <$ty>::Field3MessageType<'this>;
            fn c<'this>(&'this self) -> ::std::option::Option<Self::Field3MessageType<'this>> {
                (**self).c()
            }
        };
    }

    impl<T> Test3Trait for &'_ T
    where
        T: Test3Trait,
    {
        test3_delegate!(T);
    }

    impl<T> Test3Trait for ::std::boxed::Box<T>
    where
        T: Test3Trait,
    {
        test3_delegate!(T);
    }
    pub trait Test4Trait {
        type Field4RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = i32>;
        fn d<'this>(&'this self) -> Self::Field4RepeatedType<'this>;
    }

    macro_rules! test4_delegate {
        ($ty:ty) => {
            type Field4RepeatedType<'this> = <$ty>::Field4RepeatedType<'this>;
            fn d<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
                (**self).d()
            }
        };
    }

    impl<T> Test4Trait for &'_ T
    where
        T: Test4Trait,
    {
        test4_delegate!(T);
    }

    impl<T> Test4Trait for ::std::boxed::Box<T>
    where
        T: Test4Trait,
    {
        test4_delegate!(T);
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
