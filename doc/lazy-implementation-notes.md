# Lazy Implementation Notes for Protocol Buffers

This document captures important properties and requirements for implementing lazy deserialization of Protocol Buffer messages.

## Key Property: Field Number Overwriting and Concatenation

In Protocol Buffers wire format, multiple fields with the same field number can appear in a message. How we handle this depends on the field type:

### 1. Scalar Non-Message Fields

**Important Property**: To retrieve the correct value of a scalar non-message field, we **MUST** parse all fields in the owner message until the last field, because later fields with the same field number can **overwrite** earlier ones.

**Example**:
```protobuf
message Person {
  int32 age = 1;
}
```

If the wire format contains:
```
Field 1: age = 25
Field 2: name = "Alice"
Field 1: age = 30  // This overwrites the previous age
```

To correctly retrieve `age`, we must parse until the end to get `30`, not stop at the first occurrence and get `25`.

**Implementation Implication**: When deserializing a scalar field lazily, we cannot stop at the first occurrence. We must scan the entire message to find the last occurrence of that field number.

### 2. Scalar Message Fields

**Important Property**: For a scalar message field, we **MUST** parse all fields with the same field number at the **same nesting level**, because the scalar message field is **concatenated** from all fields with the same field number.

However, when accessing **nested fields within that message**, we can still apply lazy parsing rules based on what is actually needed.

**Example**:
```protobuf
message Person {
  Address address = 1;  // scalar message field
}

message Address {
  string street = 1;
  string city = 2;
  repeated string streets = 3;
}
```

If the wire format contains:
```
Field 1 (Person.address.street): "Main St"
Field 1 (Person.address.city): "New York"
Field 1 (Person.address.streets[0]): "Oak Ave"
```

To correctly reconstruct the complete `address` message, we need to collect all fields with field number 1 at the Person level.

**However**, if we only need the first item of `address.streets` (a nested repeated field), we don't need to parse the entire `address` message. We can:
1. Parse all occurrences of field 1 at the Person level (to get the complete address message structure)
2. Within the address message, stop parsing the repeated field `streets` after getting the first item

**Implementation Implication**: 
- At each nesting level, we must collect all occurrences of a scalar message field's field number
- But within that collected message, we can apply lazy parsing for nested fields (especially repeated fields)
- The key is understanding the **scope** of where we need to be exhaustive vs. where we can be lazy

### 3. Repeated Fields

**Important Property**: Unlike scalar fields, repeated fields **DO NOT** require parsing all fields unless we need all values. We can stop parsing as soon as we have the required number of items.

**Rationale**: 
- Later fields with the same field number are **additional items** in the repeated field list
- They **cannot affect** preceding items in the list
- Each occurrence of the field number is a separate list item

**Example**:
```protobuf
message Person {
  repeated int32 scores = 1;
}
```

If the wire format contains:
```
Field 1: scores[0] = 10
Field 1: scores[1] = 20
Field 1: scores[2] = 30
```

If we only need the first item (e.g., when using an iterator and requesting the first element), we can stop after finding the first occurrence and return `10`. The later occurrences (`20`, `30`) are just additional items that will be parsed only when requested.

**Implementation Implication**: 
- For repeated fields, we can implement **lazy iteration**: parse items on-demand as they are requested
- If the API only requires the first item, we can stop parsing after finding it
- If the API requires all items (e.g., `len()`, `iter_box()` that collects all), we need to parse all occurrences

**Performance Benefit**: This enables efficient lazy loading where we only parse what is actually needed, rather than parsing the entire message upfront.

## Summary Table

| Field Type | Parse Until Last? | Scope | Reason |
|------------|-------------------|-------|--------|
| Scalar non-message | **Yes** | At same nesting level | Later fields overwrite earlier ones |
| Scalar message | **Yes** | At same nesting level | All occurrences are concatenated/merged |
| Repeated | **No** (unless all items needed) | At same nesting level | Each occurrence is a separate list item |

**Key Point**: The "parse until last" requirement applies **at each nesting level independently**. Within a nested message, we can still apply lazy parsing for repeated fields or other nested structures based on what is actually accessed.

## Implementation Guidelines

When implementing lazy deserialization:

1. **Scalar non-message fields**: Always parse all fields **at the same nesting level** before returning the value (later fields overwrite earlier ones)

2. **Scalar message fields**: 
   - Collect and merge all occurrences of the field number **at the same nesting level**
   - However, when accessing nested fields within that message, apply lazy parsing rules recursively
   - Example: If accessing `person.address.streets[0]` (nested repeated field), you still need to collect all `address` fields at Person level, but can stop parsing `streets` after the first item

3. **Repeated fields**: 
   - Implement iterator-like parsing that stops when enough items are found **at the current nesting level**
   - Only parse all items when methods like `len()` or full iteration are called
   - Cache parsed items to avoid re-parsing
   - When nested inside a message, can be lazy even if the parent message requires full parsing

**Nested Example**:
```
Top-level: Person { address: Address }
  Address (scalar message) - must collect all field 1 occurrences at Person level
    streets (repeated field) - can stop after first item if only first item needed
```

The parsing strategy is **recursive** - at each level, apply the appropriate rule based on the field type and what is actually being accessed.

## Code References

- `PersonLazyImpl` in `sandbox/src/generated/person.rs`
- `AddressLazyImpl` in `sandbox/src/generated/address.rs`
- Deserialization methods: `deserialize_field_*` functions

