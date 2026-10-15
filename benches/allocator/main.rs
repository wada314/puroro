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
//! Construct / bump loops pass the message through [`black_box`] by value so
//! LLVM cannot skip the stores (a `&msg` barrier is not enough).
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

use messages::{FlatScalars, Nest, PackedInts, ShortStrings, StringHeavy, Wrap, WrapInline};
use payloads::{
    FLAT_BYTES, NEST_BYTES, NEST_DEPTH, PACKED_BYTES, PACKED_LEN, SHORT_STRINGS_BYTES,
    STRINGS_BYTES, WRAP_BYTES,
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
                black_box(decode_in::<FlatScalars<_>>(&bump, black_box(bytes)));
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
                black_box(decode_in::<Nest<_>>(&bump, black_box(bytes)));
                bump.reset();
            })
        });
    }

    let bytes = WRAP_BYTES.as_slice();
    group.throughput(Throughput::Bytes(bytes.len() as u64));
    group.bench_function(BenchmarkId::new("wrap", "global"), |b| {
        b.iter(|| decode_in::<Wrap<Global>>(Global, black_box(bytes)))
    });
    {
        let mut bump = Bump::new();
        group.bench_function(BenchmarkId::new("wrap", "bump"), |b| {
            b.iter(|| {
                black_box(decode_in::<Wrap<_>>(&bump, black_box(bytes)));
                bump.reset();
            })
        });
    }
    group.bench_function(BenchmarkId::new("wrap_inline", "global"), |b| {
        b.iter(|| decode_in::<WrapInline<Global>>(Global, black_box(bytes)))
    });
    {
        let mut bump = Bump::new();
        group.bench_function(BenchmarkId::new("wrap_inline", "bump"), |b| {
            b.iter(|| {
                black_box(decode_in::<WrapInline<_>>(&bump, black_box(bytes)));
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
                black_box(decode_in::<PackedInts<_>>(&bump, black_box(bytes)));
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
                black_box(decode_in::<StringHeavy<_>>(&bump, black_box(bytes)));
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
                black_box(decode_in::<ShortStrings<_>>(&bump, black_box(bytes)));
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
        b.iter(|| black_box(FlatScalars::sample_in(Global)))
    });
    {
        let mut bump = Bump::new();
        group.bench_function(BenchmarkId::new("flat_scalars", "bump"), |b| {
            b.iter(|| {
                black_box(FlatScalars::sample_in(&bump));
                bump.reset();
            })
        });
    }

    group.throughput(Throughput::Elements(NEST_DEPTH as u64));
    group.bench_function(BenchmarkId::new("nested", "global"), |b| {
        b.iter(|| black_box(Nest::sample_in(Global, NEST_DEPTH)))
    });
    {
        let mut bump = Bump::new();
        group.bench_function(BenchmarkId::new("nested", "bump"), |b| {
            b.iter(|| {
                black_box(Nest::sample_in(&bump, NEST_DEPTH));
                bump.reset();
            })
        });
    }

    group.throughput(Throughput::Elements(1));
    group.bench_function(BenchmarkId::new("wrap", "global"), |b| {
        b.iter(|| black_box(Wrap::sample_in(Global)))
    });
    {
        let mut bump = Bump::new();
        group.bench_function(BenchmarkId::new("wrap", "bump"), |b| {
            b.iter(|| {
                black_box(Wrap::sample_in(&bump));
                bump.reset();
            })
        });
    }
    group.bench_function(BenchmarkId::new("wrap_inline", "global"), |b| {
        b.iter(|| black_box(WrapInline::sample_in(Global)))
    });
    {
        let mut bump = Bump::new();
        group.bench_function(BenchmarkId::new("wrap_inline", "bump"), |b| {
            b.iter(|| {
                black_box(WrapInline::sample_in(&bump));
                bump.reset();
            })
        });
    }

    group.throughput(Throughput::Elements(PACKED_LEN as u64));
    group.bench_function(BenchmarkId::new("packed_ints", "global"), |b| {
        b.iter(|| black_box(PackedInts::sample_in(Global, PACKED_LEN)))
    });
    {
        let mut bump = Bump::new();
        group.bench_function(BenchmarkId::new("packed_ints", "bump"), |b| {
            b.iter(|| {
                black_box(PackedInts::sample_in(&bump, PACKED_LEN));
                bump.reset();
            })
        });
    }

    group.throughput(Throughput::Elements(1));
    group.bench_function(BenchmarkId::new("strings", "global"), |b| {
        b.iter(|| black_box(StringHeavy::sample_in(Global)))
    });
    {
        let mut bump = Bump::new();
        group.bench_function(BenchmarkId::new("strings", "bump"), |b| {
            b.iter(|| {
                black_box(StringHeavy::sample_in(&bump));
                bump.reset();
            })
        });
    }

    group.bench_function(BenchmarkId::new("short_strings", "global"), |b| {
        b.iter(|| black_box(ShortStrings::sample_in(Global)))
    });
    {
        let mut bump = Bump::new();
        group.bench_function(BenchmarkId::new("short_strings", "bump"), |b| {
            b.iter(|| {
                black_box(ShortStrings::sample_in(&bump));
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
        decode_in::<Wrap<_>>(&bump, WRAP_BYTES.as_slice()).encode_to_vec(),
        *WRAP_BYTES
    );
    assert_eq!(
        decode_in::<WrapInline<_>>(&bump, WRAP_BYTES.as_slice()).encode_to_vec(),
        *WRAP_BYTES
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
