//! Lazy decode counts nested messages with the same limit as eager decode.

use crate::recursion_limit::{Holder, HolderLazy, Nest, NestLazy, Wrap, WrapLazy};
use ::puroro::{DecodeError, Message, RECURSION_LIMIT, ScopedBuf};
use ::puroro_rt::MessageMerge;

fn encode_nest_levels(levels: usize) -> Vec<u8> {
    let mut payload = Vec::new();
    for _ in 0..levels {
        let inner = payload;
        let mut outer = Vec::new();
        outer.push(0x0a); // field 1, LEN
        let mut len = inner.len() as u64;
        loop {
            let mut byte = (len & 0x7f) as u8;
            len >>= 7;
            if len != 0 {
                byte |= 0x80;
            }
            outer.push(byte);
            if len == 0 {
                break;
            }
        }
        outer.extend_from_slice(&inner);
        payload = outer;
    }
    payload
}

#[test]
fn lazy_decode_rejects_at_recursion_limit() {
    let over = encode_nest_levels(RECURSION_LIMIT);
    let eager_over: Result<Nest, _> = Nest::decode(&over[..]);
    let lazy_over: Result<NestLazy, _> = NestLazy::decode(&over[..]);
    assert_eq!(eager_over.err(), Some(DecodeError::RecursionLimitExceeded));
    assert_eq!(lazy_over.err(), Some(DecodeError::RecursionLimitExceeded));

    let ok = encode_nest_levels(RECURSION_LIMIT - 1);
    let eager_ok: Result<Nest, _> = Nest::decode(&ok[..]);
    let lazy_ok: Result<NestLazy, _> = NestLazy::decode(&ok[..]);
    eager_ok.expect("eager within limit");
    lazy_ok.expect("lazy within limit");
}

fn encode_holder(item: &[u8]) -> Vec<u8> {
    let mut outer = Vec::new();
    outer.push(0x0a); // field 1, LEN
    let mut len = item.len() as u64;
    loop {
        let mut byte = (len & 0x7f) as u8;
        len >>= 7;
        if len != 0 {
            byte |= 0x80;
        }
        outer.push(byte);
        if len == 0 {
            break;
        }
    }
    outer.extend_from_slice(item);
    outer
}

fn merge_at<M: MessageMerge + Default>(bytes: &[u8], depth: usize) -> Result<M, DecodeError> {
    let mut message = M::default();
    let mut cursor = bytes;
    let mut scoped = ScopedBuf::new(&mut cursor);
    MessageMerge::merge_from_with_depth(&mut message, &mut scoped, depth)?;
    Ok(message)
}

#[test]
fn leaf_child_fails_only_at_the_limit() {
    let with_item = encode_holder(&[0x08, 0x01]);
    assert!(matches!(
        merge_at::<Holder>(&with_item, RECURSION_LIMIT - 1),
        Err(DecodeError::RecursionLimitExceeded)
    ));
    assert!(matches!(
        merge_at::<HolderLazy>(&with_item, RECURSION_LIMIT - 1),
        Err(DecodeError::RecursionLimitExceeded)
    ));
    merge_at::<Holder>(&with_item, RECURSION_LIMIT - 2).expect("eager under the limit");
    merge_at::<HolderLazy>(&with_item, RECURSION_LIMIT - 2).expect("lazy under the limit");

    let scalar = vec![0x10, 0x01]; // field 2, varint 1
    merge_at::<Holder>(&scalar, RECURSION_LIMIT - 1).expect("no nested field");
    merge_at::<HolderLazy>(&scalar, RECURSION_LIMIT - 1).expect("no nested field");

    // Wrap calls Holder::check_message_depth. Holder's children are leaves, so
    // that walk is skipped until the next level would be the limit.
    let wrap = encode_holder(&with_item);
    assert!(matches!(
        merge_at::<Wrap>(&wrap, RECURSION_LIMIT - 2),
        Err(DecodeError::RecursionLimitExceeded)
    ));
    assert!(matches!(
        merge_at::<WrapLazy>(&wrap, RECURSION_LIMIT - 2),
        Err(DecodeError::RecursionLimitExceeded)
    ));
    merge_at::<Wrap>(&wrap, RECURSION_LIMIT - 3).expect("eager wrap under the limit");
    merge_at::<WrapLazy>(&wrap, RECURSION_LIMIT - 3).expect("lazy wrap under the limit");
}
