// A generated source code by puroro library
// package oneofs3

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub use _puroro_impls::MsgSimple as Msg;
pub use _puroro_impls::SubmsgSimple as Submsg;
pub mod _puroro_impls {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct MsgSimple {
        pub g1_int32: i32,
        pub g1_string: ::std::string::String,
        pub g2_f32: f32,
        pub g2_string: ::std::string::String,
        pub g2_submsg: ::std::option::Option<
            ::std::boxed::Box<self::_puroro_root::oneofs3::_puroro_impls::SubmsgSimple>,
        >,
        pub g3_int32: i32,
        pub group_one:
            ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne_Simple>,
        pub group_two:
            ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo_Simple>,
        pub group_three:
            ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree_Simple>,
    }
    impl ::puroro::Message for MsgSimple {}

    impl super::_puroro_traits::MsgTrait for MsgSimple {
        type Field5MessageType<'this> = self::_puroro_root::oneofs3::_puroro_impls::SubmsgSimple;
        fn group_one<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this>>
        {
            self.group_one
                .as_ref()
                .map(|o| ::std::convert::From::from(o))
        }
        fn group_two<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>>
        {
            self.group_two
                .as_ref()
                .map(|o| ::std::convert::From::from(o))
        }
        fn group_three<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            self.group_three
                .as_ref()
                .map(|o| ::std::convert::From::from(o))
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
                1 => {
                    use super::_puroro_nested::msg::_puroro_oneofs::GroupOne_Simple;
                    #[allow(unused)]
                    use std::option::Option::Some;
                    if !matches!(&self.group_one, Some(GroupOne_Simple::G1Int32(_))) {
                        self.group_one =
                            Some(GroupOne_Simple::G1Int32(::std::default::Default::default()));
                    }
                    let field_value_mut_ref = match &mut self.group_one {
                        Some(GroupOne_Simple::G1Int32(v)) => v,
                        _ => unreachable!(),
                    };
                    ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                        ::puroro::tags::OneofItem,
                        ::puroro::tags::Int32,
                    >::deser_field(field_value_mut_ref, data)
                }
                2 => {
                    use super::_puroro_nested::msg::_puroro_oneofs::GroupOne_Simple;
                    #[allow(unused)]
                    use std::option::Option::Some;
                    if !matches!(&self.group_one, Some(GroupOne_Simple::G1String(_))) {
                        self.group_one =
                            Some(GroupOne_Simple::G1String(::std::default::Default::default()));
                    }
                    let field_value_mut_ref = match &mut self.group_one {
                        Some(GroupOne_Simple::G1String(v)) => v,
                        _ => unreachable!(),
                    };
                    ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                        ::puroro::tags::OneofItem,
                        ::puroro::tags::String,
                    >::deser_field(field_value_mut_ref, data)
                }
                3 => {
                    use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo_Simple;
                    #[allow(unused)]
                    use std::option::Option::Some;
                    if !matches!(&self.group_two, Some(GroupTwo_Simple::G2F32(_))) {
                        self.group_two =
                            Some(GroupTwo_Simple::G2F32(::std::default::Default::default()));
                    }
                    let field_value_mut_ref = match &mut self.group_two {
                        Some(GroupTwo_Simple::G2F32(v)) => v,
                        _ => unreachable!(),
                    };
                    ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                        ::puroro::tags::OneofItem,
                        ::puroro::tags::Float,
                    >::deser_field(field_value_mut_ref, data)
                }
                4 => {
                    use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo_Simple;
                    #[allow(unused)]
                    use std::option::Option::Some;
                    if !matches!(&self.group_two, Some(GroupTwo_Simple::G2String(_))) {
                        self.group_two =
                            Some(GroupTwo_Simple::G2String(::std::default::Default::default()));
                    }
                    let field_value_mut_ref = match &mut self.group_two {
                        Some(GroupTwo_Simple::G2String(v)) => v,
                        _ => unreachable!(),
                    };
                    ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                        ::puroro::tags::OneofItem,
                        ::puroro::tags::String,
                    >::deser_field(field_value_mut_ref, data)
                }
                5 => {
                    use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo_Simple;
                    #[allow(unused)]
                    use std::option::Option::Some;
                    if !matches!(&self.group_two, Some(GroupTwo_Simple::G2Submsg(_))) {
                        self.group_two =
                            Some(GroupTwo_Simple::G2Submsg(::std::default::Default::default()));
                    }
                    let field_value_mut_ref = match &mut self.group_two {
                        Some(GroupTwo_Simple::G2Submsg(v)) => v,
                        _ => unreachable!(),
                    };
                    ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                        ::puroro::tags::OneofItem,
                        ::puroro::tags::Message<
                            self::_puroro_root::oneofs3::_puroro_impls::SubmsgSimple,
                        >,
                    >::deser_field(field_value_mut_ref, data)
                }
                6 => {
                    use super::_puroro_nested::msg::_puroro_oneofs::GroupThree_Simple;
                    #[allow(unused)]
                    use std::option::Option::Some;
                    if !matches!(&self.group_three, Some(GroupThree_Simple::G3Int32(_))) {
                        self.group_three = Some(GroupThree_Simple::G3Int32(
                            ::std::default::Default::default(),
                        ));
                    }
                    let field_value_mut_ref = match &mut self.group_three {
                        Some(GroupThree_Simple::G3Int32(v)) => v,
                        _ => unreachable!(),
                    };
                    ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                        ::puroro::tags::OneofItem,
                        ::puroro::tags::Int32,
                    >::deser_field(field_value_mut_ref, data)
                }

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
            >::ser_field(&self.g1_int32, 1, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::String,
            >::ser_field(&self.g1_string, 2, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Float,
            >::ser_field(&self.g2_f32, 3, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::String,
            >::ser_field(&self.g2_string, 4, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Message<self::_puroro_root::oneofs3::_puroro_impls::SubmsgSimple>,
            >::ser_field(&self.g2_submsg, 5, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Int32,
            >::ser_field(&self.g3_int32, 6, out)?;
            ::std::result::Result::Ok(())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct MsgEmpty;

    impl ::puroro::Message for MsgEmpty {}

    impl super::_puroro_traits::MsgTrait for MsgEmpty {
        type Field5MessageType<'this> = self::_puroro_root::oneofs3::_puroro_impls::SubmsgEmpty;
        fn group_one<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this>>
        {
            None
        }
        fn group_two<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>>
        {
            None
        }
        fn group_three<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
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
        type Field5MessageType<'this> = ::puroro::Either<
            ::std::borrow::Cow<
                'this,
                <T as super::_puroro_traits::MsgTrait>::Field5MessageType<'this>,
            >,
            ::std::borrow::Cow<
                'this,
                <U as super::_puroro_traits::MsgTrait>::Field5MessageType<'this>,
            >,
        >;
        fn group_one<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this>>
        {
            todo!()
        }
        fn group_two<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>>
        {
            todo!()
        }
        fn group_three<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            todo!()
        }
    }

    impl<'a, T> super::_puroro_traits::MsgTrait for ::std::borrow::Cow<'a, T>
    where
        T: 'a + ::std::clone::Clone + super::_puroro_traits::MsgTrait,
    {
        type Field5MessageType<'this> =
            <T as super::_puroro_traits::MsgTrait>::Field5MessageType<'this>;
        fn group_one<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this>>
        {
            use std::ops::Deref;
            self.deref().group_one()
        }
        fn group_two<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>>
        {
            use std::ops::Deref;
            self.deref().group_two()
        }
        fn group_three<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            use std::ops::Deref;
            self.deref().group_three()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField1 {
        g1_int32: i32,
    }

    impl ::puroro::Message for MsgSimpleField1 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField1 {
        type Field5MessageType<'this> = self::_puroro_root::oneofs3::_puroro_impls::SubmsgEmpty;
        fn group_one<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this>>
        {
            todo!()
        }
        fn group_two<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>>
        {
            todo!()
        }
        fn group_three<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            todo!()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField2 {
        g1_string: ::std::string::String,
    }

    impl ::puroro::Message for MsgSimpleField2 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField2 {
        type Field5MessageType<'this> = self::_puroro_root::oneofs3::_puroro_impls::SubmsgEmpty;
        fn group_one<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this>>
        {
            todo!()
        }
        fn group_two<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>>
        {
            todo!()
        }
        fn group_three<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            todo!()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField3 {
        g2_f32: f32,
    }

    impl ::puroro::Message for MsgSimpleField3 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField3 {
        type Field5MessageType<'this> = self::_puroro_root::oneofs3::_puroro_impls::SubmsgEmpty;
        fn group_one<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this>>
        {
            todo!()
        }
        fn group_two<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>>
        {
            todo!()
        }
        fn group_three<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            todo!()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField4 {
        g2_string: ::std::string::String,
    }

    impl ::puroro::Message for MsgSimpleField4 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField4 {
        type Field5MessageType<'this> = self::_puroro_root::oneofs3::_puroro_impls::SubmsgEmpty;
        fn group_one<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this>>
        {
            todo!()
        }
        fn group_two<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>>
        {
            todo!()
        }
        fn group_three<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            todo!()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField5 {
        g2_submsg: ::std::option::Option<
            ::std::boxed::Box<self::_puroro_root::oneofs3::_puroro_impls::SubmsgSimple>,
        >,
    }

    impl ::puroro::Message for MsgSimpleField5 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField5 {
        type Field5MessageType<'this> = self::_puroro_root::oneofs3::_puroro_impls::SubmsgSimple;
        fn group_one<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this>>
        {
            todo!()
        }
        fn group_two<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>>
        {
            todo!()
        }
        fn group_three<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            todo!()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MsgSimpleField6 {
        g3_int32: i32,
    }

    impl ::puroro::Message for MsgSimpleField6 {}

    impl super::_puroro_traits::MsgTrait for MsgSimpleField6 {
        type Field5MessageType<'this> = self::_puroro_root::oneofs3::_puroro_impls::SubmsgEmpty;
        fn group_one<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this>>
        {
            todo!()
        }
        fn group_two<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>>
        {
            todo!()
        }
        fn group_three<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            todo!()
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct SubmsgSimple {
        pub i32_unlabeled: i32,
    }
    impl ::puroro::Message for SubmsgSimple {}

    impl super::_puroro_traits::SubmsgTrait for SubmsgSimple {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            ::std::clone::Clone::clone(&self.i32_unlabeled)
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
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct SubmsgEmpty;

    impl ::puroro::Message for SubmsgEmpty {}

    impl super::_puroro_traits::SubmsgTrait for SubmsgEmpty {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            ::std::default::Default::default()
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
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            self.as_ref().either(
                <T as super::_puroro_traits::SubmsgTrait>::i32_unlabeled,
                <U as super::_puroro_traits::SubmsgTrait>::i32_unlabeled,
            )
        }
    }

    impl<'a, T> super::_puroro_traits::SubmsgTrait for ::std::borrow::Cow<'a, T>
    where
        T: 'a + ::std::clone::Clone + super::_puroro_traits::SubmsgTrait,
    {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            use std::ops::Deref;
            self.deref().i32_unlabeled()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct SubmsgSimpleField1 {
        i32_unlabeled: i32,
    }

    impl ::puroro::Message for SubmsgSimpleField1 {}

    impl super::_puroro_traits::SubmsgTrait for SubmsgSimpleField1 {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            ::std::clone::Clone::clone(&self.i32_unlabeled)
        }
    }
}
pub use _puroro_traits::*;
pub mod _puroro_traits {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }

    pub trait MsgTrait: ::std::clone::Clone {
        type Field5MessageType<'this>: 'this
            + self::_puroro_root::oneofs3::_puroro_traits::SubmsgTrait;
        fn group_one<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this>>;
        fn group_two<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>>;
        fn group_three<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree>;
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

        pub use _puroro_oneofs::*;
        pub mod _puroro_oneofs {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }
            pub enum GroupOne<'msg> {
                G1Int32(i32),
                G1String(::std::borrow::Cow<'msg, str>),
            }

            #[allow(non_camel_case_types)]
            #[derive(Clone, PartialEq, Debug)]
            pub enum GroupOne_Simple {
                G1Int32(i32),
                G1String(::std::string::String),
            }

            impl<'msg> ::std::convert::From<&'msg GroupOne_Simple> for GroupOne<'msg> {
                fn from(from: &'msg GroupOne_Simple) -> Self {
                    match from {
                        GroupOne_Simple::G1Int32(v) => {
                            GroupOne::G1Int32(::std::clone::Clone::clone(&v))
                        }
                        GroupOne_Simple::G1String(v) => {
                            GroupOne::G1String(::std::borrow::Cow::Borrowed(v.as_ref()))
                        }
                    }
                }
            }
            pub enum GroupTwo<'msg, T: self::_puroro_root::oneofs3::_puroro_traits::MsgTrait> {
                G2F32(f32),
                G2String(::std::borrow::Cow<'msg, str>),
                G2Submsg(::std::borrow::Cow<'msg, <T as self::_puroro_root::oneofs3::_puroro_traits::MsgTrait>::Field5MessageType<'msg>>),
            }

            #[allow(non_camel_case_types)]
            #[derive(Clone, PartialEq, Debug)]
            pub enum GroupTwo_Simple {
                G2F32(f32),
                G2String(::std::string::String),
                G2Submsg(
                    ::std::boxed::Box<self::_puroro_root::oneofs3::_puroro_impls::SubmsgSimple>,
                ),
            }

            impl<'msg> ::std::convert::From<&'msg GroupTwo_Simple>
                for GroupTwo<'msg, self::_puroro_root::oneofs3::_puroro_impls::MsgSimple>
            {
                fn from(from: &'msg GroupTwo_Simple) -> Self {
                    match from {
                        GroupTwo_Simple::G2F32(v) => {
                            GroupTwo::G2F32(::std::clone::Clone::clone(&v))
                        }
                        GroupTwo_Simple::G2String(v) => {
                            GroupTwo::G2String(::std::borrow::Cow::Borrowed(v.as_ref()))
                        }
                        GroupTwo_Simple::G2Submsg(v) => {
                            GroupTwo::G2Submsg(::std::borrow::Cow::Borrowed(v.as_ref()))
                        }
                    }
                }
            }
            pub enum GroupThree {
                G3Int32(i32),
            }

            #[allow(non_camel_case_types)]
            #[derive(Clone, PartialEq, Debug)]
            pub enum GroupThree_Simple {
                G3Int32(i32),
            }

            impl<'msg> ::std::convert::From<&'msg GroupThree_Simple> for GroupThree {
                fn from(from: &'msg GroupThree_Simple) -> Self {
                    match from {
                        GroupThree_Simple::G3Int32(v) => {
                            GroupThree::G3Int32(::std::clone::Clone::clone(&v))
                        }
                    }
                }
            }
        }
    }
    pub mod submsg {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
    }
}
