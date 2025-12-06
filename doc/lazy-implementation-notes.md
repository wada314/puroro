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

**Important Property**: Similar to scalar non-message fields, we **MUST** parse all fields with the same field number, because the scalar message field is **concatenated** from all fields with the same field number.

**Example**:
```protobuf
message Person {
  Address address = 1;
}
```

If the wire format contains:
```
Field 1: address.street = "Main St"
Field 1: address.city = "New York"
```

The final `address` message contains both `street` and `city` fields. We need to merge all occurrences of field 1 to reconstruct the complete message.

**Implementation Implication**: When deserializing a scalar message field lazily, we must collect all occurrences of that field number and merge them into a single message.

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

| Field Type | Parse Until Last? | Reason |
|------------|-------------------|--------|
| Scalar non-message | **Yes** | Later fields overwrite earlier ones |
| Scalar message | **Yes** | All occurrences are concatenated/merged |
| Repeated | **No** (unless all items needed) | Each occurrence is a separate list item |

## Implementation Guidelines

When implementing lazy deserialization:

1. **Scalar fields**: Always parse all fields in the message before returning the value
2. **Message fields**: Collect and merge all occurrences of the field number
3. **Repeated fields**: 
   - Implement iterator-like parsing that stops when enough items are found
   - Only parse all items when methods like `len()` or full iteration are called
   - Cache parsed items to avoid re-parsing

## Code References

- `PersonLazyImpl` in `sandbox/src/generated/person.rs`
- `AddressLazyImpl` in `sandbox/src/generated/address.rs`
- Deserialization methods: `deserialize_field_*` functions

