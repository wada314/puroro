use ::allocator_api2::alloc::Allocator;
use ::allocator_api2::boxed::Box;
use ::allocator_api2::unsize_box;
use ::allocator_api2::vec::Vec;

/// Object-safe trait representing a repeated field. Items are yielded by value.
pub trait Repeated<'a, T> {
    /// Number of elements.
    fn len(&self) -> usize;
    /// Whether empty.
    fn is_empty(&self) -> bool;
    /// Get an item by index.
    fn get(&self, index: usize) -> Option<T>;
    /// Returns a boxed iterator over items.
    fn iter_box(&self) -> Box<dyn Iterator<Item = T> + 'a>;
}

// Note: No blanket impl for slices to avoid missing_docs on public trait methods in impls.

/// Adapter over an allocator-aware Vec for Copy items (e.g., i32).
pub struct VecRepeated<'a, T: Copy, A: Allocator> {
    vec: &'a Vec<T, A>,
}

impl<'a, T: Copy, A: Allocator> VecRepeated<'a, T, A> {
    #[inline]
    /// Creates a repeated adapter over an allocator-aware Vec.
    pub fn new(vec: &'a Vec<T, A>) -> Self {
        Self { vec }
    }
}

#[allow(missing_docs)]
impl<'a, T: Copy + 'a, A: Allocator + 'a> Repeated<'a, T> for VecRepeated<'a, T, A> {
    fn len(&self) -> usize {
        self.vec.len()
    }
    fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }
    fn get(&self, index: usize) -> Option<T> {
        self.vec.get(index).copied()
    }
    fn iter_box(&self) -> Box<dyn Iterator<Item = T> + 'a> {
        let owned: std::vec::Vec<T> = self.vec.iter().copied().collect();
        let it = owned.into_iter();
        let boxed = Box::new(it);
        let boxed_dyn: Box<dyn Iterator<Item = T> + 'a> = unsize_box!(boxed);
        boxed_dyn
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
    /// Creates a mapping repeated adapter over an allocator-aware Vec.
    pub fn new(vec: &'a Vec<S, A>, map: F) -> Self {
        Self { vec, map }
    }
}

#[allow(missing_docs)]
impl<'a, S: 'a, T: 'a, A: Allocator + 'a, F> Repeated<'a, T> for VecRepeatedMap<'a, S, T, A, F>
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
    fn iter_box(&self) -> Box<dyn Iterator<Item = T> + 'a> {
        let owned: std::vec::Vec<T> = self.vec.iter().map(&self.map).collect();
        let it = owned.into_iter();
        let boxed = Box::new(it);
        let boxed_dyn: Box<dyn Iterator<Item = T> + 'a> = unsize_box!(boxed);
        boxed_dyn
    }
}

/// Builds a `Box<dyn Repeated<'a, T>>` from a borrowed slice of Copy items.
///
/// This avoids allocation for elements and exposes iteration/indexing via the `Repeated` trait.
#[doc = "Builds a Box<dyn Repeated<'a, T> + 'a> from a borrowed slice of Copy items."]
pub fn repeated_from_slice<'a, T: Copy + 'a>(slice: &'a [T]) -> Box<dyn Repeated<'a, T> + 'a> {
    struct SliceRepeated<'b, U: Copy>(&'b [U]);
    impl<'b, U: Copy + 'b> Repeated<'b, U> for SliceRepeated<'b, U> {
        fn len(&self) -> usize { self.0.len() }
        fn is_empty(&self) -> bool { self.0.is_empty() }
        fn get(&self, index: usize) -> Option<U> { self.0.get(index).copied() }
        fn iter_box(&self) -> Box<dyn Iterator<Item = U> + 'b> {
            let owned: std::vec::Vec<U> = self.0.iter().copied().collect();
            let it = owned.into_iter();
            let boxed = Box::new(it);
            let boxed_dyn: Box<dyn Iterator<Item = U> + 'b> = unsize_box!(boxed);
            boxed_dyn
        }
    }
    let boxed = Box::new(SliceRepeated(slice));
    let boxed_dyn: Box<dyn Repeated<'a, T> + 'a> = unsize_box!(boxed);
    boxed_dyn
}

/// Builds a `Box<dyn Repeated<'a, T>>` from a borrowed slice and a mapping function.
///
/// The function `f` is applied lazily on get/iterate; only immutable access is supported.
#[doc = "Builds a Box<dyn Repeated<'a, T>> from a borrowed slice and a mapping function."]
pub fn repeated_map_from_slice<'a, S: 'a, T: 'a, F>(
    slice: &'a [S],
    f: F,
) -> Box<dyn Repeated<'a, T> + 'a>
where
    F: Fn(&S) -> T + 'a,
{
    struct SliceRepeatedMap<'b, X, Y, G>
    where
        G: Fn(&X) -> Y,
    {
        slice: &'b [X],
        map: G,
    }
    impl<'b, X: 'b, Y: 'b, G> Repeated<'b, Y> for SliceRepeatedMap<'b, X, Y, G>
    where
        G: Fn(&X) -> Y + 'b,
    {
        fn len(&self) -> usize { self.slice.len() }
        fn is_empty(&self) -> bool { self.slice.is_empty() }
        fn get(&self, index: usize) -> Option<Y> { self.slice.get(index).map(&self.map) }
    fn iter_box(&self) -> Box<dyn Iterator<Item = Y> + 'b> {
        let owned: std::vec::Vec<Y> = self.slice.iter().map(&self.map).collect();
        let it = owned.into_iter();
        let boxed = Box::new(it);
        let boxed_dyn: Box<dyn Iterator<Item = Y> + 'b> = unsize_box!(boxed);
        boxed_dyn
    }
    }
    let boxed = Box::new(SliceRepeatedMap { slice, map: f });
    let boxed_dyn: Box<dyn Repeated<'a, T> + 'a> = unsize_box!(boxed);
    boxed_dyn
}

