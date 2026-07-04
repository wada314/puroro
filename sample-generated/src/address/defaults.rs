//! Compile-time default markers for [`super::Address`] fields.
//!
//! Both `street` and `city` use protobuf type zero (`""`), so the field types
//! rely on the default [`::puroro::ProtoDefault`] type parameter.
