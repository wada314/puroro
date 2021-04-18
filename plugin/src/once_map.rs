use ::once_cell::unsync::OnceCell;

pub struct OnceMapNode<K, V> {
    key: K,
    value: V,
    next: OnceCell<Box<OnceMapNode<K, V>>>,
}

pub struct OnceMap<K, V> {
    first: OnceCell<Box<OnceMapNode<K, V>>>,
}

impl<K, V> OnceMap<K, V> {
    pub fn get_or_init<F>(&self, key: K, f: F) -> &V
    where
        F: FnOnce() -> V,
    {
        let mut cell = &self.first;
        loop {
            let node = cell.get_or_init(|| {
                Box::new(OnceMapNode {
                    key,
                    value: (f)(),
                    next: OnceCell::new(),
                })
            });
        }
    }
}
