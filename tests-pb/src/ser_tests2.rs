// A generated source code by puroro library
// package ser_tests2
pub mod msg;

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub mod _puroro {
    pub use ::puroro::*;
}

#[derive(Default)]
pub struct Msg {
    // Optional, Variant(Int32)
    i32_optional: self::_puroro::internal::field_type::OptionalNumericalField<
        i32,
        self::_puroro::tags::Int32,
        0,
    >,
    // Repeated, Variant(Int32)
    i32_repeated: self::_puroro::internal::field_type::RepeatedNumericalField<
        i32,
        self::_puroro::tags::Int32,
    >,
    // Optional, Bits32(Float)
    float_optional: self::_puroro::internal::field_type::OptionalNumericalField<
        f32,
        self::_puroro::tags::Float,
        1,
    >,
    // Repeated, Bits32(Float)
    float_repeated: self::_puroro::internal::field_type::RepeatedNumericalField<
        f32,
        self::_puroro::tags::Float,
    >,
    // Optional, LengthDelimited(String)
    string_optional: self::_puroro::internal::field_type::OptionalStringField<2>,
    // Repeated, LengthDelimited(String)
    string_repeated: self::_puroro::internal::field_type::RepeatedStringField,
    // Optional, LengthDelimited(Message(Fqtn(".ser_tests2.Msg.Submsg")))
    submsg_optional: self::_puroro::internal::field_type::SingularHeapMessageField<
        _puroro_root::ser_tests2::msg::Submsg,
    >,
    // Repeated, LengthDelimited(Message(Fqtn(".ser_tests2.Msg.Submsg")))
    submsg_repeated: self::_puroro::internal::field_type::RepeatedMessageField<
        _puroro_root::ser_tests2::msg::Submsg,
    >,
    // Optional, Variant(Enum2(Fqtn(".ser_tests2.Enum")))
    enum_optional: self::_puroro::internal::field_type::OptionalNumericalField<
        _puroro_root::ser_tests2::Enum,
        self::_puroro::tags::Enum2<_puroro_root::ser_tests2::Enum>,
        4,
    >,
    // Repeated, Variant(Enum2(Fqtn(".ser_tests2.Enum")))
    enum_repeated: self::_puroro::internal::field_type::RepeatedNumericalField<
        _puroro_root::ser_tests2::Enum,
        self::_puroro::tags::Enum2<_puroro_root::ser_tests2::Enum>,
    >,
    // Optional, Variant(Int32)
    very_large_field_number: self::_puroro::internal::field_type::OptionalNumericalField<
        i32,
        self::_puroro::tags::Int32,
        5,
    >,

    _bitfield: self::_puroro::bitvec::BitArray<1>,
}

impl Msg {
    // Optional, Variant(Int32)
    pub fn i32_optional(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as NonRepeatedFieldType>::get_field(
            &self.i32_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn i32_optional_opt(&self) -> ::std::option::Option<i32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as NonRepeatedFieldType>::get_field_opt(&self.i32_optional, &self._bitfield)
    }
    pub fn has_i32_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as NonRepeatedFieldType>::get_field_opt(&self.i32_optional, &self._bitfield)
        .is_some()
    }
    pub fn i32_optional_mut(&mut self) -> &mut i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.i32_optional,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_i32_optional(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            0,
        > as NonRepeatedFieldType>::clear(&mut self.i32_optional, &mut self._bitfield)
    }
    // Repeated, Variant(Int32)
    pub fn i32_repeated(&self) -> &[i32] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField<i32, self::_puroro::tags::Int32> as RepeatedFieldType>::get_field(
            &self.i32_repeated, &self._bitfield, 
        )
    }
    pub fn i32_repeated_mut(&mut self) -> &mut ::std::vec::Vec<i32> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField<i32, self::_puroro::tags::Int32> as RepeatedFieldType>::mut_field(
            &mut self.i32_repeated, &mut self._bitfield, 
        )
    }
    pub fn clear_i32_repeated(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField<i32, self::_puroro::tags::Int32> as RepeatedFieldType>::clear(
            &mut self.i32_repeated, &mut self._bitfield, 
        )
    }
    // Optional, Bits32(Float)
    pub fn float_optional(&self) -> f32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            f32,
            self::_puroro::tags::Float,
            1,
        > as NonRepeatedFieldType>::get_field(
            &self.float_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn float_optional_opt(&self) -> ::std::option::Option<f32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            f32,
            self::_puroro::tags::Float,
            1,
        > as NonRepeatedFieldType>::get_field_opt(&self.float_optional, &self._bitfield)
    }
    pub fn has_float_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            f32,
            self::_puroro::tags::Float,
            1,
        > as NonRepeatedFieldType>::get_field_opt(&self.float_optional, &self._bitfield)
        .is_some()
    }
    pub fn float_optional_mut(&mut self) -> &mut f32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            f32,
            self::_puroro::tags::Float,
            1,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.float_optional,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_float_optional(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            f32,
            self::_puroro::tags::Float,
            1,
        > as NonRepeatedFieldType>::clear(&mut self.float_optional, &mut self._bitfield)
    }
    // Repeated, Bits32(Float)
    pub fn float_repeated(&self) -> &[f32] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField<f32, self::_puroro::tags::Float> as RepeatedFieldType>::get_field(
            &self.float_repeated, &self._bitfield, 
        )
    }
    pub fn float_repeated_mut(&mut self) -> &mut ::std::vec::Vec<f32> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField<f32, self::_puroro::tags::Float> as RepeatedFieldType>::mut_field(
            &mut self.float_repeated, &mut self._bitfield, 
        )
    }
    pub fn clear_float_repeated(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField<f32, self::_puroro::tags::Float> as RepeatedFieldType>::clear(
            &mut self.float_repeated, &mut self._bitfield, 
        )
    }
    // Optional, LengthDelimited(String)
    pub fn string_optional(&self) -> &str {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField<2> as NonRepeatedFieldType>::get_field(
            &self.string_optional, &self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn string_optional_opt(&self) -> ::std::option::Option<&str> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField<2> as NonRepeatedFieldType>::get_field_opt(
            &self.string_optional, &self._bitfield,
        )
    }
    pub fn has_string_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField<2> as NonRepeatedFieldType>::get_field_opt(
            &self.string_optional, &self._bitfield,
        ).is_some()
    }
    pub fn string_optional_mut(&mut self) -> &mut ::std::string::String {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField<2> as NonRepeatedFieldType>::mut_field(
            &mut self.string_optional, &mut self._bitfield, ::std::default::Default::default,
        )
    }
    pub fn clear_string_optional(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalStringField<2> as NonRepeatedFieldType>::clear(
            &mut self.string_optional,
            &mut self._bitfield,
        )
    }
    // Repeated, LengthDelimited(String)
    pub fn string_repeated(&self) -> &[::std::string::String] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedStringField as RepeatedFieldType>::get_field(
            &self.string_repeated,
            &self._bitfield,
        )
    }
    pub fn string_repeated_mut(&mut self) -> &mut ::std::vec::Vec<::std::string::String> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedStringField as RepeatedFieldType>::mut_field(
            &mut self.string_repeated,
            &mut self._bitfield,
        )
    }
    pub fn clear_string_repeated(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedStringField as RepeatedFieldType>::clear(
            &mut self.string_repeated,
            &mut self._bitfield,
        )
    }
    // Optional, LengthDelimited(Message(Fqtn(".ser_tests2.Msg.Submsg")))
    pub fn submsg_optional(&self) -> Option<&_puroro_root::ser_tests2::msg::Submsg> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::ser_tests2::msg::Submsg,
        > as NonRepeatedFieldType>::get_field(
            &self.submsg_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn submsg_optional_opt(
        &self,
    ) -> ::std::option::Option<&_puroro_root::ser_tests2::msg::Submsg> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::ser_tests2::msg::Submsg,
        > as NonRepeatedFieldType>::get_field_opt(&self.submsg_optional, &self._bitfield)
    }
    pub fn has_submsg_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::ser_tests2::msg::Submsg,
        > as NonRepeatedFieldType>::get_field_opt(&self.submsg_optional, &self._bitfield)
        .is_some()
    }
    pub fn submsg_optional_mut(&mut self) -> &mut _puroro_root::ser_tests2::msg::Submsg {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::ser_tests2::msg::Submsg,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.submsg_optional,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_submsg_optional(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularHeapMessageField<
            _puroro_root::ser_tests2::msg::Submsg,
        > as NonRepeatedFieldType>::clear(&mut self.submsg_optional, &mut self._bitfield)
    }
    // Repeated, LengthDelimited(Message(Fqtn(".ser_tests2.Msg.Submsg")))
    pub fn submsg_repeated(&self) -> &[_puroro_root::ser_tests2::msg::Submsg] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::ser_tests2::msg::Submsg,
        > as RepeatedFieldType>::get_field(&self.submsg_repeated, &self._bitfield)
    }
    pub fn submsg_repeated_mut(
        &mut self,
    ) -> &mut ::std::vec::Vec<_puroro_root::ser_tests2::msg::Submsg> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::ser_tests2::msg::Submsg,
        > as RepeatedFieldType>::mut_field(&mut self.submsg_repeated, &mut self._bitfield)
    }
    pub fn clear_submsg_repeated(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedMessageField<
            _puroro_root::ser_tests2::msg::Submsg,
        > as RepeatedFieldType>::clear(&mut self.submsg_repeated, &mut self._bitfield)
    }
    // Optional, Variant(Enum2(Fqtn(".ser_tests2.Enum")))
    pub fn enum_optional(&self) -> _puroro_root::ser_tests2::Enum {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::ser_tests2::Enum,
            self::_puroro::tags::Enum2<_puroro_root::ser_tests2::Enum>,
            4,
        > as NonRepeatedFieldType>::get_field(
            &self.enum_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn enum_optional_opt(&self) -> ::std::option::Option<_puroro_root::ser_tests2::Enum> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::ser_tests2::Enum,
            self::_puroro::tags::Enum2<_puroro_root::ser_tests2::Enum>,
            4,
        > as NonRepeatedFieldType>::get_field_opt(&self.enum_optional, &self._bitfield)
    }
    pub fn has_enum_optional(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::ser_tests2::Enum,
            self::_puroro::tags::Enum2<_puroro_root::ser_tests2::Enum>,
            4,
        > as NonRepeatedFieldType>::get_field_opt(&self.enum_optional, &self._bitfield)
        .is_some()
    }
    pub fn enum_optional_mut(&mut self) -> &mut _puroro_root::ser_tests2::Enum {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::ser_tests2::Enum,
            self::_puroro::tags::Enum2<_puroro_root::ser_tests2::Enum>,
            4,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.enum_optional,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_enum_optional(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            _puroro_root::ser_tests2::Enum,
            self::_puroro::tags::Enum2<_puroro_root::ser_tests2::Enum>,
            4,
        > as NonRepeatedFieldType>::clear(&mut self.enum_optional, &mut self._bitfield)
    }
    // Repeated, Variant(Enum2(Fqtn(".ser_tests2.Enum")))
    pub fn enum_repeated(&self) -> &[_puroro_root::ser_tests2::Enum] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            _puroro_root::ser_tests2::Enum,
            self::_puroro::tags::Enum2<_puroro_root::ser_tests2::Enum>,
        > as RepeatedFieldType>::get_field(&self.enum_repeated, &self._bitfield)
    }
    pub fn enum_repeated_mut(&mut self) -> &mut ::std::vec::Vec<_puroro_root::ser_tests2::Enum> {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            _puroro_root::ser_tests2::Enum,
            self::_puroro::tags::Enum2<_puroro_root::ser_tests2::Enum>,
        > as RepeatedFieldType>::mut_field(&mut self.enum_repeated, &mut self._bitfield)
    }
    pub fn clear_enum_repeated(&mut self) {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField<
            _puroro_root::ser_tests2::Enum,
            self::_puroro::tags::Enum2<_puroro_root::ser_tests2::Enum>,
        > as RepeatedFieldType>::clear(&mut self.enum_repeated, &mut self._bitfield)
    }
    // Optional, Variant(Int32)
    pub fn very_large_field_number(&self) -> i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            5,
        > as NonRepeatedFieldType>::get_field(
            &self.very_large_field_number,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn very_large_field_number_opt(&self) -> ::std::option::Option<i32> {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            5,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.very_large_field_number, &self._bitfield
        )
    }
    pub fn has_very_large_field_number(&self) -> bool {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            5,
        > as NonRepeatedFieldType>::get_field_opt(
            &self.very_large_field_number, &self._bitfield
        )
        .is_some()
    }
    pub fn very_large_field_number_mut(&mut self) -> &mut i32 {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            5,
        > as NonRepeatedFieldType>::mut_field(
            &mut self.very_large_field_number,
            &mut self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn clear_very_large_field_number(&mut self) {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField<
            i32,
            self::_puroro::tags::Int32,
            5,
        > as NonRepeatedFieldType>::clear(
            &mut self.very_large_field_number, &mut self._bitfield
        )
    }
}

impl self::_puroro::Message for Msg {
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
        use ::std::option::Option::Some;
        #[allow(unused)]
        use ::std::result::Result::Ok;
        while let Some((number, _field_data)) =
            self::_puroro::internal::ser::FieldData::from_bytes_iter(iter.by_ref())?
        {
            match number {
                1 => <
                    self::_puroro::internal::field_type::OptionalNumericalField<i32, self::_puroro::tags::Int32, 0> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.i32_optional,
                    &mut self._bitfield,
                    _field_data,
                )?,
                2 => <
                    self::_puroro::internal::field_type::RepeatedNumericalField<i32, self::_puroro::tags::Int32> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.i32_repeated,
                    &mut self._bitfield,
                    _field_data,
                )?,
                3 => <
                    self::_puroro::internal::field_type::OptionalNumericalField<f32, self::_puroro::tags::Float, 1> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.float_optional,
                    &mut self._bitfield,
                    _field_data,
                )?,
                4 => <
                    self::_puroro::internal::field_type::RepeatedNumericalField<f32, self::_puroro::tags::Float> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.float_repeated,
                    &mut self._bitfield,
                    _field_data,
                )?,
                5 => <
                    self::_puroro::internal::field_type::OptionalStringField<2> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.string_optional,
                    &mut self._bitfield,
                    _field_data,
                )?,
                6 => <
                    self::_puroro::internal::field_type::RepeatedStringField as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.string_repeated,
                    &mut self._bitfield,
                    _field_data,
                )?,
                7 => <
                    self::_puroro::internal::field_type::SingularHeapMessageField<_puroro_root::ser_tests2::msg::Submsg> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.submsg_optional,
                    &mut self._bitfield,
                    _field_data,
                )?,
                8 => <
                    self::_puroro::internal::field_type::RepeatedMessageField<_puroro_root::ser_tests2::msg::Submsg> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.submsg_repeated,
                    &mut self._bitfield,
                    _field_data,
                )?,
                9 => <
                    self::_puroro::internal::field_type::OptionalNumericalField<_puroro_root::ser_tests2::Enum, self::_puroro::tags::Enum2<_puroro_root::ser_tests2::Enum>, 4> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.enum_optional,
                    &mut self._bitfield,
                    _field_data,
                )?,
                10 => <
                    self::_puroro::internal::field_type::RepeatedNumericalField<_puroro_root::ser_tests2::Enum, self::_puroro::tags::Enum2<_puroro_root::ser_tests2::Enum>> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.enum_repeated,
                    &mut self._bitfield,
                    _field_data,
                )?,
                536870911 => <
                    self::_puroro::internal::field_type::OptionalNumericalField<i32, self::_puroro::tags::Int32, 5> as self::_puroro::internal::field_type::FieldType
                >::deser_from_iter(
                    &mut self.very_large_field_number,
                    &mut self._bitfield,
                    _field_data,
                )?,
                _ => todo!(),
            }
        }
        Ok(())
    }
}

impl ::std::clone::Clone for Msg {
    fn clone(&self) -> Self {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
        #[allow(unused)]
        use ::std::clone::Clone;
        Self {
            i32_optional: Clone::clone(&self.i32_optional),
            i32_repeated: Clone::clone(&self.i32_repeated),
            float_optional: Clone::clone(&self.float_optional),
            float_repeated: Clone::clone(&self.float_repeated),
            string_optional: Clone::clone(&self.string_optional),
            string_repeated: Clone::clone(&self.string_repeated),
            submsg_optional: Clone::clone(&self.submsg_optional),
            submsg_repeated: Clone::clone(&self.submsg_repeated),
            enum_optional: Clone::clone(&self.enum_optional),
            enum_repeated: Clone::clone(&self.enum_repeated),
            very_large_field_number: Clone::clone(&self.very_large_field_number),

            _bitfield: Clone::clone(&self._bitfield),
        }
    }
}

impl ::std::fmt::Debug for Msg {
    fn fmt(
        &self,
        fmt: &mut ::std::fmt::Formatter<'_>,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct("Msg")
            .field("i32_optional", &self.i32_optional())
            .field("i32_repeated", &self.i32_repeated())
            .field("float_optional", &self.float_optional())
            .field("float_repeated", &self.float_repeated())
            .field("string_optional", &self.string_optional())
            .field("string_repeated", &self.string_repeated())
            .field("submsg_optional", &self.submsg_optional())
            .field("submsg_repeated", &self.submsg_repeated())
            .field("enum_optional", &self.enum_optional())
            .field("enum_repeated", &self.enum_repeated())
            .field("very_large_field_number", &self.very_large_field_number())
            .finish()
    }
}

impl ::std::ops::Drop for Msg {
    fn drop(&mut self) {
        #[allow(unused)]
        use self::_puroro::internal::oneof_type::OneofUnion;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Enum {
    ZEROTH,
    FIRST,
    TENTH,
}

impl ::std::default::Default for Enum {
    fn default() -> Self {
        Enum::ZEROTH
    }
}

impl ::std::convert::TryFrom<i32> for Enum {
    type Error = self::_puroro::PuroroError;
    fn try_from(x: i32) -> ::std::result::Result<Self, Self::Error> {
        #[allow(unused)]
        use ::std::result::Result::{Err, Ok};
        match x {
            0 => Ok(self::Enum::ZEROTH),
            1 => Ok(self::Enum::FIRST),
            10 => Ok(self::Enum::TENTH),
            e => Err(self::_puroro::ErrorKind::UnknownEnumVariant(e))?,
        }
    }
}

impl ::std::convert::From<Enum> for i32 {
    fn from(x: Enum) -> i32 {
        match x {
            self::Enum::ZEROTH => 0,
            self::Enum::FIRST => 1,
            self::Enum::TENTH => 10,
        }
    }
}
