use ::puroro::{Either, Message};
use ::tests_pb::full_coverage3::Msg;

#[test]
fn test_get_msg_descriptor() {
    assert_eq!("Msg", Msg::descriptor().name());
    assert_eq!("Msg", <() as Message<Msg>>::descriptor().name());
    assert_eq!("Msg", <((), Msg)>::descriptor().name());
    assert_eq!("Msg", <(Msg, ())>::descriptor().name());
    assert_eq!("Msg", Either::<Msg, ()>::descriptor().name());
    assert_eq!("Msg", Either::<(), Msg>::descriptor().name());
}
