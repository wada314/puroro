//! Generated-path microbenchmarks (`sample-generated` `Task` / `Point` / `Marker`).
//!
//! Covers the three eager-path holes that `ser_de` does not:
//! - decode a realistic generated message, then read one field vs all fields
//! - `map<string, int32>` encode / decode / lookup
//! - unknown-field preserve (`Point`) vs discard (`Marker`)
//!
//! Run:
//! ```text
//! cargo bench -p puroro --bench generated
//! cargo bench -p puroro --bench generated -- --test
//! ```

mod payloads;

use ::allocator_api2::alloc::Global;
use ::criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group};
use ::puroro::Message;
use ::puroro_sample_generated::{Marker, Point, Task};

use payloads::{
    MAP_BYTES, MAP_LEN, TYPICAL_BYTES, UNKNOWN_BYTES, UNKNOWN_LEN, lookup_map, map_heavy_task,
    read_all, read_one, typical_task,
};

fn bench_task(c: &mut Criterion) {
    let mut group = c.benchmark_group("task");
    group.throughput(Throughput::Bytes(TYPICAL_BYTES.len() as u64));
    let bytes = TYPICAL_BYTES.as_slice();
    let encoded = typical_task();
    let decoded = Task::<Global>::decode(bytes).expect("typical decode");

    group.bench_function("decode", |b| {
        b.iter(|| Task::<Global>::decode(black_box(bytes)).unwrap())
    });
    group.bench_function("decode_read_one", |b| {
        b.iter(|| {
            let task = Task::<Global>::decode(black_box(bytes)).unwrap();
            read_one(black_box(&task))
        })
    });
    group.bench_function("decode_read_all", |b| {
        b.iter(|| {
            let task = Task::<Global>::decode(black_box(bytes)).unwrap();
            read_all(black_box(&task))
        })
    });
    group.bench_function("read_one", |b| b.iter(|| read_one(black_box(&decoded))));
    group.bench_function("read_all", |b| b.iter(|| read_all(black_box(&decoded))));
    group.bench_function("encode", |b| b.iter(|| black_box(&encoded).encode_to_vec()));
    group.finish();
}

fn bench_map(c: &mut Criterion) {
    let mut group = c.benchmark_group("map");
    group.throughput(Throughput::Elements(MAP_LEN as u64));
    let bytes = MAP_BYTES.as_slice();
    let encoded = map_heavy_task();
    let decoded = Task::<Global>::decode(bytes).expect("map decode");

    group.bench_function("decode", |b| {
        b.iter(|| Task::<Global>::decode(black_box(bytes)).unwrap())
    });
    group.bench_function("encode", |b| b.iter(|| black_box(&encoded).encode_to_vec()));
    group.bench_function("lookup", |b| b.iter(|| lookup_map(black_box(&decoded))));
    group.finish();
}

fn bench_unknown(c: &mut Criterion) {
    let mut group = c.benchmark_group("unknown");
    group.throughput(Throughput::Elements(UNKNOWN_LEN as u64));
    let bytes = UNKNOWN_BYTES.as_slice();
    let preserved = Point::<Global>::decode(bytes).expect("point decode");
    let discarded = Marker::<Global>::decode(bytes).expect("marker decode");

    group.bench_function(BenchmarkId::new("decode", "preserve"), |b| {
        b.iter(|| Point::<Global>::decode(black_box(bytes)).unwrap())
    });
    group.bench_function(BenchmarkId::new("decode", "discard"), |b| {
        b.iter(|| Marker::<Global>::decode(black_box(bytes)).unwrap())
    });
    group.bench_function(BenchmarkId::new("encode", "preserve"), |b| {
        b.iter(|| black_box(&preserved).encode_to_vec())
    });
    group.bench_function(BenchmarkId::new("encode", "discard"), |b| {
        b.iter(|| black_box(&discarded).encode_to_vec())
    });
    group.finish();
}

criterion_group!(benches, bench_task, bench_map, bench_unknown);

fn main() {
    payloads::assert_payloads();
    benches();
}
