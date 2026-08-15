//! SSO (`InlineOrHeap`) vs heap (`Inline`) singular `string` microbenchmarks.
//!
//! Two workloads, same eight fields, identical wire bytes per workload:
//! - `all_le23` — every value fits in [`INLINE_CAP`] (23 on 64-bit)
//! - `mixed` — four inline-sized + four longer-than-inline values
//!
//! Criterion IDs are workload names, not wire sizes.
//!
//! Run:
//! ```text
//! cargo bench -p puroro --bench string_layout
//! cargo bench -p puroro --bench string_layout -- --test
//! ```

use ::allocator_api2::alloc::{Allocator, Global};
use ::bitvec::array::BitArray;
use ::bitvec::order::Lsb0;
use ::bytes::{Buf, BufMut};
use ::core::time::Duration;
use ::criterion::measurement::WallTime;
use ::criterion::{BenchmarkGroup, BenchmarkId, Criterion, Throughput, black_box, criterion_group};
use ::puroro::{DecodeBuf, DecodeError, Message, String as AllocString};
use ::puroro_rt::decode::{decode_tag, skip_field_and_save};
use ::puroro_rt::{
    EncodeCtx, Explicit, FieldDeallocate, FieldEncode, INLINE_CAP, Inline, InlineOrHeap,
    MessageCommon, MessageEncode, MessageMerge, ProtoString, SingularField,
};

/// All eight values fit in the SSO slot (`len <= INLINE_CAP`).
const ALL_LE23: [&str; 8] = ["id", "ok", "us", "v1", "name", "code", "short", "label"];

/// Four SSO-sized values, then four that must go on the heap.
const MIXED: [&str; 8] = [
    "id",
    "ok",
    "name",
    "label",
    "alpha-benchmark-string-0000",
    "bravo-benchmark-string-1111",
    "charlie-benchmark-string-22",
    "delta-benchmark-string-3333",
];

struct Workload {
    name: &'static str,
    values: [&'static str; 8],
}

const WORKLOADS: &[Workload] = &[
    Workload {
        name: "all_le23",
        values: ALL_LE23,
    },
    Workload {
        name: "mixed",
        values: MIXED,
    },
];

macro_rules! drop_fields {
    ($self:ident, $($field:ident),+ $(,)?) => {{
        $($self.$field.deallocate(&$self._common);)+
        $self._common.deallocate();
    }};
}

/// Eight explicit strings, SSO layout (presence + heap bit per field).
pub struct SsoShort<A: Allocator + Clone = Global> {
    _common: MessageCommon<BitArray<[u8; 2], Lsb0>, A>,
    s0: SingularField<ProtoString, Explicit<0>, 1, A, InlineOrHeap<1>>,
    s1: SingularField<ProtoString, Explicit<2>, 2, A, InlineOrHeap<3>>,
    s2: SingularField<ProtoString, Explicit<4>, 3, A, InlineOrHeap<5>>,
    s3: SingularField<ProtoString, Explicit<6>, 4, A, InlineOrHeap<7>>,
    s4: SingularField<ProtoString, Explicit<8>, 5, A, InlineOrHeap<9>>,
    s5: SingularField<ProtoString, Explicit<10>, 6, A, InlineOrHeap<11>>,
    s6: SingularField<ProtoString, Explicit<12>, 7, A, InlineOrHeap<13>>,
    s7: SingularField<ProtoString, Explicit<14>, 8, A, InlineOrHeap<15>>,
}

/// Same eight strings, always-heap layout (presence bit only).
pub struct HeapShort<A: Allocator + Clone = Global> {
    _common: MessageCommon<BitArray<[u8; 1], Lsb0>, A>,
    s0: SingularField<ProtoString, Explicit<0>, 1, A, Inline>,
    s1: SingularField<ProtoString, Explicit<1>, 2, A, Inline>,
    s2: SingularField<ProtoString, Explicit<2>, 3, A, Inline>,
    s3: SingularField<ProtoString, Explicit<3>, 4, A, Inline>,
    s4: SingularField<ProtoString, Explicit<4>, 5, A, Inline>,
    s5: SingularField<ProtoString, Explicit<5>, 6, A, Inline>,
    s6: SingularField<ProtoString, Explicit<6>, 7, A, Inline>,
    s7: SingularField<ProtoString, Explicit<7>, 8, A, Inline>,
}

macro_rules! impl_eight_string_message {
    ($ty:ident) => {
        impl<A: Allocator + Clone> $ty<A> {
            pub fn new_in(alloc: A) -> Self {
                Self {
                    _common: MessageCommon::new_in(BitArray::ZERO, alloc.clone()),
                    s0: SingularField::new_in(alloc.clone()),
                    s1: SingularField::new_in(alloc.clone()),
                    s2: SingularField::new_in(alloc.clone()),
                    s3: SingularField::new_in(alloc.clone()),
                    s4: SingularField::new_in(alloc.clone()),
                    s5: SingularField::new_in(alloc.clone()),
                    s6: SingularField::new_in(alloc.clone()),
                    s7: SingularField::new_in(alloc),
                }
            }
        }

        impl<A: Allocator + Clone> Drop for $ty<A> {
            fn drop(&mut self) {
                drop_fields!(self, s0, s1, s2, s3, s4, s5, s6, s7);
            }
        }

        impl<A: Allocator + Clone> ::unmanaged::DeallocateIn<A> for $ty<A> {
            #[inline]
            unsafe fn deallocate_in(self, _alloc: A) {
                drop(self);
            }
        }

        impl<A: Allocator + Clone> MessageEncode for $ty<A> {
            fn encoded_len(&self, ctx: &mut EncodeCtx) -> usize {
                let c = &self._common;
                self.s0.encoded_len(c, ctx)
                    + self.s1.encoded_len(c, ctx)
                    + self.s2.encoded_len(c, ctx)
                    + self.s3.encoded_len(c, ctx)
                    + self.s4.encoded_len(c, ctx)
                    + self.s5.encoded_len(c, ctx)
                    + self.s6.encoded_len(c, ctx)
                    + self.s7.encoded_len(c, ctx)
                    + c.unknown_fields.len()
            }

            fn encode_raw<B: BufMut>(&self, ctx: &mut EncodeCtx, buf: &mut B) {
                let c = &self._common;
                self.s0.encode_raw(c, ctx, buf);
                self.s1.encode_raw(c, ctx, buf);
                self.s2.encode_raw(c, ctx, buf);
                self.s3.encode_raw(c, ctx, buf);
                self.s4.encode_raw(c, ctx, buf);
                self.s5.encode_raw(c, ctx, buf);
                self.s6.encode_raw(c, ctx, buf);
                self.s7.encode_raw(c, ctx, buf);
                buf.put_slice(&c.unknown_fields);
            }
        }

        impl<A: Allocator + Clone> MessageMerge for $ty<A> {
            fn merge_from_with_depth<B: DecodeBuf>(
                &mut self,
                buf: &mut B,
                depth: usize,
            ) -> Result<(), DecodeError> {
                if depth >= ::puroro::RECURSION_LIMIT {
                    return Err(DecodeError::RecursionLimitExceeded);
                }
                while buf.has_remaining() {
                    let (field_number, wire_type) = decode_tag(buf)?;
                    match field_number.as_u32() {
                        1 => self
                            .s0
                            .bind_mut(&mut self._common)
                            .merge(wire_type, buf, depth)?,
                        2 => self
                            .s1
                            .bind_mut(&mut self._common)
                            .merge(wire_type, buf, depth)?,
                        3 => self
                            .s2
                            .bind_mut(&mut self._common)
                            .merge(wire_type, buf, depth)?,
                        4 => self
                            .s3
                            .bind_mut(&mut self._common)
                            .merge(wire_type, buf, depth)?,
                        5 => self
                            .s4
                            .bind_mut(&mut self._common)
                            .merge(wire_type, buf, depth)?,
                        6 => self
                            .s5
                            .bind_mut(&mut self._common)
                            .merge(wire_type, buf, depth)?,
                        7 => self
                            .s6
                            .bind_mut(&mut self._common)
                            .merge(wire_type, buf, depth)?,
                        8 => self
                            .s7
                            .bind_mut(&mut self._common)
                            .merge(wire_type, buf, depth)?,
                        _ => skip_field_and_save(
                            field_number,
                            wire_type,
                            buf,
                            &mut self._common.unknown_fields,
                            self._common.alloc.clone(),
                        )?,
                    }
                }
                Ok(())
            }
        }

        impl<A: Allocator + Clone> ::unmanaged::DefaultIn<A> for $ty<A> {
            #[inline]
            fn default_in(alloc: A) -> Self {
                Self::new_in(alloc)
            }
        }

        impl<A: Allocator + Clone> Message for $ty<A> {
            type Alloc = A;

            fn new_in(alloc: A) -> Self {
                Self::new_in(alloc)
            }

            fn encode<B: BufMut>(&self, buf: &mut B) {
                ::puroro_rt::encode_message(self, buf)
            }

            fn encode_to_vec(&self) -> Vec<u8> {
                ::puroro_rt::encode_message_to_vec(self)
            }

            fn merge_from<B: Buf>(&mut self, buf: &mut B) -> Result<(), DecodeError> {
                ::puroro_rt::merge_message(self, buf)
            }

            fn unknown_fields(&self) -> impl Iterator<Item = ::puroro::UnknownField<'_>> + '_ {
                self._common.iter_unknown_fields()
            }

            fn validate(&self) -> Result<(), DecodeError> {
                Ok(())
            }
        }
    };
}

impl_eight_string_message!(SsoShort);
impl_eight_string_message!(HeapShort);

impl Default for SsoShort<Global> {
    fn default() -> Self {
        Self::new_in(Global)
    }
}

impl Default for HeapShort<Global> {
    fn default() -> Self {
        Self::new_in(Global)
    }
}

impl SsoShort<Global> {
    fn sample(values: [&str; 8]) -> Self {
        let mut m = Self::new_in(Global);
        m.s0.bind_mut(&mut m._common).value_mut().set(values[0]);
        m.s1.bind_mut(&mut m._common).value_mut().set(values[1]);
        m.s2.bind_mut(&mut m._common).value_mut().set(values[2]);
        m.s3.bind_mut(&mut m._common).value_mut().set(values[3]);
        m.s4.bind_mut(&mut m._common).value_mut().set(values[4]);
        m.s5.bind_mut(&mut m._common).value_mut().set(values[5]);
        m.s6.bind_mut(&mut m._common).value_mut().set(values[6]);
        m.s7.bind_mut(&mut m._common).value_mut().set(values[7]);
        m
    }
}

impl HeapShort<Global> {
    fn sample(values: [&str; 8]) -> Self {
        let mut m = Self::new_in(Global);
        *m.s0.bind_mut(&mut m._common).value_mut() = AllocString::from_str_in(values[0], Global);
        *m.s1.bind_mut(&mut m._common).value_mut() = AllocString::from_str_in(values[1], Global);
        *m.s2.bind_mut(&mut m._common).value_mut() = AllocString::from_str_in(values[2], Global);
        *m.s3.bind_mut(&mut m._common).value_mut() = AllocString::from_str_in(values[3], Global);
        *m.s4.bind_mut(&mut m._common).value_mut() = AllocString::from_str_in(values[4], Global);
        *m.s5.bind_mut(&mut m._common).value_mut() = AllocString::from_str_in(values[5], Global);
        *m.s6.bind_mut(&mut m._common).value_mut() = AllocString::from_str_in(values[6], Global);
        *m.s7.bind_mut(&mut m._common).value_mut() = AllocString::from_str_in(values[7], Global);
        m
    }
}

fn configure(group: &mut BenchmarkGroup<'_, WallTime>) {
    group.warm_up_time(Duration::from_millis(400));
    group.measurement_time(Duration::from_secs(2));
    group.sample_size(60);
}

fn fill_sso(m: &mut SsoShort, values: [&str; 8]) {
    m.s0.bind_mut(&mut m._common).value_mut().set(values[0]);
    m.s1.bind_mut(&mut m._common).value_mut().set(values[1]);
    m.s2.bind_mut(&mut m._common).value_mut().set(values[2]);
    m.s3.bind_mut(&mut m._common).value_mut().set(values[3]);
    m.s4.bind_mut(&mut m._common).value_mut().set(values[4]);
    m.s5.bind_mut(&mut m._common).value_mut().set(values[5]);
    m.s6.bind_mut(&mut m._common).value_mut().set(values[6]);
    m.s7.bind_mut(&mut m._common).value_mut().set(values[7]);
}

fn fill_heap(m: &mut HeapShort, values: [&str; 8]) {
    *m.s0.bind_mut(&mut m._common).value_mut() = AllocString::from_str_in(values[0], Global);
    *m.s1.bind_mut(&mut m._common).value_mut() = AllocString::from_str_in(values[1], Global);
    *m.s2.bind_mut(&mut m._common).value_mut() = AllocString::from_str_in(values[2], Global);
    *m.s3.bind_mut(&mut m._common).value_mut() = AllocString::from_str_in(values[3], Global);
    *m.s4.bind_mut(&mut m._common).value_mut() = AllocString::from_str_in(values[4], Global);
    *m.s5.bind_mut(&mut m._common).value_mut() = AllocString::from_str_in(values[5], Global);
    *m.s6.bind_mut(&mut m._common).value_mut() = AllocString::from_str_in(values[6], Global);
    *m.s7.bind_mut(&mut m._common).value_mut() = AllocString::from_str_in(values[7], Global);
}

fn bench_encode(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_layout/encode");
    configure(&mut group);
    for work in WORKLOADS {
        let sso = SsoShort::sample(work.values);
        let heap = HeapShort::sample(work.values);
        let sso_bytes = sso.encode_to_vec();
        assert_eq!(sso_bytes, heap.encode_to_vec());
        group.throughput(Throughput::Bytes(sso_bytes.len() as u64));
        group.bench_function(BenchmarkId::new("sso", work.name), |b| {
            b.iter(|| black_box(&sso).encode_to_vec())
        });
        group.bench_function(BenchmarkId::new("heap", work.name), |b| {
            b.iter(|| black_box(&heap).encode_to_vec())
        });
    }
    group.finish();
}

fn bench_decode(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_layout/decode");
    configure(&mut group);
    for work in WORKLOADS {
        let bytes = SsoShort::sample(work.values).encode_to_vec();
        group.throughput(Throughput::Bytes(bytes.len() as u64));
        group.bench_function(BenchmarkId::new("sso", work.name), |b| {
            let bytes = bytes.as_slice();
            b.iter(|| SsoShort::decode(black_box(bytes)).unwrap())
        });
        group.bench_function(BenchmarkId::new("heap", work.name), |b| {
            let bytes = bytes.as_slice();
            b.iter(|| HeapShort::decode(black_box(bytes)).unwrap())
        });
    }
    group.finish();
}

fn bench_set(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_layout/set");
    configure(&mut group);
    group.throughput(Throughput::Elements(8));
    for work in WORKLOADS {
        group.bench_function(BenchmarkId::new("sso", work.name), |b| {
            let mut m = SsoShort::new_in(Global);
            b.iter(|| fill_sso(&mut m, black_box(work.values)));
        });
        group.bench_function(BenchmarkId::new("heap", work.name), |b| {
            let mut m = HeapShort::new_in(Global);
            b.iter(|| fill_heap(&mut m, black_box(work.values)));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_encode, bench_decode, bench_set);

fn main() {
    assert!(
        ALL_LE23.iter().all(|s| s.len() <= INLINE_CAP),
        "all_le23 values must fit in the SSO slot (INLINE_CAP={INLINE_CAP})"
    );
    assert!(
        MIXED.iter().take(4).all(|s| s.len() <= INLINE_CAP),
        "mixed short half must fit in the SSO slot"
    );
    assert!(
        MIXED.iter().skip(4).all(|s| s.len() > INLINE_CAP),
        "mixed long half must exceed INLINE_CAP={INLINE_CAP}"
    );
    for work in WORKLOADS {
        let sso_bytes = SsoShort::sample(work.values).encode_to_vec();
        let heap_bytes = HeapShort::sample(work.values).encode_to_vec();
        assert_eq!(sso_bytes, heap_bytes, "wire mismatch for {}", work.name);
        assert_eq!(
            SsoShort::decode(&sso_bytes[..]).unwrap().encode_to_vec(),
            sso_bytes
        );
        assert_eq!(
            HeapShort::decode(&heap_bytes[..]).unwrap().encode_to_vec(),
            heap_bytes
        );
    }
    benches();
}
