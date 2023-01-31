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
/** Represents a single generated file.
*/
pub struct File {
    fields: self::_root::google::protobuf::compiler::code_generator_response::_fields::FileFields<
        self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        >,
        self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            1usize,
        >,
        self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            2usize,
        >,
        self::_pinternal::SingularHeapMessageField::<
            self::_root::google::protobuf::GeneratedCodeInfo,
        >,
    >,
    shared: self::_pinternal::SharedItemsImpl<1usize>,
}
impl File {
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
        NonRepeatedFieldType::get_field_opt(&self.fields.name, self.shared.bitfield())
    }
    pub fn name_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.name,
            self.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn has_name(&self) -> bool {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_opt(&self.fields.name, self.shared.bitfield())
            .is_some()
    }
    pub fn clear_name(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(&mut self.fields.name, self.shared.bitfield_mut())
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
    pub fn insertion_point_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.insertion_point,
            self.shared.bitfield_mut(),
            ::std::default::Default::default,
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
    pub fn clear_insertion_point(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(
            &mut self.fields.insertion_point,
            self.shared.bitfield_mut(),
        )
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
        NonRepeatedFieldType::get_field_opt(&self.fields.content, self.shared.bitfield())
    }
    pub fn content_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.content,
            self.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn has_content(&self) -> bool {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_opt(&self.fields.content, self.shared.bitfield())
            .is_some()
    }
    pub fn clear_content(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(&mut self.fields.content, self.shared.bitfield_mut())
    }
    pub fn generated_code_info(
        &self,
    ) -> ::std::option::Option::<&self::_root::google::protobuf::GeneratedCodeInfo> {
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
    ) -> ::std::option::Option::<&self::_root::google::protobuf::GeneratedCodeInfo> {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_opt(
            &self.fields.generated_code_info,
            self.shared.bitfield(),
        )
    }
    pub fn generated_code_info_mut(
        &mut self,
    ) -> &mut self::_root::google::protobuf::GeneratedCodeInfo {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.generated_code_info,
            self.shared.bitfield_mut(),
            ::std::default::Default::default,
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
    pub fn clear_generated_code_info(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(
            &mut self.fields.generated_code_info,
            self.shared.bitfield_mut(),
        )
    }
}
impl self::_puroro::Message for File {
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
                            &mut self.fields.name,
                            self.shared.bitfield_mut(),
                            &mut field_data,
                        )?
                    }
                    2i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.insertion_point,
                            self.shared.bitfield_mut(),
                            &mut field_data,
                        )?
                    }
                    15i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.content,
                            self.shared.bitfield_mut(),
                            &mut field_data,
                        )?
                    }
                    16i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.generated_code_info,
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
impl ::std::clone::Clone for File {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use self::_pinternal::SharedItems as _;
        Self {
            fields: self::_fields::FileFields {
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
        }
    }
}
impl ::std::ops::Drop for File {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::{OneofUnion as _, SharedItems as _};
    }
}
impl ::std::fmt::Debug for File {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        let mut debug_struct = fmt.debug_struct(stringify!(File));
        debug_struct
            .field(stringify!(name), &self.name_opt())
            .field(stringify!(insertion_point), &self.insertion_point_opt())
            .field(stringify!(content), &self.content_opt())
            .field(stringify!(generated_code_info), &self.generated_code_info_opt());
        self.shared.unknown_fields().debug_struct_fields(&mut debug_struct)?;
        debug_struct.finish()
    }
}
impl ::std::cmp::PartialEq for File {
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
    pub struct FileFields<TName, TInsertionPoint, TContent, TGeneratedCodeInfo> {
        pub name: TName,
        pub insertion_point: TInsertionPoint,
        pub content: TContent,
        pub generated_code_info: TGeneratedCodeInfo,
    }
}
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
