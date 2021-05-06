use super::FieldNew;
pub trait FieldTakeOrInit<'bump, T>: Sized {
    fn take_or_init(self) -> T;
    #[cfg(feature = "puroro-bumpalo")]
    fn take_or_init_in_bumpalo(self, _bump: &'bump ::bumpalo::Bump) -> T {
        self.take_or_init()
    }
}

impl<'bump, T> FieldTakeOrInit<'bump, T> for T {
    fn take_or_init(self) -> T {
        self
    }
}

impl<'bump, T> FieldTakeOrInit<'bump, T> for Option<T>
where
    T: Sized + FieldNew<'bump>,
{
    fn take_or_init(self) -> T {
        match self {
            Some(x) => x,
            None => FieldNew::new(),
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    fn take_or_init_in_bumpalo(self, bump: &'bump ::bumpalo::Bump) -> T {
        match self {
            Some(x) => x,
            None => FieldNew::new_in_bumpalo(bump),
        }
    }
}

impl<'bump, T> FieldTakeOrInit<'bump, T> for Option<Box<T>>
where
    T: Sized + FieldNew<'bump>,
{
    fn take_or_init(self) -> T {
        match self {
            Some(x) => *x,
            None => FieldNew::new(),
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    fn take_or_init_in_bumpalo(self, bump: &'bump ::bumpalo::Bump) -> T {
        match self {
            Some(x) => *x,
            None => FieldNew::new_in_bumpalo(bump),
        }
    }
}

// Because bumpalo's Box cannot move out the value, we need to use
// Clone instead...
impl<'bump, T> FieldTakeOrInit<'bump, T> for Option<::bumpalo::boxed::Box<'bump, T>>
where
    T: Sized + FieldNew<'bump> + Clone,
{
    fn take_or_init(self) -> T {
        match self {
            Some(x) => x.clone(),
            None => FieldNew::new(),
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    fn take_or_init_in_bumpalo(self, bump: &'bump ::bumpalo::Bump) -> T {
        match self {
            Some(x) => x.clone(),
            None => FieldNew::new_in_bumpalo(bump),
        }
    }
}
