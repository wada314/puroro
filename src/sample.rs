//! Example of what the code generator would emit for a `.proto` file.
//!
//! The source `.proto` this module corresponds to is shown below.  The Rust
//! code that follows is **hand-written** to illustrate the intended generated
//! interface; an actual code generator would produce equivalent output
//! automatically.
//!
//! ```proto
//! syntax = "proto3";
//! package example;
//!
//! // ── Enum ──────────────────────────────────────────────────────────────────
//! enum PhoneType {
//!     PHONE_TYPE_UNSPECIFIED = 0;
//!     MOBILE = 1;
//!     HOME   = 2;
//!     WORK   = 3;
//! }
//!
//! // ── Simple nested message ─────────────────────────────────────────────────
//! message Address {
//!     string street = 1;
//!     string city   = 2;
//! }
//!
//! // ── Main message (covers most field kinds) ────────────────────────────────
//! message Person {
//!     string               name               = 1;  // singular string
//!     int32                age                = 2;  // singular scalar
//!     bytes                avatar             = 3;  // singular bytes
//!     repeated string      emails             = 4;  // repeated string
//!     Address              address            = 5;  // singular message
//!     PhoneType            primary_phone_type = 6;  // enum
//!     oneof contact_method {
//!         string phone_number = 7;
//!         string fax_number   = 8;
//!     }
//! }
//! ```

pub mod example {
    use ::allocator_api2::alloc::{Allocator, Global};
    use ::allocator_api2::boxed::Box as ABox;
    use ::allocator_api2::vec::Vec as AVec;
    use crate::decode::{self, MessageDecode};
    use crate::encode::{self, MessageEncode};
    use crate::error::DecodeError;
    use crate::wire_type::WireType;

    // =========================================================================
    // Enum: PhoneType
    //
    // Proto3 enums are "open": the wire may carry values not listed here.
    // The field is stored as `i32` in the parent message; a typed accessor
    // wraps it in `Result<PhoneType, i32>` so callers can handle unknown
    // values explicitly.
    // =========================================================================

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    #[repr(i32)]
    pub enum PhoneType {
        Unspecified = 0,
        Mobile = 1,
        Home = 2,
        Work = 3,
    }

    impl TryFrom<i32> for PhoneType {
        type Error = i32;
        fn try_from(v: i32) -> Result<Self, i32> {
            match v {
                0 => Ok(PhoneType::Unspecified),
                1 => Ok(PhoneType::Mobile),
                2 => Ok(PhoneType::Home),
                3 => Ok(PhoneType::Work),
                other => Err(other),
            }
        }
    }

    impl From<PhoneType> for i32 {
        fn from(v: PhoneType) -> i32 {
            v as i32
        }
    }

    // =========================================================================
    // Message: Address
    //
    // Demonstrates the pattern for a simple message with two string fields.
    // All string fields are stored as `Box<str, A>` (the smallest owned string
    // representation with allocator support) and exposed via `&str` accessors.
    // =========================================================================

    pub struct Address<A: Allocator = Global> {
        street: ABox<str, A>,
        city: ABox<str, A>,
        _unknown_fields: AVec<u8, A>,
        _alloc: A,
    }

    // ── Constructors ──────────────────────────────────────────────────────────

    impl<A: Allocator + Clone> Address<A> {
        /// Creates an empty `Address` using the given allocator.
        pub fn new_in(alloc: A) -> Self {
            Address {
                street: decode::str_to_box_in("", alloc.clone()),
                city: decode::str_to_box_in("", alloc.clone()),
                _unknown_fields: AVec::new_in(alloc.clone()),
                _alloc: alloc,
            }
        }
    }

    impl Address<Global> {
        /// Creates an empty `Address` using the global allocator.
        pub fn new() -> Self {
            Self::new_in(Global)
        }
    }

    impl<A: Allocator + Clone + Default> Default for Address<A> {
        fn default() -> Self {
            Self::new_in(A::default())
        }
    }

    // ── Field accessors ───────────────────────────────────────────────────────

    impl<A: Allocator + Clone> Address<A> {
        // ---- street ----

        pub fn street(&self) -> &str {
            &self.street
        }

        pub fn set_street(&mut self, v: &str) {
            self.street = decode::str_to_box_in(v, self._alloc.clone());
        }

        // ---- city ----

        pub fn city(&self) -> &str {
            &self.city
        }

        pub fn set_city(&mut self, v: &str) {
            self.city = decode::str_to_box_in(v, self._alloc.clone());
        }

        // ---- unknown fields ----

        /// Returns raw bytes of any fields unknown to this schema version.
        ///
        /// These bytes are a valid (partial) protobuf message and are
        /// re-serialised verbatim on encode, preserving forward compatibility.
        pub fn unknown_fields(&self) -> &[u8] {
            &self._unknown_fields
        }
    }

    // ── MessageEncode ─────────────────────────────────────────────────────────

    impl<A: Allocator + Clone> MessageEncode for Address<A> {
        fn encoded_len(&self) -> usize {
            // Field 1: string street
            let street_bytes = self.street.as_bytes();
            let mut len = if street_bytes.is_empty() {
                0
            } else {
                encode::encoded_len_len_field(1, street_bytes.len())
            };
            // Field 2: string city
            let city_bytes = self.city.as_bytes();
            if !city_bytes.is_empty() {
                len += encode::encoded_len_len_field(2, city_bytes.len());
            }
            // Unknown fields pass through unchanged.
            len += self._unknown_fields.len();
            len
        }

        fn encode_raw<B: ::bytes::BufMut>(&self, buf: &mut B) {
            // Field 1: string street (omit if default empty string)
            if !self.street.is_empty() {
                encode::encode_len_field(1, self.street.as_bytes(), buf);
            }
            // Field 2: string city
            if !self.city.is_empty() {
                encode::encode_len_field(2, self.city.as_bytes(), buf);
            }
            // Re-emit unknown fields verbatim.
            buf.put_slice(&self._unknown_fields);
        }
    }

    // ── MessageDecode ─────────────────────────────────────────────────────────

    impl<A: Allocator + Clone + Default> MessageDecode for Address<A> {
        fn merge_from<B: ::bytes::Buf>(&mut self, buf: &mut B) -> Result<(), DecodeError> {
            use ::bytes::Buf as _;
            while buf.has_remaining() {
                let (field_number, wire_type) = decode::decode_tag(buf)?;
                match (field_number, wire_type) {
                    (1, WireType::Len) => {
                        self.street = decode::decode_string_in(buf, self._alloc.clone())?;
                    }
                    (2, WireType::Len) => {
                        self.city = decode::decode_string_in(buf, self._alloc.clone())?;
                    }
                    _ => {
                        decode::skip_field_and_save(
                            field_number,
                            wire_type,
                            buf,
                            &mut self._unknown_fields,
                        )?;
                    }
                }
            }
            Ok(())
        }
    }

    // =========================================================================
    // Oneof enum for Person::contact_method
    //
    // Placed in a submodule named after the parent message (lower-snake-case).
    // Each oneof variant carries the value of that field.
    // =========================================================================

    pub mod person {
        use ::allocator_api2::alloc::{Allocator, Global};
        use ::allocator_api2::boxed::Box as ABox;

        pub enum ContactMethod<A: Allocator = Global> {
            /// Field 7: phone_number
            PhoneNumber(ABox<str, A>),
            /// Field 8: fax_number
            FaxNumber(ABox<str, A>),
        }

        impl<A: Allocator> ::core::fmt::Debug for ContactMethod<A> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                match self {
                    ContactMethod::PhoneNumber(s) => {
                        f.debug_tuple("PhoneNumber").field(&s.as_ref()).finish()
                    }
                    ContactMethod::FaxNumber(s) => {
                        f.debug_tuple("FaxNumber").field(&s.as_ref()).finish()
                    }
                }
            }
        }
    }

    // =========================================================================
    // Message: Person
    //
    // Demonstrates:
    //   • singular scalar (int32)
    //   • singular string / bytes
    //   • repeated string (non-packed, because strings cannot be packed)
    //   • singular message (optional in the generated struct)
    //   • enum field (stored as i32, typed accessor returns Result)
    //   • oneof field
    //   • unknown fields
    // =========================================================================

    pub struct Person<A: Allocator = Global> {
        // Field 1: string name
        name: ABox<str, A>,
        // Field 2: int32 age
        age: i32,
        // Field 3: bytes avatar
        avatar: AVec<u8, A>,
        // Field 4: repeated string emails
        emails: AVec<ABox<str, A>, A>,
        // Field 5: Address address (optional in generated code; None means "not set")
        address: Option<ABox<Address<A>, A>>,
        // Field 6: PhoneType primary_phone_type (stored as i32 for "open enum" compliance)
        primary_phone_type: i32,
        // Fields 7–8: oneof contact_method
        contact_method: Option<person::ContactMethod<A>>,
        // Unknown fields from future schema versions are stored as raw wire bytes.
        _unknown_fields: AVec<u8, A>,
        // Stored allocator — used by setter / push methods to create new heap values.
        // Requires A: Clone. For zero-cost allocators such as `Global` (a ZST) this
        // adds no size or runtime overhead.
        _alloc: A,
    }

    // ── Constructors ──────────────────────────────────────────────────────────

    impl<A: Allocator + Clone> Person<A> {
        /// Creates an empty `Person` using the given allocator.
        ///
        /// All fields are initialised to their proto3 default values:
        /// scalars to `0` / `false`, strings/bytes to empty, repeated to
        /// empty, message fields to `None`.
        pub fn new_in(alloc: A) -> Self {
            Person {
                name: decode::str_to_box_in("", alloc.clone()),
                age: 0,
                avatar: AVec::new_in(alloc.clone()),
                emails: AVec::new_in(alloc.clone()),
                address: None,
                primary_phone_type: 0,
                contact_method: None,
                _unknown_fields: AVec::new_in(alloc.clone()),
                _alloc: alloc,
            }
        }
    }

    impl Person<Global> {
        /// Creates an empty `Person` using the global allocator.
        pub fn new() -> Self {
            Self::new_in(Global)
        }
    }

    impl<A: Allocator + Clone + Default> Default for Person<A> {
        fn default() -> Self {
            Self::new_in(A::default())
        }
    }

    // ── Field accessors ───────────────────────────────────────────────────────

    impl<A: Allocator + Clone> Person<A> {
        // ---- name (string) ----

        pub fn name(&self) -> &str {
            &self.name
        }

        pub fn set_name(&mut self, v: &str) {
            self.name = decode::str_to_box_in(v, self._alloc.clone());
        }

        // ---- age (int32) ----
        //
        // Canonical scalar pattern. int64, uint32, uint64, sint32, sint64,
        // fixed32, fixed64, sfixed32, sfixed64, bool, float, double all follow
        // the same structure; only the Rust type and the wire encoding differ.

        pub fn age(&self) -> i32 {
            self.age
        }

        pub fn set_age(&mut self, v: i32) {
            self.age = v;
        }

        // ---- avatar (bytes) ----

        pub fn avatar(&self) -> &[u8] {
            &self.avatar
        }

        pub fn set_avatar(&mut self, v: &[u8]) {
            self.avatar.clear();
            self.avatar.extend_from_slice(v);
        }

        // ---- emails (repeated string) ----
        //
        // Returns a slice of `Box<str, A>`.  Because `Box<str, A>: Deref<Target=str>`
        // this supports deref coercion:
        //
        //   for email in person.emails() {
        //       let s: &str = email;   // coercion via Deref
        //       println!("{email}");   // Display impl on Box<str, A>
        //   }
        //
        // Returning `&[Box<str, A>]` (not `Vec`) gives O(1) random access
        // without forcing an allocation.

        pub fn emails(&self) -> &[ABox<str, A>] {
            &self.emails
        }

        /// Appends a string to the `emails` list.
        pub fn push_email(&mut self, v: &str) {
            self.emails
                .push(decode::str_to_box_in(v, self._alloc.clone()));
        }

        pub fn clear_emails(&mut self) {
            self.emails.clear();
        }

        // ---- address (message, optional) ----

        pub fn address(&self) -> Option<&Address<A>> {
            self.address.as_deref()
        }

        /// Returns a mutable reference to the `address` field, creating a
        /// default `Address` if it is not yet set.
        pub fn address_mut(&mut self) -> &mut Address<A> {
            self.address.get_or_insert_with(|| {
                ABox::new_in(Address::new_in(self._alloc.clone()), self._alloc.clone())
            })
        }

        /// Replaces the `address` field.
        pub fn set_address(&mut self, v: Address<A>) {
            self.address =
                Some(ABox::new_in(v, self._alloc.clone()));
        }

        pub fn clear_address(&mut self) {
            self.address = None;
        }

        // ---- primary_phone_type (enum, stored as i32) ----
        //
        // Proto3 enums are open: the wire can carry numeric values not present
        // in the enum definition.  The raw i32 is always available; the typed
        // accessor returns `Result<PhoneType, i32>` so callers can handle
        // unknown values without panicking.

        pub fn primary_phone_type_raw(&self) -> i32 {
            self.primary_phone_type
        }

        pub fn set_primary_phone_type_raw(&mut self, v: i32) {
            self.primary_phone_type = v;
        }

        pub fn primary_phone_type(&self) -> Result<PhoneType, i32> {
            PhoneType::try_from(self.primary_phone_type)
        }

        pub fn set_primary_phone_type(&mut self, v: PhoneType) {
            self.primary_phone_type = v as i32;
        }

        // ---- contact_method (oneof) ----

        pub fn contact_method(&self) -> Option<&person::ContactMethod<A>> {
            self.contact_method.as_ref()
        }

        pub fn contact_method_mut(&mut self) -> Option<&mut person::ContactMethod<A>> {
            self.contact_method.as_mut()
        }

        pub fn set_contact_method(&mut self, v: Option<person::ContactMethod<A>>) {
            self.contact_method = v;
        }

        /// Sets the `phone_number` variant of the `contact_method` oneof,
        /// clearing any previously set variant.
        pub fn set_phone_number(&mut self, v: &str) {
            self.contact_method = Some(person::ContactMethod::PhoneNumber(
                decode::str_to_box_in(v, self._alloc.clone()),
            ));
        }

        /// Sets the `fax_number` variant of the `contact_method` oneof,
        /// clearing any previously set variant.
        pub fn set_fax_number(&mut self, v: &str) {
            self.contact_method = Some(person::ContactMethod::FaxNumber(
                decode::str_to_box_in(v, self._alloc.clone()),
            ));
        }

        // ---- unknown fields ----

        pub fn unknown_fields(&self) -> &[u8] {
            &self._unknown_fields
        }
    }

    // ── MessageEncode ─────────────────────────────────────────────────────────

    impl<A: Allocator + Clone> MessageEncode for Person<A> {
        fn encoded_len(&self) -> usize {
            let mut len = 0usize;

            // Field 1: string name — omit if the proto3 default (empty string).
            if !self.name.is_empty() {
                len += encode::encoded_len_len_field(1, self.name.len());
            }

            // Field 2: int32 age — omit if default (0).
            // int32 encodes as VARINT, using sign-extended 64-bit two's complement.
            if self.age != 0 {
                len += encode::encoded_len_varint_field(2, self.age as u64);
            }

            // Field 3: bytes avatar — omit if empty.
            if !self.avatar.is_empty() {
                len += encode::encoded_len_len_field(3, self.avatar.len());
            }

            // Field 4: repeated string emails — one LEN record per element.
            for email in &self.emails {
                len += encode::encoded_len_len_field(4, email.len());
            }

            // Field 5: Address address — LEN record wrapping the nested message.
            if let Some(addr) = &self.address {
                let msg_len = addr.encoded_len();
                len += encode::encoded_len_len_field(5, msg_len);
            }

            // Field 6: enum primary_phone_type (stored as i32 VARINT).
            if self.primary_phone_type != 0 {
                len += encode::encoded_len_varint_field(6, self.primary_phone_type as u64);
            }

            // Fields 7–8: oneof contact_method.
            if let Some(cm) = &self.contact_method {
                match cm {
                    person::ContactMethod::PhoneNumber(s) => {
                        len += encode::encoded_len_len_field(7, s.len());
                    }
                    person::ContactMethod::FaxNumber(s) => {
                        len += encode::encoded_len_len_field(8, s.len());
                    }
                }
            }

            // Unknown fields are re-emitted verbatim.
            len += self._unknown_fields.len();
            len
        }

        fn encode_raw<B: ::bytes::BufMut>(&self, buf: &mut B) {
            // Field 1
            if !self.name.is_empty() {
                encode::encode_len_field(1, self.name.as_bytes(), buf);
            }
            // Field 2
            if self.age != 0 {
                encode::encode_varint_field(2, self.age as u64, buf);
            }
            // Field 3
            if !self.avatar.is_empty() {
                encode::encode_len_field(3, &self.avatar, buf);
            }
            // Field 4: one record per email
            for email in &self.emails {
                encode::encode_len_field(4, email.as_bytes(), buf);
            }
            // Field 5: nested message — write tag, then varint length, then payload.
            if let Some(addr) = &self.address {
                let msg_len = addr.encoded_len();
                encode::encode_tag(5, WireType::Len, buf);
                encode::encode_varint(msg_len as u64, buf);
                addr.encode_raw(buf);
            }
            // Field 6
            if self.primary_phone_type != 0 {
                encode::encode_varint_field(6, self.primary_phone_type as u64, buf);
            }
            // Fields 7–8
            if let Some(cm) = &self.contact_method {
                match cm {
                    person::ContactMethod::PhoneNumber(s) => {
                        encode::encode_len_field(7, s.as_bytes(), buf);
                    }
                    person::ContactMethod::FaxNumber(s) => {
                        encode::encode_len_field(8, s.as_bytes(), buf);
                    }
                }
            }
            buf.put_slice(&self._unknown_fields);
        }
    }

    // ── MessageDecode ─────────────────────────────────────────────────────────

    impl<A: Allocator + Clone + Default> MessageDecode for Person<A> {
        fn merge_from<B: ::bytes::Buf>(&mut self, buf: &mut B) -> Result<(), DecodeError> {
            use ::bytes::Buf as _;
            while buf.has_remaining() {
                let (field_number, wire_type) = decode::decode_tag(buf)?;
                match (field_number, wire_type) {
                    // Field 1: string name
                    (1, WireType::Len) => {
                        self.name = decode::decode_string_in(buf, self._alloc.clone())?;
                    }
                    // Field 2: int32 age
                    //
                    // Canonical scalar decode pattern. For other integer types:
                    //   int64   → decode_varint → as i64
                    //   uint32  → decode_varint → as u32
                    //   uint64  → decode_varint (already u64)
                    //   sint32  → decode_varint → unzigzag32
                    //   sint64  → decode_varint → unzigzag64
                    //   bool    → decode_varint → != 0
                    //   fixed32 → buf.get_u32_le()  (WireType::I32)
                    //   fixed64 → buf.get_u64_le()  (WireType::I64)
                    //   float   → f32::from_bits(buf.get_u32_le()) (WireType::I32)
                    //   double  → f64::from_bits(buf.get_u64_le()) (WireType::I64)
                    (2, WireType::Varint) => {
                        self.age = decode::decode_varint(buf)? as i32;
                    }
                    // Field 3: bytes avatar
                    (3, WireType::Len) => {
                        self.avatar = decode::decode_bytes_in(buf, self._alloc.clone())?;
                    }
                    // Field 4: repeated string emails (non-packed; strings cannot be packed)
                    (4, WireType::Len) => {
                        let s = decode::decode_string_in(buf, self._alloc.clone())?;
                        self.emails.push(s);
                    }
                    // Field 5: Address address (nested message, LEN-prefixed)
                    (5, WireType::Len) => {
                        let len = decode::decode_varint(buf)? as usize;
                        if buf.remaining() < len {
                            return Err(DecodeError::TruncatedMessage);
                        }
                        let addr = self.address.get_or_insert_with(|| {
                            ABox::new_in(
                                Address::new_in(self._alloc.clone()),
                                self._alloc.clone(),
                            )
                        });
                        // Use Buf::take to bound the merge to exactly `len` bytes,
                        // matching the proto3 spec for nested message parsing.
                        let leftover = {
                            let mut sub_buf = (&mut *buf).take(len);
                            addr.merge_from(&mut sub_buf)?;
                            sub_buf.remaining()
                        };
                        // Skip any unread bytes within the nested message.
                        if leftover > 0 {
                            buf.advance(leftover);
                        }
                    }
                    // Field 6: PhoneType primary_phone_type (enum, VARINT)
                    (6, WireType::Varint) => {
                        self.primary_phone_type = decode::decode_varint(buf)? as i32;
                    }
                    // Fields 7–8: oneof contact_method
                    // Setting one variant always clears the other (proto3 oneof semantics).
                    (7, WireType::Len) => {
                        let s = decode::decode_string_in(buf, self._alloc.clone())?;
                        self.contact_method = Some(person::ContactMethod::PhoneNumber(s));
                    }
                    (8, WireType::Len) => {
                        let s = decode::decode_string_in(buf, self._alloc.clone())?;
                        self.contact_method = Some(person::ContactMethod::FaxNumber(s));
                    }
                    // Unknown fields: save verbatim for round-trip fidelity.
                    _ => {
                        decode::skip_field_and_save(
                            field_number,
                            wire_type,
                            buf,
                            &mut self._unknown_fields,
                        )?;
                    }
                }
            }
            Ok(())
        }
    }

    // =========================================================================
    // Tests
    //
    // These tests verify the encode ↔ decode round-trip for each field kind
    // and serve as a concrete spec example in runnable form.
    // =========================================================================

    #[cfg(test)]
    mod tests {
        // `MessageDecode` and `MessageEncode` are accessible here without an
        // explicit `use` because `tests` is a child module of `example` and can
        // therefore see `example`'s private `use` aliases for those traits.
        use super::*;

        fn round_trip(p: &Person) -> Person {
            let bytes = p.encode_to_vec();
            Person::decode(::bytes::Bytes::from(bytes)).unwrap()
        }

        #[test]
        fn empty_person_encodes_to_zero_bytes() {
            let p = Person::new();
            assert_eq!(p.encoded_len(), 0, "all defaults must be suppressed");
            assert_eq!(p.encode_to_vec(), &[] as &[u8]);
        }

        #[test]
        fn scalar_fields_round_trip() {
            let mut p = Person::new();
            p.set_name("Alice");
            p.set_age(42);

            let p2 = round_trip(&p);
            assert_eq!(p2.name(), "Alice");
            assert_eq!(p2.age(), 42);
        }

        #[test]
        fn bytes_field_round_trips() {
            let mut p = Person::new();
            p.set_avatar(&[0xDE, 0xAD, 0xBE, 0xEF]);

            let p2 = round_trip(&p);
            assert_eq!(p2.avatar(), &[0xDE, 0xAD, 0xBE, 0xEF]);
        }

        #[test]
        fn repeated_string_field_round_trips() {
            let mut p = Person::new();
            p.push_email("a@example.com");
            p.push_email("b@example.com");

            let p2 = round_trip(&p);
            assert_eq!(p2.emails().len(), 2);
            // Deref coercion: &Box<str, Global> → &str
            let e0: &str = &p2.emails()[0];
            let e1: &str = &p2.emails()[1];
            assert_eq!(e0, "a@example.com");
            assert_eq!(e1, "b@example.com");
        }

        #[test]
        fn nested_message_round_trips() {
            let mut p = Person::new();
            p.address_mut().set_street("123 Main St");
            p.address_mut().set_city("Springfield");

            let p2 = round_trip(&p);
            let addr = p2.address().expect("address should be present");
            assert_eq!(addr.street(), "123 Main St");
            assert_eq!(addr.city(), "Springfield");
        }

        #[test]
        fn enum_field_round_trips() {
            let mut p = Person::new();
            p.set_primary_phone_type(PhoneType::Mobile);

            let p2 = round_trip(&p);
            assert_eq!(p2.primary_phone_type(), Ok(PhoneType::Mobile));
        }

        #[test]
        fn enum_unknown_value_preserved() {
            let mut p = Person::new();
            p.set_primary_phone_type_raw(99); // unknown value

            let p2 = round_trip(&p);
            assert_eq!(p2.primary_phone_type_raw(), 99);
            assert_eq!(p2.primary_phone_type(), Err(99));
        }

        #[test]
        fn oneof_phone_number_round_trips() {
            let mut p = Person::new();
            p.set_phone_number("+1-800-PROTOBUF");

            let p2 = round_trip(&p);
            match p2.contact_method().unwrap() {
                person::ContactMethod::PhoneNumber(s) => {
                    assert_eq!(&**s, "+1-800-PROTOBUF");
                }
                other => panic!("unexpected variant: {other:?}"),
            }
        }

        #[test]
        fn oneof_fax_overrides_phone() {
            let mut p = Person::new();
            p.set_phone_number("555-1234");
            p.set_fax_number("555-5678"); // replaces phone

            let p2 = round_trip(&p);
            // Only the fax number should survive.
            match p2.contact_method().unwrap() {
                person::ContactMethod::FaxNumber(s) => {
                    assert_eq!(&**s, "555-5678");
                }
                other => panic!("unexpected variant: {other:?}"),
            }
        }

        #[test]
        fn unknown_fields_preserved_on_round_trip() {
            // Manually craft a message with an extra field (field 99, varint 42).
            let mut raw = Vec::new();
            // tag = (99 << 3) | 0 = 792
            crate::encode::encode_varint(792, &mut raw);
            crate::encode::encode_varint(42, &mut raw);

            let p = Person::decode(::bytes::Bytes::from(raw)).unwrap();
            assert!(!p.unknown_fields().is_empty(), "unknown field must be saved");

            // Re-encode and decode again: unknown bytes must survive the round-trip.
            let p2 = round_trip(&p);
            assert_eq!(p.unknown_fields(), p2.unknown_fields());
        }
    }
}

// =============================================================================
// proto2 sample
//
// Source `.proto` this module corresponds to:
//
// ```proto
// syntax = "proto2";
// package example;
//
// // Closed enum: unknown numeric values must NOT be stored in the typed field;
// // they are preserved as unknown fields instead.
// enum Difficulty {
//     EASY   = 0;
//     NORMAL = 1;
//     HARD   = 2;
// }
//
// message PlayerConfig {
//     required string     player_id  = 1;               // required → validate()
//     optional int32      score      = 2 [default=100]; // explicit presence + custom default
//     optional bool       active     = 3;               // explicit presence, default false
//     repeated int32      levels     = 4 [packed=false];// explicitly non-packed repeated
//     optional Difficulty difficulty = 5;               // closed enum
// }
// ```
//
// Key differences from the proto3 sample:
//
//   1. `required` fields: stored as `Option<T>` for presence detection; a separate
//      `validate()` method checks all required fields are `Some(_)`.
//
//   2. `optional` scalar fields: stored as `Option<T>`.  The wire default suppression
//      rule changes — a field is omitted only when `None`, **not** when the value
//      equals zero.  Zero is a legitimate explicitly-set value.
//
//   3. Custom defaults: the accessor `score()` returns the proto-declared default
//      (100) when the field is `None`, not the Rust zero-default.
//
//   4. Closed enum: unknown numeric values are saved as unknown fields rather than
//      stored in the typed field.
//
//   5. Non-packed repeated: each element is a separate VARINT record (one tag per
//      element) instead of a single LEN record.
//
// =============================================================================

pub mod example_proto2 {
    use ::allocator_api2::alloc::{Allocator, Global};
    use ::allocator_api2::boxed::Box as ABox;
    use ::allocator_api2::vec::Vec as AVec;
    use crate::decode::{self, MessageDecode};
    use crate::encode::{self, MessageEncode};
    use crate::error::DecodeError;
    use crate::wire_type::WireType;

    // =========================================================================
    // Closed enum: Difficulty
    //
    // The enum definition is identical to the open-enum case in proto3.
    // The difference is purely in how the *decoder* handles unknown values.
    // =========================================================================

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[repr(i32)]
    pub enum Difficulty {
        Easy   = 0,
        Normal = 1,
        Hard   = 2,
    }

    impl TryFrom<i32> for Difficulty {
        type Error = i32;
        fn try_from(v: i32) -> Result<Self, i32> {
            match v {
                0 => Ok(Difficulty::Easy),
                1 => Ok(Difficulty::Normal),
                2 => Ok(Difficulty::Hard),
                other => Err(other),
            }
        }
    }

    impl From<Difficulty> for i32 {
        fn from(v: Difficulty) -> i32 { v as i32 }
    }

    // =========================================================================
    // Message: PlayerConfig (proto2)
    // =========================================================================

    pub struct PlayerConfig<A: Allocator = Global> {
        // Field 1: required string player_id
        // Stored as Option<Box<str, A>> so we can detect absence after decoding.
        player_id: Option<ABox<str, A>>,

        // Field 2: optional int32 score [default = 100]
        // None  → not set on wire; accessor returns custom default (100)
        // Some(v) → explicitly set; accessor returns v, even if v == 0
        score: Option<i32>,

        // Field 3: optional bool active [default = false]
        active: Option<bool>,

        // Field 4: repeated int32 levels [packed = false]
        // Same storage as proto3, but encoded/decoded as individual VARINT records.
        levels: AVec<i32, A>,

        // Field 5: optional Difficulty difficulty (closed enum)
        // Unknown values are rejected and routed to _unknown_fields.
        difficulty: Option<i32>,

        _unknown_fields: AVec<u8, A>,
        _alloc: A,
    }

    // ── Constructors ──────────────────────────────────────────────────────────

    impl<A: Allocator + Clone> PlayerConfig<A> {
        pub fn new_in(alloc: A) -> Self {
            PlayerConfig {
                player_id:  None,
                score:      None,
                active:     None,
                levels:     AVec::new_in(alloc.clone()),
                difficulty: None,
                _unknown_fields: AVec::new_in(alloc.clone()),
                _alloc: alloc,
            }
        }
    }

    impl PlayerConfig<Global> {
        pub fn new() -> Self { Self::new_in(Global) }
    }

    impl<A: Allocator + Clone + Default> Default for PlayerConfig<A> {
        fn default() -> Self { Self::new_in(A::default()) }
    }

    // ── Field accessors ───────────────────────────────────────────────────────

    impl<A: Allocator + Clone> PlayerConfig<A> {
        // ---- player_id (required string) ----

        /// Returns `None` if the field has not been set or decoded yet.
        ///
        /// After decoding, call [`validate`](Self::validate) to ensure a
        /// `required` field was present on the wire.
        pub fn player_id(&self) -> Option<&str> {
            self.player_id.as_deref()
        }

        pub fn set_player_id(&mut self, v: &str) {
            self.player_id = Some(decode::str_to_box_in(v, self._alloc.clone()));
        }

        pub fn clear_player_id(&mut self) {
            self.player_id = None;
        }

        // ---- score (optional int32, default = 100) ----

        /// Returns the field value, or the proto-declared default (100) if unset.
        pub fn score(&self) -> i32 {
            self.score.unwrap_or(100)
        }

        /// Returns `true` if the field was explicitly set (even if the value is 0).
        pub fn has_score(&self) -> bool {
            self.score.is_some()
        }

        pub fn set_score(&mut self, v: i32) {
            self.score = Some(v);
        }

        pub fn clear_score(&mut self) {
            self.score = None;
        }

        // ---- active (optional bool) ----

        pub fn active(&self) -> bool {
            self.active.unwrap_or(false)
        }

        pub fn has_active(&self) -> bool {
            self.active.is_some()
        }

        pub fn set_active(&mut self, v: bool) {
            self.active = Some(v);
        }

        pub fn clear_active(&mut self) {
            self.active = None;
        }

        // ---- levels (repeated int32, non-packed) ----

        pub fn levels(&self) -> &[i32] {
            &self.levels
        }

        pub fn push_level(&mut self, v: i32) {
            self.levels.push(v);
        }

        pub fn clear_levels(&mut self) {
            self.levels.clear();
        }

        // ---- difficulty (optional closed enum) ----

        pub fn difficulty_raw(&self) -> Option<i32> {
            self.difficulty
        }

        pub fn difficulty(&self) -> Option<Result<Difficulty, i32>> {
            self.difficulty.map(Difficulty::try_from)
        }

        pub fn set_difficulty(&mut self, v: Difficulty) {
            self.difficulty = Some(v as i32);
        }

        pub fn clear_difficulty(&mut self) {
            self.difficulty = None;
        }

        // ---- unknown fields ----

        pub fn unknown_fields(&self) -> &[u8] {
            &self._unknown_fields
        }

        // ── Required-field validation ──────────────────────────────────────────

        /// Checks that all `required` fields are present.
        ///
        /// Call this after [`MessageDecode::decode`] or [`MessageDecode::merge_from`]
        /// when decoding a proto2 message.  A missing required field is not an error
        /// at the wire-format level (the decoder still succeeds), but it violates the
        /// schema contract.
        pub fn validate(&self) -> Result<(), DecodeError> {
            if self.player_id.is_none() {
                return Err(DecodeError::MissingRequiredField { field_number: 1 });
            }
            Ok(())
        }

    }

    impl<A: Allocator + Clone + Default> PlayerConfig<A> {
        /// Decodes the message **and** validates required fields in one step.
        ///
        /// Equivalent to calling [`MessageDecode::decode`] followed by
        /// [`validate`](Self::validate).
        pub fn decode_strict<B: ::bytes::Buf>(buf: B) -> Result<Self, DecodeError> {
            let msg = <Self as MessageDecode>::decode(buf)?;
            msg.validate()?;
            Ok(msg)
        }
    }

    // ── MessageEncode ─────────────────────────────────────────────────────────

    impl<A: Allocator + Clone> MessageEncode for PlayerConfig<A> {
        fn encoded_len(&self) -> usize {
            let mut len = 0usize;

            // Field 1: required string player_id
            // Omit if None (field not yet set). On a valid message, validate()
            // would catch this case before encoding.
            if let Some(pid) = &self.player_id {
                len += encode::encoded_len_len_field(1, pid.len());
            }

            // Field 2: optional int32 score
            // Proto2 rule: a field is included if and only if it is explicitly set
            // (Some). Do NOT apply the "omit if zero" rule — zero is a legitimate
            // explicitly-set value.
            if let Some(v) = self.score {
                len += encode::encoded_len_varint_field(2, v as u64);
            }

            // Field 3: optional bool active
            if let Some(v) = self.active {
                len += encode::encoded_len_varint_field(3, v as u64);
            }

            // Field 4: repeated int32 levels [packed = false]
            // Each element is its own VARINT record (tag + value).
            for &v in &self.levels {
                len += encode::encoded_len_varint_field(4, v as u64);
            }

            // Field 5: optional closed enum difficulty
            if let Some(v) = self.difficulty {
                len += encode::encoded_len_varint_field(5, v as u64);
            }

            len += self._unknown_fields.len();
            len
        }

        fn encode_raw<B: ::bytes::BufMut>(&self, buf: &mut B) {
            if let Some(pid) = &self.player_id {
                encode::encode_len_field(1, pid.as_bytes(), buf);
            }
            if let Some(v) = self.score {
                encode::encode_varint_field(2, v as u64, buf);
            }
            if let Some(v) = self.active {
                encode::encode_varint_field(3, v as u64, buf);
            }
            // Non-packed: one VARINT record per element.
            for &v in &self.levels {
                encode::encode_varint_field(4, v as u64, buf);
            }
            if let Some(v) = self.difficulty {
                encode::encode_varint_field(5, v as u64, buf);
            }
            buf.put_slice(&self._unknown_fields);
        }
    }

    // ── MessageDecode ─────────────────────────────────────────────────────────

    impl<A: Allocator + Clone + Default> MessageDecode for PlayerConfig<A> {
        fn merge_from<B: ::bytes::Buf>(&mut self, buf: &mut B) -> Result<(), DecodeError> {
            use ::bytes::Buf as _;
            while buf.has_remaining() {
                let (field_number, wire_type) = decode::decode_tag(buf)?;
                match (field_number, wire_type) {
                    // Field 1: required string player_id
                    (1, WireType::Len) => {
                        self.player_id =
                            Some(decode::decode_string_in(buf, self._alloc.clone())?);
                    }
                    // Field 2: optional int32 score
                    // Store as Some(v) regardless of whether v == 0.
                    (2, WireType::Varint) => {
                        self.score = Some(decode::decode_varint(buf)? as i32);
                    }
                    // Field 3: optional bool active
                    (3, WireType::Varint) => {
                        self.active = Some(decode::decode_varint(buf)? != 0);
                    }
                    // Field 4: repeated int32 levels (non-packed)
                    // Parses one element per record. The decoder must also handle
                    // the packed form (a single LEN record) for forward-compat:
                    (4, WireType::Varint) => {
                        self.levels.push(decode::decode_varint(buf)? as i32);
                    }
                    (4, WireType::Len) => {
                        // Accept packed encoding even though the field declares [packed=false].
                        // The proto spec requires parsers to accept both forms.
                        let payload_len = decode::decode_varint(buf)? as usize;
                        if buf.remaining() < payload_len {
                            return Err(DecodeError::TruncatedMessage);
                        }
                        let leftover = {
                            let mut sub = (&mut *buf).take(payload_len);
                            while sub.has_remaining() {
                                self.levels.push(decode::decode_varint(&mut sub)? as i32);
                            }
                            sub.remaining()
                        };
                        if leftover > 0 { buf.advance(leftover); }
                    }
                    // Field 5: optional Difficulty (CLOSED enum)
                    // Unknown values are treated as unknown fields, not stored in the
                    // typed field. This is the defining difference from open (proto3)
                    // enums.
                    (5, WireType::Varint) => {
                        let raw = decode::decode_varint(buf)? as i32;
                        if Difficulty::try_from(raw).is_ok() {
                            self.difficulty = Some(raw);
                        } else {
                            // Unknown value → preserve as unknown field for round-trip.
                            decode::save_unknown_varint_field(
                                5,
                                raw as u64,
                                &mut self._unknown_fields,
                            );
                        }
                    }
                    _ => {
                        decode::skip_field_and_save(
                            field_number,
                            wire_type,
                            buf,
                            &mut self._unknown_fields,
                        )?;
                    }
                }
            }
            Ok(())
        }
    }

    // =========================================================================
    // Tests
    // =========================================================================

    #[cfg(test)]
    mod tests {
        // `MessageDecode` and `MessageEncode` are accessible here without an
        // explicit `use` because `tests` is a child module of `example_proto2`.
        use super::*;

        fn round_trip(p: &PlayerConfig) -> PlayerConfig {
            let bytes = p.encode_to_vec();
            PlayerConfig::decode(::bytes::Bytes::from(bytes)).unwrap()  // PlayerConfig = PlayerConfig<Global>
        }

        #[test]
        fn required_field_missing_fails_validate() {
            let p = PlayerConfig::new();
            assert_eq!(
                p.validate(),
                Err(crate::error::DecodeError::MissingRequiredField { field_number: 1 }),
            );
        }

        #[test]
        fn required_field_present_passes_validate() {
            let mut p = PlayerConfig::new();
            p.set_player_id("user-42");
            assert!(p.validate().is_ok());
        }

        #[test]
        fn decode_strict_rejects_missing_required() {
            // An empty wire buffer decodes fine but validate() should catch the
            // missing required field.
            let result: Result<PlayerConfig, _> = PlayerConfig::decode_strict(::bytes::Bytes::new());
            assert!(matches!(
                result,
                Err(crate::error::DecodeError::MissingRequiredField { field_number: 1 })
            ));
        }

        #[test]
        fn optional_scalar_zero_is_different_from_unset() {
            // score = 0 is a valid, explicitly-set value and must survive the round-trip.
            let mut p = PlayerConfig::new();
            p.set_player_id("u1");
            p.set_score(0);

            let p2 = round_trip(&p);
            assert!(p2.has_score(), "score should be present");
            assert_eq!(p2.score(), 0, "explicitly-set zero must round-trip");
        }

        #[test]
        fn optional_scalar_unset_returns_custom_default() {
            // score field is absent → accessor returns the proto-declared default 100.
            let p = PlayerConfig::new();
            assert!(!p.has_score());
            assert_eq!(p.score(), 100, "custom default must be returned when unset");
        }

        #[test]
        fn non_packed_repeated_round_trips() {
            let mut p = PlayerConfig::new();
            p.set_player_id("u1");
            p.push_level(10);
            p.push_level(20);
            p.push_level(30);

            let p2 = round_trip(&p);
            assert_eq!(p2.levels(), &[10, 20, 30]);
        }

        #[test]
        fn non_packed_repeated_accepts_packed_encoding() {
            // A newer version of the schema might declare the field packed; the
            // decoder must still accept it.
            let mut raw = Vec::new();
            // Encode levels [10, 20] as packed LEN record for field 4.
            let payload: Vec<u8> = {
                let mut v = Vec::new();
                crate::encode::encode_varint(10, &mut v);
                crate::encode::encode_varint(20, &mut v);
                v
            };
            crate::encode::encode_len_field(4, &payload, &mut raw);
            // Also include the required player_id field.
            crate::encode::encode_len_field(1, b"u1", &mut raw);

            let p: PlayerConfig = PlayerConfig::decode(::bytes::Bytes::from(raw)).unwrap();
            assert_eq!(p.levels(), &[10, 20]);
        }

        #[test]
        fn closed_enum_known_value_round_trips() {
            let mut p = PlayerConfig::new();
            p.set_player_id("u1");
            p.set_difficulty(Difficulty::Hard);

            let p2 = round_trip(&p);
            assert_eq!(p2.difficulty(), Some(Ok(Difficulty::Hard)));
        }

        #[test]
        fn closed_enum_unknown_value_goes_to_unknown_fields() {
            // Encode a raw difficulty value of 99 (not defined in the enum).
            let mut raw = Vec::new();
            crate::encode::encode_len_field(1, b"u1", &mut raw);
            crate::encode::encode_varint_field(5, 99, &mut raw);

            let p: PlayerConfig = PlayerConfig::decode(::bytes::Bytes::from(raw)).unwrap();
            // The typed field must be empty.
            assert!(p.difficulty().is_none(), "unknown value must not be stored in typed field");
            // The unknown bytes must be non-empty (the value was preserved).
            assert!(!p.unknown_fields().is_empty(), "unknown value must be saved");

            // The unknown field must survive a re-encode → decode round-trip.
            let p2 = round_trip(&p);
            assert_eq!(p.unknown_fields(), p2.unknown_fields());
        }
    }
}
