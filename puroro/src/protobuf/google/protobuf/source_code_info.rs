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
pub struct Location(
    ::std::boxed::Box<
        self::_root::google::protobuf::source_code_info::_view::LocationView,
    >,
);
impl Location {
    pub fn path_mut(&mut self) -> &mut ::std::vec::Vec::<i32> {
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
    pub fn span_mut(&mut self) -> &mut ::std::vec::Vec::<i32> {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.0.fields.span,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn clear_span(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(&mut self.0.fields.span, self.0.shared.bitfield_mut())
    }
    pub fn leading_comments_mut(&mut self) -> &mut self::_puroro::String {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::source_code_info::_view::LocationView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.leading_comments,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_leading_comments(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::source_code_info::_view::LocationView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.leading_comments,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn trailing_comments_mut(&mut self) -> &mut self::_puroro::String {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::source_code_info::_view::LocationView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.trailing_comments,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_trailing_comments(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::source_code_info::_view::LocationView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.trailing_comments,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn leading_detached_comments_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec::<self::_puroro::String> {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.0.fields.leading_detached_comments,
            self.0.shared.bitfield_mut(),
        )
    }
    pub fn clear_leading_detached_comments(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(
            &mut self.0.fields.leading_detached_comments,
            self.0.shared.bitfield_mut(),
        )
    }
    pub const PATH_FIELD_NUMBER: i32 = 1i32;
    pub const SPAN_FIELD_NUMBER: i32 = 2i32;
    pub const LEADING_COMMENTS_FIELD_NUMBER: i32 = 3i32;
    pub const TRAILING_COMMENTS_FIELD_NUMBER: i32 = 4i32;
    pub const LEADING_DETACHED_COMMENTS_FIELD_NUMBER: i32 = 6i32;
}
impl self::_puroro::Message for Location {
    type ViewType = self::_root::google::protobuf::source_code_info::_view::LocationView;
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
impl self::_pinternal::MessageInternal for Location {
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
                        let view_ref: &mut self::_root::google::protobuf::source_code_info::_view::LocationView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.path,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    2i32 => {
                        let view_ref: &mut self::_root::google::protobuf::source_code_info::_view::LocationView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.span,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    3i32 => {
                        let view_ref: &mut self::_root::google::protobuf::source_code_info::_view::LocationView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.leading_comments,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    4i32 => {
                        let view_ref: &mut self::_root::google::protobuf::source_code_info::_view::LocationView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.trailing_comments,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    6i32 => {
                        let view_ref: &mut self::_root::google::protobuf::source_code_info::_view::LocationView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.leading_detached_comments,
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
}
impl ::std::borrow::Borrow<
    self::_root::google::protobuf::source_code_info::_view::LocationView,
> for Location {
    fn borrow(
        &self,
    ) -> &self::_root::google::protobuf::source_code_info::_view::LocationView {
        &self
    }
}
impl ::std::clone::Clone for Location {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use ::std::borrow::ToOwned;
        <self::_root::google::protobuf::source_code_info::_view::LocationView as ToOwned>::to_owned(
            &self,
        )
    }
}
impl ::std::fmt::Debug for Location {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        <self::_root::google::protobuf::source_code_info::_view::LocationView as ::std::fmt::Debug>::fmt(
            &self,
            fmt,
        )
    }
}
impl ::std::default::Default for self::Location {
    fn default() -> Self {
        Self(
            <self::_root::google::protobuf::source_code_info::_view::LocationView as self::_pinternal::MessageViewInternal>::new_boxed(),
        )
    }
}
impl ::std::ops::Deref for Location {
    type Target = self::_root::google::protobuf::source_code_info::_view::LocationView;
    fn deref(&self) -> &Self::Target {
        <::std::boxed::Box<_> as ::std::ops::Deref>::deref(&self.0)
    }
}
impl ::std::cmp::PartialEq for Location {
    fn eq(&self, rhs: &Self) -> bool {
        <self::_root::google::protobuf::source_code_info::_view::LocationView as ::std::cmp::PartialEq>::eq(
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
    pub struct LocationView {
        pub(super) fields: self::_root::google::protobuf::source_code_info::_fields::LocationFields::<
            self::_pinternal::RepeatedNumericalField::<
                i32,
                self::_pinternal::tags::Int32,
            >,
            self::_pinternal::RepeatedNumericalField::<
                i32,
                self::_pinternal::tags::Int32,
            >,
            self::_pinternal::OptionalUnsizedField::<
                self::_puroro::String,
                self::_pinternal::tags::String,
                0usize,
            >,
            self::_pinternal::OptionalUnsizedField::<
                self::_puroro::String,
                self::_pinternal::tags::String,
                1usize,
            >,
            self::_pinternal::RepeatedUnsizedField::<
                self::_puroro::String,
                self::_pinternal::tags::String,
            >,
        >,
        pub(super) shared: self::_pinternal::SharedItemsImpl<1usize>,
    }
    impl LocationView {
        /** Identifies which part of the FileDescriptorProto was defined at this
 location.

 Each element is a field number or an index.  They form a path from
 the root FileDescriptorProto to the place where the definition.  For
 example, this path:
   [ 4, 3, 2, 7, 1 ]
 refers to:
   file.message_type(3)  // 4, 3
       .field(7)         // 2, 7
       .name()           // 1
 This is because FileDescriptorProto.message_type has field number 4:
   repeated DescriptorProto message_type = 4;
 and DescriptorProto.field has field number 2:
   repeated FieldDescriptorProto field = 2;
 and FieldDescriptorProto.name has field number 1:
   optional string name = 1;

 Thus, the above path gives the location of a field name.  If we removed
 the last element:
   [ 4, 3, 2, 7 ]
 this path refers to the whole field declaration (from the beginning
 of the label to the terminating semicolon).
*/
        pub fn path(
            &self,
        ) -> impl '_ + self::_puroro::repeated::RepeatedFieldView<'_, Item = i32> {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field(&self.fields.path, self.shared.bitfield())
        }
        /** Always has exactly three or four elements: start line, start column,
 end line (optional, otherwise assumed same as start line), end column.
 These are packed into a single field for efficiency.  Note that line
 and column numbers are zero-based -- typically you will want to add
 1 to each before displaying to a user.
*/
        pub fn span(
            &self,
        ) -> impl '_ + self::_puroro::repeated::RepeatedFieldView<'_, Item = i32> {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field(&self.fields.span, self.shared.bitfield())
        }
        pub fn leading_comments(&self) -> &str {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.leading_comments,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        /** If this SourceCodeInfo represents a complete declaration, these are any
 comments appearing before and after the declaration which appear to be
 attached to the declaration.

 A series of line comments appearing on consecutive lines, with no other
 tokens appearing on those lines, will be treated as a single comment.

 leading_detached_comments will keep paragraphs of comments that appear
 before (but not connected to) the current element. Each paragraph,
 separated by empty lines, will be one comment element in the repeated
 field.

 Only the comment content is provided; comment markers (e.g. //) are
 stripped out.  For block comments, leading whitespace and an asterisk
 will be stripped from the beginning of each line other than the first.
 Newlines are included in the output.

 Examples:

   optional int32 foo = 1;  // Comment attached to foo.
   // Comment attached to bar.
   optional int32 bar = 2;

   optional string baz = 3;
   // Comment attached to baz.
   // Another line attached to baz.

   // Comment attached to qux.
   //
   // Another line attached to qux.
   optional double qux = 4;

   // Detached comment for corge. This is not leading or trailing comments
   // to qux or corge because there are blank lines separating it from
   // both.

   // Detached comment for corge paragraph 2.

   optional string corge = 5;
   /* Block comment attached
    * to corge.  Leading asterisks
    * will be removed. */
   /* Block comment attached to
    * grault. */
   optional int32 grault = 6;

   // ignored detached comments.
*/
        pub fn leading_comments_opt(&self) -> ::std::option::Option::<&str> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.leading_comments,
                self.shared.bitfield(),
            )
        }
        pub fn has_leading_comments(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.leading_comments,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn trailing_comments(&self) -> &str {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.trailing_comments,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        pub fn trailing_comments_opt(&self) -> ::std::option::Option::<&str> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.trailing_comments,
                self.shared.bitfield(),
            )
        }
        pub fn has_trailing_comments(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.trailing_comments,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn leading_detached_comments(
            &self,
        ) -> impl '_ + self::_puroro::repeated::RepeatedFieldView<'_, Item = str> {
            use self::_pinternal::{RepeatedFieldType, SharedItems as _};
            RepeatedFieldType::get_field(
                &self.fields.leading_detached_comments,
                self.shared.bitfield(),
            )
        }
    }
    impl self::_puroro::MessageView for self::LocationView {
        type MessageType = self::_root::google::protobuf::source_code_info::Location;
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
                &self.fields.span,
                self.shared.bitfield(),
                2i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.leading_comments,
                self.shared.bitfield(),
                3i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.trailing_comments,
                self.shared.bitfield(),
                4i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.leading_detached_comments,
                self.shared.bitfield(),
                6i32,
                out,
            )?;
            self.shared.unknown_fields().ser_to_write(out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl self::_pinternal::MessageViewInternal for self::LocationView {
        fn new_boxed() -> ::std::boxed::Box<Self> {
            use self::_pinternal::SharedItems as _;
            let mut shared: self::_pinternal::SharedItemsImpl::<1usize> = ::std::default::Default::default();
            let fields = self::_root::google::protobuf::source_code_info::_fields::LocationFields {
                path: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                span: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                leading_comments: self::_pinternal::FieldType::new(
                    shared.bitfield_mut(),
                ),
                trailing_comments: self::_pinternal::FieldType::new(
                    shared.bitfield_mut(),
                ),
                leading_detached_comments: self::_pinternal::FieldType::new(
                    shared.bitfield_mut(),
                ),
            };
            ::std::boxed::Box::new(Self { fields, shared })
        }
    }
    impl ::std::ops::Drop for LocationView {
        fn drop(&mut self) {
            #[allow(unused)]
            use self::_pinternal::{OneofUnion as _, SharedItems as _};
        }
    }
    impl ::std::fmt::Debug for LocationView {
        fn fmt(
            &self,
            fmt: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            let mut debug_struct = fmt.debug_struct(stringify!(LocationView));
            debug_struct
                .field(
                    stringify!(path),
                    &self.path().into_iter().collect::<::std::vec::Vec<_>>().as_slice(),
                )
                .field(
                    stringify!(span),
                    &self.span().into_iter().collect::<::std::vec::Vec<_>>().as_slice(),
                )
                .field(stringify!(leading_comments), &self.leading_comments_opt())
                .field(stringify!(trailing_comments), &self.trailing_comments_opt())
                .field(
                    stringify!(leading_detached_comments),
                    &self
                        .leading_detached_comments()
                        .into_iter()
                        .collect::<::std::vec::Vec<_>>()
                        .as_slice(),
                );
            self.shared.unknown_fields().debug_struct_fields(&mut debug_struct)?;
            debug_struct.finish()
        }
    }
    impl ::std::cmp::PartialEq for LocationView {
        fn eq(&self, rhs: &Self) -> bool {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::SharedItems as _;
            true && self.path().into_iter().eq(rhs.path())
                && self.span().into_iter().eq(rhs.span())
                && self.leading_comments_opt() == rhs.leading_comments_opt()
                && self.trailing_comments_opt() == rhs.trailing_comments_opt()
                && self
                    .leading_detached_comments()
                    .into_iter()
                    .eq(rhs.leading_detached_comments())
                && self.shared.unknown_fields() == rhs.shared.unknown_fields()
        }
    }
    impl ::std::borrow::ToOwned for LocationView {
        type Owned = self::_root::google::protobuf::source_code_info::Location;
        fn to_owned(&self) -> Self::Owned {
            #[allow(unused)]
            use self::_pinternal::SharedItems;
            self::_root::google::protobuf::source_code_info::Location(
                ::std::boxed::Box::new(Self {
                    fields: self::_root::google::protobuf::source_code_info::_fields::LocationFields {
                        path: ::std::clone::Clone::clone(&self.fields.path),
                        span: ::std::clone::Clone::clone(&self.fields.span),
                        leading_comments: ::std::clone::Clone::clone(
                            &self.fields.leading_comments,
                        ),
                        trailing_comments: ::std::clone::Clone::clone(
                            &self.fields.trailing_comments,
                        ),
                        leading_detached_comments: ::std::clone::Clone::clone(
                            &self.fields.leading_detached_comments,
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
    pub struct LocationFields<
        TPath,
        TSpan,
        TLeadingComments,
        TTrailingComments,
        TLeadingDetachedComments,
    > {
        pub path: TPath,
        pub span: TSpan,
        pub leading_comments: TLeadingComments,
        pub trailing_comments: TTrailingComments,
        pub leading_detached_comments: TLeadingDetachedComments,
    }
}
#[doc(hidden)]
pub use self::_fields::*;
