use ::allocator_api2::alloc::Allocator;
use ::allocator_api2::boxed::Box;
use ::allocator_api2::unsize_box;
use ::allocator_api2::vec::Vec;

/// Object-safe trait representing a repeated field using an associated type.
///
/// This trait uses an associated type `Item` instead of a type parameter,
/// which allows for `impl Repeated<'_>` syntax which is more ergonomic,
/// especially when returning `impl Repeated<'_, Item = impl Address>`.
pub trait Repeated<'a> {
    /// The type of items in this repeated field.
    type Item;

    /// Number of elements.
    fn len(&self) -> usize;
    /// Whether empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Get an item by index.
    fn get(&self, index: usize) -> Option<Self::Item>;
    /// Returns a boxed iterator over items.
    fn iter_box(&self) -> Box<dyn Iterator<Item = Self::Item> + 'a>;
}

// Note: No blanket impl for slices to avoid missing_docs on public trait methods in impls.

/// Adapter over a reference to an allocator-aware Vec for Copy items (e.g., i32).
pub struct RefVec<'a, T: Copy, A: Allocator> {
    vec: &'a Vec<T, A>,
}

impl<'a, T: Copy, A: Allocator> RefVec<'a, T, A> {
    #[inline]
    /// Creates a repeated adapter over a reference to an allocator-aware Vec.
    pub fn new(vec: &'a Vec<T, A>) -> Self {
        Self { vec }
    }
}

#[allow(missing_docs)]
impl<'a, T: Copy + 'a, A: Allocator + 'a> Repeated<'a> for RefVec<'a, T, A> {
    type Item = T;

    fn len(&self) -> usize {
        self.vec.len()
    }
    fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }
    fn get(&self, index: usize) -> Option<Self::Item> {
        self.vec.get(index).copied()
    }
    fn iter_box(&self) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
        let owned: std::vec::Vec<T> = self.vec.iter().copied().collect();
        let it = owned.into_iter();
        let boxed = Box::new(it);
        let boxed_dyn: Box<dyn Iterator<Item = T> + 'a> = unsize_box!(boxed);
        boxed_dyn
    }
}

/// Adapter over a reference to an allocator-aware Vec with a mapping function.
///
/// This is similar to `RefVec` but supports non-Copy items by applying a mapping function.
pub struct RefVecMap<'a, S, T, A: Allocator, F>
where
    F: Fn(&'a S) -> T,
{
    vec: &'a Vec<S, A>,
    map: F,
}

impl<'a, S, T, A: Allocator, F> RefVecMap<'a, S, T, A, F>
where
    F: Fn(&'a S) -> T,
{
    /// Creates a repeated adapter over a reference to an allocator-aware Vec with a mapping function.
    #[inline]
    pub fn new(vec: &'a Vec<S, A>, map: F) -> Self {
        Self { vec, map }
    }
}

#[allow(missing_docs)]
impl<'a, S: 'a, T, A: Allocator + 'a, F> Repeated<'a> for RefVecMap<'a, S, T, A, F>
where
    F: Fn(&'a S) -> T + 'a,
    T: 'a,
{
    type Item = T;

    fn len(&self) -> usize {
        self.vec.len()
    }
    fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }
    fn get(&self, index: usize) -> Option<Self::Item> {
        self.vec.get(index).map(&self.map)
    }
    fn iter_box(&self) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
        let owned: std::vec::Vec<T> = self.vec.iter().map(&self.map).collect();
        let it = owned.into_iter();
        let boxed = Box::new(it);
        let boxed_dyn: Box<dyn Iterator<Item = T> + 'a> = unsize_box!(boxed);
        boxed_dyn
    }
}

/// Builds a `Box<dyn Repeated<'a>>` from a borrowed slice of Copy items.
///
/// This avoids allocation for elements and exposes iteration/indexing via the `Repeated` trait.
pub fn repeated_from_slice<'a, T: Copy + 'a>(slice: &'a [T]) -> Box<dyn Repeated<'a, Item = T> + 'a> {
    struct SliceRepeated<'b, U: Copy>(&'b [U]);
    impl<'b, U: Copy + 'b> Repeated<'b> for SliceRepeated<'b, U> {
        type Item = U;

        fn len(&self) -> usize {
            self.0.len()
        }
        fn is_empty(&self) -> bool {
            self.0.is_empty()
        }
        fn get(&self, index: usize) -> Option<Self::Item> {
            self.0.get(index).copied()
        }
        fn iter_box(&self) -> Box<dyn Iterator<Item = Self::Item> + 'b> {
            let owned: std::vec::Vec<U> = self.0.iter().copied().collect();
            let it = owned.into_iter();
            let boxed = Box::new(it);
            let boxed_dyn: Box<dyn Iterator<Item = U> + 'b> = unsize_box!(boxed);
            boxed_dyn
        }
    }
    let boxed = Box::new(SliceRepeated(slice));
    let boxed_dyn: Box<dyn Repeated<'a, Item = T> + 'a> = unsize_box!(boxed);
    boxed_dyn
}

/// Builds a `Box<dyn Repeated<'a>>` from a borrowed slice and a mapping function.
///
/// The function `f` is applied lazily on get/iterate; only immutable access is supported.
pub fn repeated_map_from_slice<'a, S: 'a, T: 'a, F>(
    slice: &'a [S],
    f: F,
) -> Box<dyn Repeated<'a, Item = T> + 'a>
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
    impl<'b, X: 'b, Y: 'b, G> Repeated<'b> for SliceRepeatedMap<'b, X, Y, G>
    where
        G: Fn(&X) -> Y + 'b,
    {
        type Item = Y;

        fn len(&self) -> usize {
            self.slice.len()
        }
        fn is_empty(&self) -> bool {
            self.slice.is_empty()
        }
        fn get(&self, index: usize) -> Option<Self::Item> {
            self.slice.get(index).map(&self.map)
        }
        fn iter_box(&self) -> Box<dyn Iterator<Item = Self::Item> + 'b> {
            let owned: std::vec::Vec<Y> = self.slice.iter().map(&self.map).collect();
            let it = owned.into_iter();
            let boxed = Box::new(it);
            let boxed_dyn: Box<dyn Iterator<Item = Y> + 'b> = unsize_box!(boxed);
            boxed_dyn
        }
    }
    let boxed = Box::new(SliceRepeatedMap { slice, map: f });
    let boxed_dyn: Box<dyn Repeated<'a, Item = T> + 'a> = unsize_box!(boxed);
    boxed_dyn
}

/// Builds a `Box<dyn Repeated<'a>>` from a reference to an allocator-aware Vec and a mapping function.
///
/// This is similar to `repeated_map_from_slice` but works with `Vec<S, A>` instead of slices.
pub fn repeated_map_from_vec<'a, S: 'a, T: 'a, A: Allocator + 'a, F>(
    vec: &'a Vec<S, A>,
    f: F,
) -> Box<dyn Repeated<'a, Item = T> + 'a>
where
    F: Fn(&S) -> T + 'a,
{
    let adapter = RefVecMap::new(vec, f);
    let boxed = Box::new(adapter);
    let boxed_dyn: Box<dyn Repeated<'a, Item = T> + 'a> = unsize_box!(boxed);
    boxed_dyn
}
