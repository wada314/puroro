use ::once_cell::unsync::OnceCell;

macro_rules! define_node {
    ($node:ident) => {
        struct $node<K, V> {
            key: K,
            value: OnceCell<V>,
            next: OnceCell<Box<$node<K, V>>>,
        }
        impl<K, V> $node<K, V> {
            fn find_cell(&self, key: K) -> &OnceCell<V>
            where
                K: PartialEq + Clone,
            {
                let mut node = self;
                loop {
                    if node.key == key {
                        return &node.value;
                    } else {
                        node = node.next
                            .get_or_init(|| Box::new($node {
                                key: key.clone(),
                                value: OnceCell::new(),
                                next: OnceCell::new(),
                        })).as_ref();
                    }
                }
            }
        }
    };
    ($node_k:ident, $node_l:ident $(, $rest:ident)*) => {
        struct $node_k<K, V> {
            key: K,
            value: OnceCell<V>,
            next: OnceCell<$node_l<K, V>>,
        }
        impl<K, V> $node_k<K, V> {
            fn find_cell(&self, key: K) -> &OnceCell<V>
            where
                K: PartialEq + Clone,
            {
                if self.key == key {
                    &self.value
                } else {
                    self.next
                        .get_or_init(|| $node_l {
                            key: key.clone(),
                            value: OnceCell::new(),
                            next: OnceCell::new(),
                        })
                    .find_cell(key)
                }
            }
        }
        define_node!($node_l $(, $rest)*);
    };
}
define_node!(Node4, Node3, Node2, Node);

pub struct OnceMap<K, V> {
    first: OnceCell<Node4<K, V>>,
}

impl<K, V> OnceMap<K, V> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get_or_init<F>(&self, key: K, f: F) -> &V
    where
        F: FnOnce() -> V,
        K: PartialEq + Clone,
    {
    }
}

impl<K, V> Default for OnceMap<K, V> {
    fn default() -> Self {
        Self {
            first: OnceCell::new(),
        }
    }
}
