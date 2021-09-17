// A generated source code by puroro library
#![feature(generic_associated_types)]
// package (root)
pub mod full_coverage2;
pub mod full_coverage3;
pub mod official_samples2;
pub mod official_samples3;
pub mod oneofs2;
pub mod oneofs3;
pub mod proto3_defaults;
pub mod self_recursive;
pub mod ser_tests2;
pub mod ser_tests3;

pub mod _puroro_root {
    pub use super::*;
}

pub use _puroro_impls::MyMessageSimple as MyMessage;
pub use _puroro_impls::*;
pub mod _puroro_impls {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
    use super::_puroro_traits::*;
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct MyMessageSimple {
        pub my_number: i32,
        pub my_name: ::std::vec::Vec<::std::borrow::Cow<'static, str>>,
        pub my_child: ::std::option::Option<
            ::std::boxed::Box<self::_puroro_root::_puroro_impls::MyMessageSimple>,
        >,
    }
    impl ::puroro::Message<MyMessageSimple> for MyMessageSimple {}

    impl MyMessageTrait for MyMessageSimple {
        fn my_number<'this>(&'this self) -> i32 {
            Clone::clone(&self.my_number)
        }
        type Field2StringType<'this> = &'this str;
        type Field2RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            str,
            ::std::slice::Iter<'this, ::std::borrow::Cow<'static, str>>,
        >;

        fn my_name<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.my_name.iter())
        }
        type Field3MessageType<'this> = &'this self::_puroro_root::_puroro_impls::MyMessageSimple;
        fn my_child<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            self.my_child.as_ref().map(|v| v.as_ref())
        }
    }

    impl ::puroro::MessageRepresentativeImpl for MyMessageSimple {
        fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
            use ::puroro::once_cell::sync::Lazy;
            static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 3]> =
                Lazy::new(|| {
                    [
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "my_number",
                                number: 1,
                                lazy_containing_type: Lazy::new(|| {
                                    <MyMessageSimple as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "my_name",
                                number: 2,
                                lazy_containing_type: Lazy::new(|| {
                                    <MyMessageSimple as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "my_child",
                                number: 3,
                                lazy_containing_type: Lazy::new(|| {
                                    <MyMessageSimple as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                    ]
                });
            static LAZY_DESCRIPTOR: Lazy<::puroro::desc::MessageDescriptor> = Lazy::new(|| {
                let init = ::puroro::internal::MessageDescriptorInitializer {
                    name: "MyMessage",
                    lazy_fields: Lazy::new(|| Lazy::force(&LAZY_FIELD_DESCRIPTOR_ARRAY).as_ref()),
                };
                ::puroro::internal::init_message_descriptor(init)
            });
            Lazy::force(&LAZY_DESCRIPTOR)
        }
    }

    impl ::puroro::DeserFromBytesIter for MyMessageSimple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro::internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro::internal::de::DeserFieldsFromBytesIter for MyMessageSimple {
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
            >::deser_field(&mut self.my_number, data),
            2 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::String
            >::deser_field(&mut self.my_name, data),
            3 => DeserFieldFromBytesIter::<
                ::puroro::tags::Unlabeled, ::puroro::tags::Message<self::_puroro_root::_puroro_impls::MyMessageSimple>
            >::deser_field(&mut self.my_child, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerToIoWrite for MyMessageSimple {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Unlabeled, ::puroro::tags::Int32>::ser_field(
                &self.my_number,
                1,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::String>::ser_field(
                &self.my_name,
                2,
                out,
            )?;
            SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Message<self::_puroro_root::_puroro_impls::MyMessageSimple>,
            >::ser_field(&self.my_child, 3, out)?;

            ::std::result::Result::Ok(())
        }
    }
    impl MyMessageTrait for () {
        fn my_number<'this>(&'this self) -> i32 {
            Default::default()
        }
        type Field2StringType<'this> = &'static str;
        type Field2RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2StringType<'this>>;
        fn my_name<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this> = ();
    }
    impl<T, U> MyMessageTrait for (T, U)
    where
        T: MyMessageTrait,
        U: MyMessageTrait,
    {
        fn my_number<'this>(&'this self) -> i32 {
            let right = <U as MyMessageTrait>::my_number(&self.1);
            if right != ::std::default::Default::default() {
                right
            } else {
                <T as MyMessageTrait>::my_number(&self.0)
            }
        }
        type Field2StringType<'this> = ::puroro::Either<
            <T as MyMessageTrait>::Field2StringType<'this>,
            <U as MyMessageTrait>::Field2StringType<'this>,
        >;
        type Field2RepeatedType<'this> = ::puroro::internal::impls::merged::MergedRepeatedLDField<
            <T as MyMessageTrait>::Field2RepeatedType<'this>,
            <U as MyMessageTrait>::Field2RepeatedType<'this>,
        >;

        fn my_name<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedLDField::new(
                <T as MyMessageTrait>::my_name(&self.0),
                <U as MyMessageTrait>::my_name(&self.1),
            )
        }
        type Field3MessageType<'this> = (
            ::std::option::Option<<T as MyMessageTrait>::Field3MessageType<'this>>,
            ::std::option::Option<<U as MyMessageTrait>::Field3MessageType<'this>>,
        );
        fn my_child<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            match (
                <T as MyMessageTrait>::my_child(&self.0),
                <U as MyMessageTrait>::my_child(&self.1),
            ) {
                (None, None) => None,
                (Some(t), None) => Some((Some(t), None)),
                (None, Some(u)) => Some((None, Some(u))),
                (Some(t), Some(u)) => Some((Some(t), Some(u))),
            }
        }
    }
    impl<T, U> MyMessageTrait for ::puroro::Either<T, U>
    where
        T: MyMessageTrait,
        U: MyMessageTrait,
    {
        fn my_number<'this>(&'this self) -> i32 {
            self.as_ref().either(
                |t| <T as MyMessageTrait>::my_number(t),
                |u| <U as MyMessageTrait>::my_number(u),
            )
        }
        type Field2StringType<'this> = ::puroro::Either<
            <T as MyMessageTrait>::Field2StringType<'this>,
            <U as MyMessageTrait>::Field2StringType<'this>,
        >;
        type Field2RepeatedType<'this> = ::puroro::internal::impls::either::EitherRepeatedLDField<
            <T as MyMessageTrait>::Field2RepeatedType<'this>,
            <U as MyMessageTrait>::Field2RepeatedType<'this>,
        >;

        fn my_name<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedLDField::new(
                self.as_ref()
                    .map_left(|t| <T as MyMessageTrait>::my_name(t))
                    .map_right(|u| <U as MyMessageTrait>::my_name(u)),
            )
        }
        type Field3MessageType<'this> = ::puroro::Either<
            <T as MyMessageTrait>::Field3MessageType<'this>,
            <U as MyMessageTrait>::Field3MessageType<'this>,
        >;
        fn my_child<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            self.as_ref().either(
                |t| <T as MyMessageTrait>::my_child(t).map(|t| ::puroro::Either::Left(t)),
                |u| <U as MyMessageTrait>::my_child(u).map(|u| ::puroro::Either::Right(u)),
            )
        }
    }
    impl<T> MyMessageTrait for ::std::option::Option<T>
    where
        T: MyMessageTrait,
    {
        fn my_number<'this>(&'this self) -> i32 {
            self.as_ref()
                .map_or_else(::std::default::Default::default, |msg| msg.my_number())
        }
        type Field2StringType<'this> = T::Field2StringType<'this>;
        type Field2RepeatedType<'this> = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field2RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn my_name<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.my_name().into_iter())
                .into_iter()
                .flatten()
        }
        type Field3MessageType<'this> = T::Field3MessageType<'this>;
        fn my_child<'this>(&'this self) -> ::std::option::Option<Self::Field3MessageType<'this>> {
            self.as_ref().and_then(|msg| msg.my_child())
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct MyMessageSimpleField1 {
        pub my_number: i32,
    }

    impl ::puroro::Message<MyMessageSimple> for MyMessageSimpleField1 {}

    impl super::_puroro_traits::MyMessageTrait for MyMessageSimpleField1 {
        fn my_number<'this>(&'this self) -> i32 {
            Clone::clone(&self.my_number)
        }
        type Field2StringType<'this> = &'static str;
        type Field2RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2StringType<'this>>;
        fn my_name<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this> = ();
    }

    impl ::puroro::SerToIoWrite for MyMessageSimpleField1 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Unlabeled, ::puroro::tags::Int32>::ser_field(
                &self.my_number,
                1,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<i32> for MyMessageSimpleField1 {
        fn from(value: i32) -> Self {
            Self { my_number: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct MyMessageSimpleField2 {
        pub my_name: ::std::vec::Vec<::std::borrow::Cow<'static, str>>,
    }

    impl ::puroro::Message<MyMessageSimple> for MyMessageSimpleField2 {}

    impl super::_puroro_traits::MyMessageTrait for MyMessageSimpleField2 {
        fn my_number<'this>(&'this self) -> i32 {
            Default::default()
        }
        type Field2StringType<'this> = &'this str;
        type Field2RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            str,
            ::std::slice::Iter<'this, ::std::borrow::Cow<'static, str>>,
        >;

        fn my_name<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.my_name.iter())
        }
        type Field3MessageType<'this> = ();
    }

    impl ::puroro::SerToIoWrite for MyMessageSimpleField2 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::String>::ser_field(
                &self.my_name,
                2,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::vec::Vec<::std::borrow::Cow<'static, str>>>
        for MyMessageSimpleField2
    {
        fn from(value: ::std::vec::Vec<::std::borrow::Cow<'static, str>>) -> Self {
            Self { my_name: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct MyMessageSimpleField3 {
        pub my_child: ::std::option::Option<
            ::std::boxed::Box<self::_puroro_root::_puroro_impls::MyMessageSimple>,
        >,
    }

    impl ::puroro::Message<MyMessageSimple> for MyMessageSimpleField3 {}

    impl super::_puroro_traits::MyMessageTrait for MyMessageSimpleField3 {
        fn my_number<'this>(&'this self) -> i32 {
            Default::default()
        }
        type Field2StringType<'this> = &'static str;
        type Field2RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2StringType<'this>>;
        fn my_name<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this> = &'this self::_puroro_root::_puroro_impls::MyMessageSimple;
        fn my_child<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            self.my_child.as_ref().map(|v| v.as_ref())
        }
    }

    impl ::puroro::SerToIoWrite for MyMessageSimpleField3 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Message<self::_puroro_root::_puroro_impls::MyMessageSimple>,
            >::ser_field(&self.my_child, 3, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl
        ::std::convert::From<
            ::std::option::Option<
                ::std::boxed::Box<self::_puroro_root::_puroro_impls::MyMessageSimple>,
            >,
        > for MyMessageSimpleField3
    {
        fn from(
            value: ::std::option::Option<
                ::std::boxed::Box<self::_puroro_root::_puroro_impls::MyMessageSimple>,
            >,
        ) -> Self {
            Self { my_child: value }
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct MyMessageSimpleByValue {}
    impl ::puroro::Message<MyMessageSimple> for MyMessageSimpleByValue {}

    impl MyMessageTrait for MyMessageSimpleByValue {
        fn my_number<'this>(&'this self) -> i32 {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field2StringType<'this> = ::std::borrow::Cow<'this, str>;
        type Field2RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2StringType<'this>>;
        fn my_name<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field3MessageType<'this> =
            ::std::boxed::Box<self::_puroro_root::_puroro_impls::MyMessageSimple>;
        fn my_child<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
    }
    pub struct MyMessageBuilder<T>(T);

    impl<T> MyMessageBuilder<T>
    where
        T: MyMessageTrait,
    {
        pub fn append_my_number(self, value: i32) -> MyMessageBuilder<(T, MyMessageSimpleField1)> {
            MyMessageBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_my_name(
            self,
            value: ::std::vec::Vec<::std::borrow::Cow<'static, str>>,
        ) -> MyMessageBuilder<(T, MyMessageSimpleField2)> {
            MyMessageBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_my_child(
            self,
            value: ::std::option::Option<
                ::std::boxed::Box<self::_puroro_root::_puroro_impls::MyMessageSimple>,
            >,
        ) -> MyMessageBuilder<(T, MyMessageSimpleField3)> {
            MyMessageBuilder((self.0, ::std::convert::From::from(value)))
        }

        pub fn build(self) -> T {
            self.0
        }
    }

    impl MyMessageBuilder<()> {
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

    pub trait MyMessageTrait {
        fn my_number<'this>(&'this self) -> i32;
        type Field2StringType<'this>: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug;
        type Field2RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field2StringType<'this>>;
        fn my_name<'this>(&'this self) -> Self::Field2RepeatedType<'this>;
        type Field3MessageType<'this>: self::_puroro_root::_puroro_traits::MyMessageTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug;
        fn my_child<'this>(&'this self) -> ::std::option::Option<Self::Field3MessageType<'this>> {
            ::std::default::Default::default()
        }
    }

    macro_rules! my_message_delegate {
        ($ty:ty) => {
            fn my_number<'this>(&'this self) -> i32 {
                (**self).my_number()
            }
            type Field2StringType<'this> = <$ty>::Field2StringType<'this>;
            type Field2RepeatedType<'this> = <$ty>::Field2RepeatedType<'this>;
            fn my_name<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
                (**self).my_name()
            }
            type Field3MessageType<'this> = <$ty>::Field3MessageType<'this>;
            fn my_child<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field3MessageType<'this>> {
                (**self).my_child()
            }
        };
    }

    impl<T> MyMessageTrait for &'_ T
    where
        T: MyMessageTrait,
    {
        my_message_delegate!(T);
    }

    impl<T> MyMessageTrait for ::std::boxed::Box<T>
    where
        T: MyMessageTrait,
    {
        my_message_delegate!(T);
    }
}
pub use _puroro_nested::*;
pub mod _puroro_nested {
    pub mod my_message {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
    }
}
