use crate::bumpalo;
use crate::Result;
use ::puroro::tags;

pub trait WrappedFieldType<LabelTag>
where
    LabelTag: tags::FieldLabelTag,
{
    type Item;
    fn len(&self) -> usize;
    fn merge_items<I>(&mut self, iter: I) -> Result<()>
    where
        I: Iterator<Item = Result<Self::Item>>;
    fn get_or_insert_with<F>(&mut self, f: F) -> &mut Self::Item
    where
        F: FnOnce() -> Self::Item;
    fn try_for_each<F>(&self, f: F) -> Result<()>
    where
        F: FnMut(&Self::Item) -> Result<()>;
    fn as_slice(&self) -> &[Self::Item];
}
impl<T> WrappedFieldType<tags::Required> for T {
    type Item = T;
    fn len(&self) -> usize {
        1
    }
    fn merge_items<I>(&mut self, iter: I) -> Result<()>
    where
        I: Iterator<Item = Result<Self::Item>>,
    {
        if let Some(item) = iter.last().transpose()? {
            *self = item;
        }
        Ok(())
    }
    fn get_or_insert_with<F>(&mut self, _: F) -> &mut Self::Item
    where
        F: FnOnce() -> Self::Item,
    {
        self
    }
    fn as_slice(&self) -> &[Self::Item] {
        std::slice::from_ref(self)
    }
    fn try_for_each<F>(&self, mut f: F) -> Result<()>
    where
        F: FnMut(&Self::Item) -> Result<()>,
    {
        (f)(self)
    }
}
impl<T> WrappedFieldType<tags::Optional2> for Option<T> {
    type Item = T;
    fn len(&self) -> usize {
        match self {
            Some(_) => 1,
            None => 0,
        }
    }
    fn merge_items<I>(&mut self, iter: I) -> Result<()>
    where
        I: Iterator<Item = Result<Self::Item>>,
    {
        if let Some(item) = iter.last().transpose()? {
            *self = Some(item);
        }
        Ok(())
    }
    fn get_or_insert_with<F>(&mut self, f: F) -> &mut Self::Item
    where
        F: FnOnce() -> Self::Item,
    {
        self.get_or_insert_with(f)
    }
    fn as_slice(&self) -> &[Self::Item] {
        match self {
            Some(x) => std::slice::from_ref(x),
            None => &[],
        }
    }
    fn try_for_each<F>(&self, mut f: F) -> Result<()>
    where
        F: FnMut(&Self::Item) -> Result<()>,
    {
        if let Some(item) = self {
            (f)(item)?;
        }
        Ok(())
    }
}
impl<T> WrappedFieldType<tags::Optional3> for T {
    type Item = T;
    fn len(&self) -> usize {
        1
    }
    fn merge_items<I>(&mut self, iter: I) -> Result<()>
    where
        I: Iterator<Item = Result<Self::Item>>,
    {
        if let Some(item) = iter.last().transpose()? {
            *self = item;
        }
        Ok(())
    }
    fn get_or_insert_with<F>(&mut self, _: F) -> &mut Self::Item
    where
        F: FnOnce() -> Self::Item,
    {
        self
    }
    fn as_slice(&self) -> &[Self::Item] {
        std::slice::from_ref(self)
    }
    fn try_for_each<F>(&self, mut f: F) -> Result<()>
    where
        F: FnMut(&Self::Item) -> Result<()>,
    {
        (f)(self)
    }
}
impl<VT> WrappedFieldType<tags::Repeated> for VT
where
    VT: VecType,
{
    type Item = VT::Item;
    fn len(&self) -> usize {
        <Self as VecType>::len(self)
    }
    fn merge_items<I>(&mut self, iter: I) -> Result<()>
    where
        I: Iterator<Item = Result<Self::Item>>,
    {
        for ritem in iter {
            self.push(ritem?);
        }
        Ok(())
    }
    fn get_or_insert_with<F>(&mut self, f: F) -> &mut Self::Item
    where
        F: FnOnce() -> Self::Item,
    {
        self.push((f)());
        self.last_mut().unwrap()
    }
    fn as_slice(&self) -> &[Self::Item] {
        <Self as VecType>::as_slice(self)
    }
    fn try_for_each<F>(&self, f: F) -> Result<()>
    where
        F: FnMut(&Self::Item) -> Result<()>,
    {
        <Self as VecType>::try_for_each(self, f)
    }
}

pub trait VecType {
    type Item;
    fn len(&self) -> usize;
    fn push(&mut self, item: Self::Item);
    fn last_mut(&mut self) -> Option<&mut Self::Item>;
    fn as_slice(&self) -> &[Self::Item];
    fn try_for_each<F>(&self, f: F) -> Result<()>
    where
        F: FnMut(&Self::Item) -> Result<()>;
}
impl<T> VecType for Vec<T> {
    type Item = T;
    fn len(&self) -> usize {
        <Vec<Self::Item>>::len(self)
    }
    fn push(&mut self, item: Self::Item) {
        <Vec<Self::Item>>::push(self, item)
    }
    fn last_mut(&mut self) -> Option<&mut Self::Item> {
        <[Self::Item]>::last_mut(self)
    }
    fn as_slice(&self) -> &[Self::Item] {
        <Self as AsRef<[Self::Item]>>::as_ref(self)
    }
    fn try_for_each<F>(&self, f: F) -> Result<()>
    where
        F: FnMut(&Self::Item) -> Result<()>,
    {
        <std::slice::Iter<'_, Self::Item> as Iterator>::try_for_each(
            &mut <[Self::Item]>::iter(self),
            f,
        )
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump, T> VecType for crate::bumpalo::collections::Vec<'bump, T> {
    type Item = T;
    fn len(&self) -> usize {
        <bumpalo::collections::Vec<'bump, Self::Item>>::len(self)
    }
    fn push(&mut self, item: Self::Item) {
        <bumpalo::collections::Vec<'bump, Self::Item>>::push(self, item)
    }
    fn last_mut(&mut self) -> Option<&mut Self::Item> {
        <[Self::Item]>::last_mut(self)
    }
    fn as_slice(&self) -> &[Self::Item] {
        <Self as AsRef<[Self::Item]>>::as_ref(self)
    }
    fn try_for_each<F>(&self, f: F) -> Result<()>
    where
        F: FnMut(&Self::Item) -> Result<()>,
    {
        <std::slice::Iter<'_, Self::Item> as Iterator>::try_for_each(
            &mut <[Self::Item]>::iter(self),
            f,
        )
    }
}

pub trait StringType {
    fn len(&self) -> usize;
    fn as_bytes(&self) -> &[u8];
    fn push(&mut self, c: char);
    fn clear(&mut self);
    fn reserve(&mut self, bytes_len: usize);
}
impl StringType for String {
    fn len(&self) -> usize {
        <String>::len(self)
    }
    fn as_bytes(&self) -> &[u8] {
        <String>::as_bytes(self)
    }
    fn push(&mut self, c: char) {
        <String>::push(self, c)
    }
    fn clear(&mut self) {
        <String>::clear(self)
    }
    fn reserve(&mut self, bytes_len: usize) {
        <String>::reserve(self, bytes_len)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> StringType for crate::bumpalo::collections::String<'bump> {
    fn len(&self) -> usize {
        <bumpalo::collections::String<'bump>>::len(self)
    }
    fn as_bytes(&self) -> &[u8] {
        <bumpalo::collections::String<'bump>>::as_bytes(self)
    }
    fn push(&mut self, c: char) {
        <bumpalo::collections::String<'bump>>::push(self, c)
    }
    fn clear(&mut self) {
        <bumpalo::collections::String<'bump>>::clear(self)
    }
    fn reserve(&mut self, bytes_len: usize) {
        <bumpalo::collections::String<'bump>>::reserve(self, bytes_len)
    }
}

pub trait BytesType {
    fn len(&self) -> usize;
    fn push(&mut self, byte: u8);
    fn clear(&mut self);
    fn reserve(&mut self, bytes_len: usize);
    fn as_slice(&self) -> &[u8];
}
impl BytesType for Vec<u8> {
    fn len(&self) -> usize {
        <Vec<u8>>::len(self)
    }
    fn push(&mut self, byte: u8) {
        <Vec<u8>>::push(self, byte)
    }
    fn clear(&mut self) {
        <Vec<u8>>::clear(self)
    }
    fn reserve(&mut self, bytes_len: usize) {
        <Vec<u8>>::reserve(self, bytes_len)
    }
    fn as_slice(&self) -> &[u8] {
        <Vec<u8>>::as_slice(self)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> BytesType for crate::bumpalo::collections::Vec<'bump, u8> {
    fn len(&self) -> usize {
        <bumpalo::collections::Vec<'bump, u8>>::len(self)
    }
    fn push(&mut self, byte: u8) {
        <bumpalo::collections::Vec<'bump, u8>>::push(self, byte)
    }
    fn clear(&mut self) {
        <bumpalo::collections::Vec<'bump, u8>>::clear(self)
    }
    fn reserve(&mut self, bytes_len: usize) {
        <bumpalo::collections::Vec<'bump, u8>>::reserve(self, bytes_len)
    }
    fn as_slice(&self) -> &[u8] {
        <bumpalo::collections::Vec<'bump, u8>>::as_slice(self)
    }
}
