/// We need this JUST ONLY for `::bumpalo::boxed::Box`, because it doesn't have
/// the reference to the bumpalo instance.
pub trait FieldClone {
    fn clone_with<T: ::puroro::Message>(msg: &T) -> Self;
}
