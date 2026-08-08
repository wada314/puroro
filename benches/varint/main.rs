//! Varint encode / decode microbenchmarks (baseline before fast-path work).
//!
//! Encode uses the same shape as `puroro-rt` (`Varint::encode` + `put_slice`).
//! Decode calls [`puroro_rt::decode::decode_varint`] directly.
//!
//! Inputs are generated once before the benches. Skewed cases sample from an
//! exponential distribution calibrated so that
//! `CDF(2^threshold_bits) == threshold_probability` (same construction as the
//! historical `puroro/benches/variant.rs`). The uniform case samples `u64`
//! directly.
//!
//! Run:
//! ```text
//! cargo bench -p puroro --bench varint
//! cargo bench -p puroro --bench varint -- --test   # smoke only
//! ```

use ::bytes::{Buf, BufMut};
use ::criterion::{BenchmarkId, Criterion, Throughput, black_box};
use ::protobuf_core::Varint;
use ::puroro_rt::decode::decode_varint;
use ::rand::rngs::SmallRng;
use ::rand::{Rng, SeedableRng};
use ::rand_distr::{Distribution, Exp};

const BATCH_LEN: usize = 1000;

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
}

#[derive(Clone, Copy)]
struct Scenario {
    label: &'static str,
    dist: ValueDist,
}

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
}

fn encode_one(value: u64) -> ([u8; 10], usize) {
    Varint::from_uint64(value).encode()
}

fn encode_one_to(value: u64, buf: &mut impl BufMut) {
    let (bytes, count) = Varint::from_uint64(value).encode();
    buf.put_slice(&bytes[..count]);
}

fn decode_one(buf: &mut impl Buf) -> u64 {
    decode_varint(buf).expect("varint decode")
}

fn encode_batch(values: &[u64]) -> Vec<u8> {
    let mut out = Vec::with_capacity(values.len() * 10);
    for &v in values {
        encode_one_to(v, &mut out);
    }
    out
}

fn decode_batch(mut input: &[u8], out: &mut [u64]) {
    for slot in out.iter_mut() {
        *slot = decode_one(&mut input);
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
            PreparedCase {
                label: scenario.label,
                values,
                batch_wire,
            }
        })
        .collect()
}

fn validate_roundtrip(cases: &[PreparedCase]) {
    let mut decoded = vec![0u64; BATCH_LEN];
    for case in cases {
        decode_batch(&case.batch_wire, &mut decoded);
        assert_eq!(
            decoded, case.values,
            "{}: decode roundtrip failed",
            case.label
        );
    }
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
            BenchmarkId::from_parameter(case.label),
            &wires,
            |b, wires| {
                let mut i = 0usize;
                b.iter(|| {
                    let wire = &wires[i % wires.len()];
                    i = i.wrapping_add(1);
                    let mut cursor = black_box(wire.as_slice());
                    black_box(decode_one(&mut cursor))
                })
            },
        );
    }
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

    for case in cases {
        group.throughput(Throughput::Bytes(case.batch_wire.len() as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(case.label),
            &case.batch_wire,
            |b, wire| {
                b.iter(|| {
                    decode_batch(black_box(wire.as_slice()), black_box(&mut out));
                    black_box(&out);
                })
            },
        );
    }
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
    c.final_summary();
}
