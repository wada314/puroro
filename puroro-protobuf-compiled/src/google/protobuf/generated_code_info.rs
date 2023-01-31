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
pub struct Annotation {
    fields: self::_root::google::protobuf::generated_code_info::_fields::AnnotationFields<
        self::_pinternal::RepeatedNumericalField::<i32, self::_pinternal::tags::Int32>,
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
    shared: self::_pinternal::SharedItemsImpl<1usize>,
}
impl Annotation {
    /** Identifies the element in the original source .proto file. This field
 is formatted the same as SourceCodeInfo.Location.path.
*/
    pub fn path(&self) -> &[i32] {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field(&self.fields.path, self.shared.bitfield())
    }
    pub fn path_mut(&mut self) -> &mut ::std::vec::Vec::<i32> {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::get_field_mut(
            &mut self.fields.path,
            self.shared.bitfield_mut(),
        )
    }
    pub fn clear_path(&mut self) {
        use self::_pinternal::{RepeatedFieldType, SharedItems as _};
        RepeatedFieldType::clear(&mut self.fields.path, self.shared.bitfield_mut())
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
    pub fn source_file_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.source_file,
            self.shared.bitfield_mut(),
            ::std::default::Default::default,
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
    pub fn clear_source_file(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(
            &mut self.fields.source_file,
            self.shared.bitfield_mut(),
        )
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
        NonRepeatedFieldType::get_field_opt(&self.fields.begin, self.shared.bitfield())
    }
    pub fn begin_mut(&mut self) -> &mut i32 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.begin,
            self.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn has_begin(&self) -> bool {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_opt(&self.fields.begin, self.shared.bitfield())
            .is_some()
    }
    pub fn clear_begin(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(&mut self.fields.begin, self.shared.bitfield_mut())
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
    pub fn end_mut(&mut self) -> &mut i32 {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_mut(
            &mut self.fields.end,
            self.shared.bitfield_mut(),
            ::std::default::Default::default,
        )
    }
    pub fn has_end(&self) -> bool {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::get_field_opt(&self.fields.end, self.shared.bitfield())
            .is_some()
    }
    pub fn clear_end(&mut self) {
        use self::_pinternal::{NonRepeatedFieldType, SharedItems as _};
        NonRepeatedFieldType::clear(&mut self.fields.end, self.shared.bitfield_mut())
    }
}
impl self::_puroro::Message for Annotation {
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
                            &mut self.fields.path,
                            self.shared.bitfield_mut(),
                            &mut field_data,
                        )?
                    }
                    2i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.source_file,
                            self.shared.bitfield_mut(),
                            &mut field_data,
                        )?
                    }
                    3i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.begin,
                            self.shared.bitfield_mut(),
                            &mut field_data,
                        )?
                    }
                    4i32 => {
                        self::_pinternal::FieldType::deser_from_iter(
                            &mut self.fields.end,
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
impl ::std::clone::Clone for Annotation {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use self::_pinternal::SharedItems as _;
        Self {
            fields: self::_fields::AnnotationFields {
                path: ::std::clone::Clone::clone(&self.fields.path),
                source_file: ::std::clone::Clone::clone(&self.fields.source_file),
                begin: ::std::clone::Clone::clone(&self.fields.begin),
                end: ::std::clone::Clone::clone(&self.fields.end),
            },
            shared: ::std::clone::Clone::clone(&self.shared),
        }
    }
}
impl ::std::ops::Drop for Annotation {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::{OneofUnion as _, SharedItems as _};
    }
}
impl ::std::fmt::Debug for Annotation {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        use self::_pinternal::{SharedItems as _, UnknownFields as _};
        let mut debug_struct = fmt.debug_struct(stringify!(Annotation));
        debug_struct
            .field(stringify!(path), &self.path())
            .field(stringify!(source_file), &self.source_file_opt())
            .field(stringify!(begin), &self.begin_opt())
            .field(stringify!(end), &self.end_opt());
        self.shared.unknown_fields().debug_struct_fields(&mut debug_struct)?;
        debug_struct.finish()
    }
}
impl ::std::cmp::PartialEq for Annotation {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        use self::_pinternal::SharedItems as _;
        true && self.path() == rhs.path()
            && self.source_file_opt() == rhs.source_file_opt()
            && self.begin_opt() == rhs.begin_opt() && self.end_opt() == rhs.end_opt()
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
    pub struct AnnotationFields<TPath, TSourceFile, TBegin, TEnd> {
        pub path: TPath,
        pub source_file: TSourceFile,
        pub begin: TBegin,
        pub end: TEnd,
    }
}
pub use self::_fields::*;
