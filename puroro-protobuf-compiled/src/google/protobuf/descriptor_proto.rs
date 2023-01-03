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
pub struct ExtensionRange {
    start: self::_pinternal::OptionalNumericalField::<
        i32,
        self::_pinternal::tags::Int32,
        0usize,
    >,
    end: self::_pinternal::OptionalNumericalField::<
        i32,
        self::_pinternal::tags::Int32,
        1usize,
    >,
    options: self::_pinternal::SingularHeapMessageField::<
        self::_root::google::protobuf::ExtensionRangeOptions,
    >,
    _bitfield: self::_pinternal::BitArray<1usize>,
}
impl ExtensionRange {
    pub fn start(&self) -> i32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.start,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn start_opt(&self) -> ::std::option::Option::<i32> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.start, &self._bitfield)
    }
    pub fn start_mut(&mut self) -> &mut i32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.start,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_start(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.start, &self._bitfield)
            .is_some()
    }
    pub fn clear_start(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::clear(&mut self.start, &mut self._bitfield)
    }
    pub fn end(&self) -> i32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            1usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.end,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn end_opt(&self) -> ::std::option::Option::<i32> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.end, &self._bitfield)
    }
    pub fn end_mut(&mut self) -> &mut i32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            1usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.end,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_end(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.end, &self._bitfield)
            .is_some()
    }
    pub fn clear_end(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            1usize,
        > as NonRepeatedFieldType>::clear(&mut self.end, &mut self._bitfield)
    }
    pub fn options(
        &self,
    ) -> ::std::option::Option::<&self::_root::google::protobuf::ExtensionRangeOptions> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularHeapMessageField::<
            self::_root::google::protobuf::ExtensionRangeOptions,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.options,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn options_opt(
        &self,
    ) -> ::std::option::Option::<&self::_root::google::protobuf::ExtensionRangeOptions> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularHeapMessageField::<
            self::_root::google::protobuf::ExtensionRangeOptions,
        > as NonRepeatedFieldType>::get_field_opt(&self.options, &self._bitfield)
    }
    pub fn options_mut(
        &mut self,
    ) -> &mut self::_root::google::protobuf::ExtensionRangeOptions {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularHeapMessageField::<
            self::_root::google::protobuf::ExtensionRangeOptions,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.options,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_options(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularHeapMessageField::<
            self::_root::google::protobuf::ExtensionRangeOptions,
        > as NonRepeatedFieldType>::get_field_opt(&self.options, &self._bitfield)
            .is_some()
    }
    pub fn clear_options(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::SingularHeapMessageField::<
            self::_root::google::protobuf::ExtensionRangeOptions,
        > as NonRepeatedFieldType>::clear(&mut self.options, &mut self._bitfield)
    }
}
impl self::_puroro::Message for ExtensionRange {
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
                    <self::_pinternal::OptionalNumericalField::<
                        i32,
                        self::_pinternal::tags::Int32,
                        0usize,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.start,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                2i32 => {
                    <self::_pinternal::OptionalNumericalField::<
                        i32,
                        self::_pinternal::tags::Int32,
                        1usize,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.end,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                3i32 => {
                    <self::_pinternal::SingularHeapMessageField::<
                        self::_root::google::protobuf::ExtensionRangeOptions,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.options,
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
        use self::_pinternal::OneofUnion as _;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.start,
            &self._bitfield,
            1i32,
            out,
        )?;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            1usize,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.end,
            &self._bitfield,
            2i32,
            out,
        )?;
        <self::_pinternal::SingularHeapMessageField::<
            self::_root::google::protobuf::ExtensionRangeOptions,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.options,
            &self._bitfield,
            3i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for ExtensionRange {
    fn clone(&self) -> Self {
        Self {
            start: <self::_pinternal::OptionalNumericalField::<
                i32,
                self::_pinternal::tags::Int32,
                0usize,
            > as ::std::clone::Clone>::clone(&self.start),
            end: <self::_pinternal::OptionalNumericalField::<
                i32,
                self::_pinternal::tags::Int32,
                1usize,
            > as ::std::clone::Clone>::clone(&self.end),
            options: <self::_pinternal::SingularHeapMessageField::<
                self::_root::google::protobuf::ExtensionRangeOptions,
            > as ::std::clone::Clone>::clone(&self.options),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
impl ::std::ops::Drop for ExtensionRange {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for ExtensionRange {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(ExtensionRange))
            .field(stringify!(start), &self.start_opt())
            .field(stringify!(end), &self.end_opt())
            .field(stringify!(options), &self.options_opt())
            .finish()
    }
}
impl ::std::cmp::PartialEq for ExtensionRange {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        true && self.start_opt() == rhs.start_opt() && self.end_opt() == rhs.end_opt()
            && self.options_opt() == rhs.options_opt()
    }
}
#[derive(::std::default::Default)]
pub struct ReservedRange {
    start: self::_pinternal::OptionalNumericalField::<
        i32,
        self::_pinternal::tags::Int32,
        0usize,
    >,
    end: self::_pinternal::OptionalNumericalField::<
        i32,
        self::_pinternal::tags::Int32,
        1usize,
    >,
    _bitfield: self::_pinternal::BitArray<1usize>,
}
impl ReservedRange {
    pub fn start(&self) -> i32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.start,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn start_opt(&self) -> ::std::option::Option::<i32> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.start, &self._bitfield)
    }
    pub fn start_mut(&mut self) -> &mut i32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.start,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_start(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.start, &self._bitfield)
            .is_some()
    }
    pub fn clear_start(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        > as NonRepeatedFieldType>::clear(&mut self.start, &mut self._bitfield)
    }
    pub fn end(&self) -> i32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            1usize,
        > as NonRepeatedFieldType>::get_field_or_else(
            &self.end,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn end_opt(&self) -> ::std::option::Option::<i32> {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.end, &self._bitfield)
    }
    pub fn end_mut(&mut self) -> &mut i32 {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            1usize,
        > as NonRepeatedFieldType>::get_field_mut(
            &mut self.end,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn has_end(&self) -> bool {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            1usize,
        > as NonRepeatedFieldType>::get_field_opt(&self.end, &self._bitfield)
            .is_some()
    }
    pub fn clear_end(&mut self) {
        use self::_pinternal::NonRepeatedFieldType;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            1usize,
        > as NonRepeatedFieldType>::clear(&mut self.end, &mut self._bitfield)
    }
}
impl self::_puroro::Message for ReservedRange {
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
                    <self::_pinternal::OptionalNumericalField::<
                        i32,
                        self::_pinternal::tags::Int32,
                        0usize,
                    > as self::_pinternal::FieldType>::deser_from_iter(
                        &mut self.start,
                        &mut self._bitfield,
                        field_data,
                    )?
                }
                2i32 => {
                    <self::_pinternal::OptionalNumericalField::<
                        i32,
                        self::_pinternal::tags::Int32,
                        1usize,
                    > as self::_pinternal::FieldType>::deser_from_iter(
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
        use self::_pinternal::OneofUnion as _;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            0usize,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.start,
            &self._bitfield,
            1i32,
            out,
        )?;
        <self::_pinternal::OptionalNumericalField::<
            i32,
            self::_pinternal::tags::Int32,
            1usize,
        > as self::_pinternal::FieldType>::ser_to_write(
            &self.end,
            &self._bitfield,
            2i32,
            out,
        )?;
        ::std::result::Result::Ok(())
    }
}
impl ::std::clone::Clone for ReservedRange {
    fn clone(&self) -> Self {
        Self {
            start: <self::_pinternal::OptionalNumericalField::<
                i32,
                self::_pinternal::tags::Int32,
                0usize,
            > as ::std::clone::Clone>::clone(&self.start),
            end: <self::_pinternal::OptionalNumericalField::<
                i32,
                self::_pinternal::tags::Int32,
                1usize,
            > as ::std::clone::Clone>::clone(&self.end),
            _bitfield: ::std::clone::Clone::clone(&self._bitfield),
        }
    }
}
impl ::std::ops::Drop for ReservedRange {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
    }
}
impl ::std::fmt::Debug for ReservedRange {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct(stringify!(ReservedRange))
            .field(stringify!(start), &self.start_opt())
            .field(stringify!(end), &self.end_opt())
            .finish()
    }
}
impl ::std::cmp::PartialEq for ReservedRange {
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(unused)]
        use self::_pinternal::OneofUnion as _;
        true && self.start_opt() == rhs.start_opt() && self.end_opt() == rhs.end_opt()
    }
}
