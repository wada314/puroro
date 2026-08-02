//! Serialize / deserialize microbenchmarks.
//!
//! Run:
//! ```text
//! cargo bench -p puroro
//! cargo bench -p puroro -- --test   # smoke only
//! ```

mod messages;
mod payloads;

use ::criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group};
use ::puroro::Message;

use messages::{FlatScalars, Nest, PackedInts, StringHeavy};
use payloads::{FLAT_BYTES, NEST_BYTES, NEST_DEPTH, PACKED_BYTES, PACKED_LEN, STRINGS_BYTES};

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
    group.finish();
}

criterion_group!(benches, bench_encode, bench_decode);

fn main() {
    payloads::assert_roundtrips();
    benches();
}
