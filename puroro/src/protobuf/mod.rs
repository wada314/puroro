#![cfg_attr(feature = "allocator_api", feature(allocator_api))]
mod _root {
    #[allow(unused)]
    pub(crate) use super::*;
}
mod _puroro {
    #[allow(unused)]
    pub use crate::*;
}
mod _pinternal {
    #[allow(unused)]
    pub(crate) use crate::internal::*;
}
/// Re-exporting puroro
pub mod puroro {
    pub use crate::*;
}
pub mod google;
