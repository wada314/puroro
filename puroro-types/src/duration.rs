//! Hand-written helpers for [`Duration`](crate::google::protobuf::Duration).
//!
//! Kept outside the generated forest so `protoc-gen-puroro` stays free of WKT
//! special cases. Normalisation follows
//! [`google::protobuf::util::CreateNormalized`][1].
//!
//! [1]: https://github.com/protocolbuffers/protobuf/blob/v3.3.2/src/google/protobuf/util/time_util.cc#L79-L100

use crate::google::protobuf::Duration as ProtoDuration;
use ::allocator_api2::alloc::Allocator;
use ::core::fmt;
use ::std::time::Duration as StdDuration;

const NANOS_PER_SECOND: i32 = 1_000_000_000;
const NANOS_MAX: i32 = NANOS_PER_SECOND - 1;

/// Error converting between [`ProtoDuration`] and [`StdDuration`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DurationError {
    /// Protobuf duration was negative; the inner value is the magnitude.
    NegativeDuration(StdDuration),
    /// Magnitude does not fit in a protobuf `Duration` / `std::time::Duration`.
    OutOfRange,
}

impl fmt::Display for DurationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DurationError::NegativeDuration(d) => {
                write!(f, "failed to convert negative Duration (magnitude {d:?})")
            }
            DurationError::OutOfRange => {
                write!(f, "duration magnitude out of representable range")
            }
        }
    }
}

impl ::std::error::Error for DurationError {}

impl<A: Allocator + ::core::clone::Clone> ProtoDuration<A> {
    /// Sets `seconds` / `nanos` on an empty message allocated with `alloc`.
    ///
    /// Does not normalise; call [`normalize()`](Self::normalize) if needed.
    #[inline]
    pub fn from_seconds_nanos_in(alloc: A, seconds: i64, nanos: i32) -> Self {
        let mut msg = Self::new_in(alloc);
        *msg.seconds_mut() = seconds;
        *msg.nanos_mut() = nanos;
        msg
    }

    /// Builds from [`StdDuration`] using `alloc`.
    pub fn try_from_std_in(alloc: A, duration: StdDuration) -> Result<Self, DurationError> {
        let seconds = i64::try_from(duration.as_secs()).map_err(|_| DurationError::OutOfRange)?;
        let nanos = duration.subsec_nanos() as i32;
        let mut msg = Self::from_seconds_nanos_in(alloc, seconds, nanos);
        msg.normalize();
        Ok(msg)
    }

    /// Canonicalises so `|nanos| < 1e9` and `nanos` has the same sign as
    /// `seconds` (or is zero).
    ///
    /// On overflow of `seconds`, clamps to the extreme representable pair.
    pub fn normalize(&mut self) {
        let mut seconds = self.seconds();
        let mut nanos = self.nanos();

        if nanos <= -NANOS_PER_SECOND || nanos >= NANOS_PER_SECOND {
            if let Some(s) = seconds.checked_add((nanos / NANOS_PER_SECOND) as i64) {
                seconds = s;
                nanos %= NANOS_PER_SECOND;
            } else if nanos < 0 {
                seconds = i64::MIN;
                nanos = -NANOS_MAX;
            } else {
                seconds = i64::MAX;
                nanos = NANOS_MAX;
            }
        }

        if seconds < 0 && nanos > 0 {
            if let Some(s) = seconds.checked_add(1) {
                seconds = s;
                nanos -= NANOS_PER_SECOND;
            } else {
                debug_assert_eq!(seconds, i64::MAX);
                nanos = NANOS_MAX;
            }
        } else if seconds > 0 && nanos < 0 {
            if let Some(s) = seconds.checked_sub(1) {
                seconds = s;
                nanos += NANOS_PER_SECOND;
            } else {
                debug_assert_eq!(seconds, i64::MIN);
                nanos = -NANOS_MAX;
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

    /// Converts to [`StdDuration`] after normalisation.
    ///
    /// Fails when the normalised duration is negative.
    pub fn try_to_std(&self) -> Result<StdDuration, DurationError> {
        let norm = self.normalized();
        let seconds = norm.seconds();
        let nanos = norm.nanos();
        if seconds >= 0 && nanos >= 0 {
            Ok(StdDuration::new(seconds as u64, nanos as u32))
        } else {
            Err(DurationError::NegativeDuration(StdDuration::new(
                (-seconds) as u64,
                (-nanos) as u32,
            )))
        }
    }
}

impl<A: Allocator + ::core::clone::Clone + Default> ProtoDuration<A> {
    /// Like [`from_seconds_nanos_in()`](Self::from_seconds_nanos_in) with
    /// [`A::default()`](Default::default).
    #[inline]
    pub fn from_seconds_nanos(seconds: i64, nanos: i32) -> Self {
        Self::from_seconds_nanos_in(A::default(), seconds, nanos)
    }

    /// Like [`try_from_std_in()`](Self::try_from_std_in) with
    /// [`A::default()`](Default::default).
    #[inline]
    pub fn try_from_std(duration: StdDuration) -> Result<Self, DurationError> {
        Self::try_from_std_in(A::default(), duration)
    }
}

impl<A: Allocator + ::core::clone::Clone + Default> TryFrom<StdDuration> for ProtoDuration<A> {
    type Error = DurationError;

    fn try_from(duration: StdDuration) -> Result<Self, Self::Error> {
        Self::try_from_std(duration)
    }
}

impl<A: Allocator + ::core::clone::Clone> TryFrom<&ProtoDuration<A>> for StdDuration {
    type Error = DurationError;

    fn try_from(duration: &ProtoDuration<A>) -> Result<Self, Self::Error> {
        duration.try_to_std()
    }
}

impl<A: Allocator + ::core::clone::Clone> TryFrom<ProtoDuration<A>> for StdDuration {
    type Error = DurationError;

    fn try_from(duration: ProtoDuration<A>) -> Result<Self, Self::Error> {
        duration.try_to_std()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn std_round_trip() {
        let std = StdDuration::new(42, 123_456_789);
        let d: ProtoDuration = ProtoDuration::try_from(std).unwrap();
        assert_eq!(d.seconds(), 42);
        assert_eq!(d.nanos(), 123_456_789);
        assert_eq!(StdDuration::try_from(&d).unwrap(), std);
    }

    #[test]
    fn negative_rejected_for_std() {
        let d: ProtoDuration = ProtoDuration::from_seconds_nanos(-1, -500);
        match d.try_to_std() {
            Err(DurationError::NegativeDuration(mag)) => {
                assert_eq!(mag, StdDuration::new(1, 500));
            }
            other => panic!("expected NegativeDuration, got {other:?}"),
        }
    }

    #[test]
    fn normalize_aligns_sign() {
        let mut d: ProtoDuration = ProtoDuration::from_seconds_nanos(1, -1);
        d.normalize();
        assert_eq!(d.seconds(), 0);
        assert_eq!(d.nanos(), 999_999_999);

        let mut d: ProtoDuration = ProtoDuration::from_seconds_nanos(-1, 1);
        d.normalize();
        assert_eq!(d.seconds(), 0);
        assert_eq!(d.nanos(), -999_999_999);
    }

    #[test]
    fn default_alloc_constructors() {
        use ::allocator_api2::alloc::Global;

        let std = StdDuration::new(7, 8);
        let via_method = ProtoDuration::<Global>::try_from_std(std).unwrap();
        let via_try: ProtoDuration<Global> = std.try_into().unwrap();
        assert_eq!(via_method.seconds(), 7);
        assert_eq!(via_try.nanos(), 8);
        assert_eq!(
            ProtoDuration::<Global>::from_seconds_nanos(-2, -3).seconds(),
            -2
        );
    }
}
