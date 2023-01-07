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
    bitfield: self::_pinternal::BitArray<1usize>,
}
impl Annotation {
    pub fn path(&self) -> &[i32] {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
        > as RepeatedFieldType>::get_field(&self.fields.path, &self.bitfield)
    }
    pub fn path_mut(&mut self) -> &mut ::std::vec::Vec::<i32> {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
        > as RepeatedFieldType>::get_field_mut(&mut self.fields.path, &mut self.bitfield)
    }
    pub fn clear_path(&mut self) {
        use self::_pinternal::RepeatedFieldType;
        <self::_pinternal::RepeatedNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
        > as RepeatedFieldType>::clear(&mut self.fields.path, &mut self.bitfield)
    }
    pub fn source_file(&self) -> &str {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.fields.source_file,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn source_file_opt(&self) -> ::std::option::Option::<&str> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.fields.source_file,
            &self.bitfield,
        )
    }
    pub fn source_file_mut(&mut self) -> &mut ::std::string::String {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.fields.source_file,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_source_file(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(
                &self.fields.source_file,
                &self.bitfield,
            )
            .is_some()
    }
    pub fn clear_source_file(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as NonRepeatedFieldType>::clear(
            &mut self.fields.source_file,
            &mut self.bitfield,
        )
    }
    pub fn begin(&self) -> i32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            1usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.fields.begin,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn begin_opt(&self) -> ::std::option::Option::<i32> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.fields.begin, &self.bitfield)
    }
    pub fn begin_mut(&mut self) -> &mut i32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            1usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.fields.begin,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_begin(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.fields.begin, &self.bitfield)
            .is_some()
    }
    pub fn clear_begin(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            1usize,
        > as NonRepeatedFieldType>::clear(&mut self.fields.begin, &mut self.bitfield)
    }
    pub fn end(&self) -> i32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            2usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.fields.end,
            &self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn end_opt(&self) -> ::std::option::Option::<i32> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            2usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.fields.end, &self.bitfield)
    }
    pub fn end_mut(&mut self) -> &mut i32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            2usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.fields.end,
            &mut self.bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_end(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            2usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.fields.end, &self.bitfield)
            .is_some()
    }
    pub fn clear_end(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            2usize,
        > as NonRepeatedFieldType>::clear(&mut self.fields.end, &mut self.bitfield)
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
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            match number {
                1i32 => {
                    <self::_pinternal::RepeatedNumericalField::<
                        i32,
                        self::_pinternal::tags::Int32,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.fields.path,
                        &mut self.bitfield,
                        field_data,
                    )?
                }
                2i32 => {
                    <self::_pinternal::OptionalUnsizedField::<
                        ::std::string::String,
                        self::_pinternal::tags::String,
                        0usize,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.fields.source_file,
                        &mut self.bitfield,
                        field_data,
                    )?
                }
                3i32 => {
                    <self::_pinternal::OptionalNumericalField::<
                        i32,
                        self::_pinternal::tags::Int32,
                        1usize,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.fields.begin,
                        &mut self.bitfield,
                        field_data,
                    )?
                }
                4i32 => {
                    <self::_pinternal::OptionalNumericalField::<
                        i32,
                        self::_pinternal::tags::Int32,
                        2usize,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.fields.end,
                        &mut self.bitfield,
                        field_data,
                    )?
                }
                _ => todo!(),
            }
        }
        ::std::result::Result::Ok(())
    }
    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)]
        out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        <self::_pinternal::RepeatedNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.fields.path,
            &self.bitfield,
            1i32,
            out,
        )?;
        <self::_pinternal::OptionalUnsizedField::<
            ::std::string::String,
            self::_pinternal::tags::String,
            0usize,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.fields.source_file,
            &self.bitfield,
            2i32,
            out,
        )?;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            1usize,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.fields.begin,
            &self.bitfield,
            3i32,
            out,
        )?;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            2usize,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.fields.end,
            &self.bitfield,
            4i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for Annotation {
    fn clone(&self) -> Self {
        Self {
            fields: self::_fields::AnnotationFields {
                path: <self::_pinternal::RepeatedNumericalField::<
                    i32,
                    self::_pinternal::tags::Int32,
                > as ::std::clone::Clone>::clone(&self.fields.path),
                source_file: <self::_pinternal::OptionalUnsizedField::<
                    ::std::string::String,
                    self::_pinternal::tags::String,
                    0usize,
                > as ::std::clone::Clone>::clone(&self.fields.source_file),
                begin: <self::_pinternal::OptionalNumericalField::<
                    i32,
                    self::_pinternal::tags::Int32,
                    1usize,
                > as ::std::clone::Clone>::clone(&self.fields.begin),
                end: <self::_pinternal::OptionalNumericalField::<
                    i32,
                    self::_pinternal::tags::Int32,
                    2usize,
                > as ::std::clone::Clone>::clone(&self.fields.end),
            },
            bitfield: ::std::clone::Clone::clone(&self.bitfield),
        }
    }
}
impl ::std::ops::Drop for Annotation {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for Annotation {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(Annotation))
            .field(stringify!(path), &self.path())
            .field(stringify!(source_file), &self.source_file_opt())
            .field(stringify!(begin), &self.begin_opt())
            .field(stringify!(end), &self.end_opt())
            .finish()
    }
}
impl ::std::cmp::PartialEq for Annotation {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        true && self.path() == rhs.path()
            && self.source_file_opt() == rhs.source_file_opt()
            && self.begin_opt() == rhs.begin_opt() && self.end_opt() == rhs.end_opt()
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
