// Copyright 2021 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use ::criterion::{black_box, criterion_group, criterion_main, Criterion};
use ::puroro::internal::variant::{BufReadExt, ReadExt, Variant, WriteExt};
use ::rand::prelude::*;
use ::rand_distr::Exp;
use ::rand_pcg::Pcg32;
use ::std::array;
use ::std::iter;

fn test_cases_random<const SIZE: usize>() -> Box<[u8]> {
    let mut rand = Pcg32::seed_from_u64(1234567890u64);
    let mut output = Vec::with_capacity(10 * SIZE);
    for item_u64 in iter::repeat_with(|| rand.gen::<u64>()).take(SIZE) {
        let item_var = Variant::from(item_u64);
        WriteExt::write_variant(&mut output, item_var).unwrap();
    }
    dbg!(output.len());
    output.into_boxed_slice()
}

// The values are genereted by exponential distribution, where:
// `CDF(2^threshold_bits) == threshold_percentile`.
fn test_cases_exponential_dist<const SIZE: usize>(
    threshold_bits: usize,
    threshold_percentile: f64,
) -> Box<[u8]> {
    let mut rand = Pcg32::seed_from_u64(1234567890u64);
    let mut output = Vec::with_capacity(10 * SIZE);
    let exp_distr =
        Exp::new(f64::ln(1.0 - (threshold_percentile / 100.0)) / -f64::exp2(threshold_bits as f64))
            .unwrap();
    for item_f64 in iter::repeat_with(|| exp_distr.sample(&mut rand)).take(SIZE) {
        let item_var = Variant::from(item_f64 as u64);
        WriteExt::write_variant(&mut output, item_var).unwrap();
    }
    dbg!(output.len());
    output.into_boxed_slice()
}

fn read_variants<const SIZE: usize>(mut input: &[u8]) -> [Variant; SIZE] {
    array::from_fn(|_| <&[u8] as ReadExt>::read_variant(&mut input).unwrap())
}

fn read_variants_assume_4<const SIZE: usize>(mut input: &[u8]) -> [Variant; SIZE] {
    array::from_fn(|_| <&[u8] as BufReadExt>::read_variant_assume_4(&mut input).unwrap())
}

fn read_variants_assume_2<const SIZE: usize>(mut input: &[u8]) -> [Variant; SIZE] {
    array::from_fn(|_| <&[u8] as BufReadExt>::read_variant_assume_2(&mut input).unwrap())
}

fn bench_read_variants(criterion: &mut Criterion) {
    const SIZE: usize = 1000;
    let input_rand_box = test_cases_random::<SIZE>();
    let input_rand: &[u8] = &input_rand_box;
    let input_28bits_box = test_cases_exponential_dist::<SIZE>(28, 99.0);
    let input_28bits: &[u8] = &input_28bits_box;
    let input_14bits_box = test_cases_exponential_dist::<SIZE>(14, 99.0);
    let input_14bits: &[u8] = &input_14bits_box;

    let mut group = criterion.benchmark_group("read random variants");
    group.bench_function("default read", |b| {
        b.iter(|| read_variants::<SIZE>(black_box(input_rand)))
    });
    group.bench_function("assume 4", |b| {
        b.iter(|| read_variants_assume_4::<SIZE>(black_box(input_rand)))
    });
    group.bench_function("assume 2", |b| {
        b.iter(|| read_variants_assume_2::<SIZE>(black_box(input_rand)))
    });
    group.finish();

    let mut group = criterion.benchmark_group("read 99% <2^28");
    group.bench_function("default read", |b| {
        b.iter(|| read_variants::<SIZE>(black_box(input_28bits)))
    });
    group.bench_function("assume 4", |b| {
        b.iter(|| read_variants_assume_4::<SIZE>(black_box(input_28bits)))
    });
    group.bench_function("assume 2", |b| {
        b.iter(|| read_variants_assume_2::<SIZE>(black_box(input_28bits)))
    });
    group.finish();

    let mut group = criterion.benchmark_group("read 99% <2^14");
    group.bench_function("default read", |b| {
        b.iter(|| read_variants::<SIZE>(black_box(input_14bits)))
    });
    group.bench_function("assume 4", |b| {
        b.iter(|| read_variants_assume_4::<SIZE>(black_box(input_14bits)))
    });
    group.bench_function("assume 2", |b| {
        b.iter(|| read_variants_assume_2::<SIZE>(black_box(input_14bits)))
    });
    group.finish();
}

criterion_group!(benches, bench_read_variants);
criterion_main!(benches);
