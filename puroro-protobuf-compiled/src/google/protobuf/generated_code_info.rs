// A generated source code by puroro library
// package .google.protobuf.GeneratedCodeInfo

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub mod _puroro {
    pub use ::puroro::*;
}

#[derive(Default)]
pub struct Annotation {
    // Repeated, Variant(Int32)
    path: self::_puroro::internal::field_type::RepeatedNumericalField<
        i32,
        self::_puroro::tags::Int32,
    >,
    // Optional, LengthDelimited(String)
    source_file: self::_puroro::internal::field_type::OptionalStringField<0>,
    // Optional, Variant(Int32)
    begin: self::_puroro::internal::field_type::OptionalNumericalField<
        i32,
        self::_puroro::tags::Int32,
        1,
    >,
    // Optional, Variant(Int32)
    end: self::_puroro::internal::field_type::OptionalNumericalField<
        i32,
        self::_puroro::tags::Int32,
        2,
    >,

    _bitfield: self::_puroro::bitvec::BitArray<1>,
}

impl Annotation {
    // Repeated, Variant(Int32)
    pub fn path(&self) -> &[i32] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<i32, self::_puroro::tags::Int32> as RepeatedFieldType>::get_field(
            &self.path, &self._bitfield, 
        )
    }
    pub fn path_mut(&mut self) -> &mut ::std::vec::Vec<i32> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<i32, self::_puroro::tags::Int32> as RepeatedFieldType>::mut_field(
            &mut self.path, &mut self._bitfield, 
        )
    }
    pub fn clear_path(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<i32, self::_puroro::tags::Int32> as RepeatedFieldType>::clear(
            &mut self.path, &mut self._bitfield, 
        )
    }
    // Optional, LengthDelimited(String)
    pub fn source_file(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field(
            &self.source_file, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn source_file_opt(&self) -> ::std::option::Option<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field_opt(
            &self.source_file, &self._bitfield,
        )
    }
    pub fn has_source_file(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::get_field_opt(
            &self.source_file, &self._bitfield,
        ).is_some()
    }
    pub fn source_file_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField::<0> as NonRepeatedFieldType>::mut_field(
            &mut self.source_file, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_source_file(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField<0> as NonRepeatedFieldType>::clear(
            &mut self.source_file,
            &mut self._bitfield,
        )
    }
    // Optional, Variant(Int32)
    pub fn begin(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            1,
        > as NonRepeatedFieldType>::get_field(
            &self.begin,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn begin_opt(&self) -> ::std::option::Option<i32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            1,
        > as NonRepeatedFieldType>::get_field_opt(&self.begin, &self._bitfield)
    }
    pub fn has_begin(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            1,
        > as NonRepeatedFieldType>::get_field_opt(&self.begin, &self._bitfield)
        .is_some()
    }
    pub fn begin_mut(&mut self) -> &mut i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            1,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.begin,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_begin(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            1,
        > as NonRepeatedFieldType>::clear(&mut self.begin, &mut self._bitfield)
    }
    // Optional, Variant(Int32)
    pub fn end(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            2,
        > as NonRepeatedFieldType>::get_field(
            &self.end,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn end_opt(&self) -> ::std::option::Option<i32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            2,
        > as NonRepeatedFieldType>::get_field_opt(&self.end, &self._bitfield)
    }
    pub fn has_end(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            2,
        > as NonRepeatedFieldType>::get_field_opt(&self.end, &self._bitfield)
        .is_some()
    }
    pub fn end_mut(&mut self) -> &mut i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            2,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.end,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_end(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            2,
        > as NonRepeatedFieldType>::clear(&mut self.end, &mut self._bitfield)
    }
}

impl self::_puroro::Message for Annotation {
    fn from_bytes_iter<I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>>(
        iter: I,
    ) -> self::_puroro::Result<Self> {
        #[allow(unused)]
        use ::std::result::Result::Ok;
        let mut msg: Self = ::std::default::Default::default();
        msg.merge_from_bytes_iter(iter)?;
        Ok(msg)
    }

    fn merge_from_bytes_iter<I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>>(
        &mut self,
        mut iter: I,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use self::_puroro::internal::field_type::FieldType;
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::option::Option::Some;
        #[allow(unused)]
        use ::std::result::Result::Ok;
        while let Some((number, field_data)) =
            self::_puroro::internal::ser::FieldData::from_bytes_iter(iter.by_ref())?
        {
            match number {
                1 => <self::_puroro::internal::field_type::RepeatedNumericalField::<i32, self::_puroro::tags::Int32> as FieldType>::deser_from_iter(
                    &mut self.path,
                    &mut self._bitfield,
                    field_data,
                )?,
                2 => <self::_puroro::internal::field_type::OptionalStringField::<0> as FieldType>::deser_from_iter(
                    &mut self.source_file,
                    &mut self._bitfield,
                    field_data,
                )?,
                3 => <self::_puroro::internal::field_type::OptionalNumericalField::<i32, self::_puroro::tags::Int32, 1> as FieldType>::deser_from_iter(
                    &mut self.begin,
                    &mut self._bitfield,
                    field_data,
                )?,
                4 => <self::_puroro::internal::field_type::OptionalNumericalField::<i32, self::_puroro::tags::Int32, 2> as FieldType>::deser_from_iter(
                    &mut self.end,
                    &mut self._bitfield,
                    field_data,
                )?,
                _ => todo!(), // Unknown field...
            }
        }
        Ok(())
    }

    fn to_bytes<W: ::std::io::Write>(
        &self,
        #[allow(unused)] out: &mut W,
    ) -> self::_puroro::Result<()> {
        #[allow(unused)]
        use self::_puroro::internal::field_type::FieldType;
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::result::Result::Ok;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            i32,
            self::_puroro::tags::Int32,
        > as FieldType>::ser_to_write(&self.path, &self._bitfield, 1, out)?;
        <self::_puroro::internal::field_type::OptionalStringField<0> as FieldType>::ser_to_write(
            &self.source_file,
            &self._bitfield,
            2,
            out,
        )?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            1,
        > as FieldType>::ser_to_write(&self.begin, &self._bitfield, 3, out)?;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            2,
        > as FieldType>::ser_to_write(&self.end, &self._bitfield, 4, out)?;

        Ok(())
    }
}

impl ::std::clone::Clone for Annotation {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::clone::Clone;
        Self {
            path: Clone::clone(&self.path),
            source_file: Clone::clone(&self.source_file),
            begin: Clone::clone(&self.begin),
            end: Clone::clone(&self.end),

            _bitfield: Clone::clone(&self._bitfield),
        }
    }
}

impl ::std::fmt::Debug for Annotation {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct("Annotation")
            .field("path", &self.path())
            .field("source_file", &self.source_file())
            .field("begin", &self.begin())
            .field("end", &self.end())
            .finish()
    }
}

impl ::std::cmp::PartialEq for Annotation {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;

        true && self.path() == rhs.path()
            && self.source_file_opt() == rhs.source_file_opt()
            && self.begin_opt() == rhs.begin_opt()
            && self.end_opt() == rhs.end_opt()
    }
}

impl ::std::ops::Drop for Annotation {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
    }
}
