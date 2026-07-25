//! Distinct packages in one generation response.

use crate::multi_package::pkg::a::Foo;
use crate::multi_package::pkg::b::Bar;
use ::puroro::Message;

#[test]
fn cross_package_message_round_trip() {
    let mut bar = Bar::new();
    *bar.foo_mut().x_mut() = 42;

    let decoded: Bar = Bar::decode(&bar.encode_to_vec()[..]).expect("decode");
    assert_eq!(decoded.foo().unwrap().x(), 42);

    let _: Foo = Foo::new();
}
