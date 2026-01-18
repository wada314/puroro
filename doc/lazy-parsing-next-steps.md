# Next Steps for Lazy Repeated Field Parsing

**Last Updated**: 2025-01-01  
**Status**: ✅ Basic implementation complete; optimizations and documentation updates in progress

## Current Implementation Status

✅ **Implemented**:
- `LazyRepeated` wrapper type that implements `Repeated` trait
- `LazyRepeatedIter` iterator that triggers parsing on-demand in `next()`
- `PersonLazyImpl::scores()` and `addresses()` return `LazyRepeated`
- All tests passing (15 tests)

## Completed Features

1. **On-demand parsing**: Elements are parsed only when accessed via `iter()`, `get()`, `len()`, etc.
2. **Incremental parsing**: `ensure_at_least()` parses until required number of elements is found
3. **Iterator support**: `LazyRepeatedIter` triggers parsing when iterator is exhausted
4. **Repeated trait implementation**: All methods (`get()`, `len()`, `is_empty()`, `iter_box()`) work correctly

## Known Issues and Potential Improvements

### 1. Efficiency: Counting and Iterator Recreation Overhead

**Current Behavior**:
- `ensure_at_least()` calls `parse_until_with_callback(...)` and checks a condition after each processed field
- This avoids parsing the entire message when only a small number of elements are needed
- However, there is still overhead from repeated counting and iterator recreation

**Impact**:
- Correctness: ✅ Correct (all tests pass)
- Efficiency: ⚠️ Could be improved (parses more than necessary)

**Potential Solution**:
- Reduce repeated `list.iter().count()` calls (currently O(n) per check) by tracking counts more directly
- Reduce iterator recreation/skip cost in `LazyRepeatedIter::next()`

### 2. Unused `_field_number` Field

**Current State**:
- `LazyRepeated` stores `_field_number` but doesn't use it (indicated by `_` prefix)
- This field could be used for future optimizations (e.g., field-specific parsing logic)

**Action**: Document why it's stored but unused, or remove if not needed for future optimizations

### 3. Design Documentation Update

**Status**: In progress (this document is being updated to match the current implementation)

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
   - Decide whether to keep or remove `_field_number` field
   - Add documentation comments explaining design decisions

### Low Priority

4. **Additional Testing**
   - Add tests for edge cases (very large repeated fields, interleaved fields, etc.)
   - Performance benchmarks
   - Stress tests

5. **Future Enhancements**
   - Consider field-specific parsing optimizations using `_field_number`
   - Consider caching parsed state more aggressively
   - Consider supporting parallel parsing (if applicable)

## Unresolved Questions

1. **Performance Trade-offs**: Is the current implementation's efficiency acceptable, or do we need to optimize the current `LazyRepeated`/`LazyRepeatedIter` hot paths?

2. **`_field_number` Usage**: Should we keep this field for future optimizations, or remove it if not needed?

3. **Documentation Strategy**: Should we update the existing design document or create a new implementation guide?

