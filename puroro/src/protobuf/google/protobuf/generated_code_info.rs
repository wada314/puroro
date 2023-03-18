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
pub struct Annotation<A: ::std::alloc::Allocator = ::std::alloc::Global>(
    ::std::boxed::Box<
        self::_root::google::protobuf::generated_code_info::_view::AnnotationView,
        A,
    >,
);
#[cfg(not(feature = "allocator_api"))]
pub struct Annotation(
    ::std::boxed::Box<
        self::_root::google::protobuf::generated_code_info::_view::AnnotationView,
    >,
);
impl Annotation {
    pub fn path_mut(
        &mut self,
    ) -> impl '_ + ::std::ops::Deref<
        Target = ::std::vec::Vec::<i32>,
    > + ::std::ops::DerefMut {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.0.fields.path,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn clear_path(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(&mut self.0.fields.path, self.0.shared.bitfield_mut())
    }
    pub fn source_file_mut(
        &mut self,
    ) -> impl '_ + ::std::ops::Deref<
        Target = ::std::string::String,
    > + ::std::ops::DerefMut {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::generated_code_info::_view::AnnotationView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.source_file,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_source_file(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::generated_code_info::_view::AnnotationView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.source_file,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn begin_mut(&mut self) -> &mut i32 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::generated_code_info::_view::AnnotationView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.begin,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_begin(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::generated_code_info::_view::AnnotationView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.begin,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn end_mut(&mut self) -> &mut i32 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::generated_code_info::_view::AnnotationView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.end,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_end(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::generated_code_info::_view::AnnotationView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.end,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub const PATH_FIELD_NUMBER: i32 = 1i32;
    pub const SOURCE_FILE_FIELD_NUMBER: i32 = 2i32;
    pub const BEGIN_FIELD_NUMBER: i32 = 3i32;
    pub const END_FIELD_NUMBER: i32 = 4i32;
}
impl self::_puroro::Message for Annotation {
    type ViewType = self::_root::google::protobuf::generated_code_info::_view::AnnotationView;
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
impl self::_pinternal::MessageInternal for Annotation {
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
                        let view_ref: &mut self::_root::google::protobuf::generated_code_info::_view::AnnotationView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.path,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    2i32 => {
                        let view_ref: &mut self::_root::google::protobuf::generated_code_info::_view::AnnotationView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.source_file,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    3i32 => {
                        let view_ref: &mut self::_root::google::protobuf::generated_code_info::_view::AnnotationView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.begin,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    4i32 => {
                        let view_ref: &mut self::_root::google::protobuf::generated_code_info::_view::AnnotationView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.end,
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
    self::_root::google::protobuf::generated_code_info::_view::AnnotationView,
> for Annotation {
    fn borrow(
        &self,
    ) -> &self::_root::google::protobuf::generated_code_info::_view::AnnotationView {
        &self
    }
}
impl ::std::clone::Clone for Annotation {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use ::std::borrow::ToOwned;
        <self::_root::google::protobuf::generated_code_info::_view::AnnotationView as ToOwned>::to_owned(
            &self,
        )
    }
}
impl ::std::fmt::Debug for Annotation {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        <self::_root::google::protobuf::generated_code_info::_view::AnnotationView as ::std::fmt::Debug>::fmt(
            &self,
            fmt,
        )
    }
}
impl ::std::ops::Deref for Annotation {
    type Target = self::_root::google::protobuf::generated_code_info::_view::AnnotationView;
    fn deref(&self) -> &Self::Target {
        <::std::boxed::Box<_> as ::std::ops::Deref>::deref(&self.0)
    }
}
impl ::std::default::Default for self::Annotation {
    fn default() -> Self {
        Self(
            ::std::boxed::Box::new(
                <self::_root::google::protobuf::generated_code_info::_view::AnnotationView as ::std::default::Default>::default(),
            ),
        )
    }
}
#[cfg(feature = "allocator_api")]
impl<A> self::_puroro::DefaultIn<A> for self::Annotation::<A>
where
    A: ::std::alloc::Allocator + ::std::clone::Clone,
    self::_root::google::protobuf::generated_code_info::_view::AnnotationView: self::_puroro::DefaultIn<
        A,
    >,
{
    fn default_in(allocator: A) -> Self {
        Self(
            ::std::boxed::Box::new_in(
                <self::_root::google::protobuf::generated_code_info::_view::AnnotationView as self::_puroro::DefaultIn<
                    A,
                >>::default_in(::std::clone::Clone::clone(&allocator)),
                ::std::clone::Clone::clone(&allocator),
            ),
        )
    }
}
#[cfg(not(feature = "allocator_api"))]
impl ::std::cmp::PartialEq for Annotation {
    fn eq(&self, rhs: &Self) -> bool {
        <self::_root::google::protobuf::generated_code_info::_view::AnnotationView as ::std::cmp::PartialEq>::eq(
            &self.0,
            &rhs.0,
        )
    }
}
#[cfg(feature = "allocator_api")]
impl<
    A1: ::std::alloc::Allocator,
    A2: ::std::alloc::Allocator,
> ::std::cmp::PartialEq<self::Annotation<A2>> for self::Annotation<A1> {
    fn eq(&self, rhs: &self::Annotation<A2>) -> bool {
        <self::_root::google::protobuf::generated_code_info::_view::AnnotationView as ::std::cmp::PartialEq>::eq(
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
    pub struct AnnotationView {
        pub(super) fields: self::_root::google::protobuf::generated_code_info::_fields::AnnotationFields::<
            self::_pinternal::RepeatedNumericalField::<
                i32,
                self::_pinternal::tags::Int32,
            >,
            self::_pinternal::OptionalUnsizedField::<
                ::std::string::String,
                self::_pinternal::tags::String,
                0usize,
            >,
            self::_pinternal::OptionalNumericalField::<
                i32,
                self::_pinternal::tags::Int32,
                1usize,
            >,
            self::_pinternal::OptionalNumericalField::<
                i32,
                self::_pinternal::tags::Int32,
                2usize,
            >,
        >,
        pub(super) shared: self::_pinternal::SharedItemsImpl<1usize>,
    }
    impl AnnotationView {
        /** Identifies the element in the original source .proto file. This field
 is formatted the same as SourceCodeInfo.Location.path.
*/
        pub fn path(
            &self,
        ) -> impl '_ + self::_puroro::repeated::RepeatedFieldView<'_, Item = i32> {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field(&self.fields.path, self.shared.bitfield())
        }
        pub fn source_file(&self) -> &str {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.source_file,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        /** Identifies the filesystem path to the original source .proto.
*/
        pub fn source_file_opt(&self) -> ::std::option::Option::<&str> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.source_file,
                self.shared.bitfield(),
            )
        }
        pub fn has_source_file(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.source_file,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn begin(&self) -> i32 {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.begin,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        /** Identifies the starting offset in bytes in the generated code
 that relates to the identified object.
*/
        pub fn begin_opt(&self) -> ::std::option::Option::<i32> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.begin,
                self.shared.bitfield(),
            )
        }
        pub fn has_begin(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.begin,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn end(&self) -> i32 {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.end,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        /** Identifies the ending offset in bytes in the generated code that
 relates to the identified offset. The end offset should be one past
 the last relevant byte (so the length of the text = end - begin).
*/
        pub fn end_opt(&self) -> ::std::option::Option::<i32> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(&self.fields.end, self.shared.bitfield())
        }
        pub fn has_end(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(&self.fields.end, self.shared.bitfield())
                .is_some()
        }
    }
    impl self::_puroro::MessageView for self::AnnotationView {
        type MessageType = self::_root::google::protobuf::generated_code_info::Annotation;
        fn to_bytes<W: ::std::io::Write>(
            &self,
            #[allow(unused)]
            out: &mut W,
        ) -> self::_puroro::Result<()> {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.path,
                self.shared.bitfield(),
                1i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.source_file,
                self.shared.bitfield(),
                2i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.begin,
                self.shared.bitfield(),
                3i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.end,
                self.shared.bitfield(),
                4i32,
                out,
            )?;
            self.shared.unknown_fields().ser_to_write(out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl self::_pinternal::MessageViewInternal for self::AnnotationView {
        fn new_boxed() -> ::std::boxed::Box<Self> {
            use self::_pinternal::SharedItems as _;
            let mut shared: self::_pinternal::SharedItemsImpl::<1usize> = ::std::default::Default::default();
            let fields = self::_root::google::protobuf::generated_code_info::_fields::AnnotationFields {
                path: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                source_file: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                begin: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                end: self::_pinternal::FieldType::new(shared.bitfield_mut()),
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
                let (path, _allocator) = self::_pinternal::FieldType::new_in(
                    shared.bitfield_mut(),
                    _allocator,
                );
                let (source_file, _allocator) = self::_pinternal::FieldType::new_in(
                    shared.bitfield_mut(),
                    _allocator,
                );
                let (begin, _allocator) = self::_pinternal::FieldType::new_in(
                    shared.bitfield_mut(),
                    _allocator,
                );
                let (end, _allocator) = self::_pinternal::FieldType::new_in(
                    shared.bitfield_mut(),
                    _allocator,
                );
                (
                    self::_root::google::protobuf::generated_code_info::_fields::AnnotationFields {
                        path,
                        source_file,
                        begin,
                        end,
                    },
                    _allocator,
                )
            };
            ::std::boxed::Box::new_in(Self { fields, shared }, allocator)
        }
    }
    impl ::std::ops::Drop for AnnotationView {
        fn drop(&mut self) {
            #[allow(unused)]
            use self::_pinternal::{OneofUnion as _, SharedItems as _};
        }
    }
    impl ::std::fmt::Debug for AnnotationView {
        fn fmt(
            &self,
            fmt: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            let mut debug_struct = fmt.debug_struct(stringify!(AnnotationView));
            debug_struct
                .field(
                    stringify!(path),
                    &self.path().into_iter().collect::<::std::vec::Vec<_>>().as_slice(),
                )
                .field(stringify!(source_file), &self.source_file_opt())
                .field(stringify!(begin), &self.begin_opt())
                .field(stringify!(end), &self.end_opt());
            self.shared.unknown_fields().debug_struct_fields(&mut debug_struct)?;
            debug_struct.finish()
        }
    }
    impl ::std::cmp::PartialEq for AnnotationView {
        fn eq(&self, rhs: &Self) -> bool {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::SharedItems as _;
            true && self.path().into_iter().eq(rhs.path())
                && self.source_file_opt() == rhs.source_file_opt()
                && self.begin_opt() == rhs.begin_opt() && self.end_opt() == rhs.end_opt()
                && self.shared.unknown_fields() == rhs.shared.unknown_fields()
        }
    }
    impl ::std::borrow::ToOwned for AnnotationView {
        type Owned = self::_root::google::protobuf::generated_code_info::Annotation;
        fn to_owned(&self) -> Self::Owned {
            #[allow(unused)]
            use self::_pinternal::SharedItems;
            self::_root::google::protobuf::generated_code_info::Annotation(
                ::std::boxed::Box::new(Self {
                    fields: self::_root::google::protobuf::generated_code_info::_fields::AnnotationFields {
                        path: ::std::clone::Clone::clone(&self.fields.path),
                        source_file: ::std::clone::Clone::clone(
                            &self.fields.source_file,
                        ),
                        begin: ::std::clone::Clone::clone(&self.fields.begin),
                        end: ::std::clone::Clone::clone(&self.fields.end),
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
    pub struct AnnotationFields<TPath, TSourceFile, TBegin, TEnd> {
        pub path: TPath,
        pub source_file: TSourceFile,
        pub begin: TBegin,
        pub end: TEnd,
    }
}
#[doc(hidden)]
pub use self::_fields::*;
