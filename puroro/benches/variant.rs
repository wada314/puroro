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

fn read_variants<const SIZE: usize>(mut input: &[u8]) -> [Variant; SIZE] {
    array::from_fn(|_| <&[u8] as ReadExt>::read_variant(&mut input).unwrap())
}

fn read_variants_assume_4<const SIZE: usize>(mut input: &[u8]) -> [Variant; SIZE] {
    array::from_fn(|_| <&[u8] as BufReadExt>::read_variant_assume_4(&mut input).unwrap())
}

fn bench_read_variants(criterion: &mut Criterion) {
    const SIZE: usize = 1000;
    let input_box: Box<[u8]> = test_cases_random::<SIZE>();
    let input: &[u8] = &input_box;

    let mut group = criterion.benchmark_group("read random variants");
    group.bench_function("default read", |b| {
        b.iter(|| read_variants::<SIZE>(black_box(input)))
    });
    group.bench_function("assume 4", |b| {
        b.iter(|| read_variants_assume_4::<SIZE>(black_box(input)))
    });
    group.finish();
}

criterion_group!(benches, bench_read_variants);
criterion_main!(benches);
