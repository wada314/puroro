use crate::tests_pb::self_recursive::Msg;

#[test]
fn test_new() {
    //let msg = Msg::default();
    let msg: Msg;
}

fn hoge(msg: &mut Msg) {
    use ::puroro::DeserFromBytesIter;
    use std::io::Read;
    let buf = vec![0u8];
    msg.deser(buf.bytes());
}
