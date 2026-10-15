//! Serialize / deserialize microbenchmarks (hand-written catalog messages).
//!
//! Isolates [`puroro_rt`] field-catalog encode / decode without generated
//! `visit_fields` dispatch. `encode` is `encode_to_vec` (length + alloc +
//! write). `encode_reuse` writes into a pre-sized buffer so small messages
//! are not dominated by `Vec` allocation.
//!
//! Run:
//! ```text
//! cargo bench -p puroro --bench ser_de
//! cargo bench -p puroro --bench ser_de -- --test   # smoke only
//! ```

mod messages;
mod payloads;

use ::criterion::{black_box, criterion_group, BenchmarkId, Criterion, Throughput};
use ::puroro::Message;

use messages::{FlatScalars, Nest, PackedInts, ShortStrings, StringHeavy};
use payloads::{
    FLAT_BYTES, NEST_BYTES, NEST_DEPTH, PACKED_BYTES, PACKED_LEN, SHORT_STRINGS_BYTES,
    STRINGS_BYTES,
};

fn bench_encode(c: &mut Criterion) {
    let mut group = c.benchmark_group("encode");

    let flat = FlatScalars::sample();
    group.throughput(Throughput::Bytes(FLAT_BYTES.len() as u64));
    group.bench_function(BenchmarkId::new("flat_scalars", FLAT_BYTES.len()), |b| {
        b.iter(|| black_box(&flat).encode_to_vec())
    });

    let nest = Nest::sample(NEST_DEPTH);
    group.throughput(Throughput::Bytes(NEST_BYTES.len() as u64));
    group.bench_function(BenchmarkId::new("nested", NEST_BYTES.len()), |b| {
        b.iter(|| black_box(&nest).encode_to_vec())
    });

    let packed = PackedInts::sample(PACKED_LEN);
    group.throughput(Throughput::Bytes(PACKED_BYTES.len() as u64));
    group.bench_function(BenchmarkId::new("packed_ints", PACKED_BYTES.len()), |b| {
        b.iter(|| black_box(&packed).encode_to_vec())
    });

    let strings = StringHeavy::sample();
    group.throughput(Throughput::Bytes(STRINGS_BYTES.len() as u64));
    group.bench_function(BenchmarkId::new("strings", STRINGS_BYTES.len()), |b| {
        b.iter(|| black_box(&strings).encode_to_vec())
    });

    let short = ShortStrings::sample();
    group.throughput(Throughput::Bytes(SHORT_STRINGS_BYTES.len() as u64));
    group.bench_function(
        BenchmarkId::new("short_strings", SHORT_STRINGS_BYTES.len()),
        |b| b.iter(|| black_box(&short).encode_to_vec()),
    );
    group.finish();
}

fn bench_encode_reuse(c: &mut Criterion) {
    let mut group = c.benchmark_group("encode_reuse");

    let flat = FlatScalars::sample();
    group.throughput(Throughput::Bytes(FLAT_BYTES.len() as u64));
    group.bench_function(BenchmarkId::new("flat_scalars", FLAT_BYTES.len()), |b| {
        let mut buf = Vec::with_capacity(FLAT_BYTES.len());
        b.iter(|| {
            buf.clear();
            black_box(&flat).encode(&mut buf);
            black_box(buf.len())
        })
    });

    let nest = Nest::sample(NEST_DEPTH);
    group.throughput(Throughput::Bytes(NEST_BYTES.len() as u64));
    group.bench_function(BenchmarkId::new("nested", NEST_BYTES.len()), |b| {
        let mut buf = Vec::with_capacity(NEST_BYTES.len());
        b.iter(|| {
            buf.clear();
            black_box(&nest).encode(&mut buf);
            black_box(buf.len())
        })
    });

    let packed = PackedInts::sample(PACKED_LEN);
    group.throughput(Throughput::Bytes(PACKED_BYTES.len() as u64));
    group.bench_function(BenchmarkId::new("packed_ints", PACKED_BYTES.len()), |b| {
        let mut buf = Vec::with_capacity(PACKED_BYTES.len());
        b.iter(|| {
            buf.clear();
            black_box(&packed).encode(&mut buf);
            black_box(buf.len())
        })
    });

    let strings = StringHeavy::sample();
    group.throughput(Throughput::Bytes(STRINGS_BYTES.len() as u64));
    group.bench_function(BenchmarkId::new("strings", STRINGS_BYTES.len()), |b| {
        let mut buf = Vec::with_capacity(STRINGS_BYTES.len());
        b.iter(|| {
            buf.clear();
            black_box(&strings).encode(&mut buf);
            black_box(buf.len())
        })
    });

    let short = ShortStrings::sample();
    group.throughput(Throughput::Bytes(SHORT_STRINGS_BYTES.len() as u64));
    group.bench_function(
        BenchmarkId::new("short_strings", SHORT_STRINGS_BYTES.len()),
        |b| {
            let mut buf = Vec::with_capacity(SHORT_STRINGS_BYTES.len());
            b.iter(|| {
                buf.clear();
                black_box(&short).encode(&mut buf);
                black_box(buf.len())
            })
        },
    );
    group.finish();
}

fn bench_decode(c: &mut Criterion) {
    let mut group = c.benchmark_group("decode");

    group.throughput(Throughput::Bytes(FLAT_BYTES.len() as u64));
    group.bench_function(BenchmarkId::new("flat_scalars", FLAT_BYTES.len()), |b| {
        let bytes = FLAT_BYTES.as_slice();
        b.iter(|| FlatScalars::decode(black_box(bytes)).unwrap())
    });

    group.throughput(Throughput::Bytes(NEST_BYTES.len() as u64));
    group.bench_function(BenchmarkId::new("nested", NEST_BYTES.len()), |b| {
        let bytes = NEST_BYTES.as_slice();
        b.iter(|| Nest::decode(black_box(bytes)).unwrap())
    });

    group.throughput(Throughput::Bytes(PACKED_BYTES.len() as u64));
    group.bench_function(BenchmarkId::new("packed_ints", PACKED_BYTES.len()), |b| {
        let bytes = PACKED_BYTES.as_slice();
        b.iter(|| PackedInts::decode(black_box(bytes)).unwrap())
    });

    group.throughput(Throughput::Bytes(STRINGS_BYTES.len() as u64));
    group.bench_function(BenchmarkId::new("strings", STRINGS_BYTES.len()), |b| {
        let bytes = STRINGS_BYTES.as_slice();
        b.iter(|| StringHeavy::decode(black_box(bytes)).unwrap())
    });

    group.throughput(Throughput::Bytes(SHORT_STRINGS_BYTES.len() as u64));
    group.bench_function(
        BenchmarkId::new("short_strings", SHORT_STRINGS_BYTES.len()),
        |b| {
            let bytes = SHORT_STRINGS_BYTES.as_slice();
            b.iter(|| ShortStrings::decode(black_box(bytes)).unwrap())
        },
    );
    group.finish();
}

criterion_group!(benches, bench_encode, bench_encode_reuse, bench_decode);

fn main() {
    payloads::assert_roundtrips();
    benches();
}
