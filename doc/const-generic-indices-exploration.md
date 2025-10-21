# Const Generic Indices Exploration

## Idea

Pass field-specific information (bit indices) as const generic parameters instead of runtime parameters.

## Comparison

### Current Approach: Runtime Parameters

```rust
// Library function
pub fn set_string<const BYTES: usize>(
    shared: &mut SharedFields<BYTES>,
    bit_index: usize,  // ← Runtime parameter
    storage: &mut String,
    value: &str,
) {
    value.clone_into(storage);
    shared.has_bits_mut().set(bit_index, true);
}

// Generated code
impl PersonAppend for PersonImpl {
    fn set_name(&mut self, v: &str) {
        field::set_string(&mut self._shared, IDX_NAME, &mut self.name, v);
        //                                   ^^^^^^^^ runtime parameter
    }
}
```

**Characteristics:**
- Simple function call syntax
- Bit index passed at runtime
- Single monomorphization (all indices use same code)

### Alternative: Const Generic Parameters

```rust
// Library function
pub fn set_string<const BYTES: usize, const BIT_INDEX: usize>(
    shared: &mut SharedFields<BYTES>,
    storage: &mut String,
    value: &str,
) {
    value.clone_into(storage);
    shared.has_bits_mut().set(BIT_INDEX, true);  // Compile-time constant
}

// Generated code
impl PersonAppend for PersonImpl {
    fn set_name(&mut self, v: &str) {
        field::set_string::<1, IDX_NAME>(&mut self._shared, &mut self.name, v);
        //                 ^^^^^^^^^^^^^ turbofish syntax
    }
}
```

**Characteristics:**
- Turbofish syntax required (`::<1, IDX_NAME>`)
- Bit index known at compile time
- Separate monomorphization per field (more code generation)

## Analysis

### Code Size Impact

**Runtime parameters (current):**
- One function instantiation: `set_string<1>`
- All fields (name, email) call the same instantiation
- Bit index passed as runtime value

**Const generic parameters:**
- Multiple function instantiations: `set_string<1, 0>`, `set_string<1, 1>`, `set_string<1, 2>`
- Each field gets its own specialized version
- Bit index is compile-time constant

**Verdict:** Runtime approach generates **less code** (single monomorphization).

### Performance Impact

**Runtime parameters:**
```rust
shared.has_bits_mut().set(bit_index, true);
// bit_index is a runtime value
// Simple array indexing: O(1)
```

**Const generic parameters:**
```rust
shared.has_bits_mut().set(BIT_INDEX, true);
// BIT_INDEX is compile-time constant
// Compiler might optimize further?
```

**Reality check:**
- With `#[inline]` on `set_string`, the bit index becomes a constant anyway
- Compiler can already optimize `set(IDX_NAME, true)` where IDX_NAME is const
- No significant performance difference expected

**Verdict:** Performance is likely **identical** with inlining.

### Readability Impact

**Runtime parameters:**
```rust
field::set_string(&mut self._shared, IDX_NAME, &mut self.name, v);
```
- ✅ Clean, simple function call
- ✅ Parameters are obvious
- ✅ No special syntax

**Const generic parameters:**
```rust
field::set_string::<1, IDX_NAME>(&mut self._shared, &mut self.name, v);
```
- ⚠️ Turbofish syntax required
- ⚠️ Two const params (BYTES, BIT_INDEX)
- ⚠️ More cognitive load

**Verdict:** Runtime approach is **more readable**.

### Boolean Field Example

For boolean fields with two indices:

**Runtime parameters:**
```rust
pub fn set_bool<const BYTES: usize, const BOOL_BYTES: usize>(
    shared: &mut SharedFields<BYTES, BOOL_BYTES>,
    has_index: usize,   // ← Runtime
    bool_index: usize,  // ← Runtime
    value: bool,
)

// Generated code
field::set_bool(&mut self._shared, IDX_IS_ACTIVE_HAS, IDX_IS_ACTIVE_BOOL, true);
```

**Const generic parameters:**
```rust
pub fn set_bool<const BYTES: usize, const BOOL_BYTES: usize, const HAS_INDEX: usize, const BOOL_INDEX: usize>(
    shared: &mut SharedFields<BYTES, BOOL_BYTES>,
    value: bool,
)

// Generated code
field::set_bool::<1, 1, IDX_IS_ACTIVE_HAS, IDX_IS_ACTIVE_BOOL>(&mut self._shared, true);
//                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ 4 generic params!
```

**Verdict:** For multiple indices, runtime parameters are **much cleaner**.

## Recommendations

### Keep Runtime Parameters (Current Approach)

**Reasons:**
1. **Simpler generated code** - No turbofish syntax
2. **Smaller binary size** - Single monomorphization per function
3. **Better readability** - Clear what each parameter is
4. **No performance penalty** - Inlining makes constants anyway
5. **Scales better** - Multiple indices (bool fields, nested messages, etc.)

### When Const Generics Might Help

Const generics are useful when:
- Type-level computation needed
- Zero-sized types (ZST) optimization important
- Generic code needs compile-time dispatch

For field indices:
- ❌ Not needed for type-level computation
- ❌ Indices are already optimized with inlining
- ❌ No benefit from compile-time dispatch

## Conclusion

**Recommendation: Keep runtime parameters.**

The current approach (`bit_index: usize`) is:
- ✅ Simpler
- ✅ More readable
- ✅ Smaller code size
- ✅ Same performance (with `#[inline]`)

Const generic parameters would:
- ❌ Add complexity (turbofish)
- ❌ Increase code size (more monomorphizations)
- ❌ Reduce readability
- ≈ No performance benefit

## Performance Note

With `#[inline]`, this:
```rust
#[inline]
pub fn set_string<const BYTES: usize>(
    shared: &mut SharedFields<BYTES>,
    bit_index: usize,  // Runtime value
    storage: &mut String,
    value: &str,
)

// Called with:
field::set_string(&mut self._shared, IDX_NAME, &mut self.name, v);
//                                   ^^^^^^^^ const value
```

Gets optimized to the same code as:
```rust
#[inline]
pub fn set_string<const BYTES: usize, const BIT_INDEX: usize>(...)

// Called with:
field::set_string::<1, IDX_NAME>(&mut self._shared, &mut self.name, v);
```

Because `IDX_NAME` is a `const`, the compiler already treats it as a constant
when inlining, making the runtime parameter effectively compile-time.

**Therefore: No need for const generics here.**

