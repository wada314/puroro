//! Lazy decode counts nested messages with the same limit as eager decode.

use crate::recursion_limit::{Nest, NestLazy};
use ::puroro::{DecodeError, Message, RECURSION_LIMIT};

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
