// A generated source code by puroro library
// package proto3_packed

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub use _puroro_simple_impl::Msg;
pub mod _puroro_simple_impl {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
    pub struct Msg {
        _bitfield:
            ::puroro::bitvec::array::BitArray<::puroro::bitvec::order::Lsb0, [u32; (0 + 31) / 32]>,
        explicitly_packed: ::std::vec::Vec<i32>,
        explicitly_not_packed: ::std::vec::Vec<i32>,
        not_annotated: ::std::vec::Vec<i32>,
    }
    impl ::puroro::Message<Msg> for Msg {}

    impl Msg {
        pub fn new() -> Self {
            Self {
                _bitfield: ::std::default::Default::default(),
                explicitly_packed: ::std::default::Default::default(),
                explicitly_not_packed: ::std::default::Default::default(),
                not_annotated: ::std::default::Default::default(),
            }
        }
        pub fn explicitly_packed(&self) -> &'_ [i32] {
            &self.explicitly_packed
        }
        pub fn explicitly_not_packed(&self) -> &'_ [i32] {
            &self.explicitly_not_packed
        }
        pub fn not_annotated(&self) -> &'_ [i32] {
            &self.not_annotated
        }
        pub fn explicitly_packed_mut(&mut self) -> &'_ mut ::std::vec::Vec<i32> {
            &mut self.explicitly_packed
        }
        pub fn explicitly_not_packed_mut(&mut self) -> &'_ mut ::std::vec::Vec<i32> {
            &mut self.explicitly_not_packed
        }
        pub fn not_annotated_mut(&mut self) -> &'_ mut ::std::vec::Vec<i32> {
            &mut self.not_annotated
        }
    }

    impl super::_puroro_traits::MsgTrait for Msg {
        type ExplicitlyPackedRepeatedType<'this> =
            ::puroro::CloneThenIntoRepeatedField<'this, ::std::vec::Vec<i32>, i32, i32>;

        fn explicitly_packed<'this>(&'this self) -> Self::ExplicitlyPackedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.explicitly_packed)
        }
        type ExplicitlyNotPackedRepeatedType<'this> =
            ::puroro::CloneThenIntoRepeatedField<'this, ::std::vec::Vec<i32>, i32, i32>;

        fn explicitly_not_packed<'this>(
            &'this self,
        ) -> Self::ExplicitlyNotPackedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.explicitly_not_packed)
        }
        type NotAnnotatedRepeatedType<'this> =
            ::puroro::CloneThenIntoRepeatedField<'this, ::std::vec::Vec<i32>, i32, i32>;

        fn not_annotated<'this>(&'this self) -> Self::NotAnnotatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.not_annotated)
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
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Int32
                >::deser_field(&mut self.explicitly_packed, data)
            }
            2 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Int32
                >::deser_field(&mut self.explicitly_not_packed, data)
            }
            3 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Int32
                >::deser_field(&mut self.not_annotated, data)
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
                ::puroro::tags::Repeated,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::explicitly_packed(self),
                1,
                out,
                true,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::explicitly_not_packed(self),
                2,
                out,
                false,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::not_annotated(self),
                3,
                out,
                true,
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
                .field("explicitly_packed", &self.explicitly_packed())
                .field("explicitly_not_packed", &self.explicitly_not_packed())
                .field("not_annotated", &self.not_annotated())
                .finish()
        }
    }

    impl ::std::clone::Clone for Msg {
        fn clone(&self) -> Self {
            Self {
                _bitfield: ::std::clone::Clone::clone(&self._bitfield),
                explicitly_packed: ::std::clone::Clone::clone(&self.explicitly_packed),
                explicitly_not_packed: ::std::clone::Clone::clone(&self.explicitly_not_packed),
                not_annotated: ::std::clone::Clone::clone(&self.not_annotated),
            }
        }
    }

    impl ::std::cmp::PartialEq for Msg {
        fn eq(&self, rhs: &Self) -> bool {
            self._bitfield == rhs._bitfield
                && self.explicitly_packed == rhs.explicitly_packed
                && self.explicitly_not_packed == rhs.explicitly_not_packed
                && self.not_annotated == rhs.not_annotated
                && true
        }
    }
}

pub use _puroro_impls::*;
pub mod _puroro_impls {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
    use super::_puroro_traits::*;

    pub struct MsgSingleField1<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub explicitly_packed: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::Msg>
        for MsgSingleField1<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::MsgTrait
        for MsgSingleField1<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type ExplicitlyPackedRepeatedType<'this> =
    ::puroro::CloneThenIntoRepeatedField<
        'this,
        RepeatedType,
        ScalarType,
        i32
    > where Self: 'this;

        fn explicitly_packed<'this>(&'this self) -> Self::ExplicitlyPackedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.explicitly_packed)
        }
        type ExplicitlyNotPackedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32> where Self: 'this;
        fn explicitly_not_packed<'this>(
            &'this self,
        ) -> Self::ExplicitlyNotPackedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type NotAnnotatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32> where Self: 'this;
        fn not_annotated<'this>(&'this self) -> Self::NotAnnotatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::se::SerMessageToIoWrite
        for MsgSingleField1<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::explicitly_packed(self),
                1,
                out,
                true,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for MsgSingleField1<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                explicitly_packed: value,
            }
        }
    }

    impl<ScalarType, RepeatedType> ::std::clone::Clone for MsgSingleField1<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        RepeatedType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                explicitly_packed: ::std::clone::Clone::clone(&self.explicitly_packed),
            }
        }
    }

    pub struct MsgSingleField2<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub explicitly_not_packed: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::Msg>
        for MsgSingleField2<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::MsgTrait
        for MsgSingleField2<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type ExplicitlyPackedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32> where Self: 'this;
        fn explicitly_packed<'this>(&'this self) -> Self::ExplicitlyPackedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        type ExplicitlyNotPackedRepeatedType<'this> =
    ::puroro::CloneThenIntoRepeatedField<
        'this,
        RepeatedType,
        ScalarType,
        i32
    > where Self: 'this;

        fn explicitly_not_packed<'this>(
            &'this self,
        ) -> Self::ExplicitlyNotPackedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.explicitly_not_packed)
        }
        type NotAnnotatedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32> where Self: 'this;
        fn not_annotated<'this>(&'this self) -> Self::NotAnnotatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::se::SerMessageToIoWrite
        for MsgSingleField2<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::explicitly_not_packed(self),
                2,
                out,
                false,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for MsgSingleField2<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                explicitly_not_packed: value,
            }
        }
    }

    impl<ScalarType, RepeatedType> ::std::clone::Clone for MsgSingleField2<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        RepeatedType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                explicitly_not_packed: ::std::clone::Clone::clone(&self.explicitly_not_packed),
            }
        }
    }

    pub struct MsgSingleField3<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub not_annotated: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::Msg>
        for MsgSingleField3<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::MsgTrait
        for MsgSingleField3<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type ExplicitlyPackedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32> where Self: 'this;
        fn explicitly_packed<'this>(&'this self) -> Self::ExplicitlyPackedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type ExplicitlyNotPackedRepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32> where Self: 'this;
        fn explicitly_not_packed<'this>(
            &'this self,
        ) -> Self::ExplicitlyNotPackedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        type NotAnnotatedRepeatedType<'this> =
    ::puroro::CloneThenIntoRepeatedField<
        'this,
        RepeatedType,
        ScalarType,
        i32
    > where Self: 'this;

        fn not_annotated<'this>(&'this self) -> Self::NotAnnotatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.not_annotated)
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::se::SerMessageToIoWrite
        for MsgSingleField3<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::not_annotated(self),
                3,
                out,
                true,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for MsgSingleField3<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                not_annotated: value,
            }
        }
    }

    impl<ScalarType, RepeatedType> ::std::clone::Clone for MsgSingleField3<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
        for<'a> &'a RepeatedType:
            ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        RepeatedType: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self {
                not_annotated: ::std::clone::Clone::clone(&self.not_annotated),
            }
        }
    }
    pub struct MsgBumpalo<'bump> {
        _bump: &'bump ::puroro::bumpalo::Bump,
        _bitfield:
            ::puroro::bitvec::array::BitArray<::puroro::bitvec::order::Lsb0, [u32; (0 + 31) / 32]>,
        explicitly_packed: ::puroro::internal::NoAllocBumpVec<i32>,
        explicitly_not_packed: ::puroro::internal::NoAllocBumpVec<i32>,
        not_annotated: ::puroro::internal::NoAllocBumpVec<i32>,
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
                explicitly_packed: ::std::default::Default::default(),
                explicitly_not_packed: ::std::default::Default::default(),
                not_annotated: ::std::default::Default::default(),
            }
        }
        pub fn explicitly_packed<'this>(&'this self) -> &'this [i32] {
            &self.explicitly_packed
        }
        pub fn explicitly_not_packed<'this>(&'this self) -> &'this [i32] {
            &self.explicitly_not_packed
        }
        pub fn not_annotated<'this>(&'this self) -> &'this [i32] {
            &self.not_annotated
        }
        pub fn explicitly_packed_mut<'this>(
            &'this mut self,
        ) -> ::puroro::internal::RefMutBumpVec<'bump, 'this, i32> {
            unsafe { self.explicitly_packed.as_mut_vec_in(self._bump) }
        }
        pub fn explicitly_not_packed_mut<'this>(
            &'this mut self,
        ) -> ::puroro::internal::RefMutBumpVec<'bump, 'this, i32> {
            unsafe { self.explicitly_not_packed.as_mut_vec_in(self._bump) }
        }
        pub fn not_annotated_mut<'this>(
            &'this mut self,
        ) -> ::puroro::internal::RefMutBumpVec<'bump, 'this, i32> {
            unsafe { self.not_annotated.as_mut_vec_in(self._bump) }
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
        type ExplicitlyPackedRepeatedType<'this> =
    ::puroro::CloneThenIntoRepeatedField<
        'this,
        ::puroro::internal::NoAllocBumpVec<i32>,
        i32,
        i32
    > where Self: 'this;

        fn explicitly_packed<'this>(&'this self) -> Self::ExplicitlyPackedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.explicitly_packed)
        }
        type ExplicitlyNotPackedRepeatedType<'this> =
    ::puroro::CloneThenIntoRepeatedField<
        'this,
        ::puroro::internal::NoAllocBumpVec<i32>,
        i32,
        i32
    > where Self: 'this;

        fn explicitly_not_packed<'this>(
            &'this self,
        ) -> Self::ExplicitlyNotPackedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.explicitly_not_packed)
        }
        type NotAnnotatedRepeatedType<'this> =
    ::puroro::CloneThenIntoRepeatedField<
        'this,
        ::puroro::internal::NoAllocBumpVec<i32>,
        i32,
        i32
    > where Self: 'this;

        fn not_annotated<'this>(&'this self) -> Self::NotAnnotatedRepeatedType<'this> {
            ::puroro::CloneThenIntoRepeatedField::new(&self.not_annotated)
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
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Int32
                >::deser_field(&mut self.explicitly_packed, data, self._bump)
            }
            2 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Int32
                >::deser_field(&mut self.explicitly_not_packed, data, self._bump)
            }
            3 => {
                DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Int32
                >::deser_field(&mut self.not_annotated, data, self._bump)
            }

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl<'bump> ::puroro::internal::se::SerMessageToIoWrite for MsgBumpalo<'bump>
    where
        Self: super::_puroro_traits::MsgTrait,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::explicitly_packed(self),
                1,
                out,
                true,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::explicitly_not_packed(self),
                2,
                out,
                false,
            )?;
            ::puroro::internal::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Int32,
            >::ser_field(
                <Self as super::_puroro_traits::MsgTrait>::not_annotated(self),
                3,
                out,
                true,
            )?;
            ::std::result::Result::Ok(())
        }
    }
    pub struct MsgBuilder<T>(T);

    impl<T> MsgBuilder<T>
    where
        T: MsgTrait,
    {
        pub fn append_explicitly_packed<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> MsgBuilder<(T, MsgSingleField1<ScalarType, RepeatedType>)>
        where
            ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
            for<'a> &'a RepeatedType:
                ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            MsgBuilder((
                self.0,
                MsgSingleField1 {
                    explicitly_packed: value,
                },
            ))
        }

        pub fn append_explicitly_not_packed<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> MsgBuilder<(T, MsgSingleField2<ScalarType, RepeatedType>)>
        where
            ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
            for<'a> &'a RepeatedType:
                ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            MsgBuilder((
                self.0,
                MsgSingleField2 {
                    explicitly_not_packed: value,
                },
            ))
        }

        pub fn append_not_annotated<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> MsgBuilder<(T, MsgSingleField3<ScalarType, RepeatedType>)>
        where
            ScalarType: ::std::convert::Into<i32> + ::std::clone::Clone,
            for<'a> &'a RepeatedType:
                ::puroro::RepeatedField<'a> + ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            MsgBuilder((
                self.0,
                MsgSingleField3 {
                    not_annotated: value,
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
        type ExplicitlyPackedRepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = i32>
        where
            Self: 'this;
        fn explicitly_packed<'this>(&'this self) -> Self::ExplicitlyPackedRepeatedType<'this>;

        type ExplicitlyNotPackedRepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = i32>
        where
            Self: 'this;
        fn explicitly_not_packed<'this>(
            &'this self,
        ) -> Self::ExplicitlyNotPackedRepeatedType<'this>;

        type NotAnnotatedRepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = i32>
        where
            Self: 'this;
        fn not_annotated<'this>(&'this self) -> Self::NotAnnotatedRepeatedType<'this>;
    }

    macro_rules! msg_delegate {
        ($ty:ty) => {

            type ExplicitlyPackedRepeatedType<'this> = <$ty>::ExplicitlyPackedRepeatedType<'this> where Self: 'this;
            fn explicitly_packed<'this>(&'this self) -> Self::ExplicitlyPackedRepeatedType<'this> {
                (**self).explicitly_packed()
            }

            type ExplicitlyNotPackedRepeatedType<'this> = <$ty>::ExplicitlyNotPackedRepeatedType<'this> where Self: 'this;
            fn explicitly_not_packed<'this>(&'this self) -> Self::ExplicitlyNotPackedRepeatedType<'this> {
                (**self).explicitly_not_packed()
            }

            type NotAnnotatedRepeatedType<'this> = <$ty>::NotAnnotatedRepeatedType<'this> where Self: 'this;
            fn not_annotated<'this>(&'this self) -> Self::NotAnnotatedRepeatedType<'this> {
                (**self).not_annotated()
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
        type ExplicitlyPackedRepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<i32>
            where Self: 'this;
        fn explicitly_packed<'this>(&'this self) -> Self::ExplicitlyPackedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type ExplicitlyNotPackedRepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<i32>
            where Self: 'this;
        fn explicitly_not_packed<'this>(
            &'this self,
        ) -> Self::ExplicitlyNotPackedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type NotAnnotatedRepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<i32>
            where Self: 'this;
        fn not_annotated<'this>(&'this self) -> Self::NotAnnotatedRepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }
    impl<T, U> MsgTrait for (T, U)
    where
        T: MsgTrait,
        U: MsgTrait,
    {
        type ExplicitlyPackedRepeatedType<'this>
            = ::puroro::internal::impls::merged::MergedRepeatedField<
                <T as MsgTrait>::ExplicitlyPackedRepeatedType<'this>,
                <U as MsgTrait>::ExplicitlyPackedRepeatedType<'this>,
            > where Self: 'this;

        fn explicitly_packed<'this>(&'this self) -> Self::ExplicitlyPackedRepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as MsgTrait>::explicitly_packed(&self.0),
                <U as MsgTrait>::explicitly_packed(&self.1),
            )
        }
        type ExplicitlyNotPackedRepeatedType<'this>
            = ::puroro::internal::impls::merged::MergedRepeatedField<
                <T as MsgTrait>::ExplicitlyNotPackedRepeatedType<'this>,
                <U as MsgTrait>::ExplicitlyNotPackedRepeatedType<'this>,
            > where Self: 'this;

        fn explicitly_not_packed<'this>(
            &'this self,
        ) -> Self::ExplicitlyNotPackedRepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as MsgTrait>::explicitly_not_packed(&self.0),
                <U as MsgTrait>::explicitly_not_packed(&self.1),
            )
        }
        type NotAnnotatedRepeatedType<'this>
            = ::puroro::internal::impls::merged::MergedRepeatedField<
                <T as MsgTrait>::NotAnnotatedRepeatedType<'this>,
                <U as MsgTrait>::NotAnnotatedRepeatedType<'this>,
            > where Self: 'this;

        fn not_annotated<'this>(&'this self) -> Self::NotAnnotatedRepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as MsgTrait>::not_annotated(&self.0),
                <U as MsgTrait>::not_annotated(&self.1),
            )
        }
    }
    impl<T, U> MsgTrait for ::puroro::Either<T, U>
    where
        T: MsgTrait,
        U: MsgTrait,
    {
        type ExplicitlyPackedRepeatedType<'this>
            = ::puroro::internal::impls::either::EitherRepeatedField<
                <T as MsgTrait>::ExplicitlyPackedRepeatedType<'this>,
                <U as MsgTrait>::ExplicitlyPackedRepeatedType<'this>,
            > where Self: 'this;

        fn explicitly_packed<'this>(&'this self) -> Self::ExplicitlyPackedRepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::explicitly_packed(t))
                    .map_right(|u| <U as MsgTrait>::explicitly_packed(u)),
            )
        }
        type ExplicitlyNotPackedRepeatedType<'this>
            = ::puroro::internal::impls::either::EitherRepeatedField<
                <T as MsgTrait>::ExplicitlyNotPackedRepeatedType<'this>,
                <U as MsgTrait>::ExplicitlyNotPackedRepeatedType<'this>,
            > where Self: 'this;

        fn explicitly_not_packed<'this>(
            &'this self,
        ) -> Self::ExplicitlyNotPackedRepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::explicitly_not_packed(t))
                    .map_right(|u| <U as MsgTrait>::explicitly_not_packed(u)),
            )
        }
        type NotAnnotatedRepeatedType<'this>
            = ::puroro::internal::impls::either::EitherRepeatedField<
                <T as MsgTrait>::NotAnnotatedRepeatedType<'this>,
                <U as MsgTrait>::NotAnnotatedRepeatedType<'this>,
            > where Self: 'this;

        fn not_annotated<'this>(&'this self) -> Self::NotAnnotatedRepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as MsgTrait>::not_annotated(t))
                    .map_right(|u| <U as MsgTrait>::not_annotated(u)),
            )
        }
    }
    impl<T> MsgTrait for ::std::option::Option<T>
    where
        T: MsgTrait,
    {
        type ExplicitlyPackedRepeatedType<'this> =
            ::puroro::internal::impls::option::OptionRepeatedField<
                T::ExplicitlyPackedRepeatedType<'this>
            > where Self: 'this;
        fn explicitly_packed<'this>(&'this self) -> Self::ExplicitlyPackedRepeatedType<'this> {
            ::puroro::internal::impls::option::OptionRepeatedField::new(
                self.as_ref().map(|msg| msg.explicitly_packed()),
            )
        }

        type ExplicitlyNotPackedRepeatedType<'this> =
            ::puroro::internal::impls::option::OptionRepeatedField<
                T::ExplicitlyNotPackedRepeatedType<'this>
            > where Self: 'this;
        fn explicitly_not_packed<'this>(
            &'this self,
        ) -> Self::ExplicitlyNotPackedRepeatedType<'this> {
            ::puroro::internal::impls::option::OptionRepeatedField::new(
                self.as_ref().map(|msg| msg.explicitly_not_packed()),
            )
        }

        type NotAnnotatedRepeatedType<'this> =
            ::puroro::internal::impls::option::OptionRepeatedField<
                T::NotAnnotatedRepeatedType<'this>
            > where Self: 'this;
        fn not_annotated<'this>(&'this self) -> Self::NotAnnotatedRepeatedType<'this> {
            ::puroro::internal::impls::option::OptionRepeatedField::new(
                self.as_ref().map(|msg| msg.not_annotated()),
            )
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
