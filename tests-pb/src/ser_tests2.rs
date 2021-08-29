// A generated source code by puroro library
// package ser_tests2

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub use _puroro_impls::MsgSimple as Msg;
pub mod _puroro_impls {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct MsgSimple {
        pub i32_optional: ::std::option::Option<i32>,
        pub i32_repeated: ::std::vec::Vec<i32>,
        pub float_optional: ::std::option::Option<f32>,
        pub float_repeated: ::std::vec::Vec<f32>,
        pub string_optional: ::std::option::Option<::std::string::String>,
        pub string_repeated: ::std::vec::Vec<::std::string::String>,
        pub submsg_optional: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple,
            >,
        >,
        pub submsg_repeated: ::std::vec::Vec<
            self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple,
        >,
        pub enum_optional: ::std::option::Option<self::_puroro_root::ser_tests2::Enum>,
        pub enum_repeated: ::std::vec::Vec<self::_puroro_root::ser_tests2::Enum>,
        pub very_large_field_number: ::std::option::Option<i32>,
    }
    impl ::puroro::Message for MsgSimple {}

    impl super::_puroro_traits::MsgTrait for MsgSimple {
        fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::clone::Clone::clone(&self.i32_optional)
        }
        type Field2RepeatedType<'this> = ::std::iter::Cloned<::std::slice::Iter<'this, i32>>;

        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            self.i32_repeated.iter().cloned()
        }
        fn float_optional<'this>(&'this self) -> ::std::option::Option<f32> {
            ::std::clone::Clone::clone(&self.float_optional)
        }
        type Field4RepeatedType<'this> = ::std::iter::Cloned<::std::slice::Iter<'this, f32>>;

        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            self.float_repeated.iter().cloned()
        }
        fn string_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            self.string_optional
                .as_ref()
                .map(|v| ::std::borrow::Cow::Borrowed(v.as_ref()))
        }
        type Field6RepeatedType<'this> = ::puroro_internal::impls::simple::CowedIter<
            str,
            ::std::slice::Iter<'this, ::std::string::String>,
        >;

        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::simple::CowedIter::new(self.string_repeated.iter())
        }
        type Field7MessageType<'this> =
            self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple;
        fn submsg_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field7MessageType<'this>>>
        {
            self.submsg_optional
                .as_ref()
                .map(|boxed| ::std::borrow::Cow::Borrowed(boxed.as_ref()))
        }
        type Field8MessageType<'this> =
            self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple;
        type Field8RepeatedType<'this> = ::puroro_internal::impls::simple::CowedIter<
            self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple,
            >,
        >;

        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro_internal::impls::simple::CowedIter::new(self.submsg_repeated.iter())
        }
        fn enum_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::ser_tests2::Enum> {
            ::std::clone::Clone::clone(&self.enum_optional)
        }
        type Field10RepeatedType<'this> =
            ::std::iter::Cloned<::std::slice::Iter<'this, self::_puroro_root::ser_tests2::Enum>>;

        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            self.enum_repeated.iter().cloned()
        }
        fn very_large_field_number<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::clone::Clone::clone(&self.very_large_field_number)
        }
    }

    impl ::puroro::DeserFromBytesIter for MsgSimple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for MsgSimple {
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
                ::puroro::tags::Optional, ::puroro::tags::Int32
            >::deser_field(&mut self.i32_optional, data),
            2 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Int32
            >::deser_field(&mut self.i32_repeated, data),
            3 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Float
            >::deser_field(&mut self.float_optional, data),
            4 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Float
            >::deser_field(&mut self.float_repeated, data),
            5 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.string_optional, data),
            6 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::String
            >::deser_field(&mut self.string_repeated, data),
            7 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Message<self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple>
            >::deser_field(&mut self.submsg_optional, data),
            8 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple>
            >::deser_field(&mut self.submsg_repeated, data),
            9 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Enum2<self::_puroro_root::ser_tests2::Enum>
            >::deser_field(&mut self.enum_optional, data),
            10 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Enum2<self::_puroro_root::ser_tests2::Enum>
            >::deser_field(&mut self.enum_repeated, data),
            536870911 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Int32
            >::deser_field(&mut self.very_large_field_number, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerToIoWrite for MsgSimple {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >::ser_field(&self.i32_optional, 1, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Int32,
            >::ser_field(&self.i32_repeated, 2, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Float,
            >::ser_field(&self.float_optional, 3, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Float,
            >::ser_field(&self.float_repeated, 4, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.string_optional, 5, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::String,
            >::ser_field(&self.string_repeated, 6, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
            ::puroro::tags::Optional, ::puroro::tags::Message<self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple>
        >::ser_field(&self.submsg_optional, 7, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple>
        >::ser_field(&self.submsg_repeated, 8, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Enum2<self::_puroro_root::ser_tests2::Enum>,
            >::ser_field(&self.enum_optional, 9, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Enum2<self::_puroro_root::ser_tests2::Enum>,
            >::ser_field(&self.enum_repeated, 10, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >::ser_field(&self.very_large_field_number, 536870911, out)?;
            ::std::result::Result::Ok(())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct MsgEmpty;

    impl ::puroro::Message for MsgEmpty {}

    impl super::_puroro_traits::MsgTrait for MsgEmpty {
        fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
            None
        }
        type Field2RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_optional<'this>(&'this self) -> ::std::option::Option<f32> {
            None
        }
        type Field4RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn string_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            None
        }
        type Field6RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<::std::borrow::Cow<'this, str>>;
        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> =
            self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        fn submsg_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field7MessageType<'this>>>
        {
            None
        }
        type Field8MessageType<'this> =
            self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field8RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            ::std::borrow::Cow<'this, Self::Field8MessageType<'this>>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::ser_tests2::Enum> {
            None
        }
        type Field10RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::ser_tests2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn very_large_field_number<'this>(&'this self) -> ::std::option::Option<i32> {
            None
        }
    }

    impl ::puroro::SerToIoWrite for MsgEmpty {
        fn ser<W>(&self, _out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::std::result::Result::Ok(())
        }
    }
    pub struct MsgMerged<T, U> {
        t: T,
        u: U,
    }

    impl<T, U> MsgMerged<T, U> {
        pub fn new(t: T, u: U) -> Self {
            Self { t, u }
        }
    }

    impl<T, U> ::puroro::Message for MsgMerged<T, U> {}

    /*
    impl<T, U> super::_puroro_traits::MsgTrait for MsgMerged<T, U>
    where
        T: super::_puroro_traits::MsgTrait,
        U: super::_puroro_traits::MsgTrait,
    {
    }
    */
    impl<T, U> super::_puroro_traits::MsgTrait for ::puroro::Either<T, U>
    where
        T: super::_puroro_traits::MsgTrait,
        U: super::_puroro_traits::MsgTrait,
    {
        fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().either(
                <T as super::_puroro_traits::MsgTrait>::i32_optional,
                <U as super::_puroro_traits::MsgTrait>::i32_optional,
            )
        }
        type Field2RepeatedType<'this> = ::puroro_internal::impls::either::EitherRepeatedField<
            <T as super::_puroro_traits::MsgTrait>::Field2RepeatedType<'this>,
            <U as super::_puroro_traits::MsgTrait>::Field2RepeatedType<'this>,
        >;

        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(<T as super::_puroro_traits::MsgTrait>::i32_repeated)
                    .map_right(<U as super::_puroro_traits::MsgTrait>::i32_repeated),
            )
        }
        fn float_optional<'this>(&'this self) -> ::std::option::Option<f32> {
            self.as_ref().either(
                <T as super::_puroro_traits::MsgTrait>::float_optional,
                <U as super::_puroro_traits::MsgTrait>::float_optional,
            )
        }
        type Field4RepeatedType<'this> = ::puroro_internal::impls::either::EitherRepeatedField<
            <T as super::_puroro_traits::MsgTrait>::Field4RepeatedType<'this>,
            <U as super::_puroro_traits::MsgTrait>::Field4RepeatedType<'this>,
        >;

        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(<T as super::_puroro_traits::MsgTrait>::float_repeated)
                    .map_right(<U as super::_puroro_traits::MsgTrait>::float_repeated),
            )
        }
        fn string_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            self.as_ref().either(
                <T as super::_puroro_traits::MsgTrait>::string_optional,
                <U as super::_puroro_traits::MsgTrait>::string_optional,
            )
        }
        type Field6RepeatedType<'this> = ::puroro_internal::impls::either::EitherRepeatedField<
            <T as super::_puroro_traits::MsgTrait>::Field6RepeatedType<'this>,
            <U as super::_puroro_traits::MsgTrait>::Field6RepeatedType<'this>,
        >;

        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(<T as super::_puroro_traits::MsgTrait>::string_repeated)
                    .map_right(<U as super::_puroro_traits::MsgTrait>::string_repeated),
            )
        }
        type Field7MessageType<'this> = ::puroro::Either<
            ::std::borrow::Cow<
                'this,
                <T as super::_puroro_traits::MsgTrait>::Field7MessageType<'this>,
            >,
            ::std::borrow::Cow<
                'this,
                <U as super::_puroro_traits::MsgTrait>::Field7MessageType<'this>,
            >,
        >;
        fn submsg_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field7MessageType<'this>>>
        {
            todo!()
        }
        type Field8MessageType<'this> = ::puroro::Either<
            ::std::borrow::Cow<
                'this,
                <T as super::_puroro_traits::MsgTrait>::Field8MessageType<'this>,
            >,
            ::std::borrow::Cow<
                'this,
                <U as super::_puroro_traits::MsgTrait>::Field8MessageType<'this>,
            >,
        >;
        type Field8RepeatedType<'this> =
            ::puroro_internal::impls::either::EitherRepeatedMessageField<
                <T as super::_puroro_traits::MsgTrait>::Field8RepeatedType<'this>,
                <U as super::_puroro_traits::MsgTrait>::Field8RepeatedType<'this>,
            >;

        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(<T as super::_puroro_traits::MsgTrait>::submsg_repeated)
                    .map_right(<U as super::_puroro_traits::MsgTrait>::submsg_repeated),
            )
        }
        fn enum_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::ser_tests2::Enum> {
            self.as_ref().either(
                <T as super::_puroro_traits::MsgTrait>::enum_optional,
                <U as super::_puroro_traits::MsgTrait>::enum_optional,
            )
        }
        type Field10RepeatedType<'this> = ::puroro_internal::impls::either::EitherRepeatedField<
            <T as super::_puroro_traits::MsgTrait>::Field10RepeatedType<'this>,
            <U as super::_puroro_traits::MsgTrait>::Field10RepeatedType<'this>,
        >;

        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(<T as super::_puroro_traits::MsgTrait>::enum_repeated)
                    .map_right(<U as super::_puroro_traits::MsgTrait>::enum_repeated),
            )
        }
        fn very_large_field_number<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().either(
                <T as super::_puroro_traits::MsgTrait>::very_large_field_number,
                <U as super::_puroro_traits::MsgTrait>::very_large_field_number,
            )
        }
    }

    impl<'a, T> super::_puroro_traits::MsgTrait for ::std::borrow::Cow<'a, T>
    where
        T: 'a + ::std::clone::Clone + super::_puroro_traits::MsgTrait,
    {
        fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
            use std::ops::Deref;
            self.deref().i32_optional()
        }
        type Field2RepeatedType<'this> =
            <T as super::_puroro_traits::MsgTrait>::Field2RepeatedType<'this>;

        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            use std::ops::Deref;
            self.deref().i32_repeated()
        }
        fn float_optional<'this>(&'this self) -> ::std::option::Option<f32> {
            use std::ops::Deref;
            self.deref().float_optional()
        }
        type Field4RepeatedType<'this> =
            <T as super::_puroro_traits::MsgTrait>::Field4RepeatedType<'this>;

        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            use std::ops::Deref;
            self.deref().float_repeated()
        }
        fn string_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            use std::ops::Deref;
            self.deref().string_optional()
        }
        type Field6RepeatedType<'this> =
            <T as super::_puroro_traits::MsgTrait>::Field6RepeatedType<'this>;

        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            use std::ops::Deref;
            self.deref().string_repeated()
        }
        type Field7MessageType<'this> =
            <T as super::_puroro_traits::MsgTrait>::Field7MessageType<'this>;
        fn submsg_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field7MessageType<'this>>>
        {
            use std::ops::Deref;
            self.deref().submsg_optional()
        }
        type Field8MessageType<'this> =
            <T as super::_puroro_traits::MsgTrait>::Field8MessageType<'this>;
        type Field8RepeatedType<'this> =
            <T as super::_puroro_traits::MsgTrait>::Field8RepeatedType<'this>;

        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            use std::ops::Deref;
            self.deref().submsg_repeated()
        }
        fn enum_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::ser_tests2::Enum> {
            use std::ops::Deref;
            self.deref().enum_optional()
        }
        type Field10RepeatedType<'this> =
            <T as super::_puroro_traits::MsgTrait>::Field10RepeatedType<'this>;

        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            use std::ops::Deref;
            self.deref().enum_repeated()
        }
        fn very_large_field_number<'this>(&'this self) -> ::std::option::Option<i32> {
            use std::ops::Deref;
            self.deref().very_large_field_number()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField1 {
        i32_optional: ::std::option::Option<i32>,
    }

    impl ::puroro::Message for MsgSimpleField1 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField1 {
        fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::clone::Clone::clone(&self.i32_optional)
        }
        type Field2RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_optional<'this>(&'this self) -> ::std::option::Option<f32> {
            None
        }
        type Field4RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn string_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            None
        }
        type Field6RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<::std::borrow::Cow<'this, str>>;
        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> =
            self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        fn submsg_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field7MessageType<'this>>>
        {
            None
        }
        type Field8MessageType<'this> =
            self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field8RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            ::std::borrow::Cow<'this, Self::Field8MessageType<'this>>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::ser_tests2::Enum> {
            None
        }
        type Field10RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::ser_tests2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn very_large_field_number<'this>(&'this self) -> ::std::option::Option<i32> {
            None
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField2 {
        i32_repeated: ::std::vec::Vec<i32>,
    }

    impl ::puroro::Message for MsgSimpleField2 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField2 {
        fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
            None
        }
        type Field2RepeatedType<'this> = ::std::iter::Cloned<::std::slice::Iter<'this, i32>>;

        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            self.i32_repeated.iter().cloned()
        }
        fn float_optional<'this>(&'this self) -> ::std::option::Option<f32> {
            None
        }
        type Field4RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn string_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            None
        }
        type Field6RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<::std::borrow::Cow<'this, str>>;
        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> =
            self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        fn submsg_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field7MessageType<'this>>>
        {
            None
        }
        type Field8MessageType<'this> =
            self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field8RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            ::std::borrow::Cow<'this, Self::Field8MessageType<'this>>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::ser_tests2::Enum> {
            None
        }
        type Field10RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::ser_tests2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn very_large_field_number<'this>(&'this self) -> ::std::option::Option<i32> {
            None
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField3 {
        float_optional: ::std::option::Option<f32>,
    }

    impl ::puroro::Message for MsgSimpleField3 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField3 {
        fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
            None
        }
        type Field2RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_optional<'this>(&'this self) -> ::std::option::Option<f32> {
            ::std::clone::Clone::clone(&self.float_optional)
        }
        type Field4RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn string_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            None
        }
        type Field6RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<::std::borrow::Cow<'this, str>>;
        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> =
            self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        fn submsg_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field7MessageType<'this>>>
        {
            None
        }
        type Field8MessageType<'this> =
            self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field8RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            ::std::borrow::Cow<'this, Self::Field8MessageType<'this>>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::ser_tests2::Enum> {
            None
        }
        type Field10RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::ser_tests2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn very_large_field_number<'this>(&'this self) -> ::std::option::Option<i32> {
            None
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField4 {
        float_repeated: ::std::vec::Vec<f32>,
    }

    impl ::puroro::Message for MsgSimpleField4 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField4 {
        fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
            None
        }
        type Field2RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_optional<'this>(&'this self) -> ::std::option::Option<f32> {
            None
        }
        type Field4RepeatedType<'this> = ::std::iter::Cloned<::std::slice::Iter<'this, f32>>;

        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            self.float_repeated.iter().cloned()
        }
        fn string_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            None
        }
        type Field6RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<::std::borrow::Cow<'this, str>>;
        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> =
            self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        fn submsg_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field7MessageType<'this>>>
        {
            None
        }
        type Field8MessageType<'this> =
            self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field8RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            ::std::borrow::Cow<'this, Self::Field8MessageType<'this>>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::ser_tests2::Enum> {
            None
        }
        type Field10RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::ser_tests2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn very_large_field_number<'this>(&'this self) -> ::std::option::Option<i32> {
            None
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField5 {
        string_optional: ::std::option::Option<::std::string::String>,
    }

    impl ::puroro::Message for MsgSimpleField5 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField5 {
        fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
            None
        }
        type Field2RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_optional<'this>(&'this self) -> ::std::option::Option<f32> {
            None
        }
        type Field4RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn string_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            self.string_optional
                .as_ref()
                .map(|v| ::std::borrow::Cow::Borrowed(v.as_ref()))
        }
        type Field6RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<::std::borrow::Cow<'this, str>>;
        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> =
            self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        fn submsg_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field7MessageType<'this>>>
        {
            None
        }
        type Field8MessageType<'this> =
            self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field8RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            ::std::borrow::Cow<'this, Self::Field8MessageType<'this>>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::ser_tests2::Enum> {
            None
        }
        type Field10RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::ser_tests2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn very_large_field_number<'this>(&'this self) -> ::std::option::Option<i32> {
            None
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField6 {
        string_repeated: ::std::vec::Vec<::std::string::String>,
    }

    impl ::puroro::Message for MsgSimpleField6 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField6 {
        fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
            None
        }
        type Field2RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_optional<'this>(&'this self) -> ::std::option::Option<f32> {
            None
        }
        type Field4RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn string_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            None
        }
        type Field6RepeatedType<'this> = ::puroro_internal::impls::simple::CowedIter<
            str,
            ::std::slice::Iter<'this, ::std::string::String>,
        >;

        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::simple::CowedIter::new(self.string_repeated.iter())
        }
        type Field7MessageType<'this> =
            self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        fn submsg_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field7MessageType<'this>>>
        {
            None
        }
        type Field8MessageType<'this> =
            self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field8RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            ::std::borrow::Cow<'this, Self::Field8MessageType<'this>>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::ser_tests2::Enum> {
            None
        }
        type Field10RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::ser_tests2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn very_large_field_number<'this>(&'this self) -> ::std::option::Option<i32> {
            None
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField7 {
        submsg_optional: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple,
            >,
        >,
    }

    impl ::puroro::Message for MsgSimpleField7 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField7 {
        fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
            None
        }
        type Field2RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_optional<'this>(&'this self) -> ::std::option::Option<f32> {
            None
        }
        type Field4RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn string_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            None
        }
        type Field6RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<::std::borrow::Cow<'this, str>>;
        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> =
            self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple;
        fn submsg_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field7MessageType<'this>>>
        {
            self.submsg_optional
                .as_ref()
                .map(|boxed| ::std::borrow::Cow::Borrowed(boxed.as_ref()))
        }
        type Field8MessageType<'this> =
            self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field8RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            ::std::borrow::Cow<'this, Self::Field8MessageType<'this>>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::ser_tests2::Enum> {
            None
        }
        type Field10RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::ser_tests2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn very_large_field_number<'this>(&'this self) -> ::std::option::Option<i32> {
            None
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField8 {
        submsg_repeated: ::std::vec::Vec<
            self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple,
        >,
    }

    impl ::puroro::Message for MsgSimpleField8 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField8 {
        fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
            None
        }
        type Field2RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_optional<'this>(&'this self) -> ::std::option::Option<f32> {
            None
        }
        type Field4RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn string_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            None
        }
        type Field6RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<::std::borrow::Cow<'this, str>>;
        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> =
            self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        fn submsg_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field7MessageType<'this>>>
        {
            None
        }
        type Field8MessageType<'this> =
            self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple;
        type Field8RepeatedType<'this> = ::puroro_internal::impls::simple::CowedIter<
            self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgSimple,
            >,
        >;

        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro_internal::impls::simple::CowedIter::new(self.submsg_repeated.iter())
        }
        fn enum_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::ser_tests2::Enum> {
            None
        }
        type Field10RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::ser_tests2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn very_large_field_number<'this>(&'this self) -> ::std::option::Option<i32> {
            None
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField9 {
        enum_optional: ::std::option::Option<self::_puroro_root::ser_tests2::Enum>,
    }

    impl ::puroro::Message for MsgSimpleField9 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField9 {
        fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
            None
        }
        type Field2RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_optional<'this>(&'this self) -> ::std::option::Option<f32> {
            None
        }
        type Field4RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn string_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            None
        }
        type Field6RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<::std::borrow::Cow<'this, str>>;
        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> =
            self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        fn submsg_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field7MessageType<'this>>>
        {
            None
        }
        type Field8MessageType<'this> =
            self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field8RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            ::std::borrow::Cow<'this, Self::Field8MessageType<'this>>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::ser_tests2::Enum> {
            ::std::clone::Clone::clone(&self.enum_optional)
        }
        type Field10RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::ser_tests2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn very_large_field_number<'this>(&'this self) -> ::std::option::Option<i32> {
            None
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField10 {
        enum_repeated: ::std::vec::Vec<self::_puroro_root::ser_tests2::Enum>,
    }

    impl ::puroro::Message for MsgSimpleField10 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField10 {
        fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
            None
        }
        type Field2RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_optional<'this>(&'this self) -> ::std::option::Option<f32> {
            None
        }
        type Field4RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn string_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            None
        }
        type Field6RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<::std::borrow::Cow<'this, str>>;
        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> =
            self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        fn submsg_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field7MessageType<'this>>>
        {
            None
        }
        type Field8MessageType<'this> =
            self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field8RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            ::std::borrow::Cow<'this, Self::Field8MessageType<'this>>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::ser_tests2::Enum> {
            None
        }
        type Field10RepeatedType<'this> =
            ::std::iter::Cloned<::std::slice::Iter<'this, self::_puroro_root::ser_tests2::Enum>>;

        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            self.enum_repeated.iter().cloned()
        }
        fn very_large_field_number<'this>(&'this self) -> ::std::option::Option<i32> {
            None
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField536870911 {
        very_large_field_number: ::std::option::Option<i32>,
    }

    impl ::puroro::Message for MsgSimpleField536870911 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField536870911 {
        fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
            None
        }
        type Field2RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_optional<'this>(&'this self) -> ::std::option::Option<f32> {
            None
        }
        type Field4RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn string_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            None
        }
        type Field6RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<::std::borrow::Cow<'this, str>>;
        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> =
            self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        fn submsg_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field7MessageType<'this>>>
        {
            None
        }
        type Field8MessageType<'this> =
            self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_impls::SubmsgEmpty;
        type Field8RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            ::std::borrow::Cow<'this, Self::Field8MessageType<'this>>,
        >;
        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::ser_tests2::Enum> {
            None
        }
        type Field10RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::ser_tests2::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn very_large_field_number<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::clone::Clone::clone(&self.very_large_field_number)
        }
    }
}
pub use _puroro_traits::*;
pub mod _puroro_traits {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }

    pub trait MsgTrait: ::std::clone::Clone {
        fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::default::Default::default()
        }
        type Field2RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            todo!()
        }
        fn float_optional<'this>(&'this self) -> ::std::option::Option<f32> {
            ::std::default::Default::default()
        }
        type Field4RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            todo!()
        }
        fn string_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            ::std::default::Default::default()
        }
        type Field6RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = ::std::borrow::Cow<'this, str>>;
        fn string_repeated<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            todo!()
        }
        type Field7MessageType<'this>: 'this + self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_traits::SubmsgTrait;
        fn submsg_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field7MessageType<'this>>>
        {
            ::std::default::Default::default()
        }
        type Field8MessageType<'this>: 'this + self::_puroro_root::ser_tests2::_puroro_nested::msg::_puroro_traits::SubmsgTrait;
        type Field8RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<
                Item = ::std::borrow::Cow<'this, Self::Field8MessageType<'this>>,
            >;
        fn submsg_repeated<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            todo!()
        }
        fn enum_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::ser_tests2::Enum> {
            ::std::default::Default::default()
        }
        type Field10RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = self::_puroro_root::ser_tests2::Enum>;
        fn enum_repeated<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            todo!()
        }
        fn very_large_field_number<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::default::Default::default()
        }
    }
}
#[derive(::std::fmt::Debug, ::std::clone::Clone, ::std::cmp::PartialEq)]
pub enum Enum {
    Zeroth,
    First,
    Tenth,
}

impl ::puroro::Enum2 for Enum {}
impl ::std::convert::TryFrom<i32> for Enum {
    type Error = i32;
    fn try_from(value: i32) -> ::std::result::Result<Self, i32> {
        ::std::result::Result::Ok(match value {
            0 => Enum::Zeroth,
            1 => Enum::First,
            10 => Enum::Tenth,
            _ => Err(value)?,
        })
    }
}

impl ::std::convert::From<Enum> for i32 {
    fn from(value: Enum) -> i32 {
        match value {
            Enum::Zeroth => 0,
            Enum::First => 1,
            Enum::Tenth => 10,
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

        pub use _puroro_impls::SubmsgSimple as Submsg;
        pub mod _puroro_impls {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }
            #[derive(
                ::std::clone::Clone,
                ::std::default::Default,
                ::std::cmp::PartialEq,
                ::std::fmt::Debug,
            )]
            pub struct SubmsgSimple {
                pub i32_optional: ::std::option::Option<i32>,
            }
            impl ::puroro::Message for SubmsgSimple {}

            impl super::_puroro_traits::SubmsgTrait for SubmsgSimple {
                fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
                    ::std::clone::Clone::clone(&self.i32_optional)
                }
            }

            impl ::puroro::DeserFromBytesIter for SubmsgSimple {
                fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
                }
            }

            impl ::puroro_internal::de::DeserFieldsFromBytesIter for SubmsgSimple {
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
                            ::puroro::tags::Optional,
                            ::puroro::tags::Int32,
                        >::deser_field(&mut self.i32_optional, data),

                        _ => unimplemented!("TODO: This case should be handled properly..."),
                    }
                }
            }

            impl ::puroro::SerToIoWrite for SubmsgSimple {
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
            #[derive(
                ::std::clone::Clone,
                ::std::default::Default,
                ::std::cmp::PartialEq,
                ::std::fmt::Debug,
            )]
            pub struct SubmsgEmpty;

            impl ::puroro::Message for SubmsgEmpty {}

            impl super::_puroro_traits::SubmsgTrait for SubmsgEmpty {
                fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
                    None
                }
            }

            impl ::puroro::SerToIoWrite for SubmsgEmpty {
                fn ser<W>(&self, _out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    ::std::result::Result::Ok(())
                }
            }
            pub struct SubmsgMerged<T, U> {
                t: T,
                u: U,
            }

            impl<T, U> SubmsgMerged<T, U> {
                pub fn new(t: T, u: U) -> Self {
                    Self { t, u }
                }
            }

            impl<T, U> ::puroro::Message for SubmsgMerged<T, U> {}

            /*
            impl<T, U> super::_puroro_traits::SubmsgTrait for SubmsgMerged<T, U>
            where
                T: super::_puroro_traits::SubmsgTrait,
                U: super::_puroro_traits::SubmsgTrait,
            {
            }
            */
            impl<T, U> super::_puroro_traits::SubmsgTrait for ::puroro::Either<T, U>
            where
                T: super::_puroro_traits::SubmsgTrait,
                U: super::_puroro_traits::SubmsgTrait,
            {
                fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
                    self.as_ref().either(
                        <T as super::_puroro_traits::SubmsgTrait>::i32_optional,
                        <U as super::_puroro_traits::SubmsgTrait>::i32_optional,
                    )
                }
            }

            impl<'a, T> super::_puroro_traits::SubmsgTrait for ::std::borrow::Cow<'a, T>
            where
                T: 'a + ::std::clone::Clone + super::_puroro_traits::SubmsgTrait,
            {
                fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
                    use std::ops::Deref;
                    self.deref().i32_optional()
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            struct SubmsgSimpleField1 {
                i32_optional: ::std::option::Option<i32>,
            }

            impl ::puroro::Message for SubmsgSimpleField1 {}

            impl super::_puroro_traits::SubmsgTrait for SubmsgSimpleField1 {
                fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
                    ::std::clone::Clone::clone(&self.i32_optional)
                }
            }
        }
        pub use _puroro_traits::*;
        pub mod _puroro_traits {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }

            pub trait SubmsgTrait: ::std::clone::Clone {
                fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
                    ::std::default::Default::default()
                }
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
