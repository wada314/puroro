//! Hand-written code for Person message.
//!
//! This represents our ideal API for the generated code using the Closed Struct approach.
//!
//! Uses trait-based field operations for type-safe, scalable code generation.

use ::allocator_api2::vec::Vec as AllocVec;
use ::allocator_extras::{Allocator, Global};
use puroro::{
    Message,
    error::Error,
    field_ops::{
        ExplicitOptional, FieldOperations, FieldStorage, ImplicitOptional, MessageFieldWrapper,
        SingularMessage, StringFieldWrapper,
    },
    repeated::{Repeated, VecRepeated},
    shared::SharedFields,
    view::ViewCow,
};

/// Flexible view trait for Person message (not dyn-compatible).
///
/// Code generation note: This trait MUST NOT reference any implementation struct names (e.g., PersonImpl, AddressImpl).
/// Use only trait names and `impl Trait` syntax to maintain abstraction.
pub trait Person: DynPerson {
    // Methods that delegate to DynPerson (default implementations)
    #[inline]
    fn name(&self) -> &str {
        DynPerson::name(self)
    }
    #[inline]
    fn age(&self) -> i32 {
        DynPerson::age(self)
    }
    #[inline]
    fn email(&self) -> Option<&str> {
        DynPerson::email(self)
    }
    #[inline]
    fn score(&self) -> Option<i32> {
        DynPerson::score(self)
    }
    #[inline]
    fn status(&self) -> Result<Status, i32> {
        DynPerson::status(self)
    }
    #[inline]
    fn secondary_status(&self) -> Result<Option<Status>, i32> {
        DynPerson::secondary_status(self)
    }

    // Presence checks (delegating to DynPerson)
    #[inline]
    fn has_name(&self) -> bool {
        DynPerson::has_name(self)
    }

    // Methods with custom implementations (must be implemented)
    // NOTE: Must return `impl Address`, not a concrete struct type
    fn address(&self) -> impl Address + use<'_, Self>;

    // Repeated fields (delegating to DynPerson)
    #[inline]
    fn scores(&self) -> Box<dyn Repeated<i32> + '_> {
        DynPerson::scores(self)
    }
    #[inline]
    fn addresses(&self) -> Box<dyn Repeated<&dyn DynAddress> + '_> {
        DynPerson::addresses(self)
    }
}

/// Dyn-compatible immutable trait for Person message.
///
/// Code generation note: This trait MUST be dyn-compatible. Do not use `impl Trait` here;
/// use `ViewCow` or other dyn-compatible return types for message fields.
pub trait DynPerson {
    // Getters
    fn name(&self) -> &str;
    fn age(&self) -> i32;
    fn email(&self) -> Option<&str>;
    fn score(&self) -> Option<i32>;

    // Enum field getters
    fn status(&self) -> Result<Status, i32>;
    fn secondary_status(&self) -> Result<Option<Status>, i32>;

    // Message field getters
    fn address(&self) -> Option<ViewCow<'_, dyn DynAddress>>; // Message fields always return Option, even for ImplicitOptional

    // Repeated field getters
    fn scores(&self) -> Box<dyn Repeated<i32> + '_>;
    fn addresses(&self) -> Box<dyn Repeated<&dyn DynAddress> + '_>;

    // Presence checks (for optional semantics) - sample: has_name (others follow same pattern)
    fn has_name(&self) -> bool;
}

/// Flexible view append-only trait for Person message (not dyn-compatible).
///
/// Code generation note: MUST NOT reference implementation struct names. All methods must use trait types only.
pub trait PersonAppend: Person + DynPersonAppend {
    // Methods that delegate to DynPersonAppend (default implementations)
    #[inline]
    fn set_name(&mut self, v: &str) {
        DynPersonAppend::set_name(self, v)
    }

    // Methods with custom implementations (must be implemented)
    // NOTE: Must return `impl AddressAppend`, not a concrete struct type
    fn address_mut(&mut self) -> impl AddressAppend + use<'_, Self>;

    // Repeated mutators (delegating to DynPersonAppend)
    #[inline]
    fn push_score(&mut self, v: i32) {
        DynPersonAppend::push_score(self, v)
    }
    #[inline]
    fn clear_scores(&mut self) {
        DynPersonAppend::clear_scores(self)
    }
    #[inline]
    fn push_address(&mut self) -> &mut dyn DynAddressMut {
        DynPersonAppend::push_address(self)
    }
    #[inline]
    fn clear_addresses(&mut self) {
        DynPersonAppend::clear_addresses(self)
    }
}

/// Dyn-compatible append-only trait for Person message.
///
/// Code generation note: This trait MUST be dyn-compatible. Do not use `impl Trait` here.
pub trait DynPersonAppend: DynPerson {
    // Setters - sample: set_name (others follow same pattern: field.set(&mut self._shared, v) or field.set(&mut self._shared, v.to_wire()))
    fn set_name(&mut self, v: &str);

    // Repeated mutators
    fn push_score(&mut self, v: i32);
    fn clear_scores(&mut self);
    /// Appends a new default address and returns a mutable dyn view to build it.
    fn push_address(&mut self) -> &mut dyn DynAddressMut;
    fn clear_addresses(&mut self);
}

/// Flexible view fully mutable trait for Person message (not dyn-compatible).
///
/// Code generation note: MUST NOT reference implementation struct names. All methods must use trait types only.
pub trait PersonMut: PersonAppend + DynPersonMut {
    // Clear methods (delegating to DynPersonMut)
    #[inline]
    fn clear_name(&mut self) {
        DynPersonMut::clear_name(self)
    }

    // Methods with custom implementations (must be implemented)
    // NOTE: Must return `impl AddressMut`, not a concrete struct type
    fn address_mut(&mut self) -> impl AddressMut + use<'_, Self>;
}

/// Dyn-compatible fully mutable trait for Person message.
///
/// Code generation note: This trait MUST be dyn-compatible. Do not use `impl Trait` here.
pub trait DynPersonMut: DynPersonAppend {
    // Clear methods - sample: clear_name (others follow same pattern: field.clear(&mut self._shared))
    fn clear_name(&mut self);

    // Builder-style methods for nested message construction
    fn address_mut(&mut self) -> &mut dyn DynAddressMut;
}

// ============================================================================
// Status Enum
// ============================================================================

/// Status enum implementation
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum Status {
    /// Default value (must be 0)
    Unspecified = 0,
    Active = 1,
    Inactive = 2,
    Pending = 3,
}

impl Status {
    /// Convert from wire format (i32) - known values only
    pub fn from_wire(value: i32) -> Result<Self, i32> {
        match value {
            0 => Ok(Self::Unspecified),
            1 => Ok(Self::Active),
            2 => Ok(Self::Inactive),
            3 => Ok(Self::Pending),
            unknown => Err(unknown), // Unknown values are errors
        }
    }

    /// Convert to wire format (i32)
    pub fn to_wire(self) -> i32 {
        self as i32
    }
}

impl TryFrom<i32> for Status {
    type Error = i32;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::from_wire(value)
    }
}

// ============================================================================
// Address Traits and Implementation
// ============================================================================

/// Flexible view trait for Address message (not dyn-compatible).
///
/// Code generation note: MUST NOT reference implementation struct names. Trait-only abstraction.
pub trait Address: DynAddress {}

// Blanket implementation for references
impl<T: Address> Address for &T {}
impl<T: Address> Address for &mut T {}
impl<T: AddressAppend> AddressAppend for &mut T {}
impl<T: AddressMut> AddressMut for &mut T {}

// Blanket implementation for Option
impl<T: DynAddress> DynAddress for Option<T> {
    fn street(&self) -> &str {
        self.as_ref().map(|v| v.street()).unwrap_or("")
    }

    fn city(&self) -> &str {
        self.as_ref().map(|v| v.city()).unwrap_or("")
    }

    fn zip_code(&self) -> i32 {
        self.as_ref().map(|v| v.zip_code()).unwrap_or(0)
    }
}

impl<T: Address> Address for Option<T> {}

// Blanket implementation for Box
impl<T: Address> Address for Box<T> {}
impl<T: AddressAppend> AddressAppend for Box<T> {}
impl<T: AddressMut> AddressMut for Box<T> {}

// Blanket implementation for Option
impl<T: AddressAppend> AddressAppend for Option<T> {}
impl<T: AddressMut> AddressMut for Option<T> {}

/// Flexible view append-only trait for Address message (not dyn-compatible).
///
/// Code generation note: MUST NOT reference implementation struct names. Trait-only abstraction.
pub trait AddressAppend: Address + DynAddressAppend {}

/// Flexible view fully mutable trait for Address message (not dyn-compatible).
///
/// Code generation note: MUST NOT reference implementation struct names. Trait-only abstraction.
pub trait AddressMut: AddressAppend + DynAddressMut {}

/// Dyn-compatible immutable trait for Address message.
///
/// Code generation note: This trait MUST be dyn-compatible. Do not use `impl Trait` here.
pub trait DynAddress {
    fn street(&self) -> &str;
    fn city(&self) -> &str;
    fn zip_code(&self) -> i32;
}

// Blanket implementation for references
impl<T: DynAddress> DynAddress for &T {
    fn street(&self) -> &str {
        (*self).street()
    }

    fn city(&self) -> &str {
        (*self).city()
    }

    fn zip_code(&self) -> i32 {
        (*self).zip_code()
    }
}

impl<T: DynAddress> DynAddress for &mut T {
    fn street(&self) -> &str {
        (**self).street()
    }

    fn city(&self) -> &str {
        (**self).city()
    }

    fn zip_code(&self) -> i32 {
        (**self).zip_code()
    }
}

// Blanket implementation for Box
impl<T: DynAddress> DynAddress for Box<T> {
    fn street(&self) -> &str {
        (**self).street()
    }

    fn city(&self) -> &str {
        (**self).city()
    }

    fn zip_code(&self) -> i32 {
        (**self).zip_code()
    }
}

/// Dyn-compatible append-only trait for Address message.
///
/// Code generation note: This trait MUST be dyn-compatible. Do not use `impl Trait` here.
pub trait DynAddressAppend: DynAddress {
    // Setters - sample: set_street (others follow same pattern: field.set(&mut self._shared, v))
    fn set_street(&mut self, v: &str);
}

// Blanket implementation for references
impl<T: DynAddressAppend> DynAddressAppend for &mut T {
    fn set_street(&mut self, v: &str) {
        (**self).set_street(v)
    }
}

// Blanket implementation for Box
impl<T: DynAddressAppend> DynAddressAppend for Box<T> {
    fn set_street(&mut self, v: &str) {
        (**self).set_street(v)
    }
}

// Blanket implementation for Option
impl<T: DynAddressAppend> DynAddressAppend for Option<T> {
    fn set_street(&mut self, v: &str) {
        if let Some(t) = self {
            t.set_street(v);
        }
    }
}

/// Dyn-compatible fully mutable trait for Address message.
///
/// Code generation note: This trait MUST be dyn-compatible. Do not use `impl Trait` here.
pub trait DynAddressMut: DynAddressAppend {
    // Clear methods - sample: clear_street (others follow same pattern: field.clear(&mut self._shared))
    fn clear_street(&mut self);
}

// Blanket implementation for references
impl<T: DynAddressMut> DynAddressMut for &mut T {
    fn clear_street(&mut self) {
        (**self).clear_street()
    }
}

// Blanket implementation for Box
impl<T: DynAddressMut> DynAddressMut for Box<T> {
    fn clear_street(&mut self) {
        (**self).clear_street()
    }
}

// Blanket implementation for Option
impl<T: DynAddressMut> DynAddressMut for Option<T> {
    fn clear_street(&mut self) {
        if let Some(t) = self {
            t.clear_street();
        }
    }
}

/// Address message implementation
#[derive(Debug, Clone)]
pub struct AddressImpl<A: Allocator = Global> {
    street: FieldStorage<StringFieldWrapper<A>, ImplicitOptional, 1, 1, A>,
    city: FieldStorage<StringFieldWrapper<A>, ImplicitOptional, 2, 1, A>,
    zip_code: FieldStorage<i32, ImplicitOptional, 3, 1, A>,
    _shared: SharedFields<1, A>,
}

impl<A> AddressImpl<A>
where
    A: Allocator + Clone,
{
    pub fn new_in(alloc: A) -> Self {
        Self {
            street: FieldStorage::new(StringFieldWrapper::new_in(alloc.clone())),
            city: FieldStorage::new(StringFieldWrapper::new_in(alloc.clone())),
            zip_code: Default::default(),
            _shared: SharedFields::new_in(alloc),
        }
    }
}

impl AddressImpl<Global> {
    pub fn new() -> Self {
        Self::new_in(Global)
    }
}

impl<A> Default for AddressImpl<A>
where
    A: Allocator + Clone + Default,
{
    fn default() -> Self {
        Self::new_in(A::default())
    }
}

impl<A> PartialEq for AddressImpl<A>
where
    A: Allocator,
{
    fn eq(&self, other: &Self) -> bool {
        self.street == other.street
            && self.city == other.city
            && self.zip_code == other.zip_code
            && self._shared == other._shared
    }
}

impl<A> Eq for AddressImpl<A> where A: Allocator {}

impl<A: Allocator + Clone> Address for AddressImpl<A> {}

impl<A: Allocator + Clone> DynAddress for AddressImpl<A> {
    fn street(&self) -> &str {
        self.street.get(&self._shared)
    }
    fn city(&self) -> &str {
        self.city.get(&self._shared)
    }
    fn zip_code(&self) -> i32 {
        self.zip_code.get(&self._shared)
    }
}

impl<A: Allocator + Clone> AddressAppend for AddressImpl<A> {}

impl<A: Allocator + Clone> DynAddressAppend for AddressImpl<A> {
    // set_* methods - sample implementation (others follow same pattern: field.set(&mut self._shared, v))
    fn set_street(&mut self, v: &str) {
        self.street.set(&mut self._shared, v)
    }
}

impl<A: Allocator + Clone> AddressMut for AddressImpl<A> {}

impl<A: Allocator + Clone> DynAddressMut for AddressImpl<A> {
    // clear_* methods - sample implementation (others follow same pattern: field.clear(&mut self._shared))
    fn clear_street(&mut self) {
        self.street.clear(&mut self._shared)
    }
}

impl<A: Allocator + Clone> Message for AddressImpl<A> {
    fn parse_from_bytes_in<B>(_bytes: &[u8], _alloc: B) -> Result<Self, Error>
    where
        B: Allocator + Clone,
    {
        todo!("Parsing not yet implemented")
    }

    fn write_to_bytes_in<B>(&self, _alloc: B) -> Result<AllocVec<u8, B>, Error>
    where
        B: Allocator + Clone,
    {
        todo!("Serialization not yet implemented")
    }

    fn compute_size(&self) -> usize {
        todo!("Size computation not yet implemented")
    }
}

// ============================================================================
// PersonImpl Structure
// ============================================================================

/// Standard implementation of Person message.
///
/// Uses trait-based field operations for type-safe code generation.
/// Fields are ordered by size (descending) to minimize padding.
/// Memory layout optimized for performance.
#[derive(Debug, Clone)]
pub struct PersonImpl<A: Allocator = Global> {
    // Shared fields: presence tracking, etc.
    // For 3 explicit optional fields (email, score, secondary_status): ⌈3/8⌉ = 1 byte (stack-allocated)
    // Message fields use heap allocation with Option<Box<M>> for presence tracking
    _shared: SharedFields<1, A>,

    // Exclusive fields ordered by size (descending)
    // String: 24 bytes (3 words on 64-bit)
    // Direct field types with explicit parameters for clarity
    // Format: FieldStorage<T, L, FIELD_NUMBER, SHARED_BYTES_LEN>
    name: FieldStorage<StringFieldWrapper<A>, ImplicitOptional, 1, 1, A>, // Field 1, implicit presence, 1 byte shared
    email: FieldStorage<StringFieldWrapper<A>, ExplicitOptional<0>, 3, 1, A>, // Field 3, explicit presence, bit 0, 1 byte shared

    // Message fields: use heap allocation with Option<Box<M>> for presence tracking
    // No presence bits needed - Option<Box<M>> handles presence directly
    address: FieldStorage<MessageFieldWrapper<AddressImpl<A>, A>, SingularMessage, 6, 1, A>, // Field 6, heap-allocated presence

    // Enum fields: stored as i32
    status: FieldStorage<i32, ImplicitOptional, 4, 1, A>, // Field 4, implicit presence, 1 byte shared
    secondary_status: FieldStorage<i32, ExplicitOptional<2>, 8, 1, A>, // Field 8, explicit presence, bit 2, 1 byte shared

    // Scalar fields: 4 bytes
    age: FieldStorage<i32, ImplicitOptional, 2, 1, A>, // Field 2, implicit presence, 1 byte shared
    score: FieldStorage<i32, ExplicitOptional<1>, 5, 1, A>, // Field 5, explicit presence, bit 1, 1 byte shared

    // Repeated fields
    scores: FieldStorage<AllocVec<i32, A>, puroro::field_ops::Repeated, 10, 1, A>,
    addresses: FieldStorage<AllocVec<AddressImpl<A>, A>, puroro::field_ops::Repeated, 9, 1, A>,
}

impl<A> PersonImpl<A>
where
    A: Allocator + Clone,
{
    /// Creates a new Person with default values using the provided allocator.
    pub fn new_in(alloc: A) -> Self {
        let shared = SharedFields::new_in(alloc.clone());
        Self {
            name: FieldStorage::default_in(alloc.clone()),
            email: FieldStorage::default_in(alloc.clone()),
            address: FieldStorage::default_in(alloc.clone()),
            status: Default::default(),
            secondary_status: Default::default(),
            _shared: shared,
            age: Default::default(),
            score: Default::default(),
            scores: FieldStorage::new_in(alloc.clone(), AllocVec::new_in),
            addresses: FieldStorage::new_in(alloc.clone(), AllocVec::new_in),
        }
    }
}

impl PersonImpl<Global> {
    /// Creates a new Person with default values using the global allocator.
    pub fn new() -> Self {
        Self::new_in(Global)
    }
}

impl<A> Default for PersonImpl<A>
where
    A: Allocator + Clone + Default,
{
    fn default() -> Self {
        Self::new_in(A::default())
    }
}

impl<A> PartialEq for PersonImpl<A>
where
    A: Allocator,
{
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.email == other.email
            && self.address == other.address
            && self.status == other.status
            && self.secondary_status == other.secondary_status
            && self._shared == other._shared
            && self.age == other.age
            && self.score == other.score
    }
}

impl<A> Eq for PersonImpl<A> where A: Allocator {}

impl<A: Allocator + Clone> Person for PersonImpl<A> {
    fn address(&self) -> impl Address + use<'_, A> {
        self.address.get(&self._shared)
    }
    // All other methods use default implementations from the trait definition
}

impl<A: Allocator + Clone> PersonAppend for PersonImpl<A> {
    fn address_mut(&mut self) -> impl AddressAppend + use<'_, A> {
        self.address.data.as_mut()
    }
    // All other methods use default implementations from the trait definition
}

impl<A: Allocator + Clone> PersonMut for PersonImpl<A> {
    fn address_mut(&mut self) -> impl AddressMut + use<'_, A> {
        self.address.data.as_mut()
    }
    // All other methods use default implementations from the trait definition
}

impl<A: Allocator + Clone> DynPerson for PersonImpl<A> {
    fn name(&self) -> &str {
        self.name.get(&self._shared)
    }
    fn age(&self) -> i32 {
        self.age.get(&self._shared)
    }
    fn email(&self) -> Option<&str> {
        self.email.get(&self._shared)
    }
    fn score(&self) -> Option<i32> {
        self.score.get(&self._shared)
    }
    fn status(&self) -> Result<Status, i32> {
        Status::from_wire(self.status.get(&self._shared))
    }

    fn secondary_status(&self) -> Result<Option<Status>, i32> {
        if self.secondary_status.is_present(&self._shared) {
            match Status::from_wire(self.secondary_status.get(&self._shared).unwrap_or(0)) {
                Ok(status) => Ok(Some(status)),
                Err(unknown) => Err(unknown),
            }
        } else {
            Ok(None)
        }
    }

    fn address(&self) -> Option<ViewCow<'_, dyn DynAddress>> {
        self.address
            .get(&self._shared)
            .map(|address| ViewCow::Borrowed(address as &dyn DynAddress))
    }

    fn scores(&self) -> Box<dyn Repeated<i32> + '_> {
        Box::new(VecRepeated::new(&self.scores.data))
    }

    fn addresses(&self) -> Box<dyn Repeated<&dyn DynAddress> + '_> {
        struct AddressesRepeated<'a, A: Allocator> {
            vec: &'a AllocVec<AddressImpl<A>, A>,
        }
        impl<'a, A: Allocator + Clone + 'a> Repeated<&'a dyn DynAddress> for AddressesRepeated<'a, A> {
            fn len(&self) -> usize {
                self.vec.len()
            }
            fn is_empty(&self) -> bool {
                self.vec.is_empty()
            }
            fn get(&self, index: usize) -> Option<&'a dyn DynAddress> {
                self.vec.get(index).map(|m| m as &dyn DynAddress)
            }
            fn iter_box(&self) -> Box<dyn Iterator<Item = &'a dyn DynAddress> + '_> {
                Box::new(self.vec.iter().map(|m| m as &dyn DynAddress))
            }
        }
        Box::new(AddressesRepeated { vec: &self.addresses.data })
    }

    // has_* methods - sample implementation (others follow same pattern: field.is_present(&self._shared))
    fn has_name(&self) -> bool {
        self.name.is_present(&self._shared)
    }
}

impl<A: Allocator + Clone> DynPersonAppend for PersonImpl<A> {
    // set_* methods - sample implementation (others follow same pattern: field.set(&mut self._shared, v) or field.set(&mut self._shared, v.to_wire()))
    fn set_name(&mut self, v: &str) {
        self.name.set(&mut self._shared, v)
    }

    fn push_score(&mut self, v: i32) {
        self.scores.data.push(v)
    }

    fn clear_scores(&mut self) {
        self.scores.data.clear()
    }

    fn push_address(&mut self) -> &mut dyn DynAddressMut {
        let alloc = self._shared.allocator().clone();
        self.addresses.data.push(AddressImpl::new_in(alloc));
        // Safe to unwrap: just pushed one
        let last_index = self.addresses.data.len() - 1;
        &mut self.addresses.data[last_index] as &mut dyn DynAddressMut
    }

    fn clear_addresses(&mut self) {
        self.addresses.data.clear()
    }
}

impl<A: Allocator + Clone> DynPersonMut for PersonImpl<A> {
    // clear_* methods - sample implementation (others follow same pattern: field.clear(&mut self._shared))
    fn clear_name(&mut self) {
        self.name.clear(&mut self._shared)
    }

    fn address_mut(&mut self) -> &mut dyn DynAddressMut {
        let alloc = self._shared.allocator().clone();
        self.address
            .data
            .get_or_insert_with(|| AddressImpl::new_in(alloc)) as &mut dyn DynAddressMut
    }
}

impl<A: Allocator + Clone> Message for PersonImpl<A> {
    fn parse_from_bytes_in<B>(_bytes: &[u8], _alloc: B) -> Result<Self, Error>
    where
        B: Allocator + Clone,
    {
        // TODO: Implement actual parsing
        todo!("Parsing not yet implemented")
    }

    fn write_to_bytes_in<B>(&self, _alloc: B) -> Result<AllocVec<u8, B>, Error>
    where
        B: Allocator + Clone,
    {
        // TODO: Implement actual serialization
        todo!("Serialization not yet implemented")
    }

    fn compute_size(&self) -> usize {
        // TODO: Implement actual size computation
        todo!("Size computation not yet implemented")
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::{
        AddressImpl, DynAddress, DynAddressAppend, DynPersonMut, MessageFieldWrapper, Person,
        PersonAppend, PersonImpl, PersonMut, Status, StringFieldWrapper,
    };

    #[test]
    fn test_message_fields() {
        let mut person = PersonImpl::new();

        // Test setting message fields via mutable builder
        // Initialize first using DynPersonMut (destructive, but needed for testing)
        {
            DynPersonMut::address_mut(&mut person);
        }
        {
            let mut address_impl = PersonMut::address_mut(&mut person);
            // address_impl is Option<&mut AddressImpl> which implements AddressMut
            // For Option<T>, None case does nothing, so initialization is required
            address_impl.set_street("123 Main St");
        }

        // Test getting message fields
        {
            let address_impl = Person::address(&person);
            // address_impl is Option<&AddressImpl> which implements Address
            // For Option<T>, None case returns default values
            assert_eq!(address_impl.street(), "123 Main St");
        }

        // Test builder pattern
        {
            let mut address_impl = PersonMut::address_mut(&mut person);
            address_impl.set_street("456 Oak Ave");
        }
        {
            let address_impl = Person::address(&person);
            assert_eq!(address_impl.street(), "456 Oak Ave");
        }
    }

    #[test]
    fn test_enum_fields() {
        let mut person = PersonImpl::new();

        // Test setting enum fields (sample: only set_name is available in DynPersonAppend)
        PersonAppend::set_name(&mut person, "Test");

        // Test getting enum fields
        match Person::status(&person) {
            Ok(Status::Active) => println!("Status is Active"),
            Ok(status) => println!("Status is {:?}", status),
            Err(unknown) => println!("Unknown status: {}", unknown),
        }

        match Person::secondary_status(&person) {
            Ok(Some(Status::Pending)) => println!("Secondary status is Pending"),
            Ok(Some(status)) => println!("Secondary status is {:?}", status),
            Ok(None) => println!("No secondary status"),
            Err(unknown) => println!("Unknown secondary status: {}", unknown),
        }

        // Test presence checking (sample: only has_name is available in DynPerson)
        assert!(Person::has_name(&person));

        // Test clearing enum fields (sample: only clear_name is available in DynPersonMut)
        PersonMut::clear_name(&mut person);
        assert!(!Person::has_name(&person));
    }

    #[test]
    fn test_wrapper_types() {
        // Test StringFieldWrapper wrapper
        let mut string_field = StringFieldWrapper::new();
        string_field.as_mut_string().push_str("Hello");
        assert_eq!(string_field.as_str(), "Hello");

        // Test MessageFieldWrapper wrapper
        let mut message_field = MessageFieldWrapper::new();
        let address = message_field.get_or_insert_with(AddressImpl::new);
        assert_eq!(address.street(), "");
    }

    #[test]
    fn test_repeated_scalars_and_messages() {
        let mut person = PersonImpl::new();

        // push and read scores
        PersonAppend::push_score(&mut person, 10);
        PersonAppend::push_score(&mut person, 20);
        {
            let rep = Person::scores(&person);
            assert_eq!(rep.len(), 2);
            let collected: Vec<i32> = rep.iter_box().collect();
            assert_eq!(collected, vec![10, 20]);
            assert_eq!(rep.get(1), Some(20));
        }
        PersonAppend::clear_scores(&mut person);
        assert!(Person::scores(&person).is_empty());

        // push and read addresses
        {
            let addr_mut = PersonAppend::push_address(&mut person);
            addr_mut.set_street("First St");
        }
        {
            let addr_mut = PersonAppend::push_address(&mut person);
            addr_mut.set_street("Second Ave");
        }
        {
            let rep = Person::addresses(&person);
            assert_eq!(rep.len(), 2);
            let streets: Vec<String> = rep.iter_box().map(|m| m.street().to_string()).collect();
            assert_eq!(
                streets,
                vec!["First St".to_string(), "Second Ave".to_string()]
            );
            assert!(rep.get(0).is_some());
        }
        PersonAppend::clear_addresses(&mut person);
        assert!(Person::addresses(&person).is_empty());
    }
}

// Example: Future lazy deserialization implementation
// This would deserialize fields on-demand when accessed
/*
pub struct PersonLazy {
    raw_bytes: Vec<u8>,
    // Cache for already-deserialized fields
    cached_name: Option<String>,
    cached_age: Option<i32>,
    cached_email: Option<String>,
}

impl PersonTry for PersonLazy {
    fn try_name(&self) -> Result<&str, Error> {
        // Deserialize on first access, then cache
        // May fail if raw_bytes are corrupted
        todo!("Lazy deserialization")
    }

    fn try_age(&self) -> Result<i32, Error> {
        // Deserialize on first access
        // May fail if raw_bytes are corrupted
        todo!("Lazy deserialization")
    }

    // ... etc
}
*/
