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
use ::puroro::internal::variant::{ReadExt, Variant, WriteExt};
use ::rand::prelude::*;
use ::rand_pcg::Pcg32;
use ::std::iter;

fn test_cases_random<const SIZE: usize>() -> Box<[u8]> {
    let mut rand = Pcg32::seed_from_u64(123456u64);
    let mut output = Vec::with_capacity(10 * SIZE);
    for item_u64 in iter::repeat_with(|| rand.gen::<u64>()) {
        let item_var = Variant::from(item_u64);
        WriteExt::write_variant(&mut output, item_var).unwrap();
    }
    output.into_boxed_slice()
}

fn read_variants<const SIZE: usize>(mut input: &[u8], output: &mut [Variant; SIZE]) {
    for i in 0..SIZE {
        let var = <&[u8] as ReadExt>::read_variant(&mut input).unwrap();
        output[i] = var.into();
    }
}

fn bench_read_variants(criterion: &mut Criterion) {
    let mut output = Box::new([Variant::default(); 10]);
    let input: Box<[u8]> = test_cases_random::<10>();
    let input2 = [0x01];
    criterion.bench_function("read_variant_random_10", |b| {
        b.iter(|| {
            // read_variants(black_box(input.as_ref()), output.as_mut());
            // let mut input_slice: &[u8] = black_box(&input2);
            // <&[u8] as ReadExt>::read_variant(&mut input_slice).unwrap();
            black_box(1) + black_box(2)
        })
    });
}

criterion_group!(benches, bench_read_variants);
criterion_main!(benches);
