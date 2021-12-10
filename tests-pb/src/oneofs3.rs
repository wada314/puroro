// A generated source code by puroro library
// package oneofs3

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub use _puroro_simple_impl::Msg;
pub use _puroro_simple_impl::Submsg;
pub mod _puroro_simple_impl {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
    pub struct Msg {
        group_one: ::std::option::Option<
            super::_puroro_nested::msg::_puroro_private_oneofs::GroupOneSimple,
        >,
        group_two: ::std::option::Option<
            super::_puroro_nested::msg::_puroro_private_oneofs::GroupTwoSimple,
        >,
        group_three: ::std::option::Option<
            super::_puroro_nested::msg::_puroro_private_oneofs::GroupThreeSimple,
        >,
    }
    impl ::puroro::Message<Msg> for Msg {}

    impl Msg {
        pub fn new() -> Self {
            Self {
                group_one: ::std::default::Default::default(),
                group_two: ::std::default::Default::default(),
                group_three: ::std::default::Default::default(),
            }
        }
        pub fn clear_group_one(&mut self) {
            self.group_one = ::std::option::Option::None;
        }
        pub fn g1_int32_mut(&mut self) -> &mut i32 {
            use super::_puroro_nested::msg::_puroro_private_oneofs::GroupOneSimple as E;
            if !matches!(&self.group_one, Some(E::G1Int32(_))) {
                self.group_one = Some(E::G1Int32(::std::default::Default::default()));
            }
            match &mut self.group_one {
                Some(E::G1Int32(v)) => v,
                _ => unreachable!(),
            }
        }
        pub fn g1_string_mut(&mut self) -> &mut ::std::string::String {
            use super::_puroro_nested::msg::_puroro_private_oneofs::GroupOneSimple as E;
            if !matches!(&self.group_one, Some(E::G1String(_))) {
                self.group_one = Some(E::G1String(::std::default::Default::default()));
            }
            match &mut self.group_one {
                Some(E::G1String(v)) => v,
                _ => unreachable!(),
            }
        }
        pub fn clear_group_two(&mut self) {
            self.group_two = ::std::option::Option::None;
        }
        pub fn g2_f32_mut(&mut self) -> &mut f32 {
            use super::_puroro_nested::msg::_puroro_private_oneofs::GroupTwoSimple as E;
            if !matches!(&self.group_two, Some(E::G2F32(_))) {
                self.group_two = Some(E::G2F32(::std::default::Default::default()));
            }
            match &mut self.group_two {
                Some(E::G2F32(v)) => v,
                _ => unreachable!(),
            }
        }
        pub fn g2_string_mut(&mut self) -> &mut ::std::string::String {
            use super::_puroro_nested::msg::_puroro_private_oneofs::GroupTwoSimple as E;
            if !matches!(&self.group_two, Some(E::G2String(_))) {
                self.group_two = Some(E::G2String(::std::default::Default::default()));
            }
            match &mut self.group_two {
                Some(E::G2String(v)) => v,
                _ => unreachable!(),
            }
        }
        pub fn g2_submsg_mut(
            &mut self,
        ) -> &mut ::std::boxed::Box<self::_puroro_root::oneofs3::_puroro_simple_impl::Submsg>
        {
            use super::_puroro_nested::msg::_puroro_private_oneofs::GroupTwoSimple as E;
            if !matches!(&self.group_two, Some(E::G2Submsg(_))) {
                self.group_two = Some(E::G2Submsg(::std::default::Default::default()));
            }
            match &mut self.group_two {
                Some(E::G2Submsg(v)) => v,
                _ => unreachable!(),
            }
        }
        pub fn clear_group_three(&mut self) {
            self.group_three = ::std::option::Option::None;
        }
        pub fn g3_int32_mut(&mut self) -> &mut i32 {
            use super::_puroro_nested::msg::_puroro_private_oneofs::GroupThreeSimple as E;
            if !matches!(&self.group_three, Some(E::G3Int32(_))) {
                self.group_three = Some(E::G3Int32(::std::default::Default::default()));
            }
            match &mut self.group_three {
                Some(E::G3Int32(v)) => v,
                _ => unreachable!(),
            }
        }
    }

    impl super::_puroro_traits::MsgTrait for Msg {
        type Field5MessageType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::oneofs3::_puroro_simple_impl::Submsg;
        fn group_one<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this>> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupOne as F;
            use super::_puroro_nested::msg::_puroro_private_oneofs::GroupOneSimple as E;
            self.group_one.as_ref().map(|oneof| match oneof {
                E::G1Int32(v) => F::G1Int32(v.clone()),

                E::G1String(v) => F::G1String(v.as_ref()),
            })
        }
        fn group_two<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as F;
            use super::_puroro_nested::msg::_puroro_private_oneofs::GroupTwoSimple as E;
            self.group_two.as_ref().map(|oneof| match oneof {
                E::G2F32(v) => F::G2F32(v.clone()),

                E::G2String(v) => F::G2String(v.as_ref()),

                E::G2Submsg(v) => F::G2Submsg(v.as_ref()),
            })
        }
        fn group_three<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupThree as F;
            use super::_puroro_nested::msg::_puroro_private_oneofs::GroupThreeSimple as E;
            self.group_three.as_ref().map(|oneof| match oneof {
                E::G3Int32(v) => F::G3Int32(v.clone()),
            })
        }
    }

    impl ::puroro::MessageRepresentativeImpl for Msg {}

    impl ::puroro::internal::de::DeserMessageFromBytesIter for Msg {
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
                1 => {
                    use super::_puroro_nested::msg::_puroro_private_oneofs::GroupOneSimple as E;
                    if !matches!(&self.group_one, Some(E::G1Int32(_))) {
                        self.group_one = Some(E::G1Int32(::std::default::Default::default()));
                    }
                    let field_value_mut_ref = match &mut self.group_one {
                        Some(E::G1Int32(v)) => v,
                        _ => unreachable!(),
                    };
                    DeserFieldFromBytesIter::<
                    ::puroro::tags::OneofField, ::puroro::tags::Int32
                >::deser_field(field_value_mut_ref, data)
                }
                2 => {
                    use super::_puroro_nested::msg::_puroro_private_oneofs::GroupOneSimple as E;
                    if !matches!(&self.group_one, Some(E::G1String(_))) {
                        self.group_one = Some(E::G1String(::std::default::Default::default()));
                    }
                    let field_value_mut_ref = match &mut self.group_one {
                        Some(E::G1String(v)) => v,
                        _ => unreachable!(),
                    };
                    DeserFieldFromBytesIter::<
                    ::puroro::tags::OneofField, ::puroro::tags::String
                >::deser_field(field_value_mut_ref, data)
                }
                3 => {
                    use super::_puroro_nested::msg::_puroro_private_oneofs::GroupTwoSimple as E;
                    if !matches!(&self.group_two, Some(E::G2F32(_))) {
                        self.group_two = Some(E::G2F32(::std::default::Default::default()));
                    }
                    let field_value_mut_ref = match &mut self.group_two {
                        Some(E::G2F32(v)) => v,
                        _ => unreachable!(),
                    };
                    DeserFieldFromBytesIter::<
                    ::puroro::tags::OneofField, ::puroro::tags::Float
                >::deser_field(field_value_mut_ref, data)
                }
                4 => {
                    use super::_puroro_nested::msg::_puroro_private_oneofs::GroupTwoSimple as E;
                    if !matches!(&self.group_two, Some(E::G2String(_))) {
                        self.group_two = Some(E::G2String(::std::default::Default::default()));
                    }
                    let field_value_mut_ref = match &mut self.group_two {
                        Some(E::G2String(v)) => v,
                        _ => unreachable!(),
                    };
                    DeserFieldFromBytesIter::<
                    ::puroro::tags::OneofField, ::puroro::tags::String
                >::deser_field(field_value_mut_ref, data)
                }
                5 => {
                    use super::_puroro_nested::msg::_puroro_private_oneofs::GroupTwoSimple as E;
                    if !matches!(&self.group_two, Some(E::G2Submsg(_))) {
                        self.group_two = Some(E::G2Submsg(::std::default::Default::default()));
                    }
                    let field_value_mut_ref = match &mut self.group_two {
                        Some(E::G2Submsg(v)) => v,
                        _ => unreachable!(),
                    };
                    DeserFieldFromBytesIter::<
                        ::puroro::tags::OneofField,
                        ::puroro::tags::Message<
                            ::std::boxed::Box<
                                self::_puroro_root::oneofs3::_puroro_simple_impl::Submsg,
                            >,
                        >,
                    >::deser_field(field_value_mut_ref, data)
                }
                6 => {
                    use super::_puroro_nested::msg::_puroro_private_oneofs::GroupThreeSimple as E;
                    if !matches!(&self.group_three, Some(E::G3Int32(_))) {
                        self.group_three = Some(E::G3Int32(::std::default::Default::default()));
                    }
                    let field_value_mut_ref = match &mut self.group_three {
                        Some(E::G3Int32(v)) => v,
                        _ => unreachable!(),
                    };
                    DeserFieldFromBytesIter::<
                    ::puroro::tags::OneofField, ::puroro::tags::Int32
                >::deser_field(field_value_mut_ref, data)
                }

                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl ::puroro::internal::se::SerMessageToIoWrite for Msg
    where
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g1_int32_opt(self),
                1,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g1_string_opt(self),
                2,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::Float,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g2_f32_opt(self),
                3,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g2_string_opt(self),
                4,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::Message<
                    <Self as super::_puroro_traits::MsgTrait>::Field5MessageType<'_>,
                >,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g2_submsg_opt(self),
                5,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g3_int32_opt(self),
                6,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::default::Default for Msg {
        fn default() -> Self {
            Self::new()
        }
    }

    impl ::std::fmt::Debug for Msg
    where
        Self: super::_puroro_traits::MsgTrait,
    {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.debug_struct("Msg")
                .field(
                    "g1_int32",
                    &<Self as super::_puroro_traits::MsgTrait>::g1_int32_opt(self),
                )
                .field(
                    "g1_string",
                    &<Self as super::_puroro_traits::MsgTrait>::g1_string_opt(self),
                )
                .field(
                    "g2_f32",
                    &<Self as super::_puroro_traits::MsgTrait>::g2_f32_opt(self),
                )
                .field(
                    "g2_string",
                    &<Self as super::_puroro_traits::MsgTrait>::g2_string_opt(self),
                )
                .field(
                    "g2_submsg",
                    &<Self as super::_puroro_traits::MsgTrait>::g2_submsg_opt(self),
                )
                .field(
                    "g3_int32",
                    &<Self as super::_puroro_traits::MsgTrait>::g3_int32_opt(self),
                )
                .finish()
        }
    }

    impl ::std::clone::Clone for Msg {
        fn clone(&self) -> Self {
            Self {
                group_one: ::std::clone::Clone::clone(&self.group_one),
                group_two: ::std::clone::Clone::clone(&self.group_two),
                group_three: ::std::clone::Clone::clone(&self.group_three),
            }
        }
    }

    impl ::std::cmp::PartialEq for Msg {
        fn eq(&self, rhs: &Self) -> bool {
            self.group_one == rhs.group_one
                && self.group_two == rhs.group_two
                && self.group_three == rhs.group_three
                && true
        }
    }
    pub struct Submsg {
        i32_unlabeled: i32,
    }
    impl ::puroro::Message<Submsg> for Submsg {}

    impl Submsg {
        pub fn new() -> Self {
            Self {
                i32_unlabeled: ::std::default::Default::default(),
            }
        }
        pub fn i32_unlabeled_mut(&mut self) -> &mut i32 {
            &mut self.i32_unlabeled
        }
    }

    impl super::_puroro_traits::SubmsgTrait for Submsg {
        fn i32_unlabeled_opt<'this>(&'this self) -> Option<i32> {
            if self.i32_unlabeled == ::std::default::Default::default() {
                ::std::option::Option::None
            } else {
                ::std::option::Option::Some(self.i32_unlabeled.clone())
            }
        }
    }

    impl ::puroro::MessageRepresentativeImpl for Submsg {}

    impl ::puroro::internal::de::DeserMessageFromBytesIter for Submsg {
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
                ::puroro::tags::Unlabeled, ::puroro::tags::Int32
            >::deser_field(&mut self.i32_unlabeled, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::internal::se::SerMessageToIoWrite for Submsg
    where
        Self: super::_puroro_traits::SubmsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::SubmsgTrait>::i32_unlabeled_opt(self),
                1,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::default::Default for Submsg {
        fn default() -> Self {
            Self::new()
        }
    }

    impl ::std::fmt::Debug for Submsg
    where
        Self: super::_puroro_traits::SubmsgTrait,
    {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.debug_struct("Submsg")
                .field(
                    "i32_unlabeled",
                    &<Self as super::_puroro_traits::SubmsgTrait>::i32_unlabeled(self),
                )
                .finish()
        }
    }

    impl ::std::clone::Clone for Submsg {
        fn clone(&self) -> Self {
            Self {
                i32_unlabeled: ::std::clone::Clone::clone(&self.i32_unlabeled),
            }
        }
    }

    impl ::std::cmp::PartialEq for Submsg {
        fn eq(&self, rhs: &Self) -> bool {
            self.i32_unlabeled == rhs.i32_unlabeled && true
        }
    }
}

pub use _puroro_impls::*;
pub mod _puroro_impls {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
    use super::_puroro_traits::*;

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub g1_int32: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField1<ScalarType> where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field5MessageType<'this>
        where
            Self: 'this,
        = ();
        fn group_one<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this>>
        {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupOne as E;
            ::std::option::Option::Some(E::G1Int32(::std::convert::Into::into(
                ::std::clone::Clone::clone(&self.g1_int32),
            )))
        }
        fn group_two<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>>
        {
            ::std::option::Option::None
        }
        fn group_three<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            ::std::option::Option::None
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g1_int32_opt(self),
                1,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self { g1_int32: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub g1_string: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField2<ScalarType> where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field5MessageType<'this>
        where
            Self: 'this,
        = ();
        fn group_one<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this>>
        {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupOne as E;
            ::std::option::Option::Some(E::G1String(self.g1_string.as_ref()))
        }
        fn group_two<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>>
        {
            ::std::option::Option::None
        }
        fn group_three<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            ::std::option::Option::None
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g1_string_opt(self),
                2,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self { g1_string: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField3<ScalarType>
    where
        ScalarType: ::std::convert::Into<f32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub g2_f32: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField3<ScalarType> where
        ScalarType: ::std::convert::Into<f32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField3<ScalarType>
    where
        ScalarType: ::std::convert::Into<f32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field5MessageType<'this>
        where
            Self: 'this,
        = ();
        fn group_one<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this>>
        {
            ::std::option::Option::None
        }
        fn group_two<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>>
        {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
            ::std::option::Option::Some(E::G2F32(::std::convert::Into::into(
                ::std::clone::Clone::clone(&self.g2_f32),
            )))
        }
        fn group_three<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            ::std::option::Option::None
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField3<ScalarType>
    where
        ScalarType: ::std::convert::Into<f32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::Float,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g2_f32_opt(self),
                3,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField3<ScalarType>
    where
        ScalarType: ::std::convert::Into<f32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self { g2_f32: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField4<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub g2_string: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField4<ScalarType> where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField4<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field5MessageType<'this>
        where
            Self: 'this,
        = ();
        fn group_one<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this>>
        {
            ::std::option::Option::None
        }
        fn group_two<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>>
        {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
            ::std::option::Option::Some(E::G2String(self.g2_string.as_ref()))
        }
        fn group_three<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            ::std::option::Option::None
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField4<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g2_string_opt(self),
                4,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField4<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self { g2_string: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField5<ScalarType>
    where
        ScalarType: self::_puroro_root::oneofs3::_puroro_traits::SubmsgTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub g2_submsg: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField5<ScalarType> where
        ScalarType: self::_puroro_root::oneofs3::_puroro_traits::SubmsgTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField5<ScalarType>
    where
        ScalarType: self::_puroro_root::oneofs3::_puroro_traits::SubmsgTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field5MessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;
        fn group_one<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this>>
        {
            ::std::option::Option::None
        }
        fn group_two<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>>
        {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
            ::std::option::Option::Some(E::G2Submsg(&self.g2_submsg))
        }
        fn group_three<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            ::std::option::Option::None
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField5<ScalarType>
    where
        ScalarType: self::_puroro_root::oneofs3::_puroro_traits::SubmsgTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        Self: super::_puroro_traits::MsgTrait,
        for<'a> <Self as super::_puroro_traits::MsgTrait>::Field5MessageType<'a>:
            ::puroro::internal::se::SerMessageToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::Message<
                    <Self as super::_puroro_traits::MsgTrait>::Field5MessageType<'_>,
                >,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g2_submsg_opt(self),
                5,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField5<ScalarType>
    where
        ScalarType: self::_puroro_root::oneofs3::_puroro_traits::SubmsgTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self { g2_submsg: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MsgSingleField6<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub g3_int32: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField6<ScalarType> where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField6<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field5MessageType<'this>
        where
            Self: 'this,
        = ();
        fn group_one<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this>>
        {
            ::std::option::Option::None
        }
        fn group_two<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>>
        {
            ::std::option::Option::None
        }
        fn group_three<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupThree as E;
            ::std::option::Option::Some(E::G3Int32(::std::convert::Into::into(
                ::std::clone::Clone::clone(&self.g3_int32),
            )))
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField6<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g3_int32_opt(self),
                6,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField6<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self { g3_int32: value }
        }
    }
    pub struct MsgBumpalo<'bump> {
        _bump: &'bump ::puroro::bumpalo::Bump,
        _bitfield:
            ::puroro::bitvec::array::BitArray<::puroro::bitvec::order::Lsb0, [u32; (5 + 31) / 32]>,
        group_one: super::_puroro_nested::msg::_puroro_private_oneofs::GroupOneBumpalo<'bump>,
        group_two: super::_puroro_nested::msg::_puroro_private_oneofs::GroupTwoBumpalo<'bump>,
        group_three: super::_puroro_nested::msg::_puroro_private_oneofs::GroupThreeBumpalo<'bump>,
    }

    pub type MsgBumpaloOwned = ::puroro::BumpaloOwned<MsgBumpalo<'static>>;
    impl<'bump> MsgBumpalo<'bump> {
        pub fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
            #[allow(unused)]
            let bump_ref: &::puroro::bumpalo::Bump =
                unsafe { ::std::mem::transmute(::std::ops::Deref::deref(&bump)) };

            Self {
                _bump: bump,
                _bitfield: ::std::default::Default::default(),
                group_one: ::std::default::Default::default(),
                group_two: ::std::default::Default::default(),
                group_three: ::std::default::Default::default(),
            }
        }
    }
    impl<'bump> ::puroro::Message<super::_puroro_simple_impl::Msg> for MsgBumpalo<'bump> {}

    impl<'bump> ::puroro::BumpaloMessage<'bump> for MsgBumpalo<'bump> {
        fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
            Self::new_in(bump)
        }
    }

    impl<'bump> ::puroro::internal::BumpDefault<'bump> for MsgBumpalo<'bump> {
        fn default_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
            Self::new_in(bump)
        }
    }

    impl<'bump> super::_puroro_traits::MsgTrait for MsgBumpalo<'bump> {
        type Field5MessageType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::oneofs3::_puroro_impls::SubmsgBumpalo<'this>;
        fn group_one<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this>> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupOne as F;
            use super::_puroro_nested::msg::_puroro_private_oneofs::GroupOneBumpalo as E;
            match &self.group_one {
                E::_None(_) => ::std::option::Option::None,
                E::G1Int32(val) => ::std::option::Option::Some(F::G1Int32(val.inner().clone())),
                E::G1String(val) => ::std::option::Option::Some(F::G1String(val.as_ref())),
            }
        }
        fn group_two<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as F;
            use super::_puroro_nested::msg::_puroro_private_oneofs::GroupTwoBumpalo as E;
            match &self.group_two {
                E::_None(_) => ::std::option::Option::None,
                E::G2F32(val) => ::std::option::Option::Some(F::G2F32(val.inner().clone())),
                E::G2String(val) => ::std::option::Option::Some(F::G2String(val.as_ref())),
                E::G2Submsg(val) => ::std::option::Option::Some(F::G2Submsg(unsafe {
                    ::std::mem::transmute(val.as_ref())
                })),
            }
        }
        fn group_three<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupThree as F;
            use super::_puroro_nested::msg::_puroro_private_oneofs::GroupThreeBumpalo as E;
            match &self.group_three {
                E::_None(_) => ::std::option::Option::None,
                E::G3Int32(val) => ::std::option::Option::Some(F::G3Int32(val.inner().clone())),
            }
        }
    }

    impl<'bump> ::puroro::internal::de::DeserMessageFromBytesIter for MsgBumpalo<'bump> {
        fn deser_field<'this, I>(
            &'this mut self,
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
                1 => {
                    use super::_puroro_nested::msg::_puroro_private_oneofs::GroupOneBumpalo as E;
                    if !matches!(&self.group_one, E::G1Int32(_)) {
                        self.group_one =
                            E::G1Int32(::puroro::internal::BumpDefault::default_in(self._bump));
                    }
                    let field_value_mut_ref = match &mut self.group_one {
                        E::G1Int32(v) => v,
                        _ => unreachable!(),
                    };
                    DeserFieldFromBytesIter::<
                    ::puroro::tags::OneofField, ::puroro::tags::Int32,
                >::deser_field(field_value_mut_ref, data, self._bump)
                }
                2 => {
                    use super::_puroro_nested::msg::_puroro_private_oneofs::GroupOneBumpalo as E;
                    if !matches!(&self.group_one, E::G1String(_)) {
                        self.group_one =
                            E::G1String(::puroro::internal::BumpDefault::default_in(self._bump));
                    }
                    let field_value_mut_ref = match &mut self.group_one {
                        E::G1String(v) => v,
                        _ => unreachable!(),
                    };
                    DeserFieldFromBytesIter::<
                    ::puroro::tags::OneofField, ::puroro::tags::String,
                >::deser_field(field_value_mut_ref, data, self._bump)
                }
                3 => {
                    use super::_puroro_nested::msg::_puroro_private_oneofs::GroupTwoBumpalo as E;
                    if !matches!(&self.group_two, E::G2F32(_)) {
                        self.group_two =
                            E::G2F32(::puroro::internal::BumpDefault::default_in(self._bump));
                    }
                    let field_value_mut_ref = match &mut self.group_two {
                        E::G2F32(v) => v,
                        _ => unreachable!(),
                    };
                    DeserFieldFromBytesIter::<
                    ::puroro::tags::OneofField, ::puroro::tags::Float,
                >::deser_field(field_value_mut_ref, data, self._bump)
                }
                4 => {
                    use super::_puroro_nested::msg::_puroro_private_oneofs::GroupTwoBumpalo as E;
                    if !matches!(&self.group_two, E::G2String(_)) {
                        self.group_two =
                            E::G2String(::puroro::internal::BumpDefault::default_in(self._bump));
                    }
                    let field_value_mut_ref = match &mut self.group_two {
                        E::G2String(v) => v,
                        _ => unreachable!(),
                    };
                    DeserFieldFromBytesIter::<
                    ::puroro::tags::OneofField, ::puroro::tags::String,
                >::deser_field(field_value_mut_ref, data, self._bump)
                }
                5 => {
                    use super::_puroro_nested::msg::_puroro_private_oneofs::GroupTwoBumpalo as E;
                    if !matches!(&self.group_two, E::G2Submsg(_)) {
                        self.group_two = E::G2Submsg(::puroro::internal::Bare::new(
                            ::puroro::BumpaloMessage::new_in(self._bump),
                        ));
                    }
                    let field_value_mut_ref = match &mut self.group_two {
                        E::G2Submsg(v) => v,
                        _ => unreachable!(),
                    };
                    DeserFieldFromBytesIter::<
                        ::puroro::tags::OneofField,
                        ::puroro::tags::Message<
                            ::puroro::internal::NoAllocBumpBox<
                                self::_puroro_root::oneofs3::_puroro_impls::SubmsgBumpalo<'bump>,
                            >,
                        >,
                    >::deser_field(field_value_mut_ref, data, self._bump)
                }
                6 => {
                    use super::_puroro_nested::msg::_puroro_private_oneofs::GroupThreeBumpalo as E;
                    if !matches!(&self.group_three, E::G3Int32(_)) {
                        self.group_three =
                            E::G3Int32(::puroro::internal::BumpDefault::default_in(self._bump));
                    }
                    let field_value_mut_ref = match &mut self.group_three {
                        E::G3Int32(v) => v,
                        _ => unreachable!(),
                    };
                    DeserFieldFromBytesIter::<
                    ::puroro::tags::OneofField, ::puroro::tags::Int32,
                >::deser_field(field_value_mut_ref, data, self._bump)
                }

                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl<'bump> ::puroro::internal::se::SerMessageToIoWrite for MsgBumpalo<'bump>
    where
        Self: super::_puroro_traits::MsgTrait,
        for<'a> <Self as super::_puroro_traits::MsgTrait>::Field5MessageType<'a>:
            ::puroro::internal::se::SerMessageToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g1_int32_opt(self),
                1,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g1_string_opt(self),
                2,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::Float,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g2_f32_opt(self),
                3,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::String,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g2_string_opt(self),
                4,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::Message<
                    <Self as super::_puroro_traits::MsgTrait>::Field5MessageType<'_>,
                >,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g2_submsg_opt(self),
                5,
                out,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::OneofField,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::g3_int32_opt(self),
                6,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }
    pub struct MsgBuilder<T>(T);

    impl<T> MsgBuilder<T>
    where
        T: MsgTrait,
    {
        pub fn append_g1_int32<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField1<ScalarType>)>
        where
            ScalarType: ::std::convert::Into<i32>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            MsgBuilder((self.0, MsgSingleField1 { g1_int32: value }))
        }

        pub fn append_g1_string<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField2<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            MsgBuilder((self.0, MsgSingleField2 { g1_string: value }))
        }

        pub fn append_g2_f32<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField3<ScalarType>)>
        where
            ScalarType: ::std::convert::Into<f32>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            MsgBuilder((self.0, MsgSingleField3 { g2_f32: value }))
        }

        pub fn append_g2_string<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField4<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            MsgBuilder((self.0, MsgSingleField4 { g2_string: value }))
        }

        pub fn append_g2_submsg<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField5<ScalarType>)>
        where
            ScalarType: self::_puroro_root::oneofs3::_puroro_traits::SubmsgTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            MsgBuilder((self.0, MsgSingleField5 { g2_submsg: value }))
        }

        pub fn append_g3_int32<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField6<ScalarType>)>
        where
            ScalarType: ::std::convert::Into<i32>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            MsgBuilder((self.0, MsgSingleField6 { g3_int32: value }))
        }

        pub fn build(self) -> T {
            self.0
        }
    }

    impl MsgBuilder<()> {
        pub fn new() -> Self {
            Self(())
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct SubmsgSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub i32_unlabeled: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Submsg> for SubmsgSingleField1<ScalarType> where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::SubmsgTrait for SubmsgSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn i32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::Some(::std::convert::Into::into(::std::clone::Clone::clone(
                &self.i32_unlabeled,
            )))
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for SubmsgSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        Self: super::_puroro_traits::SubmsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::SubmsgTrait>::i32_unlabeled_opt(self),
                1,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for SubmsgSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::Into<i32>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                i32_unlabeled: value,
            }
        }
    }
    pub struct SubmsgBumpalo<'bump> {
        _bump: &'bump ::puroro::bumpalo::Bump,
        _bitfield:
            ::puroro::bitvec::array::BitArray<::puroro::bitvec::order::Lsb0, [u32; (0 + 31) / 32]>,
        i32_unlabeled: ::puroro::internal::Bare<i32>,
    }

    pub type SubmsgBumpaloOwned = ::puroro::BumpaloOwned<SubmsgBumpalo<'static>>;
    impl<'bump> SubmsgBumpalo<'bump> {
        pub fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
            #[allow(unused)]
            let bump_ref: &::puroro::bumpalo::Bump =
                unsafe { ::std::mem::transmute(::std::ops::Deref::deref(&bump)) };

            Self {
                _bump: bump,
                _bitfield: ::std::default::Default::default(),
                i32_unlabeled: ::std::default::Default::default(),
            }
        }
        pub fn i32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            if !::puroro::internal::IsDefault::is_default(&*self.i32_unlabeled) {
                ::std::option::Option::Some(self.i32_unlabeled.inner())
            } else {
                ::std::option::Option::None
            }
        }
        pub fn i32_unlabeled<'this>(&'this self) -> i32 {
            match self.i32_unlabeled_opt() {
                ::std::option::Option::Some(x) => x,
                _ => ::std::default::Default::default(),
            }
        }
        pub fn has_i32_unlabeled(&self) -> bool {
            self.i32_unlabeled_opt().is_some()
        }
        pub fn clear_i32_unlabeled(&mut self) {
            self.i32_unlabeled = ::std::default::Default::default();
        }
        pub fn i32_unlabeled_mut<'this>(&'this mut self) -> &'this mut i32 {
            if !self.has_i32_unlabeled() {
                self.i32_unlabeled = ::std::default::Default::default();
            }
            &mut self.i32_unlabeled
        }
    }
    impl<'bump> ::puroro::Message<super::_puroro_simple_impl::Submsg> for SubmsgBumpalo<'bump> {}

    impl<'bump> ::puroro::BumpaloMessage<'bump> for SubmsgBumpalo<'bump> {
        fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
            Self::new_in(bump)
        }
    }

    impl<'bump> ::puroro::internal::BumpDefault<'bump> for SubmsgBumpalo<'bump> {
        fn default_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
            Self::new_in(bump)
        }
    }

    impl<'bump> super::_puroro_traits::SubmsgTrait for SubmsgBumpalo<'bump> {
        fn i32_unlabeled_opt<'this>(&'this self) -> Option<i32> {
            <Self>::i32_unlabeled_opt(self)
        }
    }

    impl<'bump> ::puroro::internal::de::DeserMessageFromBytesIter for SubmsgBumpalo<'bump> {
        fn deser_field<'this, I>(
            &'this mut self,
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
            1 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled, ::puroro::tags::Int32
                >::deser_field(&mut self.i32_unlabeled, data, self._bump)
            }

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl<'bump> ::puroro::internal::se::SerMessageToIoWrite for SubmsgBumpalo<'bump>
    where
        Self: super::_puroro_traits::SubmsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::SubmsgTrait>::i32_unlabeled_opt(self),
                1,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }
    pub struct SubmsgBuilder<T>(T);

    impl<T> SubmsgBuilder<T>
    where
        T: SubmsgTrait,
    {
        pub fn append_i32_unlabeled<ScalarType>(
            self,
            value: ScalarType,
        ) -> SubmsgBuilder<(T, SubmsgSingleField1<ScalarType>)>
        where
            ScalarType: ::std::convert::Into<i32>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            SubmsgBuilder((
                self.0,
                SubmsgSingleField1 {
                    i32_unlabeled: value,
                },
            ))
        }

        pub fn build(self) -> T {
            self.0
        }
    }

    impl SubmsgBuilder<()> {
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

    pub trait MsgTrait {
        fn g1_int32<'this>(&'this self) -> i32 {
            self.g1_int32_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_g1_int32<'this>(&'this self) -> bool {
            self.g1_int32_opt().is_some()
        }
        fn g1_string<'this>(&'this self) -> &'this str {
            self.g1_string_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_g1_string<'this>(&'this self) -> bool {
            self.g1_string_opt().is_some()
        }
        fn g2_f32<'this>(&'this self) -> f32 {
            self.g2_f32_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_g2_f32<'this>(&'this self) -> bool {
            self.g2_f32_opt().is_some()
        }
        fn g2_string<'this>(&'this self) -> &'this str {
            self.g2_string_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_g2_string<'this>(&'this self) -> bool {
            self.g2_string_opt().is_some()
        }
        type Field5MessageType<'this>: self::_puroro_root::oneofs3::_puroro_traits::SubmsgTrait
        where
            Self: 'this;
        fn g2_submsg<'this>(&'this self) -> ::std::option::Option<Self::Field5MessageType<'this>> {
            self.g2_submsg_opt()
        }
        fn has_g2_submsg<'this>(&'this self) -> bool {
            self.g2_submsg_opt().is_some()
        }
        fn g3_int32<'this>(&'this self) -> i32 {
            self.g3_int32_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_g3_int32<'this>(&'this self) -> bool {
            self.g3_int32_opt().is_some()
        }
        fn group_one<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this>>
        {
            ::std::option::Option::None
        }
        fn g1_int32_opt<'this>(&'this self) -> Option<i32> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupOne as E;
            self.group_one().and_then(|oneof| {
                #[allow(irrefutable_let_patterns)]
                if let E::G1Int32(v) = oneof {
                    ::std::option::Option::Some(v)
                } else {
                    ::std::option::Option::None
                }
            })
        }
        fn g1_string_opt<'this>(&'this self) -> Option<&'this str> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupOne as E;
            self.group_one().and_then(|oneof| {
                #[allow(irrefutable_let_patterns)]
                if let E::G1String(v) = oneof {
                    ::std::option::Option::Some(v)
                } else {
                    ::std::option::Option::None
                }
            })
        }
        fn group_two<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>>
        {
            ::std::option::Option::None
        }
        fn g2_f32_opt<'this>(&'this self) -> Option<f32> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
            self.group_two().and_then(|oneof| {
                #[allow(irrefutable_let_patterns)]
                if let E::G2F32(v) = oneof {
                    ::std::option::Option::Some(v)
                } else {
                    ::std::option::Option::None
                }
            })
        }
        fn g2_string_opt<'this>(&'this self) -> Option<&'this str> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
            self.group_two().and_then(|oneof| {
                #[allow(irrefutable_let_patterns)]
                if let E::G2String(v) = oneof {
                    ::std::option::Option::Some(v)
                } else {
                    ::std::option::Option::None
                }
            })
        }
        fn g2_submsg_opt<'this>(
            &'this self,
        ) -> Option<
            <Self as self::_puroro_root::oneofs3::_puroro_traits::MsgTrait>::Field5MessageType<
                'this,
            >,
        > {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
            self.group_two().and_then(|oneof| {
                #[allow(irrefutable_let_patterns)]
                if let E::G2Submsg(v) = oneof {
                    ::std::option::Option::Some(v)
                } else {
                    ::std::option::Option::None
                }
            })
        }
        fn group_three<'this>(
            &'this self,
        ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            ::std::option::Option::None
        }
        fn g3_int32_opt<'this>(&'this self) -> Option<i32> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupThree as E;
            self.group_three().and_then(|oneof| {
                #[allow(irrefutable_let_patterns)]
                if let E::G3Int32(v) = oneof {
                    ::std::option::Option::Some(v)
                } else {
                    ::std::option::Option::None
                }
            })
        }
    }

    macro_rules! msg_delegate {
        ($ty:ty) => {
            type Field5MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field5MessageType<'this>;
            fn group_one<'this>(
                &'this self,
            ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this>>
            {
                (**self).group_one().map(|v| v.into())
            }
            fn group_two<'this>(
                &'this self,
            ) -> ::std::option::Option<
                super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>,
            > {
                (**self).group_two().map(|v| v.into())
            }
            fn group_three<'this>(
                &'this self,
            ) -> ::std::option::Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
                (**self).group_three().map(|v| v.into())
            }
        };
    }

    impl<T> MsgTrait for &'_ T
    where
        T: MsgTrait,
    {
        msg_delegate!(T);
    }

    impl<T> MsgTrait for &'_ mut T
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

    impl<'bump, T> MsgTrait for ::puroro::bumpalo::boxed::Box<'bump, T>
    where
        T: MsgTrait,
    {
        msg_delegate!(T);
    }

    impl<T> MsgTrait for ::puroro::BumpaloOwned<T>
    where
        T: MsgTrait,
    {
        msg_delegate!(T);
    }
    impl MsgTrait for () {
        type Field5MessageType<'this>
        where
            Self: 'this,
        = ();
        fn group_one<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this>> {
            None
        }
        fn group_two<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>> {
            None
        }
        fn group_three<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            None
        }
    }
    impl<T, U> MsgTrait for (T, U)
    where
        T: MsgTrait,
        U: MsgTrait,
    {
        type Field5MessageType<'this>
        where
            Self: 'this,
        = (
            ::std::option::Option<<T as MsgTrait>::Field5MessageType<'this>>,
            ::std::option::Option<<U as MsgTrait>::Field5MessageType<'this>>,
        );
        fn group_one<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this>> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupOne as E;
            Some(match (self.0.group_one(), self.1.group_one()) {
                (None, None) => {
                    return None;
                }
                (Some(E::G1Int32(_)), Some(E::G1Int32(right))) => E::G1Int32(right),
                (Some(E::G1String(_)), Some(E::G1String(right))) => E::G1String(right),
                (_, Some(E::G1Int32(right))) => E::G1Int32(right),
                (_, Some(E::G1String(right))) => E::G1String(right),
                (Some(E::G1Int32(left)), None) => E::G1Int32(left),
                (Some(E::G1String(left)), None) => E::G1String(left),
            })
        }
        fn group_two<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
            Some(match (self.0.group_two(), self.1.group_two()) {
                (None, None) => {
                    return None;
                }
                (Some(E::G2F32(_)), Some(E::G2F32(right))) => E::G2F32(right),
                (Some(E::G2String(_)), Some(E::G2String(right))) => E::G2String(right),
                (Some(E::G2Submsg(left)), Some(E::G2Submsg(right))) => {
                    E::G2Submsg((Some(left), Some(right)))
                }
                (_, Some(E::G2F32(right))) => E::G2F32(right),
                (_, Some(E::G2String(right))) => E::G2String(right),
                (_, Some(E::G2Submsg(right))) => E::G2Submsg((None, Some(right))),
                (Some(E::G2F32(left)), None) => E::G2F32(left),
                (Some(E::G2String(left)), None) => E::G2String(left),
                (Some(E::G2Submsg(left)), None) => E::G2Submsg((Some(left), None)),
            })
        }
        fn group_three<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupThree as E;
            Some(match (self.0.group_three(), self.1.group_three()) {
                (None, None) => {
                    return None;
                }
                (Some(E::G3Int32(_)), Some(E::G3Int32(right))) => E::G3Int32(right),
                (_, Some(E::G3Int32(right))) => E::G3Int32(right),
                (Some(E::G3Int32(left)), None) => E::G3Int32(left),
            })
        }
    }
    impl<T, U> MsgTrait for ::puroro::Either<T, U>
    where
        T: MsgTrait,
        U: MsgTrait,
    {
        type Field5MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as MsgTrait>::Field5MessageType<'this>,
            <U as MsgTrait>::Field5MessageType<'this>,
        >;
        fn group_one<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this>> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupOne as E;
            match self.as_ref().either(
                |t| t.group_one().map(|t| ::puroro::Either::Left(t)),
                |u| u.group_one().map(|u| ::puroro::Either::Right(u)),
            ) {
                Some(::puroro::Either::Left(E::G1Int32(v))) => Some(E::G1Int32(v)),
                Some(::puroro::Either::Right(E::G1Int32(v))) => Some(E::G1Int32(v)),
                Some(::puroro::Either::Left(E::G1String(v))) => Some(E::G1String(v)),
                Some(::puroro::Either::Right(E::G1String(v))) => Some(E::G1String(v)),
                None => None,
            }
        }
        fn group_two<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
            match self.as_ref().either(
                |t| t.group_two().map(|t| ::puroro::Either::Left(t)),
                |u| u.group_two().map(|u| ::puroro::Either::Right(u)),
            ) {
                Some(::puroro::Either::Left(E::G2F32(v))) => Some(E::G2F32(v)),
                Some(::puroro::Either::Right(E::G2F32(v))) => Some(E::G2F32(v)),
                Some(::puroro::Either::Left(E::G2String(v))) => Some(E::G2String(v)),
                Some(::puroro::Either::Right(E::G2String(v))) => Some(E::G2String(v)),
                Some(::puroro::Either::Left(E::G2Submsg(v))) => {
                    Some(E::G2Submsg(::puroro::Either::Left(v)))
                }
                Some(::puroro::Either::Right(E::G2Submsg(v))) => {
                    Some(E::G2Submsg(::puroro::Either::Right(v)))
                }
                None => None,
            }
        }
        fn group_three<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupThree as E;
            match self.as_ref().either(
                |t| t.group_three().map(|t| ::puroro::Either::Left(t)),
                |u| u.group_three().map(|u| ::puroro::Either::Right(u)),
            ) {
                Some(::puroro::Either::Left(E::G3Int32(v))) => Some(E::G3Int32(v)),
                Some(::puroro::Either::Right(E::G3Int32(v))) => Some(E::G3Int32(v)),
                None => None,
            }
        }
    }
    impl<T> MsgTrait for ::std::option::Option<T>
    where
        T: MsgTrait,
    {
        type Field5MessageType<'this>
        where
            Self: 'this,
        = T::Field5MessageType<'this>;
        fn group_one<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupOne<'this>> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupOne as E;
            self.as_ref().and_then(|msg| {
                msg.group_one().map(|oneof| match oneof {
                    E::G1Int32(v) => E::G1Int32(v),
                    E::G1String(v) => E::G1String(v),
                })
            })
        }
        fn group_two<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupTwo<'this, Self>> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupTwo as E;
            self.as_ref().and_then(|msg| {
                msg.group_two().map(|oneof| match oneof {
                    E::G2F32(v) => E::G2F32(v),
                    E::G2String(v) => E::G2String(v),
                    E::G2Submsg(v) => E::G2Submsg(v),
                })
            })
        }
        fn group_three<'this>(
            &'this self,
        ) -> Option<super::_puroro_nested::msg::_puroro_oneofs::GroupThree> {
            use super::_puroro_nested::msg::_puroro_oneofs::GroupThree as E;
            self.as_ref().and_then(|msg| {
                msg.group_three().map(|oneof| match oneof {
                    E::G3Int32(v) => E::G3Int32(v),
                })
            })
        }
    }

    pub trait SubmsgTrait {
        fn i32_unlabeled<'this>(&'this self) -> i32 {
            self.i32_unlabeled_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_i32_unlabeled<'this>(&'this self) -> bool {
            self.i32_unlabeled_opt().is_some()
        }
        fn i32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::None
        }
    }

    macro_rules! submsg_delegate {
        ($ty:ty) => {
            fn i32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).i32_unlabeled_opt()
            }
        };
    }

    impl<T> SubmsgTrait for &'_ T
    where
        T: SubmsgTrait,
    {
        submsg_delegate!(T);
    }

    impl<T> SubmsgTrait for &'_ mut T
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

    impl<'bump, T> SubmsgTrait for ::puroro::bumpalo::boxed::Box<'bump, T>
    where
        T: SubmsgTrait,
    {
        submsg_delegate!(T);
    }

    impl<T> SubmsgTrait for ::puroro::BumpaloOwned<T>
    where
        T: SubmsgTrait,
    {
        submsg_delegate!(T);
    }
    impl SubmsgTrait for () {}
    impl<T, U> SubmsgTrait for (T, U)
    where
        T: SubmsgTrait,
        U: SubmsgTrait,
    {
        fn i32_unlabeled_opt<'this>(&'this self) -> Option<i32> {
            <U as SubmsgTrait>::i32_unlabeled_opt(&self.1)
                .or_else(|| <T as SubmsgTrait>::i32_unlabeled_opt(&self.0))
        }
    }
    impl<T, U> SubmsgTrait for ::puroro::Either<T, U>
    where
        T: SubmsgTrait,
        U: SubmsgTrait,
    {
        fn i32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().either(
                |t| <T as SubmsgTrait>::i32_unlabeled_opt(t),
                |u| <U as SubmsgTrait>::i32_unlabeled_opt(u),
            )
        }
    }
    impl<T> SubmsgTrait for ::std::option::Option<T>
    where
        T: SubmsgTrait,
    {
        fn i32_unlabeled_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().and_then(|msg| msg.i32_unlabeled_opt())
        }
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
                G1String(&'msg str),
            }

            impl<'msg> GroupOne<'msg> {
                pub fn g1_int32(self) -> ::std::option::Option<i32> {
                    match self {
                        Self::G1Int32(v) => ::std::option::Option::Some(v),
                        #[allow(unreachable_patterns)]
                        _ => None,
                    }
                }
                pub fn g1_string(self) -> ::std::option::Option<&'msg str> {
                    match self {
                        Self::G1String(v) => ::std::option::Option::Some(v),
                        #[allow(unreachable_patterns)]
                        _ => None,
                    }
                }
            }

            impl<'msg> ::std::fmt::Debug for GroupOne<'msg>
            where
                i32: ::std::fmt::Debug,
                &'msg str: ::std::fmt::Debug,
            {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    match self {
                        Self::G1Int32(v) => f.debug_tuple("GroupOne::G1Int32").field(&v).finish(),
                        Self::G1String(v) => f.debug_tuple("GroupOne::G1String").field(&v).finish(),
                    }
                }
            }

            impl<'msg> ::std::clone::Clone for GroupOne<'msg>
            where
                i32: ::std::clone::Clone,
                &'msg str: ::std::clone::Clone,
            {
                fn clone(&self) -> Self {
                    match self {
                        Self::G1Int32(v) => Self::G1Int32(::std::clone::Clone::clone(&v)),
                        Self::G1String(v) => Self::G1String(::std::clone::Clone::clone(&v)),
                    }
                }
            }

            impl<'msg> ::std::cmp::PartialEq for GroupOne<'msg>
            where
                i32: ::std::cmp::PartialEq,
                &'msg str: ::std::cmp::PartialEq,
            {
                fn eq(&self, rhs: &Self) -> bool {
                    match (self, rhs) {
                        (Self::G1Int32(left), Self::G1Int32(right)) => left == right,
                        (Self::G1String(left), Self::G1String(right)) => left == right,
                        #[allow(unreachable_patterns)]
                        _ => false,
                    }
                }
            }

            pub enum GroupTwo<'msg, T>
            where
                T: 'msg + ?Sized + self::_puroro_root::oneofs3::_puroro_traits::MsgTrait,
            {
                G2F32(f32),
                G2String(&'msg str),
                G2Submsg(<T as self::_puroro_root::oneofs3::_puroro_traits::MsgTrait>::Field5MessageType<'msg>),
            }

            impl<'msg, T> GroupTwo<'msg, T>
            where
                T: 'msg + ?Sized + self::_puroro_root::oneofs3::_puroro_traits::MsgTrait,
            {
                pub fn g2_f32(self) -> ::std::option::Option<f32> {
                    match self {
                        Self::G2F32(v) => ::std::option::Option::Some(v),
                        #[allow(unreachable_patterns)]
                        _ => None,
                    }
                }
                pub fn g2_string(self) -> ::std::option::Option<&'msg str> {
                    match self {
                        Self::G2String(v) => ::std::option::Option::Some(v),
                        #[allow(unreachable_patterns)]
                        _ => None,
                    }
                }
                pub fn g2_submsg(
                    self,
                ) -> ::std::option::Option<
                    <T as self::_puroro_root::oneofs3::_puroro_traits::MsgTrait>::Field5MessageType<
                        'msg,
                    >,
                > {
                    match self {
                        Self::G2Submsg(v) => ::std::option::Option::Some(v),
                        #[allow(unreachable_patterns)]
                        _ => None,
                    }
                }
            }

            impl<'msg, T> ::std::fmt::Debug for GroupTwo<'msg, T>
            where
                f32: ::std::fmt::Debug,
                &'msg str: ::std::fmt::Debug,
                <T as self::_puroro_root::oneofs3::_puroro_traits::MsgTrait>::Field5MessageType<
                    'msg,
                >: ::std::fmt::Debug,
                T: 'msg + ?Sized + self::_puroro_root::oneofs3::_puroro_traits::MsgTrait,
            {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    match self {
                        Self::G2F32(v) => f.debug_tuple("GroupTwo::G2F32").field(&v).finish(),
                        Self::G2String(v) => f.debug_tuple("GroupTwo::G2String").field(&v).finish(),
                        Self::G2Submsg(v) => f.debug_tuple("GroupTwo::G2Submsg").field(&v).finish(),
                    }
                }
            }

            impl<'msg, T> ::std::clone::Clone for GroupTwo<'msg, T>
            where
                f32: ::std::clone::Clone,
                &'msg str: ::std::clone::Clone,
                <T as self::_puroro_root::oneofs3::_puroro_traits::MsgTrait>::Field5MessageType<
                    'msg,
                >: ::std::clone::Clone,
                T: 'msg + ?Sized + self::_puroro_root::oneofs3::_puroro_traits::MsgTrait,
            {
                fn clone(&self) -> Self {
                    match self {
                        Self::G2F32(v) => Self::G2F32(::std::clone::Clone::clone(&v)),
                        Self::G2String(v) => Self::G2String(::std::clone::Clone::clone(&v)),
                        Self::G2Submsg(v) => Self::G2Submsg(::std::clone::Clone::clone(&v)),
                    }
                }
            }

            impl<'msg, T> ::std::cmp::PartialEq for GroupTwo<'msg, T>
            where
                f32: ::std::cmp::PartialEq,
                &'msg str: ::std::cmp::PartialEq,
                <T as self::_puroro_root::oneofs3::_puroro_traits::MsgTrait>::Field5MessageType<
                    'msg,
                >: ::std::cmp::PartialEq,
                T: 'msg + ?Sized + self::_puroro_root::oneofs3::_puroro_traits::MsgTrait,
            {
                fn eq(&self, rhs: &Self) -> bool {
                    match (self, rhs) {
                        (Self::G2F32(left), Self::G2F32(right)) => left == right,
                        (Self::G2String(left), Self::G2String(right)) => left == right,
                        (Self::G2Submsg(left), Self::G2Submsg(right)) => left == right,
                        #[allow(unreachable_patterns)]
                        _ => false,
                    }
                }
            }
            impl<'msg, T> ::std::convert::From<GroupTwo<'msg, T>> for GroupTwo<'msg, &'_ T>
            where
                T: 'msg + self::_puroro_root::oneofs3::_puroro_traits::MsgTrait,
            {
                fn from(value: GroupTwo<'msg, T>) -> Self {
                    match value {
                        GroupTwo::G2F32(v) => GroupTwo::G2F32(v),
                        GroupTwo::G2String(v) => GroupTwo::G2String(v),
                        GroupTwo::G2Submsg(v) => GroupTwo::G2Submsg(v),
                    }
                }
            }
            impl<'msg, T> ::std::convert::From<GroupTwo<'msg, T>> for GroupTwo<'msg, &'_ mut T>
            where
                T: 'msg + self::_puroro_root::oneofs3::_puroro_traits::MsgTrait,
            {
                fn from(value: GroupTwo<'msg, T>) -> Self {
                    match value {
                        GroupTwo::G2F32(v) => GroupTwo::G2F32(v),
                        GroupTwo::G2String(v) => GroupTwo::G2String(v),
                        GroupTwo::G2Submsg(v) => GroupTwo::G2Submsg(v),
                    }
                }
            }
            impl<'msg, T> ::std::convert::From<GroupTwo<'msg, T>> for GroupTwo<'msg, ::std::boxed::Box<T>>
            where
                T: 'msg + self::_puroro_root::oneofs3::_puroro_traits::MsgTrait,
            {
                fn from(value: GroupTwo<'msg, T>) -> Self {
                    match value {
                        GroupTwo::G2F32(v) => GroupTwo::G2F32(v),
                        GroupTwo::G2String(v) => GroupTwo::G2String(v),
                        GroupTwo::G2Submsg(v) => GroupTwo::G2Submsg(v),
                    }
                }
            }
            impl<'msg, 'bump, T> ::std::convert::From<GroupTwo<'msg, T>>
                for GroupTwo<'msg, ::puroro::bumpalo::boxed::Box<'bump, T>>
            where
                T: 'msg + self::_puroro_root::oneofs3::_puroro_traits::MsgTrait,
            {
                fn from(value: GroupTwo<'msg, T>) -> Self {
                    match value {
                        GroupTwo::G2F32(v) => GroupTwo::G2F32(v),
                        GroupTwo::G2String(v) => GroupTwo::G2String(v),
                        GroupTwo::G2Submsg(v) => GroupTwo::G2Submsg(v),
                    }
                }
            }
            impl<'msg, 'bump, T> ::std::convert::From<GroupTwo<'msg, T>>
                for GroupTwo<'msg, ::puroro::BumpaloOwned<T>>
            where
                T: 'msg + self::_puroro_root::oneofs3::_puroro_traits::MsgTrait,
            {
                fn from(value: GroupTwo<'msg, T>) -> Self {
                    match value {
                        GroupTwo::G2F32(v) => GroupTwo::G2F32(v),
                        GroupTwo::G2String(v) => GroupTwo::G2String(v),
                        GroupTwo::G2Submsg(v) => GroupTwo::G2Submsg(v),
                    }
                }
            }

            pub enum GroupThree {
                G3Int32(i32),
            }

            impl GroupThree {
                pub fn g3_int32(self) -> ::std::option::Option<i32> {
                    match self {
                        Self::G3Int32(v) => ::std::option::Option::Some(v),
                        #[allow(unreachable_patterns)]
                        _ => None,
                    }
                }
            }

            impl ::std::fmt::Debug for GroupThree
            where
                i32: ::std::fmt::Debug,
            {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    match self {
                        Self::G3Int32(v) => f.debug_tuple("GroupThree::G3Int32").field(&v).finish(),
                    }
                }
            }

            impl ::std::clone::Clone for GroupThree
            where
                i32: ::std::clone::Clone,
            {
                fn clone(&self) -> Self {
                    match self {
                        Self::G3Int32(v) => Self::G3Int32(::std::clone::Clone::clone(&v)),
                    }
                }
            }

            impl ::std::cmp::PartialEq for GroupThree
            where
                i32: ::std::cmp::PartialEq,
            {
                fn eq(&self, rhs: &Self) -> bool {
                    match (self, rhs) {
                        (Self::G3Int32(left), Self::G3Int32(right)) => left == right,
                        #[allow(unreachable_patterns)]
                        _ => false,
                    }
                }
            }
        }
        pub mod _puroro_private_oneofs {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }

            pub(crate) enum GroupOneSimple {
                G1Int32(i32),
                G1String(::std::string::String),
            }

            impl ::std::fmt::Debug for GroupOneSimple {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    match self {
                        Self::G1Int32(v) => {
                            f.debug_tuple("GroupOneSimple::G1Int32").field(&v).finish()
                        }
                        Self::G1String(v) => {
                            f.debug_tuple("GroupOneSimple::G1String").field(&v).finish()
                        }
                    }
                }
            }

            impl ::std::clone::Clone for GroupOneSimple {
                fn clone(&self) -> Self {
                    match self {
                        Self::G1Int32(v) => Self::G1Int32(::std::clone::Clone::clone(&v)),
                        Self::G1String(v) => Self::G1String(::std::clone::Clone::clone(&v)),
                    }
                }
            }

            impl ::std::cmp::PartialEq for GroupOneSimple {
                fn eq(&self, rhs: &Self) -> bool {
                    match (self, rhs) {
                        (Self::G1Int32(left), Self::G1Int32(right)) => left == right,
                        (Self::G1String(left), Self::G1String(right)) => left == right,
                        #[allow(unreachable_patterns)]
                        _ => false,
                    }
                }
            }
            pub(crate) enum GroupOneBumpalo<'bump> {
                _None(::std::marker::PhantomData<&'bump ()>),
                G1Int32(::puroro::internal::Bare<i32>),
                G1String(::puroro::internal::Bare<::puroro::internal::NoAllocBumpString>),
            }

            impl<'bump> ::std::default::Default for GroupOneBumpalo<'bump> {
                fn default() -> Self {
                    Self::_None(::std::marker::PhantomData)
                }
            }

            pub(crate) enum GroupTwoSimple {
                G2F32(f32),
                G2String(::std::string::String),
                G2Submsg(
                    ::std::boxed::Box<self::_puroro_root::oneofs3::_puroro_simple_impl::Submsg>,
                ),
            }

            impl ::std::fmt::Debug for GroupTwoSimple {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    match self {
                        Self::G2F32(v) => f.debug_tuple("GroupTwoSimple::G2F32").field(&v).finish(),
                        Self::G2String(v) => {
                            f.debug_tuple("GroupTwoSimple::G2String").field(&v).finish()
                        }
                        Self::G2Submsg(v) => {
                            f.debug_tuple("GroupTwoSimple::G2Submsg").field(&v).finish()
                        }
                    }
                }
            }

            impl ::std::clone::Clone for GroupTwoSimple {
                fn clone(&self) -> Self {
                    match self {
                        Self::G2F32(v) => Self::G2F32(::std::clone::Clone::clone(&v)),
                        Self::G2String(v) => Self::G2String(::std::clone::Clone::clone(&v)),
                        Self::G2Submsg(v) => Self::G2Submsg(::std::clone::Clone::clone(&v)),
                    }
                }
            }

            impl ::std::cmp::PartialEq for GroupTwoSimple {
                fn eq(&self, rhs: &Self) -> bool {
                    match (self, rhs) {
                        (Self::G2F32(left), Self::G2F32(right)) => left == right,
                        (Self::G2String(left), Self::G2String(right)) => left == right,
                        (Self::G2Submsg(left), Self::G2Submsg(right)) => left == right,
                        #[allow(unreachable_patterns)]
                        _ => false,
                    }
                }
            }
            pub(crate) enum GroupTwoBumpalo<'bump> {
                _None(::std::marker::PhantomData<&'bump ()>),
                G2F32(::puroro::internal::Bare<f32>),
                G2String(::puroro::internal::Bare<::puroro::internal::NoAllocBumpString>),
                G2Submsg(
                    ::puroro::internal::Bare<
                        ::puroro::internal::NoAllocBumpBox<
                            self::_puroro_root::oneofs3::_puroro_impls::SubmsgBumpalo<'bump>,
                        >,
                    >,
                ),
            }

            impl<'bump> ::std::default::Default for GroupTwoBumpalo<'bump> {
                fn default() -> Self {
                    Self::_None(::std::marker::PhantomData)
                }
            }

            pub(crate) enum GroupThreeSimple {
                G3Int32(i32),
            }

            impl ::std::fmt::Debug for GroupThreeSimple {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    match self {
                        Self::G3Int32(v) => f
                            .debug_tuple("GroupThreeSimple::G3Int32")
                            .field(&v)
                            .finish(),
                    }
                }
            }

            impl ::std::clone::Clone for GroupThreeSimple {
                fn clone(&self) -> Self {
                    match self {
                        Self::G3Int32(v) => Self::G3Int32(::std::clone::Clone::clone(&v)),
                    }
                }
            }

            impl ::std::cmp::PartialEq for GroupThreeSimple {
                fn eq(&self, rhs: &Self) -> bool {
                    match (self, rhs) {
                        (Self::G3Int32(left), Self::G3Int32(right)) => left == right,
                        #[allow(unreachable_patterns)]
                        _ => false,
                    }
                }
            }
            pub(crate) enum GroupThreeBumpalo<'bump> {
                _None(::std::marker::PhantomData<&'bump ()>),
                G3Int32(::puroro::internal::Bare<i32>),
            }

            impl<'bump> ::std::default::Default for GroupThreeBumpalo<'bump> {
                fn default() -> Self {
                    Self::_None(::std::marker::PhantomData)
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
