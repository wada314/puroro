//! `AddressView` versus eager `Address` and `AddressLazy`.
//!
//! Archived spike, outside `sample-generated`. One scan stores last-wins offsets
//! into the caller's buffer. String UTF-8 is checked on every get.
//!
//! `rescan_*` walks the whole buffer on every read. That is the fully-lazy
//! alternative the index exists to avoid; it is not a public API.
//!
//! Lazy steady-state reads (`lazy_read_*`) run after one untimed get, so the
//! string has already been promoted out of the wire span. The one-shot cost
//! of that promotion is in `lazy_decode_read_*`.
//!
//! `eager_decode` includes `Drop`. `eager_decode_keep` does not.
//!
//! One run on this cloud VM (Criterion median, 20 samples, 1s each), ASCII
//! payloads. Short is about 30 bytes (`"Main St"` / `"Oakland"`, SSO). Long is
//! about 2 KiB (two 1024-byte strings). Absolute times move; the gaps below
//! are large enough to read.
//!
//! | | short | long |
//! |---|---:|---:|
//! | eager decode | 89 ns | 1.58 µs |
//! | lazy decode | 201 ns | 352 ns |
//! | view parse | 34 ns | 33 ns |
//! | eager decode + street | 92 ns | 1.41 µs |
//! | lazy decode + street | 276 ns | 1.11 µs |
//! | view parse + street | 49 ns | 82 ns |
//! | eager decode + postal only | 89 ns | 1.42 µs |
//! | lazy decode + postal only | 277 ns | 357 ns |
//! | view parse + postal only | 34 ns | 35 ns |
//! | eager street, already built | 2.2 ns | 1.3 ns |
//! | view street, already built | 11 ns | 57 ns |
//! | rescan street every get | 28 ns | 80 ns |
//! | rescan all four fields | 143 ns | 167 ns |
//!
//! View parse does not touch string bytes, so short and long parses match.
//! Repeated string gets re-check UTF-8. `to_owned` is a second eager decode.
//!
//! Run from this crate (it is outside the puroro workspace):
//! ```text
//! cargo bench --bench view
//! ```

use ::address_view_spike::AddressView;
use ::allocator_api2::alloc::Global;
use ::bytes::Buf;
use ::core::str;
use ::core::time::Duration;
use ::criterion::{Criterion, Throughput, black_box, criterion_group, criterion_main};
use ::puroro::{DecodeError, Message, WireType};
use ::puroro_rt::decode::{decode_tag, decode_varint};
use ::puroro_sample_generated::{Address, AddressLazy};
use ::std::sync::LazyLock;

use keep::iter_keep;

const LONG: usize = 1024;

static SHORT_BYTES: LazyLock<Vec<u8>> = LazyLock::new(|| encode("Main St", "Oakland", 94612, 37.5));

static LONG_BYTES: LazyLock<Vec<u8>> = LazyLock::new(|| {
    let street = "S".repeat(LONG);
    let city = "C".repeat(LONG);
    encode(&street, &city, 94612, 37.5)
});

fn encode(street: &str, city: &str, postal_code: u32, latitude: f64) -> Vec<u8> {
    use ::puroro::StringMut;
    let mut msg = Address::new();
    msg.street_mut().set(street);
    msg.city_mut().set(city);
    *msg.postal_code_mut() = postal_code;
    *msg.latitude_mut() = latitude;
    msg.encode_to_vec()
}

fn eager_street(msg: &Address) -> usize {
    msg.street().get().len()
}

fn eager_postal(msg: &Address) -> u32 {
    msg.postal_code().get()
}

fn eager_all(msg: &Address) -> usize {
    msg.street().get().len()
        + msg.city().get().len()
        + msg.postal_code().get() as usize
        + msg.latitude().get().to_bits() as usize
}

fn lazy_street(msg: &AddressLazy) -> usize {
    msg.street().unwrap().get().len()
}

fn lazy_postal(msg: &AddressLazy) -> u32 {
    msg.postal_code().unwrap().get()
}

fn lazy_all(msg: &AddressLazy) -> usize {
    msg.street().unwrap().get().len()
        + msg.city().unwrap().get().len()
        + msg.postal_code().unwrap().get() as usize
        + msg.latitude().unwrap().get().to_bits() as usize
}

fn view_street(view: &AddressView) -> usize {
    view.street().unwrap().map(|s| s.len()).unwrap_or(0)
}

fn view_postal(view: &AddressView) -> u32 {
    view.postal_code().unwrap_or(0)
}

fn view_all(view: &AddressView) -> usize {
    view.street().unwrap().map(|s| s.len()).unwrap_or(0)
        + view.city().unwrap().map(|s| s.len()).unwrap_or(0)
        + view.postal_code().unwrap_or(0) as usize
        + view.latitude().unwrap_or(0.0).to_bits() as usize
}

fn rescan_street(buf: &[u8]) -> usize {
    rescan_field(buf, 1).len()
}

fn rescan_all(buf: &[u8]) -> usize {
    rescan_field(buf, 1).len()
        + rescan_field(buf, 2).len()
        + rescan_fixed32(buf, 3).unwrap_or(0) as usize
        + rescan_fixed64(buf, 4).unwrap_or(0.0).to_bits() as usize
}

fn rescan_field(buf: &[u8], field: u32) -> &str {
    let mut rest = buf;
    let mut found: Option<&[u8]> = None;
    while rest.has_remaining() {
        let (number, wire_type) = decode_tag(&mut rest).unwrap();
        if number.as_u32() == field {
            assert_eq!(wire_type, WireType::Len);
            let len = decode_varint(&mut rest).unwrap() as usize;
            let (payload, tail) = rest.split_at(len);
            found = Some(payload);
            rest = tail;
        } else {
            skip_payload(wire_type, &mut rest).unwrap();
        }
    }
    str::from_utf8(found.unwrap()).unwrap()
}

fn rescan_fixed32(buf: &[u8], field: u32) -> Option<u32> {
    let mut rest = buf;
    let mut found = None;
    while rest.has_remaining() {
        let (number, wire_type) = decode_tag(&mut rest).unwrap();
        if number.as_u32() == field {
            assert_eq!(wire_type, WireType::Int32);
            let mut bytes = [0u8; 4];
            bytes.copy_from_slice(&rest[..4]);
            rest = &rest[4..];
            found = Some(u32::from_le_bytes(bytes));
        } else {
            skip_payload(wire_type, &mut rest).unwrap();
        }
    }
    found
}

fn rescan_fixed64(buf: &[u8], field: u32) -> Option<f64> {
    let mut rest = buf;
    let mut found = None;
    while rest.has_remaining() {
        let (number, wire_type) = decode_tag(&mut rest).unwrap();
        if number.as_u32() == field {
            assert_eq!(wire_type, WireType::Int64);
            let mut bytes = [0u8; 8];
            bytes.copy_from_slice(&rest[..8]);
            rest = &rest[8..];
            found = Some(f64::from_le_bytes(bytes));
        } else {
            skip_payload(wire_type, &mut rest).unwrap();
        }
    }
    found
}

fn skip_payload(wire_type: WireType, rest: &mut &[u8]) -> Result<(), DecodeError> {
    match wire_type {
        WireType::Varint => {
            decode_varint(rest)?;
        }
        WireType::Int64 => {
            *rest = &rest[8..];
        }
        WireType::Len => {
            let len = decode_varint(rest)? as usize;
            *rest = &rest[len..];
        }
        WireType::Int32 => {
            *rest = &rest[4..];
        }
        WireType::SGroup | WireType::EGroup => return Err(DecodeError::InvalidTag),
    }
    Ok(())
}

fn bench_payload(c: &mut Criterion, label: &str, bytes: &[u8]) {
    let eager = Address::<Global>::decode(bytes).unwrap();
    let lazy = AddressLazy::decode(bytes).unwrap();
    let view = AddressView::parse(bytes).unwrap();
    // Promote lazy strings before the steady-state reads.
    black_box(lazy_all(&lazy));

    let mut group = c.benchmark_group(label);
    group.throughput(Throughput::Bytes(bytes.len() as u64));

    group.bench_function("eager_decode", |b| {
        b.iter(|| Address::<Global>::decode(black_box(bytes)).unwrap())
    });
    group.bench_function("eager_decode_keep", |b| {
        iter_keep(b, || Address::<Global>::decode(black_box(bytes)).unwrap())
    });
    group.bench_function("eager_decode_read_street", |b| {
        b.iter(|| {
            let msg = Address::<Global>::decode(black_box(bytes)).unwrap();
            eager_street(black_box(&msg))
        })
    });
    group.bench_function("eager_decode_read_postal", |b| {
        b.iter(|| {
            let msg = Address::<Global>::decode(black_box(bytes)).unwrap();
            eager_postal(black_box(&msg))
        })
    });
    group.bench_function("eager_read_street", |b| {
        b.iter(|| eager_street(black_box(&eager)))
    });
    group.bench_function("eager_read_all", |b| {
        b.iter(|| eager_all(black_box(&eager)))
    });

    group.bench_function("lazy_decode", |b| {
        b.iter(|| AddressLazy::decode(black_box(bytes)).unwrap())
    });
    group.bench_function("lazy_decode_keep", |b| {
        iter_keep(b, || AddressLazy::decode(black_box(bytes)).unwrap())
    });
    group.bench_function("lazy_decode_read_street", |b| {
        b.iter(|| {
            let msg = AddressLazy::decode(black_box(bytes)).unwrap();
            lazy_street(black_box(&msg))
        })
    });
    group.bench_function("lazy_decode_read_postal", |b| {
        b.iter(|| {
            let msg = AddressLazy::decode(black_box(bytes)).unwrap();
            lazy_postal(black_box(&msg))
        })
    });
    group.bench_function("lazy_read_street", |b| {
        b.iter(|| lazy_street(black_box(&lazy)))
    });
    group.bench_function("lazy_read_all", |b| b.iter(|| lazy_all(black_box(&lazy))));

    group.bench_function("view_parse", |b| {
        b.iter(|| AddressView::parse(black_box(bytes)).unwrap())
    });
    group.bench_function("view_parse_read_street", |b| {
        b.iter(|| {
            let view = AddressView::parse(black_box(bytes)).unwrap();
            view_street(black_box(&view))
        })
    });
    group.bench_function("view_parse_read_postal", |b| {
        b.iter(|| {
            let view = AddressView::parse(black_box(bytes)).unwrap();
            view_postal(black_box(&view))
        })
    });
    group.bench_function("view_read_street", |b| {
        b.iter(|| view_street(black_box(&view)))
    });
    group.bench_function("view_read_all", |b| b.iter(|| view_all(black_box(&view))));

    group.bench_function("rescan_street", |b| {
        b.iter(|| rescan_street(black_box(bytes)))
    });
    group.bench_function("rescan_all", |b| b.iter(|| rescan_all(black_box(bytes))));

    group.finish();
}

fn bench_short(c: &mut Criterion) {
    bench_payload(c, "address_view_short", SHORT_BYTES.as_slice());
}

fn bench_long(c: &mut Criterion) {
    bench_payload(c, "address_view_long", LONG_BYTES.as_slice());
}

fn short_config() -> Criterion {
    Criterion::default()
        .sample_size(20)
        .warm_up_time(Duration::from_millis(200))
        .measurement_time(Duration::from_secs(1))
}

criterion_group! {
    name = benches;
    config = short_config();
    targets = bench_short, bench_long
}
criterion_main!(benches);

/// Local copy of `puroro/benches/keep.rs` so this archive does not import the
/// product bench module.
mod keep {
    use ::core::time::Duration;
    use ::criterion::Bencher;
    use ::criterion::measurement::WallTime;
    use ::std::time::Instant;

    const KEEP_BATCH: usize = 1024;

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
}
