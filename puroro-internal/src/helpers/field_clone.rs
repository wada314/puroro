/// We need this JUST ONLY for `::bumpalo::boxed::Box`, because it doesn't have
/// the reference to the bumpalo instance.
pub trait FieldClone<'bump>: Sized {
    fn clone(&self) -> Self;
    #[cfg(feature = "puroro-bumpalo")]
    fn clone_in_bumpalo(&self, _bump: &'bump ::bumpalo::Bump) -> Self {
        <Self as FieldClone>::clone(self)
    }
}
