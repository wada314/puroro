// A generated source code by puroro library
// package self_recursive

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub use _puroro_simple_impl::Msg;
pub mod _puroro_simple_impl {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
    pub struct Msg {
        recursive_unlabeled: ::std::option::Option<
            ::std::boxed::Box<self::_puroro_root::self_recursive::_puroro_simple_impl::Msg>,
        >,
    }
    impl ::puroro::Message<Msg> for Msg {}

    impl Msg {
        pub fn new() -> Self {
            Self {
                recursive_unlabeled: ::std::default::Default::default(),
            }
        }
        pub fn recursive_unlabeled_opt(
            &self,
        ) -> ::std::option::Option<&'_ self::_puroro_root::self_recursive::_puroro_simple_impl::Msg>
        {
            self.recursive_unlabeled.as_deref()
        }

        pub fn has_recursive_unlabeled(&self) -> bool {
            Self::recursive_unlabeled_opt(self).is_some()
        }

        pub fn recursive_unlabeled(
            &self,
        ) -> ::std::option::Option<&'_ self::_puroro_root::self_recursive::_puroro_simple_impl::Msg>
        {
            self.recursive_unlabeled_opt()
        }
        pub fn recursive_unlabeled_mut(
            &mut self,
        ) -> &mut ::std::option::Option<
            ::std::boxed::Box<self::_puroro_root::self_recursive::_puroro_simple_impl::Msg>,
        > {
            &mut self.recursive_unlabeled
        }
    }

    impl super::_puroro_traits::MsgTrait for Msg {
        type Field1MessageType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::self_recursive::_puroro_simple_impl::Msg;
        fn recursive_unlabeled_opt<'this>(&'this self) -> Option<Self::Field1MessageType<'this>> {
            <self::Msg>::recursive_unlabeled_opt(self)
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
                1 => DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled,
                    ::puroro::tags::Message<
                        ::std::boxed::Box<
                            self::_puroro_root::self_recursive::_puroro_simple_impl::Msg,
                        >,
                    >,
                >::deser_field(&mut self.recursive_unlabeled, data),

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
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Message<
                    <Self as super::_puroro_traits::MsgTrait>::Field1MessageType<'_>,
                >,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::recursive_unlabeled_opt(self),
                1,
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
                    "recursive_unlabeled",
                    &<Self as super::_puroro_traits::MsgTrait>::recursive_unlabeled(self),
                )
                .finish()
        }
    }

    impl ::std::clone::Clone for Msg {
        fn clone(&self) -> Self {
            Self {
                recursive_unlabeled: ::std::clone::Clone::clone(&self.recursive_unlabeled),
            }
        }
    }

    impl ::std::cmp::PartialEq for Msg {
        fn eq(&self, rhs: &Self) -> bool {
            self.recursive_unlabeled == rhs.recursive_unlabeled && true
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
        ScalarType: self::_puroro_root::self_recursive::_puroro_traits::MsgTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub recursive_unlabeled: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Msg> for MsgSingleField1<ScalarType> where
        ScalarType: self::_puroro_root::self_recursive::_puroro_traits::MsgTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::MsgTrait for MsgSingleField1<ScalarType>
    where
        ScalarType: self::_puroro_root::self_recursive::_puroro_traits::MsgTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field1MessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;

        fn recursive_unlabeled_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field1MessageType<'this>> {
            ::std::option::Option::Some(&self.recursive_unlabeled)
        }
    }

    impl<ScalarType> ::puroro::internal::se::SerMessageToIoWrite for MsgSingleField1<ScalarType>
    where
        ScalarType: self::_puroro_root::self_recursive::_puroro_traits::MsgTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        Self: super::_puroro_traits::MsgTrait,
        for<'a> <Self as super::_puroro_traits::MsgTrait>::Field1MessageType<'a>:
            ::puroro::internal::se::SerMessageToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Message<
                    <Self as super::_puroro_traits::MsgTrait>::Field1MessageType<'_>,
                >,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::recursive_unlabeled_opt(self),
                1,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MsgSingleField1<ScalarType>
    where
        ScalarType: self::_puroro_root::self_recursive::_puroro_traits::MsgTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                recursive_unlabeled: value,
            }
        }
    }
    pub struct MsgBumpalo<'bump> {
        _bump: &'bump ::puroro::bumpalo::Bump,
        _bitfield:
            ::puroro::bitvec::array::BitArray<::puroro::bitvec::order::Lsb0, [u32; (0 + 31) / 32]>,
        recursive_unlabeled: ::std::option::Option<
            ::puroro::internal::NoAllocBumpBox<
                self::_puroro_root::self_recursive::_puroro_impls::MsgBumpalo<'bump>,
            >,
        >,
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
                recursive_unlabeled: ::std::default::Default::default(),
            }
        }
        pub fn recursive_unlabeled_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<
            &'this self::_puroro_root::self_recursive::_puroro_impls::MsgBumpalo<'this>,
        > {
            self.recursive_unlabeled
                .as_ref()
                .map(|x| unsafe { ::std::mem::transmute(::std::ops::Deref::deref(x)) })
        }
        pub fn recursive_unlabeled<'this>(
            &'this self,
        ) -> ::std::option::Option<
            &'this self::_puroro_root::self_recursive::_puroro_impls::MsgBumpalo<'this>,
        > {
            self.recursive_unlabeled_opt()
        }

        pub fn has_recursive_unlabeled(&self) -> bool {
            self.recursive_unlabeled_opt().is_some()
        }
        pub fn clear_recursive_unlabeled(&mut self) {
            self.recursive_unlabeled = ::std::default::Default::default();
        }
        pub fn recursive_unlabeled_mut<'this>(
            &'this mut self,
        ) -> &'this mut self::_puroro_root::self_recursive::_puroro_impls::MsgBumpalo<'bump>
        {
            if !self.has_recursive_unlabeled() {
                self.recursive_unlabeled = ::std::default::Default::default();
            }
            let bump = self._bump;
            self.recursive_unlabeled.get_or_insert_with(|| {
                ::puroro::internal::NoAllocBumpBox::new_in(
                    ::puroro::internal::BumpDefault::default_in(bump),
                    bump,
                )
            })
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
        type Field1MessageType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::self_recursive::_puroro_impls::MsgBumpalo<'this>;
        fn recursive_unlabeled_opt<'this>(&'this self) -> Option<Self::Field1MessageType<'this>> {
            <Self>::recursive_unlabeled_opt(self)
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
                1 => DeserFieldFromBytesIter::<
                    ::puroro::tags::Unlabeled,
                    ::puroro::tags::Message<
                        ::puroro::internal::NoAllocBumpBox<
                            self::_puroro_root::self_recursive::_puroro_impls::MsgBumpalo<'bump>,
                        >,
                    >,
                >::deser_field(&mut self.recursive_unlabeled, data, self._bump),

                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl<'bump> ::puroro::internal::se::SerMessageToIoWrite for MsgBumpalo<'bump>
    where
        Self: super::_puroro_traits::MsgTrait,
        for<'a> <Self as super::_puroro_traits::MsgTrait>::Field1MessageType<'a>:
            ::puroro::internal::se::SerMessageToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Unlabeled,
                ::puroro::tags::Message<
                    <Self as super::_puroro_traits::MsgTrait>::Field1MessageType<'_>,
                >,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::recursive_unlabeled_opt(self),
                1,
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
        pub fn append_recursive_unlabeled<ScalarType>(
            self,
            value: ScalarType,
        ) -> MsgBuilder<(T, MsgSingleField1<ScalarType>)>
        where
            ScalarType: self::_puroro_root::self_recursive::_puroro_traits::MsgTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            MsgBuilder((
                self.0,
                MsgSingleField1 {
                    recursive_unlabeled: value,
                },
            ))
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
}
pub use _puroro_traits::*;
pub mod _puroro_traits {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }

    pub trait MsgTrait {
        type Field1MessageType<'this>: self::_puroro_root::self_recursive::_puroro_traits::MsgTrait
        where
            Self: 'this;
        fn recursive_unlabeled<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field1MessageType<'this>> {
            self.recursive_unlabeled_opt()
        }
        fn has_recursive_unlabeled<'this>(&'this self) -> bool {
            self.recursive_unlabeled_opt().is_some()
        }
        fn recursive_unlabeled_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field1MessageType<'this>> {
            ::std::option::Option::None
        }
    }

    macro_rules! msg_delegate {
        ($ty:ty) => {
            type Field1MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field1MessageType<'this>;
            fn recursive_unlabeled_opt<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field1MessageType<'this>> {
                (**self).recursive_unlabeled_opt()
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
        type Field1MessageType<'this>
        where
            Self: 'this,
        = ();
    }
    impl<T, U> MsgTrait for (T, U)
    where
        T: MsgTrait,
        U: MsgTrait,
    {
        type Field1MessageType<'this>
        where
            Self: 'this,
        = (
            ::std::option::Option<<T as MsgTrait>::Field1MessageType<'this>>,
            ::std::option::Option<<U as MsgTrait>::Field1MessageType<'this>>,
        );
        fn recursive_unlabeled_opt<'this>(&'this self) -> Option<Self::Field1MessageType<'this>> {
            match (
                <T as MsgTrait>::recursive_unlabeled_opt(&self.0),
                <U as MsgTrait>::recursive_unlabeled_opt(&self.1),
            ) {
                (None, None) => None,
                (Some(t), None) => Some((Some(t), None)),
                (None, Some(u)) => Some((None, Some(u))),
                (Some(t), Some(u)) => Some((Some(t), Some(u))),
            }
        }
    }
    impl<T, U> MsgTrait for ::puroro::Either<T, U>
    where
        T: MsgTrait,
        U: MsgTrait,
    {
        type Field1MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as MsgTrait>::Field1MessageType<'this>,
            <U as MsgTrait>::Field1MessageType<'this>,
        >;
        fn recursive_unlabeled_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field1MessageType<'this>> {
            self.as_ref().either(
                |t| <T as MsgTrait>::recursive_unlabeled_opt(t).map(|t| ::puroro::Either::Left(t)),
                |u| <U as MsgTrait>::recursive_unlabeled_opt(u).map(|u| ::puroro::Either::Right(u)),
            )
        }
    }
    impl<T> MsgTrait for ::std::option::Option<T>
    where
        T: MsgTrait,
    {
        type Field1MessageType<'this>
        where
            Self: 'this,
        = T::Field1MessageType<'this>;
        fn recursive_unlabeled_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field1MessageType<'this>> {
            self.as_ref().and_then(|msg| msg.recursive_unlabeled_opt())
        }
    }
}
pub use _puroro_nested::*;
pub mod _puroro_nested {
    pub mod msg {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
    }
}
