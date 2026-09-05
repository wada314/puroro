//! `Global` vs bumpalo arena (`A = &Bump`) on the eager path.
//!
//! Both sides use [`Message::new_in`] + [`Message::merge_from`] so the API path
//! matches. The bump case drops the message (field `deallocate` is a no-op) and
//! then [`Bump::reset`]s, which is the [DESIGN.md](../../DESIGN.md) arena
//! pattern. Encode is omitted: [`Message::encode_to_vec`] always allocates the
//! output `Vec` on `Global`.
//!
//! Workloads reuse `ser_de` messages. `flat_scalars` / `short_strings` are
//! controls (little or no heap); `nested` / `strings` / `packed_ints` allocate.
//!
//! Run:
//! ```text
//! cargo bench -p puroro --bench allocator
//! cargo bench -p puroro --bench allocator -- --test
//! ```

#[path = "../ser_de/messages.rs"]
mod messages;
#[path = "../ser_de/payloads.rs"]
mod payloads;

use ::allocator_api2::alloc::Global;
use ::bumpalo::Bump;
use ::criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group};
use ::puroro::Message;

use messages::{FlatScalars, Nest, PackedInts, ShortStrings, StringHeavy};
use payloads::{
    FLAT_BYTES, NEST_BYTES, NEST_DEPTH, PACKED_BYTES, PACKED_LEN, SHORT_STRINGS_BYTES,
    STRINGS_BYTES,
};

fn decode_in<M>(alloc: M::Alloc, bytes: &[u8]) -> M
where
    M: Message,
    M::Alloc: Clone,
{
    let mut msg = M::new_in(alloc);
    let mut buf = bytes;
    msg.merge_from(&mut buf).expect("decode");
    msg
}

fn bench_decode_alloc(c: &mut Criterion) {
    let mut group = c.benchmark_group("decode_alloc");

    let bytes = FLAT_BYTES.as_slice();
    group.throughput(Throughput::Bytes(bytes.len() as u64));
    group.bench_function(BenchmarkId::new("flat_scalars", "global"), |b| {
        b.iter(|| decode_in::<FlatScalars<Global>>(Global, black_box(bytes)))
    });
    {
        let mut bump = Bump::new();
        group.bench_function(BenchmarkId::new("flat_scalars", "bump"), |b| {
            b.iter(|| {
                {
                    let msg = decode_in::<FlatScalars<_>>(&bump, black_box(bytes));
                    black_box(&msg);
                }
                bump.reset();
            })
        });
    }

    let bytes = NEST_BYTES.as_slice();
    group.throughput(Throughput::Bytes(bytes.len() as u64));
    group.bench_function(BenchmarkId::new("nested", "global"), |b| {
        b.iter(|| decode_in::<Nest<Global>>(Global, black_box(bytes)))
    });
    {
        let mut bump = Bump::new();
        group.bench_function(BenchmarkId::new("nested", "bump"), |b| {
            b.iter(|| {
                {
                    let msg = decode_in::<Nest<_>>(&bump, black_box(bytes));
                    black_box(&msg);
                }
                bump.reset();
            })
        });
    }

    let bytes = PACKED_BYTES.as_slice();
    group.throughput(Throughput::Bytes(bytes.len() as u64));
    group.bench_function(BenchmarkId::new("packed_ints", "global"), |b| {
        b.iter(|| decode_in::<PackedInts<Global>>(Global, black_box(bytes)))
    });
    {
        let mut bump = Bump::new();
        group.bench_function(BenchmarkId::new("packed_ints", "bump"), |b| {
            b.iter(|| {
                {
                    let msg = decode_in::<PackedInts<_>>(&bump, black_box(bytes));
                    black_box(&msg);
                }
                bump.reset();
            })
        });
    }

    let bytes = STRINGS_BYTES.as_slice();
    group.throughput(Throughput::Bytes(bytes.len() as u64));
    group.bench_function(BenchmarkId::new("strings", "global"), |b| {
        b.iter(|| decode_in::<StringHeavy<Global>>(Global, black_box(bytes)))
    });
    {
        let mut bump = Bump::new();
        group.bench_function(BenchmarkId::new("strings", "bump"), |b| {
            b.iter(|| {
                {
                    let msg = decode_in::<StringHeavy<_>>(&bump, black_box(bytes));
                    black_box(&msg);
                }
                bump.reset();
            })
        });
    }

    let bytes = SHORT_STRINGS_BYTES.as_slice();
    group.throughput(Throughput::Bytes(bytes.len() as u64));
    group.bench_function(BenchmarkId::new("short_strings", "global"), |b| {
        b.iter(|| decode_in::<ShortStrings<Global>>(Global, black_box(bytes)))
    });
    {
        let mut bump = Bump::new();
        group.bench_function(BenchmarkId::new("short_strings", "bump"), |b| {
            b.iter(|| {
                {
                    let msg = decode_in::<ShortStrings<_>>(&bump, black_box(bytes));
                    black_box(&msg);
                }
                bump.reset();
            })
        });
    }

    group.finish();
}

fn bench_construct_alloc(c: &mut Criterion) {
    let mut group = c.benchmark_group("construct_alloc");

    group.throughput(Throughput::Elements(1));
    group.bench_function(BenchmarkId::new("flat_scalars", "global"), |b| {
        b.iter(|| FlatScalars::sample_in(Global))
    });
    {
        let mut bump = Bump::new();
        group.bench_function(BenchmarkId::new("flat_scalars", "bump"), |b| {
            b.iter(|| {
                {
                    let msg = FlatScalars::sample_in(&bump);
                    black_box(&msg);
                }
                bump.reset();
            })
        });
    }

    group.throughput(Throughput::Elements(NEST_DEPTH as u64));
    group.bench_function(BenchmarkId::new("nested", "global"), |b| {
        b.iter(|| Nest::sample_in(Global, NEST_DEPTH))
    });
    {
        let mut bump = Bump::new();
        group.bench_function(BenchmarkId::new("nested", "bump"), |b| {
            b.iter(|| {
                {
                    let msg = Nest::sample_in(&bump, NEST_DEPTH);
                    black_box(&msg);
                }
                bump.reset();
            })
        });
    }

    group.throughput(Throughput::Elements(PACKED_LEN as u64));
    group.bench_function(BenchmarkId::new("packed_ints", "global"), |b| {
        b.iter(|| PackedInts::sample_in(Global, PACKED_LEN))
    });
    {
        let mut bump = Bump::new();
        group.bench_function(BenchmarkId::new("packed_ints", "bump"), |b| {
            b.iter(|| {
                {
                    let msg = PackedInts::sample_in(&bump, PACKED_LEN);
                    black_box(&msg);
                }
                bump.reset();
            })
        });
    }

    group.throughput(Throughput::Elements(1));
    group.bench_function(BenchmarkId::new("strings", "global"), |b| {
        b.iter(|| StringHeavy::sample_in(Global))
    });
    {
        let mut bump = Bump::new();
        group.bench_function(BenchmarkId::new("strings", "bump"), |b| {
            b.iter(|| {
                {
                    let msg = StringHeavy::sample_in(&bump);
                    black_box(&msg);
                }
                bump.reset();
            })
        });
    }

    group.bench_function(BenchmarkId::new("short_strings", "global"), |b| {
        b.iter(|| ShortStrings::sample_in(Global))
    });
    {
        let mut bump = Bump::new();
        group.bench_function(BenchmarkId::new("short_strings", "bump"), |b| {
            b.iter(|| {
                {
                    let msg = ShortStrings::sample_in(&bump);
                    black_box(&msg);
                }
                bump.reset();
            })
        });
    }

    group.finish();
}

fn assert_bump_roundtrips() {
    let bump = Bump::new();
    assert_eq!(
        decode_in::<FlatScalars<_>>(&bump, FLAT_BYTES.as_slice()).encode_to_vec(),
        *FLAT_BYTES
    );
    assert_eq!(
        decode_in::<Nest<_>>(&bump, NEST_BYTES.as_slice()).encode_to_vec(),
        *NEST_BYTES
    );
    assert_eq!(
        decode_in::<PackedInts<_>>(&bump, PACKED_BYTES.as_slice()).encode_to_vec(),
        *PACKED_BYTES
    );
    assert_eq!(
        decode_in::<StringHeavy<_>>(&bump, STRINGS_BYTES.as_slice()).encode_to_vec(),
        *STRINGS_BYTES
    );
    assert_eq!(
        decode_in::<ShortStrings<_>>(&bump, SHORT_STRINGS_BYTES.as_slice()).encode_to_vec(),
        *SHORT_STRINGS_BYTES
    );
}

criterion_group!(benches, bench_decode_alloc, bench_construct_alloc);

fn main() {
    payloads::assert_roundtrips();
    assert_bump_roundtrips();
    benches();
}
