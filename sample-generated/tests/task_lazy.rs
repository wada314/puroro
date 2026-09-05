//! Numerical-only `TaskLazy` ingest: scanner + catalog merge.

use ::puroro::DecodeError;
use ::puroro_rt::Varint;
use ::puroro_rt::encode::{encode_varint_field, field_number_const};
use ::puroro_sample_generated::task::{FIELD_LABELS, FIELD_SCORE, FIELD_SCORES, FIELD_TAG_IDS};
use ::puroro_sample_generated::{Priority, Status, TaskLazy};

fn encode_u64_varint(mut v: u64, buf: &mut Vec<u8>) {
    loop {
        let mut byte = (v & 0x7f) as u8;
        v >>= 7;
        if v != 0 {
            byte |= 0x80;
        }
        buf.push(byte);
        if v == 0 {
            break;
        }
    }
}

fn encode_packed_int32_field(field_number: u32, values: &[i32], buf: &mut Vec<u8>) {
    let mut payload = Vec::new();
    for &v in values {
        encode_u64_varint(v as u64, &mut payload);
    }
    encode_u64_varint(u64::from(field_number) << 3 | 2, buf);
    encode_u64_varint(payload.len() as u64, buf);
    buf.extend_from_slice(&payload);
}

fn encode_len_bytes(field_number: u32, payload: &[u8], buf: &mut Vec<u8>) {
    encode_u64_varint(u64::from(field_number) << 3 | 2, buf);
    encode_u64_varint(payload.len() as u64, buf);
    buf.extend_from_slice(payload);
}

#[test]
fn empty_message_is_implicit_zero() {
    let lazy = TaskLazy::new();
    assert_eq!(lazy.score().unwrap(), 0);
    assert!(!lazy.max_retries().unwrap().is_set());
    assert_eq!(lazy.max_retries().unwrap().get(), 3);
    assert!(lazy.tag_ids().unwrap().is_empty());
    assert!(lazy.scores().unwrap().is_empty());
    assert!(!lazy.status().unwrap().is_set());
    assert!(!lazy.priority().unwrap().is_set());
    assert!(!lazy.done().unwrap());
    assert!(!lazy.flag().unwrap().is_set());
    assert!(lazy.votes().unwrap().is_empty());
}

#[test]
fn last_wins_scalar_in_one_message() {
    let mut bytes = Vec::new();
    encode_varint_field(
        field_number_const::<FIELD_SCORE>(),
        Varint::from_int32(1),
        &mut bytes,
    );
    encode_varint_field(
        field_number_const::<FIELD_SCORE>(),
        Varint::from_int32(9),
        &mut bytes,
    );

    let lazy = TaskLazy::decode(&bytes[..]).unwrap();
    assert_eq!(lazy.score().unwrap(), 9);
}

#[test]
fn second_merge_from_is_protobuf_merge() {
    let mut first = Vec::new();
    encode_varint_field(
        field_number_const::<FIELD_SCORE>(),
        Varint::from_int32(1),
        &mut first,
    );
    encode_varint_field(
        field_number_const::<FIELD_TAG_IDS>(),
        Varint::from_int32(10),
        &mut first,
    );

    let mut second = Vec::new();
    encode_varint_field(
        field_number_const::<FIELD_SCORE>(),
        Varint::from_int32(2),
        &mut second,
    );
    encode_varint_field(
        field_number_const::<FIELD_TAG_IDS>(),
        Varint::from_int32(20),
        &mut second,
    );

    let mut lazy = TaskLazy::new();
    lazy.merge_from(&mut first.as_slice()).unwrap();
    lazy.merge_from(&mut second.as_slice()).unwrap();
    assert_eq!(lazy.score().unwrap(), 2);
    assert_eq!(lazy.tag_ids().unwrap(), &[10, 20]);
}

#[test]
fn packed_and_expanded_mixed() {
    let mut bytes = Vec::new();
    encode_packed_int32_field(FIELD_TAG_IDS, &[1, 2], &mut bytes);
    encode_varint_field(
        field_number_const::<FIELD_TAG_IDS>(),
        Varint::from_int32(3),
        &mut bytes,
    );
    encode_varint_field(
        field_number_const::<FIELD_SCORES>(),
        Varint::from_int32(10),
        &mut bytes,
    );
    encode_packed_int32_field(FIELD_SCORES, &[20, 30], &mut bytes);

    let lazy = TaskLazy::decode(&bytes[..]).unwrap();
    assert_eq!(lazy.tag_ids().unwrap(), &[1, 2, 3]);
    assert_eq!(lazy.scores().unwrap(), &[10, 20, 30]);
}

#[test]
fn truncated_input_errors_on_finish() {
    let mut bytes = Vec::new();
    encode_varint_field(
        field_number_const::<FIELD_SCORE>(),
        Varint::from_int32(150),
        &mut bytes,
    );
    assert!(bytes.len() >= 2);

    let mut lazy = TaskLazy::new();
    lazy.push(&bytes[..bytes.len() - 1]).unwrap();
    assert_eq!(lazy.finish(), Err(DecodeError::TruncatedMessage));
    assert_eq!(lazy.score(), Err(DecodeError::TruncatedMessage));
}

#[test]
fn getter_before_finish_is_unfinished() {
    let mut bytes = Vec::new();
    encode_varint_field(
        field_number_const::<FIELD_SCORE>(),
        Varint::from_int32(4),
        &mut bytes,
    );

    let mut lazy = TaskLazy::new();
    lazy.push(&bytes).unwrap();
    assert_eq!(lazy.score(), Err(DecodeError::UnfinishedMessage));
    lazy.finish().unwrap();
    assert_eq!(lazy.score().unwrap(), 4);
}

#[test]
fn incremental_chunks_join_one_record() {
    let mut bytes = Vec::new();
    encode_varint_field(
        field_number_const::<FIELD_SCORE>(),
        Varint::from_int32(150),
        &mut bytes,
    );
    assert!(bytes.len() >= 3);

    let mut lazy = TaskLazy::new();
    lazy.push(&bytes[..1]).unwrap();
    lazy.push(&bytes[1..2]).unwrap();
    lazy.push(&bytes[2..]).unwrap();
    lazy.finish().unwrap();
    assert_eq!(lazy.score().unwrap(), 150);
}

#[test]
fn skips_len_records() {
    let mut bytes = Vec::new();
    encode_len_bytes(FIELD_LABELS, b"ignored", &mut bytes);
    encode_varint_field(
        field_number_const::<FIELD_SCORE>(),
        Varint::from_int32(7),
        &mut bytes,
    );

    let lazy = TaskLazy::decode(&bytes[..]).unwrap();
    assert_eq!(lazy.score().unwrap(), 7);
}

#[test]
fn enums_and_bools() {
    let mut bytes = Vec::new();
    encode_varint_field(field_number_const::<9>(), Varint::from_int32(1), &mut bytes);
    encode_varint_field(
        field_number_const::<10>(),
        Varint::from_int32(2),
        &mut bytes,
    );
    encode_varint_field(
        field_number_const::<16>(),
        Varint::from_bool(true),
        &mut bytes,
    );
    encode_varint_field(
        field_number_const::<17>(),
        Varint::from_bool(false),
        &mut bytes,
    );
    encode_packed_int32_field(20, &[1, 0, 1], &mut bytes);

    let lazy = TaskLazy::decode(&bytes[..]).unwrap();
    assert_eq!(lazy.status().unwrap().get(), Status::PENDING);
    assert_eq!(lazy.priority().unwrap().get(), Priority::HIGH);
    assert!(lazy.done().unwrap());
    assert!(lazy.flag().unwrap().is_set());
    assert!(!lazy.flag().unwrap().get());
    assert_eq!(lazy.votes().unwrap(), &[true, false, true]);
}
