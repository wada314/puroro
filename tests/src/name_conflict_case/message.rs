mod _root {
    #[allow(unused)]
    pub(crate) use super::super::_root::*;
}
mod _puroro {
    #[allow(unused)]
    pub(crate) use super::_root::_puroro::*;
}
mod _pinternal {
    #[allow(unused)]
    pub(crate) use super::_root::_pinternal::*;
}
#[cfg(feature = "allocator_api")]
pub struct ConflictCase<A: ::std::alloc::Allocator = ::std::alloc::Global>(
    ::std::boxed::Box<
        self::_root::name_conflict_case::message::_view::ConflictCaseView,
        A,
    >,
);
#[cfg(not(feature = "allocator_api"))]
pub struct ConflictCase(
    ::std::boxed::Box<self::_root::name_conflict_case::message::_view::ConflictCaseView>,
);
impl ConflictCase {
    pub fn this_is_message_field_mut(&mut self) -> &mut i32 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::name_conflict_case::message::_view::ConflictCaseView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.this_is_message_field,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_this_is_message_field(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::name_conflict_case::message::_view::ConflictCaseView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.this_is_message_field,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub const THIS_IS_MESSAGE_FIELD_FIELD_NUMBER: i32 = 1i32;
}
impl self::_puroro::Message for ConflictCase {
    type ViewType = self::_root::name_conflict_case::message::_view::ConflictCaseView;
    fn from_bytes_iter<I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>>(
        iter: I,
    ) -> self::_puroro::Result<Self> {
        let mut msg = <Self as ::std::default::Default>::default();
        msg.merge_from_bytes_iter(iter)?;
        ::std::result::Result::Ok(msg)
    }
    fn merge_from_bytes_iter<I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>>(
        &mut self,
        iter: I,
    ) -> self::_puroro::Result<()> {
        let mut pos_iter = self::_pinternal::PosIter::new(iter);
        let mut scoped_iter = self::_pinternal::ScopedIter::from_mut_pos_iter(
            &mut pos_iter,
        );
        <Self as self::_pinternal::MessageInternal>::merge_from_scoped_bytes_iter(
            self,
            &mut scoped_iter,
        )?;
        scoped_iter.drop_and_check_scope_completed()?;
        Ok(())
    }
}
impl self::_pinternal::MessageInternal for ConflictCase {
    fn merge_from_scoped_bytes_iter<
        'a,
        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
    >(
        &mut self,
        iter: &mut self::_pinternal::ScopedIter<'a, I>,
    ) -> self::_puroro::Result<()> {
        use self::_pinternal::ser::FieldData;
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        #[allow(unused)]
        use ::std::result::Result;
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        #[allow(unused)]
        use ::std::vec::Vec;
        use self::_puroro::PuroroError;
        while let Some((number, field_data))
            = FieldData::from_bytes_scoped_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    1i32 => {
                        let view_ref: &mut self::_root::name_conflict_case::message::_view::ConflictCaseView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.this_is_message_field,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    _ => {
                        let field_data = field_data
                            .map(|iter| { iter.collect::<Result<Vec<_>, _>>() })
                            .transpose()?;
                        Err(PuroroError::UnknownFieldNumber(field_data))?
                    }
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(PuroroError::UnknownFieldNumber(field_data)) => {
                    self.0.shared.unknown_fields_mut().push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
    fn from_boxed_view(
        v: ::std::boxed::Box<<Self as self::_puroro::Message>::ViewType>,
    ) -> Self {
        Self(v)
    }
    fn into_boxed_view(
        self,
    ) -> ::std::boxed::Box<<Self as self::_puroro::Message>::ViewType> {
        self.0
    }
}
impl ::std::borrow::Borrow<
    self::_root::name_conflict_case::message::_view::ConflictCaseView,
> for ConflictCase {
    fn borrow(
        &self,
    ) -> &self::_root::name_conflict_case::message::_view::ConflictCaseView {
        &self
    }
}
impl ::std::clone::Clone for ConflictCase {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use ::std::borrow::ToOwned;
        <self::_root::name_conflict_case::message::_view::ConflictCaseView as ToOwned>::to_owned(
            &self,
        )
    }
}
impl ::std::fmt::Debug for ConflictCase {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        <self::_root::name_conflict_case::message::_view::ConflictCaseView as ::std::fmt::Debug>::fmt(
            &self,
            fmt,
        )
    }
}
impl ::std::ops::Deref for ConflictCase {
    type Target = self::_root::name_conflict_case::message::_view::ConflictCaseView;
    fn deref(&self) -> &Self::Target {
        <::std::boxed::Box<_> as ::std::ops::Deref>::deref(&self.0)
    }
}
impl ::std::default::Default for self::ConflictCase {
    fn default() -> Self {
        Self(
            ::std::boxed::Box::new(
                <self::_root::name_conflict_case::message::_view::ConflictCaseView as ::std::default::Default>::default(),
            ),
        )
    }
}
#[cfg(feature = "allocator_api")]
impl<A> self::_puroro::DefaultIn<A> for self::ConflictCase::<A>
where
    A: ::std::alloc::Allocator + ::std::clone::Clone,
    self::_root::name_conflict_case::message::_view::ConflictCaseView: self::_puroro::DefaultIn<
        A,
    >,
{
    fn default_in(allocator: A) -> Self {
        Self(
            ::std::boxed::Box::new_in(
                <self::_root::name_conflict_case::message::_view::ConflictCaseView as self::_puroro::DefaultIn<
                    A,
                >>::default_in(::std::clone::Clone::clone(&allocator)),
                ::std::clone::Clone::clone(&allocator),
            ),
        )
    }
}
#[cfg(not(feature = "allocator_api"))]
impl ::std::cmp::PartialEq for ConflictCase {
    fn eq(&self, rhs: &Self) -> bool {
        <self::_root::name_conflict_case::message::_view::ConflictCaseView as ::std::cmp::PartialEq>::eq(
            &self.0,
            &rhs.0,
        )
    }
}
#[cfg(feature = "allocator_api")]
impl<
    A1: ::std::alloc::Allocator,
    A2: ::std::alloc::Allocator,
> ::std::cmp::PartialEq<self::ConflictCase<A2>> for self::ConflictCase<A1> {
    fn eq(&self, rhs: &self::ConflictCase<A2>) -> bool {
        <self::_root::name_conflict_case::message::_view::ConflictCaseView as ::std::cmp::PartialEq>::eq(
            &self.0,
            &rhs.0,
        )
    }
}
#[doc(hidden)]
pub mod _view {
    mod _root {
        #[allow(unused)]
        pub(crate) use super::super::_root::*;
    }
    mod _puroro {
        #[allow(unused)]
        pub(crate) use super::_root::_puroro::*;
    }
    mod _pinternal {
        #[allow(unused)]
        pub(crate) use super::_root::_pinternal::*;
    }
    #[derive(::std::default::Default)]
    pub struct ConflictCaseView {
        pub(super) fields: self::_root::name_conflict_case::message::_fields::ConflictCaseFields::<
            self::_pinternal::OptionalNumericalField::<
                i32,
                self::_pinternal::tags::Int32,
                0usize,
            >,
        >,
        pub(super) shared: self::_pinternal::SharedItemsImpl<1usize>,
    }
    impl ConflictCaseView {
        pub fn this_is_message_field(&self) -> i32 {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.this_is_message_field,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn this_is_message_field_opt(&self) -> ::std::option::Option::<i32> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.this_is_message_field,
                self.shared.bitfield(),
            )
        }
        pub fn has_this_is_message_field(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.this_is_message_field,
                    self.shared.bitfield(),
                )
                .is_some()
        }
    }
    impl self::_puroro::MessageView for self::ConflictCaseView {
        type MessageType = self::_root::name_conflict_case::message::ConflictCase;
        fn to_bytes<W: ::std::io::Write>(
            &self,
            #[allow(unused)]
            out: &mut W,
        ) -> self::_puroro::Result<()> {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.this_is_message_field,
                self.shared.bitfield(),
                1i32,
                out,
            )?;
            self.shared.unknown_fields().ser_to_write(out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl self::_pinternal::MessageViewInternal for self::ConflictCaseView {
        fn new_boxed() -> ::std::boxed::Box<Self> {
            use self::_pinternal::SharedItems as _;
            let mut shared: self::_pinternal::SharedItemsImpl::<1usize> = ::std::default::Default::default();
            let fields = self::_root::name_conflict_case::message::_fields::ConflictCaseFields {
                this_is_message_field: self::_pinternal::FieldType::new(
                    shared.bitfield_mut(),
                ),
            };
            ::std::boxed::Box::new(Self { fields, shared })
        }
        #[cfg(feature = "allocator_api")]
        fn new_boxed_in<A: ::std::alloc::Allocator>(
            _allocator: A,
        ) -> ::std::boxed::Box<Self, A> {
            use self::_pinternal::SharedItems as _;
            let mut shared: self::_pinternal::SharedItemsImpl::<1usize> = ::std::default::Default::default();
            let (fields, allocator) = {
                let (this_is_message_field, _allocator) = self::_pinternal::FieldType::new_in(
                    shared.bitfield_mut(),
                    _allocator,
                );
                (
                    self::_root::name_conflict_case::message::_fields::ConflictCaseFields {
                        this_is_message_field,
                    },
                    _allocator,
                )
            };
            ::std::boxed::Box::new_in(Self { fields, shared }, allocator)
        }
    }
    impl ::std::ops::Drop for ConflictCaseView {
        fn drop(&mut self) {
            #[allow(unused)]
            use self::_pinternal::{OneofUnion as _, SharedItems as _};
        }
    }
    impl ::std::fmt::Debug for ConflictCaseView {
        fn fmt(
            &self,
            fmt: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            let mut debug_struct = fmt.debug_struct(stringify!(ConflictCaseView));
            debug_struct
                .field(
                    stringify!(this_is_message_field),
                    &self.this_is_message_field_opt(),
                );
            self.shared.unknown_fields().debug_struct_fields(&mut debug_struct)?;
            debug_struct.finish()
        }
    }
    impl ::std::cmp::PartialEq for ConflictCaseView {
        fn eq(&self, rhs: &Self) -> bool {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::SharedItems as _;
            true && self.this_is_message_field_opt() == rhs.this_is_message_field_opt()
                && self.shared.unknown_fields() == rhs.shared.unknown_fields()
        }
    }
    impl ::std::borrow::ToOwned for ConflictCaseView {
        type Owned = self::_root::name_conflict_case::message::ConflictCase;
        fn to_owned(&self) -> Self::Owned {
            #[allow(unused)]
            use self::_pinternal::SharedItems;
            self::_root::name_conflict_case::message::ConflictCase(
                ::std::boxed::Box::new(Self {
                    fields: self::_root::name_conflict_case::message::_fields::ConflictCaseFields {
                        this_is_message_field: ::std::clone::Clone::clone(
                            &self.fields.this_is_message_field,
                        ),
                    },
                    shared: ::std::clone::Clone::clone(&self.shared),
                }),
            )
        }
    }
}
#[doc(inline)]
pub use self::_view::*;
#[doc(hidden)]
pub mod _fields {
    mod _root {
        #[allow(unused)]
        pub(crate) use super::super::_root::*;
    }
    mod _puroro {
        #[allow(unused)]
        pub(crate) use super::_root::_puroro::*;
    }
    mod _pinternal {
        #[allow(unused)]
        pub(crate) use super::_root::_pinternal::*;
    }
    #[derive(::std::default::Default)]
    pub struct ConflictCaseFields<TThisIsMessageField> {
        pub this_is_message_field: TThisIsMessageField,
    }
}
#[doc(hidden)]
pub use self::_fields::*;
pub union Conflict<TThisIsOneofField> {
    _none: (),
    this_is_oneof_field: ::std::mem::ManuallyDrop::<TThisIsOneofField>,
}
impl<TThisIsOneofField> Conflict<TThisIsOneofField>
where
    Self: self::_pinternal::OneofUnion,
    TThisIsOneofField: self::_pinternal::OneofFieldType,
{
    pub(crate) fn this_is_oneof_field<B: self::_pinternal::BitSlice>(
        &self,
        bits: &B,
    ) -> <TThisIsOneofField as self::_pinternal::OneofFieldType>::GetterOrElseType<'_> {
        #[allow(unused)]
        use ::std::option::Option::{None, Some};
        #[allow(unused)]
        use ::std::default::Default;
        use self::_pinternal::OneofFieldType;
        use ::std::ops::Deref as _;
        use self::_pinternal::OneofCase;
        let case_opt = OneofCase::from_bitslice(bits);
        let field_opt = matches!(
            case_opt,
            Some(self::_root::name_conflict_case::message::_case::ConflictCase:: < >
            ::ThisIsOneofField(()))
        )
            .then(|| { unsafe { self.this_is_oneof_field.deref() } });
        OneofFieldType::get_field_or_else(field_opt, ::std::default::Default::default)
    }
    pub(crate) fn this_is_oneof_field_opt<B: self::_pinternal::BitSlice>(
        &self,
        bits: &B,
    ) -> <TThisIsOneofField as self::_pinternal::OneofFieldType>::GetterOptType<'_> {
        #[allow(unused)]
        use ::std::option::Option::{None, Some};
        use self::_pinternal::OneofFieldType;
        use ::std::ops::Deref as _;
        use self::_pinternal::OneofCase;
        let case_opt = OneofCase::from_bitslice(bits);
        let field_opt = matches!(
            case_opt,
            Some(self::_root::name_conflict_case::message::_case::ConflictCase:: < >
            ::ThisIsOneofField(()))
        )
            .then(|| { unsafe { self.this_is_oneof_field.deref() } });
        OneofFieldType::get_field_opt(field_opt)
    }
    pub(crate) fn this_is_oneof_field_mut<B: self::_pinternal::BitSlice>(
        &mut self,
        bits: &mut B,
    ) -> <TThisIsOneofField as self::_pinternal::OneofFieldType>::GetterMutType<'_> {
        #[allow(unused)]
        use ::std::option::Option::Some;
        #[allow(unused)]
        use ::std::default::Default;
        use ::std::mem::ManuallyDrop;
        use ::std::ops::DerefMut as _;
        use self::_pinternal::{OneofCase, OneofUnion};
        use self::_pinternal::OneofFieldType;
        let case_opt = OneofCase::from_bitslice(bits);
        if let Some(
            self::_root::name_conflict_case::message::_case::ConflictCase::ThisIsOneofField(
                (),
            ),
        ) = case_opt {} else {
            <Self as OneofUnion>::clear(self, bits);
            let index = OneofCase::into_u32(
                self::_root::name_conflict_case::message::_case::ConflictCase::ThisIsOneofField(()),
            );
            bits.set_range(0usize..2usize, index);
            *self = Self {
                this_is_oneof_field: ManuallyDrop::new(
                    (::std::default::Default::default)(),
                ),
            };
        }
        let field_mut = unsafe { self.this_is_oneof_field.deref_mut() };
        OneofFieldType::get_field_mut(field_mut)
    }
}
impl<TThisIsOneofField> self::_pinternal::OneofUnion for Conflict<TThisIsOneofField>
where
    TThisIsOneofField: self::_pinternal::OneofFieldType,
{
    fn new<B: self::_pinternal::BitSlice>(bits: &mut B) -> Self {
        bits.set_range(0usize..2usize, 0);
        Self { _none: () }
    }
    #[cfg(feature = "allocator_api")]
    fn new_in<B: self::_pinternal::BitSlice, A: ::std::alloc::Allocator>(
        bits: &mut B,
        allocator: A,
    ) -> (Self, A) {
        bits.set_range(0usize..2usize, 0);
        (Self { _none: () }, allocator)
    }
    type Case = self::_root::name_conflict_case::message::_case::ConflictCase;
    type CaseRef<'a> = self::_root::name_conflict_case::message::_case::ConflictCase::<
        <TThisIsOneofField as self::_pinternal::OneofFieldType>::GetterType::<'a>,
    >
    where
        Self: 'a;
    fn case_ref<B: self::_pinternal::BitSlice>(
        &self,
        bits: &B,
    ) -> ::std::option::Option<Self::CaseRef<'_>> {
        use self::_pinternal::OneofCase;
        use ::std::mem::ManuallyDrop;
        use ::std::ops::Deref as _;
        let case_opt = OneofCase::from_bitslice(bits);
        case_opt
            .map(|case| {
                match case {
                    Self::Case::ThisIsOneofField(_) => {
                        Self::CaseRef::ThisIsOneofField(
                            ManuallyDrop::deref(unsafe { &self.this_is_oneof_field })
                                .get_field(),
                        )
                    }
                }
            })
    }
    fn clear<B: self::_pinternal::BitSlice>(&mut self, bits: &mut B) {
        use self::_pinternal::OneofCase;
        use ::std::mem::ManuallyDrop;
        #[allow(unused)]
        use ::std::option::Option::Some;
        match OneofCase::from_bitslice(bits) {
            Some(Self::Case::ThisIsOneofField(())) => {
                unsafe { ManuallyDrop::take(&mut self.this_is_oneof_field) };
            }
            _ => {}
        }
        bits.set_range(0usize..2usize, 0);
    }
    fn clone<B: self::_pinternal::BitSlice>(&self, bits: &B) -> Self {
        use self::_pinternal::OneofCase;
        #[allow(unused)]
        use ::std::option::Option::Some;
        #[allow(unused)]
        use ::std::clone::Clone;
        match OneofCase::from_bitslice(bits) {
            Some(Self::Case::ThisIsOneofField(())) => {
                Self {
                    this_is_oneof_field: Clone::clone(unsafe {
                        &self.this_is_oneof_field
                    }),
                }
            }
            _ => Self { _none: () },
        }
    }
    fn deser_from_field_data<'a, I, B>(
        &mut self,
        bitvec: &mut B,
        field_data: self::_pinternal::ser::FieldData<
            self::_pinternal::ScopedIter<'a, I>,
        >,
        case: Self::Case,
    ) -> self::_puroro::Result<()>
    where
        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        B: self::_pinternal::BitSlice,
    {
        #[allow(unused)]
        use ::std::result::Result::Ok;
        match case {
            Self::Case::ThisIsOneofField(_) => {
                let _ = <Self>::this_is_oneof_field_mut(self, bitvec);
                unsafe { &mut self.this_is_oneof_field }
                    .deser_from_field_data(field_data)?;
            }
        }
        Ok(())
    }
    fn ser_to_write<W, B>(&self, bitvec: &B, out: &mut W) -> self::_puroro::Result<()>
    where
        W: ::std::io::Write,
        B: self::_pinternal::BitSlice,
    {
        #[allow(unused)]
        use ::std::option::Option::Some;
        #[allow(unused)]
        use ::std::result::Result::Ok;
        use self::_pinternal::OneofCase;
        match OneofCase::from_bitslice(bitvec) {
            Some(Self::Case::ThisIsOneofField(_)) => {
                unsafe { &self.this_is_oneof_field }.ser_to_write(1i32, out)?;
            }
            _ => {}
        }
        Ok(())
    }
}
impl<TThisIsOneofField> ::std::default::Default for Conflict<TThisIsOneofField> {
    fn default() -> Self {
        Self { _none: () }
    }
}
pub mod _case {
    mod _root {
        #[allow(unused)]
        pub(crate) use super::super::_root::*;
    }
    mod _puroro {
        #[allow(unused)]
        pub(crate) use super::_root::_puroro::*;
    }
    mod _pinternal {
        #[allow(unused)]
        pub(crate) use super::_root::_pinternal::*;
    }
    #[derive(::std::fmt::Debug, ::std::cmp::PartialEq)]
    pub enum ConflictCase<TThisIsOneofField = ()> {
        ThisIsOneofField(TThisIsOneofField),
    }
    impl self::_pinternal::OneofCase for ConflictCase {
        const BITFIELD_BEGIN: usize = 0usize;
        const BITFIELD_END: usize = 2usize;
        fn from_u32(x: u32) -> ::std::option::Option<Self> {
            match x {
                1u32 => ::std::option::Option::Some(Self::ThisIsOneofField(())),
                _ => ::std::option::Option::None,
            }
        }
        fn into_u32(self) -> u32 {
            match self {
                Self::ThisIsOneofField(_) => 1u32,
            }
        }
    }
}
pub use self::_case::*;
