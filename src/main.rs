// mod prolly_tree;
mod node;
mod kvstore;

use std::collections::{HashMap, BTreeMap};
use core::hash::Hash;

use kvstore::KVStore;
use node::Node;



#[derive(Debug)]
struct MemoryStore<K, V> {
    data: HashMap<K, V>,
}

impl<K, V> KVStore<K, V> for MemoryStore<K, V> 
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    fn get(&self, key: K) -> Option<V> {
        self.data.get(&key).cloned()
    }

    fn put(&mut self, key: K, value: V) {
        self.data.insert(key, value);
    }
}

#[derive(Debug)]
struct ProllyTree<K, V> {
    root: Node<K, V>,
}

impl<K, V> ProllyTree<K, V>
where
    K: Ord + Clone,
    V: Clone,
{
    fn new() -> Self {
        Self {
            root: Node::Leaf {
                hash: "".to_string(),
                data: BTreeMap::new(),
            },
        }
    }

    fn get(&self, key: K) -> Option<V> {
        self.root.get(key)
    }

    fn put(&mut self, key: K, value: V) {
        self.root.put(key, value);
        let chunks = self.root.chunk();
    }
}


fn main() {
    let mut storage: MemoryStore<Vec<u8>, Vec<u8>> = MemoryStore::new();
    for i in 0..4 {
        let key = format!("key{}", i.to_string());
        let value = format!("value{}", i.to_string());
        storage.put(key.into(), value.into());
    }
    println!("Storage {:?}", storage);



    // let mut tree = prolly_tree::ProllyTree::new();
    // tree.put(vec![2, 4], vec![8, 9]);
    // println!("Testing {:?}", tree.get(vec![2, 4]).unwrap());
}
