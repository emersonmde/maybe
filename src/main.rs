mod kvstore;
mod node;

use core::hash::Hash;
use std::collections::{BTreeMap, HashMap};

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
struct ProllyTree {
    root: Node,
}

impl ProllyTree {
    fn new() -> Self {
        Self {
            root: Node::Leaf {
                digest: "".to_string(),
                data: BTreeMap::new(),
            },
        }
    }

    fn get(&self, key: Vec<u8>) -> Option<Vec<u8>> {
        self.root.get(key)
    }

    fn put(&mut self, key: Vec<u8>, value: Vec<u8>) {
        self.root.put(key, value);
        let _chunks = self.root.chunk();
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

    let mut tree: ProllyTree = ProllyTree::new();
    tree.put(vec![2, 4], vec![8, 9]);
    println!("Tree {:?}", tree);
    println!("2, 4 {:?}", tree.get(vec![2, 4]).unwrap());
    println!(
        "key1 {:?} {:?}",
        String::from("key1").as_bytes().to_vec(),
        tree.get(String::from("key1").as_bytes().to_vec())
    );
}
