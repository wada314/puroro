//! Varint encode / decode microbenchmarks (baseline before fast-path work).
//!
//! Encode uses the same shape as `puroro-rt` (`Varint::encode` + `put_slice`).
//! Decode benches monomorphize over each [`puroro_rt::decode::VarintDecoder`]
//! (via [`puroro_rt::for_each_varint_decoder`]) so algorithms can be compared in
//! one run.
//!
//! Inputs are generated once before the benches. Skewed cases sample from an
//! exponential distribution calibrated so that
//! `CDF(2^threshold_bits) == threshold_probability` (same construction as the
//! historical `puroro/benches/variant.rs`). The uniform case samples `u64`
//! directly. `varint_decode_batch_win4` reuses `mostly_1_4byte` wire but feeds
//! an item-windowed `Buf` (next 4 varints per `chunk`, natural wire lengths).
//! `tags_mostly_1byte` uses tag-shaped values (`field_number << 3 | wire_type`).
//!
//! Run:
//! ```text
//! cargo bench -p puroro --bench varint
//! cargo bench -p puroro --bench varint -- --test   # smoke only
//! ```

use ::bytes::{Buf, BufMut};
use ::criterion::{BenchmarkId, Criterion, Throughput, black_box};
use ::protobuf_core::Varint;
use ::puroro_rt::decode::VarintDecoder;
use ::puroro_rt::for_each_varint_decoder;
use ::rand::rngs::SmallRng;
use ::rand::{Rng, SeedableRng};
use ::rand_distr::{Distribution, Exp};

const BATCH_LEN: usize = 1000;

/// How many consecutive varints are visible in one [`ItemWindowedBuf`] chunk.
const WINDOW_ITEMS: usize = 4;

/// Fixed seed so repeated `cargo bench` runs are comparable.
const INPUT_SEED: u64 = 0x0070_7572_6f72_6f76;

/// How values are drawn for one named scenario.
#[derive(Clone, Copy)]
enum ValueDist {
    /// Exponential on `[0, ∞)`, cast to `u64`, with
    /// `P(X < 2^threshold_bits) == threshold_probability`.
    Exponential {
        threshold_bits: u32,
        threshold_probability: f64,
    },
    /// Uniform over the full `u64` domain.
    UniformU64,
    /// Tag-shaped values: `(field_number << 3) | wire_type`.
    ///
    /// With probability `single_byte_field_probability`, `field_number` is in
    /// `1..=15` (1-byte tags). Otherwise `16..=2047` (2-byte tags).
    TagsMostly1Byte { single_byte_field_probability: f64 },
}

#[derive(Clone, Copy)]
struct Scenario {
    label: &'static str,
    dist: ValueDist,
}

/// Valid protobuf wire-type ids used when synthesizing tags.
const TAG_WIRE_TYPES: [u8; 6] = [0, 1, 2, 3, 4, 5];

/// Realistic-ish workloads plus an unbiased stress case.
const SCENARIOS: &[Scenario] = &[
    Scenario {
        label: "mostly_1byte",
        // ~95% of mass below 2^7 → typically 1-byte varints.
        dist: ValueDist::Exponential {
            threshold_bits: 7,
            threshold_probability: 0.95,
        },
    },
    Scenario {
        label: "mostly_1_2byte",
        // ~95% below 2^14 → typically 1–2 byte varints.
        dist: ValueDist::Exponential {
            threshold_bits: 14,
            threshold_probability: 0.95,
        },
    },
    Scenario {
        label: "mostly_1_4byte",
        // ~95% below 2^28 → typically 1–4 byte varints.
        dist: ValueDist::Exponential {
            threshold_bits: 28,
            threshold_probability: 0.95,
        },
    },
    Scenario {
        label: "tags_mostly_1byte",
        // Field numbers 1..=15 → 1-byte tags (typical message layouts).
        dist: ValueDist::TagsMostly1Byte {
            single_byte_field_probability: 0.95,
        },
    },
    Scenario {
        label: "uniform_u64",
        dist: ValueDist::UniformU64,
    },
];

/// Pre-generated inputs for one scenario.
struct PreparedCase {
    label: &'static str,
    values: Vec<u64>,
    /// Concatenated encodings of `values` (for batch decode / throughput).
    batch_wire: Vec<u8>,
    /// Encoded length of each value in `values` (for item-windowed decode).
    item_lens: Vec<usize>,
}

/// `Buf` that exposes only the next [`WINDOW_ITEMS`] varints' wire as `chunk()`.
///
/// Window ends follow natural varint boundaries (no forced word alignment /
/// misalignment). Lengths therefore vary with the encoded items.
struct ItemWindowedBuf<'a> {
    data: &'a [u8],
    /// Exclusive end offset of each varint in `data`.
    item_ends: &'a [usize],
    pos: usize,
    window_end: usize,
    /// Index of the first item whose end is strictly after `pos`.
    item_idx: usize,
}

impl<'a> ItemWindowedBuf<'a> {
    fn new(data: &'a [u8], item_ends: &'a [usize]) -> Self {
        let mut buf = Self {
            data,
            item_ends,
            pos: 0,
            window_end: 0,
            item_idx: 0,
        };
        buf.open_window();
        buf
    }

    fn open_window(&mut self) {
        if self.pos >= self.data.len() {
            self.window_end = self.pos;
            return;
        }
        while self.item_idx < self.item_ends.len() && self.item_ends[self.item_idx] <= self.pos {
            self.item_idx += 1;
        }
        debug_assert!(
            self.item_idx < self.item_ends.len(),
            "pos inside data but past all item ends"
        );
        let last_item = (self.item_idx + WINDOW_ITEMS - 1).min(self.item_ends.len() - 1);
        self.window_end = self.item_ends[last_item];
    }
}

impl Buf for ItemWindowedBuf<'_> {
    fn remaining(&self) -> usize {
        self.data.len() - self.pos
    }

    fn chunk(&self) -> &[u8] {
        &self.data[self.pos..self.window_end]
    }

    fn advance(&mut self, cnt: usize) {
        assert!(cnt <= self.remaining());
        let mut left = cnt;
        while left > 0 {
            if self.pos >= self.window_end {
                self.open_window();
            }
            let take = left.min(self.window_end - self.pos);
            self.pos += take;
            left -= take;
        }
        if self.pos >= self.window_end && self.pos < self.data.len() {
            self.open_window();
        }
    }
}

fn encode_one(value: u64) -> ([u8; 10], usize) {
    Varint::from_uint64(value).encode()
}

fn encode_one_to(value: u64, buf: &mut impl BufMut) {
    let (bytes, count) = Varint::from_uint64(value).encode();
    buf.put_slice(&bytes[..count]);
}

fn encode_batch(values: &[u64]) -> Vec<u8> {
    let mut out = Vec::with_capacity(values.len() * 10);
    for &v in values {
        encode_one_to(v, &mut out);
    }
    out
}

fn item_lens(values: &[u64]) -> Vec<usize> {
    values.iter().map(|&v| encode_one(v).1).collect()
}

fn item_ends(item_lens: &[usize]) -> Vec<usize> {
    let mut ends = Vec::with_capacity(item_lens.len());
    let mut acc = 0usize;
    for &len in item_lens {
        acc += len;
        ends.push(acc);
    }
    ends
}

fn decode_batch_with<D: VarintDecoder>(mut input: &[u8], out: &mut [u64]) {
    for slot in out.iter_mut() {
        *slot = D::decode_varint(&mut input).expect("varint decode");
    }
}

fn decode_batch_windowed_with<D: VarintDecoder>(wire: &[u8], item_ends: &[usize], out: &mut [u64]) {
    let mut input = ItemWindowedBuf::new(wire, item_ends);
    for slot in out.iter_mut() {
        *slot = D::decode_varint(&mut input).expect("varint decode");
    }
}

/// λ such that an Exp(λ) random variable satisfies `P(X < 2^b) = p`.
///
/// For Exp(λ), `CDF(x) = 1 - e^{-λx}`, so
/// `p = 1 - e^{-λ 2^b}` ⇒ `λ = -ln(1-p) / 2^b`.
fn exp_lambda(threshold_bits: u32, threshold_probability: f64) -> f64 {
    debug_assert!((0.0..1.0).contains(&threshold_probability));
    (-(1.0 - threshold_probability).ln()) / ((1u64 << threshold_bits) as f64)
}

fn gen_tag_raw(rng: &mut SmallRng, single_byte_field_probability: f64) -> u64 {
    let field = if rng.random::<f64>() < single_byte_field_probability {
        rng.random_range(1u32..=15)
    } else {
        rng.random_range(16u32..=2047)
    };
    let wire_type = TAG_WIRE_TYPES[rng.random_range(0..TAG_WIRE_TYPES.len())];
    u64::from((field << 3) | u32::from(wire_type))
}

fn gen_values(rng: &mut SmallRng, dist: ValueDist, n: usize) -> Vec<u64> {
    match dist {
        ValueDist::Exponential {
            threshold_bits,
            threshold_probability,
        } => {
            let exp = Exp::new(exp_lambda(threshold_bits, threshold_probability))
                .expect("positive lambda");
            (0..n).map(|_| exp.sample(rng) as u64).collect()
        }
        ValueDist::UniformU64 => (0..n).map(|_| rng.random::<u64>()).collect(),
        ValueDist::TagsMostly1Byte {
            single_byte_field_probability,
        } => (0..n)
            .map(|_| gen_tag_raw(rng, single_byte_field_probability))
            .collect(),
    }
}

fn prepare_inputs() -> Vec<PreparedCase> {
    let mut rng = SmallRng::seed_from_u64(INPUT_SEED);
    SCENARIOS
        .iter()
        .copied()
        .map(|scenario| {
            let values = gen_values(&mut rng, scenario.dist, BATCH_LEN);
            let batch_wire = encode_batch(&values);
            let item_lens = item_lens(&values);
            PreparedCase {
                label: scenario.label,
                values,
                batch_wire,
                item_lens,
            }
        })
        .collect()
}

fn validate_roundtrip(cases: &[PreparedCase]) {
    let mut decoded = vec![0u64; BATCH_LEN];
    for_each_varint_decoder!(D => {
        for case in cases {
            decode_batch_with::<D>(&case.batch_wire, &mut decoded);
            assert_eq!(
                decoded,
                case.values,
                "{} / {}: decode roundtrip failed",
                D::NAME,
                case.label
            );

            let ends = item_ends(&case.item_lens);
            decode_batch_windowed_with::<D>(&case.batch_wire, &ends, &mut decoded);
            assert_eq!(
                decoded,
                case.values,
                "{} / {}: windowed decode roundtrip failed",
                D::NAME,
                case.label
            );
        }
    });
}

fn bench_encode_single(c: &mut Criterion, cases: &[PreparedCase]) {
    let mut group = c.benchmark_group("varint_encode_single");
    for case in cases {
        // Per-call throughput ≈ average encoded size for this sample set.
        let avg_bytes = case.batch_wire.len() as u64 / BATCH_LEN as u64;
        group.throughput(Throughput::Bytes(avg_bytes.max(1)));
        group.bench_with_input(BenchmarkId::from_parameter(case.label), case, |b, case| {
            let mut i = 0usize;
            b.iter(|| {
                let v = case.values[i % case.values.len()];
                i = i.wrapping_add(1);
                black_box(encode_one(black_box(v)))
            })
        });
    }
    group.finish();
}

fn bench_decode_single(c: &mut Criterion, cases: &[PreparedCase]) {
    let mut group = c.benchmark_group("varint_decode_single");
    for_each_varint_decoder!(D => {
        for case in cases {
            let wires: Vec<Vec<u8>> = case
                .values
                .iter()
                .map(|&v| {
                    let (bytes, count) = encode_one(v);
                    bytes[..count].to_vec()
                })
                .collect();
            let avg_bytes = case.batch_wire.len() as u64 / BATCH_LEN as u64;
            group.throughput(Throughput::Bytes(avg_bytes.max(1)));
            group.bench_with_input(
                BenchmarkId::new(D::NAME, case.label),
                &wires,
                |b, wires| {
                    let mut i = 0usize;
                    b.iter(|| {
                        let wire = &wires[i % wires.len()];
                        i = i.wrapping_add(1);
                        let mut cursor = black_box(wire.as_slice());
                        black_box(D::decode_varint(&mut cursor).expect("varint decode"))
                    })
                },
            );
        }
    });
    group.finish();
}

fn bench_encode_batch(c: &mut Criterion, cases: &[PreparedCase]) {
    let mut group = c.benchmark_group("varint_encode_batch");
    for case in cases {
        group.throughput(Throughput::Bytes(case.batch_wire.len() as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(case.label),
            &case.values,
            |b, values| b.iter(|| black_box(encode_batch(black_box(values.as_slice())))),
        );
    }
    group.finish();
}

fn bench_decode_batch(c: &mut Criterion, cases: &[PreparedCase]) {
    let mut group = c.benchmark_group("varint_decode_batch");
    let mut out = vec![0u64; BATCH_LEN];

    for_each_varint_decoder!(D => {
        for case in cases {
            group.throughput(Throughput::Bytes(case.batch_wire.len() as u64));
            group.bench_with_input(
                BenchmarkId::new(D::NAME, case.label),
                &case.batch_wire,
                |b, wire| {
                    b.iter(|| {
                        decode_batch_with::<D>(black_box(wire.as_slice()), black_box(&mut out));
                        black_box(&out);
                    })
                },
            );
        }
    });
    group.finish();
}

/// Same wire as `mostly_1_4byte`, but `Buf::chunk` only shows the next 4
/// varints at a time (natural lengths). Stresses short remainders / HotAdapt.
fn bench_decode_batch_windowed(c: &mut Criterion, cases: &[PreparedCase]) {
    let Some(case) = cases.iter().find(|c| c.label == "mostly_1_4byte") else {
        return;
    };
    let ends = item_ends(&case.item_lens);
    let mut group = c.benchmark_group("varint_decode_batch_win4");
    let mut out = vec![0u64; BATCH_LEN];

    for_each_varint_decoder!(D => {
        group.throughput(Throughput::Bytes(case.batch_wire.len() as u64));
        group.bench_with_input(
            BenchmarkId::new(D::NAME, "mostly_1_4byte"),
            &(case.batch_wire.as_slice(), ends.as_slice()),
            |b, &(wire, ends)| {
                b.iter(|| {
                    decode_batch_windowed_with::<D>(
                        black_box(wire),
                        black_box(ends),
                        black_box(&mut out),
                    );
                    black_box(&out);
                })
            },
        );
    });
    group.finish();
}

fn main() {
    let cases = prepare_inputs();
    validate_roundtrip(&cases);

    let mut c = Criterion::default().configure_from_args();
    bench_encode_single(&mut c, &cases);
    bench_decode_single(&mut c, &cases);
    bench_encode_batch(&mut c, &cases);
    bench_decode_batch(&mut c, &cases);
    bench_decode_batch_windowed(&mut c, &cases);
    c.final_summary();
}
