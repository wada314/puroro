pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}
pub mod _puroro {
    pub use ::puroro::*;
}
#[derive(::std::default::Default)]
pub struct Annotation {
    path: self::_puroro::internal::field_type::RepeatedNumericalField::<
        i32,
        self::_puroro::tags::Int32,
    >,
    source_file: self::_puroro::internal::field_type::OptionalStringField::<0usize>,
    begin: self::_puroro::internal::field_type::OptionalNumericalField::<
        i32,
        self::_puroro::tags::Int32,
        1usize,
    >,
    end: self::_puroro::internal::field_type::OptionalNumericalField::<
        i32,
        self::_puroro::tags::Int32,
        2usize,
    >,
    _bitfield: self::_puroro::bitvec::BitArray<1usize>,
}
impl Annotation {
    pub fn path(&self) -> &[i32] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            i32,
            self::_puroro::tags::Int32,
        > as RepeatedFieldType>::get_field(&self.path, &self._bitfield)
    }
    pub fn path_mut(&mut self) -> &mut ::std::vec::Vec::<i32> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            i32,
            self::_puroro::tags::Int32,
        > as RepeatedFieldType>::mut_field(&mut self.path, &mut self._bitfield)
    }
    pub fn clear_path(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            i32,
            self::_puroro::tags::Int32,
        > as RepeatedFieldType>::clear(&mut self.path, &mut self._bitfield)
    }
    pub fn source_file(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            0usize,
        > as NonRepeatedFieldType>::get_field(
            &self.source_file,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn source_file_opt(&self) -> ::std::option::Option::<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.source_file, &self._bitfield)
    }
    pub fn source_file_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            0usize,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.source_file,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_source_file(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.source_file, &self._bitfield)
            .is_some()
    }
    pub fn clear_source_file(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<
            0usize,
        > as NonRepeatedFieldType>::clear(&mut self.source_file, &mut self._bitfield)
    }
    pub fn begin(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            1usize,
        > as NonRepeatedFieldType>::get_field(
            &self.begin,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn begin_opt(&self) -> ::std::option::Option::<i32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.begin, &self._bitfield)
    }
    pub fn begin_mut(&mut self) -> &mut i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            1usize,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.begin,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_begin(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.begin, &self._bitfield)
            .is_some()
    }
    pub fn clear_begin(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            1usize,
        > as NonRepeatedFieldType>::clear(&mut self.begin, &mut self._bitfield)
    }
    pub fn end(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            2usize,
        > as NonRepeatedFieldType>::get_field(
            &self.end,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn end_opt(&self) -> ::std::option::Option::<i32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            2usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.end, &self._bitfield)
    }
    pub fn end_mut(&mut self) -> &mut i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            2usize,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.end,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_end(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            2usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.end, &self._bitfield)
            .is_some()
    }
    pub fn clear_end(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            2usize,
        > as NonRepeatedFieldType>::clear(&mut self.end, &mut self._bitfield)
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
        use self::_puroro::internal::ser::FieldData;
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion as _;
        while let Some((number, field_data))
            = FieldData::from_bytes_iter(iter.by_ref())? {
            match number {
                1i32 => {
                    <self::_puroro::internal::field_type::RepeatedNumericalField::<
                        i32,
                        self::_puroro::tags::Int32,
                    > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.path,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                2i32 => {
                    <self::_puroro::internal::field_type::OptionalStringField::<
                        0usize,
                    > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.source_file,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                3i32 => {
                    <self::_puroro::internal::field_type::OptionalNumericalField::<
                        i32,
                        self::_puroro::tags::Int32,
                        1usize,
                    > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.begin,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                4i32 => {
                    <self::_puroro::internal::field_type::OptionalNumericalField::<
                        i32,
                        self::_puroro::tags::Int32,
                        2usize,
                    > as self::_puroro::internal::field_type::FieldType>::deser_from_iter(
                        &mut self.end,
                        &mut self._bitfield,
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
        use self::_puroro::internal::oneof_type::OneofUnion as _;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            i32,
            self::_puroro::tags::Int32,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.path,
            &self._bitfield,
            1i32,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalStringField::<
            0usize,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.source_file,
            &self._bitfield,
            2i32,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            1usize,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.begin,
            &self._bitfield,
            3i32,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            i32,
            self::_puroro::tags::Int32,
            2usize,
        > as self::_puroro::internal::field_type::FieldType>::ser_to_write(
            &self.end,
            &self._bitfield,
            4i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for Annotation {
    fn clone(&self) -> Self {
        Self {
            path: <self::_puroro::internal::field_type::RepeatedNumericalField::<
                i32,
                self::_puroro::tags::Int32,
            > as ::std::clone::Clone>::clone(&self.path),
            source_file: <self::_puroro::internal::field_type::OptionalStringField::<
                0usize,
            > as ::std::clone::Clone>::clone(&self.source_file),
            begin: <self::_puroro::internal::field_type::OptionalNumericalField::<
                i32,
                self::_puroro::tags::Int32,
                1usize,
            > as ::std::clone::Clone>::clone(&self.begin),
            end: <self::_puroro::internal::field_type::OptionalNumericalField::<
                i32,
                self::_puroro::tags::Int32,
                2usize,
            > as ::std::clone::Clone>::clone(&self.end),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
impl ::std::ops::Drop for Annotation {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion as _;
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
        use self::_puroro::internal::oneof_type::OneofUnion as _;
        true && self.path() == rhs.path()
            && self.source_file_opt() == rhs.source_file_opt()
            && self.begin_opt() == rhs.begin_opt() && self.end_opt() == rhs.end_opt()
    }
}
