use ::once_cell::unsync::OnceCell;

trait NodeInOnceCell<K, V>
where
    K: PartialEq + Clone,
{
    fn find_value_cell(&self, key: K) -> &OnceCell<V>;
}

macro_rules! define_node {
    ($node:ident) => {
        #[derive(Debug, Clone)]
        struct $node<K, V> {
            key: K,
            value: OnceCell<V>,
            next: OnceCell<Box<$node<K, V>>>,
        }
        define_node!(@impl $node);
        impl<K, V> NodeInOnceCell<K, V> for OnceCell<Box<$node<K, V>>>
        where
            K: PartialEq + Clone,
        {
            fn find_value_cell(&self, key: K) -> &OnceCell<V> {
                let node = self.get_or_init(|| Box::new($node {
                    key: key.clone(),
                    value: OnceCell::new(),
                    next: OnceCell::new(),
                }));
                if node.key == key {
                    &node.value
                } else {
                    node.next.find_value_cell(key)
                }
            }
        }
    };
    ($node_k:ident, $node_l:ident $(, $rest:ident)*) => {
        #[derive(Debug, Clone)]
        struct $node_k<K, V> {
            key: K,
            value: OnceCell<V>,
            next: OnceCell<$node_l<K, V>>,
        }
        define_node!(@impl $node_k);
        define_node!($node_l $(, $rest)*);
    };
    (@impl $node:ident) => {
        impl<K, V> NodeInOnceCell<K, V> for OnceCell<$node<K, V>>
        where
            K: PartialEq + Clone,
        {
            fn find_value_cell(&self, key: K) -> &OnceCell<V> {
                let node = self.get_or_init(|| $node {
                    key: key.clone(),
                    value: OnceCell::new(),
                    next: OnceCell::new(),
                });
                if node.key == key {
                    &node.value
                } else {
                    node.next.find_value_cell(key)
                }
            }
        }
    };
}
define_node!(Node4, Node3, Node2, Node);

macro_rules! define_map {
    ($map_name:ident, $node_name:ident) => {
        #[derive(Debug, Clone)]
        pub struct $map_name<K, V> {
            first: OnceCell<$node_name<K, V>>,
        }

        impl<K, V> $map_name<K, V> {
            pub fn new() -> Self {
                Default::default()
            }
            pub fn get_or_init<F>(&self, key: K, f: F) -> &V
            where
                F: FnOnce() -> V,
                K: PartialEq + Clone,
            {
                self.first.find_value_cell(key).get_or_init(f)
            }
            pub fn get_or_try_init<F, E>(&self, key: K, f: F) -> Result<&V, E>
            where
                F: FnOnce() -> Result<V, E>,
                K: PartialEq + Clone,
            {
                self.first.find_value_cell(key).get_or_try_init(f)
            }
        }

        impl<K, V> Default for $map_name<K, V> {
            fn default() -> Self {
                Self {
                    first: OnceCell::new(),
                }
            }
        }
    };
}
define_map!(OnceCellMap, Node);
define_map!(OnceCellMap2, Node2);
define_map!(OnceCellMap3, Node3);
define_map!(OnceCellMap4, Node4);
