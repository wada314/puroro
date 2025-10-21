# FieldContext: Do We Really Need It?

## Current Implementation (with FieldContext)

```rust
// Library code
pub struct FieldContext<'a, T, O> {
    pub has_bits: &'a mut BitSlice<T, O>,
    pub bit_index: usize,
}

pub fn set_string<T, O>(mut ctx: FieldContext<T, O>, storage: &mut String, value: &str) {
    value.clone_into(storage);
    ctx.mark_set();
}

// Generated code
impl PersonAppend for PersonImpl {
    fn set_name(&mut self, v: &str) {
        let ctx = FieldContext::new(self._shared.has_bits_mut(), IDX_NAME);
        field::set_string(ctx, &mut self.name, v);
    }
}
```

**Lines of code:** 2 lines in generated method

## Alternative: Direct Parameters (without FieldContext)

```rust
// Library code
pub fn set_string<T, O>(
    has_bits: &mut BitSlice<T, O>,
    bit_index: usize,
    storage: &mut String,
    value: &str,
) {
    value.clone_into(storage);
    has_bits.set(bit_index, true);
}

// Generated code
impl PersonAppend for PersonImpl {
    fn set_name(&mut self, v: &str) {
        field::set_string(self._shared.has_bits_mut(), IDX_NAME, &mut self.name, v);
    }
}
```

**Lines of code:** 1 line in generated method (simpler!)

## Comparison

### Generated Code Readability

**With FieldContext:**
```rust
let ctx = FieldContext::new(self._shared.has_bits_mut(), IDX_NAME);
field::set_string(ctx, &mut self.name, v);
```
- 2 lines
- Clear separation: context creation vs operation
- More verbose

**Without FieldContext:**
```rust
field::set_string(self._shared.has_bits_mut(), IDX_NAME, &mut self.name, v);
```
- 1 line
- More direct
- Slightly longer line

### Function Signatures

**With FieldContext:**
```rust
pub fn set_string<T, O>(
    mut ctx: FieldContext<T, O>,
    storage: &mut String,
    value: &str,
)
```
- Shorter parameter list
- Context grouped together

**Without FieldContext:**
```rust
pub fn set_string<T, O>(
    has_bits: &mut BitSlice<T, O>,
    bit_index: usize,
    storage: &mut String,
    value: &str,
)
```
- More parameters (but clearer what each is)
- No wrapper type needed

### Future Extensibility

**With FieldContext (easier to extend):**
```rust
// Adding bool_bits support
pub struct FieldContext<'a, T, O> {
    pub has_bits: &'a mut BitSlice<T, O>,
    pub bool_bits: &'a mut BitSlice<T, O>,  // ← Add here
    pub bit_index: usize,
}

// Function signatures stay the same!
pub fn set_string<T, O>(mut ctx: FieldContext<T, O>, storage: &mut String, value: &str);
```

**Without FieldContext (breaks all signatures):**
```rust
// Adding bool_bits support
pub fn set_string<T, O>(
    has_bits: &mut BitSlice<T, O>,
    bool_bits: &mut BitSlice<T, O>,  // ← Every function signature changes!
    bit_index: usize,
    storage: &mut String,
    value: &str,
)

// All generated code must be regenerated
```

### Code Reuse: FieldContext Methods

**Current FieldContext methods:**
```rust
impl FieldContext {
    pub fn mark_set(&mut self)
    pub fn mark_unset(&mut self)
    pub fn is_set(&self) -> bool
}
```

**Are these actually used?** Let's check:
```rust
// In field operations:
pub fn set_string(...) {
    value.clone_into(storage);
    ctx.mark_set();  // ← Used here
}

pub fn clear_string(...) {
    storage.clear();
    ctx.mark_unset();  // ← Used here
}
```

Yes, they're used! Without FieldContext:
```rust
pub fn set_string(...) {
    value.clone_into(storage);
    has_bits.set(bit_index, true);  // Direct bit manipulation
}
```

This is actually fine and equally clear.

## Pros and Cons

### With FieldContext

**Pros:**
- ✅ Future extensibility (add fields without breaking signatures)
- ✅ Groups related parameters together
- ✅ Can add helper methods (mark_set, mark_unset, is_set)
- ✅ Type-safe wrapper for shared state

**Cons:**
- ❌ Extra type to maintain
- ❌ Slightly more verbose generated code (2 lines vs 1)
- ❌ Current implementation doesn't add much value (just 2 fields)

### Without FieldContext

**Pros:**
- ✅ Simpler - no wrapper type needed
- ✅ More concise generated code (1 line)
- ✅ Fewer generic parameters in field functions
- ✅ Direct and obvious

**Cons:**
- ❌ Adding new shared fields breaks ALL function signatures
- ❌ More parameters in each function call
- ❌ Less encapsulation

## Recommendation

**It depends on future plans:**

### Option A: Keep FieldContext (recommended if we plan to add more shared state)

If we're planning to add:
- `bool_bits` for boolean field packing
- Custom allocator support
- Unknown fields tracking
- Any other shared state

**Then FieldContext is valuable** because:
- Adding these won't break existing function signatures
- One-time cost (extra line) vs massive refactoring later

### Option B: Remove FieldContext (if we keep it simple)

If we're keeping it minimal and don't plan complex shared state:
- Simpler is better
- Direct parameter passing
- One less type to understand

## My Analysis

**Current situation:**
- We have `_shared: SharedFields` which already groups shared state
- `FieldContext` is essentially just passing `(&mut has_bits, index)` pair
- We're planning to add `bool_bits`, `allocator`, etc.

**Verdict:**

Given our plans for future extensions (bool_bits, allocator, unknown_fields),
**FieldContext provides good value** despite current simplicity.

However, there's a middle ground:

### Alternative: Borrow from SharedFields directly

```rust
// Instead of creating FieldContext, pass SharedFields + index
pub fn set_string<T, O>(
    shared: &mut SharedFields<BYTES>,
    bit_index: usize,
    storage: &mut String,
    value: &str,
) {
    value.clone_into(storage);
    shared.has_bits_mut().set(bit_index, true);
}

// Generated code
field::set_string(&mut self._shared, IDX_NAME, &mut self.name, v);
```

This might be cleaner - pass the whole SharedFields, not just has_bits.

## Questions for Discussion

1. Are we planning to add bool_bits, allocator, etc.? (If yes, keep FieldContext)
2. Should we pass `&mut SharedFields` instead of individual fields? (Cleaner?)
3. Is the 2-line vs 1-line difference in generated code acceptable?

