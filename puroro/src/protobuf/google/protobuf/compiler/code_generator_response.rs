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
/** Represents a single generated file.
*/
pub struct File(
    ::std::boxed::Box<
        self::_root::google::protobuf::compiler::code_generator_response::_view::FileView,
    >,
);
impl File {
    pub fn name_mut(&mut self) -> &mut self::_puroro::String {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::compiler::code_generator_response::_view::FileView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.name,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_name(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::compiler::code_generator_response::_view::FileView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.name,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn insertion_point_mut(&mut self) -> &mut self::_puroro::String {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::compiler::code_generator_response::_view::FileView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.insertion_point,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_insertion_point(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::compiler::code_generator_response::_view::FileView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.insertion_point,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn content_mut(&mut self) -> &mut self::_puroro::String {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::compiler::code_generator_response::_view::FileView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.content,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_content(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::compiler::code_generator_response::_view::FileView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.content,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub fn generated_code_info_mut(
        &mut self,
    ) -> &mut self::_root::google::protobuf::GeneratedCodeInfo {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::compiler::code_generator_response::_view::FileView = &mut self
            .0;
        NonRepeatedFieldType::get_field_mut(
            &mut mut_view.fields.generated_code_info,
            mut_view.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn clear_generated_code_info(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        let mut_view: &mut self::_root::google::protobuf::compiler::code_generator_response::_view::FileView = &mut self
            .0;
        NonRepeatedFieldType::clear(
            &mut mut_view.fields.generated_code_info,
            mut_view.shared.bitfield_mut(),
        )
    }
    pub const NAME_FIELD_NUMBER: i32 = 1i32;
    pub const INSERTION_POINT_FIELD_NUMBER: i32 = 2i32;
    pub const CONTENT_FIELD_NUMBER: i32 = 15i32;
    pub const GENERATED_CODE_INFO_FIELD_NUMBER: i32 = 16i32;
}
impl self::_puroro::Message for File {
    type ViewType = self::_root::google::protobuf::compiler::code_generator_response::_view::FileView;
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
impl self::_pinternal::MessageInternal for File {
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
                        let view_ref: &mut self::_root::google::protobuf::compiler::code_generator_response::_view::FileView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.name,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    2i32 => {
                        let view_ref: &mut self::_root::google::protobuf::compiler::code_generator_response::_view::FileView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.insertion_point,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    15i32 => {
                        let view_ref: &mut self::_root::google::protobuf::compiler::code_generator_response::_view::FileView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.content,
                            view_ref.shared.bitfield_mut(),
                            field_data,
                        )?
                    }
                    16i32 => {
                        let view_ref: &mut self::_root::google::protobuf::compiler::code_generator_response::_view::FileView = &mut self
                            .0;
                        self::_pinternal::FieldType::deser_from_field_data(
                            &mut view_ref.fields.generated_code_info,
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
    self::_root::google::protobuf::compiler::code_generator_response::_view::FileView,
> for File {
    fn borrow(
        &self,
    ) -> &self::_root::google::protobuf::compiler::code_generator_response::_view::FileView {
        &self
    }
}
impl ::std::clone::Clone for File {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use ::std::borrow::ToOwned;
        <self::_root::google::protobuf::compiler::code_generator_response::_view::FileView as ToOwned>::to_owned(
            &self,
        )
    }
}
impl ::std::fmt::Debug for File {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        <self::_root::google::protobuf::compiler::code_generator_response::_view::FileView as ::std::fmt::Debug>::fmt(
            &self,
            fmt,
        )
    }
}
impl ::std::default::Default for self::File {
    fn default() -> Self {
        Self(
            <self::_root::google::protobuf::compiler::code_generator_response::_view::FileView as self::_pinternal::MessageViewInternal>::new_boxed(),
        )
    }
}
impl ::std::ops::Deref for File {
    type Target = self::_root::google::protobuf::compiler::code_generator_response::_view::FileView;
    fn deref(&self) -> &Self::Target {
        <::std::boxed::Box<_> as ::std::ops::Deref>::deref(&self.0)
    }
}
impl ::std::cmp::PartialEq for File {
    fn eq(&self, rhs: &Self) -> bool {
        <self::_root::google::protobuf::compiler::code_generator_response::_view::FileView as ::std::cmp::PartialEq>::eq(
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
    pub struct FileView {
        pub(super) fields: self::_root::google::protobuf::compiler::code_generator_response::_fields::FileFields::<
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
            self::_pinternal::OptionalUnsizedField::<
                self::_puroro::String,
                self::_pinternal::tags::String,
                2usize,
            >,
            self::_pinternal::SingularMessageField::<
                self::_root::google::protobuf::GeneratedCodeInfo,
            >,
        >,
        pub(super) shared: self::_pinternal::SharedItemsImpl<1usize>,
    }
    impl FileView {
        pub fn name(&self) -> &str {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.name,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        /** The file name, relative to the output directory.  The name must not
 contain "." or ".." components and must be relative, not be absolute (so,
 the file cannot lie outside the output directory).  "/" must be used as
 the path separator, not "\".

 If the name is omitted, the content will be appended to the previous
 file.  This allows the generator to break large files into small chunks,
 and allows the generated text to be streamed back to protoc so that large
 files need not reside completely in memory at one time.  Note that as of
 this writing protoc does not optimize for this -- it will read the entire
 CodeGeneratorResponse before writing files to disk.
*/
        pub fn name_opt(&self) -> ::std::option::Option::<&str> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.name,
                self.shared.bitfield(),
            )
        }
        pub fn has_name(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.name,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn insertion_point(&self) -> &str {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.insertion_point,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        /** If non-empty, indicates that the named file should already exist, and the
 content here is to be inserted into that file at a defined insertion
 point.  This feature allows a code generator to extend the output
 produced by another code generator.  The original generator may provide
 insertion points by placing special annotations in the file that look
 like:
   @@protoc_insertion_point(NAME)
 The annotation can have arbitrary text before and after it on the line,
 which allows it to be placed in a comment.  NAME should be replaced with
 an identifier naming the point -- this is what other generators will use
 as the insertion_point.  Code inserted at this point will be placed
 immediately above the line containing the insertion point (thus multiple
 insertions to the same point will come out in the order they were added).
 The double-@ is intended to make it unlikely that the generated code
 could contain things that look like insertion points by accident.

 For example, the C++ code generator places the following line in the
 .pb.h files that it generates:
   // @@protoc_insertion_point(namespace_scope)
 This line appears within the scope of the file's package namespace, but
 outside of any particular class.  Another plugin can then specify the
 insertion_point "namespace_scope" to generate additional classes or
 other declarations that should be placed in this scope.

 Note that if the line containing the insertion point begins with
 whitespace, the same whitespace will be added to every line of the
 inserted text.  This is useful for languages like Python, where
 indentation matters.  In these languages, the insertion point comment
 should be indented the same amount as any inserted code will need to be
 in order to work correctly in that context.

 The code generator that generates the initial file and the one which
 inserts into it must both run as part of a single invocation of protoc.
 Code generators are executed in the order in which they appear on the
 command line.

 If |insertion_point| is present, |name| must also be present.
*/
        pub fn insertion_point_opt(&self) -> ::std::option::Option::<&str> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.insertion_point,
                self.shared.bitfield(),
            )
        }
        pub fn has_insertion_point(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.insertion_point,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn content(&self) -> &str {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.content,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        /** The file contents.
*/
        pub fn content_opt(&self) -> ::std::option::Option::<&str> {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.content,
                self.shared.bitfield(),
            )
        }
        pub fn has_content(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.content,
                    self.shared.bitfield(),
                )
                .is_some()
        }
        pub fn generated_code_info(
            &self,
        ) -> ::std::option::Option::<
            &self::_root::google::protobuf::_view::GeneratedCodeInfoView,
        > {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_or_else(
                &self.fields.generated_code_info,
                self.shared.bitfield(),
                ::std::default::Default::default,
            )
        }
        /** Information describing the file content being inserted. If an insertion
 point is used, this information will be appropriately offset and inserted
 into the code generation metadata for the generated files.
*/
        pub fn generated_code_info_opt(
            &self,
        ) -> ::std::option::Option::<
            &self::_root::google::protobuf::_view::GeneratedCodeInfoView,
        > {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                &self.fields.generated_code_info,
                self.shared.bitfield(),
            )
        }
        pub fn has_generated_code_info(&self) -> bool {
            use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
            NonRepeatedFieldType::get_field_opt(
                    &self.fields.generated_code_info,
                    self.shared.bitfield(),
                )
                .is_some()
        }
    }
    impl self::_puroro::MessageView for self::FileView {
        type MessageType = self::_root::google::protobuf::compiler::code_generator_response::File;
        fn to_bytes<W: ::std::io::Write>(
            &self,
            #[allow(unused)]
            out: &mut W,
        ) -> self::_puroro::Result<()> {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.name,
                self.shared.bitfield(),
                1i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.insertion_point,
                self.shared.bitfield(),
                2i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.content,
                self.shared.bitfield(),
                15i32,
                out,
            )?;
            self::_pinternal::FieldType::ser_to_write(
                &self.fields.generated_code_info,
                self.shared.bitfield(),
                16i32,
                out,
            )?;
            self.shared.unknown_fields().ser_to_write(out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl self::_pinternal::MessageViewInternal for self::FileView {
        fn new_boxed() -> ::std::boxed::Box<Self> {
            use self::_pinternal::SharedItems as _;
            let mut shared: self::_pinternal::SharedItemsImpl::<1usize> = ::std::default::Default::default();
            let fields = self::_root::google::protobuf::compiler::code_generator_response::_fields::FileFields {
                name: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                insertion_point: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                content: self::_pinternal::FieldType::new(shared.bitfield_mut()),
                generated_code_info: self::_pinternal::FieldType::new(
                    shared.bitfield_mut(),
                ),
            };
            ::std::boxed::Box::new(Self { fields, shared })
        }
    }
    impl ::std::ops::Drop for FileView {
        fn drop(&mut self) {
            #[allow(unused)]
            use self::_pinternal::{OneofUnion as _, SharedItems as _};
        }
    }
    impl ::std::fmt::Debug for FileView {
        fn fmt(
            &self,
            fmt: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            use self::_pinternal::{SharedItems as _, UnknownFields as _};
            let mut debug_struct = fmt.debug_struct(stringify!(FileView));
            debug_struct
                .field(stringify!(name), &self.name_opt())
                .field(stringify!(insertion_point), &self.insertion_point_opt())
                .field(stringify!(content), &self.content_opt())
                .field(stringify!(generated_code_info), &self.generated_code_info_opt());
            self.shared.unknown_fields().debug_struct_fields(&mut debug_struct)?;
            debug_struct.finish()
        }
    }
    impl ::std::cmp::PartialEq for FileView {
        fn eq(&self, rhs: &Self) -> bool {
            #[allow(unused)]
            use self::_pinternal::OneofUnion as _;
            use self::_pinternal::SharedItems as _;
            true && self.name_opt() == rhs.name_opt()
                && self.insertion_point_opt() == rhs.insertion_point_opt()
                && self.content_opt() == rhs.content_opt()
                && self.generated_code_info_opt() == rhs.generated_code_info_opt()
                && self.shared.unknown_fields() == rhs.shared.unknown_fields()
        }
    }
    impl ::std::borrow::ToOwned for FileView {
        type Owned = self::_root::google::protobuf::compiler::code_generator_response::File;
        fn to_owned(&self) -> Self::Owned {
            #[allow(unused)]
            use self::_pinternal::SharedItems;
            self::_root::google::protobuf::compiler::code_generator_response::File(
                ::std::boxed::Box::new(Self {
                    fields: self::_root::google::protobuf::compiler::code_generator_response::_fields::FileFields {
                        name: ::std::clone::Clone::clone(&self.fields.name),
                        insertion_point: ::std::clone::Clone::clone(
                            &self.fields.insertion_point,
                        ),
                        content: ::std::clone::Clone::clone(&self.fields.content),
                        generated_code_info: ::std::clone::Clone::clone(
                            &self.fields.generated_code_info,
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
    pub struct FileFields<TName, TInsertionPoint, TContent, TGeneratedCodeInfo> {
        pub name: TName,
        pub insertion_point: TInsertionPoint,
        pub content: TContent,
        pub generated_code_info: TGeneratedCodeInfo,
    }
}
#[doc(hidden)]
pub use self::_fields::*;
#[derive(
    ::std::clone::Clone,
    ::std::marker::Copy,
    ::std::cmp::PartialEq,
    ::std::cmp::Eq,
    ::std::cmp::PartialOrd,
    ::std::cmp::Ord,
    ::std::hash::Hash,
    ::std::fmt::Debug,
)]
/** Sync with code_generator.h.
*/
pub enum Feature {
    FeatureNone,
    FeatureProto3Optional,
}
impl ::std::default::Default for Feature {
    fn default() -> Self {
        Self::FeatureNone
    }
}
impl ::std::convert::From::<Feature> for i32 {
    fn from(val: Feature) -> i32 {
        match val {
            Feature::FeatureNone => 0i32,
            Feature::FeatureProto3Optional => 1i32,
        }
    }
}
impl ::std::convert::TryFrom::<i32> for Feature {
    type Error = self::_puroro::PuroroError;
    fn try_from(val: i32) -> ::std::result::Result<Self, Self::Error> {
        match val {
            0i32 => ::std::result::Result::Ok(self::Feature::FeatureNone),
            1i32 => ::std::result::Result::Ok(self::Feature::FeatureProto3Optional),
            _ => {
                ::std::result::Result::Err(
                    self::_puroro::PuroroError::UnknownEnumVariant(val),
                )?
            }
        }
    }
}
