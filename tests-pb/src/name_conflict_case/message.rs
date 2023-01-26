mod _root {
    #[allow(unused)]
    pub(crate) use super::super::_root::*;
}
mod _puroro {
    #[allow(unused)]
    pub(crate) use ::puroro::*;
}
mod _pinternal {
    #[allow(unused)]
    pub(crate) use ::puroro::internal::*;
}
#[derive(::std::default::Default)]
pub struct ConflictCase {
    fields: self::_root::name_conflict_case::message::_fields::ConflictCaseFields<
        self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        >,
    >,
    shared: self::_pinternal::SharedItemsImpl<1usize>,
}
impl ConflictCase {
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
    pub fn this_is_message_field_mut(&mut self) -> &mut i32 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.this_is_message_field,
            self.shared.bitfield_mut(),
            ::std::default::Default::default,
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
    pub fn clear_this_is_message_field(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(
            &mut self.fields.this_is_message_field,
            self.shared.bitfield_mut(),
        )
    }
}
impl self::_puroro::Message for ConflictCase {
    fn from_bytes_iter<I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>>(
        iter: I,
    ) -> self::_puroro::Result<Self> {
        let mut msg = <Self as ::std::default::Default>::default();
        msg.merge_from_bytes_iter(iter)?;
        ::std::result::Result::Ok(msg)
    }
    fn merge_from_bytes_iter<I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>>(
        &mut self,
        mut iter: I,
    ) -> self::_puroro::Result<()> {
        use self::_pinternal::ser::FieldData;
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        #[allow(unused)]
        use ::std::result::Result::{Ok, Err};
        use self::_puroro::PuroroError;
        while let Some((number, mut field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            let result: self::_puroro::Result<()> = (|| {
                match number {
                    1i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.this_is_message_field,
                            self.shared.bitfield_mut(),
                            &mut field_data,
                        )?
                    }
                    _ => Err(PuroroError::UnknownFieldNumber)?,
                }
                Ok(())
            })();
            match result {
                Ok(_) => {}
                Err(
                    PuroroError::UnknownFieldNumber | PuroroError::UnknownEnumVariant(_),
                ) => {
                    self.shared.unknown_fields_mut().push(number, field_data)?;
                }
                Err(e) => Err(e)?,
            }
        }
        Ok(())
    }
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
impl ::std::clone::Clone for ConflictCase {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use self::_pinternal::SharedItems as _;
        Self {
            fields: self::_fields::ConflictCaseFields {
                this_is_message_field: ::std::clone::Clone::clone(
                    &self.fields.this_is_message_field,
                ),
            },
            shared: ::std::clone::Clone::clone(&self.shared),
        }
    }
}
impl ::std::ops::Drop for ConflictCase {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::{OneofUnion as _, SharedItems as _};
    }
}
impl ::std::fmt::Debug for ConflictCase {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        let mut debug_struct = fmt.debug_struct(stringify!(ConflictCase));
        debug_struct
            .field(stringify!(this_is_message_field), &self.this_is_message_field_opt());
        self.shared.unknown_fields().debug_struct_fields(&mut debug_struct)?;
        debug_struct.finish()
    }
}
impl ::std::cmp::PartialEq for ConflictCase {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::SharedItems as _;
        true && self.this_is_message_field_opt() == rhs.this_is_message_field_opt()
            && self.shared.unknown_fields() == rhs.shared.unknown_fields()
    }
}
pub mod _fields {
    mod _root {
        #[allow(unused)]
        pub use super::super::_root::*;
    }
    mod _puroro {
        #[allow(unused)]
        pub use ::puroro::*;
    }
    mod _pinternal {
        #[allow(unused)]
        pub use ::puroro::internal::*;
    }
    #[derive(::std::default::Default)]
    pub struct ConflictCaseFields<TThisIsMessageField> {
        pub this_is_message_field: TThisIsMessageField,
    }
}
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
    fn deser_from_iter<I, B>(
        &mut self,
        bitvec: &mut B,
        field_data: &mut self::_pinternal::ser::FieldData<I>,
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
                unsafe { &mut self.this_is_oneof_field }.deser_from_iter(field_data)?;
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
        pub use super::super::_root::*;
    }
    mod _puroro {
        #[allow(unused)]
        pub use ::puroro::*;
    }
    mod _pinternal {
        #[allow(unused)]
        pub use ::puroro::internal::*;
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
