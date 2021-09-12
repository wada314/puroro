// A generated source code by puroro library
// package official_samples3

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
    use super::_puroro_traits::*;
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct Test1Simple {
        pub a: i32,
    }
    impl ::puroro::Message<Test1Simple> for Test1Simple {}

    impl Test1Trait for Test1Simple {
        fn a<'this>(&'this self) -> i32 {
            Clone::clone(&self.a)
        }
    }

    impl ::puroro::MessageRepresentativeImpl for Test1Simple {
        fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
            use ::puroro::once_cell::sync::Lazy;
            static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 1]> =
                Lazy::new(|| {
                    [{
                        let init = ::puroro::internal::FieldDescriptorInitializer {
                            lazy_containing_type: Lazy::new(|| {
                                <Test1Simple as ::puroro::MessageRepresentativeImpl>::descriptor()
                            }),
                        };
                        ::puroro::internal::init_field_descriptor(init)
                    }]
                });
            static LAZY_DESCRIPTOR: Lazy<::puroro::desc::MessageDescriptor> = Lazy::new(|| {
                let init = ::puroro::internal::MessageDescriptorInitializer {
                    lazy_fields: Lazy::new(|| Lazy::force(&LAZY_FIELD_DESCRIPTOR_ARRAY).as_ref()),
                };
                ::puroro::internal::init_message_descriptor(init)
            });
            Lazy::force(&LAZY_DESCRIPTOR)
        }
    }

    impl ::puroro::DeserFromBytesIter for Test1Simple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro::internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro::internal::de::DeserFieldsFromBytesIter for Test1Simple {
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
                ::puroro::tags::Unlabeled, ::puroro::tags::Int32
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
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Unlabeled, ::puroro::tags::Int32>::ser_field(
                &self.a, 1, out,
            )?;

            ::std::result::Result::Ok(())
        }
    }
    impl Test1Trait for () {
        fn a<'this>(&'this self) -> i32 {
            Default::default()
        }
    }
    impl<T, U> Test1Trait for (T, U)
    where
        T: Test1Trait,
        U: Test1Trait,
    {
        fn a<'this>(&'this self) -> i32 {
            let right = <U as Test1Trait>::a(&self.1);
            if right != ::std::default::Default::default() {
                right
            } else {
                <T as Test1Trait>::a(&self.0)
            }
        }
    }
    impl<T, U> Test1Trait for ::puroro::Either<T, U>
    where
        T: Test1Trait,
        U: Test1Trait,
    {
        fn a<'this>(&'this self) -> i32 {
            self.as_ref()
                .either(|t| <T as Test1Trait>::a(t), |u| <U as Test1Trait>::a(u))
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct Test1SimpleField1 {
        a: i32,
    }

    impl ::puroro::Message<Test1Simple> for Test1SimpleField1 {}

    impl super::_puroro_traits::Test1Trait for Test1SimpleField1 {
        fn a<'this>(&'this self) -> i32 {
            Clone::clone(&self.a)
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct Test1SimpleByValue {}
    impl ::puroro::Message<Test1Simple> for Test1SimpleByValue {}

    impl Test1Trait for Test1SimpleByValue {
        fn a<'this>(&'this self) -> i32 {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct Test2Simple {
        pub b: ::std::borrow::Cow<'static, str>,
    }
    impl ::puroro::Message<Test2Simple> for Test2Simple {}

    impl Test2Trait for Test2Simple {
        type Field2StringType<'this> = &'this str;
        fn b<'this>(&'this self) -> Self::Field2StringType<'this> {
            &self.b
        }
    }

    impl ::puroro::MessageRepresentativeImpl for Test2Simple {
        fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
            use ::puroro::once_cell::sync::Lazy;
            static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 1]> =
                Lazy::new(|| {
                    [{
                        let init = ::puroro::internal::FieldDescriptorInitializer {
                            lazy_containing_type: Lazy::new(|| {
                                <Test2Simple as ::puroro::MessageRepresentativeImpl>::descriptor()
                            }),
                        };
                        ::puroro::internal::init_field_descriptor(init)
                    }]
                });
            static LAZY_DESCRIPTOR: Lazy<::puroro::desc::MessageDescriptor> = Lazy::new(|| {
                let init = ::puroro::internal::MessageDescriptorInitializer {
                    lazy_fields: Lazy::new(|| Lazy::force(&LAZY_FIELD_DESCRIPTOR_ARRAY).as_ref()),
                };
                ::puroro::internal::init_message_descriptor(init)
            });
            Lazy::force(&LAZY_DESCRIPTOR)
        }
    }

    impl ::puroro::DeserFromBytesIter for Test2Simple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro::internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro::internal::de::DeserFieldsFromBytesIter for Test2Simple {
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
                ::puroro::tags::Unlabeled, ::puroro::tags::String
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
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Unlabeled, ::puroro::tags::String>::ser_field(
                &self.b, 2, out,
            )?;

            ::std::result::Result::Ok(())
        }
    }
    impl Test2Trait for () {
        type Field2StringType<'this> = &'static str;
        fn b<'this>(&'this self) -> Self::Field2StringType<'this> {
            ""
        }
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
        fn b<'this>(&'this self) -> Self::Field2StringType<'this> {
            let right = <U as Test2Trait>::b(&self.1);
            if !right.is_empty() {
                ::puroro::Either::Right(right)
            } else {
                ::puroro::Either::Left(<T as Test2Trait>::b(&self.0))
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
        fn b<'this>(&'this self) -> Self::Field2StringType<'this> {
            self.as_ref().either(
                |t| ::puroro::Either::Left(<T as Test2Trait>::b(t)),
                |u| ::puroro::Either::Right(<U as Test2Trait>::b(u)),
            )
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct Test2SimpleField2 {
        b: ::std::borrow::Cow<'static, str>,
    }

    impl ::puroro::Message<Test2Simple> for Test2SimpleField2 {}

    impl super::_puroro_traits::Test2Trait for Test2SimpleField2 {
        type Field2StringType<'this> = &'this str;
        fn b<'this>(&'this self) -> Self::Field2StringType<'this> {
            &self.b
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct Test2SimpleByValue {}
    impl ::puroro::Message<Test2Simple> for Test2SimpleByValue {}

    impl Test2Trait for Test2SimpleByValue {
        type Field2StringType<'this> = ::std::borrow::Cow<'this, str>;
        fn b<'this>(&'this self) -> Self::Field2StringType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct Test3Simple {
        pub c: ::std::option::Option<
            ::std::boxed::Box<self::_puroro_root::official_samples3::_puroro_impls::Test1Simple>,
        >,
    }
    impl ::puroro::Message<Test3Simple> for Test3Simple {}

    impl Test3Trait for Test3Simple {
        type Field3MessageType<'this> =
            &'this self::_puroro_root::official_samples3::_puroro_impls::Test1Simple;
        fn c<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            self.c.as_ref().map(|v| v.as_ref())
        }
    }

    impl ::puroro::MessageRepresentativeImpl for Test3Simple {
        fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
            use ::puroro::once_cell::sync::Lazy;
            static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 1]> =
                Lazy::new(|| {
                    [{
                        let init = ::puroro::internal::FieldDescriptorInitializer {
                            lazy_containing_type: Lazy::new(|| {
                                <Test3Simple as ::puroro::MessageRepresentativeImpl>::descriptor()
                            }),
                        };
                        ::puroro::internal::init_field_descriptor(init)
                    }]
                });
            static LAZY_DESCRIPTOR: Lazy<::puroro::desc::MessageDescriptor> = Lazy::new(|| {
                let init = ::puroro::internal::MessageDescriptorInitializer {
                    lazy_fields: Lazy::new(|| Lazy::force(&LAZY_FIELD_DESCRIPTOR_ARRAY).as_ref()),
                };
                ::puroro::internal::init_message_descriptor(init)
            });
            Lazy::force(&LAZY_DESCRIPTOR)
        }
    }

    impl ::puroro::DeserFromBytesIter for Test3Simple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro::internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro::internal::de::DeserFieldsFromBytesIter for Test3Simple {
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
                    ::puroro::tags::Unlabeled,
                    ::puroro::tags::Message<
                        self::_puroro_root::official_samples3::_puroro_impls::Test1Simple,
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
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Message<
                    self::_puroro_root::official_samples3::_puroro_impls::Test1Simple,
                >,
            >::ser_field(&self.c, 3, out)?;

            ::std::result::Result::Ok(())
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
        type Field3MessageType<'this> = ::puroro::Either<
            ::puroro::Either<
                <T as Test3Trait>::Field3MessageType<'this>,
                <U as Test3Trait>::Field3MessageType<'this>,
            >,
            (
                <T as Test3Trait>::Field3MessageType<'this>,
                <U as Test3Trait>::Field3MessageType<'this>,
            ),
        >;
        fn c<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            match (<T as Test3Trait>::c(&self.0), <U as Test3Trait>::c(&self.1)) {
                (None, None) => None,
                (Some(t), None) => Some(::puroro::Either::Left(::puroro::Either::Left(t))),
                (None, Some(u)) => Some(::puroro::Either::Left(::puroro::Either::Right(u))),
                (Some(t), Some(u)) => Some(::puroro::Either::Right((t, u))),
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

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct Test3SimpleField3 {
        c: ::std::option::Option<
            ::std::boxed::Box<self::_puroro_root::official_samples3::_puroro_impls::Test1Simple>,
        >,
    }

    impl ::puroro::Message<Test3Simple> for Test3SimpleField3 {}

    impl super::_puroro_traits::Test3Trait for Test3SimpleField3 {
        type Field3MessageType<'this> =
            &'this self::_puroro_root::official_samples3::_puroro_impls::Test1Simple;
        fn c<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            self.c.as_ref().map(|v| v.as_ref())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct Test3SimpleByValue {}
    impl ::puroro::Message<Test3Simple> for Test3SimpleByValue {}

    impl Test3Trait for Test3SimpleByValue {
        type Field3MessageType<'this> =
            ::std::boxed::Box<self::_puroro_root::official_samples3::_puroro_impls::Test1Simple>;
        fn c<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct Test4Simple {
        pub d: ::std::vec::Vec<i32>,
    }
    impl ::puroro::Message<Test4Simple> for Test4Simple {}

    impl Test4Trait for Test4Simple {
        type Field4RepeatedType<'this> = ::std::iter::Cloned<::std::slice::Iter<'this, i32>>;

        fn d<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            self.d.iter().cloned()
        }
    }

    impl ::puroro::MessageRepresentativeImpl for Test4Simple {
        fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
            use ::puroro::once_cell::sync::Lazy;
            static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 1]> =
                Lazy::new(|| {
                    [{
                        let init = ::puroro::internal::FieldDescriptorInitializer {
                            lazy_containing_type: Lazy::new(|| {
                                <Test4Simple as ::puroro::MessageRepresentativeImpl>::descriptor()
                            }),
                        };
                        ::puroro::internal::init_field_descriptor(init)
                    }]
                });
            static LAZY_DESCRIPTOR: Lazy<::puroro::desc::MessageDescriptor> = Lazy::new(|| {
                let init = ::puroro::internal::MessageDescriptorInitializer {
                    lazy_fields: Lazy::new(|| Lazy::force(&LAZY_FIELD_DESCRIPTOR_ARRAY).as_ref()),
                };
                ::puroro::internal::init_message_descriptor(init)
            });
            Lazy::force(&LAZY_DESCRIPTOR)
        }
    }

    impl ::puroro::DeserFromBytesIter for Test4Simple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro::internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro::internal::de::DeserFieldsFromBytesIter for Test4Simple {
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

    impl ::puroro::SerToIoWrite for Test4Simple {
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

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct Test4SimpleField4 {
        d: ::std::vec::Vec<i32>,
    }

    impl ::puroro::Message<Test4Simple> for Test4SimpleField4 {}

    impl super::_puroro_traits::Test4Trait for Test4SimpleField4 {
        type Field4RepeatedType<'this> = ::std::iter::Cloned<::std::slice::Iter<'this, i32>>;

        fn d<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            self.d.iter().cloned()
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct Test4SimpleByValue {}
    impl ::puroro::Message<Test4Simple> for Test4SimpleByValue {}

    impl Test4Trait for Test4SimpleByValue {
        type Field4RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn d<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
    }
}
pub use _puroro_traits::*;
pub mod _puroro_traits {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }

    pub trait Test1Trait {
        fn a<'this>(&'this self) -> i32;
    }

    macro_rules! test1_delegate {
        ($ty:ty) => {
            fn a<'this>(&'this self) -> i32 {
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
        fn b<'this>(&'this self) -> Self::Field2StringType<'this>;
    }

    macro_rules! test2_delegate {
        ($ty:ty) => {
            type Field2StringType<'this> = <$ty>::Field2StringType<'this>;
            fn b<'this>(&'this self) -> Self::Field2StringType<'this> {
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
            self::_puroro_root::official_samples3::_puroro_traits::Test1Trait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug;
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
