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
    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct Test1 {
        pub a: ::std::option::Option<i32>,
    }
    impl ::puroro::Message<Test1> for Test1 {}

    impl super::_puroro_traits::Test1Trait for Test1 {
        fn a_opt<'this>(&'this self) -> Option<i32> {
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

    impl ::puroro::internal::de::DeserMessageFromBytesIter for Test1 {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::internal::types::FieldData<
                &mut ::puroro::internal::de::from_iter::ScopedIter<I>,
            >,
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

    impl ::puroro::internal::se::SerMessageToIoWrite for Test1
    where
        Self: super::_puroro_traits::Test1Trait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::Test1Trait>::a_opt(self),
                1,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::default::Default for Test1 {
        fn default() -> Self {
            Self {
                a: ::std::default::Default::default(),
            }
        }
    }
    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct Test2 {
        pub b: ::std::option::Option<::std::string::String>,
    }
    impl ::puroro::Message<Test2> for Test2 {}

    impl super::_puroro_traits::Test2Trait for Test2 {
        fn b_opt<'this>(&'this self) -> Option<&'this str> {
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

    impl ::puroro::internal::de::DeserMessageFromBytesIter for Test2 {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::internal::types::FieldData<
                &mut ::puroro::internal::de::from_iter::ScopedIter<I>,
            >,
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

    impl ::puroro::internal::se::SerMessageToIoWrite for Test2
    where
        Self: super::_puroro_traits::Test2Trait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::Test2Trait>::b_opt(self),
                2,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::default::Default for Test2 {
        fn default() -> Self {
            Self {
                b: ::std::default::Default::default(),
            }
        }
    }
    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct Test3 {
        pub c: ::std::option::Option<
            ::std::boxed::Box<self::_puroro_root::official_samples2::_puroro_simple_impl::Test1>,
        >,
    }
    impl ::puroro::Message<Test3> for Test3 {}

    impl super::_puroro_traits::Test3Trait for Test3 {
        type Field3MessageType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::official_samples2::_puroro_simple_impl::Test1;
        fn c_opt<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
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

    impl ::puroro::internal::de::DeserMessageFromBytesIter for Test3 {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::internal::types::FieldData<
                &mut ::puroro::internal::de::from_iter::ScopedIter<I>,
            >,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro::internal::impls::simple::de::DeserFieldFromBytesIter;
            match field_number {
                3 => DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::Message<
                        ::std::boxed::Box<
                            self::_puroro_root::official_samples2::_puroro_simple_impl::Test1,
                        >,
                    >,
                >::deser_field(&mut self.c, data),

                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl ::puroro::internal::se::SerMessageToIoWrite for Test3
    where
        Self: super::_puroro_traits::Test3Trait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    <Self as super::_puroro_traits::Test3Trait>::Field3MessageType<'_>,
                >,
            >::ser_field(
                <Self as super::_puroro_traits::Test3Trait>::c_opt(self),
                3,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::default::Default for Test3 {
        fn default() -> Self {
            Self {
                c: ::std::default::Default::default(),
            }
        }
    }
    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
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

    impl ::puroro::internal::de::DeserMessageFromBytesIter for Test4 {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::internal::types::FieldData<
                &mut ::puroro::internal::de::from_iter::ScopedIter<I>,
            >,
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

    impl ::puroro::internal::se::SerMessageToIoWrite for Test4
    where
        Self: super::_puroro_traits::Test4Trait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Int32,
            >::ser_field(<Self as super::_puroro_traits::Test4Trait>::d(self), 4, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::default::Default for Test4 {
        fn default() -> Self {
            Self {
                d: ::std::default::Default::default(),
            }
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
        fn a_opt<'this>(&'this self) -> Option<i32> {
            <U as Test1Trait>::a_opt(&self.1).or_else(|| <T as Test1Trait>::a_opt(&self.0))
        }
    }
    impl<T, U> Test1Trait for ::puroro::Either<T, U>
    where
        T: Test1Trait,
        U: Test1Trait,
    {
        fn a_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().either(
                |t| <T as Test1Trait>::a_opt(t),
                |u| <U as Test1Trait>::a_opt(u),
            )
        }
    }
    impl<T> Test1Trait for ::std::option::Option<T>
    where
        T: Test1Trait,
    {
        fn a_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().and_then(|msg| msg.a_opt())
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct Test1SingleField1<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub a: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Test1> for Test1SingleField1<ScalarType> where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::Test1Trait for Test1SingleField1<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn a_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.a,
            )))
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for Test1SingleField1<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        Self: super::_puroro_traits::Test1Trait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::Test1Trait>::a_opt(self),
                1,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for Test1SingleField1<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self { a: value }
        }
    }
    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct Test1Bumpalo<'bump> {
        _bump: &'bump ::puroro::bumpalo::Bump,
        pub a: ::std::option::Option<i32>,
    }
    impl<'bump> ::puroro::Message<super::_puroro_simple_impl::Test1> for Test1Bumpalo<'bump> {}

    impl<'bump> super::_puroro_traits::Test1Trait for Test1Bumpalo<'bump> {
        fn a_opt<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.a)
        }
    }

    impl<'bump> ::puroro::internal::de::DeserMessageFromBytesIter for Test1Bumpalo<'bump> {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::internal::types::FieldData<
                &mut ::puroro::internal::de::from_iter::ScopedIter<I>,
            >,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro::internal::impls::bumpalo::de::DeserFieldFromBytesIter;
            match field_number {
            1 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Int32
            >::deser_field(&mut self.a, data, &self._bump),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl<'bump> ::puroro::internal::se::SerMessageToIoWrite for Test1Bumpalo<'bump>
    where
        Self: super::_puroro_traits::Test1Trait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::Test1Trait>::a_opt(self),
                1,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }
    pub struct Test1Builder<T>(T);

    impl<T> Test1Builder<T>
    where
        T: Test1Trait,
    {
        pub fn append_a<ScalarType>(
            self,
            value: ScalarType,
        ) -> Test1Builder<(T, Test1SingleField1<ScalarType>)>
        where
            ScalarType: ::std::convert::Into<i32>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            Test1Builder((self.0, Test1SingleField1 { a: value }))
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
    impl Test2Trait for () {}
    impl<T, U> Test2Trait for (T, U)
    where
        T: Test2Trait,
        U: Test2Trait,
    {
        fn b_opt<'this>(&'this self) -> Option<&'this str> {
            <U as Test2Trait>::b_opt(&self.1).or_else(|| <T as Test2Trait>::b_opt(&self.0))
        }
    }
    impl<T, U> Test2Trait for ::puroro::Either<T, U>
    where
        T: Test2Trait,
        U: Test2Trait,
    {
        fn b_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().either(
                |t| <T as Test2Trait>::b_opt(t),
                |u| <U as Test2Trait>::b_opt(u),
            )
        }
    }
    impl<T> Test2Trait for ::std::option::Option<T>
    where
        T: Test2Trait,
    {
        fn b_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().and_then(|msg| msg.b_opt())
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct Test2SingleField2<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub b: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Test2> for Test2SingleField2<ScalarType> where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::Test2Trait for Test2SingleField2<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn b_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::Some(self.b.as_ref())
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for Test2SingleField2<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        Self: super::_puroro_traits::Test2Trait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::Test2Trait>::b_opt(self),
                2,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for Test2SingleField2<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self { b: value }
        }
    }
    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct Test2Bumpalo<'bump> {
        _bump: &'bump ::puroro::bumpalo::Bump,
        pub b: ::std::option::Option<::puroro::bumpalo::collections::String<'bump>>,
    }
    impl<'bump> ::puroro::Message<super::_puroro_simple_impl::Test2> for Test2Bumpalo<'bump> {}

    impl<'bump> super::_puroro_traits::Test2Trait for Test2Bumpalo<'bump> {
        fn b_opt<'this>(&'this self) -> Option<&'this str> {
            self.b.as_ref().map(|v| v.as_ref())
        }
    }

    impl<'bump> ::puroro::internal::de::DeserMessageFromBytesIter for Test2Bumpalo<'bump> {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::internal::types::FieldData<
                &mut ::puroro::internal::de::from_iter::ScopedIter<I>,
            >,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro::internal::impls::bumpalo::de::DeserFieldFromBytesIter;
            match field_number {
            2 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.b, data, &self._bump),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl<'bump> ::puroro::internal::se::SerMessageToIoWrite for Test2Bumpalo<'bump>
    where
        Self: super::_puroro_traits::Test2Trait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::Test2Trait>::b_opt(self),
                2,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }
    pub struct Test2Builder<T>(T);

    impl<T> Test2Builder<T>
    where
        T: Test2Trait,
    {
        pub fn append_b<ScalarType>(
            self,
            value: ScalarType,
        ) -> Test2Builder<(T, Test2SingleField2<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            Test2Builder((self.0, Test2SingleField2 { b: value }))
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
        type Field3MessageType<'this>
        where
            Self: 'this,
        = ();
    }
    impl<T, U> Test3Trait for (T, U)
    where
        T: Test3Trait,
        U: Test3Trait,
    {
        type Field3MessageType<'this>
        where
            Self: 'this,
        = (
            ::std::option::Option<<T as Test3Trait>::Field3MessageType<'this>>,
            ::std::option::Option<<U as Test3Trait>::Field3MessageType<'this>>,
        );
        fn c_opt<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            match (
                <T as Test3Trait>::c_opt(&self.0),
                <U as Test3Trait>::c_opt(&self.1),
            ) {
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
        type Field3MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as Test3Trait>::Field3MessageType<'this>,
            <U as Test3Trait>::Field3MessageType<'this>,
        >;
        fn c_opt<'this>(&'this self) -> ::std::option::Option<Self::Field3MessageType<'this>> {
            self.as_ref().either(
                |t| <T as Test3Trait>::c_opt(t).map(|t| ::puroro::Either::Left(t)),
                |u| <U as Test3Trait>::c_opt(u).map(|u| ::puroro::Either::Right(u)),
            )
        }
    }
    impl<T> Test3Trait for ::std::option::Option<T>
    where
        T: Test3Trait,
    {
        type Field3MessageType<'this>
        where
            Self: 'this,
        = T::Field3MessageType<'this>;
        fn c_opt<'this>(&'this self) -> ::std::option::Option<Self::Field3MessageType<'this>> {
            self.as_ref().and_then(|msg| msg.c_opt())
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct Test3SingleField3<ScalarType>
    where
        ScalarType: self::_puroro_root::official_samples2::_puroro_traits::Test1Trait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub c: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Test3> for Test3SingleField3<ScalarType> where
        ScalarType: self::_puroro_root::official_samples2::_puroro_traits::Test1Trait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::Test3Trait for Test3SingleField3<ScalarType>
    where
        ScalarType: self::_puroro_root::official_samples2::_puroro_traits::Test1Trait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field3MessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;

        fn c_opt<'this>(&'this self) -> ::std::option::Option<Self::Field3MessageType<'this>> {
            ::std::option::Option::Some(&self.c)
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for Test3SingleField3<ScalarType>
    where
        ScalarType: self::_puroro_root::official_samples2::_puroro_traits::Test1Trait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        Self: super::_puroro_traits::Test3Trait,
        for<'a> <Self as super::_puroro_traits::Test3Trait>::Field3MessageType<'a>:
            ::puroro::internal::se::SerMessageToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    <Self as super::_puroro_traits::Test3Trait>::Field3MessageType<'_>,
                >,
            >::ser_field(
                <Self as super::_puroro_traits::Test3Trait>::c_opt(self),
                3,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for Test3SingleField3<ScalarType>
    where
        ScalarType: self::_puroro_root::official_samples2::_puroro_traits::Test1Trait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self { c: value }
        }
    }
    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct Test3Bumpalo<'bump> {
        _bump: &'bump ::puroro::bumpalo::Bump,
        pub c: ::std::option::Option<
            ::puroro::bumpalo::boxed::Box<
                'bump,
                self::_puroro_root::official_samples2::_puroro_impls::Test1Bumpalo<'bump>,
            >,
        >,
    }
    impl<'bump> ::puroro::Message<super::_puroro_simple_impl::Test3> for Test3Bumpalo<'bump> {}

    impl<'bump> super::_puroro_traits::Test3Trait for Test3Bumpalo<'bump> {
        type Field3MessageType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::official_samples2::_puroro_impls::Test1Bumpalo<'bump>;
        fn c_opt<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            self.c.as_ref().map(|v| v.as_ref())
        }
    }

    impl<'bump> ::puroro::internal::de::DeserMessageFromBytesIter for Test3Bumpalo<'bump> {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::internal::types::FieldData<
                &mut ::puroro::internal::de::from_iter::ScopedIter<I>,
            >,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro::internal::impls::bumpalo::de::DeserFieldFromBytesIter;
            match field_number {
                3 => DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::Message<
                        ::puroro::bumpalo::boxed::Box<
                            'bump,
                            self::_puroro_root::official_samples2::_puroro_impls::Test1Bumpalo<
                                'bump,
                            >,
                        >,
                    >,
                >::deser_field(&mut self.c, data, &self._bump),

                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl<'bump> ::puroro::internal::se::SerMessageToIoWrite for Test3Bumpalo<'bump>
    where
        Self: super::_puroro_traits::Test3Trait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    <Self as super::_puroro_traits::Test3Trait>::Field3MessageType<'_>,
                >,
            >::ser_field(
                <Self as super::_puroro_traits::Test3Trait>::c_opt(self),
                3,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }
    pub struct Test3Builder<T>(T);

    impl<T> Test3Builder<T>
    where
        T: Test3Trait,
    {
        pub fn append_c<ScalarType>(
            self,
            value: ScalarType,
        ) -> Test3Builder<(T, Test3SingleField3<ScalarType>)>
        where
            ScalarType: self::_puroro_root::official_samples2::_puroro_traits::Test1Trait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            Test3Builder((self.0, Test3SingleField3 { c: value }))
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
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn d<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }
    impl<T, U> Test4Trait for (T, U)
    where
        T: Test4Trait,
        U: Test4Trait,
    {
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
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
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
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
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
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

    pub struct Test4SingleField4<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub d: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::Test4>
        for Test4SingleField4<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::Test4Trait
        for Test4SingleField4<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::single_field::CloneThenIntoIter<
            <&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter,
            i32,
        >;

        fn d<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::single_field::CloneThenIntoIter::new(
                ::std::iter::IntoIterator::into_iter(&self.d),
            )
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::se::SerMessageToIoWrite
        for Test4SingleField4<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
        Self: super::_puroro_traits::Test4Trait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Int32,
            >::ser_field(<Self as super::_puroro_traits::Test4Trait>::d(self), 4, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for Test4SingleField4<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self { d: value }
        }
    }
    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct Test4Bumpalo<'bump> {
        _bump: &'bump ::puroro::bumpalo::Bump,
        pub d: ::puroro::bumpalo::collections::Vec<'bump, i32>,
    }
    impl<'bump> ::puroro::Message<super::_puroro_simple_impl::Test4> for Test4Bumpalo<'bump> {}

    impl<'bump> super::_puroro_traits::Test4Trait for Test4Bumpalo<'bump> {
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Cloned<::std::slice::Iter<'this, i32>>;

        fn d<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            self.d.iter().cloned()
        }
    }

    impl<'bump> ::puroro::internal::de::DeserMessageFromBytesIter for Test4Bumpalo<'bump> {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::internal::types::FieldData<
                &mut ::puroro::internal::de::from_iter::ScopedIter<I>,
            >,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro::internal::impls::bumpalo::de::DeserFieldFromBytesIter;
            match field_number {
            4 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Int32
            >::deser_field(&mut self.d, data, &self._bump),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl<'bump> ::puroro::internal::se::SerMessageToIoWrite for Test4Bumpalo<'bump>
    where
        Self: super::_puroro_traits::Test4Trait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Int32,
            >::ser_field(<Self as super::_puroro_traits::Test4Trait>::d(self), 4, out)?;
            ::std::result::Result::Ok(())
        }
    }
    pub struct Test4Builder<T>(T);

    impl<T> Test4Builder<T>
    where
        T: Test4Trait,
    {
        pub fn append_d<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> Test4Builder<(T, Test4SingleField4<ScalarType, RepeatedType>)>
        where
            ScalarType: ::std::convert::Into<i32>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
            for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            Test4Builder((self.0, Test4SingleField4 { d: value }))
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
        fn a<'this>(&'this self) -> i32 {
            self.a_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_a<'this>(&'this self) -> bool {
            self.a_opt().is_some()
        }
        fn a_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::None
        }
    }

    macro_rules! test1_delegate {
        ($ty:ty) => {
            fn a_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).a_opt()
            }
        };
    }

    impl<T> Test1Trait for &'_ T
    where
        T: Test1Trait,
    {
        test1_delegate!(T);
    }

    impl<T> Test1Trait for &'_ mut T
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
        fn b<'this>(&'this self) -> &'this str {
            self.b_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_b<'this>(&'this self) -> bool {
            self.b_opt().is_some()
        }
        fn b_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::None
        }
    }

    macro_rules! test2_delegate {
        ($ty:ty) => {
            fn b_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                (**self).b_opt()
            }
        };
    }

    impl<T> Test2Trait for &'_ T
    where
        T: Test2Trait,
    {
        test2_delegate!(T);
    }

    impl<T> Test2Trait for &'_ mut T
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
            self::_puroro_root::official_samples2::_puroro_traits::Test1Trait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
            where Self: 'this;
        fn c<'this>(&'this self) -> ::std::option::Option<Self::Field3MessageType<'this>> {
            self.c_opt()
        }
        fn has_c<'this>(&'this self) -> bool {
            self.c_opt().is_some()
        }
        fn c_opt<'this>(&'this self) -> ::std::option::Option<Self::Field3MessageType<'this>> {
            ::std::option::Option::None
        }
    }

    macro_rules! test3_delegate {
        ($ty:ty) => {
            type Field3MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field3MessageType<'this>;
            fn c_opt<'this>(&'this self) -> ::std::option::Option<Self::Field3MessageType<'this>> {
                (**self).c_opt()
            }
        };
    }

    impl<T> Test3Trait for &'_ T
    where
        T: Test3Trait,
    {
        test3_delegate!(T);
    }

    impl<T> Test3Trait for &'_ mut T
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
            + ::std::iter::IntoIterator<Item = i32>
        where
            Self: 'this;
        fn d<'this>(&'this self) -> Self::Field4RepeatedType<'this>;
    }

    macro_rules! test4_delegate {
        ($ty:ty) => {
            type Field4RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field4RepeatedType<'this>;
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

    impl<T> Test4Trait for &'_ mut T
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
