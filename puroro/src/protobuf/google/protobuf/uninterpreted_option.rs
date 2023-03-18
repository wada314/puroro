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
/** The name of the uninterpreted option.  Each string represents a segment in
 a dot-separated name.  is_extension is true iff a segment represents an
 extension (denoted with parentheses in options specs in .proto files).
 E.g.,{ ["foo", false], ["bar.baz", true], ["qux", false] } represents
 "foo.(bar.baz).qux".
*/
pub struct NamePart<A: ::std::alloc::Allocator = ::std::alloc::Global>(
    ::std::boxed::Box<
        self::_root::google::protobuf::uninterpreted_option::_view::NamePartView,
        A,
    >,
);
#[cfg(not(feature = "allocator_api"))]
/** The name of the uninterpreted option.  Each string represents a segment in
 a dot-separated name.  is_extension is true iff a segment represents an
 extension (denoted with parentheses in options specs in .proto files).
 E.g.,{ ["foo", false], ["bar.baz", true], ["qux", false] } represents
 "foo.(bar.baz).qux".
*/
pub struct NamePart(
    ::std::boxed::Box<
        self::_root::google::protobuf::uninterpreted_option::_view::NamePartView,
    >,
);
impl NamePart {
    pub fn name_part_mut(
        &mut self,
    ) -> impl '_ + ::std::ops::Deref<
        Target = ::std::string::String,
    > + ::std::ops::DerefMut {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::uninterpreted_option::_view::NamePartView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.name_part,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_name_part(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::uninterpreted_option::_view::NamePartView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.name_part,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn is_extension_mut(&mut self) -> &mut bool {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::uninterpreted_option::_view::NamePartView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.is_extension,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_is_extension(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::uninterpreted_option::_view::NamePartView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.is_extension,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub const NAME_PART_FIELD_NUMBER: i32 = 1i32;
    pub const IS_EXTENSION_FIELD_NUMBER: i32 = 2i32;
}
impl self::_puroro::Message for NamePart {
    type ViewType = self::_root::google::protobuf::uninterpreted_option::_view::NamePartView;
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
impl self::_pinternal::MessageInternal for NamePart {
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
                        let view_ref: &mut self::_root::google::protobuf::uninterpreted_option::_view::NamePartView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.name_part,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    2i32 => {
                        let view_ref: &mut self::_root::google::protobuf::uninterpreted_option::_view::NamePartView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.is_extension,
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
    self::_root::google::protobuf::uninterpreted_option::_view::NamePartView,
> for NamePart {
    fn borrow(
        &self,
    ) -> &self::_root::google::protobuf::uninterpreted_option::_view::NamePartView {
        &self
    }
}
impl ::std::clone::Clone for NamePart {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use ::std::borrow::ToOwned;
        <self::_root::google::protobuf::uninterpreted_option::_view::NamePartView as ToOwned>::to_owned(
            &self,
        )
    }
}
impl ::std::fmt::Debug for NamePart {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        <self::_root::google::protobuf::uninterpreted_option::_view::NamePartView as ::std::fmt::Debug>::fmt(
            &self,
            fmt,
        )
    }
}
impl ::std::ops::Deref for NamePart {
    type Target = self::_root::google::protobuf::uninterpreted_option::_view::NamePartView;
    fn deref(&self) -> &Self::Target {
        <::std::boxed::Box<_> as ::std::ops::Deref>::deref(&self.0)
    }
}
impl ::std::default::Default for self::NamePart {
    fn default() -> Self {
        Self(
            ::std::boxed::Box::new(
                <self::_root::google::protobuf::uninterpreted_option::_view::NamePartView as ::std::default::Default>::default(),
            ),
        )
    }
}
#[cfg(feature = "allocator_api")]
impl<A> self::_puroro::DefaultIn<A> for self::NamePart::<A>
where
    A: ::std::alloc::Allocator + ::std::clone::Clone,
    self::_root::google::protobuf::uninterpreted_option::_view::NamePartView: self::_puroro::DefaultIn<
        A,
    >,
{
    fn default_in(allocator: A) -> Self {
        Self(
            ::std::boxed::Box::new_in(
                <self::_root::google::protobuf::uninterpreted_option::_view::NamePartView as self::_puroro::DefaultIn<
                    A,
                >>::default_in(::std::clone::Clone::clone(&allocator)),
                ::std::clone::Clone::clone(&allocator),
            ),
        )
    }
}
#[cfg(not(feature = "allocator_api"))]
impl ::std::cmp::PartialEq for NamePart {
    fn eq(&self, rhs: &Self) -> bool {
        <self::_root::google::protobuf::uninterpreted_option::_view::NamePartView as ::std::cmp::PartialEq>::eq(
            &self.0,
            &rhs.0,
        )
    }
}
#[cfg(feature = "allocator_api")]
impl<
    A1: ::std::alloc::Allocator,
    A2: ::std::alloc::Allocator,
> ::std::cmp::PartialEq<self::NamePart<A2>> for self::NamePart<A1> {
    fn eq(&self, rhs: &self::NamePart<A2>) -> bool {
        <self::_root::google::protobuf::uninterpreted_option::_view::NamePartView as ::std::cmp::PartialEq>::eq(
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
    pub struct NamePartView {
        pub(super) fields: self::_root::google::protobuf::uninterpreted_option::_fields::NamePartFields::<
            self::_pinternal::OptionalUnsizedField::<
                ::std::string::String,
                self::_pinternal::tags::String,
                0usize,
            >,
            self::_pinternal::OptionalNumericalField::<
                bool,
                self::_pinternal::tags::Bool,
                1usize,
            >,
        >,
        pub(super) shared: self::_pinternal::SharedItemsImpl<1usize>,
    }
    impl NamePartView {
        pub fn name_part(&self) -> &str {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.name_part,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn name_part_opt(&self) -> ::std::option::Option::<&str> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.name_part,
                self.shared.bitfield(),
            )
        }
        pub fn has_name_part(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.name_part,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn is_extension(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.is_extension,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn is_extension_opt(&self) -> ::std::option::Option::<bool> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.is_extension,
                self.shared.bitfield(),
            )
        }
        pub fn has_is_extension(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.is_extension,
                    self.shared.bitfield(),
                )
                .is_some()
        }
    }
    impl self::_puroro::MessageView for self::NamePartView {
        type MessageType = self::_root::google::protobuf::uninterpreted_option::NamePart;
        fn to_bytes<W: ::std::io::Write>(
            &self,
            #[allow(unused)]
            out: &mut W,
        ) -> self::_puroro::Result<()> {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.name_part,
                self.shared.bitfield(),
                1i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.is_extension,
                self.shared.bitfield(),
                2i32,
                out,
            )?;
            self.shared.unknown_fields().ser_to_write(out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl self::_pinternal::MessageViewInternal for self::NamePartView {
        fn new_boxed() -> ::std::boxed::Box<Self> {
            use self::_pinternal::SharedItems as _;
            let mut shared: self::_pinternal::SharedItemsImpl::<1usize> = ::std::default::Default::default();
            let fields = self::_root::google::protobuf::uninterpreted_option::_fields::NamePartFields {
                name_part: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                is_extension: self::_pinternal::FieldType::new(shared.bitfield_mut()),
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
                let (name_part, _allocator) = self::_pinternal::FieldType::new_in(
                    shared.bitfield_mut(),
                    _allocator,
                );
                let (is_extension, _allocator) = self::_pinternal::FieldType::new_in(
                    shared.bitfield_mut(),
                    _allocator,
                );
                (
                    self::_root::google::protobuf::uninterpreted_option::_fields::NamePartFields {
                        name_part,
                        is_extension,
                    },
                    _allocator,
                )
            };
            ::std::boxed::Box::new_in(Self { fields, shared }, allocator)
        }
    }
    impl ::std::ops::Drop for NamePartView {
        fn drop(&mut self) {
            #[allow(unused)]
            use self::_pinternal::{OneofUnion as _, SharedItems as _};
        }
    }
    impl ::std::fmt::Debug for NamePartView {
        fn fmt(
            &self,
            fmt: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            let mut debug_struct = fmt.debug_struct(stringify!(NamePartView));
            debug_struct
                .field(stringify!(name_part), &self.name_part_opt())
                .field(stringify!(is_extension), &self.is_extension_opt());
            self.shared.unknown_fields().debug_struct_fields(&mut debug_struct)?;
            debug_struct.finish()
        }
    }
    impl ::std::cmp::PartialEq for NamePartView {
        fn eq(&self, rhs: &Self) -> bool {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::SharedItems as _;
            true && self.name_part_opt() == rhs.name_part_opt()
                && self.is_extension_opt() == rhs.is_extension_opt()
                && self.shared.unknown_fields() == rhs.shared.unknown_fields()
        }
    }
    impl ::std::borrow::ToOwned for NamePartView {
        type Owned = self::_root::google::protobuf::uninterpreted_option::NamePart;
        fn to_owned(&self) -> Self::Owned {
            #[allow(unused)]
            use self::_pinternal::SharedItems;
            self::_root::google::protobuf::uninterpreted_option::NamePart(
                ::std::boxed::Box::new(Self {
                    fields: self::_root::google::protobuf::uninterpreted_option::_fields::NamePartFields {
                        name_part: ::std::clone::Clone::clone(&self.fields.name_part),
                        is_extension: ::std::clone::Clone::clone(
                            &self.fields.is_extension,
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
    pub struct NamePartFields<TNamePart, TIsExtension> {
        pub name_part: TNamePart,
        pub is_extension: TIsExtension,
    }
}
#[doc(hidden)]
pub use self::_fields::*;
