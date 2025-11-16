use ::allocator_api2::alloc::Allocator;
use ::allocator_api2::vec::Vec;

/// Object-safe trait representing a repeated field. Items are yielded by value.
pub trait Repeated<T> {
    /// Number of elements.
    fn len(&self) -> usize;
    /// Whether empty.
    fn is_empty(&self) -> bool;
    /// Get an item by index.
    fn get(&self, index: usize) -> Option<T>;
    /// Returns a boxed iterator over items.
    fn iter_box(&self) -> Box<dyn Iterator<Item = T> + '_>;
}

/// Adapter over an allocator-aware Vec for Copy items (e.g., i32).
pub struct VecRepeated<'a, T: Copy, A: Allocator> {
    vec: &'a Vec<T, A>,
}

impl<'a, T: Copy, A: Allocator> VecRepeated<'a, T, A> {
    #[inline]
    pub fn new(vec: &'a Vec<T, A>) -> Self {
        Self { vec }
    }
}

impl<'a, T: Copy + 'a, A: Allocator + 'a> Repeated<T> for VecRepeated<'a, T, A> {
    fn len(&self) -> usize {
        self.vec.len()
    }
    fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }
    fn get(&self, index: usize) -> Option<T> {
        self.vec.get(index).copied()
    }
    fn iter_box(&self) -> Box<dyn Iterator<Item = T> + '_> {
        Box::new(self.vec.iter().copied())
    }
}

/// Generic mapping adapter from `&Vec<S, A>` to any `T` via a mapping function.
pub struct VecRepeatedMap<'a, S, T, A: Allocator, F>
where
    F: Fn(&S) -> T,
{
    vec: &'a Vec<S, A>,
    map: F,
}

impl<'a, S, T, A: Allocator, F> VecRepeatedMap<'a, S, T, A, F>
where
    F: Fn(&S) -> T,
{
    #[inline]
    pub fn new(vec: &'a Vec<S, A>, map: F) -> Self {
        Self { vec, map }
    }
}

impl<'a, S: 'a, T: 'a, A: Allocator + 'a, F> Repeated<T> for VecRepeatedMap<'a, S, T, A, F>
where
    F: Fn(&S) -> T + 'a,
{
    fn len(&self) -> usize {
        self.vec.len()
    }
    fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }
    fn get(&self, index: usize) -> Option<T> {
        self.vec.get(index).map(&self.map)
    }
    fn iter_box(&self) -> Box<dyn Iterator<Item = T> + '_> {
        Box::new(self.vec.iter().map(&self.map))
    }
}


