//! Shared Criterion helper for parse-only (`decode_keep`) benches.
//!
//! [`criterion::Bencher::iter_with_large_drop`] keeps every iteration's result
//! until the sample ends. A million-iteration sample of a large message then
//! first-touches gigabytes of heap *inside* the timed loop, so `decode_keep`
//! can read slower than `decode`. This helper still postpones [`Drop`] — it
//! just drops in fixed-size bursts after each timed burst.

use ::core::time::Duration;
use ::criterion::measurement::WallTime;
use ::criterion::Bencher;
use ::std::time::Instant;

/// Live values retained per timed burst before `Drop` runs off-clock.
const KEEP_BATCH: usize = 1024;

/// Times `f` without including [`Drop`] of the returned values.
pub fn iter_keep<T>(bencher: &mut Bencher<'_, WallTime>, mut f: impl FnMut() -> T) {
    bencher.iter_custom(|iters| {
        let mut kept = Vec::<T>::with_capacity(KEEP_BATCH);
        let mut total = Duration::ZERO;
        let mut remaining = iters;
        while remaining > 0 {
            let n = remaining.min(KEEP_BATCH as u64);
            let start = Instant::now();
            for _ in 0..n {
                kept.push(f());
            }
            total += start.elapsed();
            kept.clear();
            remaining -= n;
        }
        total
    });
}
