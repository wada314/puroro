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
pub mod message;
#[cfg(feature = "allocator_api")]
pub struct Message<A: ::std::alloc::Allocator = ::std::alloc::Global>(
    ::std::boxed::Box<self::_root::name_conflict_case::_view::MessageView, A>,
);
#[cfg(not(feature = "allocator_api"))]
pub struct Message(
    ::std::boxed::Box<self::_root::name_conflict_case::_view::MessageView>,
);
impl Message {
    pub fn clear_conflict(&mut self) {
        let view_ref: &mut self::_root::name_conflict_case::_view::MessageView = &mut self
            .0;
        use self::_pinternal::{SharedItems as _, OneofUnion as _};
        view_ref.fields.conflict.clear(view_ref.shared.bitfield_mut())
    }
    pub fn this_is_oneof_field_mut(&mut self) -> &mut i32 {
        let view_ref: &mut self::_root::name_conflict_case::_view::MessageView = &mut self
            .0;
        use self::_pinternal::SharedItems as _;
        view_ref.fields.conflict.this_is_oneof_field_mut(view_ref.shared.bitfield_mut())
    }
    pub fn clear_this_is_oneof_field(&mut self) {
        #[allow(unused)]
        use ::std::option::Option::Some;
        use self::_pinternal::{OneofCase, OneofUnion as _, SharedItems as _};
        let view_ref: &mut self::_root::name_conflict_case::_view::MessageView = &mut self
            .0;
        if let Some(
            self::_root::name_conflict_case::message::_case::ConflictCase::ThisIsOneofField(
                _,
            ),
        ) = OneofCase::from_bitslice(view_ref.shared.bitfield()) {
            view_ref.fields.conflict.clear(view_ref.shared.bitfield_mut())
        }
    }
    pub const THIS_IS_ONEOF_FIELD_FIELD_NUMBER: i32 = 1i32;
}
impl self::_puroro::Message for Message {
    type ViewType = self::_root::name_conflict_case::_view::MessageView;
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
impl self::_pinternal::MessageInternal for Message {
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
                        let view_ref: &mut self::_root::name_conflict_case::_view::MessageView = &mut self
                            .0;
                        view_ref
                            .fields
                            .conflict
                            .deser_from_field_data(
                                view_ref.shared.bitfield_mut(),
                                field_data,
                                self::_root::name_conflict_case::message::_case::ConflictCase::ThisIsOneofField(()),
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
impl ::std::borrow::Borrow<self::_root::name_conflict_case::_view::MessageView>
for Message {
    fn borrow(&self) -> &self::_root::name_conflict_case::_view::MessageView {
        &self
    }
}
impl ::std::clone::Clone for Message {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use ::std::borrow::ToOwned;
        <self::_root::name_conflict_case::_view::MessageView as ToOwned>::to_owned(&self)
    }
}
impl ::std::fmt::Debug for Message {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        <self::_root::name_conflict_case::_view::MessageView as ::std::fmt::Debug>::fmt(
            &self,
            fmt,
        )
    }
}
impl ::std::ops::Deref for Message {
    type Target = self::_root::name_conflict_case::_view::MessageView;
    fn deref(&self) -> &Self::Target {
        <::std::boxed::Box<_> as ::std::ops::Deref>::deref(&self.0)
    }
}
impl ::std::default::Default for self::Message {
    fn default() -> Self {
        Self(
            ::std::boxed::Box::new(
                <self::_root::name_conflict_case::_view::MessageView as ::std::default::Default>::default(),
            ),
        )
    }
}
#[cfg(feature = "allocator_api")]
impl<A> self::_puroro::DefaultIn<A> for self::Message::<A>
where
    A: ::std::alloc::Allocator + ::std::clone::Clone,
    self::_root::name_conflict_case::_view::MessageView: self::_puroro::DefaultIn<A>,
{
    fn default_in(allocator: A) -> Self {
        Self(
            ::std::boxed::Box::new_in(
                <self::_root::name_conflict_case::_view::MessageView as self::_puroro::DefaultIn<
                    A,
                >>::default_in(::std::clone::Clone::clone(&allocator)),
                ::std::clone::Clone::clone(&allocator),
            ),
        )
    }
}
#[cfg(not(feature = "allocator_api"))]
impl ::std::cmp::PartialEq for Message {
    fn eq(&self, rhs: &Self) -> bool {
        <self::_root::name_conflict_case::_view::MessageView as ::std::cmp::PartialEq>::eq(
            &self.0,
            &rhs.0,
        )
    }
}
#[cfg(feature = "allocator_api")]
impl<
    A1: ::std::alloc::Allocator,
    A2: ::std::alloc::Allocator,
> ::std::cmp::PartialEq<self::Message<A2>> for self::Message<A1> {
    fn eq(&self, rhs: &self::Message<A2>) -> bool {
        <self::_root::name_conflict_case::_view::MessageView as ::std::cmp::PartialEq>::eq(
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
    pub struct MessageView {
        pub(super) fields: self::_root::name_conflict_case::_fields::MessageFields::<
            self::_root::name_conflict_case::message::Conflict::<
                self::_pinternal::NumericalField::<i32, self::_pinternal::tags::Int32>,
            >,
        >,
        pub(super) shared: self::_pinternal::SharedItemsImpl<1usize>,
    }
    impl MessageView {
        pub fn conflict(
            &self,
        ) -> ::std::option::Option<
            self::_root::name_conflict_case::message::_case::ConflictCase::<i32>,
        > {
            use self::_pinternal::{SharedItems as _, OneofUnion as _};
            self.fields.conflict.case_ref(self.shared.bitfield())
        }
        pub fn this_is_oneof_field(&self) -> i32 {
            use self::_pinternal::SharedItems as _;
            self.fields.conflict.this_is_oneof_field(self.shared.bitfield())
        }
        pub fn this_is_oneof_field_opt(&self) -> ::std::option::Option::<i32> {
            use self::_pinternal::SharedItems as _;
            self.fields.conflict.this_is_oneof_field_opt(self.shared.bitfield())
        }
        pub fn has_this_is_oneof_field(&self) -> bool {
            self.this_is_oneof_field_opt().is_some()
        }
    }
    impl self::_puroro::MessageView for self::MessageView {
        type MessageType = self::_root::name_conflict_case::Message;
        fn to_bytes<W: ::std::io::Write>(
            &self,
            #[allow(unused)]
            out: &mut W,
        ) -> self::_puroro::Result<()> {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            self.fields.conflict.ser_to_write(self.shared.bitfield(), out)?;
            self.shared.unknown_fields().ser_to_write(out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl self::_pinternal::MessageViewInternal for self::MessageView {
        fn new_boxed() -> ::std::boxed::Box<Self> {
            use self::_pinternal::SharedItems as _;
            let mut shared: self::_pinternal::SharedItemsImpl::<1usize> = ::std::default::Default::default();
            let fields = self::_root::name_conflict_case::_fields::MessageFields {
                conflict: self::_pinternal::OneofUnion::new(shared.bitfield_mut()),
            };
            ::std::boxed::Box::new(Self { fields, shared })
        }
        #[cfg(feature = "allocator_api")]
        fn new_boxed_in<A: ::std::alloc::Allocator>(
            _allocator: A,
        ) -> ::std::boxed::Box<Self, A> {
            todo!()
        }
    }
    impl ::std::ops::Drop for MessageView {
        fn drop(&mut self) {
            #[allow(unused)]
            use self::_pinternal::{OneofUnion as _, SharedItems as _};
            self.fields.conflict.clear(self.shared.bitfield_mut());
        }
    }
    impl ::std::fmt::Debug for MessageView {
        fn fmt(
            &self,
            fmt: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            let mut debug_struct = fmt.debug_struct(stringify!(MessageView));
            debug_struct
                .field(stringify!(this_is_oneof_field), &self.this_is_oneof_field_opt());
            self.shared.unknown_fields().debug_struct_fields(&mut debug_struct)?;
            debug_struct.finish()
        }
    }
    impl ::std::cmp::PartialEq for MessageView {
        fn eq(&self, rhs: &Self) -> bool {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::SharedItems as _;
            true && self.conflict() == rhs.conflict()
                && self.shared.unknown_fields() == rhs.shared.unknown_fields()
        }
    }
    impl ::std::borrow::ToOwned for MessageView {
        type Owned = self::_root::name_conflict_case::Message;
        fn to_owned(&self) -> Self::Owned {
            #[allow(unused)]
            use self::_pinternal::SharedItems;
            self::_root::name_conflict_case::Message(
                ::std::boxed::Box::new(Self {
                    fields: self::_root::name_conflict_case::_fields::MessageFields {
                        conflict: self::_pinternal::OneofUnion::clone(
                            &self.fields.conflict,
                            self.shared.bitfield(),
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
    pub struct MessageFields<TConflict> {
        pub conflict: TConflict,
    }
}
#[doc(hidden)]
pub use self::_fields::*;
