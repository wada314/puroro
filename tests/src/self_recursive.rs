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
pub struct Msg<A: ::std::alloc::Allocator = ::std::alloc::Global>(
    ::std::boxed::Box<self::_root::self_recursive::_view::MsgView, A>,
);
#[cfg(not(feature = "allocator_api"))]
pub struct Msg(::std::boxed::Box<self::_root::self_recursive::_view::MsgView>);
impl Msg {
    pub fn recursive_unlabeled_mut(
        &mut self,
    ) -> impl '_ + ::std::ops::Deref<
        Target = self::_root::self_recursive::Msg,
    > + ::std::ops::DerefMut {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::self_recursive::_view::MsgView = &mut self.0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.recursive_unlabeled,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_recursive_unlabeled(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::self_recursive::_view::MsgView = &mut self.0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.recursive_unlabeled,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub const RECURSIVE_UNLABELED_FIELD_NUMBER: i32 = 1i32;
}
impl self::_puroro::Message for Msg {
    type ViewType = self::_root::self_recursive::_view::MsgView;
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
impl self::_pinternal::MessageInternal for Msg {
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
                        let view_ref: &mut self::_root::self_recursive::_view::MsgView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.recursive_unlabeled,
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
impl ::std::borrow::Borrow<self::_root::self_recursive::_view::MsgView> for Msg {
    fn borrow(&self) -> &self::_root::self_recursive::_view::MsgView {
        &self
    }
}
impl ::std::clone::Clone for Msg {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use ::std::borrow::ToOwned;
        <self::_root::self_recursive::_view::MsgView as ToOwned>::to_owned(&self)
    }
}
impl ::std::fmt::Debug for Msg {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        <self::_root::self_recursive::_view::MsgView as ::std::fmt::Debug>::fmt(
            &self,
            fmt,
        )
    }
}
impl ::std::ops::Deref for Msg {
    type Target = self::_root::self_recursive::_view::MsgView;
    fn deref(&self) -> &Self::Target {
        <::std::boxed::Box<_> as ::std::ops::Deref>::deref(&self.0)
    }
}
impl ::std::default::Default for self::Msg {
    fn default() -> Self {
        Self(
            ::std::boxed::Box::new(
                <self::_root::self_recursive::_view::MsgView as ::std::default::Default>::default(),
            ),
        )
    }
}
#[cfg(feature = "allocator_api")]
impl<A> self::_puroro::DefaultIn<A> for self::Msg::<A>
where
    A: ::std::alloc::Allocator + ::std::clone::Clone,
    self::_root::self_recursive::_view::MsgView: self::_puroro::DefaultIn<A>,
{
    fn default_in(allocator: A) -> Self {
        Self(
            ::std::boxed::Box::new_in(
                <self::_root::self_recursive::_view::MsgView as self::_puroro::DefaultIn<
                    A,
                >>::default_in(::std::clone::Clone::clone(&allocator)),
                ::std::clone::Clone::clone(&allocator),
            ),
        )
    }
}
#[cfg(not(feature = "allocator_api"))]
impl ::std::cmp::PartialEq for Msg {
    fn eq(&self, rhs: &Self) -> bool {
        <self::_root::self_recursive::_view::MsgView as ::std::cmp::PartialEq>::eq(
            &self.0,
            &rhs.0,
        )
    }
}
#[cfg(feature = "allocator_api")]
impl<
    A1: ::std::alloc::Allocator,
    A2: ::std::alloc::Allocator,
> ::std::cmp::PartialEq<self::Msg<A2>> for self::Msg<A1> {
    fn eq(&self, rhs: &self::Msg<A2>) -> bool {
        <self::_root::self_recursive::_view::MsgView as ::std::cmp::PartialEq>::eq(
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
    pub struct MsgView {
        pub(super) fields: self::_root::self_recursive::_fields::MsgFields::<
            self::_pinternal::SingularMessageField::<
                self::_root::self_recursive::_view::MsgView,
            >,
        >,
        pub(super) shared: self::_pinternal::SharedItemsImpl<0usize>,
    }
    impl MsgView {
        pub fn recursive_unlabeled(
            &self,
        ) -> ::std::option::Option::<&self::_root::self_recursive::_view::MsgView> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.recursive_unlabeled,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn recursive_unlabeled_opt(
            &self,
        ) -> ::std::option::Option::<&self::_root::self_recursive::_view::MsgView> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.recursive_unlabeled,
                self.shared.bitfield(),
            )
        }
        pub fn has_recursive_unlabeled(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.recursive_unlabeled,
                    self.shared.bitfield(),
                )
                .is_some()
        }
    }
    impl self::_puroro::MessageView for self::MsgView {
        type MessageType = self::_root::self_recursive::Msg;
        fn to_bytes<W: ::std::io::Write>(
            &self,
            #[allow(unused)]
            out: &mut W,
        ) -> self::_puroro::Result<()> {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.recursive_unlabeled,
                self.shared.bitfield(),
                1i32,
                out,
            )?;
            self.shared.unknown_fields().ser_to_write(out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl self::_pinternal::MessageViewInternal for self::MsgView {
        fn new_boxed() -> ::std::boxed::Box<Self> {
            use self::_pinternal::SharedItems as _;
            let mut shared: self::_pinternal::SharedItemsImpl::<0usize> = ::std::default::Default::default();
            let fields = self::_root::self_recursive::_fields::MsgFields {
                recursive_unlabeled: self::_pinternal::FieldType::new(
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
            let mut shared: self::_pinternal::SharedItemsImpl::<0usize> = ::std::default::Default::default();
            let (fields, allocator) = {
                let (recursive_unlabeled, _allocator) = self::_pinternal::FieldType::new_in(
                    shared.bitfield_mut(),
                    _allocator,
                );
                (
                    self::_root::self_recursive::_fields::MsgFields {
                        recursive_unlabeled,
                    },
                    _allocator,
                )
            };
            ::std::boxed::Box::new_in(Self { fields, shared }, allocator)
        }
    }
    impl ::std::ops::Drop for MsgView {
        fn drop(&mut self) {
            #[allow(unused)]
            use self::_pinternal::{OneofUnion as _, SharedItems as _};
        }
    }
    impl ::std::fmt::Debug for MsgView {
        fn fmt(
            &self,
            fmt: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            let mut debug_struct = fmt.debug_struct(stringify!(MsgView));
            debug_struct
                .field(stringify!(recursive_unlabeled), &self.recursive_unlabeled_opt());
            self.shared.unknown_fields().debug_struct_fields(&mut debug_struct)?;
            debug_struct.finish()
        }
    }
    impl ::std::cmp::PartialEq for MsgView {
        fn eq(&self, rhs: &Self) -> bool {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::SharedItems as _;
            true && self.recursive_unlabeled_opt() == rhs.recursive_unlabeled_opt()
                && self.shared.unknown_fields() == rhs.shared.unknown_fields()
        }
    }
    impl ::std::borrow::ToOwned for MsgView {
        type Owned = self::_root::self_recursive::Msg;
        fn to_owned(&self) -> Self::Owned {
            #[allow(unused)]
            use self::_pinternal::SharedItems;
            self::_root::self_recursive::Msg(
                ::std::boxed::Box::new(Self {
                    fields: self::_root::self_recursive::_fields::MsgFields {
                        recursive_unlabeled: ::std::clone::Clone::clone(
                            &self.fields.recursive_unlabeled,
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
    pub struct MsgFields<TRecursiveUnlabeled> {
        pub recursive_unlabeled: TRecursiveUnlabeled,
    }
}
#[doc(hidden)]
pub use self::_fields::*;
