// A generated source code by puroro library
// package full_coverage3

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub use _puroro_impls::MsgSimple as Msg;
pub use _puroro_impls::SomeSimple as Some;
pub mod _puroro_impls {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
    use super::_puroro_traits::*;
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct MsgSimple {
    pub i32_unlabeled: i32,
    pub i32_optional: ::std::option::Option<i32>,
    pub i32_repeated: ::std::vec::Vec<i32>,
    pub float_unlabeled: f32,
    pub float_optional: ::std::option::Option<f32>,
    pub float_repeated: ::std::vec::Vec<f32>,
    pub string_unlabeled: ::std::borrow::Cow<'static, str>,
    pub string_optional: ::std::option::Option<::std::borrow::Cow<'static, str>>,
    pub string_repeated: ::std::vec::Vec<::std::borrow::Cow<'static, str>>,
    pub enum_unlabeled: self::_puroro_root::full_coverage3::Enum,
    pub enum_optional: ::std::option::Option<self::_puroro_root::full_coverage3::Enum>,
    pub enum_repeated: ::std::vec::Vec<self::_puroro_root::full_coverage3::Enum>,
    pub submsg_unlabeled: ::std::option::Option<::std::boxed::Box<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgSimple>>,
    pub submsg_optional: ::std::option::Option<::std::boxed::Box<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgSimple>>,
    pub submsg_repeated: ::std::vec::Vec<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgSimple>,
}
    impl ::puroro::Message for MsgSimple {}

    impl MsgTrait for MsgSimple {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Clone::clone(&self.i32_unlabeled)
        }
        fn i32_optional<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.i32_optional)
        }
        type Field3RepeatedType<'this> = ::std::iter::Cloned<::std::slice::Iter<'this, i32>>;

        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            self.i32_repeated.iter().cloned()
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            Clone::clone(&self.float_unlabeled)
        }
        fn float_optional<'this>(&'this self) -> Option<f32> {
            Clone::clone(&self.float_optional)
        }
        type Field13RepeatedType<'this> = ::std::iter::Cloned<::std::slice::Iter<'this, f32>>;

        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            self.float_repeated.iter().cloned()
        }
        type Field21StringType<'this> = &'this str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field21StringType<'this> {
            &self.string_unlabeled
        }
        type Field22StringType<'this> = &'this str;
        fn string_optional<'this>(&'this self) -> Option<Self::Field22StringType<'this>> {
            self.string_optional.as_ref().map(|v| v.as_ref())
        }
        type Field23StringType<'this> = &'this str;
        type Field23RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            str,
            ::std::slice::Iter<'this, ::std::borrow::Cow<'static, str>>,
        >;

        fn string_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.string_repeated.iter())
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::full_coverage3::Enum {
            Clone::clone(&self.enum_unlabeled)
        }
        fn enum_optional<'this>(&'this self) -> Option<self::_puroro_root::full_coverage3::Enum> {
            Clone::clone(&self.enum_optional)
        }
        type Field33RepeatedType<'this> = ::std::iter::Cloned<
            ::std::slice::Iter<'this, self::_puroro_root::full_coverage3::Enum>,
        >;

        fn enum_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            self.enum_repeated.iter().cloned()
        }
        type Field41MessageType<'this> = &'this self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgSimple;
        fn submsg_unlabeled<'this>(&'this self) -> Option<Self::Field41MessageType<'this>> {
            self.submsg_unlabeled.as_ref().map(|v| v.as_ref())
        }
        type Field42MessageType<'this> = &'this self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgSimple;
        fn submsg_optional<'this>(&'this self) -> Option<Self::Field42MessageType<'this>> {
            self.submsg_optional.as_ref().map(|v| v.as_ref())
        }
        type Field43MessageType<'this> = &'this self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgSimple;
        type Field43RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
    self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgSimple,
    ::std::slice::Iter<'this, self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgSimple>>;

        fn submsg_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.submsg_repeated.iter())
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
                ::puroro::tags::Unlabeled, ::puroro::tags::Int32
            >::deser_field(&mut self.i32_unlabeled, data),
            2 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Int32
            >::deser_field(&mut self.i32_optional, data),
            3 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Int32
            >::deser_field(&mut self.i32_repeated, data),
            11 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Unlabeled, ::puroro::tags::Float
            >::deser_field(&mut self.float_unlabeled, data),
            12 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Float
            >::deser_field(&mut self.float_optional, data),
            13 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Float
            >::deser_field(&mut self.float_repeated, data),
            21 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Unlabeled, ::puroro::tags::String
            >::deser_field(&mut self.string_unlabeled, data),
            22 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.string_optional, data),
            23 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::String
            >::deser_field(&mut self.string_repeated, data),
            31 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Unlabeled, ::puroro::tags::Enum3<self::_puroro_root::full_coverage3::Enum>
            >::deser_field(&mut self.enum_unlabeled, data),
            32 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Enum3<self::_puroro_root::full_coverage3::Enum>
            >::deser_field(&mut self.enum_optional, data),
            33 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Enum3<self::_puroro_root::full_coverage3::Enum>
            >::deser_field(&mut self.enum_repeated, data),
            41 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Unlabeled, ::puroro::tags::Message<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgSimple>
            >::deser_field(&mut self.submsg_unlabeled, data),
            42 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Message<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgSimple>
            >::deser_field(&mut self.submsg_optional, data),
            43 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgSimple>
            >::deser_field(&mut self.submsg_repeated, data),

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
            >::ser_field(&self.float_unlabeled, 11, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Float,
            >::ser_field(&self.float_optional, 12, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Float,
            >::ser_field(&self.float_repeated, 13, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::String,
            >::ser_field(&self.string_unlabeled, 21, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.string_optional, 22, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::String,
            >::ser_field(&self.string_repeated, 23, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Enum3<self::_puroro_root::full_coverage3::Enum>,
            >::ser_field(&self.enum_unlabeled, 31, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Enum3<self::_puroro_root::full_coverage3::Enum>,
            >::ser_field(&self.enum_optional, 32, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Enum3<self::_puroro_root::full_coverage3::Enum>,
            >::ser_field(&self.enum_repeated, 33, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
            ::puroro::tags::Unlabeled, ::puroro::tags::Message<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgSimple>
        >::ser_field(&self.submsg_unlabeled, 41, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
            ::puroro::tags::Optional, ::puroro::tags::Message<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgSimple>
        >::ser_field(&self.submsg_optional, 42, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgSimple>
        >::ser_field(&self.submsg_repeated, 43, out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl MsgTrait for () {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Default::default()
        }
        type Field3RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            Default::default()
        }
        type Field13RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21StringType<'this> = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field21StringType<'this> {
            ""
        }
        type Field22StringType<'this> = &'static str;
        type Field23StringType<'this> = &'static str;
        type Field23RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field23StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::full_coverage3::Enum {
            Default::default()
        }
        type Field33RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field41MessageType<'this> = ();
        type Field42MessageType<'this> = ();
        type Field43MessageType<'this> = ();
        type Field43RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field43MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }
    impl<T, U> MsgTrait for (T, U)
    where
        T: MsgTrait,
        U: MsgTrait,
    {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            let right = <U as MsgTrait>::i32_unlabeled(&self.1);
            if right != ::std::default::Default::default() {
                right
            } else {
                <T as MsgTrait>::i32_unlabeled(&self.0)
            }
        }
        fn i32_optional<'this>(&'this self) -> Option<i32> {
            <U as MsgTrait>::i32_optional(&self.1)
                .or_else(|| <T as MsgTrait>::i32_optional(&self.0))
        }
        type Field3RepeatedType<'this> = ::puroro_internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::Field3RepeatedType<'this>,
            <U as MsgTrait>::Field3RepeatedType<'this>,
        >;

        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::merged::MergedRepeatedField::new(
                <T as MsgTrait>::i32_repeated(&self.0),
                <U as MsgTrait>::i32_repeated(&self.1),
            )
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            let right = <U as MsgTrait>::float_unlabeled(&self.1);
            if right != ::std::default::Default::default() {
                right
            } else {
                <T as MsgTrait>::float_unlabeled(&self.0)
            }
        }
        fn float_optional<'this>(&'this self) -> Option<f32> {
            <U as MsgTrait>::float_optional(&self.1)
                .or_else(|| <T as MsgTrait>::float_optional(&self.0))
        }
        type Field13RepeatedType<'this> = ::puroro_internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::Field13RepeatedType<'this>,
            <U as MsgTrait>::Field13RepeatedType<'this>,
        >;

        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro_internal::impls::merged::MergedRepeatedField::new(
                <T as MsgTrait>::float_repeated(&self.0),
                <U as MsgTrait>::float_repeated(&self.1),
            )
        }
        type Field21StringType<'this> = ::puroro::Either<
            <T as MsgTrait>::Field21StringType<'this>,
            <U as MsgTrait>::Field21StringType<'this>,
        >;
        fn string_unlabeled<'this>(&'this self) -> Self::Field21StringType<'this> {
            let right = <U as MsgTrait>::string_unlabeled(&self.1);
            if !right.is_empty() {
                ::puroro::Either::Right(right)
            } else {
                ::puroro::Either::Left(<T as MsgTrait>::string_unlabeled(&self.0))
            }
        }
        type Field22StringType<'this> = ::puroro::Either<
            <T as MsgTrait>::Field22StringType<'this>,
            <U as MsgTrait>::Field22StringType<'this>,
        >;
        fn string_optional<'this>(&'this self) -> Option<Self::Field22StringType<'this>> {
            if let Some(right) = <U as MsgTrait>::string_optional(&self.1) {
                Some(::puroro::Either::Right(right))
            } else if let Some(left) = <T as MsgTrait>::string_optional(&self.0) {
                Some(::puroro::Either::Left(left))
            } else {
                None
            }
        }
        type Field23StringType<'this> = ::puroro::Either<
            <T as MsgTrait>::Field23StringType<'this>,
            <U as MsgTrait>::Field23StringType<'this>,
        >;
        type Field23RepeatedType<'this> = ::puroro_internal::impls::merged::MergedRepeatedLDField<
            <T as MsgTrait>::Field23RepeatedType<'this>,
            <U as MsgTrait>::Field23RepeatedType<'this>,
        >;

        fn string_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro_internal::impls::merged::MergedRepeatedLDField::new(
                <T as MsgTrait>::string_repeated(&self.0),
                <U as MsgTrait>::string_repeated(&self.1),
            )
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::full_coverage3::Enum {
            let right = <U as MsgTrait>::enum_unlabeled(&self.1);
            if right != ::std::default::Default::default() {
                right
            } else {
                <T as MsgTrait>::enum_unlabeled(&self.0)
            }
        }
        fn enum_optional<'this>(&'this self) -> Option<self::_puroro_root::full_coverage3::Enum> {
            <U as MsgTrait>::enum_optional(&self.1)
                .or_else(|| <T as MsgTrait>::enum_optional(&self.0))
        }
        type Field33RepeatedType<'this> = ::puroro_internal::impls::merged::MergedRepeatedField<
            <T as MsgTrait>::Field33RepeatedType<'this>,
            <U as MsgTrait>::Field33RepeatedType<'this>,
        >;

        fn enum_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro_internal::impls::merged::MergedRepeatedField::new(
                <T as MsgTrait>::enum_repeated(&self.0),
                <U as MsgTrait>::enum_repeated(&self.1),
            )
        }
        type Field41MessageType<'this> = ::puroro::Either<
            ::puroro::Either<
                <T as MsgTrait>::Field41MessageType<'this>,
                <U as MsgTrait>::Field41MessageType<'this>,
            >,
            (
                <T as MsgTrait>::Field41MessageType<'this>,
                <U as MsgTrait>::Field41MessageType<'this>,
            ),
        >;
        fn submsg_unlabeled<'this>(&'this self) -> Option<Self::Field41MessageType<'this>> {
            match (
                <T as MsgTrait>::submsg_unlabeled(&self.0),
                <U as MsgTrait>::submsg_unlabeled(&self.1),
            ) {
                (None, None) => None,
                (Some(t), None) => Some(::puroro::Either::Left(::puroro::Either::Left(t))),
                (None, Some(u)) => Some(::puroro::Either::Left(::puroro::Either::Right(u))),
                (Some(t), Some(u)) => Some(::puroro::Either::Right((t, u))),
            }
        }
        type Field42MessageType<'this> = ::puroro::Either<
            ::puroro::Either<
                <T as MsgTrait>::Field42MessageType<'this>,
                <U as MsgTrait>::Field42MessageType<'this>,
            >,
            (
                <T as MsgTrait>::Field42MessageType<'this>,
                <U as MsgTrait>::Field42MessageType<'this>,
            ),
        >;
        fn submsg_optional<'this>(&'this self) -> Option<Self::Field42MessageType<'this>> {
            match (
                <T as MsgTrait>::submsg_optional(&self.0),
                <U as MsgTrait>::submsg_optional(&self.1),
            ) {
                (None, None) => None,
                (Some(t), None) => Some(::puroro::Either::Left(::puroro::Either::Left(t))),
                (None, Some(u)) => Some(::puroro::Either::Left(::puroro::Either::Right(u))),
                (Some(t), Some(u)) => Some(::puroro::Either::Right((t, u))),
            }
        }
        type Field43MessageType<'this> = ::puroro::Either<
            <T as MsgTrait>::Field43MessageType<'this>,
            <U as MsgTrait>::Field43MessageType<'this>,
        >;
        type Field43RepeatedType<'this> =
            ::puroro_internal::impls::merged::MergedRepeatedMessageField<
                <T as MsgTrait>::Field43RepeatedType<'this>,
                <U as MsgTrait>::Field43RepeatedType<'this>,
            >;

        fn submsg_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro_internal::impls::merged::MergedRepeatedMessageField::new(
                <T as MsgTrait>::submsg_repeated(&self.0),
                <U as MsgTrait>::submsg_repeated(&self.1),
            )
        }
    }
    impl<T, U> MsgTrait for ::puroro::Either<T, U>
    where
        T: MsgTrait,
        U: MsgTrait,
    {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            self.as_ref().either(
                |t| <T as MsgTrait>::i32_unlabeled(t),
                |u| <U as MsgTrait>::i32_unlabeled(u),
            )
        }
        fn i32_optional<'this>(&'this self) -> Option<i32> {
            self.as_ref().either(
                |t| <T as MsgTrait>::i32_optional(t),
                |u| <U as MsgTrait>::i32_optional(u),
            )
        }
        type Field3RepeatedType<'this> = ::puroro_internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::Field3RepeatedType<'this>,
            <U as MsgTrait>::Field3RepeatedType<'this>,
        >;

        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::i32_repeated(t))
                    .map_right(|u| <U as MsgTrait>::i32_repeated(u)),
            )
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            self.as_ref().either(
                |t| <T as MsgTrait>::float_unlabeled(t),
                |u| <U as MsgTrait>::float_unlabeled(u),
            )
        }
        fn float_optional<'this>(&'this self) -> Option<f32> {
            self.as_ref().either(
                |t| <T as MsgTrait>::float_optional(t),
                |u| <U as MsgTrait>::float_optional(u),
            )
        }
        type Field13RepeatedType<'this> = ::puroro_internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::Field13RepeatedType<'this>,
            <U as MsgTrait>::Field13RepeatedType<'this>,
        >;

        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::float_repeated(t))
                    .map_right(|u| <U as MsgTrait>::float_repeated(u)),
            )
        }
        type Field21StringType<'this> = ::puroro::Either<
            <T as MsgTrait>::Field21StringType<'this>,
            <U as MsgTrait>::Field21StringType<'this>,
        >;
        fn string_unlabeled<'this>(&'this self) -> Self::Field21StringType<'this> {
            self.as_ref().either(
                |t| ::puroro::Either::Left(<T as MsgTrait>::string_unlabeled(t)),
                |u| ::puroro::Either::Right(<U as MsgTrait>::string_unlabeled(u)),
            )
        }
        type Field22StringType<'this> = ::puroro::Either<
            <T as MsgTrait>::Field22StringType<'this>,
            <U as MsgTrait>::Field22StringType<'this>,
        >;
        fn string_optional<'this>(&'this self) -> Option<Self::Field22StringType<'this>> {
            self.as_ref().either(
                |t| <T as MsgTrait>::string_optional(t).map(|t| ::puroro::Either::Left(t)),
                |u| <U as MsgTrait>::string_optional(u).map(|u| ::puroro::Either::Right(u)),
            )
        }
        type Field23StringType<'this> = ::puroro::Either<
            <T as MsgTrait>::Field23StringType<'this>,
            <U as MsgTrait>::Field23StringType<'this>,
        >;
        type Field23RepeatedType<'this> = ::puroro_internal::impls::either::EitherRepeatedLDField<
            <T as MsgTrait>::Field23RepeatedType<'this>,
            <U as MsgTrait>::Field23RepeatedType<'this>,
        >;

        fn string_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedLDField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::string_repeated(t))
                    .map_right(|u| <U as MsgTrait>::string_repeated(u)),
            )
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::full_coverage3::Enum {
            self.as_ref().either(
                |t| <T as MsgTrait>::enum_unlabeled(t),
                |u| <U as MsgTrait>::enum_unlabeled(u),
            )
        }
        fn enum_optional<'this>(&'this self) -> Option<self::_puroro_root::full_coverage3::Enum> {
            self.as_ref().either(
                |t| <T as MsgTrait>::enum_optional(t),
                |u| <U as MsgTrait>::enum_optional(u),
            )
        }
        type Field33RepeatedType<'this> = ::puroro_internal::impls::either::EitherRepeatedField<
            <T as MsgTrait>::Field33RepeatedType<'this>,
            <U as MsgTrait>::Field33RepeatedType<'this>,
        >;

        fn enum_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::enum_repeated(t))
                    .map_right(|u| <U as MsgTrait>::enum_repeated(u)),
            )
        }
        type Field41MessageType<'this> = ::puroro::Either<
            <T as MsgTrait>::Field41MessageType<'this>,
            <U as MsgTrait>::Field41MessageType<'this>,
        >;
        fn submsg_unlabeled<'this>(&'this self) -> Option<Self::Field41MessageType<'this>> {
            self.as_ref().either(
                |t| <T as MsgTrait>::submsg_unlabeled(t).map(|t| ::puroro::Either::Left(t)),
                |u| <U as MsgTrait>::submsg_unlabeled(u).map(|u| ::puroro::Either::Right(u)),
            )
        }
        type Field42MessageType<'this> = ::puroro::Either<
            <T as MsgTrait>::Field42MessageType<'this>,
            <U as MsgTrait>::Field42MessageType<'this>,
        >;
        fn submsg_optional<'this>(&'this self) -> Option<Self::Field42MessageType<'this>> {
            self.as_ref().either(
                |t| <T as MsgTrait>::submsg_optional(t).map(|t| ::puroro::Either::Left(t)),
                |u| <U as MsgTrait>::submsg_optional(u).map(|u| ::puroro::Either::Right(u)),
            )
        }
        type Field43MessageType<'this> = ::puroro::Either<
            <T as MsgTrait>::Field43MessageType<'this>,
            <U as MsgTrait>::Field43MessageType<'this>,
        >;
        type Field43RepeatedType<'this> =
            ::puroro_internal::impls::either::EitherRepeatedMessageField<
                <T as MsgTrait>::Field43RepeatedType<'this>,
                <U as MsgTrait>::Field43RepeatedType<'this>,
            >;

        fn submsg_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::submsg_repeated(t))
                    .map_right(|u| <U as MsgTrait>::submsg_repeated(u)),
            )
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField1 {
        i32_unlabeled: i32,
    }

    impl ::puroro::Message for MsgSimpleField1 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField1 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Clone::clone(&self.i32_unlabeled)
        }
        type Field3RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            Default::default()
        }
        type Field13RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21StringType<'this> = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field21StringType<'this> {
            ""
        }
        type Field22StringType<'this> = &'static str;
        type Field23StringType<'this> = &'static str;
        type Field23RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field23StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::full_coverage3::Enum {
            Default::default()
        }
        type Field33RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field41MessageType<'this> = ();
        type Field42MessageType<'this> = ();
        type Field43MessageType<'this> = ();
        type Field43RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field43MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField2 {
        i32_optional: ::std::option::Option<i32>,
    }

    impl ::puroro::Message for MsgSimpleField2 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField2 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Default::default()
        }
        fn i32_optional<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.i32_optional)
        }
        type Field3RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            Default::default()
        }
        type Field13RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21StringType<'this> = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field21StringType<'this> {
            ""
        }
        type Field22StringType<'this> = &'static str;
        type Field23StringType<'this> = &'static str;
        type Field23RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field23StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::full_coverage3::Enum {
            Default::default()
        }
        type Field33RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field41MessageType<'this> = ();
        type Field42MessageType<'this> = ();
        type Field43MessageType<'this> = ();
        type Field43RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field43MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField3 {
        i32_repeated: ::std::vec::Vec<i32>,
    }

    impl ::puroro::Message for MsgSimpleField3 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField3 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Default::default()
        }
        type Field3RepeatedType<'this> = ::std::iter::Cloned<::std::slice::Iter<'this, i32>>;

        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            self.i32_repeated.iter().cloned()
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            Default::default()
        }
        type Field13RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21StringType<'this> = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field21StringType<'this> {
            ""
        }
        type Field22StringType<'this> = &'static str;
        type Field23StringType<'this> = &'static str;
        type Field23RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field23StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::full_coverage3::Enum {
            Default::default()
        }
        type Field33RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field41MessageType<'this> = ();
        type Field42MessageType<'this> = ();
        type Field43MessageType<'this> = ();
        type Field43RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field43MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField11 {
        float_unlabeled: f32,
    }

    impl ::puroro::Message for MsgSimpleField11 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField11 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Default::default()
        }
        type Field3RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            Clone::clone(&self.float_unlabeled)
        }
        type Field13RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21StringType<'this> = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field21StringType<'this> {
            ""
        }
        type Field22StringType<'this> = &'static str;
        type Field23StringType<'this> = &'static str;
        type Field23RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field23StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::full_coverage3::Enum {
            Default::default()
        }
        type Field33RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field41MessageType<'this> = ();
        type Field42MessageType<'this> = ();
        type Field43MessageType<'this> = ();
        type Field43RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field43MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField12 {
        float_optional: ::std::option::Option<f32>,
    }

    impl ::puroro::Message for MsgSimpleField12 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField12 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Default::default()
        }
        type Field3RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            Default::default()
        }
        fn float_optional<'this>(&'this self) -> Option<f32> {
            Clone::clone(&self.float_optional)
        }
        type Field13RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21StringType<'this> = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field21StringType<'this> {
            ""
        }
        type Field22StringType<'this> = &'static str;
        type Field23StringType<'this> = &'static str;
        type Field23RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field23StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::full_coverage3::Enum {
            Default::default()
        }
        type Field33RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field41MessageType<'this> = ();
        type Field42MessageType<'this> = ();
        type Field43MessageType<'this> = ();
        type Field43RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field43MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField13 {
        float_repeated: ::std::vec::Vec<f32>,
    }

    impl ::puroro::Message for MsgSimpleField13 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField13 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Default::default()
        }
        type Field3RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            Default::default()
        }
        type Field13RepeatedType<'this> = ::std::iter::Cloned<::std::slice::Iter<'this, f32>>;

        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            self.float_repeated.iter().cloned()
        }
        type Field21StringType<'this> = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field21StringType<'this> {
            ""
        }
        type Field22StringType<'this> = &'static str;
        type Field23StringType<'this> = &'static str;
        type Field23RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field23StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::full_coverage3::Enum {
            Default::default()
        }
        type Field33RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field41MessageType<'this> = ();
        type Field42MessageType<'this> = ();
        type Field43MessageType<'this> = ();
        type Field43RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field43MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField21 {
        string_unlabeled: ::std::borrow::Cow<'static, str>,
    }

    impl ::puroro::Message for MsgSimpleField21 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField21 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Default::default()
        }
        type Field3RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            Default::default()
        }
        type Field13RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21StringType<'this> = &'this str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field21StringType<'this> {
            &self.string_unlabeled
        }
        type Field22StringType<'this> = &'static str;
        type Field23StringType<'this> = &'static str;
        type Field23RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field23StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::full_coverage3::Enum {
            Default::default()
        }
        type Field33RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field41MessageType<'this> = ();
        type Field42MessageType<'this> = ();
        type Field43MessageType<'this> = ();
        type Field43RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field43MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField22 {
        string_optional: ::std::option::Option<::std::borrow::Cow<'static, str>>,
    }

    impl ::puroro::Message for MsgSimpleField22 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField22 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Default::default()
        }
        type Field3RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            Default::default()
        }
        type Field13RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21StringType<'this> = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field21StringType<'this> {
            ""
        }
        type Field22StringType<'this> = &'this str;
        fn string_optional<'this>(&'this self) -> Option<Self::Field22StringType<'this>> {
            self.string_optional.as_ref().map(|v| v.as_ref())
        }
        type Field23StringType<'this> = &'static str;
        type Field23RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field23StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::full_coverage3::Enum {
            Default::default()
        }
        type Field33RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field41MessageType<'this> = ();
        type Field42MessageType<'this> = ();
        type Field43MessageType<'this> = ();
        type Field43RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field43MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField23 {
        string_repeated: ::std::vec::Vec<::std::borrow::Cow<'static, str>>,
    }

    impl ::puroro::Message for MsgSimpleField23 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField23 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Default::default()
        }
        type Field3RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            Default::default()
        }
        type Field13RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21StringType<'this> = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field21StringType<'this> {
            ""
        }
        type Field22StringType<'this> = &'static str;
        type Field23StringType<'this> = &'this str;
        type Field23RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            str,
            ::std::slice::Iter<'this, ::std::borrow::Cow<'static, str>>,
        >;

        fn string_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.string_repeated.iter())
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::full_coverage3::Enum {
            Default::default()
        }
        type Field33RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field41MessageType<'this> = ();
        type Field42MessageType<'this> = ();
        type Field43MessageType<'this> = ();
        type Field43RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field43MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField31 {
        enum_unlabeled: self::_puroro_root::full_coverage3::Enum,
    }

    impl ::puroro::Message for MsgSimpleField31 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField31 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Default::default()
        }
        type Field3RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            Default::default()
        }
        type Field13RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21StringType<'this> = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field21StringType<'this> {
            ""
        }
        type Field22StringType<'this> = &'static str;
        type Field23StringType<'this> = &'static str;
        type Field23RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field23StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::full_coverage3::Enum {
            Clone::clone(&self.enum_unlabeled)
        }
        type Field33RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field41MessageType<'this> = ();
        type Field42MessageType<'this> = ();
        type Field43MessageType<'this> = ();
        type Field43RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field43MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField32 {
        enum_optional: ::std::option::Option<self::_puroro_root::full_coverage3::Enum>,
    }

    impl ::puroro::Message for MsgSimpleField32 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField32 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Default::default()
        }
        type Field3RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            Default::default()
        }
        type Field13RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21StringType<'this> = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field21StringType<'this> {
            ""
        }
        type Field22StringType<'this> = &'static str;
        type Field23StringType<'this> = &'static str;
        type Field23RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field23StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::full_coverage3::Enum {
            Default::default()
        }
        fn enum_optional<'this>(&'this self) -> Option<self::_puroro_root::full_coverage3::Enum> {
            Clone::clone(&self.enum_optional)
        }
        type Field33RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field41MessageType<'this> = ();
        type Field42MessageType<'this> = ();
        type Field43MessageType<'this> = ();
        type Field43RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field43MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField33 {
        enum_repeated: ::std::vec::Vec<self::_puroro_root::full_coverage3::Enum>,
    }

    impl ::puroro::Message for MsgSimpleField33 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField33 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Default::default()
        }
        type Field3RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            Default::default()
        }
        type Field13RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21StringType<'this> = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field21StringType<'this> {
            ""
        }
        type Field22StringType<'this> = &'static str;
        type Field23StringType<'this> = &'static str;
        type Field23RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field23StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::full_coverage3::Enum {
            Default::default()
        }
        type Field33RepeatedType<'this> = ::std::iter::Cloned<
            ::std::slice::Iter<'this, self::_puroro_root::full_coverage3::Enum>,
        >;

        fn enum_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            self.enum_repeated.iter().cloned()
        }
        type Field41MessageType<'this> = ();
        type Field42MessageType<'this> = ();
        type Field43MessageType<'this> = ();
        type Field43RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field43MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField41 {
    submsg_unlabeled: ::std::option::Option<::std::boxed::Box<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgSimple>>,
}

    impl ::puroro::Message for MsgSimpleField41 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField41 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Default::default()
        }
        type Field3RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            Default::default()
        }
        type Field13RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21StringType<'this> = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field21StringType<'this> {
            ""
        }
        type Field22StringType<'this> = &'static str;
        type Field23StringType<'this> = &'static str;
        type Field23RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field23StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::full_coverage3::Enum {
            Default::default()
        }
        type Field33RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field41MessageType<'this> = &'this self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgSimple;
        fn submsg_unlabeled<'this>(&'this self) -> Option<Self::Field41MessageType<'this>> {
            self.submsg_unlabeled.as_ref().map(|v| v.as_ref())
        }
        type Field42MessageType<'this> = ();
        type Field43MessageType<'this> = ();
        type Field43RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field43MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField42 {
    submsg_optional: ::std::option::Option<::std::boxed::Box<self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgSimple>>,
}

    impl ::puroro::Message for MsgSimpleField42 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField42 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Default::default()
        }
        type Field3RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            Default::default()
        }
        type Field13RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21StringType<'this> = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field21StringType<'this> {
            ""
        }
        type Field22StringType<'this> = &'static str;
        type Field23StringType<'this> = &'static str;
        type Field23RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field23StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::full_coverage3::Enum {
            Default::default()
        }
        type Field33RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field41MessageType<'this> = ();
        type Field42MessageType<'this> = &'this self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgSimple;
        fn submsg_optional<'this>(&'this self) -> Option<Self::Field42MessageType<'this>> {
            self.submsg_optional.as_ref().map(|v| v.as_ref())
        }
        type Field43MessageType<'this> = ();
        type Field43RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field43MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField43 {
        submsg_repeated: ::std::vec::Vec<
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgSimple,
        >,
    }

    impl ::puroro::Message for MsgSimpleField43 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField43 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Default::default()
        }
        type Field3RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            Default::default()
        }
        type Field13RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field21StringType<'this> = &'static str;
        fn string_unlabeled<'this>(&'this self) -> Self::Field21StringType<'this> {
            ""
        }
        type Field22StringType<'this> = &'static str;
        type Field23StringType<'this> = &'static str;
        type Field23RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field23StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::full_coverage3::Enum {
            Default::default()
        }
        type Field33RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field41MessageType<'this> = ();
        type Field42MessageType<'this> = ();
        type Field43MessageType<'this> = &'this self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgSimple;
        type Field43RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
    self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgSimple,
    ::std::slice::Iter<'this, self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgSimple>>;

        fn submsg_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.submsg_repeated.iter())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct MsgSimpleByValue {}
    impl ::puroro::Message for MsgSimpleByValue {}

    impl MsgTrait for MsgSimpleByValue {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn i32_optional<'this>(&'this self) -> Option<i32> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field3RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn float_unlabeled<'this>(&'this self) -> f32 {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn float_optional<'this>(&'this self) -> Option<f32> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field13RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field21StringType<'this> = ::std::borrow::Cow<'this, str>;
        fn string_unlabeled<'this>(&'this self) -> Self::Field21StringType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field22StringType<'this> = ::std::borrow::Cow<'this, str>;
        fn string_optional<'this>(&'this self) -> Option<Self::Field22StringType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field23StringType<'this> = ::std::borrow::Cow<'this, str>;
        type Field23RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field23StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::full_coverage3::Enum {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn enum_optional<'this>(&'this self) -> Option<self::_puroro_root::full_coverage3::Enum> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field33RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<
            self::_puroro_root::full_coverage3::Enum,
        >;
        fn enum_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field41MessageType<'this> = ::std::boxed::Box<
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgSimple,
        >;
        fn submsg_unlabeled<'this>(&'this self) -> Option<Self::Field41MessageType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field42MessageType<'this> = ::std::boxed::Box<
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgSimple,
        >;
        fn submsg_optional<'this>(&'this self) -> Option<Self::Field42MessageType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field43MessageType<'this> = ::std::boxed::Box<
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_impls::SubmsgSimple,
        >;
        type Field43RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field43MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct SomeSimple {
        pub i32_unlabeled: i32,
    }
    impl ::puroro::Message for SomeSimple {}

    impl SomeTrait for SomeSimple {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Clone::clone(&self.i32_unlabeled)
        }
    }

    impl ::puroro::DeserFromBytesIter for SomeSimple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for SomeSimple {
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

    impl ::puroro::SerToIoWrite for SomeSimple {
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
    impl SomeTrait for () {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Default::default()
        }
    }
    impl<T, U> SomeTrait for (T, U)
    where
        T: SomeTrait,
        U: SomeTrait,
    {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            let right = <U as SomeTrait>::i32_unlabeled(&self.1);
            if right != ::std::default::Default::default() {
                right
            } else {
                <T as SomeTrait>::i32_unlabeled(&self.0)
            }
        }
    }
    impl<T, U> SomeTrait for ::puroro::Either<T, U>
    where
        T: SomeTrait,
        U: SomeTrait,
    {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            self.as_ref().either(
                |t| <T as SomeTrait>::i32_unlabeled(t),
                |u| <U as SomeTrait>::i32_unlabeled(u),
            )
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct SomeSimpleField1 {
        i32_unlabeled: i32,
    }

    impl ::puroro::Message for SomeSimpleField1 {}

    impl super::_puroro_traits::SomeTrait for SomeSimpleField1 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            Clone::clone(&self.i32_unlabeled)
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct SomeSimpleByValue {}
    impl ::puroro::Message for SomeSimpleByValue {}

    impl SomeTrait for SomeSimpleByValue {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
    }
}
pub use _puroro_traits::*;
pub mod _puroro_traits {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }

    pub trait MsgTrait {
        fn i32_unlabeled<'this>(&'this self) -> i32;
        fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::default::Default::default()
        }
        type Field3RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = i32>;
        fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this>;
        fn float_unlabeled<'this>(&'this self) -> f32;
        fn float_optional<'this>(&'this self) -> ::std::option::Option<f32> {
            ::std::default::Default::default()
        }
        type Field13RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = f32>;
        fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this>;
        type Field21StringType<'this>: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug;
        fn string_unlabeled<'this>(&'this self) -> Self::Field21StringType<'this>;
        type Field22StringType<'this>: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug;
        fn string_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field22StringType<'this>> {
            ::std::default::Default::default()
        }
        type Field23StringType<'this>: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug;
        type Field23RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field23StringType<'this>>;
        fn string_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this>;
        fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::full_coverage3::Enum;
        fn enum_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<self::_puroro_root::full_coverage3::Enum> {
            ::std::default::Default::default()
        }
        type Field33RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = self::_puroro_root::full_coverage3::Enum>;
        fn enum_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this>;
        type Field41MessageType<'this>:
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug;
        fn submsg_unlabeled<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field41MessageType<'this>> {
            ::std::default::Default::default()
        }
        type Field42MessageType<'this>:
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug;
        fn submsg_optional<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field42MessageType<'this>> {
            ::std::default::Default::default()
        }
        type Field43MessageType<'this>:
            self::_puroro_root::full_coverage3::_puroro_nested::msg::_puroro_traits::SubmsgTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug;
        type Field43RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field43MessageType<'this>>;
        fn submsg_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this>;
    }

    macro_rules! msg_delegate {
        ($ty:ty) => {
            fn i32_unlabeled<'this>(&'this self) -> i32 {
                (**self).i32_unlabeled()
            }
            fn i32_optional<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).i32_optional()
            }
            type Field3RepeatedType<'this> = <$ty>::Field3RepeatedType<'this>;
            fn i32_repeated<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
                (**self).i32_repeated()
            }
            fn float_unlabeled<'this>(&'this self) -> f32 {
                (**self).float_unlabeled()
            }
            fn float_optional<'this>(&'this self) -> ::std::option::Option<f32> {
                (**self).float_optional()
            }
            type Field13RepeatedType<'this> = <$ty>::Field13RepeatedType<'this>;
            fn float_repeated<'this>(&'this self) -> Self::Field13RepeatedType<'this> {
                (**self).float_repeated()
            }
            type Field21StringType<'this> = <$ty>::Field21StringType<'this>;
            fn string_unlabeled<'this>(&'this self) -> Self::Field21StringType<'this> {
                (**self).string_unlabeled()
            }
            type Field22StringType<'this> = <$ty>::Field22StringType<'this>;
            fn string_optional<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field22StringType<'this>> {
                (**self).string_optional()
            }
            type Field23StringType<'this> = <$ty>::Field23StringType<'this>;
            type Field23RepeatedType<'this> = <$ty>::Field23RepeatedType<'this>;
            fn string_repeated<'this>(&'this self) -> Self::Field23RepeatedType<'this> {
                (**self).string_repeated()
            }
            fn enum_unlabeled<'this>(&'this self) -> self::_puroro_root::full_coverage3::Enum {
                (**self).enum_unlabeled()
            }
            fn enum_optional<'this>(
                &'this self,
            ) -> ::std::option::Option<self::_puroro_root::full_coverage3::Enum> {
                (**self).enum_optional()
            }
            type Field33RepeatedType<'this> = <$ty>::Field33RepeatedType<'this>;
            fn enum_repeated<'this>(&'this self) -> Self::Field33RepeatedType<'this> {
                (**self).enum_repeated()
            }
            type Field41MessageType<'this> = <$ty>::Field41MessageType<'this>;
            fn submsg_unlabeled<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field41MessageType<'this>> {
                (**self).submsg_unlabeled()
            }
            type Field42MessageType<'this> = <$ty>::Field42MessageType<'this>;
            fn submsg_optional<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field42MessageType<'this>> {
                (**self).submsg_optional()
            }
            type Field43MessageType<'this> = <$ty>::Field43MessageType<'this>;
            type Field43RepeatedType<'this> = <$ty>::Field43RepeatedType<'this>;
            fn submsg_repeated<'this>(&'this self) -> Self::Field43RepeatedType<'this> {
                (**self).submsg_repeated()
            }
        };
    }

    impl<T> MsgTrait for &'_ T
    where
        T: MsgTrait,
    {
        msg_delegate!(T);
    }

    impl<T> MsgTrait for ::std::boxed::Box<T>
    where
        T: MsgTrait,
    {
        msg_delegate!(T);
    }
    pub trait SomeTrait {
        fn i32_unlabeled<'this>(&'this self) -> i32;
    }

    macro_rules! some_delegate {
        ($ty:ty) => {
            fn i32_unlabeled<'this>(&'this self) -> i32 {
                (**self).i32_unlabeled()
            }
        };
    }

    impl<T> SomeTrait for &'_ T
    where
        T: SomeTrait,
    {
        some_delegate!(T);
    }

    impl<T> SomeTrait for ::std::boxed::Box<T>
    where
        T: SomeTrait,
    {
        some_delegate!(T);
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

        pub use _puroro_impls::SubmsgSimple as Submsg;
        pub mod _puroro_impls {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }
            use super::_puroro_traits::*;
            #[derive(
                ::std::clone::Clone,
                ::std::default::Default,
                ::std::cmp::PartialEq,
                ::std::fmt::Debug,
            )]
            pub struct SubmsgSimple {
                pub i32_unlabeled: i32,
            }
            impl ::puroro::Message for SubmsgSimple {}

            impl SubmsgTrait for SubmsgSimple {
                fn i32_unlabeled<'this>(&'this self) -> i32 {
                    Clone::clone(&self.i32_unlabeled)
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
                            ::puroro::tags::Unlabeled,
                            ::puroro::tags::Int32,
                        >::deser_field(&mut self.i32_unlabeled, data),

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
                        ::puroro::tags::Unlabeled,
                        ::puroro::tags::Int32,
                    >::ser_field(&self.i32_unlabeled, 1, out)?;
                    ::std::result::Result::Ok(())
                }
            }
            impl SubmsgTrait for () {
                fn i32_unlabeled<'this>(&'this self) -> i32 {
                    Default::default()
                }
            }
            impl<T, U> SubmsgTrait for (T, U)
            where
                T: SubmsgTrait,
                U: SubmsgTrait,
            {
                fn i32_unlabeled<'this>(&'this self) -> i32 {
                    let right = <U as SubmsgTrait>::i32_unlabeled(&self.1);
                    if right != ::std::default::Default::default() {
                        right
                    } else {
                        <T as SubmsgTrait>::i32_unlabeled(&self.0)
                    }
                }
            }
            impl<T, U> SubmsgTrait for ::puroro::Either<T, U>
            where
                T: SubmsgTrait,
                U: SubmsgTrait,
            {
                fn i32_unlabeled<'this>(&'this self) -> i32 {
                    self.as_ref().either(
                        |t| <T as SubmsgTrait>::i32_unlabeled(t),
                        |u| <U as SubmsgTrait>::i32_unlabeled(u),
                    )
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            struct SubmsgSimpleField1 {
                i32_unlabeled: i32,
            }

            impl ::puroro::Message for SubmsgSimpleField1 {}

            impl super::_puroro_traits::SubmsgTrait for SubmsgSimpleField1 {
                fn i32_unlabeled<'this>(&'this self) -> i32 {
                    Clone::clone(&self.i32_unlabeled)
                }
            }
            #[derive(
                ::std::clone::Clone,
                ::std::default::Default,
                ::std::cmp::PartialEq,
                ::std::fmt::Debug,
            )]
            pub struct SubmsgSimpleByValue {}
            impl ::puroro::Message for SubmsgSimpleByValue {}

            impl SubmsgTrait for SubmsgSimpleByValue {
                fn i32_unlabeled<'this>(&'this self) -> i32 {
                    unimplemented!("Please don't use / instantiate this struct!!")
                }
            }
        }
        pub use _puroro_traits::*;
        pub mod _puroro_traits {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }

            pub trait SubmsgTrait {
                fn i32_unlabeled<'this>(&'this self) -> i32;
            }

            macro_rules! submsg_delegate {
                ($ty:ty) => {
                    fn i32_unlabeled<'this>(&'this self) -> i32 {
                        (**self).i32_unlabeled()
                    }
                };
            }

            impl<T> SubmsgTrait for &'_ T
            where
                T: SubmsgTrait,
            {
                submsg_delegate!(T);
            }

            impl<T> SubmsgTrait for ::std::boxed::Box<T>
            where
                T: SubmsgTrait,
            {
                submsg_delegate!(T);
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
    pub mod some {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
    }
}
