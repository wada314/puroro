//! Hand-written helpers for [`Timestamp`](crate::google::protobuf::Timestamp).
//!
//! Kept outside the generated forest so `protoc-gen-puroro` stays free of WKT
//! special cases. Normalisation follows
//! [`google::protobuf::util::CreateNormalized`][1].
//!
//! [1]: https://github.com/protocolbuffers/protobuf/blob/v3.3.2/src/google/protobuf/util/time_util.cc#L59-L77

use crate::google::protobuf::Timestamp;
use ::allocator_api2::alloc::Allocator;
use ::core::fmt;
use ::std::time::{Duration as StdDuration, SystemTime, UNIX_EPOCH};

const NANOS_PER_SECOND: i32 = 1_000_000_000;

/// Error converting a [`Timestamp`] to [`SystemTime`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum TimestampError {
    /// The normalised seconds/nanos cannot be represented as [`SystemTime`]
    /// on this platform (range varies; Windows / 32-bit hosts are narrower).
    OutOfSystemRange { seconds: i64, nanos: i32 },
}

impl fmt::Display for TimestampError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimestampError::OutOfSystemRange { seconds, nanos } => {
                write!(
                    f,
                    "Timestamp {{ seconds: {seconds}, nanos: {nanos} }} is not \
                     representable as a SystemTime on this platform"
                )
            }
        }
    }
}

impl ::std::error::Error for TimestampError {}

impl<A: Allocator + ::core::clone::Clone> Timestamp<A> {
    /// Sets `seconds` / `nanos` on an empty message allocated with `alloc`.
    ///
    /// Does not normalise; call [`normalize()`](Self::normalize) if the pair
    /// may be denormal (for example `nanos >= 1_000_000_000`).
    #[inline]
    pub fn from_seconds_nanos_in(alloc: A, seconds: i64, nanos: i32) -> Self {
        let mut msg = Self::new_in(alloc);
        *msg.seconds_mut() = seconds;
        *msg.nanos_mut() = nanos;
        msg
    }

    /// Builds a [`Timestamp`] from [`SystemTime`] using `alloc`.
    ///
    /// Pre-epoch times keep non-negative `nanos` that count forward within the
    /// second, matching the protobuf Timestamp rules.
    pub fn from_system_time_in(alloc: A, system_time: SystemTime) -> Self {
        let (seconds, nanos) = match system_time.duration_since(UNIX_EPOCH) {
            Ok(duration) => {
                let seconds =
                    i64::try_from(duration.as_secs()).expect("SystemTime seconds fit i64");
                (seconds, duration.subsec_nanos() as i32)
            }
            Err(error) => {
                let duration = error.duration();
                let seconds =
                    i64::try_from(duration.as_secs()).expect("SystemTime seconds fit i64");
                let nanos = duration.subsec_nanos() as i32;
                if nanos == 0 {
                    (-seconds, 0)
                } else {
                    (-seconds - 1, NANOS_PER_SECOND - nanos)
                }
            }
        };
        Self::from_seconds_nanos_in(alloc, seconds, nanos)
    }

    /// Canonicalises `nanos` into `[0, 999_999_999]`, carrying into `seconds`.
    ///
    /// On overflow of `seconds`, clamps to the extreme representable pair
    /// (`i64::MIN` / `0`, or `i64::MAX` / `999_999_999`).
    pub fn normalize(&mut self) {
        let mut seconds = self.seconds();
        let mut nanos = self.nanos();

        if nanos <= -NANOS_PER_SECOND || nanos >= NANOS_PER_SECOND {
            if let Some(s) = seconds.checked_add((nanos / NANOS_PER_SECOND) as i64) {
                seconds = s;
                nanos %= NANOS_PER_SECOND;
            } else if nanos < 0 {
                seconds = i64::MIN;
                nanos = 0;
            } else {
                seconds = i64::MAX;
                nanos = NANOS_PER_SECOND - 1;
            }
        }

        // Timestamp nanos must be non-negative.
        if nanos < 0 {
            if let Some(s) = seconds.checked_sub(1) {
                seconds = s;
                nanos += NANOS_PER_SECOND;
            } else {
                debug_assert_eq!(seconds, i64::MIN);
                nanos = 0;
            }
        }

        *self.seconds_mut() = seconds;
        *self.nanos_mut() = nanos;
    }

    /// Returns a normalised clone.
    #[inline]
    pub fn normalized(&self) -> Self {
        let mut out = self.clone();
        out.normalize();
        out
    }

    /// Converts this timestamp to [`SystemTime`] after normalisation.
    pub fn try_to_system_time(&self) -> Result<SystemTime, TimestampError> {
        let norm = self.normalized();
        let seconds = norm.seconds();
        let nanos = norm.nanos();
        debug_assert!((0..NANOS_PER_SECOND).contains(&nanos));

        let base = if seconds >= 0 {
            UNIX_EPOCH.checked_add(StdDuration::from_secs(seconds as u64))
        } else {
            let neg = seconds
                .checked_neg()
                .ok_or(TimestampError::OutOfSystemRange { seconds, nanos })?;
            UNIX_EPOCH.checked_sub(StdDuration::from_secs(neg as u64))
        };

        base.and_then(|t| t.checked_add(StdDuration::from_nanos(nanos as u64)))
            .ok_or(TimestampError::OutOfSystemRange { seconds, nanos })
    }
}

impl<A: Allocator + ::core::clone::Clone + Default> Timestamp<A> {
    /// Like [`from_seconds_nanos_in()`](Self::from_seconds_nanos_in) with
    /// [`A::default()`](Default::default).
    #[inline]
    pub fn from_seconds_nanos(seconds: i64, nanos: i32) -> Self {
        Self::from_seconds_nanos_in(A::default(), seconds, nanos)
    }

    /// Like [`from_system_time_in()`](Self::from_system_time_in) with
    /// [`A::default()`](Default::default).
    #[inline]
    pub fn from_system_time(system_time: SystemTime) -> Self {
        Self::from_system_time_in(A::default(), system_time)
    }
}

impl<A: Allocator + ::core::clone::Clone + Default> From<SystemTime> for Timestamp<A> {
    fn from(system_time: SystemTime) -> Self {
        Self::from_system_time(system_time)
    }
}

impl<A: Allocator + ::core::clone::Clone> TryFrom<&Timestamp<A>> for SystemTime {
    type Error = TimestampError;

    fn try_from(timestamp: &Timestamp<A>) -> Result<Self, Self::Error> {
        timestamp.try_to_system_time()
    }
}

impl<A: Allocator + ::core::clone::Clone> TryFrom<Timestamp<A>> for SystemTime {
    type Error = TimestampError;

    fn try_from(timestamp: Timestamp<A>) -> Result<Self, Self::Error> {
        timestamp.try_to_system_time()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn system_time_round_trip_after_epoch() {
        let t = UNIX_EPOCH + StdDuration::new(1_700_000_000, 123_456_789);
        let ts: Timestamp = Timestamp::from(t);
        assert_eq!(ts.seconds(), 1_700_000_000);
        assert_eq!(ts.nanos(), 123_456_789);
        assert_eq!(SystemTime::try_from(&ts).unwrap(), t);
    }

    #[test]
    fn system_time_before_epoch_keeps_non_negative_nanos() {
        let t = UNIX_EPOCH - StdDuration::new(1, 250);
        let ts: Timestamp = Timestamp::from(t);
        assert_eq!(ts.seconds(), -2);
        assert_eq!(ts.nanos(), NANOS_PER_SECOND - 250);
        assert_eq!(SystemTime::try_from(ts).unwrap(), t);
    }

    #[test]
    fn normalize_carries_nanos() {
        let mut ts: Timestamp = Timestamp::from_seconds_nanos(1, 1_000_000_001);
        ts.normalize();
        assert_eq!(ts.seconds(), 2);
        assert_eq!(ts.nanos(), 1);

        let mut ts: Timestamp = Timestamp::from_seconds_nanos(0, -1);
        ts.normalize();
        assert_eq!(ts.seconds(), -1);
        assert_eq!(ts.nanos(), 999_999_999);
    }

    #[test]
    fn default_alloc_constructors() {
        use ::allocator_api2::alloc::Global;

        let t = UNIX_EPOCH + StdDuration::from_secs(10);
        let via_method = Timestamp::<Global>::from_system_time(t);
        let via_from: Timestamp<Global> = t.into();
        assert_eq!(via_method.seconds(), 10);
        assert_eq!(via_from.seconds(), 10);
        assert_eq!(Timestamp::<Global>::from_seconds_nanos(3, 4).nanos(), 4);
    }
}
