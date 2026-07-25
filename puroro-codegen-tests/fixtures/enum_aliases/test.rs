//! Enum value aliases (same wire number, distinct const names).

use crate::enum_aliases::demo::{Code, Holder};
use ::puroro::Message;

#[test]
fn aliases_share_wire_number() {
    assert_eq!(Code::OK, Code::SUCCESS);
    assert_eq!(i32::try_from(Code::OK).unwrap(), 1);
    assert_eq!(Code::from(1), Code::OK);
    assert_eq!(Code::from(1), Code::SUCCESS);
}

#[test]
fn aliased_value_round_trip() {
    let mut msg = Holder::new();
    *msg.code_mut() = Code::SUCCESS;
    let decoded: Holder = Holder::decode(&msg.encode_to_vec()[..]).expect("decode");
    assert_eq!(decoded.code().get(), Code::OK);
    assert_eq!(decoded.code().get(), Code::SUCCESS);
}
