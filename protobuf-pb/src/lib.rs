#![cfg_attr(feature = "puroro-nightly", feature(generic_associated_types))]
#![cfg_attr(feature = "puroro-nightly", feature(min_type_alias_impl_trait))]

include!(concat!(env!("OUT_DIR"), "/mod.rs"));
