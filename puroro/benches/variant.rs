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
use ::puroro::internal::variant::{BufReadExtVariant, ReadExtVariant, Variant, WriteExtVariant};
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
        WriteExtVariant::write_variant(&mut output, item_var).unwrap();
    }
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
        WriteExtVariant::write_variant(&mut output, item_var).unwrap();
    }
    output.into_boxed_slice()
}

fn read_variants<const SIZE: usize>(mut input: &[u8]) -> [Variant; SIZE] {
    array::from_fn(|_| <&[u8] as ReadExtVariant>::read_variant(&mut input).unwrap())
}

fn read_variants_peek_10<const SIZE: usize>(mut input: &[u8]) -> [Variant; SIZE] {
    array::from_fn(|_| <&[u8] as BufReadExtVariant>::read_variant_peek_10(&mut input).unwrap())
}

fn read_variants_assume_4<const SIZE: usize>(mut input: &[u8]) -> [Variant; SIZE] {
    array::from_fn(|_| <&[u8] as BufReadExtVariant>::read_variant_assume_4(&mut input).unwrap())
}

fn read_variants_assume_2<const SIZE: usize>(mut input: &[u8]) -> [Variant; SIZE] {
    array::from_fn(|_| <&[u8] as BufReadExtVariant>::read_variant_assume_2(&mut input).unwrap())
}

fn bench_read_variants(criterion: &mut Criterion) {
    const SIZE: usize = 1000;
    let input_rand = test_cases_random::<SIZE>();
    let input_28bits = test_cases_exponential_dist::<SIZE>(28, 99.0);
    let input_14bits = test_cases_exponential_dist::<SIZE>(14, 99.0);

    let target_functions: &[(&str, fn(&[u8]) -> _)] = &[
        ("read Bytes", read_variants::<SIZE>),
        ("peek 10", read_variants_peek_10::<SIZE>),
        ("assume 4", read_variants_assume_4::<SIZE>),
        ("assume 2", read_variants_assume_2::<SIZE>),
    ];
    let input_bytes: &[(&str, &[u8])] = &[
        ("random variants", &input_rand),
        ("99% < 2^28", &input_28bits),
        ("99% < 2^14", &input_14bits),
    ];

    for &(input_name, input) in input_bytes {
        let mut group = criterion.benchmark_group(input_name);
        for &(target_name, target) in target_functions {
            group.bench_function(target_name, |b| b.iter(|| target(black_box(input))));
        }
        group.finish();
    }
}

criterion_group!(benches, bench_read_variants);
criterion_main!(benches);
