# Next Steps for Lazy Repeated Field Parsing

**Last Updated**: 2026-01-22  
**Status**: ✅ Basic implementation complete; key performance optimizations implemented

## Current Implementation Status

✅ **Implemented**:
- `LazyRepeated` wrapper type that implements `Repeated` trait
- `LazyRepeatedIter` iterator that triggers parsing on-demand in `next()`
- `PersonLazyImpl::scores()` and `addresses()` return `LazyRepeated`
- `once-list2` tail append optimization integrated (0.4.0)
- Reduced overhead in hot paths:
  - `ensure_at_least()` uses O(1) `len()` (tail+len caching)
  - `LazyRepeatedIter` stores the concrete `once_list2::Iter` (no `Box<dyn Iterator>` field)
  - `Repeated::iter_box()` returns an on-demand iterator (does not force full parsing up-front)
- All tests passing

## Completed Features

1. **On-demand parsing**: Elements are parsed only when accessed via `iter()`, `get()`, `len()`, etc.
2. **Incremental parsing**: `ensure_at_least()` parses until required number of elements is found
3. **Iterator support**: `LazyRepeatedIter` triggers parsing when iterator is exhausted
4. **Repeated trait implementation**: All methods (`get()`, `len()`, `is_empty()`, `iter_box()`) work correctly

## Known Issues and Potential Improvements

### 1. Efficiency: Counting and Iterator Recreation Overhead

**Current Behavior**:
- `ensure_at_least()` advances the parent parser one field at a time and uses a cheap check (`len()`)
- `LazyRepeatedIter` advances parsing only when its underlying iterator is exhausted
- This avoids parsing the entire message when only a small number of elements are needed

**Impact**:
- Correctness: ✅ Correct (all tests pass)
- Efficiency: ✅ Improved (removed repeated O(n) counting and iterator boxing/collection)

**Remaining Considerations**:
- `Repeated::get(index)` is inherently O(n) on a singly-linked list (acceptable for now; can be revisited if needed)
- If we ever remove random-access APIs (`get()`/`len()`) from `Repeated`, we may be able to simplify or remove `ensure_at_least()`

### 2. Unused `_field_number` Field

**Current State**:
- (resolved) `LazyRepeated` no longer stores `_field_number` since it was unused.

**Action**: Document why it's stored but unused, or remove if not needed for future optimizations

### 3. Design Documentation Update

**Status**: In progress (this document is being updated to match the current implementation)

### 4. Zero-copy string/bytes views (high priority)

**Goal**:
- Avoid allocating `String`/`Vec<u8>` in lazy message implementations when the input slice can be borrowed.

**Proposed approach**:
- On decoding a `string` field, validate UTF-8 via `str::from_utf8(data)` and store `&'slice str` in the message body.
- For `bytes`, store `&'slice [u8]` directly.
- Preserve the protobuf "last one wins" semantics by allowing overwrite on repeated occurrences (e.g., store `Option<&'slice str>`).

## Next Steps (Priority Order)

### High Priority

1. **Update Design Documentation**
   - Keep `lazy-parsing-state-design.md` aligned with current code and avoid drifting historical notes

### Medium Priority

2. **Performance Optimization (if needed)**
   - Measure performance impact of current implementation
   - Consider optimizing the current `LazyRepeated`/`LazyRepeatedIter` hot paths if performance is an issue
   - This is a "nice to have" - current implementation is correct and works

3. **Code Cleanup**
   - (done) Remove unused `_field_number` field from `LazyRepeated`

### Low Priority

4. **Additional Testing**
   - Add tests for edge cases (very large repeated fields, interleaved fields, etc.)
   - Performance benchmarks
   - Stress tests

5. **Future Enhancements**
   - Consider field-specific parsing optimizations (would require reintroducing field metadata)
   - Consider caching parsed state more aggressively
   - Consider supporting parallel parsing (if applicable)

## Unresolved Questions

1. **Performance Trade-offs**: Is the current implementation's efficiency acceptable, or do we need to optimize the current `LazyRepeated`/`LazyRepeatedIter` hot paths?

2. **Field metadata**: If we need field-specific parsing optimizations later, what's the right way to pass the metadata (without bloating the common case)?

3. **Documentation Strategy**: Should we update the existing design document or create a new implementation guide?

